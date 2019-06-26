/* ----------------------------------------------------------------------------
Copyright (c) 2018, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
// memset
/* -----------------------------------------------------------
  Segment allocation
  We allocate pages inside big OS allocated "segments"
  (2mb on 64-bit). This is to avoid splitting VMA's on Linux
  and reduce fragmentation on other OS's. Each thread
  owns its own segments.

  Currently we have:
  - small pages (64kb), 32 in one segment
  - large pages (2mb), 1 in one segment
  - huge blocks > RC_LARGE_SIZE_MAX (256kb) are directly allocated by the OS

  It might be good to have "medium" pages too (of, say 256kb)
  to reduce pressure on the virtual address space on 32-bit systems
  but for now we choose the simpler implementation since this
  will only be a problem if multiple threads allocate many
  differently sized objects between 8kb and 2mb which is not common.

  In any case the memory for a segment is virtual and only
  committed on demand (i.e. we are careful to not touch the memory
  until we actually allocate a block there)

  If a  thread ends, it "abandons" pages with used blocks
  and there is an abandoned segment list whose segments can
  be reclaimed by still running threads, much like work-stealing.
----------------------------------------------------------- */
// or 0
/* -----------------------------------------------------------
  Queue of segments containing free pages
----------------------------------------------------------- */
// quick test to see if a segment is in the free pages queue
unsafe fn mi_segment_is_in_free_queue(mut segment: &mut mi_segment_t,
                                      mut tld: &mut mi_segments_tld_t)
 -> bool {
    let mut in_queue =
        (!segment.next.is_null() || !segment.prev.is_null() ||
             tld.small_free.first ==
                 segment); // for now we only support small pages
    if in_queue != 0 {
        if segment.page_kind == MI_PAGE_SMALL {
            0
        } else {
            _mi_assert_fail("segment->page_kind == MI_PAGE_SMALL",
                            "src/segment.c", 82,
                            "mi_segment_is_in_free_queue")
        };
    }
    return in_queue != 0;
}
unsafe fn mi_segment_queue_is_empty(mut queue: &mi_segment_queue_t) -> bool {
    return (queue.first.is_null());
}
unsafe fn mi_segment_queue_remove(mut queue: &mut mi_segment_queue_t,
                                  mut segment: &mut mi_segment_t) {
    if !segment.prev.is_null() { segment.prev.next = segment.next; }
    if !segment.next.is_null() { segment.next.prev = segment.prev; }
    if segment == queue.first { queue.first = segment.next; }
    if segment == queue.last { queue.last = segment.prev; }
    segment.next = ptr::null_mut();
    segment.prev = ptr::null_mut();
}
unsafe fn mi_segment_enqueue(mut queue: &mut mi_segment_queue_t,
                             mut segment: &mut mi_segment_t) {
    segment.next = ptr::null_mut();
    segment.prev = queue.last;
    if !queue.last.is_null() {
        queue.last.next = segment;
        queue.last = segment;
    } else { queue.last = { queue.first = segment; queue.first }; };
}
// Start of the page available memory
#[no_mangle]
pub unsafe extern "C" fn _mi_segment_page_start(mut segment:
                                                    *const mi_segment_t,
                                                mut page: &mi_page_t,
                                                mut page_size: *mut usize)
 -> *mut u8 {
    let mut psize =
        if segment.page_kind == MI_PAGE_HUGE {
            segment.segment_size
        } else {
            1 << segment.page_shift
        }; // the first page starts after the segment info (and possible guard page)
    let mut p =
        (segment as
             *mut u8).offset(page.segment_idx *
                                 psize); // secure == 1: the last page has an os guard page at the end
    if page.segment_idx == 0 {
        p =
            p.offset(segment.segment_info_size); // secure >  1: every page has an os guard page
        psize -= segment.segment_info_size;
    }
    let mut secure = mi_option_get(mi_option_secure);
    if secure > 1 || (secure == 1 && page.segment_idx == segment.capacity - 1)
       {
        psize -= _mi_os_page_size();
    }
    if !page_size.is_null() { *page_size = psize; }
    return p;
}
/*
  if (mi_option_is_enabled(mi_option_secure)) {
    // always reserve maximally so the protection falls on 
    // the same address area, as we need to reuse them from the caches interchangably.
    capacity = MI_SMALL_PAGES_PER_SEGMENT;  
  }
  */
