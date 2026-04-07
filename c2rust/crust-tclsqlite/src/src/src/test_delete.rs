unsafe extern "C" {
    fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MFile {
    pub zFmt: *const ::core::ffi::c_char,
    pub iOffset: ::core::ffi::c_int,
    pub b83: ::core::ffi::c_int,
}
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const MX_CHUNK_NUMBER: ::core::ffi::c_int = 299 as ::core::ffi::c_int;
pub const SQLITE_MULTIPLEX_JOURNAL_8_3_OFFSET: ::core::ffi::c_int = 400
    as ::core::ffi::c_int;
pub const SQLITE_MULTIPLEX_WAL_8_3_OFFSET: ::core::ffi::c_int = 700
    as ::core::ffi::c_int;
unsafe extern "C" fn sqlite3Delete83Name(mut z: *mut ::core::ffi::c_char) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut sz: ::core::ffi::c_int = 0;
        sz = strlen(z) as ::core::ffi::c_int;
        i = sz - 1 as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int
            && *z.offset(i as isize) as ::core::ffi::c_int != '/' as i32
            && *z.offset(i as isize) as ::core::ffi::c_int != '.' as i32
        {
            i -= 1;
        }
        if *z.offset(i as isize) as ::core::ffi::c_int == '.' as i32
            && sz > i + 4 as ::core::ffi::c_int
        {
            memmove(
                z.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                z.offset((sz - 3 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            );
        }
    }
}
unsafe extern "C" fn sqlite3DeleteUnlinkIfExists(
    mut pVfs: *mut sqlite3_vfs,
    mut zFile: *const ::core::ffi::c_char,
    mut pbExists: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_ERROR;
        rc = access(zFile, F_OK);
        if rc != 0 {
            if *__errno_location() == ENOENT {
                if !pbExists.is_null() {
                    *pbExists = 0 as ::core::ffi::c_int;
                }
                rc = SQLITE_OK;
            }
        } else {
            if !pbExists.is_null() {
                *pbExists = 1 as ::core::ffi::c_int;
            }
            rc = unlink(zFile);
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_delete_database(
    mut zFile: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nBuf: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut azFmt: [*const ::core::ffi::c_char; 4] = [
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            b"%s-journal\0".as_ptr() as *const ::core::ffi::c_char,
            b"%s-wal\0".as_ptr() as *const ::core::ffi::c_char,
            b"%s-shm\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut aMFile: [MFile; 6] = [
            MFile {
                zFmt: b"%s%03d\0".as_ptr() as *const ::core::ffi::c_char,
                iOffset: 0 as ::core::ffi::c_int,
                b83: 0 as ::core::ffi::c_int,
            },
            MFile {
                zFmt: b"%s-journal%03d\0".as_ptr() as *const ::core::ffi::c_char,
                iOffset: 0 as ::core::ffi::c_int,
                b83: 0 as ::core::ffi::c_int,
            },
            MFile {
                zFmt: b"%s-wal%03d\0".as_ptr() as *const ::core::ffi::c_char,
                iOffset: 0 as ::core::ffi::c_int,
                b83: 0 as ::core::ffi::c_int,
            },
            MFile {
                zFmt: b"%s%03d\0".as_ptr() as *const ::core::ffi::c_char,
                iOffset: 0 as ::core::ffi::c_int,
                b83: 1 as ::core::ffi::c_int,
            },
            MFile {
                zFmt: b"%s-journal%03d\0".as_ptr() as *const ::core::ffi::c_char,
                iOffset: SQLITE_MULTIPLEX_JOURNAL_8_3_OFFSET,
                b83: 1 as ::core::ffi::c_int,
            },
            MFile {
                zFmt: b"%s-wal%03d\0".as_ptr() as *const ::core::ffi::c_char,
                iOffset: SQLITE_MULTIPLEX_WAL_8_3_OFFSET,
                b83: 1 as ::core::ffi::c_int,
            },
        ];
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        nBuf = strlen(zFile) as ::core::ffi::c_int + 100 as ::core::ffi::c_int;
        zBuf = sqlite3_malloc(nBuf) as *mut ::core::ffi::c_char;
        if zBuf.is_null() {
            return SQLITE_NOMEM;
        }
        i = 0 as ::core::ffi::c_int;
        while rc == 0 as ::core::ffi::c_int
            && (i as usize)
                < (::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>() as usize)
                    .wrapping_div(
                        ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                    )
        {
            sqlite3_snprintf(nBuf, zBuf, azFmt[i as usize], zFile);
            rc = sqlite3DeleteUnlinkIfExists(
                pVfs,
                zBuf,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if rc == 0 as ::core::ffi::c_int && i != 0 as ::core::ffi::c_int {
                sqlite3Delete83Name(zBuf);
                rc = sqlite3DeleteUnlinkIfExists(
                    pVfs,
                    zBuf,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while rc == 0 as ::core::ffi::c_int
            && (i as usize)
                < (::core::mem::size_of::<[MFile; 6]>() as usize)
                    .wrapping_div(::core::mem::size_of::<MFile>() as usize)
        {
            let mut p: *mut MFile = (&raw mut aMFile as *mut MFile).offset(i as isize)
                as *mut MFile;
            let mut iChunk: ::core::ffi::c_int = 0;
            iChunk = 1 as ::core::ffi::c_int;
            while iChunk <= MX_CHUNK_NUMBER {
                let mut bExists: ::core::ffi::c_int = 0;
                sqlite3_snprintf(nBuf, zBuf, (*p).zFmt, zFile, iChunk + (*p).iOffset);
                if (*p).b83 != 0 {
                    sqlite3Delete83Name(zBuf);
                }
                rc = sqlite3DeleteUnlinkIfExists(pVfs, zBuf, &raw mut bExists);
                if bExists == 0 as ::core::ffi::c_int || rc != 0 as ::core::ffi::c_int {
                    break;
                }
                iChunk += 1;
            }
            i += 1;
        }
        sqlite3_free(zBuf as *mut ::core::ffi::c_void);
        return if rc != 0 { SQLITE_ERROR } else { SQLITE_OK };
    }
}
