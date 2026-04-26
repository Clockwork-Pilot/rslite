
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_RECURSIVE;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;
pub use crate::src::headers::sqlite3_h::Sqlite3Pcache;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;
pub use crate::src::headers::sqliteInt_h::Sqlite3Config;
pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::src::global::sqlite3Config;
pub use crate::src::src::main::sqlite3_initialize;
pub use crate::src::src::mutex_noop::sqlite3NoopMutex;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::mutex_unix::sqlite3DefaultMutex;
pub use crate::src::src::mutex_unix::sqlite3MemoryBarrier;
pub unsafe extern "C" fn sqlite3MutexInit() -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    if (&raw const crate::src::src::global::sqlite3Config.mutex.xMutexAlloc)
        .read()
        .is_none()
    {
        let pFrom: *const crate::src::headers::sqlite3_h::sqlite3_mutex_methods;
        let pTo: *mut crate::src::headers::sqlite3_h::sqlite3_mutex_methods =
            &raw mut crate::src::src::global::sqlite3Config.mutex;
        if crate::src::src::global::sqlite3Config.bCoreMutex != 0 {
            pFrom = crate::src::src::mutex_unix::sqlite3DefaultMutex()
                as *const crate::src::headers::sqlite3_h::sqlite3_mutex_methods;
        } else {
            pFrom = crate::src::src::mutex_noop::sqlite3NoopMutex()
                as *const crate::src::headers::sqlite3_h::sqlite3_mutex_methods;
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
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3MutexEnd() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (&raw const crate::src::src::global::sqlite3Config.mutex.xMutexEnd)
        .read()
        .is_some()
    {
        rc = (&raw const crate::src::src::global::sqlite3Config.mutex.xMutexEnd)
            .read()
            .expect("non-null function pointer")();
    }
    rc
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_mutex_alloc(
    id: ::core::ffi::c_int,
) -> *mut crate::src::src::mutex_unix::sqlite3_mutex {
    if id <= crate::src::headers::sqlite3_h::SQLITE_MUTEX_RECURSIVE
        && crate::src::src::main::sqlite3_initialize() != 0
    {
        return ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    }
    if id > crate::src::headers::sqlite3_h::SQLITE_MUTEX_RECURSIVE && sqlite3MutexInit() != 0 {
        return ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    }
    crate::src::src::global::sqlite3Config
        .mutex
        .xMutexAlloc
        .expect("non-null function pointer")(id)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3MutexAlloc(
    id: ::core::ffi::c_int,
) -> *mut crate::src::src::mutex_unix::sqlite3_mutex {
    if crate::src::src::global::sqlite3Config.bCoreMutex == 0 {
        return ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    }
    crate::src::src::global::sqlite3Config
        .mutex
        .xMutexAlloc
        .expect("non-null function pointer")(id)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_mutex_free(
    p: *mut crate::src::src::mutex_unix::sqlite3_mutex,
) {
    if !p.is_null() {
        crate::src::src::global::sqlite3Config
            .mutex
            .xMutexFree
            .expect("non-null function pointer")(p);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_mutex_enter(
    p: *mut crate::src::src::mutex_unix::sqlite3_mutex,
) {
    if !p.is_null() {
        crate::src::src::global::sqlite3Config
            .mutex
            .xMutexEnter
            .expect("non-null function pointer")(p);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_mutex_try(
    p: *mut crate::src::src::mutex_unix::sqlite3_mutex,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !p.is_null() {
        return crate::src::src::global::sqlite3Config
            .mutex
            .xMutexTry
            .expect("non-null function pointer")(p);
    }
    rc
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_mutex_leave(
    p: *mut crate::src::src::mutex_unix::sqlite3_mutex,
) {
    if !p.is_null() {
        crate::src::src::global::sqlite3Config
            .mutex
            .xMutexLeave
            .expect("non-null function pointer")(p);
    }
}
