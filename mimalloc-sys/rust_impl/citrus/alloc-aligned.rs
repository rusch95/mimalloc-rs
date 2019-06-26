/* ----------------------------------------------------------------------------
Copyright (c) 2018, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
// memset
// ------------------------------------------------------
// Aligned Allocation
// ------------------------------------------------------
// note: we don't require `size > offset`, we just guarantee that
// the address at offset is aligned regardless of the allocated size.
pub static SIZE_MAX: c_long = 18446744073709551615;
// overflow
// try if there is a current small block with just the right alignment
pub static MI_SMALL_SIZE_MAX: c_long =
    128 * std::mem::size_of::<*mut c_void>();
unsafe fn mi_heap_malloc_zero_aligned_at(mut heap: &mut mi_heap_t,
                                         mut size: usize,
                                         mut alignment: usize,
                                         mut offset: usize, mut zero: bool)
 -> *mut c_void {
    if alignment > 0 {
        0
    } else {
        _mi_assert_fail("alignment > 0", "src/alloc-aligned.c", 21,
                        "mi_heap_malloc_zero_aligned_at")
    }
    if alignment <= std::mem::size_of::<usize>() {
        return _mi_heap_malloc_zero(heap, size, zero != 0);
    }
    if size >= (SIZE_MAX - alignment) { return ptr::null_mut(); }
    if size <= MI_SMALL_SIZE_MAX {
        let mut page = _mi_heap_get_free_small_page(heap, size);
        if !page.free.is_null() &&
               (((page.free as usize) + offset) % alignment) == 0 {
            _mi_stat_increase(&mut ((heap).tld.stats.malloc), size);
            let mut p = _mi_page_malloc(heap, page, size);
            if zero != 0 { memset(p, 0, size); }
            return p;
        };
    }
    // otherwise over-allocate
    let mut p = _mi_heap_malloc_zero(heap, size + alignment - 1, zero != 0);
    if p.is_null() { return ptr::null_mut(); }
    // .. and align within the allocation
    _mi_ptr_page(p).flags.has_aligned =
        true; // reallocation still fits, is aligned and not more than 50% waste
    let mut adjust = alignment - (((p as usize) + offset) % alignment);
    let mut aligned_p =
        if adjust == alignment {
            p
        } else { ((p as usize) + adjust) as *mut c_void };
    return aligned_p;
}
unsafe fn mi_malloc_zero_aligned_at(mut size: usize, mut alignment: usize,
                                    mut offset: usize, mut zero: bool)
 -> *mut c_void {
    return mi_heap_malloc_zero_aligned_at(mi_get_default_heap(), size,
                                          alignment, offset, zero != 0);
}
#[no_mangle]
pub unsafe extern "C" fn mi_malloc_aligned_at(mut size: usize,
                                              mut alignment: usize,
                                              mut offset: usize)
 -> *mut c_void {
    return mi_malloc_zero_aligned_at(size, alignment, offset, false);
}
#[no_mangle]
pub unsafe extern "C" fn mi_malloc_aligned(mut size: usize,
                                           mut alignment: usize)
 -> *mut c_void {
    return mi_malloc_aligned_at(size, alignment, 0);
}
#[no_mangle]
pub unsafe extern "C" fn mi_zalloc_aligned_at(mut size: usize,
                                              mut alignment: usize,
                                              mut offset: usize)
 -> *mut c_void {
    return mi_malloc_zero_aligned_at(size, alignment, offset, true);
}
#[no_mangle]
pub unsafe extern "C" fn mi_zalloc_aligned(mut size: usize,
                                           mut alignment: usize)
 -> *mut c_void {
    return mi_zalloc_aligned_at(size, alignment, 0);
}
#[no_mangle]
pub unsafe extern "C" fn mi_calloc_aligned_at(mut count: usize,
                                              mut size: usize,
                                              mut alignment: usize,
                                              mut offset: usize)
 -> *mut c_void {
    let mut total: usize;
    if mi_mul_overflow(count, size, &mut total) { return ptr::null_mut(); }
    return mi_zalloc_aligned_at(total, alignment, offset);
}
#[no_mangle]
pub unsafe extern "C" fn mi_calloc_aligned(mut count: usize, mut size: usize,
                                           mut alignment: usize)
 -> *mut c_void {
    let mut total: usize;
    if mi_mul_overflow(count, size, &mut total) { return ptr::null_mut(); }
    return mi_zalloc_aligned(total, alignment);
}
unsafe fn mi_realloc_zero_aligned_at(mut p: *mut c_void, mut newsize: usize,
                                     mut alignment: usize, mut offset: usize,
                                     mut zero: bool) -> *mut c_void {
    if alignment > 0 {
        0
    } else {
        _mi_assert_fail("alignment > 0", "src/alloc-aligned.c", 90,
                        "mi_realloc_zero_aligned_at")
    }
    if alignment <= std::mem::size_of::<usize>() {
        return _mi_realloc_zero(p, newsize, zero != 0);
    }
    if p.is_null() {
        return mi_malloc_zero_aligned_at(newsize, alignment, offset,
                                         zero != 0);
    }
    let mut size = mi_usable_size(p);
    if newsize <= size && newsize >= (size - (size / 2)) &&
           (((p as usize) + offset) % alignment) == 0 {
        return p;
    } else {
        let mut newp = mi_malloc_aligned_at(newsize, alignment, offset);
        if !newp.is_null() {
            if zero != 0 != 0 && newsize > size {
                // also set last word in the previous allocation to zero to ensure any padding is zero-initialized
                let mut start =
                    if size >= std::mem::size_of::<isize>() {
                        size - std::mem::size_of::<isize>()
                    } else { 0 }; // only free if successful
                memset((newp as *mut u8).offset(start), 0,
                       newsize -
                           start); // use offset of previous allocation (p can be NULL)
            }
            memcpy(newp, p as *const c_void,
                   if newsize > size { size } else { newsize });
            mi_free(p);
        }
        return newp;
    };
}
unsafe fn _mi_realloc_aligned(mut p: *mut c_void, mut newsize: usize,
                              mut alignment: usize, mut zero: bool)
 -> *mut c_void {
    if alignment > 0 {
        0
    } else {
        _mi_assert_fail("alignment > 0", "src/alloc-aligned.c", 114,
                        "_mi_realloc_aligned")
    }
    if alignment <= std::mem::size_of::<usize>() {
        return _mi_realloc_zero(p, newsize, zero != 0);
    }
    let mut offset = ((p as usize) % alignment);
    return mi_realloc_zero_aligned_at(p, newsize, alignment, offset,
                                      zero != 0);
}
#[no_mangle]
pub unsafe extern "C" fn mi_realloc_aligned_at(mut p: *mut c_void,
                                               mut newsize: usize,
                                               mut alignment: usize,
                                               mut offset: usize)
 -> *mut c_void {
    return mi_realloc_zero_aligned_at(p, newsize, alignment, offset, false);
}
#[no_mangle]
pub unsafe extern "C" fn mi_realloc_aligned(mut p: *mut c_void,
                                            mut newsize: usize,
                                            mut alignment: usize)
 -> *mut c_void {
    return _mi_realloc_aligned(p, newsize, alignment, false);
}
#[no_mangle]
pub unsafe extern "C" fn mi_rezalloc_aligned_at(mut p: *mut c_void,
                                                mut newsize: usize,
                                                mut alignment: usize,
                                                mut offset: usize)
 -> *mut c_void {
    return mi_realloc_zero_aligned_at(p, newsize, alignment, offset, true);
}
#[no_mangle]
pub unsafe extern "C" fn mi_rezalloc_aligned(mut p: *mut c_void,
                                             mut newsize: usize,
                                             mut alignment: usize)
 -> *mut c_void {
    return _mi_realloc_aligned(p, newsize, alignment, true);
}
#[no_mangle]
pub unsafe extern "C" fn mi_recalloc_aligned_at(mut p: *mut c_void,
                                                mut count: usize,
                                                mut size: usize,
                                                mut alignment: usize,
                                                mut offset: usize)
 -> *mut c_void {
    let mut total: usize;
    if mi_mul_overflow(count, size, &mut total) { return ptr::null_mut(); }
    return mi_rezalloc_aligned_at(p, total, alignment, offset);
}
#[no_mangle]
pub unsafe extern "C" fn mi_recalloc_aligned(mut p: *mut c_void,
                                             mut count: usize,
                                             mut size: usize,
                                             mut alignment: usize)
 -> *mut c_void {
    let mut total: usize;
    if mi_mul_overflow(count, size, &mut total) { return ptr::null_mut(); }
    return mi_rezalloc_aligned(p, total, alignment);
}
