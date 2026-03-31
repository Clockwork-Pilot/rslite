





pub use crate::__stddef_size_t_h::size_t;



pub use crate::stdlib::pthread_attr_t;pub use crate::stdlib::pthread_t;pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::src::util::sqlite3FaultSim;pub use crate::src::src::malloc::sqlite3Malloc;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct SQLiteThread {
    pub tid: crate::stdlib::pthread_t,
    pub done: ::core::ffi::c_int,
    pub pOut: *mut ::core::ffi::c_void,
    pub xTask: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    pub pIn: *mut ::core::ffi::c_void,
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3ThreadCreate(
    mut ppThread: *mut *mut SQLiteThread,
    mut xTask: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    mut pIn: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p: *mut SQLiteThread = ::core::ptr::null_mut::<SQLiteThread>();
    let mut rc: ::core::ffi::c_int = 0;
    *ppThread = ::core::ptr::null_mut::<SQLiteThread>();
    p = crate::src::src::malloc::sqlite3Malloc(::core::mem::size_of::<SQLiteThread>() as crate::src::ext::rtree::rtree::u64_0) as *mut SQLiteThread;
    if p.is_null() {
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SQLiteThread>() as crate::__stddef_size_t_h::size_t,
    );
    (*p).xTask = xTask;
    (*p).pIn = pIn;
    if crate::src::src::util::sqlite3FaultSim(200 as ::core::ffi::c_int) != 0 {
        rc = 1 as ::core::ffi::c_int;
    } else {
        rc = crate::stdlib::pthread_create(
            &raw mut (*p).tid,
            ::core::ptr::null::<crate::stdlib::pthread_attr_t>(),
            xTask,
            pIn,
        );
    }
    if rc != 0 {
        (*p).done = 1 as ::core::ffi::c_int;
        (*p).pOut = xTask.expect("non-null function pointer")(pIn);
    }
    *ppThread = p;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3ThreadJoin(
    mut p: *mut SQLiteThread,
    mut ppOut: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if p.is_null() {
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    if (*p).done != 0 {
        *ppOut = (*p).pOut;
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    } else {
        rc = if ::libc::pthread_join((*p).tid, ppOut) != 0 {
            crate::src::headers::sqlite3_h::SQLITE_ERROR
        } else {
            crate::src::headers::sqlite3_h::SQLITE_OK
        };
    }
    crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    rc
}
