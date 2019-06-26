/*----------------------------------------------------------------------------
Copyright (c) 2018, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
// memset, memcpy
/* -----------------------------------------------------------
  Helpers
----------------------------------------------------------- */
// return `true` if ok, `false` to break
pub type heap_page_visitor_fun = unsafe extern "C" fn(, ...);
// Visit all pages in a heap; returns `false` if break was called.
// visit all pages
pub static MI_BIN_FULL: c_long = (64 + 1);
unsafe fn mi_heap_visit_pages(mut heap: *mut mi_heap_t,
                              mut fn_: &mut heap_page_visitor_fun,
                              mut arg1: *mut c_void, mut arg2: *mut c_void)
 -> bool {
    if heap.is_null() || heap.page_count == 0 {
        return 0 !=
                   0; // save next in case the page gets removed from the queue
    } // and continue
    let mut count = 0;
    for mut i in 0..(MI_BIN_FULL + 1) {
        let mut pq = &mut heap.pages[i];
        let mut page = pq.first;
        while !page.is_null() {
            let mut next = page.next;
            count += 1;
            if !fn_(heap, pq, page, arg1, arg2) { return false; }
            page = next;
        };
    }
    return true;
}
/* -----------------------------------------------------------
  "Collect" pages by migrating `local_free` and `thread_free`
  lists and freeing empty pages. This is done when a thread
  stops (and in that case abandons pages if there are still
  blocks alive)
----------------------------------------------------------- */
pub enum mi_collect_e { NORMAL, FORCE, ABANDON, }
unsafe fn mi_heap_page_collect(mut heap: *mut mi_heap_t,
                               mut pq: *mut mi_page_queue_t,
                               mut page: *mut mi_page_t,
                               mut arg_collect: *mut c_void,
                               mut arg2: *mut c_void) -> bool {
    (arg2);
    (heap);
    let mut collect = arg_collect as mi_collect_t;
    _mi_page_free_collect(page);
    if mi_page_all_free(page) {
        // no more used blocks, free the page. TODO: should we retire here and be less aggressive?
        _mi_page_free(page, pq,
                      collect !=
                          NORMAL); // still used blocks but the thread is done; abandon the page
    } else if collect == ABANDON {
        _mi_page_abandon(page, pq); // don't break
    }
    return true;
}
unsafe fn mi_heap_collect_ex(mut heap: &mut mi_heap_t,
                             mut collect: mi_collect_t) {
    _mi_deferred_free(heap, collect > NORMAL);
    if !mi_heap_is_initialized(heap) { return; }
    // collect (some) abandoned pages
    if collect >= NORMAL && heap.no_reclaim == 0 {
        if collect == NORMAL
           { // this may free some segments (but also take ownership of abandoned pages)
            _mi_segment_try_reclaim_abandoned(heap, false,
                                              &mut heap.tld.segments); // the main thread is abandoned, try to free all abandoned segments.
        } else if collect == ABANDON && _mi_is_main_thread() != 0 &&
                      mi_heap_is_backing(heap) != 0
         { // if all memory is freed by now, all segments should be freed.
            _mi_segment_try_reclaim_abandoned(heap, true,
                                              &mut heap.tld.segments);
        };
    }
    // if abandoning, mark all full pages to no longer add to delayed_free
    if collect == ABANDON {
        let mut page = (heap.pages[MI_BIN_FULL]).first;
        while !page.is_null() {
            _mi_page_use_delayed_free(page, false);
            page = page.next
            // set thread_free.delayed to MI_NO_DELAYED_FREE      
        };
    }
    // free thread delayed blocks. 
    // (if abandoning, after this there are no more local references into the pages.)
    _mi_heap_delayed_free(heap);
    // collect all pages owned by this thread
    mi_heap_visit_pages(heap, &mut mi_heap_page_collect,
                        (collect) as *mut c_void, ptr::null_mut());
    // collect segment caches
    if collect >= FORCE {
        _mi_segment_thread_collect(&mut heap.tld.segments);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mi_heap_collect_abandon(mut heap: *mut mi_heap_t) {
    mi_heap_collect_ex(heap, ABANDON);
}
#[no_mangle]
pub unsafe extern "C" fn mi_heap_collect(mut heap: *mut mi_heap_t,
                                         mut force: bool) {
    mi_heap_collect_ex(heap, if force != 0 != 0 { FORCE } else { NORMAL });
}
#[no_mangle]
pub unsafe extern "C" fn mi_collect(mut force: bool) {
    mi_heap_collect(mi_get_default_heap(), force != 0);
}
/* -----------------------------------------------------------
  Heap new
----------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn mi_heap_get_default() -> *mut mi_heap_t {
    mi_thread_init(); // don't reclaim abandoned pages or otherwise destroy is unsafe
    return mi_get_default_heap();
}
#[no_mangle]
pub unsafe extern "C" fn mi_heap_get_backing() -> *mut mi_heap_t {
    let mut heap = mi_heap_get_default();
    let mut bheap = heap.tld.heap_backing;
    return bheap;
}
#[no_mangle]
pub unsafe extern "C" fn _mi_heap_random(mut heap: &mut mi_heap_t) -> usize {
    let mut r = heap.random;
    heap.random = _mi_random_shuffle(r);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mi_heap_new() -> *mut mi_heap_t {
    let mut bheap = mi_heap_get_backing();
    let mut heap =
        (mi_heap_malloc(bheap, std::mem::size_of::<mi_heap_t>()) as
             *mut mi_heap_t);
    if heap.is_null() { return ptr::null_mut(); }
    memcpy(heap as *mut _, &_mi_heap_empty, std::mem::size_of::<mi_heap_t>());
    heap.tld = bheap.tld;
    heap.thread_id = _mi_thread_id();
    heap.cookie = ((heap as usize) ^ _mi_heap_random(bheap)) | 1;
    heap.random = _mi_heap_random(bheap);
    heap.no_reclaim = true;
    return heap;
}
// zero out the page queues
unsafe fn mi_heap_reset_pages(mut heap: &mut mi_heap_t) {
    // TODO: copy full empty heap instead?
    memset(&mut heap.pages_free_direct, 0,
           std::mem::size_of::<[*mut mi_page_t; 130]>());
    memcpy(&mut heap.pages, &_mi_heap_empty.pages,
           std::mem::size_of::<[mi_page_queue_t; 66]>());
    heap.thread_delayed_free = ptr::null_mut();
    heap.page_count = 0;
}
// called from `mi_heap_destroy` and `mi_heap_delete` to free the internal heap resources.
unsafe fn mi_heap_free(mut heap: &mut mi_heap_t) {
    if mi_heap_is_backing(heap) {
        return; // dont free the backing heap
    }
    // reset default
    if mi_heap_is_default(heap) { _mi_heap_default = heap.tld.heap_backing; }
    // and free the used memory
    mi_free(heap as *mut _);
}
/* -----------------------------------------------------------
  Heap destroy
----------------------------------------------------------- */
// ensure no more thread_delayed_free will be added
// stats
pub static MI_LARGE_SIZE_MAX: c_long = ((1 << (6 + (13 + 3))) / 8);
unsafe fn _mi_heap_page_destroy(mut heap: &mut mi_heap_t,
                                mut pq: *mut mi_page_queue_t,
                                mut page: &mut mi_page_t,
                                mut arg1: *mut c_void, mut arg2: *mut c_void)
 -> bool {
    (arg1); // todo: off for aligned blocks...
    (arg2);
    (heap);
    (pq);
    _mi_page_use_delayed_free(page, false);
    if page.block_size > MI_LARGE_SIZE_MAX {
        _mi_stat_decrease(&mut ((heap).tld.stats.huge), page.block_size);
    }
    let mut inuse = page.used - page.thread_freed;
    if page.block_size <= MI_LARGE_SIZE_MAX {
        _mi_stat_decrease(&mut ((heap).tld.stats.normal[_mi_bin(page.block_size)]),
                          inuse);
    }
    _mi_stat_decrease(&mut ((heap).tld.stats.malloc),
                      page.block_size * inuse);
    // pretend it is all free now
    page.used = page.thread_freed as u16;
    // and free the page
    _mi_segment_page_free(page, false, /* no force? */
                          &mut heap.tld.segments); // keep going
    return true; // don't free in case it may contain reclaimed pages
}
#[no_mangle]
pub unsafe extern "C" fn _mi_heap_destroy_pages(mut heap: *mut mi_heap_t) {
    mi_heap_visit_pages(heap, &mut _mi_heap_page_destroy, ptr::null_mut(),
                        ptr::null_mut()); // free all pages
    mi_heap_reset_pages(heap);
}
#[no_mangle]
pub unsafe extern "C" fn mi_heap_destroy(mut heap: &mut mi_heap_t) {
    if mi_heap_is_initialized(heap) != 0 {
        0
    } else {
        _mi_assert_fail("mi_heap_is_initialized(heap)", "src/heap.c", 261,
                        "mi_heap_destroy")
    }
    if (heap.no_reclaim) != 0 != 0 {
        0
    } else {
        _mi_assert_fail("heap->no_reclaim", "src/heap.c", 262,
                        "mi_heap_destroy")
    }
    if !mi_heap_is_initialized(heap) { return; }
    if heap.no_reclaim == 0 {
        mi_heap_delete(heap);
    } else { _mi_heap_destroy_pages(heap); mi_heap_free(heap); };
}
/* -----------------------------------------------------------
  Safe Heap delete
----------------------------------------------------------- */
// Tranfer the pages from one heap to the other
unsafe fn mi_heap_absorb(mut heap: &mut mi_heap_t, mut from: *mut mi_heap_t) {
    if from.is_null() || from.page_count == 0 { return; }
    // unfull all full pages
    let mut page = (heap.pages[MI_BIN_FULL]).first;
    while !page.is_null() {
        let mut next = page.next;
        _mi_page_unfull(page);
        page = next;
    }
    // free outstanding thread delayed free blocks
    _mi_heap_delayed_free(from);
    // transfer all pages by appending the queues; this will set
    // a new heap field which is ok as all pages are unfull'd and thus 
    // other threads won't access this field anymore (see `mi_free_block_mt`)
    for mut i in 0..MI_BIN_FULL {
        let mut pq = &mut heap.pages[i];
        let mut append = &mut from.pages[i];
        _mi_page_queue_append(heap, pq, append);
    }
    // and reset the `from` heap
    mi_heap_reset_pages(from);
}
// Safe delete a heap without freeing any still allocated blocks in that heap.
#[no_mangle]
pub unsafe extern "C" fn mi_heap_delete(mut heap: &mut mi_heap_t) {
    if mi_heap_is_initialized(heap) != 0 {
        0
    } else {
        _mi_assert_fail("mi_heap_is_initialized(heap)", "src/heap.c", 316,
                        "mi_heap_delete")
    } // tranfer still used pages to the backing heap
    if !mi_heap_is_initialized(heap) {
        return; // the backing heap abandons its pages
    }
    if !mi_heap_is_backing(heap) {
        mi_heap_absorb(heap.tld.heap_backing, heap);
    } else { _mi_heap_collect_abandon(heap); }
    mi_heap_free(heap);
}
#[no_mangle]
pub unsafe extern "C" fn mi_heap_set_default(mut heap: *mut mi_heap_t)
 -> *mut mi_heap_t {
    if mi_heap_is_initialized(heap) != 0 {
        0
    } else {
        _mi_assert_fail("mi_heap_is_initialized(heap)", "src/heap.c", 333,
                        "mi_heap_set_default")
    }
    if !mi_heap_is_initialized(heap) { return ptr::null_mut(); }
    let mut old = _mi_heap_default;
    _mi_heap_default = heap;
    return old;
}
/* -----------------------------------------------------------
  Analysis
----------------------------------------------------------- */
// static since it is not thread safe to access heaps from other threads.
unsafe fn mi_heap_of_block(mut p: *const c_void) -> *mut mi_heap_t {
    if p.is_null() {
        return ptr::null_mut(); // continue if not found
    } // only aligned pointers
    let mut segment = _mi_ptr_segment(p);
    let mut valid = (_mi_ptr_cookie(segment as *const _) == segment.cookie);
    if __builtin_expect((valid == 0), 0) != 0 { return ptr::null_mut(); }
    return _mi_segment_page_of(segment, p).heap;
}
#[no_mangle]
pub unsafe extern "C" fn mi_heap_contains_block(mut heap: *mut mi_heap_t,
                                                mut p: *const c_void)
 -> bool {
    if !heap.is_null() {
        0
    } else {
        _mi_assert_fail("heap != NULL", "src/heap.c", 359,
                        "mi_heap_contains_block")
    }
    if !mi_heap_is_initialized(heap) { return false; }
    return (heap == mi_heap_of_block(p));
}
unsafe fn mi_heap_page_check_owned(mut heap: *mut mi_heap_t,
                                   mut pq: *mut mi_page_queue_t,
                                   mut page: &mut mi_page_t,
                                   mut p: *mut c_void,
                                   mut vfound: *mut c_void) -> bool {
    (heap);
    (pq);
    let mut found = vfound as *mut bool;
    let mut segment = _mi_page_segment(page);
    let mut start = _mi_page_start(segment, page, ptr::null_mut());
    let mut end =
        (start as *mut u8).offset((page.capacity * page.block_size));
    *found = (p >= start && p < end);
    return (*found == 0);
}
pub static MI_INTPTR_SIZE: c_int = 1 << 3;
#[no_mangle]
pub unsafe extern "C" fn mi_heap_check_owned(mut heap: *mut mi_heap_t,
                                             mut p: *const c_void) -> bool {
    if !heap.is_null() {
        0
    } else {
        _mi_assert_fail("heap != NULL", "src/heap.c", 377,
                        "mi_heap_check_owned")
    }
    if !mi_heap_is_initialized(heap) { return false; }
    if ((p as usize) & (MI_INTPTR_SIZE - 1)) != 0 { return false; }
    let mut found = false;
    mi_heap_visit_pages(heap, &mut mi_heap_page_check_owned, p as *mut c_void,
                        &mut found);
    return found != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mi_check_owned(mut p: *const c_void) -> bool {
    return mi_heap_check_owned(mi_get_default_heap(), p);
}
/* -----------------------------------------------------------
  Visit all heap blocks and areas
  Todo: enable visiting abandoned pages, and
        enable visiting all blocks of all heaps across threads
----------------------------------------------------------- */
// Separate struct to keep `mi_page_t` out of the public interface
pub struct mi_heap_area_ex_s {
    pub area: mi_heap_area_t,
    pub page: *mut mi_page_t,
}
// optimize page with one block
// create a bitmap of free blocks.
// Todo: avoid division?
// walk through all blocks skipping the free ones
pub static UINTPTR_MAX: c_long = 18446744073709551615;
unsafe fn mi_heap_area_visit_blocks(mut xarea: *const mi_heap_area_ex_t,
                                    mut visitor: &mut mi_block_visit_fun,
                                    mut arg: *mut c_void) -> bool {
    if !xarea.is_null() {
        0
    } else {
        _mi_assert_fail("xarea != NULL", "src/heap.c", 402,
                        "mi_heap_area_visit_blocks")
    } // skip a run of free blocks
    if xarea.is_null() {
        return true; // race is ok
    }
    let mut area = &xarea.area;
    let mut page = xarea.page;
    if !page.is_null() {
        0
    } else {
        _mi_assert_fail("page != NULL", "src/heap.c", 406,
                        "mi_heap_area_visit_blocks")
    }
    if page.is_null() { return true; }
    _mi_page_free_collect(page);
    if page.used == 0 { return true; }
    let mut psize: usize;
    let mut pstart = _mi_page_start(_mi_page_segment(page), page, &mut psize);
    if page.capacity == 1 {
        return visitor(page.heap, area, pstart as *mut _, page.block_size,
                       arg);
    }
    let mut free_map: [usize; 1024];
    memset(free_map as *mut _, 0, std::mem::size_of::<[usize; 1024]>());
    let mut free_count = 0;
    let mut block = page.free;
    while !block.is_null() {
        free_count += 1;
        let mut offset = (block as *mut u8).offset(-pstart);
        let mut blockidx = offset / page.block_size;
        let mut bitidx = (blockidx / std::mem::size_of::<usize>());
        let mut bit = blockidx - (bitidx * std::mem::size_of::<usize>());
        free_map[bitidx] |= (1 << bit);
        block = mi_block_next(page, block)
    }
    let mut used_count = 0;
    for mut i in 0..page.capacity {
        let mut bitidx = (i / std::mem::size_of::<usize>());
        let mut bit = i - (bitidx * std::mem::size_of::<usize>());
        let mut m = free_map[bitidx];
        if bit == 0 && m == UINTPTR_MAX {
            i += (std::mem::size_of::<usize>() - 1);
        } else if (m & (1 << bit)) == 0 {
            used_count += 1;
            let mut block = pstart.offset((i * page.block_size));
            if !visitor(page.heap, area, block as *mut _, page.block_size,
                        arg) {
                return false;
            };
        };
    }
    return true;
}
pub type mi_heap_area_visit_fun = unsafe extern "C" fn(, ...);
unsafe fn mi_heap_visit_areas_page(mut heap: *mut mi_heap_t,
                                   mut pq: *mut mi_page_queue_t,
                                   mut page: &mut mi_page_t,
                                   mut vfun: *mut c_void,
                                   mut arg: *mut c_void) -> bool {
    (heap);
    (pq);
    let mut fun = vfun as *mut mi_heap_area_visit_fun;
    let mut xarea: mi_heap_area_ex_t;
    xarea.page = page;
    xarea.area.reserved = page.reserved * page.block_size;
    xarea.area.committed = page.capacity * page.block_size;
    xarea.area.blocks =
        _mi_page_start(_mi_page_segment(page), page, ptr::null_mut());
    xarea.area.used = page.used - page.thread_freed;
    xarea.area.block_size = page.block_size;
    return fun(heap, &mut xarea, arg);
}
// Visit all heap pages as areas
unsafe fn mi_heap_visit_areas(mut heap: *const mi_heap_t,
                              mut visitor: *mut mi_heap_area_visit_fun,
                              mut arg: *mut c_void) -> bool {
    if visitor.is_null() { return false; }
    return mi_heap_visit_pages(heap as *mut mi_heap_t,
                               &mut mi_heap_visit_areas_page,
                               visitor as *mut _, arg);
}
// Just to pass arguments
pub struct mi_visit_blocks_args_s {
    pub visit_blocks: bool,
    pub visitor: *mut mi_block_visit_fun,
    pub arg: *mut c_void,
}
unsafe fn mi_heap_area_visitor(mut heap: *const mi_heap_t,
                               mut xarea: &mi_heap_area_ex_t,
                               mut arg: *mut c_void) -> bool {
    let mut args = arg as *mut mi_visit_blocks_args_t;
    if !args.visitor(heap, &xarea.area, ptr::null(), xarea.area.block_size,
                     arg) {
        return false;
    }
    if args.visit_blocks != 0 {
        return mi_heap_area_visit_blocks(xarea, args.visitor, args.arg);
    } else { return true; };
}
// Visit all blocks in a heap
#[no_mangle]
pub unsafe extern "C" fn mi_heap_visit_blocks(mut heap: *const mi_heap_t,
                                              mut visit_blocks: bool,
                                              mut visitor:
                                                  *mut mi_block_visit_fun,
                                              mut arg: *mut c_void) -> bool {
    let mut args =
        mi_visit_blocks_args_t{_0: visit_blocks != 0, _1: visitor, _2: arg,};
    return mi_heap_visit_areas(heap, &mut mi_heap_area_visitor, &mut args);
}
