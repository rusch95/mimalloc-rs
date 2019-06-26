/* ----------------------------------------------------------------------------
Copyright (c) 2018, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
// ensure mmap flags are defined
// memset
// debug fprintf
/* -----------------------------------------------------------
  Raw allocation on Windows (VirtualAlloc) and Unix's (mmap).
  Defines a portable `mmap`, `munmap` and `mmap_trim`.
----------------------------------------------------------- */
// mmap
// sysconf
// Comment out functions ported to Rust
// uintptr_t _mi_align_up(uintptr_t sz, size_t alignment) {
//   uintptr_t x = (sz / alignment) * alignment;
//   if (x < sz) x += alignment;
//   if (x < sz) return 0; // overflow
//   return x;
// }
unsafe fn mi_align_up_ptr(mut p: *mut c_void, mut alignment: usize)
 -> *mut c_void {
    return _mi_align_up_rs(p as usize, alignment) as *mut c_void;
}
unsafe fn _mi_align_down(mut sz: usize, mut alignment: usize) -> usize {
    return (sz / alignment) * alignment;
}
unsafe fn mi_align_down_ptr(mut p: *mut c_void, mut alignment: usize)
 -> *mut c_void {
    return _mi_align_down(p as usize, alignment) as *mut c_void;
}
// cached OS page size
pub static _SC_PAGESIZE: c_int = _SC_PAGESIZE;
#[no_mangle]
pub unsafe extern "C" fn _mi_os_page_size() -> usize {
    let mut page_size = 0; // BSD
    if page_size == 0 {
        let mut result = sysconf(_SC_PAGESIZE); // Linux
        page_size = if result > 0 { result as usize } else { 4096 };
    }
    return page_size;
}
pub static errno: c_int = (*__errno_location());
unsafe fn mi_munmap(mut addr: *mut c_void, mut size: usize) {
    if addr.is_null() || size == 0 { return; }
    let mut err = false;
    err = (munmap(addr, size) == -1);
    if err != 0 {
        _mi_warning_message("munmap failed: %s, addr 0x%8li, size %lu\n",
                            strerror(errno), addr as usize, size);
    };
}
pub static MAP_PRIVATE: c_int = 2;
pub static MAP_ANONYMOUS: c_int = 32;
pub static MAP_FIXED: c_int = 16;
pub static PROT_READ: c_int = 1;
pub static PROT_WRITE: c_int = 2;
pub static MAP_FAILED: *mut c_void = -1 as *mut c_void;
unsafe fn mi_mmap(mut addr: *mut c_void, mut size: usize,
                  mut extra_flags: c_int, mut stats: &mut mi_stats_t)
 -> *mut c_void {
    (stats);
    if size == 0 { return ptr::null_mut(); }
    let mut flags = MAP_PRIVATE | MAP_ANONYMOUS | extra_flags;
    if !addr.is_null() { flags |= MAP_FIXED; }
    let mut p = mmap(addr, size, (PROT_READ | PROT_WRITE), flags, -1, 0);
    if p == MAP_FAILED { p = ptr::null_mut(); }
    if !addr.is_null() && p != addr {
        mi_munmap(p, size);
        p = ptr::null_mut();
    }
    if p.is_null() || (addr.is_null() && p != addr) ||
           (!addr.is_null() && p == addr) {
        0
    } else {
        _mi_assert_fail("p == NULL || (addr == NULL && p != addr) || (addr != NULL && p == addr)",
                        "src/os.c", 113, "mi_mmap")
    }
    if !p.is_null() { _mi_stat_increase(&mut (stats.mmap_calls), 1); }
    return p;
}
unsafe fn mi_os_page_align_region(mut addr: *mut c_void, mut size: usize,
                                  mut newsize: *mut usize) -> *mut c_void {
    if !addr.is_null() && size > 0 {
        0
    } else {
        _mi_assert_fail("addr != NULL && size > 0", "src/os.c", 120,
                        "mi_os_page_align_region")
    }
    if !newsize.is_null() { *newsize = 0; }
    if size == 0 || addr.is_null() { return ptr::null_mut(); }
    // page align conservatively within the range
    let mut start = mi_align_up_ptr(addr, _mi_os_page_size());
    let mut end =
        mi_align_down_ptr((addr as *mut u8).offset(size), _mi_os_page_size());
    let mut diff = (end as *mut u8).offset(-(start as *mut u8));
    if diff <= 0 { return ptr::null_mut(); }
    if !newsize.is_null() { *newsize = diff as usize; }
    return start;
}
// Signal to the OS that the address range is no longer in use
// but may be used later again. This will release physical memory
// pages and reduce swapping while keeping the memory committed.
// We page align to a conservative area inside the range to reset.
// page align conservatively within the range
pub static MADV_FREE: c_int = 8;
pub static EINVAL: c_int = 22;
// if MADV_FREE is not supported, fall back to MADV_DONTNEED from now on
pub static MADV_DONTNEED: c_int = 4;
#[no_mangle]
pub unsafe extern "C" fn _mi_os_reset(mut addr: *mut c_void, mut size: usize)
 -> bool {
    let mut csize: usize;
    let mut start = mi_os_page_align_region(addr, size, &mut csize);
    if csize == 0 { return true; }
    let mut advice = MADV_FREE;
    let mut err = madvise(start, csize, advice);
    if err != 0 && errno == EINVAL && advice == MADV_FREE {
        advice = MADV_DONTNEED;
        err = madvise(start, csize, advice);
    }
    if err != 0 {
        _mi_warning_message("madvise reset error: start: 0x%8p, csize: 0x%8zux, errno: %i\n",
                            start, csize, errno);
    }
    //mi_assert(err == 0);
    return (err == 0);
}
// Protect a region in memory to be not accessible.
// page align conservatively within the range
pub static PROT_NONE: c_int = 0;
unsafe fn mi_os_protectx(mut addr: *mut c_void, mut size: usize,
                         mut protect: bool) -> bool {
    let mut csize = 0;
    let mut start = mi_os_page_align_region(addr, size, &mut csize);
    if csize == 0 { return false; }
    let mut err = 0;
    err =
        mprotect(start, csize,
                 if protect != 0 != 0 {
                     PROT_NONE
                 } else { PROT_READ | PROT_WRITE });
    if err != 0 {
        _mi_warning_message("mprotect error: start: 0x%8p, csize: 0x%8zux, errno: %i\n",
                            start, csize, errno);
    }
    return (err == 0);
}
#[no_mangle]
pub unsafe extern "C" fn _mi_os_protect(mut addr: *mut c_void,
                                        mut size: usize) -> bool {
    return mi_os_protectx(addr, size, true);
}
#[no_mangle]
pub unsafe extern "C" fn _mi_os_unprotect(mut addr: *mut c_void,
                                          mut size: usize) -> bool {
    return mi_os_protectx(addr, size, false);
}
/* -----------------------------------------------------------
  OS allocation using mmap/munmap
----------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn _mi_os_alloc(mut size: usize,
                                      mut stats: &mut mi_stats_t)
 -> *mut c_void {
    if size == 0 { return ptr::null_mut(); }
    let mut p = mi_mmap(ptr::null_mut(), size, 0, stats);
    if !p.is_null() {
        0
    } else { _mi_assert_fail("p!=NULL", "src/os.c", 205, "_mi_os_alloc") }
    if !p.is_null() { _mi_stat_increase(&mut (stats.reserved), size); }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn _mi_os_free(mut p: *mut c_void, mut size: usize,
                                     mut stats: &mut mi_stats_t) {
    (stats);
    mi_munmap(p, size);
    _mi_stat_decrease(&mut (stats.reserved), size);
}
// Slow but guaranteed way to allocated aligned memory
// by over-allocating and then reallocating at a fixed aligned
// address that should be available then.
unsafe fn mi_os_alloc_aligned_ensured(mut size: usize, mut alignment: usize,
                                      mut trie: usize,
                                      mut stats: *mut mi_stats_t)
 -> *mut c_void {
    if trie >= 3 {
        return ptr::null_mut(); // stop recursion (only on Windows)
    } // overflow?
    let mut alloc_size = size + alignment;
    if alloc_size >= size {
        0
    } else {
        _mi_assert_fail("alloc_size >= size", "src/os.c", 223,
                        "mi_os_alloc_aligned_ensured")
    }
    if alloc_size < size { return ptr::null_mut(); }
    // allocate a chunk that includes the alignment
    let mut p = mi_mmap(ptr::null_mut(), alloc_size, 0, stats);
    if p.is_null() { return ptr::null_mut(); }
    // create an aligned pointer in the allocated area
    let mut aligned_p = mi_align_up_ptr(p, alignment);
    if !aligned_p.is_null() {
        0
    } else {
        _mi_assert_fail("aligned_p != NULL", "src/os.c", 231,
                        "mi_os_alloc_aligned_ensured")
    }
    // free it and try to allocate `size` at exactly `aligned_p`
    // note: this may fail in case another thread happens to VirtualAlloc
    // concurrently at that spot. We try up to 3 times to mitigate this.
    // we selectively unmap parts around the over-allocated area.
    let mut pre_size = (aligned_p as *mut u8).offset(-(p as *mut u8));
    let mut mid_size = _mi_align_up_rs(size, _mi_os_page_size());
    let mut post_size = alloc_size - pre_size - mid_size;
    if pre_size > 0 { mi_munmap(p, pre_size); }
    if post_size > 0 {
        mi_munmap((aligned_p as *mut u8).offset(mid_size), post_size);
    }
    if (aligned_p as usize) % alignment == 0 {
        0
    } else {
        _mi_assert_fail("((uintptr_t)aligned_p) % alignment == 0", "src/os.c",
                        251, "mi_os_alloc_aligned_ensured")
    }
    return aligned_p;
}
// Allocate an aligned block.
// Since `mi_mmap` is relatively slow we try to allocate directly at first and
// hope to get an aligned address; only when that fails we fall back
// to a guaranteed method by overallocating at first and adjusting.
// TODO: use VirtualAlloc2 with alignment on Windows 10 / Windows Server 2016.
// on BSD, use the aligned mmap api
// alignment is a power of 2 and >= 4096
// use the NetBSD/freeBSD aligned flags
// if the next probable address is aligned,
// then try to just allocate `size` and hope it is aligned...
//fprintf(stderr, "segment address guess: %s, p=%lxu, guess:%lxu\n", (p != NULL && (uintptr_t)p % alignment ==0 ? "correct" : "incorrect"), (uintptr_t)p, next_probable);
// if `p` is not yet aligned after all, free the block and use a slower
// but guaranteed way to allocate an aligned block
//fprintf(stderr, "mimalloc: slow mmap 0x%lx\n", _mi_thread_id());
// next probable address is the page-aligned address just after the newly allocated area.
// Windows allocates 64kb aligned
// page size on other OS's
pub static MI_SEGMENT_SIZE: usize = (1 << (6 + (13 + 3)));
#[no_mangle]
pub unsafe extern "C" fn _mi_os_alloc_aligned(mut size: usize,
                                              mut alignment: usize,
                                              mut tld: &mut mi_os_tld_t)
 -> *mut c_void {
    if size == 0 {
        return ptr::null_mut(); // Linux tends to allocate downward
    }
    if alignment < 1024 { return _mi_os_alloc(size, tld.stats); }
    let mut p = os_pool_alloc(size, alignment, tld);
    if !p.is_null() { return p; }
    let mut suggest = ptr::null_mut();
    if p.is_null() && (tld.mmap_next_probable % alignment) == 0 {
        p = mi_mmap(suggest, size, 0, tld.stats);
        if p.is_null() { return ptr::null_mut(); }
        if ((p as usize) % alignment) == 0 {
            _mi_stat_increase(&mut (tld.stats.mmap_right_align), 1);
        };
    }
    if p.is_null() || ((p as usize) % alignment) != 0 {
        if !p.is_null() { mi_munmap(p, size); }
        _mi_stat_increase(&mut (tld.stats.mmap_ensure_aligned), 1);
        p = mi_os_alloc_aligned_ensured(size, alignment, 0, tld.stats);
    }
    if !p.is_null() {
        _mi_stat_increase(&mut (tld.stats.reserved), size);
        let alloc_align = _mi_os_page_size();
        let mut probable_size = MI_SEGMENT_SIZE;
        if tld.mmap_previous > p {
            tld.mmap_next_probable =
                _mi_align_down((p as usize) - probable_size, alloc_align);
            // ((uintptr_t)previous - (uintptr_t)p);
        } else { // Otherwise, guess the next address is page aligned `size` from current pointer
            tld.mmap_next_probable =
                _mi_align_up_rs((p as usize) + probable_size, alloc_align);
        }
        tld.mmap_previous = p;
    }
    return p;
}
// Pooled allocation: on 64-bit systems with plenty
// of virtual addresses, we allocate 10 segments at the
// time to minimize `mmap` calls and increase aligned
// allocations. This is only good on systems that
// do overcommit so we put it behind the `MIMALLOC_POOL_COMMIT` option.
// For now, we disable it on windows as VirtualFree must
// be called on the original allocation and cannot be called
// for individual fragments.
pub static MI_POOL_ALIGNMENT: c_long = (1 << (6 + (13 + 3)));
pub static MI_POOL_SIZE: c_long = (10 * (1 << (6 + (13 + 3))));
unsafe fn os_pool_alloc(mut size: usize, mut alignment: usize,
                        mut tld: &mut mi_os_tld_t) -> *mut c_void {
    if !mi_option_is_enabled(mi_option_pool_commit) {
        return ptr::null_mut();
    }
    if alignment != MI_POOL_ALIGNMENT { return ptr::null_mut(); }
    size = _mi_align_up_rs(size, MI_POOL_ALIGNMENT);
    if size > MI_POOL_SIZE { return ptr::null_mut(); }
    if tld.pool_available == 0 {
        tld.pool =
            mi_os_alloc_aligned_ensured(MI_POOL_SIZE, MI_POOL_ALIGNMENT, 0,
                                        tld.stats) as *mut u8;
        if tld.pool.is_null() { return ptr::null_mut(); }
        tld.pool_available += MI_POOL_SIZE;
    }
    if size > tld.pool_available { return ptr::null_mut(); }
    let mut p = tld.pool as *mut _;
    tld.pool_available -= size;
    tld.pool = tld.pool.offset(size);
    return p;
}
