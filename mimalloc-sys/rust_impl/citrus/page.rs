/*----------------------------------------------------------------------------
Copyright (c) 2018, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
/* -----------------------------------------------------------
  The core of the allocator. Every segment contains
  pages of a certain block size. The main function
  exported is `mi_malloc_generic`.
----------------------------------------------------------- */
// memset, memcpy
/* -----------------------------------------------------------
  Definition of page queues for each block size
----------------------------------------------------------- */
/* -----------------------------------------------------------
  Page helpers
----------------------------------------------------------- */
// Index a block in a page
unsafe fn mi_page_block_at(mut page: &mi_page_t, mut page_start: *mut c_void,
                           mut i: usize) -> *mut mi_block_t {
    return (page_start as *mut u8).offset((i * page.block_size)) as
               *mut mi_block_t;
}
// Start of the page available memory
//mi_assert_internal(start + page->capacity*page->block_size == page->top);
#[no_mangle]
pub unsafe extern "C" fn _mi_page_use_delayed_free(mut page: &mut mi_page_t,
                                                   mut enable: bool) {
    let mut tfree:
            mi_thread_free_t; // delay until outstanding MI_DELAYED_FREEING are done.
    let mut tfreex: mi_thread_free_t; // and try again
    loop  {
        tfreex =
            {
                tfree =
                    page.thread_free; // avoid atomic operation if already equal
                tfree
            };
        tfreex.delayed =
            if enable != 0 != 0 {
                MI_USE_DELAYED_FREE
            } else { MI_NO_DELAYED_FREE };
        if __builtin_expect((tfree.delayed == MI_DELAYED_FREEING), 0) != 0 {
            mi_atomic_yield();
            continue ;
        }
        if !(tfreex.delayed != tfree.delayed &&
                 !mi_atomic_compare_exchange(&mut page.thread_free as
                                                 *mut volatile_uintptr_t,
                                             tfreex.value, tfree.value)) {
            break
        };
    };
}
/* -----------------------------------------------------------
  Page collect the `local_free` and `thread_free` lists
----------------------------------------------------------- */
// Collect the local `thread_free` list using an atomic exchange.
// Note: The exchange must be done atomically as this is used right after
// moving to the full list in `mi_page_collect_ex` and we need to
// ensure that there was no race where the page became unfull just before the move.
pub static MI_TF_PTR_SHIFT: c_int = 2;
unsafe fn mi_page_thread_free_collect(mut page: &mut mi_page_t) {
    let mut head: *mut mi_block_t;
    let mut tfree: mi_thread_free_t;
    let mut tfreex: mi_thread_free_t;
    loop  {
        tfreex = { tfree = page.thread_free; tfree };
        head = (tfree.head << MI_TF_PTR_SHIFT) as *mut mi_block_t;
        tfreex.head = 0;
        if !!mi_atomic_compare_exchange(&mut page.thread_free as
                                            *mut volatile_uintptr_t,
                                        tfreex.value, tfree.value) {
            break
        };
    }
    // return if the list is empty
    if head.is_null() { return; }
    // find the tail
    let mut count = 1;
    let mut tail = head;
    let mut next: *mut mi_block_t;
    while !{ next = mi_block_next(page, tail); next }.is_null() {
        count += 1;
        tail = next;
    }
    // and prepend to the free list
    mi_block_set_next(page, tail, page.free);
    page.free = head;
    // update counts now
    mi_atomic_subtract(&mut page.thread_freed, count);
    page.used -= count;
}
#[no_mangle]
pub unsafe extern "C" fn _mi_page_free_collect(mut page: &mut mi_page_t) {
    //if (page->free != NULL) return; // avoid expensive append
    // free the local free list
    if !page.local_free.is_null() {
        if __builtin_expect((page.free.is_null()), 1) != 0 { // usual caes
            page.free = page.local_free;
        } else {
            let mut tail = page.free;
            let mut next: *mut mi_block_t;
            while !{ next = mi_block_next(page, tail); next }.is_null() {
                tail = next;
            }
            mi_block_set_next(page, tail, page.local_free);
        }
        page.local_free = ptr::null_mut();
    }
    // and the thread free list
    if page.thread_free.head != 0 { // quick test to avoid an atomic operation
        mi_page_thread_free_collect(page);
    };
}
/* -----------------------------------------------------------
  Page fresh and retire
----------------------------------------------------------- */
// called from segments when reclaiming abandoned pages
#[no_mangle]
pub unsafe extern "C" fn _mi_page_reclaim(mut heap: *mut mi_heap_t,
                                          mut page: &mut mi_page_t) {
    _mi_page_free_collect(page);
    let mut pq = mi_page_queue(heap, page.block_size);
    mi_page_queue_push(heap, pq, page);
}
// allocate a fresh page from a segment
unsafe fn mi_page_fresh_alloc(mut heap: &mut mi_heap_t,
                              mut pq: *mut mi_page_queue_t,
                              mut block_size: usize) -> *mut mi_page_t {
    let mut page =
        _mi_segment_page_alloc(block_size, &mut heap.tld.segments,
                               &mut heap.tld.os);
    if page.is_null() { return ptr::null_mut(); }
    mi_page_init(heap, page, block_size, &mut heap.tld.stats);
    _mi_stat_increase(&mut ((heap).tld.stats.pages), 1);
    mi_page_queue_push(heap, pq, page);
    return page;
}
// Get a fresh page to use
unsafe fn mi_page_fresh(mut heap: &mut mi_heap_t,
                        mut pq: &mut mi_page_queue_t) -> *mut mi_page_t {
    // try to reclaim an abandoned page first
    let mut page =
        pq.first; // we reclaimed, and we got lucky with a reclaimed page in our queue
    if heap.no_reclaim == 0 &&
           _mi_segment_try_reclaim_abandoned(heap, false,
                                             &mut heap.tld.segments) != 0 &&
           page != pq.first {
        page = pq.first;
        if !page.free.is_null() { return page; };
    }
    // otherwise allocate the page
    page = mi_page_fresh_alloc(heap, pq, pq.block_size);
    if page.is_null() { return ptr::null_mut(); }
    return page;
}
/* -----------------------------------------------------------
   Do any delayed frees
   (put there by other threads if they deallocated in a full page)
----------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn _mi_heap_delayed_free(mut heap: &mut mi_heap_t) {
    // take over the list
    let mut block: *mut mi_block_t;
    loop  {
        block = heap.thread_delayed_free as *mut mi_block_t;
        if !(!block.is_null() &&
                 !mi_atomic_compare_exchange_ptr(&mut heap.thread_delayed_free
                                                     as *mut *mut c_void,
                                                 ptr::null(),
                                                 block as *mut _)) {
            break
        };
    }
    // and free them all
    while !block.is_null() {
        let mut next =
            mi_block_nextx(heap.cookie,
                           block); // use internal free instead of regular one to keep stats etc correct
        _mi_free_delayed_block(block);
        block = next;
    };
}
/* -----------------------------------------------------------
  Unfull, abandon, free and retire
----------------------------------------------------------- */
// Move a page from the full list back to a regular list
pub static MI_BIN_FULL: c_uint = 64 + 1;
#[no_mangle]
pub unsafe extern "C" fn _mi_page_unfull(mut page: &mut mi_page_t) {
    _mi_page_use_delayed_free(page, false); // to get the right queue
    if page.flags.in_full == 0 { return; }
    let mut heap = page.heap;
    let mut pqfull = &mut heap.pages[MI_BIN_FULL];
    page.flags.in_full = false;
    let mut pq = mi_heap_page_queue_of(heap, page);
    page.flags.in_full = true;
    mi_page_queue_enqueue_from(pq, pqfull, page);
}
unsafe fn mi_page_to_full(mut page: &mut mi_page_t,
                          mut pq: *mut mi_page_queue_t) {
    _mi_page_use_delayed_free(page, true);
    if page.flags.in_full != 0 { return; }
    mi_page_queue_enqueue_from(&mut page.heap.pages[MI_BIN_FULL], pq, page);
    mi_page_thread_free_collect(page);
    // try to collect right away in case another thread freed just before MI_USE_DELAYED_FREE was set
}
// Abandon a page with used blocks at the end of a thread.
// Note: only call if it is ensured that no references exist from
// the `page->heap->thread_delayed_free` into this page.
// Currently only called through `mi_heap_collect_ex` which ensures this.
#[no_mangle]
pub unsafe extern "C" fn _mi_page_abandon(mut page: &mut mi_page_t,
                                          mut pq: *mut mi_page_queue_t) {
    // check there are no references left..
    // and then remove from our page list
    let mut segments_tld = &mut page.heap.tld.segments;
    mi_page_queue_remove(pq, page);
    // and abandon it
    _mi_segment_page_abandon(page, segments_tld);
}
// Free a page with no more free blocks
// account for huge pages here
pub static MI_LARGE_SIZE_MAX: c_long = ((1 << (6 + (13 + 3))) / 8);
#[no_mangle]
pub unsafe extern "C" fn _mi_page_free(mut page: &mut mi_page_t,
                                       mut pq: *mut mi_page_queue_t,
                                       mut force: bool) {
    page.flags.has_aligned = false;
    if page.block_size > MI_LARGE_SIZE_MAX {
        _mi_stat_decrease(&mut ((page.heap).tld.stats.huge), page.block_size);
    }
    // remove from the page list
    // (no need to do _mi_heap_delayed_free first as all blocks are already free)
    let mut segments_tld = &mut page.heap.tld.segments;
    mi_page_queue_remove(pq, page);
    // and free it
    _mi_segment_page_free(page, force != 0, segments_tld);
}
// Retire a page with no more used blocks
// Important to not retire too quickly though as new
// allocations might coming.
// Note: called from `mi_free` and benchmarks often
// trigger this due to freeing everything and then
// allocating again so careful when changing this.
#[no_mangle]
pub unsafe extern "C" fn _mi_page_retire(mut page: &mut mi_page_t) {
    page.flags.has_aligned = false;
    // don't retire too often..
    // (or we end up retiring and re-allocating most of the time)
    // NOTE: refine this more: we should not retire if this
    // is the only page left with free blocks. It is not clear
    // how to check this efficiently though... for now we just check
    // if its neighbours are almost fully used.
    if __builtin_expect((page.block_size <= MI_LARGE_SIZE_MAX), 1) != 0 {
        if mi_page_mostly_used(page.prev) != 0 &&
               mi_page_mostly_used(page.next) != 0 {
            return; // dont't retire after all
        };
    }
    _mi_page_free(page, mi_page_queue_of(page), false);
}
/* -----------------------------------------------------------
  Initialize the initial free list in a page.
  In secure mode we initialize a randomized list by 
  alternating between slices.
----------------------------------------------------------- */
// at most 64 slices
pub static MI_MIN_SLICES: c_long = 2;
pub static MI_MAX_SLICE_SHIFT: usize =
     // initialize a sequential free list
    // initialize a randomized free list
    // set up `slice_count` slices to alternate between
    6;
