unsafe extern "C" {
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub struct fs_real_file {
    pub pFile: *mut sqlite3_file,
    pub zName: *const ::core::ffi::c_char,
    pub nDatabase: ::core::ffi::c_int,
    pub nJournal: ::core::ffi::c_int,
    pub nBlob: ::core::ffi::c_int,
    pub nRef: ::core::ffi::c_int,
    pub pNext: *mut fs_real_file,
    pub ppThis: *mut *mut fs_real_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_file {
    pub base: sqlite3_file,
    pub eType: ::core::ffi::c_int,
    pub pReal: *mut fs_real_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tmp_file {
    pub base: sqlite3_file,
    pub nSize: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
    pub zAlloc: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_vfs_t {
    pub base: sqlite3_vfs,
    pub pFileList: *mut fs_real_file,
    pub pParent: *mut sqlite3_vfs,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_FULL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TEMP_DB: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_JOURNAL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_SYNC_DATAONLY: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_PRAGMA: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_ACCESS_EXISTS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BLOCKSIZE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const BLOBSIZE: ::core::ffi::c_int = 10485760 as ::core::ffi::c_int;
pub const FS_VFS_NAME: [::core::ffi::c_char; 3] = unsafe {
    ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"fs\0")
};
pub const DATABASE_FILE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JOURNAL_FILE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut fs_vfs: fs_vfs_t = unsafe {
    fs_vfs_t {
        base: sqlite3_vfs {
            iVersion: 1 as ::core::ffi::c_int,
            szOsFile: 0 as ::core::ffi::c_int,
            mxPathname: 0 as ::core::ffi::c_int,
            pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
            zName: FS_VFS_NAME.as_ptr(),
            pAppData: ::core::ptr::null::<::core::ffi::c_void>()
                as *mut ::core::ffi::c_void,
            xOpen: Some(
                fsOpen
                    as unsafe extern "C" fn(
                        *mut sqlite3_vfs,
                        *const ::core::ffi::c_char,
                        *mut sqlite3_file,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xDelete: Some(
                fsDelete
                    as unsafe extern "C" fn(
                        *mut sqlite3_vfs,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xAccess: Some(
                fsAccess
                    as unsafe extern "C" fn(
                        *mut sqlite3_vfs,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xFullPathname: Some(
                fsFullPathname
                    as unsafe extern "C" fn(
                        *mut sqlite3_vfs,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            xDlOpen: Some(
                fsDlOpen
                    as unsafe extern "C" fn(
                        *mut sqlite3_vfs,
                        *const ::core::ffi::c_char,
                    ) -> *mut ::core::ffi::c_void,
            ),
            xDlError: Some(
                fsDlError
                    as unsafe extern "C" fn(
                        *mut sqlite3_vfs,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_char,
                    ) -> (),
            ),
            xDlSym: Some(
                fsDlSym
                    as unsafe extern "C" fn(
                        *mut sqlite3_vfs,
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                    ) -> Option<unsafe extern "C" fn() -> ()>,
            ),
            xDlClose: Some(
                fsDlClose
                    as unsafe extern "C" fn(
                        *mut sqlite3_vfs,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            xRandomness: Some(
                fsRandomness
                    as unsafe extern "C" fn(
                        *mut sqlite3_vfs,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            xSleep: Some(
                fsSleep
                    as unsafe extern "C" fn(
                        *mut sqlite3_vfs,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xCurrentTime: Some(
                fsCurrentTime
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
        },
        pFileList: ::core::ptr::null::<fs_real_file>() as *mut fs_real_file,
        pParent: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
    }
};
static mut fs_io_methods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 1 as ::core::ffi::c_int,
        xClose: Some(
            fsClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            fsRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            fsWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            fsTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            fsSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            fsFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            fsLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            fsUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            fsCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            fsFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            fsSectorSize as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            fsDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: None,
        xShmLock: None,
        xShmBarrier: None,
        xShmUnmap: None,
        xFetch: None,
        xUnfetch: None,
    }
};
static mut tmp_io_methods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 1 as ::core::ffi::c_int,
        xClose: Some(
            tmpClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            tmpRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            tmpWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            tmpTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            tmpSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            tmpFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            tmpLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            tmpUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            tmpCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            tmpFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            tmpSectorSize
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            tmpDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: None,
        xShmLock: None,
        xShmBarrier: None,
        xShmUnmap: None,
        xFetch: None,
        xUnfetch: None,
    }
};
unsafe extern "C" fn tmpClose(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut pTmp: *mut tmp_file = pFile as *mut tmp_file;
        sqlite3_free((*pTmp).zAlloc as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tmpRead(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTmp: *mut tmp_file = pFile as *mut tmp_file;
        if iAmt as sqlite_int64 + iOfst > (*pTmp).nSize as ::core::ffi::c_longlong {
            return SQLITE_IOERR_SHORT_READ;
        }
        memcpy(
            zBuf,
            (*pTmp).zAlloc.offset(iOfst as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            iAmt as size_t,
        );
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tmpWrite(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTmp: *mut tmp_file = pFile as *mut tmp_file;
        if iAmt as sqlite_int64 + iOfst > (*pTmp).nAlloc as ::core::ffi::c_longlong {
            let mut nNew: ::core::ffi::c_int = (2 as ::core::ffi::c_longlong
                * (iAmt as ::core::ffi::c_longlong + iOfst as ::core::ffi::c_longlong
                    + (*pTmp).nAlloc as ::core::ffi::c_longlong)) as ::core::ffi::c_int;
            let mut zNew: *mut ::core::ffi::c_char = sqlite3_realloc(
                (*pTmp).zAlloc as *mut ::core::ffi::c_void,
                nNew,
            ) as *mut ::core::ffi::c_char;
            if zNew.is_null() {
                return SQLITE_NOMEM;
            }
            (*pTmp).zAlloc = zNew;
            (*pTmp).nAlloc = nNew;
        }
        memcpy(
            (*pTmp).zAlloc.offset(iOfst as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            zBuf,
            iAmt as size_t,
        );
        (*pTmp).nSize = (if (*pTmp).nSize as ::core::ffi::c_longlong
            > iOfst as ::core::ffi::c_longlong + iAmt as ::core::ffi::c_longlong
        {
            (*pTmp).nSize as ::core::ffi::c_longlong
        } else {
            iOfst as ::core::ffi::c_longlong + iAmt as ::core::ffi::c_longlong
        }) as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tmpTruncate(
    mut pFile: *mut sqlite3_file,
    mut size: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTmp: *mut tmp_file = pFile as *mut tmp_file;
        (*pTmp).nSize = (if ((*pTmp).nSize as ::core::ffi::c_longlong) < size {
            (*pTmp).nSize as ::core::ffi::c_longlong
        } else {
            size as ::core::ffi::c_longlong
        }) as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tmpSync(
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tmpFileSize(
    mut pFile: *mut sqlite3_file,
    mut pSize: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTmp: *mut tmp_file = pFile as *mut tmp_file;
        *pSize = (*pTmp).nSize as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tmpLock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tmpUnlock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tmpCheckReservedLock(
    mut pFile: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        *pResOut = 0 as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tmpFileControl(
    mut pFile: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tmpSectorSize(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn tmpDeviceCharacteristics(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fsClose(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *mut fs_file = pFile as *mut fs_file;
        let mut pReal: *mut fs_real_file = (*p).pReal;
        (*pReal).nRef -= 1;
        if (*pReal).nRef == 0 as ::core::ffi::c_int {
            *(*pReal).ppThis = (*pReal).pNext;
            if !(*pReal).pNext.is_null() {
                (*(*pReal).pNext).ppThis = (*pReal).ppThis;
            }
            rc = (*(*(*pReal).pFile).pMethods)
                .xClose
                .expect("non-null function pointer")((*pReal).pFile);
            sqlite3_free(pReal as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn fsRead(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *mut fs_file = pFile as *mut fs_file;
        let mut pReal: *mut fs_real_file = (*p).pReal;
        let mut pF: *mut sqlite3_file = (*pReal).pFile;
        if (*p).eType == DATABASE_FILE
            && iAmt as sqlite_int64 + iOfst
                > (*pReal).nDatabase as ::core::ffi::c_longlong
            || (*p).eType == JOURNAL_FILE
                && iAmt as sqlite_int64 + iOfst
                    > (*pReal).nJournal as ::core::ffi::c_longlong
        {
            rc = SQLITE_IOERR_SHORT_READ;
        } else if (*p).eType == DATABASE_FILE {
            rc = (*(*pF).pMethods)
                .xRead
                .expect(
                    "non-null function pointer",
                )(pF, zBuf, iAmt, iOfst as sqlite3_int64 + BLOCKSIZE as sqlite3_int64);
        } else {
            let mut iRem: ::core::ffi::c_int = iAmt;
            let mut iBuf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut ii: ::core::ffi::c_int = iOfst as ::core::ffi::c_int;
            while iRem > 0 as ::core::ffi::c_int && rc == SQLITE_OK {
                let mut iRealOff: ::core::ffi::c_int = (*pReal).nBlob
                    - BLOCKSIZE * (ii / BLOCKSIZE + 1 as ::core::ffi::c_int)
                    + ii % BLOCKSIZE;
                let mut iRealAmt: ::core::ffi::c_int = if iRem
                    < 512 as ::core::ffi::c_int - iRealOff % 512 as ::core::ffi::c_int
                {
                    iRem
                } else {
                    512 as ::core::ffi::c_int - iRealOff % 512 as ::core::ffi::c_int
                };
                rc = (*(*pF).pMethods)
                    .xRead
                    .expect(
                        "non-null function pointer",
                    )(
                    pF,
                    (zBuf as *mut ::core::ffi::c_char).offset(iBuf as isize)
                        as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    iRealAmt,
                    iRealOff as sqlite3_int64,
                );
                ii += iRealAmt;
                iBuf += iRealAmt;
                iRem -= iRealAmt;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn fsWrite(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *mut fs_file = pFile as *mut fs_file;
        let mut pReal: *mut fs_real_file = (*p).pReal;
        let mut pF: *mut sqlite3_file = (*pReal).pFile;
        if (*p).eType == DATABASE_FILE {
            if iAmt as ::core::ffi::c_longlong + iOfst as ::core::ffi::c_longlong
                + BLOCKSIZE as ::core::ffi::c_longlong
                > ((*pReal).nBlob - (*pReal).nJournal) as ::core::ffi::c_longlong
            {
                rc = SQLITE_FULL;
            } else {
                rc = (*(*pF).pMethods)
                    .xWrite
                    .expect(
                        "non-null function pointer",
                    )(
                    pF,
                    zBuf,
                    iAmt,
                    iOfst as sqlite3_int64 + BLOCKSIZE as sqlite3_int64,
                );
                if rc == SQLITE_OK {
                    (*pReal).nDatabase = (if (*pReal).nDatabase
                        as ::core::ffi::c_longlong > iAmt as sqlite_int64 + iOfst
                    {
                        (*pReal).nDatabase as ::core::ffi::c_longlong
                    } else {
                        iAmt as ::core::ffi::c_longlong
                            + iOfst as ::core::ffi::c_longlong
                    }) as ::core::ffi::c_int;
                }
            }
        } else {
            let mut iRem: ::core::ffi::c_int = iAmt;
            let mut iBuf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut ii: ::core::ffi::c_int = iOfst as ::core::ffi::c_int;
            while iRem > 0 as ::core::ffi::c_int && rc == SQLITE_OK {
                let mut iRealOff: ::core::ffi::c_int = (*pReal).nBlob
                    - BLOCKSIZE * (ii / BLOCKSIZE + 1 as ::core::ffi::c_int)
                    + ii % BLOCKSIZE;
                let mut iRealAmt: ::core::ffi::c_int = if iRem
                    < 512 as ::core::ffi::c_int - iRealOff % 512 as ::core::ffi::c_int
                {
                    iRem
                } else {
                    512 as ::core::ffi::c_int - iRealOff % 512 as ::core::ffi::c_int
                };
                if iRealOff < (*pReal).nDatabase + BLOCKSIZE {
                    rc = SQLITE_FULL;
                } else {
                    rc = (*(*pF).pMethods)
                        .xWrite
                        .expect(
                            "non-null function pointer",
                        )(
                        pF,
                        (zBuf as *mut ::core::ffi::c_char).offset(iBuf as isize)
                            as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                        iRealAmt,
                        iRealOff as sqlite3_int64,
                    );
                    ii += iRealAmt;
                    iBuf += iRealAmt;
                    iRem -= iRealAmt;
                }
            }
            if rc == SQLITE_OK {
                (*pReal).nJournal = (if (*pReal).nJournal as ::core::ffi::c_longlong
                    > iAmt as sqlite_int64 + iOfst
                {
                    (*pReal).nJournal as ::core::ffi::c_longlong
                } else {
                    iAmt as ::core::ffi::c_longlong + iOfst as ::core::ffi::c_longlong
                }) as ::core::ffi::c_int;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn fsTruncate(
    mut pFile: *mut sqlite3_file,
    mut size: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut fs_file = pFile as *mut fs_file;
        let mut pReal: *mut fs_real_file = (*p).pReal;
        if (*p).eType == DATABASE_FILE {
            (*pReal).nDatabase = (if ((*pReal).nDatabase as ::core::ffi::c_longlong)
                < size
            {
                (*pReal).nDatabase as ::core::ffi::c_longlong
            } else {
                size as ::core::ffi::c_longlong
            }) as ::core::ffi::c_int;
        } else {
            (*pReal).nJournal = (if ((*pReal).nJournal as ::core::ffi::c_longlong) < size
            {
                (*pReal).nJournal as ::core::ffi::c_longlong
            } else {
                size as ::core::ffi::c_longlong
            }) as ::core::ffi::c_int;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fsSync(
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut fs_file = pFile as *mut fs_file;
        let mut pReal: *mut fs_real_file = (*p).pReal;
        let mut pRealFile: *mut sqlite3_file = (*pReal).pFile;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*p).eType == DATABASE_FILE {
            let mut zSize: [::core::ffi::c_uchar; 4] = [0; 4];
            zSize[0 as ::core::ffi::c_int as usize] = (((*pReal).nDatabase
                as ::core::ffi::c_uint & 0xff000000 as ::core::ffi::c_uint)
                >> 24 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            zSize[1 as ::core::ffi::c_int as usize] = (((*pReal).nDatabase
                & 0xff0000 as ::core::ffi::c_int) >> 16 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            zSize[2 as ::core::ffi::c_int as usize] = (((*pReal).nDatabase
                & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            zSize[3 as ::core::ffi::c_int as usize] = ((*pReal).nDatabase
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            rc = (*(*pRealFile).pMethods)
                .xWrite
                .expect(
                    "non-null function pointer",
                )(
                pRealFile,
                &raw mut zSize as *mut ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                4 as ::core::ffi::c_int,
                0 as sqlite3_int64,
            );
        }
        if rc == SQLITE_OK {
            rc = (*(*pRealFile).pMethods)
                .xSync
                .expect(
                    "non-null function pointer",
                )(pRealFile, flags & !SQLITE_SYNC_DATAONLY);
        }
        return rc;
    }
}
unsafe extern "C" fn fsFileSize(
    mut pFile: *mut sqlite3_file,
    mut pSize: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut fs_file = pFile as *mut fs_file;
        let mut pReal: *mut fs_real_file = (*p).pReal;
        if (*p).eType == DATABASE_FILE {
            *pSize = (*pReal).nDatabase as sqlite_int64;
        } else {
            *pSize = (*pReal).nJournal as sqlite_int64;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fsLock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fsUnlock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fsCheckReservedLock(
    mut pFile: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        *pResOut = 0 as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fsFileControl(
    mut pFile: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        if op == SQLITE_FCNTL_PRAGMA {
            return SQLITE_NOTFOUND;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fsSectorSize(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        return BLOCKSIZE;
    }
}
unsafe extern "C" fn fsDeviceCharacteristics(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fsOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFsVfs: *mut fs_vfs_t = pVfs as *mut fs_vfs_t;
        let mut p: *mut fs_file = pFile as *mut fs_file;
        let mut pReal: *mut fs_real_file = ::core::ptr::null_mut::<fs_real_file>();
        let mut eType: ::core::ffi::c_int = 0;
        let mut nName: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if 0 as ::core::ffi::c_int
            == flags & (SQLITE_OPEN_MAIN_DB | SQLITE_OPEN_MAIN_JOURNAL)
        {
            let mut p2: *mut tmp_file = pFile as *mut tmp_file;
            memset(
                p2 as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<tmp_file>() as size_t,
            );
            (*p2).base.pMethods = &raw mut tmp_io_methods;
            return SQLITE_OK;
        }
        eType = if flags & 0x100 as ::core::ffi::c_int != 0 {
            DATABASE_FILE
        } else {
            JOURNAL_FILE
        };
        (*p).base.pMethods = &raw mut fs_io_methods;
        (*p).eType = eType;
        nName = strlen(zName) as ::core::ffi::c_int
            - (if eType == JOURNAL_FILE {
                8 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            });
        pReal = (*pFsVfs).pFileList;
        while !pReal.is_null() && strncmp((*pReal).zName, zName, nName as size_t) != 0 {
            pReal = (*pReal).pNext;
        }
        if pReal.is_null() {
            let mut real_flags: ::core::ffi::c_int = flags
                & !(0x100 as ::core::ffi::c_int) | SQLITE_OPEN_TEMP_DB;
            let mut size: sqlite3_int64 = 0;
            let mut pRealFile: *mut sqlite3_file = ::core::ptr::null_mut::<
                sqlite3_file,
            >();
            let mut pParent: *mut sqlite3_vfs = (*pFsVfs).pParent;
            pReal = sqlite3_malloc(
                (::core::mem::size_of::<fs_real_file>() as usize)
                    .wrapping_add((*pParent).szOsFile as usize) as ::core::ffi::c_int,
            ) as *mut fs_real_file;
            if pReal.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                memset(
                    pReal as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<fs_real_file>() as size_t)
                        .wrapping_add((*pParent).szOsFile as size_t),
                );
                (*pReal).zName = zName;
                (*pReal).pFile = pReal.offset(1 as ::core::ffi::c_int as isize)
                    as *mut fs_real_file as *mut sqlite3_file;
                rc = (*pParent)
                    .xOpen
                    .expect(
                        "non-null function pointer",
                    )(
                    pParent,
                    zName as sqlite3_filename,
                    (*pReal).pFile,
                    real_flags,
                    pOutFlags,
                );
                if !(rc != SQLITE_OK) {
                    pRealFile = (*pReal).pFile;
                    rc = (*(*pRealFile).pMethods)
                        .xFileSize
                        .expect("non-null function pointer")(pRealFile, &raw mut size);
                    if !(rc != SQLITE_OK) {
                        if size == 0 as ::core::ffi::c_longlong {
                            rc = (*(*pRealFile).pMethods)
                                .xWrite
                                .expect(
                                    "non-null function pointer",
                                )(
                                pRealFile,
                                b"\0\0".as_ptr() as *const ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                1 as ::core::ffi::c_int,
                                (BLOBSIZE - 1 as ::core::ffi::c_int) as sqlite3_int64,
                            );
                            (*pReal).nBlob = BLOBSIZE;
                        } else {
                            let mut zS: [::core::ffi::c_uchar; 4] = [0; 4];
                            (*pReal).nBlob = size as ::core::ffi::c_int;
                            rc = (*(*pRealFile).pMethods)
                                .xRead
                                .expect(
                                    "non-null function pointer",
                                )(
                                pRealFile,
                                &raw mut zS as *mut ::core::ffi::c_uchar
                                    as *mut ::core::ffi::c_void,
                                4 as ::core::ffi::c_int,
                                0 as sqlite3_int64,
                            );
                            (*pReal).nDatabase = ((zS[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int) << 24 as ::core::ffi::c_int)
                                + ((zS[1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                                + ((zS[2 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                + zS[3 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int;
                            if rc == SQLITE_OK {
                                rc = (*(*pRealFile).pMethods)
                                    .xRead
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    pRealFile,
                                    &raw mut zS as *mut ::core::ffi::c_uchar
                                        as *mut ::core::ffi::c_void,
                                    4 as ::core::ffi::c_int,
                                    ((*pReal).nBlob - 4 as ::core::ffi::c_int) as sqlite3_int64,
                                );
                                if zS[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int != 0
                                    || zS[1 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int != 0
                                    || zS[2 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int != 0
                                    || zS[3 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int != 0
                                {
                                    (*pReal).nJournal = (*pReal).nBlob;
                                }
                            }
                        }
                        if rc == SQLITE_OK {
                            (*pReal).pNext = (*pFsVfs).pFileList;
                            if !(*pReal).pNext.is_null() {
                                (*(*pReal).pNext).ppThis = &raw mut (*pReal).pNext;
                            }
                            (*pReal).ppThis = &raw mut (*pFsVfs).pFileList;
                            (*pFsVfs).pFileList = pReal;
                        }
                    }
                }
            }
        }
        if !pReal.is_null() {
            if rc == SQLITE_OK {
                (*p).pReal = pReal;
                (*pReal).nRef += 1;
            } else {
                if !(*(*pReal).pFile).pMethods.is_null() {
                    (*(*(*pReal).pFile).pMethods)
                        .xClose
                        .expect("non-null function pointer")((*pReal).pFile);
                }
                sqlite3_free(pReal as *mut ::core::ffi::c_void);
            }
        }
        return rc;
    }
}
unsafe extern "C" fn fsDelete(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pFsVfs: *mut fs_vfs_t = pVfs as *mut fs_vfs_t;
        let mut pReal: *mut fs_real_file = ::core::ptr::null_mut::<fs_real_file>();
        let mut pF: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut nName: ::core::ffi::c_int = strlen(zPath) as ::core::ffi::c_int
            - 8 as ::core::ffi::c_int;
        pReal = (*pFsVfs).pFileList;
        while !pReal.is_null() && strncmp((*pReal).zName, zPath, nName as size_t) != 0 {
            pReal = (*pReal).pNext;
        }
        if !pReal.is_null() {
            pF = (*pReal).pFile;
            rc = (*(*pF).pMethods)
                .xWrite
                .expect(
                    "non-null function pointer",
                )(
                pF,
                b"\0\0\0\0\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                4 as ::core::ffi::c_int,
                ((*pReal).nBlob - BLOCKSIZE) as sqlite3_int64,
            );
            if rc == SQLITE_OK {
                (*pReal).nJournal = 0 as ::core::ffi::c_int;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn fsAccess(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFsVfs: *mut fs_vfs_t = pVfs as *mut fs_vfs_t;
        let mut pReal: *mut fs_real_file = ::core::ptr::null_mut::<fs_real_file>();
        let mut isJournal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nName: ::core::ffi::c_int = strlen(zPath) as ::core::ffi::c_int;
        if flags != SQLITE_ACCESS_EXISTS {
            let mut pParent: *mut sqlite3_vfs = (*(pVfs as *mut fs_vfs_t)).pParent;
            return (*pParent)
                .xAccess
                .expect("non-null function pointer")(pParent, zPath, flags, pResOut);
        }
        if nName > 8 as ::core::ffi::c_int
            && strcmp(
                b"-journal\0".as_ptr() as *const ::core::ffi::c_char,
                zPath.offset((nName - 8 as ::core::ffi::c_int) as isize)
                    as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            nName -= 8 as ::core::ffi::c_int;
            isJournal = 1 as ::core::ffi::c_int;
        }
        pReal = (*pFsVfs).pFileList;
        while !pReal.is_null() && strncmp((*pReal).zName, zPath, nName as size_t) != 0 {
            pReal = (*pReal).pNext;
        }
        *pResOut = (!pReal.is_null()
            && (isJournal == 0 || (*pReal).nJournal > 0 as ::core::ffi::c_int))
            as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fsFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pParent: *mut sqlite3_vfs = (*(pVfs as *mut fs_vfs_t)).pParent;
        return (*pParent)
            .xFullPathname
            .expect("non-null function pointer")(pParent, zPath, nOut, zOut);
    }
}
unsafe extern "C" fn fsDlOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pParent: *mut sqlite3_vfs = (*(pVfs as *mut fs_vfs_t)).pParent;
        return (*pParent).xDlOpen.expect("non-null function pointer")(pParent, zPath);
    }
}
unsafe extern "C" fn fsDlError(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zErrMsg: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut pParent: *mut sqlite3_vfs = (*(pVfs as *mut fs_vfs_t)).pParent;
        (*pParent).xDlError.expect("non-null function pointer")(pParent, nByte, zErrMsg);
    }
}
unsafe extern "C" fn fsDlSym(
    mut pVfs: *mut sqlite3_vfs,
    mut pH: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        let mut pParent: *mut sqlite3_vfs = (*(pVfs as *mut fs_vfs_t)).pParent;
        return (*pParent).xDlSym.expect("non-null function pointer")(pParent, pH, zSym);
    }
}
unsafe extern "C" fn fsDlClose(
    mut pVfs: *mut sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut pParent: *mut sqlite3_vfs = (*(pVfs as *mut fs_vfs_t)).pParent;
        (*pParent).xDlClose.expect("non-null function pointer")(pParent, pHandle);
    }
}
unsafe extern "C" fn fsRandomness(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pParent: *mut sqlite3_vfs = (*(pVfs as *mut fs_vfs_t)).pParent;
        return (*pParent)
            .xRandomness
            .expect("non-null function pointer")(pParent, nByte, zBufOut);
    }
}
unsafe extern "C" fn fsSleep(
    mut pVfs: *mut sqlite3_vfs,
    mut nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pParent: *mut sqlite3_vfs = (*(pVfs as *mut fs_vfs_t)).pParent;
        return (*pParent).xSleep.expect("non-null function pointer")(pParent, nMicro);
    }
}
unsafe extern "C" fn fsCurrentTime(
    mut pVfs: *mut sqlite3_vfs,
    mut pTimeOut: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pParent: *mut sqlite3_vfs = (*(pVfs as *mut fs_vfs_t)).pParent;
        return (*pParent)
            .xCurrentTime
            .expect("non-null function pointer")(pParent, pTimeOut);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_register() -> ::core::ffi::c_int {
    unsafe {
        if !fs_vfs.pParent.is_null() {
            return SQLITE_OK;
        }
        fs_vfs.pParent = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        fs_vfs.base.mxPathname = (*fs_vfs.pParent).mxPathname;
        fs_vfs.base.szOsFile = (if ::core::mem::size_of::<tmp_file>() as usize
            > ::core::mem::size_of::<fs_file>() as usize
        {
            ::core::mem::size_of::<tmp_file>() as usize
        } else {
            ::core::mem::size_of::<fs_file>() as usize
        }) as ::core::ffi::c_int;
        return sqlite3_vfs_register(&raw mut fs_vfs.base, 0 as ::core::ffi::c_int);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SqlitetestOnefile_Init() -> ::core::ffi::c_int {
    unsafe {
        return fs_register();
    }
}
