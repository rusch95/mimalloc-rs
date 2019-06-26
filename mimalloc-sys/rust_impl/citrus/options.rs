/* ----------------------------------------------------------------------------
Copyright (c) 2018, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
// strcmp
// toupper
// --------------------------------------------------------
// Options
// --------------------------------------------------------
pub enum mi_init_e {
    UNINIT,
    DEFAULTED,
    INITIALIZED, // not yet initialized
    // not found in the environment, use default value
    // found in environment or set explicitly
}
pub struct mi_option_desc_s {
    pub value: c_long,
    pub init: mi_init_t,
    pub name: *const i8, // the value
    // is it initialized yet? (from the environment)
    // option name without `mimalloc_` prefix
}
// in secure build the environment setting is ignored
pub static MI_DEBUG: c_long = 1;
pub static mut options: [mi_option_desc_t; 7] =
    [mi_option_desc_t{_0: 0, _1: UNINIT, _2: "page_reset",},
     mi_option_desc_t{_0: 0, _1: UNINIT, _2: "cache_reset",},
     mi_option_desc_t{_0: 0, _1: UNINIT, _2: "pool_commit",},
     mi_option_desc_t{_0: 0, _1: UNINIT, _2: "secure",},
     mi_option_desc_t{_0: 0, _1: UNINIT, _2: "show_stats",},
     mi_option_desc_t{_0: MI_DEBUG, _1: UNINIT, _2: "show_errors",},
     mi_option_desc_t{_0: MI_DEBUG, _1: UNINIT, _2: "verbose",}];
#[no_mangle]
pub unsafe extern "C" fn mi_option_get(mut option: mi_option_t) -> c_long {
    if option >= 0 && option < _mi_option_last {
        0
    } else {
        _mi_assert_fail("option >= 0 && option < _mi_option_last",
                        "src/options.c", 47, "mi_option_get")
    }
    let mut desc = &mut options[option];
    if desc.init == UNINIT {
        mi_option_init(desc);
        if option != mi_option_verbose {
            _mi_verbose_message("option \'%s\': %zd\n", desc.name,
                                desc.value);
        };
    }
    return desc.value;
}
#[no_mangle]
pub unsafe extern "C" fn mi_option_set(mut option: mi_option_t,
                                       mut value: c_long) {
    if option >= 0 && option < _mi_option_last {
        0
    } else {
        _mi_assert_fail("option >= 0 && option < _mi_option_last",
                        "src/options.c", 59, "mi_option_set")
    }
    let mut desc = &mut options[option];
    desc.value = value;
    desc.init = INITIALIZED;
}
#[no_mangle]
pub unsafe extern "C" fn mi_option_set_default(mut option: mi_option_t,
                                               mut value: c_long) {
    if option >= 0 && option < _mi_option_last {
        0
    } else {
        _mi_assert_fail("option >= 0 && option < _mi_option_last",
                        "src/options.c", 66, "mi_option_set_default")
    }
    let mut desc = &mut options[option];
    if desc.init != INITIALIZED { desc.value = value; };
}
#[no_mangle]
pub unsafe extern "C" fn mi_option_is_enabled(mut option: mi_option_t)
 -> bool {
    return (mi_option_get(option) != 0);
}
#[no_mangle]
pub unsafe extern "C" fn mi_option_enable(mut option: mi_option_t,
                                          mut enable: bool) {
    mi_option_set(option, if enable != 0 != 0 { 1 } else { 0 });
}
#[no_mangle]
pub unsafe extern "C" fn mi_option_enable_default(mut option: mi_option_t,
                                                  mut enable: bool) {
    mi_option_set_default(option, if enable != 0 != 0 { 1 } else { 0 });
}
// --------------------------------------------------------
// Messages
// --------------------------------------------------------
// Define our own limited `fprintf` that avoids memory allocation.
// We do this using `snprintf` with a limited buffer.
unsafe fn mi_vfprintf(mut out: *mut FILE, mut prefix: *const i8,
                      mut fmt: *const i8, mut args: va_list) {
    let mut buf: [i8; 256];
    if fmt.is_null() { return; }
    if out.is_null() { out = stdout; }
    vsnprintf(buf, std::mem::size_of_val(&buf) - 1, fmt, args);
    if !prefix.is_null() { fputs(prefix, out); }
    fputs(buf, out);
}
#[no_mangle]
pub unsafe extern "C" fn _mi_fprintf(mut out: *mut FILE,
                                     mut fmt: *const i8, ...) {
    let mut args: va_list;
    __builtin_va_start(args, fmt);
    mi_vfprintf(out, ptr::null_mut(), fmt, args);
    __builtin_va_end(args);
}
#[no_mangle]
pub unsafe extern "C" fn _mi_verbose_message(mut fmt: *const i8, ...) {
    if !mi_option_is_enabled(mi_option_verbose) { return; }
    let mut args: va_list;
    __builtin_va_start(args, fmt);
    mi_vfprintf(stderr, "mimalloc: ", fmt, args);
    __builtin_va_end(args);
}
#[no_mangle]
pub unsafe extern "C" fn _mi_error_message(mut fmt: *const i8, ...) {
    if !mi_option_is_enabled(mi_option_show_errors) &&
           !mi_option_is_enabled(mi_option_verbose) {
        return;
    }
    let mut args: va_list;
    __builtin_va_start(args, fmt);
    mi_vfprintf(stderr, "mimalloc: error: ", fmt, args);
    __builtin_va_end(args);
    if (false) != 0 {
        0
    } else {
        _mi_assert_fail("false", "src/options.c", 121, "_mi_error_message")
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mi_warning_message(mut fmt: *const i8, ...) {
    if !mi_option_is_enabled(mi_option_show_errors) &&
           !mi_option_is_enabled(mi_option_verbose) {
        return;
    }
    let mut args: va_list;
    __builtin_va_start(args, fmt);
    mi_vfprintf(stderr, "mimalloc: warning: ", fmt, args);
    __builtin_va_end(args);
}
#[no_mangle]
pub unsafe extern "C" fn _mi_assert_fail(mut assertion: *const i8,
                                         mut fname: *const i8,
                                         mut line: c_uint,
                                         mut func: *const i8) {
    _mi_fprintf(stderr,
                "mimalloc: assertion failed: at \"%s\":%u, %s\n  assertion: \"%s\"\n",
                fname, line, if func.is_null() { "" } else { func },
                assertion);
    abort();
}
// --------------------------------------------------------
// Initialize options by checking the environment
// --------------------------------------------------------
unsafe fn mi_strlcpy(mut dest: *mut i8, mut src: *const i8,
                     mut dest_size: usize) {
    dest[0] = 0i8;
    strncpy(dest, src, dest_size - 1);
    dest[dest_size - 1] = 0i8;
}
unsafe fn mi_strlcat(mut dest: *mut i8, mut src: *const i8,
                     mut dest_size: usize) {
    strncat(dest, src, dest_size - 1);
    dest[dest_size - 1] = 0i8;
}
unsafe fn mi_option_init(mut desc: &mut mi_option_desc_t) {
    desc.init = DEFAULTED;
    // Read option value from the environment
    let mut buf: [i8; 32];
    mi_strlcpy(buf, "mimalloc_", std::mem::size_of_val(&buf));
    mi_strlcat(buf, desc.name, std::mem::size_of_val(&buf));
    let mut s = getenv(buf);
    if s.is_null() {
        for mut i in 0..strlen(buf) { buf[i] = toupper(buf[i]); }
        s = getenv(buf);
    }
    if !s.is_null() {
        mi_strlcpy(buf, s, std::mem::size_of_val(&buf));
        for mut i in 0..strlen(buf) { buf[i] = toupper(buf[i]); }
        if buf[0] == 0 || !strstr("1;TRUE;YES;ON", buf).is_null() {
            desc.value = 1;
            desc.init = INITIALIZED;
        } else if !strstr("0;FALSE;NO;OFF", buf).is_null() {
            desc.value = 0;
            desc.init = INITIALIZED;
        } else {
            let mut end = buf;
            let mut value = strtol(buf, &mut end, 10);
            if *end == 0 {
                desc.value = value;
                desc.init = INITIALIZED;
            } else {
                _mi_warning_message("environment option mimalloc_%s has an invalid value: %s\n",
                                    desc.name, buf);
            };
        };
    };
}