// current start of the slice
// available objects in the slice
// final slice holds the modulus too (todo: distribute evenly?)
// and initialize the free list by randomly threading through them    
// set up first element
// and iterate through the rest
// call random_shuffle only every INTPTR_SIZE rounds
pub static MI_INTPTR_SIZE: c_long = (1 << 3);
unsafe fn mi_page_free_list_extend(mut heap: &mut mi_heap_t,
                                   mut page: &mut mi_page_t,
                                   mut extend: usize,
                                   mut stats: &mut mi_stats_t) {
    (stats); // select a random next slice index
    let mut page_area =
        _mi_page_start(_mi_page_segment(page), page,
                       ptr::null_mut()); // ensure it still has space
    let mut bsize = page.block_size; // and link the current block to it
    let mut start =
        mi_page_block_at(page, page_area,
                         page.capacity); // bump to the following block
    if extend < MI_MIN_SLICES || !mi_option_is_enabled(mi_option_secure) {
        let mut end =
            mi_page_block_at(page, page_area, page.capacity + extend - 1);
        let mut block = start;
        for mut i in 0..extend {
            let mut next =
                (block as *mut u8).offset(bsize) as *mut mi_block_t;
            mi_block_set_next(page, block, next);
            block = next;
        }
        mi_block_set_next(page, end, ptr::null_mut());
        page.free = start;
    } else {
        let mut shift = MI_MAX_SLICE_SHIFT;
        while (extend >> shift) == 0 { shift -= 1; }
        let mut slice_count = 1 << shift;
        let mut slice_extend = extend / slice_count;
        let mut blocks: [*mut mi_block_t; 64];
        let mut counts: [usize; 64];
        for mut i in 0..slice_count {
            blocks[i] =
                mi_page_block_at(page, page_area,
                                 page.capacity + i * slice_extend);
            counts[i] = slice_extend;
        }
        counts[slice_count - 1] += (extend % slice_count);
        let mut current = _mi_heap_random(heap) % slice_count;
        counts[current] -= 1;
        page.free = blocks[current];
        let mut rnd = heap.random;
        for mut i in 1..extend {
            let mut round = i % MI_INTPTR_SIZE;
            if round == 0 { rnd = _mi_random_shuffle(rnd); }
            let mut next = ((rnd >> 8 * round) & (slice_count - 1));
            while counts[next] == 0 {
                next += 1;
                if next == slice_count { next = 0; };
            }
            counts[next] -= 1;
            let mut block = blocks[current];
            blocks[current] =
                (block as *mut u8).offset(bsize) as *mut mi_block_t;
            mi_block_set_next(page, block, blocks[next]);
            // and set next; note: we may have `current == next` 
            current = next; // end of the list
        }
        mi_block_set_next(page, blocks[current], ptr::null_mut());
        heap.random = _mi_random_shuffle(rnd);
    }
    // enable the new free list
    page.capacity += extend as u16;
    _mi_stat_increase(&mut (stats.committed), extend * page.block_size);
}
/* -----------------------------------------------------------
  Page initialize and extend the capacity
----------------------------------------------------------- */
pub static MI_MAX_EXTEND_SIZE: c_long =
     // heuristic, one OS page seems to work well.
    // extend at least by this many 
    // Extend the capacity (up to reserved) by initializing a free list
    // We do at most `MI_MAX_EXTEND` to avoid touching too much memory
    // Note: we also experimented with "bump" allocation on the first
    // allocations but this did not speed up any benchmark (due to an
    // extra test in malloc? or cache effects?)
    // calculate the extend count
    (4 * 1024);
