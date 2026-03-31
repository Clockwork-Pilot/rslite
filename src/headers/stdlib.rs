    pub fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.cos() as ::core::ffi::c_double
    }
    pub fn acos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.acos() as ::core::ffi::c_double
    }
    pub fn asin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.asin() as ::core::ffi::c_double
    }
    pub fn atan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.atan() as ::core::ffi::c_double
    }
    pub fn atan2(__y: ::core::ffi::c_double, __x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __y.atan2(__x) as ::core::ffi::c_double
    }
    pub fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.sin() as ::core::ffi::c_double
    }
    pub fn tan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.tan() as ::core::ffi::c_double
    }
    pub fn cosh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.cosh() as ::core::ffi::c_double
    }
    pub fn sinh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.sinh() as ::core::ffi::c_double
    }
    pub fn tanh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.tanh() as ::core::ffi::c_double
    }
    pub fn acosh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.acosh() as ::core::ffi::c_double
    }
    pub fn asinh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.asinh() as ::core::ffi::c_double
    }
    pub fn atanh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.atanh() as ::core::ffi::c_double
    }
    pub fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.exp() as ::core::ffi::c_double
    }
    pub fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.ln() as ::core::ffi::c_double
    }
    pub fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.log10() as ::core::ffi::c_double
    }
    pub fn log2(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.log2() as ::core::ffi::c_double
    }
    pub fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.powf(__y) as ::core::ffi::c_double
    }
    pub fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.sqrt() as ::core::ffi::c_double
    }
    pub fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.ceil() as ::core::ffi::c_double
    }
    pub fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.abs() as ::core::ffi::c_double
    }
    pub fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.floor() as ::core::ffi::c_double
    }
    pub fn fmod(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double {
        (__x % __y) as ::core::ffi::c_double
    }
    pub fn trunc(__x: ::core::ffi::c_double) -> ::core::ffi::c_double {
        __x.trunc() as ::core::ffi::c_double
    }
    pub unsafe fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong {
        let mut n: usize = 0;
        let mut p = __s;
        'outer: loop {
            let c = *p;
            if c == 0 { break; }
            let mut q = __reject;
            loop {
                let r = *q;
                if r == 0 { break; }
                if c == r { break 'outer; }
                q = q.add(1);
            }
            p = p.add(1);
            n += 1;
        }
        n as ::core::ffi::c_ulong
    }
    pub unsafe fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong {
        let mut n: usize = 0;
        let mut p = __s;
        'outer: loop {
            let c = *p;
            if c == 0 { break; }
            let mut q = __accept;
            loop {
                let r = *q;
                if r == 0 { break 'outer; }
                if c == r { break; }
                q = q.add(1);
            }
            p = p.add(1);
            n += 1;
        }
        n as ::core::ffi::c_ulong
    }
    pub unsafe fn gettimeofday(
        __tv: *mut ::libc::timeval,
        _tz: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int {
        use ::std::time::{SystemTime, UNIX_EPOCH};
        if !__tv.is_null() {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(d) => {
                    (*__tv).tv_sec = d.as_secs() as _;
                    (*__tv).tv_usec = d.subsec_micros() as _;
                }
                Err(_) => return -1,
            }
        }
        0
    }
    
    pub type Tcl_Interp = *mut std::ffi::c_void;

    unsafe extern "C" {
        pub static mut stdout: *mut FILE;
        pub fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
        pub fn pthread_create(
            __newthread: *mut crate::src::headers::stdlib::pthread_t,
            __attr: *const crate::src::headers::stdlib::pthread_attr_t,
            __start_routine: Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            >,
            __arg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        pub fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;

        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn pthread_mutex_init(
            __mutex: *mut crate::src::headers::stdlib::pthread_mutex_t,
            __mutexattr: *const crate::src::headers::stdlib::pthread_mutexattr_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutex_destroy(
            __mutex: *mut crate::src::headers::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutex_trylock(
            __mutex: *mut crate::src::headers::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutex_lock(
            __mutex: *mut crate::src::headers::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutex_unlock(
            __mutex: *mut crate::src::headers::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutexattr_init(
            __attr: *mut crate::src::headers::stdlib::pthread_mutexattr_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutexattr_destroy(
            __attr: *mut crate::src::headers::stdlib::pthread_mutexattr_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutexattr_settype(
            __attr: *mut crate::src::headers::stdlib::pthread_mutexattr_t,
            __kind: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub fn stat(
            __file: *const ::core::ffi::c_char,
            __buf: *mut crate::src::headers::stdlib::stat,
        ) -> ::core::ffi::c_int;

        pub fn fstat(
            __fd: ::core::ffi::c_int,
            __buf: *mut crate::src::headers::stdlib::stat,
        ) -> ::core::ffi::c_int;

        pub fn lstat(
            __file: *const ::core::ffi::c_char,
            __buf: *mut crate::src::headers::stdlib::stat,
        ) -> ::core::ffi::c_int;



        #[cfg(feature = "test")]
        pub fn TclFreeObj(objPtr: *mut crate::src::headers::stdlib::Tcl_Obj);

        #[cfg(feature = "test")]
        pub fn Tcl_ListObjAppendElement(
            interp: *mut crate::src::headers::stdlib::Tcl_Interp,
            listPtr: *mut crate::src::headers::stdlib::Tcl_Obj,
            objPtr: *mut crate::src::headers::stdlib::Tcl_Obj,
        ) -> ::core::ffi::c_int;

        #[cfg(feature = "test")]
        pub fn Tcl_NewObj() -> *mut crate::src::headers::stdlib::Tcl_Obj;

        #[cfg(feature = "test")]
        pub fn Tcl_NewStringObj(
            bytes: *const ::core::ffi::c_char,
            length: crate::src::headers::stdlib::Tcl_Size,
        ) -> *mut crate::src::headers::stdlib::Tcl_Obj;

        #[cfg(feature = "test")]
        pub fn Tcl_NewWideIntObj(
            wideValue: crate::src::headers::stdlib::Tcl_WideInt,
        ) -> *mut crate::src::headers::stdlib::Tcl_Obj;

        #[cfg(feature = "test")]
        pub fn Tcl_GetStringFromObj(
            objPtr: *mut crate::src::headers::stdlib::Tcl_Obj,
            lengthPtr: *mut crate::src::headers::stdlib::Tcl_Size,
        ) -> *mut ::core::ffi::c_char;

        pub fn read(
            __fd: ::core::ffi::c_int,
            __buf: *mut ::core::ffi::c_void,
            __nbytes: crate::__stddef_size_t_h::size_t,
        ) -> crate::src::headers::stdlib::ssize_t;

        pub fn write(
            __fd: ::core::ffi::c_int,
            __buf: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> crate::src::headers::stdlib::ssize_t;

        pub fn pread64(
            __fd: ::core::ffi::c_int,
            __buf: *mut ::core::ffi::c_void,
            __nbytes: crate::__stddef_size_t_h::size_t,
            __offset: crate::src::headers::stdlib::__off64_t,
        ) -> crate::src::headers::stdlib::ssize_t;

        pub fn pwrite64(
            __fd: ::core::ffi::c_int,
            __buf: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
            __offset: crate::src::headers::stdlib::__off64_t,
        ) -> crate::src::headers::stdlib::ssize_t;

        pub fn readlink(
            __path: *const ::core::ffi::c_char,
            __buf: *mut ::core::ffi::c_char,
            __len: crate::__stddef_size_t_h::size_t,
        ) -> crate::src::headers::stdlib::ssize_t;
    }
    pub type FILE = crate::src::headers::stdlib::_IO_FILE;
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
    pub const _SC_ARG_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 0;

    pub const _SC_CHILD_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 1;

    pub const _SC_CLK_TCK: crate::src::headers::stdlib::C2RustUnnamed_1 = 2;

    pub const _SC_NGROUPS_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 3;

    pub const _SC_OPEN_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 4;

    pub const _SC_STREAM_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 5;

    pub const _SC_TZNAME_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 6;

    pub const _SC_JOB_CONTROL: crate::src::headers::stdlib::C2RustUnnamed_1 = 7;

    pub const _SC_SAVED_IDS: crate::src::headers::stdlib::C2RustUnnamed_1 = 8;

    pub const _SC_REALTIME_SIGNALS: crate::src::headers::stdlib::C2RustUnnamed_1 = 9;

    pub const _SC_PRIORITY_SCHEDULING: crate::src::headers::stdlib::C2RustUnnamed_1 = 10;

    pub const _SC_TIMERS: crate::src::headers::stdlib::C2RustUnnamed_1 = 11;

    pub const _SC_ASYNCHRONOUS_IO: crate::src::headers::stdlib::C2RustUnnamed_1 = 12;

    pub const _SC_PRIORITIZED_IO: crate::src::headers::stdlib::C2RustUnnamed_1 = 13;

    pub const _SC_SYNCHRONIZED_IO: crate::src::headers::stdlib::C2RustUnnamed_1 = 14;

    pub const _SC_FSYNC: crate::src::headers::stdlib::C2RustUnnamed_1 = 15;

    pub const _SC_MAPPED_FILES: crate::src::headers::stdlib::C2RustUnnamed_1 = 16;

    pub const _SC_MEMLOCK: crate::src::headers::stdlib::C2RustUnnamed_1 = 17;

    pub const _SC_MEMLOCK_RANGE: crate::src::headers::stdlib::C2RustUnnamed_1 = 18;

    pub const _SC_MEMORY_PROTECTION: crate::src::headers::stdlib::C2RustUnnamed_1 = 19;

    pub const _SC_MESSAGE_PASSING: crate::src::headers::stdlib::C2RustUnnamed_1 = 20;

    pub const _SC_SEMAPHORES: crate::src::headers::stdlib::C2RustUnnamed_1 = 21;

    pub const _SC_SHARED_MEMORY_OBJECTS: crate::src::headers::stdlib::C2RustUnnamed_1 = 22;

    pub const _SC_AIO_LISTIO_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 23;

    pub const _SC_AIO_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 24;

    pub const _SC_AIO_PRIO_DELTA_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 25;

    pub const _SC_DELAYTIMER_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 26;

    pub const _SC_MQ_OPEN_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 27;

    pub const _SC_MQ_PRIO_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 28;

    pub const _SC_VERSION: crate::src::headers::stdlib::C2RustUnnamed_1 = 29;

    pub const _SC_PAGESIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 30;

    pub const _SC_RTSIG_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 31;

    pub const _SC_SEM_NSEMS_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 32;

    pub const _SC_SEM_VALUE_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 33;

    pub const _SC_SIGQUEUE_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 34;

    pub const _SC_TIMER_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 35;

    pub const _SC_BC_BASE_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 36;

    pub const _SC_BC_DIM_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 37;

    pub const _SC_BC_SCALE_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 38;

    pub const _SC_BC_STRING_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 39;

    pub const _SC_COLL_WEIGHTS_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 40;

    pub const _SC_EQUIV_CLASS_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 41;

    pub const _SC_EXPR_NEST_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 42;

    pub const _SC_LINE_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 43;

    pub const _SC_RE_DUP_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 44;

    pub const _SC_CHARCLASS_NAME_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 45;

    pub const _SC_2_VERSION: crate::src::headers::stdlib::C2RustUnnamed_1 = 46;

    pub const _SC_2_C_BIND: crate::src::headers::stdlib::C2RustUnnamed_1 = 47;

    pub const _SC_2_C_DEV: crate::src::headers::stdlib::C2RustUnnamed_1 = 48;

    pub const _SC_2_FORT_DEV: crate::src::headers::stdlib::C2RustUnnamed_1 = 49;

    pub const _SC_2_FORT_RUN: crate::src::headers::stdlib::C2RustUnnamed_1 = 50;

    pub const _SC_2_SW_DEV: crate::src::headers::stdlib::C2RustUnnamed_1 = 51;

    pub const _SC_2_LOCALEDEF: crate::src::headers::stdlib::C2RustUnnamed_1 = 52;

    pub const _SC_PII: crate::src::headers::stdlib::C2RustUnnamed_1 = 53;

    pub const _SC_PII_XTI: crate::src::headers::stdlib::C2RustUnnamed_1 = 54;

    pub const _SC_PII_SOCKET: crate::src::headers::stdlib::C2RustUnnamed_1 = 55;

    pub const _SC_PII_INTERNET: crate::src::headers::stdlib::C2RustUnnamed_1 = 56;

    pub const _SC_PII_OSI: crate::src::headers::stdlib::C2RustUnnamed_1 = 57;

    pub const _SC_POLL: crate::src::headers::stdlib::C2RustUnnamed_1 = 58;

    pub const _SC_SELECT: crate::src::headers::stdlib::C2RustUnnamed_1 = 59;

    pub const _SC_UIO_MAXIOV: crate::src::headers::stdlib::C2RustUnnamed_1 = 60;

    pub const _SC_IOV_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 60;

    pub const _SC_PII_INTERNET_STREAM: crate::src::headers::stdlib::C2RustUnnamed_1 = 61;

    pub const _SC_PII_INTERNET_DGRAM: crate::src::headers::stdlib::C2RustUnnamed_1 = 62;

    pub const _SC_PII_OSI_COTS: crate::src::headers::stdlib::C2RustUnnamed_1 = 63;

    pub const _SC_PII_OSI_CLTS: crate::src::headers::stdlib::C2RustUnnamed_1 = 64;

    pub const _SC_PII_OSI_M: crate::src::headers::stdlib::C2RustUnnamed_1 = 65;

    pub const _SC_T_IOV_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 66;

    pub const _SC_THREADS: crate::src::headers::stdlib::C2RustUnnamed_1 = 67;

    pub const _SC_THREAD_SAFE_FUNCTIONS: crate::src::headers::stdlib::C2RustUnnamed_1 = 68;

    pub const _SC_GETGR_R_SIZE_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 69;

    pub const _SC_GETPW_R_SIZE_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 70;

    pub const _SC_LOGIN_NAME_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 71;

    pub const _SC_TTY_NAME_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 72;

    pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: crate::src::headers::stdlib::C2RustUnnamed_1 = 73;

    pub const _SC_THREAD_KEYS_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 74;

    pub const _SC_THREAD_STACK_MIN: crate::src::headers::stdlib::C2RustUnnamed_1 = 75;

    pub const _SC_THREAD_THREADS_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 76;

    pub const _SC_THREAD_ATTR_STACKADDR: crate::src::headers::stdlib::C2RustUnnamed_1 = 77;

    pub const _SC_THREAD_ATTR_STACKSIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 78;

    pub const _SC_THREAD_PRIORITY_SCHEDULING: crate::src::headers::stdlib::C2RustUnnamed_1 = 79;

    pub const _SC_THREAD_PRIO_INHERIT: crate::src::headers::stdlib::C2RustUnnamed_1 = 80;

    pub const _SC_THREAD_PRIO_PROTECT: crate::src::headers::stdlib::C2RustUnnamed_1 = 81;

    pub const _SC_THREAD_PROCESS_SHARED: crate::src::headers::stdlib::C2RustUnnamed_1 = 82;

    pub const _SC_NPROCESSORS_CONF: crate::src::headers::stdlib::C2RustUnnamed_1 = 83;

    pub const _SC_NPROCESSORS_ONLN: crate::src::headers::stdlib::C2RustUnnamed_1 = 84;

    pub const _SC_PHYS_PAGES: crate::src::headers::stdlib::C2RustUnnamed_1 = 85;

    pub const _SC_AVPHYS_PAGES: crate::src::headers::stdlib::C2RustUnnamed_1 = 86;

    pub const _SC_ATEXIT_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 87;

    pub const _SC_PASS_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 88;

    pub const _SC_XOPEN_VERSION: crate::src::headers::stdlib::C2RustUnnamed_1 = 89;

    pub const _SC_XOPEN_XCU_VERSION: crate::src::headers::stdlib::C2RustUnnamed_1 = 90;

    pub const _SC_XOPEN_UNIX: crate::src::headers::stdlib::C2RustUnnamed_1 = 91;

    pub const _SC_XOPEN_CRYPT: crate::src::headers::stdlib::C2RustUnnamed_1 = 92;

    pub const _SC_XOPEN_ENH_I18N: crate::src::headers::stdlib::C2RustUnnamed_1 = 93;

    pub const _SC_XOPEN_SHM: crate::src::headers::stdlib::C2RustUnnamed_1 = 94;

    pub const _SC_2_CHAR_TERM: crate::src::headers::stdlib::C2RustUnnamed_1 = 95;

    pub const _SC_2_C_VERSION: crate::src::headers::stdlib::C2RustUnnamed_1 = 96;

    pub const _SC_2_UPE: crate::src::headers::stdlib::C2RustUnnamed_1 = 97;

    pub const _SC_XOPEN_XPG2: crate::src::headers::stdlib::C2RustUnnamed_1 = 98;

    pub const _SC_XOPEN_XPG3: crate::src::headers::stdlib::C2RustUnnamed_1 = 99;

    pub const _SC_XOPEN_XPG4: crate::src::headers::stdlib::C2RustUnnamed_1 = 100;

    pub const _SC_CHAR_BIT: crate::src::headers::stdlib::C2RustUnnamed_1 = 101;

    pub const _SC_CHAR_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 102;

    pub const _SC_CHAR_MIN: crate::src::headers::stdlib::C2RustUnnamed_1 = 103;

    pub const _SC_INT_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 104;

    pub const _SC_INT_MIN: crate::src::headers::stdlib::C2RustUnnamed_1 = 105;

    pub const _SC_LONG_BIT: crate::src::headers::stdlib::C2RustUnnamed_1 = 106;

    pub const _SC_WORD_BIT: crate::src::headers::stdlib::C2RustUnnamed_1 = 107;

    pub const _SC_MB_LEN_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 108;

    pub const _SC_NZERO: crate::src::headers::stdlib::C2RustUnnamed_1 = 109;

    pub const _SC_SSIZE_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 110;

    pub const _SC_SCHAR_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 111;

    pub const _SC_SCHAR_MIN: crate::src::headers::stdlib::C2RustUnnamed_1 = 112;

    pub const _SC_SHRT_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 113;

    pub const _SC_SHRT_MIN: crate::src::headers::stdlib::C2RustUnnamed_1 = 114;

    pub const _SC_UCHAR_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 115;

    pub const _SC_UINT_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 116;

    pub const _SC_ULONG_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 117;

    pub const _SC_USHRT_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 118;

    pub const _SC_NL_ARGMAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 119;

    pub const _SC_NL_LANGMAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 120;

    pub const _SC_NL_MSGMAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 121;

    pub const _SC_NL_NMAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 122;

    pub const _SC_NL_SETMAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 123;

    pub const _SC_NL_TEXTMAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 124;

    pub const _SC_XBS5_ILP32_OFF32: crate::src::headers::stdlib::C2RustUnnamed_1 = 125;

    pub const _SC_XBS5_ILP32_OFFBIG: crate::src::headers::stdlib::C2RustUnnamed_1 = 126;

    pub const _SC_XBS5_LP64_OFF64: crate::src::headers::stdlib::C2RustUnnamed_1 = 127;

    pub const _SC_XBS5_LPBIG_OFFBIG: crate::src::headers::stdlib::C2RustUnnamed_1 = 128;

    pub const _SC_XOPEN_LEGACY: crate::src::headers::stdlib::C2RustUnnamed_1 = 129;

    pub const _SC_XOPEN_REALTIME: crate::src::headers::stdlib::C2RustUnnamed_1 = 130;

    pub const _SC_XOPEN_REALTIME_THREADS: crate::src::headers::stdlib::C2RustUnnamed_1 = 131;

    pub const _SC_ADVISORY_INFO: crate::src::headers::stdlib::C2RustUnnamed_1 = 132;

    pub const _SC_BARRIERS: crate::src::headers::stdlib::C2RustUnnamed_1 = 133;

    pub const _SC_BASE: crate::src::headers::stdlib::C2RustUnnamed_1 = 134;

    pub const _SC_C_LANG_SUPPORT: crate::src::headers::stdlib::C2RustUnnamed_1 = 135;

    pub const _SC_C_LANG_SUPPORT_R: crate::src::headers::stdlib::C2RustUnnamed_1 = 136;

    pub const _SC_CLOCK_SELECTION: crate::src::headers::stdlib::C2RustUnnamed_1 = 137;

    pub const _SC_CPUTIME: crate::src::headers::stdlib::C2RustUnnamed_1 = 138;

    pub const _SC_THREAD_CPUTIME: crate::src::headers::stdlib::C2RustUnnamed_1 = 139;

    pub const _SC_DEVICE_IO: crate::src::headers::stdlib::C2RustUnnamed_1 = 140;

    pub const _SC_DEVICE_SPECIFIC: crate::src::headers::stdlib::C2RustUnnamed_1 = 141;

    pub const _SC_DEVICE_SPECIFIC_R: crate::src::headers::stdlib::C2RustUnnamed_1 = 142;

    pub const _SC_FD_MGMT: crate::src::headers::stdlib::C2RustUnnamed_1 = 143;

    pub const _SC_FIFO: crate::src::headers::stdlib::C2RustUnnamed_1 = 144;

    pub const _SC_PIPE: crate::src::headers::stdlib::C2RustUnnamed_1 = 145;

    pub const _SC_FILE_ATTRIBUTES: crate::src::headers::stdlib::C2RustUnnamed_1 = 146;

    pub const _SC_FILE_LOCKING: crate::src::headers::stdlib::C2RustUnnamed_1 = 147;

    pub const _SC_FILE_SYSTEM: crate::src::headers::stdlib::C2RustUnnamed_1 = 148;

    pub const _SC_MONOTONIC_CLOCK: crate::src::headers::stdlib::C2RustUnnamed_1 = 149;

    pub const _SC_MULTI_PROCESS: crate::src::headers::stdlib::C2RustUnnamed_1 = 150;

    pub const _SC_SINGLE_PROCESS: crate::src::headers::stdlib::C2RustUnnamed_1 = 151;

    pub const _SC_NETWORKING: crate::src::headers::stdlib::C2RustUnnamed_1 = 152;

    pub const _SC_READER_WRITER_LOCKS: crate::src::headers::stdlib::C2RustUnnamed_1 = 153;

    pub const _SC_SPIN_LOCKS: crate::src::headers::stdlib::C2RustUnnamed_1 = 154;

    pub const _SC_REGEXP: crate::src::headers::stdlib::C2RustUnnamed_1 = 155;

    pub const _SC_REGEX_VERSION: crate::src::headers::stdlib::C2RustUnnamed_1 = 156;

    pub const _SC_SHELL: crate::src::headers::stdlib::C2RustUnnamed_1 = 157;

    pub const _SC_SIGNALS: crate::src::headers::stdlib::C2RustUnnamed_1 = 158;

    pub const _SC_SPAWN: crate::src::headers::stdlib::C2RustUnnamed_1 = 159;

    pub const _SC_SPORADIC_SERVER: crate::src::headers::stdlib::C2RustUnnamed_1 = 160;

    pub const _SC_THREAD_SPORADIC_SERVER: crate::src::headers::stdlib::C2RustUnnamed_1 = 161;

    pub const _SC_SYSTEM_DATABASE: crate::src::headers::stdlib::C2RustUnnamed_1 = 162;

    pub const _SC_SYSTEM_DATABASE_R: crate::src::headers::stdlib::C2RustUnnamed_1 = 163;

    pub const _SC_TIMEOUTS: crate::src::headers::stdlib::C2RustUnnamed_1 = 164;

    pub const _SC_TYPED_MEMORY_OBJECTS: crate::src::headers::stdlib::C2RustUnnamed_1 = 165;

    pub const _SC_USER_GROUPS: crate::src::headers::stdlib::C2RustUnnamed_1 = 166;

    pub const _SC_USER_GROUPS_R: crate::src::headers::stdlib::C2RustUnnamed_1 = 167;

    pub const _SC_2_PBS: crate::src::headers::stdlib::C2RustUnnamed_1 = 168;

    pub const _SC_2_PBS_ACCOUNTING: crate::src::headers::stdlib::C2RustUnnamed_1 = 169;

    pub const _SC_2_PBS_LOCATE: crate::src::headers::stdlib::C2RustUnnamed_1 = 170;

    pub const _SC_2_PBS_MESSAGE: crate::src::headers::stdlib::C2RustUnnamed_1 = 171;

    pub const _SC_2_PBS_TRACK: crate::src::headers::stdlib::C2RustUnnamed_1 = 172;

    pub const _SC_SYMLOOP_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 173;

    pub const _SC_STREAMS: crate::src::headers::stdlib::C2RustUnnamed_1 = 174;

    pub const _SC_2_PBS_CHECKPOINT: crate::src::headers::stdlib::C2RustUnnamed_1 = 175;

    pub const _SC_V6_ILP32_OFF32: crate::src::headers::stdlib::C2RustUnnamed_1 = 176;

    pub const _SC_V6_ILP32_OFFBIG: crate::src::headers::stdlib::C2RustUnnamed_1 = 177;

    pub const _SC_V6_LP64_OFF64: crate::src::headers::stdlib::C2RustUnnamed_1 = 178;

    pub const _SC_V6_LPBIG_OFFBIG: crate::src::headers::stdlib::C2RustUnnamed_1 = 179;

    pub const _SC_HOST_NAME_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 180;

    pub const _SC_TRACE: crate::src::headers::stdlib::C2RustUnnamed_1 = 181;

    pub const _SC_TRACE_EVENT_FILTER: crate::src::headers::stdlib::C2RustUnnamed_1 = 182;

    pub const _SC_TRACE_INHERIT: crate::src::headers::stdlib::C2RustUnnamed_1 = 183;

    pub const _SC_TRACE_LOG: crate::src::headers::stdlib::C2RustUnnamed_1 = 184;

    pub const _SC_LEVEL1_ICACHE_SIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 185;

    pub const _SC_LEVEL1_ICACHE_ASSOC: crate::src::headers::stdlib::C2RustUnnamed_1 = 186;

    pub const _SC_LEVEL1_ICACHE_LINESIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 187;

    pub const _SC_LEVEL1_DCACHE_SIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 188;

    pub const _SC_LEVEL1_DCACHE_ASSOC: crate::src::headers::stdlib::C2RustUnnamed_1 = 189;

    pub const _SC_LEVEL1_DCACHE_LINESIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 190;

    pub const _SC_LEVEL2_CACHE_SIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 191;

    pub const _SC_LEVEL2_CACHE_ASSOC: crate::src::headers::stdlib::C2RustUnnamed_1 = 192;

    pub const _SC_LEVEL2_CACHE_LINESIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 193;

    pub const _SC_LEVEL3_CACHE_SIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 194;

    pub const _SC_LEVEL3_CACHE_ASSOC: crate::src::headers::stdlib::C2RustUnnamed_1 = 195;

    pub const _SC_LEVEL3_CACHE_LINESIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 196;

    pub const _SC_LEVEL4_CACHE_SIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 197;

    pub const _SC_LEVEL4_CACHE_ASSOC: crate::src::headers::stdlib::C2RustUnnamed_1 = 198;

    pub const _SC_LEVEL4_CACHE_LINESIZE: crate::src::headers::stdlib::C2RustUnnamed_1 = 199;

    pub const _SC_IPV6: crate::src::headers::stdlib::C2RustUnnamed_1 = 235;

    pub const _SC_RAW_SOCKETS: crate::src::headers::stdlib::C2RustUnnamed_1 = 236;

    pub const _SC_V7_ILP32_OFF32: crate::src::headers::stdlib::C2RustUnnamed_1 = 237;

    pub const _SC_V7_ILP32_OFFBIG: crate::src::headers::stdlib::C2RustUnnamed_1 = 238;

    pub const _SC_V7_LP64_OFF64: crate::src::headers::stdlib::C2RustUnnamed_1 = 239;

    pub const _SC_V7_LPBIG_OFFBIG: crate::src::headers::stdlib::C2RustUnnamed_1 = 240;

    pub const _SC_SS_REPL_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 241;

    pub const _SC_TRACE_EVENT_NAME_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 242;

    pub const _SC_TRACE_NAME_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 243;

    pub const _SC_TRACE_SYS_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 244;

    pub const _SC_TRACE_USER_EVENT_MAX: crate::src::headers::stdlib::C2RustUnnamed_1 = 245;

    pub const _SC_XOPEN_STREAMS: crate::src::headers::stdlib::C2RustUnnamed_1 = 246;

    pub const _SC_THREAD_ROBUST_PRIO_INHERIT: crate::src::headers::stdlib::C2RustUnnamed_1 = 247;

    pub const _SC_THREAD_ROBUST_PRIO_PROTECT: crate::src::headers::stdlib::C2RustUnnamed_1 = 248;

    pub const _SC_MINSIGSTKSZ: crate::src::headers::stdlib::C2RustUnnamed_1 = 249;

    pub const _SC_SIGSTKSZ: crate::src::headers::stdlib::C2RustUnnamed_1 = 250;
    pub type C2RustUnnamed_0_1 = ::core::ffi::c_uint;

    pub const _ISupper: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 256;

    pub const _ISlower: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 512;

    pub const _ISalpha: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 1024;

    pub const _ISdigit: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 2048;

    pub const _ISxdigit: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 4096;

    pub const _ISspace: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 8192;

    pub const _ISprint: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 16384;

    pub const _ISgraph: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 32768;

    pub const _ISblank: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 1;

    pub const _IScntrl: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 2;

    pub const _ISpunct: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 4;

    pub const _ISalnum: crate::src::headers::stdlib::C2RustUnnamed_0_1 = 8;
    pub const __O_LARGEFILE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const F_GETLK64: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

    pub const F_SETLK64: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;

    pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
    pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846f64;
    pub type C2RustUnnamed_1 = ::core::ffi::c_uint;

    pub const PTHREAD_MUTEX_TIMED_NP: crate::src::headers::stdlib::C2RustUnnamed_1 = 0;

    pub const PTHREAD_MUTEX_RECURSIVE_NP: crate::src::headers::stdlib::C2RustUnnamed_1 = 1;

    pub const PTHREAD_MUTEX_ERRORCHECK_NP: crate::src::headers::stdlib::C2RustUnnamed_1 = 2;

    pub const PTHREAD_MUTEX_ADAPTIVE_NP: crate::src::headers::stdlib::C2RustUnnamed_1 = 3;

    pub const PTHREAD_MUTEX_NORMAL: crate::src::headers::stdlib::C2RustUnnamed_1 = 0;

    pub const PTHREAD_MUTEX_RECURSIVE: crate::src::headers::stdlib::C2RustUnnamed_1 = 1;

    pub const PTHREAD_MUTEX_ERRORCHECK: crate::src::headers::stdlib::C2RustUnnamed_1 = 2;

    pub const PTHREAD_MUTEX_DEFAULT: crate::src::headers::stdlib::C2RustUnnamed_1 = 0;

    pub const PTHREAD_MUTEX_FAST_NP: crate::src::headers::stdlib::C2RustUnnamed_1 = 0;
    pub type pthread_t = ::core::ffi::c_ulong;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union pthread_mutexattr_t {
        pub __size: [::core::ffi::c_char; 4],
        pub __align: ::core::ffi::c_int,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union pthread_attr_t {
        pub __size: [::core::ffi::c_char; 56],
        pub __align: ::core::ffi::c_long,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union pthread_mutex_t {
        pub __data: crate::src::headers::stdlib::__pthread_mutex_s,
        pub __size: [::core::ffi::c_char; 40],
        pub __align: ::core::ffi::c_long,
    }
    pub type intptr_t = isize;

    pub type uintptr_t = usize;
    pub type int8_t = crate::src::headers::stdlib::__int8_t;

    pub type int16_t = crate::src::headers::stdlib::__int16_t;
    pub type uint8_t = crate::src::headers::stdlib::__uint8_t;

    pub type uint16_t = crate::src::headers::stdlib::__uint16_t;

    pub type uint32_t = crate::src::headers::stdlib::__uint32_t;
    pub type va_list = crate::__stdarg___gnuc_va_list_h::__gnuc_va_list;

    pub type off_t = crate::src::headers::stdlib::__off64_t;

    pub type off64_t = crate::src::headers::stdlib::__off64_t;

    pub type ssize_t = crate::src::headers::stdlib::__ssize_t;
    pub const FILENAME_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
    pub type __compar_fn_t = Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >;
    pub type _IO_lock_t = ();

    pub type _IO_FILE = libc::FILE;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __pthread_mutex_s {
        pub __lock: ::core::ffi::c_int,
        pub __count: ::core::ffi::c_uint,
        pub __owner: ::core::ffi::c_int,
        pub __nusers: ::core::ffi::c_uint,
        pub __kind: ::core::ffi::c_int,
        pub __spins: ::core::ffi::c_short,
        pub __elision: ::core::ffi::c_short,
        pub __list: crate::src::headers::stdlib::__pthread_list_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: crate::src::headers::stdlib::__dev_t,
        pub st_ino: crate::src::headers::stdlib::__ino_t,
        pub st_nlink: crate::src::headers::stdlib::__nlink_t,
        pub st_mode: crate::src::headers::stdlib::__mode_t,
        pub st_uid: crate::src::headers::stdlib::__uid_t,
        pub st_gid: crate::src::headers::stdlib::__gid_t,
        pub __pad0: ::core::ffi::c_int,
        pub st_rdev: crate::src::headers::stdlib::__dev_t,
        pub st_size: crate::src::headers::stdlib::__off_t,
        pub st_blksize: crate::src::headers::stdlib::__blksize_t,
        pub st_blocks: crate::src::headers::stdlib::__blkcnt_t,
        pub st_atim: ::libc::timespec,
        pub st_mtim: ::libc::timespec,
        pub st_ctim: ::libc::timespec,
        pub __glibc_reserved: [crate::src::headers::stdlib::__syscall_slong_t; 3],
    }
    pub type dev_t = crate::src::headers::stdlib::__dev_t;

    pub type gid_t = crate::src::headers::stdlib::__gid_t;

    pub type mode_t = crate::src::headers::stdlib::__mode_t;

    pub type uid_t = crate::src::headers::stdlib::__uid_t;

    pub type pid_t = crate::src::headers::stdlib::__pid_t;

    #[cfg(feature = "test")]
    pub type Tcl_WideInt = ::core::ffi::c_longlong;

    #[cfg(feature = "test")]
    pub type Tcl_Size = crate::__stddef_ptrdiff_t_h::ptrdiff_t;

    #[cfg(feature = "test")]
    pub type Tcl_DupInternalRepProc =
        unsafe extern "C" fn(*mut crate::src::headers::stdlib::Tcl_Obj, *mut crate::src::headers::stdlib::Tcl_Obj) -> ();

    #[cfg(feature = "test")]

    pub type Tcl_FreeInternalRepProc = unsafe extern "C" fn(*mut crate::src::headers::stdlib::Tcl_Obj) -> ();

    #[cfg(feature = "test")]
    pub type Tcl_SetFromAnyProc = unsafe extern "C" fn(
        *mut crate::src::headers::stdlib::Tcl_Interp,
        *mut crate::src::headers::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]

    pub type Tcl_UpdateStringProc = unsafe extern "C" fn(*mut crate::src::headers::stdlib::Tcl_Obj) -> ();

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeLengthProc =
        unsafe extern "C" fn(*mut crate::src::headers::stdlib::Tcl_Obj) -> crate::src::headers::stdlib::Tcl_Size;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeIndexProc = unsafe extern "C" fn(
        *mut crate::src::headers::stdlib::Tcl_Interp,
        *mut crate::src::headers::stdlib::Tcl_Obj,
        crate::src::headers::stdlib::Tcl_Size,
        *mut *mut crate::src::headers::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeSliceProc = unsafe extern "C" fn(
        *mut crate::src::headers::stdlib::Tcl_Interp,
        *mut crate::src::headers::stdlib::Tcl_Obj,
        crate::src::headers::stdlib::Tcl_Size,
        crate::src::headers::stdlib::Tcl_Size,
        *mut *mut crate::src::headers::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeReverseProc = unsafe extern "C" fn(
        *mut crate::src::headers::stdlib::Tcl_Interp,
        *mut crate::src::headers::stdlib::Tcl_Obj,
        *mut *mut crate::src::headers::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeGetElements = unsafe extern "C" fn(
        *mut crate::src::headers::stdlib::Tcl_Interp,
        *mut crate::src::headers::stdlib::Tcl_Obj,
        *mut crate::src::headers::stdlib::Tcl_Size,
        *mut *mut *mut crate::src::headers::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeSetElement = unsafe extern "C" fn(
        *mut crate::src::headers::stdlib::Tcl_Interp,
        *mut crate::src::headers::stdlib::Tcl_Obj,
        crate::src::headers::stdlib::Tcl_Size,
        *const *mut crate::src::headers::stdlib::Tcl_Obj,
        *mut crate::src::headers::stdlib::Tcl_Obj,
    ) -> *mut crate::src::headers::stdlib::Tcl_Obj;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeReplaceProc = unsafe extern "C" fn(
        *mut crate::src::headers::stdlib::Tcl_Interp,
        *mut crate::src::headers::stdlib::Tcl_Obj,
        crate::src::headers::stdlib::Tcl_Size,
        crate::src::headers::stdlib::Tcl_Size,
        crate::src::headers::stdlib::Tcl_Size,
        *const *mut crate::src::headers::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeInOperatorProc = unsafe extern "C" fn(
        *mut crate::src::headers::stdlib::Tcl_Interp,
        *mut crate::src::headers::stdlib::Tcl_Obj,
        *mut crate::src::headers::stdlib::Tcl_Obj,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct Tcl_ObjType {
        pub name: *const ::core::ffi::c_char,
        pub freeIntRepProc: Option<crate::src::headers::stdlib::Tcl_FreeInternalRepProc>,
        pub dupIntRepProc: Option<crate::src::headers::stdlib::Tcl_DupInternalRepProc>,
        pub updateStringProc: Option<crate::src::headers::stdlib::Tcl_UpdateStringProc>,
        pub setFromAnyProc: Option<crate::src::headers::stdlib::Tcl_SetFromAnyProc>,
        pub version: crate::__stddef_size_t_h::size_t,
        pub lengthProc: Option<crate::src::headers::stdlib::Tcl_ObjTypeLengthProc>,
        pub indexProc: Option<crate::src::headers::stdlib::Tcl_ObjTypeIndexProc>,
        pub sliceProc: Option<crate::src::headers::stdlib::Tcl_ObjTypeSliceProc>,
        pub reverseProc: Option<crate::src::headers::stdlib::Tcl_ObjTypeReverseProc>,
        pub getElementsProc: Option<crate::src::headers::stdlib::Tcl_ObjTypeGetElements>,
        pub setElementProc: Option<crate::src::headers::stdlib::Tcl_ObjTypeSetElement>,
        pub replaceProc: Option<crate::src::headers::stdlib::Tcl_ObjTypeReplaceProc>,
        pub inOperProc: Option<crate::src::headers::stdlib::Tcl_ObjTypeInOperatorProc>,
    }
    #[cfg(feature = "test")]
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union Tcl_ObjInternalRep {
        pub longValue: ::core::ffi::c_long,
        pub doubleValue: ::core::ffi::c_double,
        pub otherValuePtr: *mut ::core::ffi::c_void,
        pub wideValue: crate::src::headers::stdlib::Tcl_WideInt,
        pub twoPtrValue: crate::src::headers::stdlib::C2RustUnnamed_0,
        pub ptrAndLongRep: crate::src::headers::stdlib::C2RustUnnamed,
    }
    #[cfg(feature = "test")]
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct Tcl_Obj {
        pub refCount: crate::src::headers::stdlib::Tcl_Size,
        pub bytes: *mut ::core::ffi::c_char,
        pub length: crate::src::headers::stdlib::Tcl_Size,
        pub typePtr: *const crate::src::headers::stdlib::Tcl_ObjType,
        pub internalRep: crate::src::headers::stdlib::Tcl_ObjInternalRep,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct C2RustUnnamed_0 {
        pub ptr1: *mut ::core::ffi::c_void,
        pub ptr2: *mut ::core::ffi::c_void,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct C2RustUnnamed {
        pub ptr: *mut ::core::ffi::c_void,
        pub value: ::core::ffi::c_ulong,
    }

    pub type __pthread_list_t = crate::src::headers::stdlib::__pthread_internal_list;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct __pthread_internal_list {
        pub __prev: *mut crate::src::headers::stdlib::__pthread_internal_list,
        pub __next: *mut crate::src::headers::stdlib::__pthread_internal_list,
    }
    pub type time_t = crate::src::headers::stdlib::__time_t;
    pub type __int8_t = i8;

    pub type __uint8_t = u8;

    pub type __int16_t = i16;

    pub type __uint16_t = u16;

    pub type __uint32_t = u32;

    pub type __dev_t = ::core::ffi::c_ulong;

    pub type __uid_t = ::core::ffi::c_uint;

    pub type __gid_t = ::core::ffi::c_uint;

    pub type __ino_t = ::core::ffi::c_ulong;

    pub type __mode_t = ::core::ffi::c_uint;

    pub type __nlink_t = ::core::ffi::c_ulong;

    pub type __off_t = ::core::ffi::c_long;

    pub type __off64_t = ::core::ffi::c_long;

    pub type __pid_t = ::core::ffi::c_int;

    pub type __time_t = ::core::ffi::c_long;

    pub type __suseconds_t = ::core::ffi::c_long;

    pub type __blksize_t = ::core::ffi::c_long;

    pub type __blkcnt_t = ::core::ffi::c_long;

    pub type __ssize_t = ::core::ffi::c_long;

    pub type __syscall_slong_t = ::core::ffi::c_long;