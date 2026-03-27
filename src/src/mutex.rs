






pub use crate::src::src::main::sqlite3_initialize;pub use crate::sqlite3_h::sqlite3_int64;pub use crate::sqlite3_h::sqlite3_mem_methods;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::sqlite3_h::sqlite3_mutex_methods;pub use crate::sqlite3_h::sqlite3_pcache;pub use crate::sqlite3_h::sqlite3_pcache_methods2;pub use crate::sqlite3_h::sqlite3_pcache_page;pub use crate::sqlite3_h::sqlite_int64;pub use crate::sqlite3_h::SQLITE_MUTEX_RECURSIVE;pub use crate::sqlite3_h::SQLITE_OK;pub use crate::src::src::global::sqlite3Config;pub use crate::src::src::mutex_unix::sqlite3DefaultMutex;pub use crate::src::src::mutex_unix::sqlite3MemoryBarrier;pub use crate::src::src::mutex_noop::sqlite3NoopMutex;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::sqliteInt_h::Sqlite3Config;pub use crate::stdlib::uint32_t;pub use crate::stdlib::uint8_t;pub use crate::stdlib::__uint32_t;pub use crate::stdlib::__uint8_t;
#[no_mangle]

pub unsafe extern "C" fn sqlite3MutexInit() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_OK;
    if crate::src::src::global::sqlite3Config.mutex.xMutexAlloc.is_none() {
        let mut pFrom: *const crate::sqlite3_h::sqlite3_mutex_methods = ::core::ptr::null::<crate::sqlite3_h::sqlite3_mutex_methods>();
        let mut pTo: *mut crate::sqlite3_h::sqlite3_mutex_methods = &raw mut crate::src::src::global::sqlite3Config.mutex;
        if crate::src::src::global::sqlite3Config.bCoreMutex != 0 {
            pFrom =  crate::src::src::mutex_unix::sqlite3DefaultMutex() as
    *const crate::sqlite3_h::sqlite3_mutex_methods;
        } else {
            pFrom =  crate::src::src::mutex_noop::sqlite3NoopMutex() as
    *const crate::sqlite3_h::sqlite3_mutex_methods;
        }
        let __pTo_ref = unsafe { &mut *pTo };
        __pTo_ref.xMutexInit = (*pFrom).xMutexInit;
        __pTo_ref.xMutexEnd = (*pFrom).xMutexEnd;
        __pTo_ref.xMutexFree = (*pFrom).xMutexFree;
        __pTo_ref.xMutexEnter = (*pFrom).xMutexEnter;
        __pTo_ref.xMutexTry = (*pFrom).xMutexTry;
        __pTo_ref.xMutexLeave = (*pFrom).xMutexLeave;
        __pTo_ref.xMutexHeld = (*pFrom).xMutexHeld;
        __pTo_ref.xMutexNotheld = (*pFrom).xMutexNotheld;
        crate::src::src::mutex_unix::sqlite3MemoryBarrier();
        __pTo_ref.xMutexAlloc = (*pFrom).xMutexAlloc;
    }
    rc = crate::src::src::global::sqlite3Config
        .mutex
        .xMutexInit
        .expect("non-null function pointer")();
    crate::src::src::mutex_unix::sqlite3MemoryBarrier();
    return rc;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3MutexEnd() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_OK;
    if crate::src::src::global::sqlite3Config.mutex.xMutexEnd.is_some() {
        rc = crate::src::src::global::sqlite3Config
            .mutex
            .xMutexEnd
            .expect("non-null function pointer")();
    }
    return rc;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3_mutex_alloc(mut id: ::core::ffi::c_int) -> *mut crate::src::src::mutex_unix::sqlite3_mutex {
    if id <= crate::sqlite3_h::SQLITE_MUTEX_RECURSIVE && crate::src::src::main::sqlite3_initialize() != 0 {
        return ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    }
    if id > crate::sqlite3_h::SQLITE_MUTEX_RECURSIVE && sqlite3MutexInit() != 0 {
        return ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    }
    return crate::src::src::global::sqlite3Config
        .mutex
        .xMutexAlloc
        .expect("non-null function pointer")(id);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3MutexAlloc(mut id: ::core::ffi::c_int) -> *mut crate::src::src::mutex_unix::sqlite3_mutex {
    if crate::src::src::global::sqlite3Config.bCoreMutex == 0 {
        return ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    }
    return crate::src::src::global::sqlite3Config
        .mutex
        .xMutexAlloc
        .expect("non-null function pointer")(id);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3_mutex_free(mut p: *mut crate::src::src::mutex_unix::sqlite3_mutex) {
    if !p.is_null() {
        crate::src::src::global::sqlite3Config
            .mutex
            .xMutexFree
            .expect("non-null function pointer")(p);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3_mutex_enter(mut p: *mut crate::src::src::mutex_unix::sqlite3_mutex) {
    if !p.is_null() {
        crate::src::src::global::sqlite3Config
            .mutex
            .xMutexEnter
            .expect("non-null function pointer")(p);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3_mutex_try(mut p: *mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_OK;
    if !p.is_null() {
        return crate::src::src::global::sqlite3Config
            .mutex
            .xMutexTry
            .expect("non-null function pointer")(p);
    }
    return rc;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3_mutex_leave(mut p: *mut crate::src::src::mutex_unix::sqlite3_mutex) {
    if !p.is_null() {
        crate::src::src::global::sqlite3Config
            .mutex
            .xMutexLeave
            .expect("non-null function pointer")(p);
    }
}