pub static MI_MIN_EXTEND: c_long = 1;
unsafe fn mi_page_extend_free(mut heap: *mut mi_heap_t,
                              mut page: &mut mi_page_t,
                              mut stats: &mut mi_stats_t) {
    (stats); // ensure we don't touch memory beyond the page to reduce page commit.
    if page.free.is_null() {
        0
    } else {
        _mi_assert_fail("page->free == NULL", "src/page.c", 477,
                        "mi_page_extend_free")
    } // the `lean` benchmark tests this. Going from 1 to 8 increases rss by 50%.
    if page.local_free.is_null() {
        0
    } else {
        _mi_assert_fail("page->local_free == NULL", "src/page.c", 478,
                        "mi_page_extend_free")
    }
    if !page.free.is_null() { return; }
    if page.capacity >= page.reserved { return; }
    let mut page_size: usize;
    _mi_page_start(_mi_page_segment(page), page, &mut page_size);
    if page.is_reset != 0 {
        page.is_reset = false;
        _mi_stat_decrease(&mut (stats.reset), page_size);
    }
    _mi_stat_increase(&mut (stats.pages_extended), 1);
    let mut extend = page.reserved - page.capacity;
    let mut max_extend = MI_MAX_EXTEND_SIZE / page.block_size;
    if max_extend < MI_MIN_EXTEND { max_extend = MI_MIN_EXTEND; }
    if extend > max_extend {
        extend = if max_extend == 0 { 1 } else { max_extend };
    }
    // and append the extend the free list
    mi_page_free_list_extend(heap, page, extend, stats);
}
// Initialize a fresh page
unsafe fn mi_page_init(mut heap: *mut mi_heap_t, mut page: *mut mi_page_t,
                       mut block_size: usize, mut stats: *mut mi_stats_t) {
    if !page.is_null() {
        0
    } else {
        _mi_assert_fail("page != NULL", "src/page.c", 514, "mi_page_init")
    }
    let mut segment = _mi_page_segment(page);
    if !segment.is_null() {
        0
    } else {
        _mi_assert_fail("segment != NULL", "src/page.c", 516, "mi_page_init")
    }
    // set fields
    let mut page_size: usize;
    _mi_segment_page_start(segment, page, &mut page_size);
    page.block_size = block_size;
    page.reserved = (page_size / block_size) as u16;
    page.cookie = _mi_heap_random(heap) | 1;
    // initialize an initial free list
    mi_page_extend_free(heap, page, stats);
    if mi_page_immediate_available(page) != 0 {
        0
    } else {
        _mi_assert_fail("mi_page_immediate_available(page)", "src/page.c",
                        539, "mi_page_init")
    };
}
/* -----------------------------------------------------------
  Find pages with free blocks
-------------------------------------------------------------*/
// Find a page with free blocks of `page->block_size`.
unsafe fn mi_page_queue_find_free_ex(mut heap: &mut mi_heap_t,
                                     mut pq: &mut mi_page_queue_t)
 -> *mut mi_page_t {
    // search through the pages in "next fit" order
    let mut rpage = ptr::null_mut(); // remember next
    let mut count = 0; // 0. collect freed blocks by us and other threads
    let mut page_free_count =
        0; // 1. if the page contains free blocks, we are done
    let mut page =
        pq.first; // If all blocks are free, we might retire this page instead.
    while !page.is_null() {
        let mut next =
            page.next; // do this at most 8 times to bound allocation time.
        count +=
            1; // (note: this can happen if a page was earlier not retired due
        _mi_page_free_collect(page); //  to having neighbours that were mostly full or due to concurrent frees)
        if mi_page_immediate_available(page) {
            if page_free_count < 8 && mi_page_all_free(page) != 0 {
                page_free_count += 1; // and keep looking
                if !rpage.is_null() {
                    _mi_page_free(rpage, pq, false); // pick this one
                } // 2. Try to extend
                rpage =
                    page; // 3. If the page is completely full, move it to the `mi_pages_full`
                page =
                    next; // queue so we don't visit long-lived pages too often.
                continue ; // for each page
            } else { break ; };
        }
        if page.capacity < page.reserved {
            mi_page_extend_free(heap, page, &mut heap.tld.stats);
            break ;
        }
        mi_page_to_full(page, pq);
        page = next;
    }
    _mi_stat_counter_increase(&mut (heap.tld.stats.searches), count);
    if page.is_null() { page = rpage; rpage = ptr::null_mut(); }
    if !rpage.is_null() { _mi_page_free(rpage, pq, false); }
    if page.is_null() {
        page = mi_page_fresh(heap, pq);
    } else {
        if pq.first == page {
            0
        } else {
            _mi_assert_fail("pq->first == page", "src/page.c", 609,
                            "mi_page_queue_find_free_ex")
        };
    }
    return page;
}
// Find a page with free blocks of `size`.
unsafe fn mi_find_free_page(mut heap: &mut mi_heap_t, mut size: usize)
 -> *mut mi_page_t {
    _mi_heap_delayed_free(heap); // in secure mode, we extend half the time to increase randomness
    let mut pq = mi_page_queue(heap, size); // fast path
    let mut page = pq.first;
    if !page.is_null() {
        if mi_option_get(mi_option_secure) >= 3 &&
               page.capacity < page.reserved &&
               ((_mi_heap_random(heap) & 1) == 1) {
            mi_page_extend_free(heap, page, &mut heap.tld.stats);
        } else { _mi_page_free_collect(page); }
        if mi_page_immediate_available(page) { return page; };
    }
    return mi_page_queue_find_free_ex(heap, pq);
}
/* -----------------------------------------------------------
  Users can register a deferred free function called
  when the `free` list is empty. Since the `local_free`
  is separate this is deterministically called after
  a certain number of allocations.
----------------------------------------------------------- */
pub static mut deferred_free: *mut mi_deferred_free_fun = ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn _mi_deferred_free(mut heap: &mut mi_heap_t,
                                           mut force: bool) {
    heap.tld.heartbeat += 1;
    if !deferred_free.is_null() {
        deferred_free(force != 0, heap.tld.heartbeat);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mi_register_deferred_free(mut fn_:
                                                       *mut mi_deferred_free_fun) {
    deferred_free = fn_;
}
/* -----------------------------------------------------------
  General allocation
----------------------------------------------------------- */
// A huge page is allocated directly without being in a queue
unsafe fn mi_huge_page_alloc(mut heap: &mut mi_heap_t, mut size: usize)
 -> *mut mi_page_t {
    let mut block_size =
        _mi_wsize_from_size(size) * std::mem::size_of::<usize>();
    let mut pq = mi_page_queue(heap, block_size);
    let mut page = mi_page_fresh_alloc(heap, pq, block_size);
    if !page.is_null() {
        _mi_stat_increase(&mut ((heap).tld.stats.huge), block_size);
    }
    return page;
}
// Generic allocation routine if the fast path (`alloc.c:mi_page_malloc`) does not succeed.
#[no_mangle]
pub unsafe extern "C" fn _mi_malloc_generic(mut heap: *mut mi_heap_t,
                                            mut size: usize) -> *mut c_void {
    // initialize if necessary
    if __builtin_expect((!mi_heap_is_initialized(heap)), 0) != 0 {
        mi_thread_init(); // calls `_mi_heap_init` in turn
        heap = mi_get_default_heap();
    }
    // call potential deferred free routines
    _mi_deferred_free(heap, false);
    // huge allocation?
    let mut page:
            *mut mi_page_t; // otherwise find a page with free blocks in our size segregated queues
    if __builtin_expect((size > MI_LARGE_SIZE_MAX), 0) != 0 {
        page = mi_huge_page_alloc(heap, size); // out of memory
    } else { page = mi_find_free_page(heap, size); }
    if page.is_null() { return ptr::null_mut(); }
    // and try again, this time succeeding! (i.e. this should never recurse)
    return _mi_page_malloc(heap, page, size);
}
