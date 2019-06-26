/* ----------------------------------------------------------------------------
Copyright (c) 2018, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
// memset
// ------------------------------------------------------
// Allocation
// ------------------------------------------------------
// Fast allocation in a page: just pop from the free list.
// Fall back to generic allocation only if the list is empty.
// slow path
// pop from the free list
pub static MI_DEBUG_UNINIT: c_int = 208;
pub static MI_LARGE_SIZE_MAX: c_long = ((1 << (6 + (13 + 3))) / 8);
#[no_mangle]
pub unsafe extern "C" fn _mi_page_malloc(mut heap: &mut mi_heap_t,
                                         mut page: &mut mi_page_t,
                                         mut size: usize) -> *mut c_void {
    let mut block = page.free;
    if __builtin_expect((block.is_null()), 0) != 0 {
        return _mi_malloc_generic(heap, size);
    }
    page.free = mi_block_next(page, block);
    page.used += 1;
    memset(block as *mut _, MI_DEBUG_UNINIT, size);
    if size <= MI_LARGE_SIZE_MAX {
        _mi_stat_increase(&mut ((heap).tld.stats.normal[_mi_bin(size)]), 1);
    }
    return block as *mut _;
}
// allocate a small block
pub static MI_SMALL_SIZE_MAX: c_long =
    128 * std::mem::size_of::<*mut c_void>();
#[no_mangle]
pub unsafe extern "C" fn mi_heap_malloc_small(mut heap: *mut mi_heap_t,
                                              mut size: usize)
 -> *mut c_void {
    if size <= MI_SMALL_SIZE_MAX {
        0
    } else {
        _mi_assert_fail("size <= MI_SMALL_SIZE_MAX", "src/alloc.c", 48,
                        "mi_heap_malloc_small")
    }
    let mut page = _mi_heap_get_free_small_page(heap, size);
    return _mi_page_malloc(heap, page, size);
}
#[no_mangle]
pub unsafe extern "C" fn mi_malloc_small(mut size: usize) -> *mut c_void {
    return mi_heap_malloc_small(mi_get_default_heap(), size);
}
// zero initialized small block
#[no_mangle]
pub unsafe extern "C" fn mi_zalloc_small(mut size: usize) -> *mut c_void {
    let mut p = mi_malloc_small(size);
    if !p.is_null() { memset(p, 0, size); }
    return p;
}
// The main allocation function
#[no_mangle]
pub unsafe extern "C" fn mi_heap_malloc(mut heap: *mut mi_heap_t,
                                        mut size: usize) -> *mut c_void {
    if !heap.is_null() {
        0
    } else {
        _mi_assert_fail("heap!=NULL", "src/alloc.c", 66, "mi_heap_malloc")
    } // heaps are thread local
    if heap.thread_id == 0 || heap.thread_id == _mi_thread_id() {
        0
    } else {
        _mi_assert_fail("heap->thread_id == 0 || heap->thread_id == _mi_thread_id()",
                        "src/alloc.c", 67, "mi_heap_malloc")
    } // overestimate for aligned sizes
    let mut p: *mut c_void;
    if __builtin_expect((size <= MI_SMALL_SIZE_MAX), 1) != 0 {
        p = mi_heap_malloc_small(heap, size);
    } else { p = _mi_malloc_generic(heap, size); }
    if !p.is_null() {
        if !mi_heap_is_initialized(heap) { heap = mi_get_default_heap(); }
        _mi_stat_increase(&mut ((heap).tld.stats.malloc), mi_good_size(size));
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn mi_malloc(mut size: usize) -> *mut c_void {
    return mi_heap_malloc(mi_get_default_heap(), size);
}
#[no_mangle]
pub unsafe extern "C" fn _mi_heap_malloc_zero(mut heap: *mut mi_heap_t,
                                              mut size: usize, mut zero: bool)
 -> *mut c_void {
    let mut p = mi_heap_malloc(heap, size);
    if zero != 0 != 0 && !p.is_null() { memset(p, 0, size); }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn mi_heap_zalloc(mut heap: *mut mi_heap_t,
                                        mut size: usize) -> *mut c_void {
    return _mi_heap_malloc_zero(heap, size, true);
}
#[no_mangle]
pub unsafe extern "C" fn mi_zalloc(mut size: usize) -> *mut c_void {
    return mi_heap_zalloc(mi_get_default_heap(), size);
}
// ------------------------------------------------------
// Free
// ------------------------------------------------------
// multi-threaded free
// unlikely: this only happens on the first concurrent free in a page that is in the full list
// usual: directly add to page thread_free list
pub static MI_TF_PTR_SHIFT: c_int = 2;
unsafe fn _mi_free_block_mt(mut page: &mut mi_page_t,
                            mut block: *mut mi_block_t) {
    let mut tfree:
            mi_thread_free_t; // increment the thread free count and return
    let mut tfreex: mi_thread_free_t;
    let mut use_delayed: bool;
    loop  {
        tfreex = { tfree = page.thread_free; tfree };
        use_delayed = (tfree.delayed == MI_USE_DELAYED_FREE);
        if __builtin_expect((use_delayed) != 0, 0) != 0 {
            tfreex.delayed = MI_DELAYED_FREEING;
        } else {
            mi_block_set_next(page, block,
                              (tfree.head << MI_TF_PTR_SHIFT) as
                                  *mut mi_block_t);
            tfreex.head = (block as usize) >> MI_TF_PTR_SHIFT;
        }
        if !!mi_atomic_compare_exchange(&mut page.thread_free as
                                            *mut volatile_uintptr_t,
                                        tfreex.value, tfree.value) {
            break
        };
    }
    if __builtin_expect((use_delayed == 0), 1) != 0 {
        mi_atomic_increment(&mut page.thread_freed);
    } else {
        // racy read on `heap`, but ok because MI_DELAYED_FREEING is set (see `mi_heap_delete` and `mi_heap_collect_abandon`)
        let mut heap = page.heap;
        if !heap.is_null() {
            // add to the delayed free list of this heap. (do this atomically as the lock only protects heap memory validity)
            let mut dfree:
                    *mut mi_block_t; // and reset the MI_DELAYED_FREEING flag
            loop  {
                dfree = heap.thread_delayed_free as *mut mi_block_t;
                mi_block_set_nextx(heap.cookie, block, dfree);
                if !!mi_atomic_compare_exchange_ptr(&mut heap.thread_delayed_free
                                                        as *mut *mut c_void,
                                                    block as *mut _,
                                                    dfree as *mut _) {
                    break
                };
            };
        }
        loop  {
            tfreex = { tfree = page.thread_free; tfree };
            tfreex.delayed = MI_NO_DELAYED_FREE;
            if !!mi_atomic_compare_exchange(&mut page.thread_free as
                                                *mut volatile_uintptr_t,
                                            tfreex.value, tfree.value) {
                break
            };
        };
    };
}
// regular free
pub static MI_DEBUG_FREED: c_int = 223;
unsafe fn _mi_free_block(mut page: &mut mi_page_t, mut local: bool,
                         mut block: *mut mi_block_t) {
    memset(block as *mut _, MI_DEBUG_FREED, page.block_size);
    // and push it on the free list
    if __builtin_expect((local) != 0, 1) != 0
       { // owning thread can free a block directly
        mi_block_set_next(page, block, page.local_free);
        page.local_free = block;
        page.used -= 1;
        if __builtin_expect(mi_page_all_free(page), 0) != 0 {
            _mi_page_retire(page);
        } else if __builtin_expect((page.flags.in_full) != 0, 0) != 0 {
            _mi_page_unfull(page);
        };
    } else { _mi_free_block_mt(page, block); };
}
// Adjust a block that was allocated aligned, to the actual start of the block in the page.
#[no_mangle]
pub unsafe extern "C" fn _mi_page_ptr_unalign(mut segment:
                                                  *const mi_segment_t,
                                              mut page: &mi_page_t,
                                              mut p: *mut c_void)
 -> *mut mi_block_t {
    let mut diff =
        (p as
             *mut u8).offset(-_mi_page_start(segment, page, ptr::null_mut()));
    let mut adjust = (diff % page.block_size);
    return ((p as usize) - adjust) as *mut mi_block_t;
}
unsafe fn mi_free_generic(mut segment: *const mi_segment_t,
                          mut page: &mut mi_page_t, mut local: bool,
                          mut p: *mut c_void) {
    let mut block =
        if page.flags.has_aligned != 0 != 0 {
            _mi_page_ptr_unalign(segment, page, p)
        } else { p as *mut mi_block_t };
    _mi_free_block(page, local != 0, block);
}
// Free a block
// optimize: merge null check with the segment masking (below)
//if (p == NULL) return;
pub static MI_INTPTR_SIZE: c_int = 1 << 3;
#[no_mangle]
pub unsafe extern "C" fn mi_free(mut p: *mut c_void) {
    if __builtin_expect((((p as usize) & (MI_INTPTR_SIZE - 1)) != 0), 0) != 0
       {
        _mi_error_message("trying to free an invalid (unaligned) pointer: %p\n",
                          p); // checks for (p==NULL)
        return;
    }
    let segment = _mi_ptr_segment(p as *const c_void);
    if segment.is_null() { return; }
    let mut local = (_mi_thread_id() == segment.thread_id);
    // preload, note: putting the thread_id in the page->flags does not improve performance
    if __builtin_expect((_mi_ptr_cookie(segment as *const _) !=
                             segment.cookie), 0) != 0 {
        _mi_error_message("trying to mi_free a pointer that does not point to a valid heap space: %p\n",
                          p);
        return;
    }
    let mut page = _mi_segment_page_of(segment, p as *const c_void);
    let mut heap = mi_heap_get_default();
    _mi_stat_decrease(&mut ((heap).tld.stats.malloc), mi_usable_size(p));
    if page.block_size <= MI_LARGE_SIZE_MAX {
        _mi_stat_decrease(&mut ((heap).tld.stats.normal[_mi_bin(page.block_size)]),
                          1);
    }
    // huge page stat is accounted for in `_mi_page_retire`
    // adjust if it might be an un-aligned block
    if __builtin_expect((page.flags.value == 0), 1) != 0 {
        // note: merging both tests (local | value) does not matter for performance
        let mut block =
            p as *mut mi_block_t; // owning thread can free a block directly
        if __builtin_expect((local) != 0, 1) != 0 {
            mi_block_set_next(page, block, page.local_free);
            // note: moving this write earlier does not matter for performance
            page.local_free =
                block; // use atomic operations for a multi-threaded free
            page.used -=
                1; // aligned blocks, or a full page; use the more generic path
            if __builtin_expect(mi_page_all_free(page), 0) != 0 {
                _mi_page_retire(page);
            };
        } else { _mi_free_block_mt(page, block); };
    } else { mi_free_generic(segment, page, local != 0, p); };
}
#[no_mangle]
pub unsafe extern "C" fn _mi_free_delayed_block(mut block: *mut mi_block_t) {
    let mut segment = _mi_ptr_segment(block as *const _);
    let mut page = _mi_segment_page_of(segment, block as *const _);
    _mi_free_block(page, true, block);
}
// Bytes available in a block
#[no_mangle]
pub unsafe extern "C" fn mi_usable_size(mut p: *mut c_void) -> usize {
    if p.is_null() { return 0; }
    let mut segment = _mi_ptr_segment(p as *const c_void);
    let mut page = _mi_segment_page_of(segment, p as *const c_void);
    let mut size = page.block_size;
    if __builtin_expect((page.flags.has_aligned) != 0, 0) != 0 {
        let mut adjust =
            (p as
                 *mut u8).offset(-(_mi_page_ptr_unalign(segment, page, p) as
                                       *mut u8));
        return (size - adjust);
    } else { return size; };
}
// ------------------------------------------------------
// ensure explicit external inline definitions are emitted!
// ------------------------------------------------------
// ------------------------------------------------------
// Allocation extensions
// ------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn mi_heap_calloc(mut heap: *mut mi_heap_t,
                                        mut count: usize, mut size: usize)
 -> *mut c_void {
    let mut total: usize;
    if mi_mul_overflow(count, size, &mut total) { return ptr::null_mut(); }
    return mi_heap_zalloc(heap, total);
}
#[no_mangle]
pub unsafe extern "C" fn mi_calloc(mut count: usize, mut size: usize)
 -> *mut c_void {
    return mi_heap_calloc(mi_get_default_heap(), count, size);
}
// Uninitialized `calloc`
#[no_mangle]
pub unsafe extern "C" fn mi_heap_mallocn(mut heap: *mut mi_heap_t,
                                         mut count: usize, mut size: usize)
 -> *mut c_void {
    let mut total: usize;
    if mi_mul_overflow(count, size, &mut total) { return ptr::null_mut(); }
    return mi_heap_malloc(heap, total);
}
#[no_mangle]
pub unsafe extern "C" fn mi_mallocn(mut count: usize, mut size: usize)
 -> *mut c_void {
    return mi_heap_mallocn(mi_get_default_heap(), count, size);
}
// Expand in place or fail
#[no_mangle]
pub unsafe extern "C" fn mi_expand(mut p: *mut c_void, mut newsize: usize)
 -> *mut c_void {
    if p.is_null() {
        return ptr::null_mut(); // it fits
    } // reallocation still fits and not more than 50% waste
    let mut size = mi_usable_size(p); // maybe in another heap
    if newsize > size { return ptr::null_mut(); }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn _mi_realloc_zero(mut p: *mut c_void,
                                          mut newsize: usize, mut zero: bool)
 -> *mut c_void {
    if p.is_null() {
        return _mi_heap_malloc_zero(mi_get_default_heap(), newsize,
                                    zero != 0);
    }
    let mut size = mi_usable_size(p);
    if newsize <= size && newsize >= (size / 2) { return p; }
    let mut newp = mi_malloc(newsize);
    if __builtin_expect((!newp.is_null()), 1) != 0 {
        if zero != 0 != 0 && newsize > size {
            // also set last word in the previous allocation to zero to ensure any padding is zero-initialized
            let mut start =
                if size >= std::mem::size_of::<isize>() {
                    size - std::mem::size_of::<isize>()
                } else { 0 }; // only free if successful
            memset((newp as *mut u8).offset(start), 0, newsize - start);
        }
        memcpy(newp, p as *const c_void,
               if newsize > size { size } else { newsize });
        mi_free(p);
    }
    return newp;
}
#[no_mangle]
pub unsafe extern "C" fn mi_realloc(mut p: *mut c_void, mut newsize: usize)
 -> *mut c_void {
    return _mi_realloc_zero(p, newsize, false);
}
// Zero initialized reallocation
#[no_mangle]
pub unsafe extern "C" fn mi_rezalloc(mut p: *mut c_void, mut newsize: usize)
 -> *mut c_void {
    return _mi_realloc_zero(p, newsize, true);
}
#[no_mangle]
pub unsafe extern "C" fn mi_recalloc(mut p: *mut c_void, mut count: usize,
                                     mut size: usize) -> *mut c_void {
    let mut total: usize;
    if mi_mul_overflow(count, size, &mut total) { return ptr::null_mut(); }
    return mi_rezalloc(p, total);
}
#[no_mangle]
pub unsafe extern "C" fn mi_reallocn(mut p: *mut c_void, mut count: usize,
                                     mut size: usize) -> *mut c_void {
    let mut total: usize;
    if mi_mul_overflow(count, size, &mut total) { return ptr::null_mut(); }
    return mi_realloc(p, total);
}
// Reallocate but free `p` on errors
#[no_mangle]
pub unsafe extern "C" fn mi_reallocf(mut p: *mut c_void, mut newsize: usize)
 -> *mut c_void {
    let mut newp = mi_realloc(p, newsize);
    if newp.is_null() && !p.is_null() { mi_free(p); }
    return newp;
}
// `strdup` using mi_malloc
#[no_mangle]
pub unsafe extern "C" fn mi_heap_strdup(mut heap: *mut mi_heap_t,
                                        mut s: *const i8) -> *mut i8 {
    if s.is_null() { return ptr::null_mut(); }
    let mut n = strlen(s);
    let mut t = mi_heap_malloc(heap, n + 1) as *mut i8;
    if !t.is_null() { memcpy(t as *mut _, s as *const _, n + 1); }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn mi_strdup(mut s: *const i8) -> *mut i8 {
    return mi_heap_strdup(mi_get_default_heap(), s);
}
// `strndup` using mi_malloc
#[no_mangle]
pub unsafe extern "C" fn mi_heap_strndup(mut heap: *mut mi_heap_t,
                                         mut s: *const i8, mut n: usize)
 -> *mut i8 {
    if s.is_null() { return ptr::null_mut(); }
    let mut m = strlen(s);
    if n > m { n = m; }
    let mut t = mi_heap_malloc(heap, n + 1) as *mut i8;
    if t.is_null() { return ptr::null_mut(); }
    memcpy(t as *mut _, s as *const _, n);
    *t.offset(n) = 0i8;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn mi_strndup(mut s: *const i8, mut n: usize)
 -> *mut i8 {
    return mi_heap_strndup(mi_get_default_heap(), s, n);
}
// `realpath` using mi_malloc
// todo: use GetFullPathNameW to allow longer file names
pub static PATH_MAX: usize = 4096;
#[no_mangle]
pub unsafe extern "C" fn mi_heap_realpath(mut heap: *mut mi_heap_t,
                                          mut fname: *const i8,
                                          mut resolved_name: *mut i8)
 -> *mut i8 {
    if !resolved_name.is_null() {
        return realpath(fname, resolved_name); // ok if `rname==NULL`
    } else {
        let mut buf: [i8; 4097];
        let mut rname = realpath(fname, buf);
        return mi_heap_strndup(heap, rname, PATH_MAX);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mi_realpath(mut fname: *const i8,
                                     mut resolved_name: *mut i8) -> *mut i8 {
    return mi_heap_realpath(mi_get_default_heap(), fname, resolved_name);
}