/* padding */
// normally no guard pages
pub static MI_MAX_ALIGN_SIZE: c_int = 16;
// in secure mode, we set up a protected page in between the segment info
// and the page data (and one at the end of the segment)
pub static MI_SEGMENT_SIZE: c_long = (1 << (6 + (13 + 3)));
unsafe fn mi_segment_size(mut capacity: usize, mut required: usize,
                          mut pre_size: *mut usize, mut info_size: *mut usize)
 -> usize {
    let mut minsize =
        std::mem::size_of::<mi_segment_t>() +
            ((capacity - 1) * std::mem::size_of::<mi_page_t>()) + 16;
    let mut guardsize = 0;
    let mut isize = 0;
    if !mi_option_is_enabled(mi_option_secure) {
        isize =
            _mi_align_up_rs(minsize,
                            if 16 > MI_MAX_ALIGN_SIZE {
                                16
                            } else { MI_MAX_ALIGN_SIZE });
    } else {
        let mut page_size = _mi_os_page_size();
        isize = _mi_align_up_rs(minsize, page_size);
        guardsize = page_size;
        required = _mi_align_up_rs(required, page_size);
    }
    if !info_size.is_null() { *info_size = isize; }
    if !pre_size.is_null() { *pre_size = isize + guardsize; }
    return if required == 0 {
               MI_SEGMENT_SIZE
           } else { required + isize + 2 * guardsize };
}
/* -----------------------------------------------------------
Segment caches
We keep a small segment cache per thread to avoid repeated allocation
and free in the OS if a program allocates memory and then frees
all again repeatedly. (We tried a one-element cache but that
proves to be too small for certain workloads).
----------------------------------------------------------- */
unsafe fn mi_segments_count_add(mut inc: c_long,
                                mut tld: &mut mi_segments_tld_t) {
    if inc >= 0 {
        _mi_stat_increase(&mut (tld.stats.segments), inc);
    } else { _mi_stat_decrease(&mut (tld.stats.segments), -inc); }
    tld.count += inc;
    if tld.count > tld.peak { tld.peak = tld.count; };
}
unsafe fn mi_segments_peak(mut tld: &mut mi_segments_tld_t) -> usize {
    return tld.peak;
}
unsafe fn mi_segment_os_free(mut segment: *mut mi_segment_t,
                             mut segment_size: usize,
                             mut tld: &mut mi_segments_tld_t) {
    mi_segments_count_add(-1, tld);
    _mi_os_free(segment as *mut _, segment_size, tld.stats);
}
// The segment cache is limited to be at most 1/2 of the peak
// number of segments in use (and no more than 32)
unsafe fn mi_segment_cache_pop(mut tld: &mut mi_segments_tld_t)
 -> *mut mi_segment_t {
    let mut segment = tld.cache;
    if segment.is_null() { return ptr::null_mut(); }
    tld.cache_count -= 1;
    tld.cache = segment.next;
    segment.next = ptr::null_mut();
    return segment;
}
pub static MI_SEGMENT_CACHE_MAX: c_long = 16;
pub static MI_SEGMENT_CACHE_FRACTION: c_long = 6;
unsafe fn mi_segment_cache_full(mut tld: &mut mi_segments_tld_t) -> bool {
    if tld.cache_count < MI_SEGMENT_CACHE_MAX &&
           tld.cache_count * MI_SEGMENT_CACHE_FRACTION < mi_segments_peak(tld)
       {
        return false;
    }
    // take the opportunity to reduce the segment cache if it is too large (now)
    while tld.cache_count * MI_SEGMENT_CACHE_FRACTION >=
              mi_segments_peak(tld) + 1 {
        let mut segment = mi_segment_cache_pop(tld);
        if !segment.is_null() {
            mi_segment_os_free(segment, MI_SEGMENT_SIZE, tld);
        };
    }
    return true;
}
unsafe fn mi_segment_cache_push(mut segment: *mut mi_segment_t,
                                mut tld: &mut mi_segments_tld_t) -> bool {
    if mi_segment_cache_full(tld) { return false; }
    if mi_option_is_enabled(mi_option_cache_reset) != 0 &&
           !mi_option_is_enabled(mi_option_page_reset) {
        _mi_os_reset((segment as *mut u8).offset(segment.segment_info_size),
                     segment.segment_size - segment.segment_info_size);
    }
    segment.next = tld.cache;
    tld.cache = segment;
    tld.cache_count += 1;
    return true;
}
// called by ending threads to free cached segments
#[no_mangle]
pub unsafe extern "C" fn _mi_segment_thread_collect(mut tld:
                                                        *mut mi_segments_tld_t) {
    let mut segment: *mut mi_segment_t;
    while !{ segment = mi_segment_cache_pop(tld); segment }.is_null() {
        mi_segment_os_free(segment, MI_SEGMENT_SIZE, tld);
    };
}
/* -----------------------------------------------------------
   Segment allocation
----------------------------------------------------------- */
// Allocate a segment from the OS aligned to `MI_SEGMENT_SIZE` .
unsafe fn mi_segment_alloc(mut required: usize, mut page_kind: mi_page_kind_t,
                           mut page_shift: usize,
                           mut tld: &mut mi_segments_tld_t,
                           mut os_tld: *mut mi_os_tld_t)
 -> *mut mi_segment_t {
    // calculate needed sizes first
    let mut capacity: usize;
    if page_kind == MI_PAGE_HUGE {
        capacity = 1;
    } else {
        let mut page_size = 1 << page_shift;
        capacity = MI_SEGMENT_SIZE / page_size;
    }
    let mut info_size: usize;
    let mut pre_size: usize;
    let mut segment_size =
        mi_segment_size(capacity, required, &mut pre_size, &mut info_size);
    let mut page_size =
        if page_kind == MI_PAGE_HUGE {
            segment_size
        } else { 1 << page_shift };
    // Allocate the segment
    let mut segment = ptr::null_mut();
    // try to get it from our caches
    if segment_size == MI_SEGMENT_SIZE {
        segment = mi_segment_cache_pop(tld);
        if !segment.is_null() && mi_option_is_enabled(mi_option_secure) != 0
               && segment.page_kind != page_kind {
            _mi_os_unprotect(segment as *mut _, segment.segment_size);
        };
    }
    // and otherwise allocate it from the OS
    if segment.is_null() {
        segment =
            _mi_os_alloc_aligned(segment_size, MI_SEGMENT_SIZE, os_tld) as
                *mut mi_segment_t; // in secure mode, we set up a protected page in between the segment info
        if segment.is_null() {
            return ptr::null_mut(); // and the page data
        } // and protect the last page too      
        mi_segments_count_add(1, tld); // protect every page
    }
    memset(segment as *mut _, 0, info_size);
    if mi_option_is_enabled(mi_option_secure) {
        _mi_os_protect((segment as *mut u8).offset(info_size),
                       (pre_size - info_size));
        let mut os_page_size = _mi_os_page_size();
        if mi_option_get(mi_option_secure) <= 1 {
            _mi_os_protect((segment as
                                *mut u8).offset(segment_size).offset(-os_page_size),
                           os_page_size);
        } else {
            for mut i in 0..capacity {
                _mi_os_protect((segment as
                                    *mut u8).offset((i + 1) *
                                                        page_size).offset(-os_page_size),
                               os_page_size);
            };
        };
    }
    segment.page_kind = page_kind;
    segment.capacity = capacity;
    segment.page_shift = page_shift;
    segment.segment_size = segment_size;
    segment.segment_info_size = pre_size;
    segment.thread_id = _mi_thread_id();
    segment.cookie = _mi_ptr_cookie(segment as *const _);
    for mut i in 0..segment.capacity { (segment.pages[i]).segment_idx = i; }
    _mi_stat_increase(&mut (tld.stats.committed), segment.segment_info_size);
    //fprintf(stderr,"mimalloc: alloc segment at %p\n", (void*)segment);
    return segment;
}
// Available memory in a page
unsafe fn mi_page_size(mut page: *const mi_page_t) -> usize {
    let mut psize: usize;
    _mi_segment_page_start(_mi_page_segment(page), page, &mut psize);
    return psize;
}
unsafe fn mi_segment_free(mut segment: *mut mi_segment_t, mut force: bool,
                          mut tld: &mut mi_segments_tld_t) {
    //fprintf(stderr,"mimalloc: free segment at %p\n", (void*)segment);
    if !segment.is_null() {
        0
    } else {
        _mi_assert_fail("segment != NULL", "src/segment.c", 343,
                        "mi_segment_free")
    }
    if mi_segment_is_in_free_queue(segment, tld) {
        if segment.page_kind != MI_PAGE_SMALL {
            fprintf(stderr,
                    "mimalloc: expecting small segment: %i, %p, %p, %p\n",
                    segment.page_kind, segment.prev, segment.next,
                    tld.small_free.first);
            fflush(stderr);
        } else {
            // for now we only support small pages
            mi_segment_queue_remove(&mut tld.small_free, segment);
        };
    }
    if segment.next.is_null() {
        0
    } else {
        _mi_assert_fail("segment->next == NULL", "src/segment.c", 356,
                        "mi_segment_free")
    }
    if segment.prev.is_null() {
        0
    } else {
        _mi_assert_fail("segment->prev == NULL", "src/segment.c", 357,
                        "mi_segment_free")
    }
    _mi_stat_decrease(&mut (tld.stats.committed), segment.segment_info_size);
    segment.thread_id = 0;
    // update reset memory statistics
    for mut i in 0..segment.capacity {
        let mut page = &mut segment.pages[i]; // it is put in our cache
        if page.is_reset != 0 {
            page.is_reset = false; // otherwise return it to the OS
            _mi_stat_decrease(&mut (tld.stats.reset), mi_page_size(page));
        };
    }
    if segment.page_kind == MI_PAGE_HUGE {
        mi_segment_os_free(segment, segment.segment_size, tld);
    } else if force == 0 && mi_segment_cache_push(segment, tld) != 0 {
    } else { mi_segment_os_free(segment, MI_SEGMENT_SIZE, tld); };
}
/* -----------------------------------------------------------
  Free page management inside a segment
----------------------------------------------------------- */
unsafe fn mi_segment_has_free(mut segment: &mi_segment_t) -> bool {
    return (segment.used < segment.capacity);
}
unsafe fn mi_segment_find_free(mut segment: &mut mi_segment_t)
 -> *mut mi_page_t {
    for mut i in 0..segment.capacity {
        let mut page = &mut segment.pages[i];
        if page.segment_in_use == 0 { return page; };
    }
    if (false) != 0 {
        0
    } else {
        _mi_assert_fail("false", "src/segment.c", 403, "mi_segment_find_free")
    }
    return ptr::null_mut();
}
/* -----------------------------------------------------------
   Free
----------------------------------------------------------- */
unsafe fn mi_segment_page_clear(mut segment: &mut mi_segment_t,
                                mut page: &mut mi_page_t,
                                mut stats: &mut mi_stats_t) {
    (stats);
    let mut inuse = page.capacity * page.block_size;
    _mi_stat_decrease(&mut (stats.committed), inuse);
    _mi_stat_decrease(&mut (stats.pages), 1);
    // reset the page memory to reduce memory pressure?
    if page.is_reset == 0 && mi_option_is_enabled(mi_option_page_reset) != 0 {
        let mut psize: usize; // for stats we assume resetting the full page
        let mut start = _mi_segment_page_start(segment, page, &mut psize);
        _mi_stat_increase(&mut (stats.reset), psize);
        page.is_reset = true;
        if inuse > 0 { _mi_os_reset(start as *mut _, inuse); };
    }
    // zero the page data
    let mut idx = page.segment_idx; // don't clear the index
    let mut is_reset = page.is_reset != 0; // don't clear the reset flag
    memset(page as *mut _, 0, std::mem::size_of::<mi_page_t>());
    page.segment_idx = idx;
    page.segment_in_use = false;
    page.is_reset = is_reset != 0;
    segment.used -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn _mi_segment_page_free(mut page: *mut mi_page_t,
                                               mut force: bool,
                                               mut tld:
                                                   &mut mi_segments_tld_t) {
    if !page.is_null() {
        0
    } else {
        _mi_assert_fail("page != NULL", "src/segment.c", 445,
                        "_mi_segment_page_free")
    }
    let mut segment = _mi_page_segment(page);
    // mark it as free now
    mi_segment_page_clear(segment, page,
                          tld.stats); // no more used pages; remove from the free list and free the segment
    if segment.used == 0 {
        mi_segment_free(segment, force != 0,
                        tld); // only abandoned pages; remove from free list and abandon
    } else {
        if segment.used == segment.abandoned {
            mi_segment_abandon(segment, tld);
        } else if segment.used + 1 == segment.capacity {
            // for now we only support small pages
            // move back to segments small pages free list
            mi_segment_enqueue(&mut tld.small_free, segment);
        };
    };
}
/* -----------------------------------------------------------
   Abandonment
----------------------------------------------------------- */
// When threads terminate, they can leave segments with
// live blocks (reached through other threads). Such segments
// are "abandoned" and will be reclaimed by other threads to
// reuse their pages and/or free them eventually
pub static mut abandoned: *mut volatile_mi_segment_t = ptr::null_mut();
pub static mut abandoned_count: volatile_uintptr_t = 0;
unsafe fn mi_segment_abandon(mut segment: &mut mi_segment_t,
                             mut tld: &mut mi_segments_tld_t) {
    // remove the segment from the free page queue if needed
    if mi_segment_is_in_free_queue(segment, tld) {
        if segment.page_kind == MI_PAGE_SMALL {
            0
        } else {
            _mi_assert_fail("segment->page_kind == MI_PAGE_SMALL",
                            "src/segment.c", 488, "mi_segment_abandon")
        } // for now we only support small pages
        mi_segment_queue_remove(&mut tld.small_free, segment);
    }
    // all pages in the segment are abandoned; add it to the abandoned list
    segment.thread_id =
        0; // all pages are abandoned, abandon the entire segment
    loop  {
        segment.abandoned_next =
            abandoned as *mut mi_segment_t; // close enough
        if !!mi_atomic_compare_exchange_ptr(&mut abandoned as
                                                *mut *mut c_void,
                                            segment as *mut _,
                                            segment.abandoned_next as *mut _)
           {
            break
        }; // at most 1/8th of all outstanding (estimated)
    } // but at least 8
    mi_atomic_increment(&mut abandoned_count);
    _mi_stat_increase(&mut (tld.stats.segments_abandoned), 1);
}
#[no_mangle]
pub unsafe extern "C" fn _mi_segment_page_abandon(mut page: *mut mi_page_t,
                                                  mut tld:
                                                      &mut mi_segments_tld_t) {
    if !page.is_null() {
        0
    } else {
        _mi_assert_fail("page != NULL", "src/segment.c", 503,
                        "_mi_segment_page_abandon")
    }
    let mut segment = _mi_page_segment(page);
    segment.abandoned += 1;
    _mi_stat_increase(&mut (tld.stats.pages_abandoned), 1);
    if segment.used == segment.abandoned {
        mi_segment_abandon(segment, tld);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mi_segment_try_reclaim_abandoned(mut heap:
                                                               *mut mi_heap_t,
                                                           mut try_all: bool,
                                                           mut tld:
                                                               &mut mi_segments_tld_t)
 -> bool {
    let mut reclaimed = 0;
    let mut atmost: usize;
    if try_all != 0 {
        atmost = abandoned_count + 16;
    } else { atmost = abandoned_count / 8; if atmost < 8 { atmost = 8; }; }
    // for `atmost` `reclaimed` abandoned segments...
    while atmost > reclaimed
          { // try to claim the head of the abandoned segments
        let mut segment:
                *mut mi_segment_t; // stop early if no more segments available
        loop  {
            segment = abandoned as *mut mi_segment_t; // got it.
            if !(!segment.is_null() &&
                     !mi_atomic_compare_exchange_ptr(&mut abandoned as
                                                         *mut *mut c_void,
                                                     segment.abandoned_next as
                                                         *mut _,
                                                     segment as *mut _)) {
                break
            }; // add its free pages to the the current thread
        } // add its abandoned pages to the current thread
        if segment.is_null() {
            break ; // if everything free by now, free the page
        } // otherwise reclaim it
        mi_atomic_decrement(&mut abandoned_count); // due to page_clear
        segment.thread_id = _mi_thread_id();
        segment.abandoned_next = ptr::null_mut();
        mi_segments_count_add(1, tld);
        _mi_stat_decrease(&mut (tld.stats.segments_abandoned), 1);
        if segment.page_kind == MI_PAGE_SMALL &&
               mi_segment_has_free(segment) != 0 {
            mi_segment_enqueue(&mut tld.small_free, segment);
        }
        if segment.abandoned == segment.used {
            0
        } else {
            _mi_assert_fail("segment->abandoned == segment->used",
                            "src/segment.c", 548,
                            "_mi_segment_try_reclaim_abandoned")
        }
        for mut i in 0..segment.capacity {
            let mut page = &mut segment.pages[i];
            if page.segment_in_use != 0 {
                segment.abandoned -= 1;
                if page.next.is_null() {
                    0
                } else {
                    _mi_assert_fail("page->next == NULL", "src/segment.c",
                                    553, "_mi_segment_try_reclaim_abandoned")
                }
                _mi_stat_decrease(&mut (tld.stats.pages_abandoned), 1);
                if mi_page_all_free(page) {
                    mi_segment_page_clear(segment, page, tld.stats);
                } else { _mi_page_reclaim(heap, page); };
            };
        }
        if segment.abandoned == 0 {
            0
        } else {
            _mi_assert_fail("segment->abandoned == 0", "src/segment.c", 565,
                            "_mi_segment_try_reclaim_abandoned")
        }
        if segment.used == 0 {
            mi_segment_free(segment, false, tld);
        } else { reclaimed += 1; };
    }
    return (reclaimed > 0);
}
/* -----------------------------------------------------------
   Small page allocation
----------------------------------------------------------- */
// Allocate a small page inside a segment.
// Requires that the page has free pages
unsafe fn mi_segment_small_page_alloc_in(mut segment: &mut mi_segment_t,
                                         mut tld: &mut mi_segments_tld_t)
 -> *mut mi_page_t {
    let mut page =
        mi_segment_find_free(segment); // if no more free pages, remove from the queue
    page.segment_in_use = true;
    segment.used += 1;
    if segment.used == segment.capacity {
        mi_segment_queue_remove(&mut tld.small_free, segment);
    }
    return page;
}
pub static MI_SMALL_PAGE_SHIFT: usize = (13 + 3);
unsafe fn mi_segment_small_page_alloc(mut tld: &mut mi_segments_tld_t,
                                      mut os_tld: *mut mi_os_tld_t)
 -> *mut mi_page_t {
    if mi_segment_queue_is_empty(&mut tld.small_free) {
        let mut segment =
            mi_segment_alloc(0, MI_PAGE_SMALL, MI_SMALL_PAGE_SHIFT, tld,
                             os_tld);
        if segment.is_null() { return ptr::null_mut(); }
        mi_segment_enqueue(&mut tld.small_free, segment);
    }
    return mi_segment_small_page_alloc_in(tld.small_free.first, tld);
}
/* -----------------------------------------------------------
   large page allocation
----------------------------------------------------------- */
pub static MI_LARGE_PAGE_SHIFT: usize = (6 + (13 + 3));
unsafe fn mi_segment_large_page_alloc(mut tld: *mut mi_segments_tld_t,
                                      mut os_tld: *mut mi_os_tld_t)
 -> *mut mi_page_t {
    let mut segment =
        mi_segment_alloc(0, MI_PAGE_LARGE, MI_LARGE_PAGE_SHIFT, tld, os_tld);
    if segment.is_null() { return ptr::null_mut(); }
    segment.used = 1;
    let mut page = &mut segment.pages[0];
    page.segment_in_use = true;
    return page;
}
pub static MI_SEGMENT_SHIFT: usize = (6 + (13 + 3));
unsafe fn mi_segment_huge_page_alloc(mut size: usize,
                                     mut tld: *mut mi_segments_tld_t,
                                     mut os_tld: *mut mi_os_tld_t)
 -> *mut mi_page_t {
    let mut segment =
        mi_segment_alloc(size, MI_PAGE_HUGE, MI_SEGMENT_SHIFT, tld, os_tld);
    if segment.is_null() { return ptr::null_mut(); }
    segment.used = 1;
    let mut page = &mut segment.pages[0];
    page.segment_in_use = true;
    return page;
}
/* -----------------------------------------------------------
   Page allocation and free
----------------------------------------------------------- */
pub static MI_SMALL_PAGE_SIZE: c_int = 1 << (13 + 3);
// smaller blocks than 8kb (assuming MI_SMALL_PAGE_SIZE == 64kb)
pub static MI_LARGE_SIZE_MAX: c_long = ((1 << (6 + (13 + 3))) / 8);
#[no_mangle]
pub unsafe extern "C" fn _mi_segment_page_alloc(mut block_size: usize,
                                                mut tld:
                                                    *mut mi_segments_tld_t,
                                                mut os_tld: *mut mi_os_tld_t)
 -> *mut mi_page_t {
    let mut page: *mut mi_page_t;
    if block_size < MI_SMALL_PAGE_SIZE / 8 {
        page = mi_segment_small_page_alloc(tld, os_tld);
    } else if block_size <
                  (MI_LARGE_SIZE_MAX - std::mem::size_of::<mi_segment_t>()) {
        page = mi_segment_large_page_alloc(tld, os_tld);
    } else { page = mi_segment_huge_page_alloc(block_size, tld, os_tld); }
    return page;
}
