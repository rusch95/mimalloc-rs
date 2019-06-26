/* ----------------------------------------------------------------------------
Copyright (c) 2018, Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/
// memcpy
// Empty page used to initialize the small free pages array
pub static _mi_page_empty: mi_page_t =
    mi_page_t{_0: 0,
              _1: false,
              _2: false,
              _3: [0],
              _4: 0,
              _5: 0,
              _6: ptr::null(),
              _7: 0,
              _8: 0, // free, used, cookie
              _9: ptr::null(),
              _10: 0,
              _11: [0],
              _12: 0,
              _13: ptr::null(),
              _14: ptr::null(),
              _15: ptr::null(),};
pub static MI_SMALL_PAGES_EMPTY: [*mut mi_page_t; 130] =

    // Empty page queues for every bin
    /*131072, Huge queue */
    /* Full queue */
    // Empty statistics
    // --------------------------------------------------------
    // Statically allocate an empty heap as the initial
    // thread local value for the default heap,
    // and statically allocate the backing heap for the main
    // thread so it can function without doing any allocation
    // itself (as accessing a thread local for the first time
    // may lead to allocation itself on some platforms)
    // --------------------------------------------------------
    [(&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t), (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t),
     (&_mi_page_empty as *mut mi_page_t)];
pub static MI_PAGE_QUEUES_EMPTY: [mi_page_queue_t; 66] =
    [mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 1 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 1 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 2 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 3 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 4 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 5 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 6 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 7 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 8 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 10 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 12 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 14 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 16 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 20 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 24 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 28 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 32 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 40 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 48 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 56 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 64 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 80 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 96 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 112 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 128 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 160 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 192 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 224 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 256 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 320 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 384 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 448 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 512 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 640 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 768 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 896 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 1024 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 1280 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 1536 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 1792 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 2048 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 2560 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 3072 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 3584 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 4096 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 5120 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 6144 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 7168 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 8192 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 10240 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 12288 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 14336 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 16384 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 20480 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 24576 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 28672 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 32768 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 40960 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 49152 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 57344 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 65536 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 81920 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 98304 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2: 114688 * std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2:
                         ((((1 << (6 + (13 + 3))) / 8) >> 3) + 1) *
                             std::mem::size_of::<usize>(),},
     mi_page_queue_t{_0: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _1: (ptr::null_mut() as *mut c_void) as *mut mi_page_t,
                     _2:
                         ((((1 << (6 + (13 + 3))) / 8) >> 3) + 2) *
                             std::mem::size_of::<usize>(),}];
pub static _mi_heap_empty: mi_heap_t =
    mi_heap_t{_0: ptr::null(),
              _1: MI_SMALL_PAGES_EMPTY,
              _2: MI_PAGE_QUEUES_EMPTY,
              _3: ptr::null(),
              _4: 0,
              _5: 0,
              _6: 0,
              _7: 0,
              _8: false,};
pub static mut _mi_heap_default: *mut mi_heap_t =
    &_mi_heap_empty as *mut mi_heap_t;
pub static tld_main_stats: *mut mi_stats_t =
    (&mut tld_main as *mut u8).offset(offsetof::<mi_tld_t>("stats")) as
        *mut mi_stats_t;
pub static MI_STATS_NULL: mi_stat_count_t =
     // segments
    // os
    mi_stat_count_t{_0: 0, _1: 0, _2: 0, _3: 0,};
