unsafe extern "C" {
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
    fn abort() -> !;
    fn sqlite3OsClose(_: *mut sqlite3_file);
    fn sqlite3OsRead(
        _: *mut sqlite3_file,
        _: *mut ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsWrite(
        _: *mut sqlite3_file,
        _: *const ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsTruncate(_: *mut sqlite3_file, size: i64_0) -> ::core::ffi::c_int;
    fn sqlite3OsSync(_: *mut sqlite3_file, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3OsFileSize(_: *mut sqlite3_file, pSize: *mut i64_0) -> ::core::ffi::c_int;
    fn sqlite3OsLock(_: *mut sqlite3_file, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3OsUnlock(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsCheckReservedLock(
        id: *mut sqlite3_file,
        pResOut: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsFileControl(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsSectorSize(id: *mut sqlite3_file) -> ::core::ffi::c_int;
    fn sqlite3OsDeviceCharacteristics(id: *mut sqlite3_file) -> ::core::ffi::c_int;
    fn sqlite3OsOpen(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsDelete(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsAccess(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        pResOut: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsFullPathname(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsDlOpen(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3OsDlError(
        _: *mut sqlite3_vfs,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    );
    fn sqlite3OsDlSym(
        _: *mut sqlite3_vfs,
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
    ) -> Option<unsafe extern "C" fn() -> ()>;
    fn sqlite3OsDlClose(_: *mut sqlite3_vfs, _: *mut ::core::ffi::c_void);
    fn sqlite3OsRandomness(
        _: *mut sqlite3_vfs,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsSleep(_: *mut sqlite3_vfs, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
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
pub struct devsym_file {
    pub base: sqlite3_file,
    pub pReal: *mut sqlite3_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DevsymGlobal {
    pub pVfs: *mut sqlite3_vfs,
    pub iDeviceChar: ::core::ffi::c_int,
    pub iSectorSize: ::core::ffi::c_int,
    pub nWriteCrash: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DEVSYM_MAX_PATHNAME: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const DEVSYM_VFS_NAME: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"devsym\0")
};
pub const WRITECRASH_NAME: [::core::ffi::c_char; 11] = unsafe {
    ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"writecrash\0")
};
#[unsafe(no_mangle)]
pub static mut g: DevsymGlobal = DevsymGlobal {
    pVfs: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
    iDeviceChar: 0 as ::core::ffi::c_int,
    iSectorSize: 512 as ::core::ffi::c_int,
    nWriteCrash: 0 as ::core::ffi::c_int,
};
unsafe extern "C" fn devsymClose(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        sqlite3OsClose((*p).pReal);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn devsymRead(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsRead((*p).pReal, zBuf, iAmt, iOfst as i64_0);
    }
}
unsafe extern "C" fn devsymWrite(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsWrite((*p).pReal, zBuf, iAmt, iOfst as i64_0);
    }
}
unsafe extern "C" fn devsymTruncate(
    mut pFile: *mut sqlite3_file,
    mut size: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsTruncate((*p).pReal, size as i64_0);
    }
}
unsafe extern "C" fn devsymSync(
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsSync((*p).pReal, flags);
    }
}
unsafe extern "C" fn devsymFileSize(
    mut pFile: *mut sqlite3_file,
    mut pSize: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsFileSize((*p).pReal, pSize as *mut i64_0);
    }
}
unsafe extern "C" fn devsymLock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsLock((*p).pReal, eLock);
    }
}
unsafe extern "C" fn devsymUnlock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsUnlock((*p).pReal, eLock);
    }
}
unsafe extern "C" fn devsymCheckReservedLock(
    mut pFile: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsCheckReservedLock((*p).pReal, pResOut);
    }
}
unsafe extern "C" fn devsymFileControl(
    mut pFile: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsFileControl((*p).pReal, op, pArg);
    }
}
unsafe extern "C" fn devsymSectorSize(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        return g.iSectorSize;
    }
}
unsafe extern "C" fn devsymDeviceCharacteristics(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        return g.iDeviceChar;
    }
}
unsafe extern "C" fn devsymShmLock(
    mut pFile: *mut sqlite3_file,
    mut ofst: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return (*(*(*p).pReal).pMethods)
            .xShmLock
            .expect("non-null function pointer")((*p).pReal, ofst, n, flags);
    }
}
unsafe extern "C" fn devsymShmMap(
    mut pFile: *mut sqlite3_file,
    mut iRegion: ::core::ffi::c_int,
    mut szRegion: ::core::ffi::c_int,
    mut isWrite: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return (*(*(*p).pReal).pMethods)
            .xShmMap
            .expect(
                "non-null function pointer",
            )((*p).pReal, iRegion, szRegion, isWrite, pp);
    }
}
unsafe extern "C" fn devsymShmBarrier(mut pFile: *mut sqlite3_file) {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        (*(*(*p).pReal).pMethods)
            .xShmBarrier
            .expect("non-null function pointer")((*p).pReal);
    }
}
unsafe extern "C" fn devsymShmUnmap(
    mut pFile: *mut sqlite3_file,
    mut delFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return (*(*(*p).pReal).pMethods)
            .xShmUnmap
            .expect("non-null function pointer")((*p).pReal, delFlag);
    }
}
unsafe extern "C" fn devsymOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        static mut devsym_io_methods: sqlite3_io_methods = unsafe {
            sqlite3_io_methods {
                iVersion: 2 as ::core::ffi::c_int,
                xClose: Some(
                    devsymClose
                        as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
                ),
                xRead: Some(
                    devsymRead
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xWrite: Some(
                    devsymWrite
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            *const ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xTruncate: Some(
                    devsymTruncate
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xSync: Some(
                    devsymSync
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xFileSize: Some(
                    devsymFileSize
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            *mut sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xLock: Some(
                    devsymLock
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xUnlock: Some(
                    devsymUnlock
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xCheckReservedLock: Some(
                    devsymCheckReservedLock
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xFileControl: Some(
                    devsymFileControl
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                xSectorSize: Some(
                    devsymSectorSize
                        as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
                ),
                xDeviceCharacteristics: Some(
                    devsymDeviceCharacteristics
                        as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
                ),
                xShmMap: Some(
                    devsymShmMap
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                xShmLock: Some(
                    devsymShmLock
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xShmBarrier: Some(
                    devsymShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
                ),
                xShmUnmap: Some(
                    devsymShmUnmap
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xFetch: None,
                xUnfetch: None,
            }
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        (*p).pReal = p.offset(1 as ::core::ffi::c_int as isize) as *mut devsym_file
            as *mut sqlite3_file;
        rc = sqlite3OsOpen(g.pVfs, zName, (*p).pReal, flags, pOutFlags);
        if !(*(*p).pReal).pMethods.is_null() {
            (*pFile).pMethods = &raw mut devsym_io_methods;
        }
        return rc;
    }
}
unsafe extern "C" fn devsymDelete(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsDelete(g.pVfs, zPath, dirSync);
    }
}
unsafe extern "C" fn devsymAccess(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsAccess(g.pVfs, zPath, flags, pResOut);
    }
}
unsafe extern "C" fn devsymFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsFullPathname(g.pVfs, zPath, nOut, zOut);
    }
}
unsafe extern "C" fn devsymDlOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return sqlite3OsDlOpen(g.pVfs, zPath);
    }
}
unsafe extern "C" fn devsymDlError(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zErrMsg: *mut ::core::ffi::c_char,
) {
    unsafe {
        sqlite3OsDlError(g.pVfs, nByte, zErrMsg);
    }
}
unsafe extern "C" fn devsymDlSym(
    mut pVfs: *mut sqlite3_vfs,
    mut p: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        return sqlite3OsDlSym(g.pVfs, p, zSym);
    }
}
unsafe extern "C" fn devsymDlClose(
    mut pVfs: *mut sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    unsafe {
        sqlite3OsDlClose(g.pVfs, pHandle);
    }
}
unsafe extern "C" fn devsymRandomness(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsRandomness(g.pVfs, nByte, zBufOut);
    }
}
unsafe extern "C" fn devsymSleep(
    mut pVfs: *mut sqlite3_vfs,
    mut nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsSleep(g.pVfs, nMicro);
    }
}
unsafe extern "C" fn devsymCurrentTime(
    mut pVfs: *mut sqlite3_vfs,
    mut pTimeOut: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        return (*g.pVfs)
            .xCurrentTime
            .expect("non-null function pointer")(g.pVfs, pTimeOut);
    }
}
unsafe extern "C" fn writecrashSectorSize(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsSectorSize((*p).pReal);
    }
}
unsafe extern "C" fn writecrashDeviceCharacteristics(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        return sqlite3OsDeviceCharacteristics((*p).pReal);
    }
}
unsafe extern "C" fn writecrashWrite(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        if g.nWriteCrash > 0 as ::core::ffi::c_int {
            g.nWriteCrash -= 1;
            if g.nWriteCrash == 0 as ::core::ffi::c_int {
                abort();
            }
        }
        return sqlite3OsWrite((*p).pReal, zBuf, iAmt, iOfst as i64_0);
    }
}
unsafe extern "C" fn writecrashOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        static mut writecrash_io_methods: sqlite3_io_methods = unsafe {
            sqlite3_io_methods {
                iVersion: 2 as ::core::ffi::c_int,
                xClose: Some(
                    devsymClose
                        as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
                ),
                xRead: Some(
                    devsymRead
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xWrite: Some(
                    writecrashWrite
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            *const ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xTruncate: Some(
                    devsymTruncate
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xSync: Some(
                    devsymSync
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xFileSize: Some(
                    devsymFileSize
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            *mut sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xLock: Some(
                    devsymLock
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xUnlock: Some(
                    devsymUnlock
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xCheckReservedLock: Some(
                    devsymCheckReservedLock
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xFileControl: Some(
                    devsymFileControl
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                xSectorSize: Some(
                    writecrashSectorSize
                        as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
                ),
                xDeviceCharacteristics: Some(
                    writecrashDeviceCharacteristics
                        as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
                ),
                xShmMap: Some(
                    devsymShmMap
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                xShmLock: Some(
                    devsymShmLock
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xShmBarrier: Some(
                    devsymShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
                ),
                xShmUnmap: Some(
                    devsymShmUnmap
                        as unsafe extern "C" fn(
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xFetch: None,
                xUnfetch: None,
            }
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut devsym_file = pFile as *mut devsym_file;
        (*p).pReal = p.offset(1 as ::core::ffi::c_int as isize) as *mut devsym_file
            as *mut sqlite3_file;
        rc = sqlite3OsOpen(g.pVfs, zName, (*p).pReal, flags, pOutFlags);
        if !(*(*p).pReal).pMethods.is_null() {
            (*pFile).pMethods = &raw mut writecrash_io_methods;
        }
        return rc;
    }
}
static mut devsym_vfs: sqlite3_vfs = unsafe {
    sqlite3_vfs {
        iVersion: 2 as ::core::ffi::c_int,
        szOsFile: ::core::mem::size_of::<devsym_file>() as ::core::ffi::c_int,
        mxPathname: DEVSYM_MAX_PATHNAME,
        pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
        zName: DEVSYM_VFS_NAME.as_ptr(),
        pAppData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        xOpen: Some(
            devsymOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xDelete: Some(
            devsymDelete
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xAccess: Some(
            devsymAccess
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFullPathname: Some(
            devsymFullPathname
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xDlOpen: Some(
            devsymDlOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_void,
        ),
        xDlError: Some(
            devsymDlError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> (),
        ),
        xDlSym: Some(
            devsymDlSym
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> Option<unsafe extern "C" fn() -> ()>,
        ),
        xDlClose: Some(
            devsymDlClose
                as unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
        ),
        xRandomness: Some(
            devsymRandomness
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSleep: Some(
            devsymSleep
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCurrentTime: Some(
            devsymCurrentTime
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
        ),
        xGetLastError: None,
        xCurrentTimeInt64: None,
        xSetSystemCall: None,
        xGetSystemCall: None,
        xNextSystemCall: None,
    }
};
static mut writecrash_vfs: sqlite3_vfs = unsafe {
    sqlite3_vfs {
        iVersion: 2 as ::core::ffi::c_int,
        szOsFile: ::core::mem::size_of::<devsym_file>() as ::core::ffi::c_int,
        mxPathname: DEVSYM_MAX_PATHNAME,
        pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
        zName: WRITECRASH_NAME.as_ptr(),
        pAppData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        xOpen: Some(
            writecrashOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xDelete: Some(
            devsymDelete
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xAccess: Some(
            devsymAccess
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFullPathname: Some(
            devsymFullPathname
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xDlOpen: Some(
            devsymDlOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_void,
        ),
        xDlError: Some(
            devsymDlError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> (),
        ),
        xDlSym: Some(
            devsymDlSym
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> Option<unsafe extern "C" fn() -> ()>,
        ),
        xDlClose: Some(
            devsymDlClose
                as unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
        ),
        xRandomness: Some(
            devsymRandomness
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSleep: Some(
            devsymSleep
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCurrentTime: Some(
            devsymCurrentTime
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
        ),
        xGetLastError: None,
        xCurrentTimeInt64: None,
        xSetSystemCall: None,
        xGetSystemCall: None,
        xNextSystemCall: None,
    }
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn devsym_register(
    mut iDeviceChar: ::core::ffi::c_int,
    mut iSectorSize: ::core::ffi::c_int,
) {
    unsafe {
        if g.pVfs.is_null() {
            g.pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
            devsym_vfs.szOsFile += (*g.pVfs).szOsFile;
            writecrash_vfs.szOsFile += (*g.pVfs).szOsFile;
            sqlite3_vfs_register(&raw mut devsym_vfs, 0 as ::core::ffi::c_int);
            sqlite3_vfs_register(&raw mut writecrash_vfs, 0 as ::core::ffi::c_int);
        }
        if iDeviceChar >= 0 as ::core::ffi::c_int {
            g.iDeviceChar = iDeviceChar;
        } else {
            g.iDeviceChar = 0 as ::core::ffi::c_int;
        }
        if iSectorSize >= 0 as ::core::ffi::c_int {
            g.iSectorSize = iSectorSize;
        } else {
            g.iSectorSize = 512 as ::core::ffi::c_int;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn devsym_unregister() {
    unsafe {
        sqlite3_vfs_unregister(&raw mut devsym_vfs);
        sqlite3_vfs_unregister(&raw mut writecrash_vfs);
        g.pVfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        g.iDeviceChar = 0 as ::core::ffi::c_int;
        g.iSectorSize = 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn devsym_crash_on_write(mut nWrite: ::core::ffi::c_int) {
    unsafe {
        if g.pVfs.is_null() {
            g.pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
            devsym_vfs.szOsFile += (*g.pVfs).szOsFile;
            writecrash_vfs.szOsFile += (*g.pVfs).szOsFile;
            sqlite3_vfs_register(&raw mut devsym_vfs, 0 as ::core::ffi::c_int);
            sqlite3_vfs_register(&raw mut writecrash_vfs, 0 as ::core::ffi::c_int);
        }
        g.nWriteCrash = nWrite;
    }
}
