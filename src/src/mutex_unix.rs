






pub use crate::__stddef_null_h::NULL;



pub use crate::stdlib::pthread_mutex_destroy;pub use crate::stdlib::pthread_mutex_init;pub use crate::stdlib::pthread_mutex_lock;pub use crate::stdlib::pthread_mutex_trylock;pub use crate::stdlib::pthread_mutex_unlock;pub use crate::stdlib::pthread_mutexattr_destroy;pub use crate::stdlib::pthread_mutexattr_init;pub use crate::stdlib::pthread_mutexattr_settype;pub use crate::stdlib::C2RustUnnamed_1;pub use crate::stdlib::PTHREAD_MUTEX_ADAPTIVE_NP;pub use crate::stdlib::PTHREAD_MUTEX_DEFAULT;pub use crate::stdlib::PTHREAD_MUTEX_ERRORCHECK;pub use crate::stdlib::PTHREAD_MUTEX_ERRORCHECK_NP;pub use crate::stdlib::PTHREAD_MUTEX_FAST_NP;pub use crate::stdlib::PTHREAD_MUTEX_NORMAL;pub use crate::stdlib::PTHREAD_MUTEX_RECURSIVE;pub use crate::stdlib::PTHREAD_MUTEX_RECURSIVE_NP;pub use crate::stdlib::PTHREAD_MUTEX_TIMED_NP;pub use crate::stdlib::pthread_mutex_t;pub use crate::stdlib::pthread_mutexattr_t;pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_BUSY;pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_FAST_1;pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_RECURSIVE_1;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::src::malloc::sqlite3MallocZero;pub use crate::src::ext::rtree::rtree::u64_0;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__pthread_internal_list;pub use crate::stdlib::__pthread_list_t;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_mutex {
    pub mutex: crate::stdlib::pthread_mutex_t,
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3MemoryBarrier() {
    ::core::intrinsics::atomic_fence_seqcst();
}

unsafe extern "C" fn pthreadMutexInit() -> ::core::ffi::c_int {
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn pthreadMutexEnd() -> ::core::ffi::c_int {
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn pthreadMutexAlloc(mut iType: ::core::ffi::c_int) -> *mut sqlite3_mutex {
    static mut staticMutexes: [sqlite3_mutex; 12] = [
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
        sqlite3_mutex {
            mutex: crate::stdlib::pthread_mutex_t {
    __data:  crate::stdlib::__pthread_mutex_s {
    __lock:  0 as ::core::ffi::c_int,
    __count:  0 as ::core::ffi::c_uint,
    __owner:  0 as ::core::ffi::c_int,
    __nusers:  0 as ::core::ffi::c_uint,
    __kind:  crate::stdlib::PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
    __spins:  0 as ::core::ffi::c_short,
    __elision:  0 as ::core::ffi::c_short,
    __list:  crate::stdlib::__pthread_internal_list {
    __prev:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
    __next:  ::core::ptr::null::<crate::stdlib::__pthread_internal_list>()
                            as *mut crate::stdlib::__pthread_internal_list,
},
},
},
        },
    ];
    let mut p: *mut sqlite3_mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
    match  iType {
    crate::src::headers::sqlite3_h::SQLITE_MUTEX_RECURSIVE_1 =>  {
            p = crate::src::src::malloc::sqlite3MallocZero(::core::mem::size_of::<sqlite3_mutex>() as crate::src::ext::rtree::rtree::u64_0)
                as *mut sqlite3_mutex;
            if !p.is_null() {
                let mut recursiveAttr: crate::stdlib::pthread_mutexattr_t = crate::stdlib::pthread_mutexattr_t { __size:  [0; 4] };
                crate::stdlib::pthread_mutexattr_init(&raw mut recursiveAttr);
                crate::stdlib::pthread_mutexattr_settype(
                    &raw mut recursiveAttr,
                    crate::stdlib::PTHREAD_MUTEX_RECURSIVE as ::core::ffi::c_int,
                );
                crate::stdlib::pthread_mutex_init(&raw mut (*p).mutex, &raw mut recursiveAttr);
                crate::stdlib::pthread_mutexattr_destroy(&raw mut recursiveAttr);
            }
        }
    crate::src::headers::sqlite3_h::SQLITE_MUTEX_FAST_1 =>  {
            p = crate::src::src::malloc::sqlite3MallocZero(::core::mem::size_of::<sqlite3_mutex>() as crate::src::ext::rtree::rtree::u64_0)
                as *mut sqlite3_mutex;
            if !p.is_null() {
                crate::stdlib::pthread_mutex_init(
                    &raw mut (*p).mutex,
                    ::core::ptr::null::<crate::stdlib::pthread_mutexattr_t>(),
                );
            }
        }
    _ =>  {
            p = (&raw mut staticMutexes as *mut sqlite3_mutex)
                .offset((iType - 2 as ::core::ffi::c_int) as isize)
                as *mut sqlite3_mutex;
        }
}
    p
}

unsafe extern "C" fn pthreadMutexFree(mut p: *mut sqlite3_mutex) {
    crate::stdlib::pthread_mutex_destroy(&raw mut (*p).mutex);
    crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn pthreadMutexEnter(mut p: *mut sqlite3_mutex) {
    crate::stdlib::pthread_mutex_lock(&raw mut (*p).mutex);
}

unsafe extern "C" fn pthreadMutexTry(mut p: *mut sqlite3_mutex) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if crate::stdlib::pthread_mutex_trylock(&raw mut (*p).mutex) == 0 as ::core::ffi::c_int {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    } else {
        rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
    }
    rc
}

unsafe extern "C" fn pthreadMutexLeave(mut p: *mut sqlite3_mutex) {
    crate::stdlib::pthread_mutex_unlock(&raw mut (*p).mutex);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3DefaultMutex() -> *const crate::src::headers::sqlite3_h::sqlite3_mutex_methods {
    static mut sMutex: crate::src::headers::sqlite3_h::sqlite3_mutex_methods = unsafe {
        crate::src::headers::sqlite3_h::sqlite3_mutex_methods {
    xMutexInit:  Some(pthreadMutexInit as unsafe extern "C" fn() -> ::core::ffi::c_int),
    xMutexEnd:  Some(pthreadMutexEnd as unsafe extern "C" fn() -> ::core::ffi::c_int),
    xMutexAlloc:  Some(
                pthreadMutexAlloc as unsafe extern "C" fn(::core::ffi::c_int) -> *mut sqlite3_mutex,
            ),
    xMutexFree:  Some(pthreadMutexFree as unsafe extern "C" fn(*mut sqlite3_mutex) -> ()),
    xMutexEnter:  Some(pthreadMutexEnter as unsafe extern "C" fn(*mut sqlite3_mutex) -> ()),
    xMutexTry:  Some(
                pthreadMutexTry as unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
            ),
    xMutexLeave:  Some(pthreadMutexLeave as unsafe extern "C" fn(*mut sqlite3_mutex) -> ()),
    xMutexHeld:  None,
    xMutexNotheld:  None,
}
    };
    &raw const sMutex
}
