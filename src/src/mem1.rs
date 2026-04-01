


pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::src::main::sqlite3_config;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::src::printf::sqlite3_log;pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::SQLITE_CONFIG_MALLOC_1;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_OK;

unsafe extern "C" fn sqlite3MemMalloc(mut nByte: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    let mut p: *mut crate::src::headers::sqlite3_h::sqlite3_int64 = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_int64>();
    p = ::libc::malloc((nByte + 8 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t) as *mut crate::src::headers::sqlite3_h::sqlite3_int64;
    if !p.is_null() {
        *p.offset(0 as isize) = nByte as crate::src::headers::sqlite3_h::sqlite3_int64;
        p = p.offset(1);
    } else {
        crate::src::src::printf::sqlite3_log(
            crate::src::headers::sqlite3_h::SQLITE_NOMEM,
            b"failed to allocate %u bytes of memory\0" as *const u8 as *const ::core::ffi::c_char,
            nByte,
        );
    }
    p as *mut ::core::ffi::c_void
}

unsafe extern "C" fn sqlite3MemFree(mut pPrior: *mut ::core::ffi::c_void) {
    let mut p: *mut crate::src::headers::sqlite3_h::sqlite3_int64 = pPrior as *mut crate::src::headers::sqlite3_h::sqlite3_int64;
    p = p.offset(-1);
    ::libc::free(p as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn sqlite3MemSize(mut pPrior: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut p: *mut crate::src::headers::sqlite3_h::sqlite3_int64 = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_int64>();
    p = pPrior as *mut crate::src::headers::sqlite3_h::sqlite3_int64;
    p = p.offset(-1);
    *p.offset(0 as isize) as ::core::ffi::c_int
}

unsafe extern "C" fn sqlite3MemRealloc(
    mut pPrior: *mut ::core::ffi::c_void,
    mut nByte: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut crate::src::headers::sqlite3_h::sqlite3_int64 = pPrior as *mut crate::src::headers::sqlite3_h::sqlite3_int64;
    p = p.offset(-1);
    p = ::libc::realloc(
        p as *mut ::core::ffi::c_void,
        (nByte + 8 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
    ) as *mut crate::src::headers::sqlite3_h::sqlite3_int64;
    if !p.is_null() {
        *p.offset(0 as isize) = nByte as crate::src::headers::sqlite3_h::sqlite3_int64;
        p = p.offset(1);
    } else {
        crate::src::src::printf::sqlite3_log(
            crate::src::headers::sqlite3_h::SQLITE_NOMEM,
            b"failed memory resize %u to %u bytes\0" as *const u8 as *const ::core::ffi::c_char,
            sqlite3MemSize(pPrior),
            nByte,
        );
    }
    p as *mut ::core::ffi::c_void
}

unsafe extern "C" fn sqlite3MemRoundup(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    n + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int)
}

unsafe extern "C" fn sqlite3MemInit(mut _NotUsed: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn sqlite3MemShutdown(mut _NotUsed: *mut ::core::ffi::c_void) {}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3MemSetDefault() {
    static mut defaultMethods: crate::src::headers::sqlite3_h::sqlite3_mem_methods = {
        crate::src::headers::sqlite3_h::sqlite3_mem_methods {
    xMalloc:  Some(
                sqlite3MemMalloc
                    as unsafe extern "C" fn(::core::ffi::c_int) -> *mut ::core::ffi::c_void,
            ),
    xFree:  Some(sqlite3MemFree as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    xRealloc:  Some(
                sqlite3MemRealloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> *mut ::core::ffi::c_void,
            ),
    xSize:  Some(
                sqlite3MemSize
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
    xRoundup:  Some(
                sqlite3MemRoundup as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
    xInit:  Some(
                sqlite3MemInit
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
    xShutdown:  Some(
                sqlite3MemShutdown as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
    pAppData:  ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
}
    };
    crate::src::src::main::sqlite3_config(crate::src::headers::sqlite3_h::SQLITE_CONFIG_MALLOC_1, &raw const defaultMethods);
}
