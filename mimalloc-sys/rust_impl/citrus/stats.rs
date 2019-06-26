/* ----------------------------------------------------------------------------
Copyright (c) 2018, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
// memset
/* -----------------------------------------------------------
  Merge thread statistics with the main one.
----------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn _mi_stats_done(mut stats: *mut mi_stats_t) {
    if stats == &mut _mi_stats_main { return; }
    mi_stats_add(&mut _mi_stats_main, stats);
    memset(stats as *mut _, 0, std::mem::size_of::<mi_stats_t>());
}
/* -----------------------------------------------------------
  Statistics operations
----------------------------------------------------------- */
unsafe fn mi_stat_update(mut stat: &mut mi_stat_count_t, mut amount: i64) {
    if amount == 0 {
        return; // add atomically (for abandoned pages)
    } // racing.. it's ok
    let mut in_main =
        ((stat as *mut u8) >= (&mut _mi_stats_main as *mut u8) &&
             (stat as *mut u8) <
                 (&mut _mi_stats_main as
                      *mut u8).offset(std::mem::size_of::<mi_stats_t>())); // add thread local
    if in_main != 0 {
        let mut current = mi_atomic_add(&mut stat.current, amount);
        if current > stat.peak { stat.peak = stat.current; }
        if amount > 0 {
            mi_atomic_add(&mut stat.allocated, amount);
        } else { mi_atomic_add(&mut stat.freed, -amount); };
    } else {
        stat.current += amount;
        if stat.current > stat.peak { stat.peak = stat.current; }
        if amount > 0 {
            stat.allocated += amount;
        } else { stat.freed += -amount; };
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mi_stat_counter_increase(mut stat:
                                                       &mut mi_stat_counter_t,
                                                   mut amount: usize) {
    // TODO: add thread safe code
    stat.count += 1;
    stat.total += amount;
}
#[no_mangle]
pub unsafe extern "C" fn _mi_stat_increase(mut stat: *mut mi_stat_count_t,
                                           mut amount: usize) {
    mi_stat_update(stat, amount as i64);
}
#[no_mangle]
pub unsafe extern "C" fn _mi_stat_decrease(mut stat: *mut mi_stat_count_t,
                                           mut amount: usize) {
    mi_stat_update(stat, -(amount as i64));
}
// must be thread safe as it is called from stats_merge
unsafe fn mi_stat_add(mut stat: &mut mi_stat_count_t,
                      mut src: &mi_stat_count_t, mut unit: i64) {
    if stat == src { return; }
    mi_atomic_add(&mut stat.allocated, src.allocated * unit);
    mi_atomic_add(&mut stat.current, src.current * unit);
    mi_atomic_add(&mut stat.freed, src.freed * unit);
    mi_atomic_add(&mut stat.peak, src.peak * unit);
}
unsafe fn mi_stat_counter_add(mut stat: &mut mi_stat_counter_t,
                              mut src: &mi_stat_counter_t, mut unit: i64) {
    if stat == src { return; }
    mi_atomic_add(&mut stat.total, src.total * unit);
    mi_atomic_add(&mut stat.count, src.count * unit);
}
// must be thread safe as it is called from stats_merge
pub static MI_BIN_HUGE: c_long = 64;
unsafe fn mi_stats_add(mut stats: &mut mi_stats_t, mut src: &mi_stats_t) {
    if stats == src { return; }
    mi_stat_add(&mut stats.segments, &src.segments, 1);
    mi_stat_add(&mut stats.pages, &src.pages, 1);
    mi_stat_add(&mut stats.reserved, &src.reserved, 1);
    mi_stat_add(&mut stats.committed, &src.committed, 1);
    mi_stat_add(&mut stats.reset, &src.reset, 1);
    mi_stat_add(&mut stats.pages_abandoned, &src.pages_abandoned, 1);
    mi_stat_add(&mut stats.segments_abandoned, &src.segments_abandoned, 1);
    mi_stat_add(&mut stats.mmap_calls, &src.mmap_calls, 1);
    mi_stat_add(&mut stats.mmap_ensure_aligned, &src.mmap_ensure_aligned, 1);
    mi_stat_add(&mut stats.mmap_right_align, &src.mmap_right_align, 1);
    mi_stat_add(&mut stats.threads, &src.threads, 1);
    mi_stat_add(&mut stats.pages_extended, &src.pages_extended, 1);
    mi_stat_add(&mut stats.malloc, &src.malloc, 1);
    mi_stat_add(&mut stats.huge, &src.huge, 1);
    mi_stat_counter_add(&mut stats.searches, &src.searches, 1);
    for mut i in 0..(MI_BIN_HUGE + 1) {
        if (src.normal[i]).allocated > 0 || (src.normal[i]).freed > 0 {
            mi_stat_add(&mut stats.normal[i], &src.normal[i], 1);
        };
    };
}
/* -----------------------------------------------------------
  Display statistics
----------------------------------------------------------- */
unsafe fn mi_printf_amount(mut n: i64, mut unit: i64, mut out: *mut FILE,
                           mut fmt: *const i8) {
    let mut buf: [i8; 32];
    let mut len = 32;
    let mut suffix = if unit <= 0 { " " } else { "b" };
    let mut base = if unit == 0 { 1000f32 } else { 1024f32 };
    if unit > 0 { n *= unit; }
    let mut pos = if n < 0 { -n } else { n } as f64;
    if pos < base {
        snprintf(buf, len, "%d %s ", n as c_int, suffix);
    } else if pos < base * base {
        snprintf(buf, len, "%.1f k%s", (n as f64) / base, suffix);
    } else if pos < base * base * base {
        snprintf(buf, len, "%.1f m%s", (n as f64) / (base * base), suffix);
    } else {
        snprintf(buf, len, "%.1f g%s", (n as f64) / (base * base * base),
                 suffix);
    }
    _mi_fprintf(out, if fmt.is_null() { "%11s" } else { fmt }, buf);
}
unsafe fn mi_print_amount(mut n: i64, mut unit: i64, mut out: *mut FILE) {
    mi_printf_amount(n, unit, out, ptr::null_mut());
}
unsafe fn mi_print_count(mut n: i64, mut unit: i64, mut out: *mut FILE) {
    if unit == 1 {
        _mi_fprintf(out, "%11s", " ");
    } else { mi_print_amount(n, 0, out); };
}
unsafe fn mi_stat_print(mut stat: &mi_stat_count_t, mut msg: *const i8,
                        mut unit: i64, mut out: *mut FILE) {
    _mi_fprintf(out, "%10s:", msg);
    mi_print_amount(stat.peak, unit, out);
    if unit != 0 {
        mi_print_amount(stat.allocated, unit, out);
        mi_print_amount(stat.freed, unit, out);
    }
    if unit > 0 {
        mi_print_amount(unit, if unit == 0 { 0 } else { 1 }, out);
        mi_print_count(stat.allocated, unit, out);
        if stat.allocated > stat.freed {
            _mi_fprintf(out, "  not all freed!\n");
        } else { _mi_fprintf(out, "  ok\n"); };
    } else { _mi_fprintf(out, "\n"); };
}
unsafe fn mi_stat_counter_print(mut stat: &mi_stat_counter_t,
                                mut msg: *const i8, mut out: *mut FILE) {
    let mut avg =
        if stat.count == 0 {
            0f32
        } else { (stat.total as f64) / (stat.count as f64) };
    _mi_fprintf(out, "%10s: %7.1f avg\n", msg, avg);
}
unsafe fn mi_print_header(mut out: *mut FILE) {
    _mi_fprintf(out, "%10s: %10s %10s %10s %10s %10s\n", "heap stats",
                "peak  ", "total  ", "freed  ", "unit  ", "count  ");
}
unsafe fn mi_stats_print_bins(mut all: *mut mi_stat_count_t,
                              mut bins: *const mi_stat_count_t,
                              mut max: usize, mut fmt: *const i8,
                              mut out: *mut FILE) {
    let mut found = false;
    let mut buf: [i8; 64];
    for mut i in 0..(max + 1) {
        if (bins[i]).allocated > 0 {
            found = true;
            let mut unit = _mi_bin_size(i as u8);
            snprintf(buf, 64, "%s %3zd", fmt, i);
            mi_stat_add(all, &bins[i], unit);
            mi_stat_print(&bins[i], buf, unit, out);
        };
    }
    //snprintf(buf, 64, "%s all", fmt);
    //mi_stat_print(all, buf, 1);
    if found != 0 { _mi_fprintf(out, "\n"); mi_print_header(out); };
}
unsafe fn _mi_stats_print(mut stats: &mut mi_stats_t, mut secs: f64,
                          mut out: *mut FILE) {
    if out.is_null() { out = stderr; }
    mi_print_header(out);
    //_mi_fprintf(out,"(mimalloc built without statistics)\n");
    let mut normal = mi_stat_count_t{_0: 0, _1: 0, _2: 0, _3: 0,};
    mi_stats_print_bins(&mut normal, stats.normal, MI_BIN_HUGE, "normal",
                        out);
    mi_stat_print(&mut normal, "normal", 1, out);
    mi_stat_print(&mut stats.huge, "huge", 1, out);
    let mut total = mi_stat_count_t{_0: 0, _1: 0, _2: 0, _3: 0,};
    mi_stat_add(&mut total, &mut normal, 1);
    mi_stat_add(&mut total, &mut stats.huge, 1);
    mi_stat_print(&mut total, "total", 1, out);
    _mi_fprintf(out, "malloc requested:     ");
    mi_print_amount(stats.malloc.allocated, 1, out);
    _mi_fprintf(out, "\n\n");
    mi_stat_print(&mut stats.committed, "committed", 1, out);
    mi_stat_print(&mut stats.reserved, "reserved", 1, out);
    mi_stat_print(&mut stats.reset, "reset", -1, out);
    mi_stat_print(&mut stats.segments, "segments", -1, out);
    mi_stat_print(&mut stats.segments_abandoned, "-abandoned", -1, out);
    mi_stat_print(&mut stats.pages, "pages", -1, out);
    mi_stat_print(&mut stats.pages_abandoned, "-abandoned", -1, out);
    mi_stat_print(&mut stats.pages_extended, "-extended", 0, out);
    mi_stat_print(&mut stats.mmap_calls, "mmaps", 0, out);
    mi_stat_print(&mut stats.mmap_right_align, "mmap fast", 0, out);
    mi_stat_print(&mut stats.mmap_ensure_aligned, "mmap slow", 0, out);
    mi_stat_print(&mut stats.threads, "threads", 0, out);
    mi_stat_counter_print(&mut stats.searches, "searches", out);
    if secs >= 0f32 { _mi_fprintf(out, "%10s: %9.3f s\n", "elapsed", secs); }
    let mut user_time: f64;
    let mut sys_time: f64;
    let mut peak_rss: usize;
    let mut page_faults: usize;
    let mut page_reclaim: usize;
    mi_process_info(&mut user_time, &mut sys_time, &mut peak_rss,
                    &mut page_faults, &mut page_reclaim);
    _mi_fprintf(out,
                "%10s: user: %.3f s, system: %.3f s, faults: %lu, reclaims: %lu, rss: ",
                "process", user_time, sys_time, page_faults as c_long,
                page_reclaim as c_long);
    mi_printf_amount(peak_rss as i64, 1, out, "%s");
    _mi_fprintf(out, "\n");
}
pub static mut mi_time_start: f64 = 0f32;
unsafe fn mi_stats_get_default() -> *mut mi_stats_t {
    let mut heap = mi_heap_get_default();
    return &mut heap.tld.stats;
}
#[no_mangle]
pub unsafe extern "C" fn mi_stats_reset() {
    let mut stats = mi_stats_get_default();
    if stats != &mut _mi_stats_main {
        memset(stats as *mut _, 0, std::mem::size_of::<mi_stats_t>());
    }
    memset(&mut _mi_stats_main, 0, std::mem::size_of::<mi_stats_t>());
    mi_time_start = mi_clock_start();
}
unsafe fn mi_stats_print_ex(mut stats: *mut mi_stats_t, mut secs: f64,
                            mut out: *mut FILE) {
    if stats != &mut _mi_stats_main {
        mi_stats_add(&mut _mi_stats_main, stats);
        memset(stats as *mut _, 0, std::mem::size_of::<mi_stats_t>());
    }
    _mi_stats_print(&mut _mi_stats_main, secs, out);
}
#[no_mangle]
pub unsafe extern "C" fn mi_stats_print(mut out: *mut FILE) {
    mi_stats_print_ex(mi_stats_get_default(), mi_clock_end(mi_time_start),
                      out);
}
#[no_mangle]
pub unsafe extern "C" fn mi_thread_stats_print(mut out: *mut FILE) {
    _mi_stats_print(mi_stats_get_default(), mi_clock_end(mi_time_start), out);
}
// --------------------------------------------------------
// Basic timer for convenience
// --------------------------------------------------------
pub static CLOCK_REALTIME: c_int = 0;
unsafe fn mi_clock_now() -> f64 {
    let mut t: timespec;
    clock_gettime(CLOCK_REALTIME, &mut t);
    return (t.tv_sec as f64) + (0.000000001f32 * (t.tv_nsec as f64));
}
// low resolution timer
pub static mut mi_clock_diff: f64 = 0f32;
unsafe fn mi_clock_start() -> f64 {
    if mi_clock_diff == 0f32 {
        let mut t0 = mi_clock_now();
        mi_clock_diff = mi_clock_now() - t0;
    }
    return mi_clock_now();
}
unsafe fn mi_clock_end(mut start: f64) -> f64 {
    let mut end = mi_clock_now();
    return (end - start - mi_clock_diff);
}
// --------------------------------------------------------
// Basic process statistics
// --------------------------------------------------------
// FILETIME is in 100 nano seconds
unsafe fn timeval_secs(mut tv: &timeval) -> f64 {
    return (tv.tv_sec as f64) + ((tv.tv_usec as f64) * 0.000001f32);
}
pub static RUSAGE_SELF: c_int = RUSAGE_SELF;
unsafe fn mi_process_info(mut utime: &mut f64, mut stime: &mut f64,
                          mut peak_rss: &mut usize,
                          mut page_faults: &mut usize,
                          mut page_reclaim: &mut usize) {
    let mut rusage: rusage;
    getrusage(RUSAGE_SELF, &mut rusage);
    *peak_rss = rusage.ru_maxrss * 1024;
    *page_faults = rusage.ru_majflt;
    *page_reclaim = rusage.ru_minflt;
    *utime = timeval_secs(&mut rusage.ru_utime);
    *stime = timeval_secs(&mut rusage.ru_stime);
}
