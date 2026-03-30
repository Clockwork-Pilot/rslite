
pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;pub use crate::src::headers::sqlite3_h::SQLITE_OK;

unsafe extern "C" fn noopMutexInit() -> ::core::ffi::c_int {
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn noopMutexEnd() -> ::core::ffi::c_int {
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn noopMutexAlloc(mut _id: ::core::ffi::c_int) -> *mut crate::src::src::mutex_unix::sqlite3_mutex {
    8 as ::core::ffi::c_int as *mut crate::src::src::mutex_unix::sqlite3_mutex
}

unsafe extern "C" fn noopMutexFree(mut _p: *mut crate::src::src::mutex_unix::sqlite3_mutex) {}

unsafe extern "C" fn noopMutexEnter(mut _p: *mut crate::src::src::mutex_unix::sqlite3_mutex) {}

unsafe extern "C" fn noopMutexTry(mut _p: *mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int {
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn noopMutexLeave(mut _p: *mut crate::src::src::mutex_unix::sqlite3_mutex) {}
#[no_mangle]

pub unsafe extern "C" fn sqlite3NoopMutex() -> *const crate::src::headers::sqlite3_h::sqlite3_mutex_methods {
    static mut sMutex: crate::src::headers::sqlite3_h::sqlite3_mutex_methods = unsafe {
        crate::src::headers::sqlite3_h::sqlite3_mutex_methods {
    xMutexInit:  Some(noopMutexInit as unsafe extern "C" fn() -> ::core::ffi::c_int),
    xMutexEnd:  Some(noopMutexEnd as unsafe extern "C" fn() -> ::core::ffi::c_int),
    xMutexAlloc:  Some(
                noopMutexAlloc as unsafe extern "C" fn(::core::ffi::c_int) -> *mut crate::src::src::mutex_unix::sqlite3_mutex,
            ),
    xMutexFree:  Some(noopMutexFree as unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()),
    xMutexEnter:  Some(noopMutexEnter as unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()),
    xMutexTry:  Some(
                noopMutexTry as unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int,
            ),
    xMutexLeave:  Some(noopMutexLeave as unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()),
    xMutexHeld:  None,
    xMutexNotheld:  None,
}
    };
    &raw const sMutex
}
