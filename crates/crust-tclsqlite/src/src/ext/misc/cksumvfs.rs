unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_api_routines;
    pub type sqlite3_value;
    pub type sqlite3_context;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_create_function(
        db: *mut sqlite3,
        zFunctionName: *const ::core::ffi::c_char,
        nArg: ::core::ffi::c_int,
        eTextRep: ::core::ffi::c_int,
        pApp: *mut ::core::ffi::c_void,
        xFunc: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xStep: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_auto_extension(
        xEntryPoint: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_cancel_auto_extension(
        xEntryPoint: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strlike(
        zGlob: *const ::core::ffi::c_char,
        zStr: *const ::core::ffi::c_char,
        cEsc: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn sqlite3_log(
        iErrCode: ::core::ffi::c_int,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_file {
    pub pMethods: *const sqlite3_io_methods,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_io_methods {
    pub iVersion: ::core::ffi::c_int,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>,
    pub xRead: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xWrite: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xTruncate: Option<
        unsafe extern "C" fn(*mut sqlite3_file, sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xSync: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xFileSize: Option<
        unsafe extern "C" fn(*mut sqlite3_file, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xLock: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xUnlock: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xCheckReservedLock: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFileControl: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xSectorSize: Option<
        unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
    >,
    pub xDeviceCharacteristics: Option<
        unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
    >,
    pub xShmMap: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xShmLock: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xShmBarrier: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ()>,
    pub xShmUnmap: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xFetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            sqlite3_int64,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xUnfetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            sqlite3_int64,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
pub type sqlite3_filename = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vfs {
    pub iVersion: ::core::ffi::c_int,
    pub szOsFile: ::core::ffi::c_int,
    pub mxPathname: ::core::ffi::c_int,
    pub pNext: *mut sqlite3_vfs,
    pub zName: *const ::core::ffi::c_char,
    pub pAppData: *mut ::core::ffi::c_void,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            sqlite3_filename,
            *mut sqlite3_file,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xDelete: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xAccess: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFullPathname: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xDlOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub xDlError: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> (),
    >,
    pub xDlSym: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> Option<unsafe extern "C" fn() -> ()>,
    >,
    pub xDlClose: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
    >,
    pub xRandomness: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSleep: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xCurrentTime: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *mut ::core::ffi::c_double,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetLastError: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xCurrentTimeInt64: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xSetSystemCall: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            sqlite3_syscall_ptr,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetSystemCall: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> sqlite3_syscall_ptr,
    >,
    pub xNextSystemCall: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >,
}
pub type sqlite3_syscall_ptr = Option<unsafe extern "C" fn() -> ()>;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CksmFile {
    pub base: sqlite3_file,
    pub zFName: *const ::core::ffi::c_char,
    pub computeCksm: ::core::ffi::c_char,
    pub verifyCksm: ::core::ffi::c_char,
    pub pPartner: *mut CksmFile,
}
pub type u8_0 = ::core::ffi::c_uchar;
pub type u32_0 = ::core::ffi::c_uint;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_IOERR_DATA: ::core::ffi::c_int = SQLITE_IOERR
    | (32 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SUBPAGE_READ: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_VFSNAME: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_PRAGMA: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_INNOCUOUS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
static mut cksm_vfs: sqlite3_vfs = unsafe {
    sqlite3_vfs {
        iVersion: 3 as ::core::ffi::c_int,
        szOsFile: 0 as ::core::ffi::c_int,
        mxPathname: 1024 as ::core::ffi::c_int,
        pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
        zName: b"cksmvfs\0".as_ptr() as *const ::core::ffi::c_char,
        pAppData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        xOpen: Some(
            cksmOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xDelete: Some(
            cksmDelete
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xAccess: Some(
            cksmAccess
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFullPathname: Some(
            cksmFullPathname
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xDlOpen: Some(
            cksmDlOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_void,
        ),
        xDlError: Some(
            cksmDlError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> (),
        ),
        xDlSym: Some(
            cksmDlSym
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> Option<unsafe extern "C" fn() -> ()>,
        ),
        xDlClose: Some(
            cksmDlClose
                as unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
        ),
        xRandomness: Some(
            cksmRandomness
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSleep: Some(
            cksmSleep
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCurrentTime: Some(
            cksmCurrentTime
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
        ),
        xGetLastError: Some(
            cksmGetLastError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xCurrentTimeInt64: Some(
            cksmCurrentTimeInt64
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSetSystemCall: Some(
            cksmSetSystemCall
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    sqlite3_syscall_ptr,
                ) -> ::core::ffi::c_int,
        ),
        xGetSystemCall: Some(
            cksmGetSystemCall
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                ) -> sqlite3_syscall_ptr,
        ),
        xNextSystemCall: Some(
            cksmNextSystemCall
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
    }
};
static mut cksm_io_methods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 3 as ::core::ffi::c_int,
        xClose: Some(
            cksmClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            cksmRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            cksmWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            cksmTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            cksmSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            cksmFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            cksmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            cksmUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            cksmCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            cksmFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            cksmSectorSize
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            cksmDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: Some(
            cksmShmMap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xShmLock: Some(
            cksmShmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShmBarrier: Some(
            cksmShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
        ),
        xShmUnmap: Some(
            cksmShmUnmap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFetch: Some(
            cksmFetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xUnfetch: Some(
            cksmUnfetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn cksmCompute(
    mut a: *mut u8_0,
    mut nByte: ::core::ffi::c_int,
    mut aOut: *mut u8_0,
) {
    unsafe {
        let mut s1: u32_0 = 0 as u32_0;
        let mut s2: u32_0 = 0 as u32_0;
        let mut aData: *mut u32_0 = a as *mut u32_0;
        let mut aEnd: *mut u32_0 = a.offset(nByte as isize) as *mut u8_0 as *mut u32_0;
        let mut x: u32_0 = 1 as u32_0;
        if 1 as ::core::ffi::c_int == *(&raw mut x as *mut u8_0) as ::core::ffi::c_int {
            loop {
                let c2rust_fresh0 = aData;
                aData = aData.offset(1);
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh0).wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                let c2rust_fresh1 = aData;
                aData = aData.offset(1);
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*c2rust_fresh1).wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                if !(aData < aEnd) {
                    break;
                }
            }
        } else {
            loop {
                s1 = (s1 as ::core::ffi::c_uint)
                    .wrapping_add(
                        ((*aData.offset(0 as ::core::ffi::c_int as isize)
                            & 0xff as u32_0) << 24 as ::core::ffi::c_int)
                            .wrapping_add(
                                (*aData.offset(0 as ::core::ffi::c_int as isize)
                                    & 0xff00 as u32_0) << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*aData.offset(0 as ::core::ffi::c_int as isize)
                                    & 0xff0000 as ::core::ffi::c_int as u32_0)
                                    >> 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*aData.offset(0 as ::core::ffi::c_int as isize)
                                    & 0xff000000 as u32_0) >> 24 as ::core::ffi::c_int,
                            )
                            .wrapping_add(s2) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                s2 = (s2 as ::core::ffi::c_uint)
                    .wrapping_add(
                        ((*aData.offset(1 as ::core::ffi::c_int as isize)
                            & 0xff as u32_0) << 24 as ::core::ffi::c_int)
                            .wrapping_add(
                                (*aData.offset(1 as ::core::ffi::c_int as isize)
                                    & 0xff00 as u32_0) << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*aData.offset(1 as ::core::ffi::c_int as isize)
                                    & 0xff0000 as ::core::ffi::c_int as u32_0)
                                    >> 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*aData.offset(1 as ::core::ffi::c_int as isize)
                                    & 0xff000000 as u32_0) >> 24 as ::core::ffi::c_int,
                            )
                            .wrapping_add(s1) as ::core::ffi::c_uint,
                    ) as u32_0 as u32_0;
                aData = aData.offset(2 as ::core::ffi::c_int as isize);
                if !(aData < aEnd) {
                    break;
                }
            }
            s1 = ((s1 as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                << 24 as ::core::ffi::c_int)
                .wrapping_add(
                    (s1 as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (s1 as ::core::ffi::c_uint
                        & 0xff0000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        >> 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (s1 as ::core::ffi::c_uint & 0xff000000 as ::core::ffi::c_uint)
                        >> 24 as ::core::ffi::c_int,
                ) as u32_0;
            s2 = ((s2 as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                << 24 as ::core::ffi::c_int)
                .wrapping_add(
                    (s2 as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (s2 as ::core::ffi::c_uint
                        & 0xff0000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        >> 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (s2 as ::core::ffi::c_uint & 0xff000000 as ::core::ffi::c_uint)
                        >> 24 as ::core::ffi::c_int,
                ) as u32_0;
        }
        memcpy(
            aOut as *mut ::core::ffi::c_void,
            &raw mut s1 as *const ::core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            aOut.offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            &raw mut s2 as *const ::core::ffi::c_void,
            4 as size_t,
        );
    }
}
unsafe extern "C" fn cksmVerifyFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut nByte: ::core::ffi::c_int = 0;
        let mut data: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut cksum: [u8_0; 8] = [0; 8];
        data = sqlite3_value_blob(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *mut u8_0;
        if data.is_null() {
            return;
        }
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            != SQLITE_BLOB
        {
            return;
        }
        nByte = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        if nByte < 512 as ::core::ffi::c_int || nByte > 65536 as ::core::ffi::c_int
            || nByte & nByte - 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            return;
        }
        cksmCompute(data, nByte - 8 as ::core::ffi::c_int, &raw mut cksum as *mut u8_0);
        sqlite3_result_int(
            context,
            (memcmp(
                data.offset(nByte as isize).offset(-(8 as ::core::ffi::c_int as isize))
                    as *const ::core::ffi::c_void,
                &raw mut cksum as *mut u8_0 as *const ::core::ffi::c_void,
                8 as size_t,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn cksmClose(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut CksmFile = pFile as *mut CksmFile;
        if !(*p).pPartner.is_null() {
            (*(*p).pPartner).pPartner = ::core::ptr::null_mut::<CksmFile>();
            (*p).pPartner = ::core::ptr::null_mut::<CksmFile>();
        }
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        return (*(*pFile).pMethods).xClose.expect("non-null function pointer")(pFile);
    }
}
unsafe extern "C" fn cksmSetFlags(
    mut p: *mut CksmFile,
    mut hasCorrectReserveSize: ::core::ffi::c_int,
) {
    unsafe {
        if hasCorrectReserveSize != (*p).computeCksm as ::core::ffi::c_int {
            (*p).verifyCksm = hasCorrectReserveSize as ::core::ffi::c_char;
            (*p).computeCksm = (*p).verifyCksm;
            if !(*p).pPartner.is_null() {
                (*(*p).pPartner).verifyCksm = hasCorrectReserveSize
                    as ::core::ffi::c_char;
                (*(*p).pPartner).computeCksm = hasCorrectReserveSize
                    as ::core::ffi::c_char;
            }
        }
    }
}
unsafe extern "C" fn cksmRead(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut CksmFile = pFile as *mut CksmFile;
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        rc = (*(*pFile).pMethods)
            .xRead
            .expect(
                "non-null function pointer",
            )(pFile, zBuf, iAmt, iOfst as sqlite3_int64);
        if rc == SQLITE_OK {
            if iOfst == 0 as ::core::ffi::c_longlong && iAmt >= 100 as ::core::ffi::c_int
                && (memcmp(
                    zBuf,
                    b"SQLite format 3\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    16 as size_t,
                ) == 0 as ::core::ffi::c_int
                    || memcmp(
                        zBuf,
                        b"ZV-\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        3 as size_t,
                    ) == 0 as ::core::ffi::c_int)
            {
                let mut d: *mut u8_0 = zBuf as *mut u8_0;
                let mut hasCorrectReserveSize: ::core::ffi::c_char = (*d
                    .offset(20 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 8 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_char;
                cksmSetFlags(p, hasCorrectReserveSize as ::core::ffi::c_int);
            }
            if iAmt >= 512 as ::core::ffi::c_int
                && iAmt & iAmt - 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (*p).verifyCksm as ::core::ffi::c_int != 0
            {
                let mut cksum: [u8_0; 8] = [0; 8];
                cksmCompute(
                    zBuf as *mut u8_0,
                    iAmt - 8 as ::core::ffi::c_int,
                    &raw mut cksum as *mut u8_0,
                );
                if memcmp(
                    (zBuf as *mut u8_0)
                        .offset(iAmt as isize)
                        .offset(-(8 as ::core::ffi::c_int as isize))
                        as *const ::core::ffi::c_void,
                    &raw mut cksum as *mut u8_0 as *const ::core::ffi::c_void,
                    8 as size_t,
                ) != 0 as ::core::ffi::c_int
                {
                    sqlite3_log(
                        SQLITE_IOERR_DATA,
                        b"checksum fault offset %lld of \"%s\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        iOfst,
                        (*p).zFName,
                    );
                    rc = SQLITE_IOERR_DATA;
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn cksmWrite(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut CksmFile = pFile as *mut CksmFile;
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        if iOfst == 0 as ::core::ffi::c_longlong && iAmt >= 100 as ::core::ffi::c_int
            && (memcmp(
                zBuf,
                b"SQLite format 3\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                16 as size_t,
            ) == 0 as ::core::ffi::c_int
                || memcmp(
                    zBuf,
                    b"ZV-\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    3 as size_t,
                ) == 0 as ::core::ffi::c_int)
        {
            let mut d: *mut u8_0 = zBuf as *mut u8_0;
            let mut hasCorrectReserveSize: ::core::ffi::c_char = (*d
                .offset(20 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 8 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_char;
            cksmSetFlags(p, hasCorrectReserveSize as ::core::ffi::c_int);
        }
        if iAmt >= 512 as ::core::ffi::c_int
            && (*p).computeCksm as ::core::ffi::c_int != 0
        {
            cksmCompute(
                zBuf as *mut u8_0,
                iAmt - 8 as ::core::ffi::c_int,
                (zBuf as *mut u8_0)
                    .offset(iAmt as isize)
                    .offset(-(8 as ::core::ffi::c_int as isize)),
            );
        }
        return (*(*pFile).pMethods)
            .xWrite
            .expect(
                "non-null function pointer",
            )(pFile, zBuf, iAmt, iOfst as sqlite3_int64);
    }
}
unsafe extern "C" fn cksmTruncate(
    mut pFile: *mut sqlite3_file,
    mut size: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        return (*(*pFile).pMethods)
            .xTruncate
            .expect("non-null function pointer")(pFile, size as sqlite3_int64);
    }
}
unsafe extern "C" fn cksmSync(
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        return (*(*pFile).pMethods)
            .xSync
            .expect("non-null function pointer")(pFile, flags);
    }
}
unsafe extern "C" fn cksmFileSize(
    mut pFile: *mut sqlite3_file,
    mut pSize: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut CksmFile = pFile as *mut CksmFile;
        pFile = p.offset(1 as ::core::ffi::c_int as isize) as *mut sqlite3_file;
        return (*(*pFile).pMethods)
            .xFileSize
            .expect("non-null function pointer")(pFile, pSize as *mut sqlite3_int64);
    }
}
unsafe extern "C" fn cksmLock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        return (*(*pFile).pMethods)
            .xLock
            .expect("non-null function pointer")(pFile, eLock);
    }
}
unsafe extern "C" fn cksmUnlock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        return (*(*pFile).pMethods)
            .xUnlock
            .expect("non-null function pointer")(pFile, eLock);
    }
}
unsafe extern "C" fn cksmCheckReservedLock(
    mut pFile: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        return (*(*pFile).pMethods)
            .xCheckReservedLock
            .expect("non-null function pointer")(pFile, pResOut);
    }
}
unsafe extern "C" fn cksmFileControl(
    mut pFile: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut CksmFile = pFile as *mut CksmFile;
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        if op == SQLITE_FCNTL_PRAGMA {
            let mut azArg: *mut *mut ::core::ffi::c_char = pArg
                as *mut *mut ::core::ffi::c_char;
            if sqlite3_stricmp(
                *azArg.offset(1 as ::core::ffi::c_int as isize),
                b"checksum_verification\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut zArg: *mut ::core::ffi::c_char = *azArg
                    .offset(2 as ::core::ffi::c_int as isize);
                if !zArg.is_null() {
                    if *zArg.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int >= '1' as i32
                        && *zArg.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int <= '9' as i32
                        || sqlite3_strlike(
                            b"enable%\0".as_ptr() as *const ::core::ffi::c_char,
                            zArg,
                            0 as ::core::ffi::c_uint,
                        ) == 0 as ::core::ffi::c_int
                        || sqlite3_stricmp(
                            b"yes\0".as_ptr() as *const ::core::ffi::c_char,
                            zArg,
                        ) == 0 as ::core::ffi::c_int
                        || sqlite3_stricmp(
                            b"on\0".as_ptr() as *const ::core::ffi::c_char,
                            zArg,
                        ) == 0 as ::core::ffi::c_int
                    {
                        (*p).verifyCksm = (*p).computeCksm;
                    } else {
                        (*p).verifyCksm = 0 as ::core::ffi::c_char;
                    }
                    if !(*p).pPartner.is_null() {
                        (*(*p).pPartner).verifyCksm = (*p).verifyCksm;
                    }
                }
                let ref mut c2rust_fresh2 = *azArg
                    .offset(0 as ::core::ffi::c_int as isize);
                *c2rust_fresh2 = sqlite3_mprintf(
                    b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).verifyCksm as ::core::ffi::c_int,
                );
                return SQLITE_OK;
            } else if (*p).computeCksm as ::core::ffi::c_int != 0
                && !(*azArg.offset(2 as ::core::ffi::c_int as isize)).is_null()
                && sqlite3_stricmp(
                    *azArg.offset(1 as ::core::ffi::c_int as isize),
                    b"page_size\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                return SQLITE_OK
            }
        }
        rc = (*(*pFile).pMethods)
            .xFileControl
            .expect("non-null function pointer")(pFile, op, pArg);
        if rc == SQLITE_OK && op == SQLITE_FCNTL_VFSNAME {
            let ref mut c2rust_fresh3 = *(pArg as *mut *mut ::core::ffi::c_char);
            *c2rust_fresh3 = sqlite3_mprintf(
                b"cksm/%z\0".as_ptr() as *const ::core::ffi::c_char,
                *(pArg as *mut *mut ::core::ffi::c_char),
            );
        }
        return rc;
    }
}
unsafe extern "C" fn cksmSectorSize(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        return (*(*pFile).pMethods)
            .xSectorSize
            .expect("non-null function pointer")(pFile);
    }
}
unsafe extern "C" fn cksmDeviceCharacteristics(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut devchar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        devchar = (*(*pFile).pMethods)
            .xDeviceCharacteristics
            .expect("non-null function pointer")(pFile);
        return devchar & !SQLITE_IOCAP_SUBPAGE_READ;
    }
}
unsafe extern "C" fn cksmShmMap(
    mut pFile: *mut sqlite3_file,
    mut iPg: ::core::ffi::c_int,
    mut pgsz: ::core::ffi::c_int,
    mut bExtend: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        return (*(*pFile).pMethods)
            .xShmMap
            .expect("non-null function pointer")(pFile, iPg, pgsz, bExtend, pp);
    }
}
unsafe extern "C" fn cksmShmLock(
    mut pFile: *mut sqlite3_file,
    mut offset: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        return (*(*pFile).pMethods)
            .xShmLock
            .expect("non-null function pointer")(pFile, offset, n, flags);
    }
}
unsafe extern "C" fn cksmShmBarrier(mut pFile: *mut sqlite3_file) {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        (*(*pFile).pMethods).xShmBarrier.expect("non-null function pointer")(pFile);
    }
}
unsafe extern "C" fn cksmShmUnmap(
    mut pFile: *mut sqlite3_file,
    mut deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        return (*(*pFile).pMethods)
            .xShmUnmap
            .expect("non-null function pointer")(pFile, deleteFlag);
    }
}
unsafe extern "C" fn cksmFetch(
    mut pFile: *mut sqlite3_file,
    mut iOfst: sqlite3_int64,
    mut iAmt: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut CksmFile = pFile as *mut CksmFile;
        if (*p).computeCksm != 0 {
            *pp = ::core::ptr::null_mut::<::core::ffi::c_void>();
            return SQLITE_OK;
        }
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        if (*(*pFile).pMethods).iVersion > 2 as ::core::ffi::c_int
            && (*(*pFile).pMethods).xFetch.is_some()
        {
            return (*(*pFile).pMethods)
                .xFetch
                .expect("non-null function pointer")(pFile, iOfst, iAmt, pp);
        }
        *pp = ::core::ptr::null_mut::<::core::ffi::c_void>();
        return SQLITE_OK;
    }
}
unsafe extern "C" fn cksmUnfetch(
    mut pFile: *mut sqlite3_file,
    mut iOfst: sqlite3_int64,
    mut pPage: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        pFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        if (*(*pFile).pMethods).iVersion > 2 as ::core::ffi::c_int
            && (*(*pFile).pMethods).xUnfetch.is_some()
        {
            return (*(*pFile).pMethods)
                .xUnfetch
                .expect("non-null function pointer")(pFile, iOfst, pPage);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn cksmOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut CksmFile = ::core::ptr::null_mut::<CksmFile>();
        let mut pSubFile: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut pSubVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut rc: ::core::ffi::c_int = 0;
        pSubVfs = (*pVfs).pAppData as *mut sqlite3_vfs;
        if flags & SQLITE_OPEN_MAIN_DB == 0 as ::core::ffi::c_int {
            return (*pSubVfs)
                .xOpen
                .expect(
                    "non-null function pointer",
                )(pSubVfs, zName as sqlite3_filename, pFile, flags, pOutFlags);
        }
        p = pFile as *mut CksmFile;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<CksmFile>() as size_t,
        );
        pSubFile = (pFile as *mut CksmFile).offset(1 as ::core::ffi::c_int as isize)
            as *mut sqlite3_file;
        (*pFile).pMethods = &raw const cksm_io_methods as *const sqlite3_io_methods;
        rc = (*pSubVfs)
            .xOpen
            .expect(
                "non-null function pointer",
            )(pSubVfs, zName as sqlite3_filename, pSubFile, flags, pOutFlags);
        if !(rc != 0) {
            (*p).zFName = zName;
        }
        if rc != 0 {
            (*pFile).pMethods = ::core::ptr::null::<sqlite3_io_methods>();
        }
        return rc;
    }
}
unsafe extern "C" fn cksmDelete(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xDelete
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, zPath, dirSync);
    }
}
unsafe extern "C" fn cksmAccess(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xAccess
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, zPath, flags, pResOut);
    }
}
unsafe extern "C" fn cksmFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xFullPathname
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, zPath, nOut, zOut);
    }
}
unsafe extern "C" fn cksmDlOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xDlOpen
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, zPath);
    }
}
unsafe extern "C" fn cksmDlError(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zErrMsg: *mut ::core::ffi::c_char,
) {
    unsafe {
        (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xDlError
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, nByte, zErrMsg);
    }
}
unsafe extern "C" fn cksmDlSym(
    mut pVfs: *mut sqlite3_vfs,
    mut p: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xDlSym
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, p, zSym);
    }
}
unsafe extern "C" fn cksmDlClose(
    mut pVfs: *mut sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xDlClose
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, pHandle);
    }
}
unsafe extern "C" fn cksmRandomness(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xRandomness
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, nByte, zBufOut);
    }
}
unsafe extern "C" fn cksmSleep(
    mut pVfs: *mut sqlite3_vfs,
    mut nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xSleep
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, nMicro);
    }
}
unsafe extern "C" fn cksmCurrentTime(
    mut pVfs: *mut sqlite3_vfs,
    mut pTimeOut: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xCurrentTime
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, pTimeOut);
    }
}
unsafe extern "C" fn cksmGetLastError(
    mut pVfs: *mut sqlite3_vfs,
    mut a: ::core::ffi::c_int,
    mut b: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xGetLastError
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, a, b);
    }
}
unsafe extern "C" fn cksmCurrentTimeInt64(
    mut pVfs: *mut sqlite3_vfs,
    mut p: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pOrig: *mut sqlite3_vfs = (*pVfs).pAppData as *mut sqlite3_vfs;
        let mut rc: ::core::ffi::c_int = 0;
        if (*pOrig).xCurrentTimeInt64.is_some() {
            rc = (*pOrig)
                .xCurrentTimeInt64
                .expect("non-null function pointer")(pOrig, p);
        } else {
            let mut r: ::core::ffi::c_double = 0.;
            rc = (*pOrig)
                .xCurrentTime
                .expect("non-null function pointer")(pOrig, &raw mut r);
            *p = (r * 86400000.0f64) as sqlite3_int64;
        }
        return rc;
    }
}
unsafe extern "C" fn cksmSetSystemCall(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pCall: sqlite3_syscall_ptr,
) -> ::core::ffi::c_int {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xSetSystemCall
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, zName, pCall);
    }
}
unsafe extern "C" fn cksmGetSystemCall(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
) -> sqlite3_syscall_ptr {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xGetSystemCall
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, zName);
    }
}
unsafe extern "C" fn cksmNextSystemCall(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*((*pVfs).pAppData as *mut sqlite3_vfs))
            .xNextSystemCall
            .expect(
                "non-null function pointer",
            )((*pVfs).pAppData as *mut sqlite3_vfs, zName);
    }
}
unsafe extern "C" fn cksmRegisterFunc(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if db.is_null() {
            return SQLITE_OK;
        }
        rc = sqlite3_create_function(
            db,
            b"verify_checksum\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            SQLITE_UTF8 | SQLITE_INNOCUOUS | SQLITE_DETERMINISTIC,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                cksmVerifyFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
        return rc;
    }
}
unsafe extern "C" fn cksmRegisterVfs() -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pOrig: *mut sqlite3_vfs = sqlite3_vfs_find(
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if pOrig.is_null() {
            return SQLITE_ERROR;
        }
        cksm_vfs.iVersion = (*pOrig).iVersion;
        cksm_vfs.pAppData = pOrig as *mut ::core::ffi::c_void;
        cksm_vfs.szOsFile = ((*pOrig).szOsFile as usize)
            .wrapping_add(::core::mem::size_of::<CksmFile>() as usize)
            as ::core::ffi::c_int;
        rc = sqlite3_vfs_register(&raw mut cksm_vfs, 1 as ::core::ffi::c_int);
        if rc == SQLITE_OK {
            rc = sqlite3_auto_extension(
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                    >,
                    Option<unsafe extern "C" fn() -> ()>,
                >(
                    Some(
                        cksmRegisterFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                ),
            );
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_register_cksumvfs(
    mut NotUsed: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return cksmRegisterVfs();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_unregister_cksumvfs() -> ::core::ffi::c_int {
    unsafe {
        if !sqlite3_vfs_find(b"cksmvfs\0".as_ptr() as *const ::core::ffi::c_char)
            .is_null()
        {
            sqlite3_vfs_unregister(&raw mut cksm_vfs);
            sqlite3_cancel_auto_extension(
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                    >,
                    Option<unsafe extern "C" fn() -> ()>,
                >(
                    Some(
                        cksmRegisterFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                ),
            );
        }
        return SQLITE_OK;
    }
}