pub static mut tld_main: mi_tld_t =
    mi_tld_t{_0: 0u64,
             _1: &mut _mi_heap_main,
             _2:
                 [[ptr::null_mut(), ptr::null_mut()], 0, 0, 0,
                  ptr::null_mut(), tld_main_stats],
             _3: [0, ptr::null_mut(), ptr::null_mut(), 0, tld_main_stats],
             _4:
                 [MI_STATS_NULL, [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                  [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                  [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                  [0, 0, 0, 0], [0, 0, 0, 0], [0, 0],
                  [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                   [0, 0, 0, 0]]],};
// stats
pub static mut _mi_heap_main: mi_heap_t =
    mi_heap_t{_0: &mut tld_main,
              _1: MI_SMALL_PAGES_EMPTY,
              _2: MI_PAGE_QUEUES_EMPTY,
              _3: ptr::null_mut(),
              _4: 0,
              _5: 0,
              _6: 0,
              _7: 0,
              _8: false,};
// can reclaim
pub static mut _mi_process_is_initialized: bool = false;
// set to `true` in `mi_process_init`.
pub static mut _mi_stats_main: mi_stats_t =
    mi_stats_t{_0: MI_STATS_NULL,
               _1: [0, 0, 0, 0],
               _2: [0, 0, 0, 0],
               _3: [0, 0, 0, 0],
               _4: [0, 0, 0, 0],
               _5: [0, 0, 0, 0],
               _6: [0, 0, 0, 0],
               _7: [0, 0, 0, 0],
               _8: [0, 0, 0, 0],
               _9: [0, 0, 0, 0],
               _10: [0, 0, 0, 0],
               _11: [0, 0, 0, 0],
               _12: [0, 0, 0, 0],
               _13: [0, 0, 0, 0],
               _14: [0, 0],
               _15:
                   [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
                    [0, 0, 0, 0]],};
/* -----------------------------------------------------------
  Initialization of random numbers
----------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn _mi_random_shuffle(mut x: usize)
 ->
     usize { // by Sebastiano Vigna, see: <http://xoshiro.di.unimi.it/splitmix64.c>
    x ^=
        x >>
            30; // by Chris Wellons, see: <https://nullprogram.com/blog/2018/07/31/>
    x *= 13787848793156543929; /* can be zero */
    x ^= x >> 27;
    x *= 10723151780598845931;
    x ^= x >> 31;
    return x;
}
// Hopefully, ASLR makes our function address random
// xor with high res time
pub static CLOCK_MONOTONIC: c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn _mi_random_init(mut seed: usize) -> usize {
    let mut x = (&mut _mi_random_init as *mut c_void) as usize;
    x ^= seed;
    let mut time: timespec;
    clock_gettime(CLOCK_MONOTONIC, &mut time);
    x ^= time.tv_sec as usize;
    x ^= time.tv_nsec as usize;
    // and do a few randomization steps
    let mut max = ((x ^ (x >> 7)) & 15) + 1;
    for mut i in 0..max { x = _mi_random_shuffle(x); }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _mi_ptr_cookie(mut p: *const c_void) -> usize {
    return ((p as usize) ^ _mi_heap_main.cookie);
}
/* -----------------------------------------------------------
  Initialization and freeing of the thread local heaps
----------------------------------------------------------- */
pub struct mi_thread_data_s {
    pub heap: mi_heap_t,
    pub tld: mi_tld_t, // must come first due to cast in `_mi_heap_done`
}
// Initialize the thread local default heap, called from `mi_thread_init`
unsafe fn _mi_heap_init() -> bool {
    if mi_heap_is_initialized(_mi_heap_default) {
        return true; // the main heap is statically allocated
    } // use `_mi_os_alloc` to allocate directly from the OS
    if _mi_is_main_thread() {
        _mi_heap_default = &mut _mi_heap_main;
    } else {
        let mut td =
            _mi_os_alloc(std::mem::size_of::<mi_thread_data_t>(),
                         &mut _mi_stats_main) as *mut mi_thread_data_t;
        // Todo: more efficient allocation?
        if td.is_null() {
            _mi_error_message("failed to allocate thread local heap memory\n");
            return false;
        }
        let mut tld = &mut td.tld;
        let mut heap = &mut td.heap;
        memcpy(heap as *mut _, &_mi_heap_empty,
               std::mem::size_of::<mi_heap_t>());
        heap.thread_id = _mi_thread_id();
        heap.random = _mi_random_init(heap.thread_id);
        heap.cookie = ((heap as usize) ^ _mi_heap_random(heap)) | 1;
        heap.tld = tld;
        memset(tld as *mut _, 0, std::mem::size_of::<mi_tld_t>());
        tld.heap_backing = heap;
        tld.segments.stats = &mut tld.stats;
        tld.os.stats = &mut tld.stats;
        _mi_heap_default = heap;
    }
    return false;
}
// Free the thread local default heap (called from `mi_thread_done`)
// reset default heap
// todo: delete all non-backing heaps?
// switch to backing heap and free it
// free if not the main thread (or in debug mode)
pub static MI_DEBUG: c_int = 1;
unsafe fn _mi_heap_done() -> bool {
    let mut heap = _mi_heap_default;
    if !mi_heap_is_initialized(heap) { return true; }
    _mi_heap_default =
        if _mi_is_main_thread() != 0 {
            &mut _mi_heap_main
        } else { &_mi_heap_empty as *mut mi_heap_t };
    heap = heap.tld.heap_backing;
    if !mi_heap_is_initialized(heap) { return false; }
    _mi_stats_done(&mut heap.tld.stats);
    if heap != &mut _mi_heap_main {
        if heap.page_count > 0 { _mi_heap_collect_abandon(heap); }
        _mi_os_free(heap as *mut _, std::mem::size_of::<mi_thread_data_t>(),
                    &mut _mi_stats_main);
    } else if MI_DEBUG > 0 { _mi_heap_destroy_pages(heap); }
    return false;
}
// --------------------------------------------------------
// Try to run `mi_thread_done()` automatically so any memory
// owned by the thread but not yet released can be abandoned
// and re-owned by another thread.
//
// 1. windows dynamic library:
//     call from DllMain on DLL_THREAD_DETACH
// 2. windows static library:
//     use `FlsAlloc` to call a destructor when the thread is done
// 3. unix, pthreads:
//     use a pthread key to call a destructor when a pthread is done
//
// In the last two cases we also need to call `mi_process_init`
// to set up the thread local keys.
// --------------------------------------------------------
// nothing to do as it is done in DllMain
// use thread local storage keys to detect thread ending
// use pthread locol storage keys to detect thread ending
pub static mut mi_pthread_key: pthread_key_t = ();
unsafe fn mi_pthread_done(mut value: *mut c_void) {
    if !value.is_null() { mi_thread_done(); };
}
// Set up handlers so `mi_thread_done` is called automatically
unsafe fn mi_process_setup_auto_thread_done() {
    let mut tls_initialized = false; // fine if it races
    if tls_initialized != 0 {
        return; // nothing to do as it is done in DllMain
    }
    tls_initialized = true;
    pthread_key_create(&mut mi_pthread_key, &mut mi_pthread_done);
}
#[no_mangle]
pub unsafe extern "C" fn _mi_is_main_thread() -> bool {
    return (_mi_heap_main.thread_id == 0 ||
                _mi_heap_main.thread_id == _mi_thread_id());
}
// This is called from the `mi_malloc_generic`
#[no_mangle]
pub unsafe extern "C" fn mi_thread_init() {
    // ensure our process has started already
    mi_process_init();
    // initialize the thread local default heap
    if _mi_heap_init() {
        return; // returns true if already initialized
    }
    // don't further initialize for the main thread
    if _mi_is_main_thread() { return; }
    _mi_stat_increase(&mut (mi_get_default_heap().tld.stats.threads), 1);
    // set hooks so our mi_thread_done() will be called
    // nothing to do as it is done in DllMain
    // set to a dummy value so that `mi_fls_done` is called
    pthread_setspecific(mi_pthread_key,
                        (_mi_thread_id() | 1) as *mut c_void as
                            *const c_void);
    // set to a dummy value so that `mi_pthread_done` is called
    _mi_verbose_message("thread init: 0x%zx\n", _mi_thread_id());
}
#[no_mangle]
pub unsafe extern "C" fn mi_thread_done() {
    // stats
    let mut heap = mi_get_default_heap();
    if !_mi_is_main_thread() && mi_heap_is_initialized(heap) != 0 {
        _mi_stat_decrease(&mut (heap.tld.stats.threads), 1);
    }
    // abandon the thread local heap
    if _mi_heap_done() {
        return; // returns true if already ran
    }
    if !_mi_is_main_thread() {
        _mi_verbose_message("thread done: 0x%zx\n", _mi_thread_id());
    };
}
// --------------------------------------------------------
// Run functions on process init/done, and thread init/done
// --------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn mi_process_init() {
    // ensure we are called once
    if _mi_process_is_initialized != 0 { return; }
    _mi_process_is_initialized = true;
    _mi_heap_main.thread_id = _mi_thread_id();
    _mi_verbose_message("process init: 0x%zx\n", _mi_heap_main.thread_id);
    let mut random = _mi_random_init(_mi_heap_main.thread_id);
    _mi_heap_main.cookie = (&mut _mi_heap_main as usize) ^ random;
    _mi_heap_main.random = _mi_random_shuffle(random);
    _mi_verbose_message("debug level : %d\n", MI_DEBUG);
    atexit(&mut mi_process_done);
    mi_process_setup_auto_thread_done();
    mi_stats_reset();
}
unsafe fn mi_process_done() {
    // only shutdown if we were initialized
    if _mi_process_is_initialized == 0 { return; }
    // ensure we are called once
    let mut process_done = false;
    if process_done != 0 { return; }
    process_done = true;
    mi_collect(true);
    if mi_option_is_enabled(mi_option_show_stats) != 0 ||
           mi_option_is_enabled(mi_option_verbose) != 0 {
        mi_stats_print(ptr::null_mut());
    }
    _mi_verbose_message("process done: 0x%zx\n", _mi_heap_main.thread_id);
}
// Windows DLL: easy to hook into process_init and thread_done
// C++: use static initialization to detect process start
// GCC,Clang: use the constructor attribute
unsafe fn _mi_process_init() { mi_process_init(); }
