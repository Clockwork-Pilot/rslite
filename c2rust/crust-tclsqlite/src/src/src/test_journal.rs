unsafe extern "C" {
    pub type sqlite3_mutex;
    pub type Bitvec;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
    fn sqlite3_mutex_alloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    fn sqlite3OsRandomness(
        _: *mut sqlite3_vfs,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsSleep(_: *mut sqlite3_vfs, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut sqlite3_io_error_pending: ::core::ffi::c_int;
    static mut sqlite3_io_error_hit: ::core::ffi::c_int;
    fn sqlite3BitvecCreate(_: u32_0) -> *mut Bitvec;
    fn sqlite3BitvecTest(_: *mut Bitvec, _: u32_0) -> ::core::ffi::c_int;
    fn sqlite3BitvecSet(_: *mut Bitvec, _: u32_0) -> ::core::ffi::c_int;
    fn sqlite3BitvecDestroy(_: *mut Bitvec);
    static mut sqlite3PendingByte: ::core::ffi::c_int;
}
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type u8_0 = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
pub type u32_0 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jt_file {
    pub base: sqlite3_file,
    pub zName: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int,
    pub eLock: ::core::ffi::c_int,
    pub nPage: u32_0,
    pub nPagesize: u32_0,
    pub pWritable: *mut Bitvec,
    pub aCksum: *mut u32_0,
    pub nSync: ::core::ffi::c_int,
    pub iMaxOff: sqlite3_int64,
    pub pNext: *mut jt_file,
    pub pReal: *mut sqlite3_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JtGlobal {
    pub pVfs: *mut sqlite3_vfs,
    pub pList: *mut jt_file,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_JOURNAL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_LOCK_RESERVED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_PRNG: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const JT_MAX_PATHNAME: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const JT_VFS_NAME: [::core::ffi::c_char; 3] = unsafe {
    ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"jt\0")
};
static mut jt_vfs: sqlite3_vfs = unsafe {
    sqlite3_vfs {
        iVersion: 2 as ::core::ffi::c_int,
        szOsFile: ::core::mem::size_of::<jt_file>() as ::core::ffi::c_int,
        mxPathname: JT_MAX_PATHNAME,
        pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
        zName: JT_VFS_NAME.as_ptr(),
        pAppData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        xOpen: Some(
            jtOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xDelete: Some(
            jtDelete
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xAccess: Some(
            jtAccess
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFullPathname: Some(
            jtFullPathname
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xDlOpen: Some(
            jtDlOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_void,
        ),
        xDlError: Some(
            jtDlError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> (),
        ),
        xDlSym: Some(
            jtDlSym
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> Option<unsafe extern "C" fn() -> ()>,
        ),
        xDlClose: Some(
            jtDlClose
                as unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
        ),
        xRandomness: Some(
            jtRandomness
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSleep: Some(
            jtSleep
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCurrentTime: Some(
            jtCurrentTime
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
        ),
        xGetLastError: Some(
            jtGetLastError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xCurrentTimeInt64: Some(
            jtCurrentTimeInt64
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSetSystemCall: None,
        xGetSystemCall: None,
        xNextSystemCall: None,
    }
};
static mut jt_io_methods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 1 as ::core::ffi::c_int,
        xClose: Some(
            jtClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            jtRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            jtWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            jtTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            jtSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            jtFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            jtLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            jtUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            jtCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            jtFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            jtSectorSize as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            jtDeviceCharacteristics
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
static mut g: JtGlobal = JtGlobal {
    pVfs: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
    pList: ::core::ptr::null::<jt_file>() as *mut jt_file,
};
unsafe extern "C" fn enterJtMutex() {
    unsafe {
        sqlite3_mutex_enter(sqlite3_mutex_alloc(SQLITE_MUTEX_STATIC_PRNG));
    }
}
unsafe extern "C" fn leaveJtMutex() {
    unsafe {
        sqlite3_mutex_leave(sqlite3_mutex_alloc(SQLITE_MUTEX_STATIC_PRNG));
    }
}
unsafe extern "C" fn stop_ioerr_simulation(
    mut piSave: *mut ::core::ffi::c_int,
    mut piSave2: *mut ::core::ffi::c_int,
) {
    unsafe {
        *piSave = sqlite3_io_error_pending;
        *piSave2 = sqlite3_io_error_hit;
        sqlite3_io_error_pending = -1 as ::core::ffi::c_int;
        sqlite3_io_error_hit = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn start_ioerr_simulation(
    mut iSave: ::core::ffi::c_int,
    mut iSave2: ::core::ffi::c_int,
) {
    unsafe {
        sqlite3_io_error_pending = iSave;
        sqlite3_io_error_hit = iSave2;
    }
}
unsafe extern "C" fn closeTransaction(mut p: *mut jt_file) {
    unsafe {
        sqlite3BitvecDestroy((*p).pWritable);
        sqlite3_free((*p).aCksum as *mut ::core::ffi::c_void);
        (*p).pWritable = ::core::ptr::null_mut::<Bitvec>();
        (*p).aCksum = ::core::ptr::null_mut::<u32_0>();
        (*p).nSync = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn jtClose(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut pp: *mut *mut jt_file = ::core::ptr::null_mut::<*mut jt_file>();
        let mut p: *mut jt_file = pFile as *mut jt_file;
        closeTransaction(p);
        enterJtMutex();
        if !(*p).zName.is_null() {
            pp = &raw mut g.pList;
            while *pp != p {
                pp = &raw mut (**pp).pNext;
            }
            *pp = (*p).pNext;
        }
        leaveJtMutex();
        sqlite3OsClose((*p).pReal);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn jtRead(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut jt_file = pFile as *mut jt_file;
        return sqlite3OsRead((*p).pReal, zBuf, iAmt, iOfst as i64_0);
    }
}
unsafe extern "C" fn locateDatabaseHandle(
    mut zJournal: *const ::core::ffi::c_char,
    mut noLock: ::core::ffi::c_int,
) -> *mut jt_file {
    unsafe {
        let mut pMain: *mut jt_file = ::core::ptr::null_mut::<jt_file>();
        enterJtMutex();
        pMain = g.pList;
        while !pMain.is_null() {
            let mut nName: ::core::ffi::c_int = strlen(zJournal)
                .wrapping_sub(
                    strlen(b"-journal\0".as_ptr() as *const ::core::ffi::c_char),
                ) as ::core::ffi::c_int;
            if (*pMain).flags & SQLITE_OPEN_MAIN_DB != 0
                && strlen((*pMain).zName) as ::core::ffi::c_int == nName
                && 0 as ::core::ffi::c_int
                    == memcmp(
                        (*pMain).zName as *const ::core::ffi::c_void,
                        zJournal as *const ::core::ffi::c_void,
                        nName as size_t,
                    ) && ((*pMain).eLock >= SQLITE_LOCK_RESERVED || noLock != 0)
            {
                break;
            }
            pMain = (*pMain).pNext;
        }
        leaveJtMutex();
        return pMain;
    }
}
unsafe extern "C" fn decodeUint32(mut z: *const ::core::ffi::c_uchar) -> u32_0 {
    unsafe {
        return (((*z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 24 as ::core::ffi::c_int)
            + ((*z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int)
            + ((*z.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int)
            + *z.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as u32_0;
    }
}
unsafe extern "C" fn genCksum(
    mut z: *const ::core::ffi::c_uchar,
    mut n: ::core::ffi::c_int,
) -> u32_0 {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut cksum: u32_0 = 0 as u32_0;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            cksum = cksum
                .wrapping_add(*z.offset(i as isize) as u32_0)
                .wrapping_add(cksum << 3 as ::core::ffi::c_int);
            i += 1;
        }
        return cksum;
    }
}
unsafe extern "C" fn decodeJournalHdr(
    mut zBuf: *const ::core::ffi::c_uchar,
    mut pnRec: *mut u32_0,
    mut pnPage: *mut u32_0,
    mut pnSector: *mut u32_0,
    mut pnPagesize: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aMagic: [::core::ffi::c_uchar; 8] = [
            0xd9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0x63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xd7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ];
        if memcmp(
            &raw mut aMagic as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            zBuf as *const ::core::ffi::c_void,
            8 as size_t,
        ) != 0
        {
            return SQLITE_ERROR;
        }
        if !pnRec.is_null() {
            *pnRec = decodeUint32(
                zBuf.offset(8 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_uchar,
            );
        }
        if !pnPage.is_null() {
            *pnPage = decodeUint32(
                zBuf.offset(16 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_uchar,
            );
        }
        if !pnSector.is_null() {
            *pnSector = decodeUint32(
                zBuf.offset(20 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_uchar,
            );
        }
        if !pnPagesize.is_null() {
            *pnPagesize = decodeUint32(
                zBuf.offset(24 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_uchar,
            );
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn openTransaction(
    mut pMain: *mut jt_file,
    mut pJournal: *mut jt_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aData: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut p: *mut sqlite3_file = (*pMain).pReal;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        closeTransaction(pMain);
        aData = sqlite3_malloc((*pMain).nPagesize as ::core::ffi::c_int)
            as *mut ::core::ffi::c_uchar;
        (*pMain).pWritable = sqlite3BitvecCreate((*pMain).nPage);
        (*pMain).aCksum = sqlite3_malloc(
            (::core::mem::size_of::<u32_0>() as usize)
                .wrapping_mul((*pMain).nPage.wrapping_add(1 as u32_0) as usize)
                as ::core::ffi::c_int,
        ) as *mut u32_0;
        (*pJournal).iMaxOff = 0 as sqlite3_int64;
        if (*pMain).pWritable.is_null() || (*pMain).aCksum.is_null() || aData.is_null() {
            rc = SQLITE_IOERR_NOMEM;
        } else if (*pMain).nPage > 0 as u32_0 {
            let mut iTrunk: u32_0 = 0;
            let mut iSave: ::core::ffi::c_int = 0;
            let mut iSave2: ::core::ffi::c_int = 0;
            stop_ioerr_simulation(&raw mut iSave, &raw mut iSave2);
            rc = sqlite3OsRead(
                p,
                aData as *mut ::core::ffi::c_void,
                (*pMain).nPagesize as ::core::ffi::c_int,
                0 as i64_0,
            );
            if rc == SQLITE_OK {
                let mut nDbsize: u32_0 = decodeUint32(
                    aData.offset(28 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_uchar,
                );
                if nDbsize > 0 as u32_0
                    && memcmp(
                        aData.offset(24 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                        aData.offset(92 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                        4 as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    let mut iPg: u32_0 = 0;
                    iPg = nDbsize.wrapping_add(1 as u32_0);
                    while iPg <= (*pMain).nPage {
                        sqlite3BitvecSet((*pMain).pWritable, iPg);
                        iPg = iPg.wrapping_add(1);
                    }
                }
            }
            iTrunk = decodeUint32(
                aData.offset(32 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uchar,
            );
            while rc == SQLITE_OK && iTrunk > 0 as u32_0 {
                let mut nLeaf: u32_0 = 0;
                let mut iLeaf: u32_0 = 0;
                let mut iOff: sqlite3_int64 = iTrunk.wrapping_sub(1 as u32_0)
                    as sqlite3_int64 * (*pMain).nPagesize as sqlite3_int64;
                rc = sqlite3OsRead(
                    p,
                    aData as *mut ::core::ffi::c_void,
                    (*pMain).nPagesize as ::core::ffi::c_int,
                    iOff as i64_0,
                );
                nLeaf = decodeUint32(
                    aData.offset(4 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_uchar,
                );
                iLeaf = 0 as u32_0;
                while rc == SQLITE_OK && iLeaf < nLeaf {
                    let mut pgno: u32_0 = decodeUint32(
                        aData
                            .offset(
                                (8 as u32_0).wrapping_add((4 as u32_0).wrapping_mul(iLeaf))
                                    as isize,
                            ) as *mut ::core::ffi::c_uchar,
                    );
                    sqlite3BitvecSet((*pMain).pWritable, pgno);
                    iLeaf = iLeaf.wrapping_add(1);
                }
                iTrunk = decodeUint32(aData);
            }
            if rc == SQLITE_OK {
                let mut ii: ::core::ffi::c_int = 0;
                ii = 0 as ::core::ffi::c_int;
                while rc == SQLITE_OK && ii < (*pMain).nPage as ::core::ffi::c_int {
                    let mut iOff_0: i64_0 = (*pMain).nPagesize as i64_0 * ii as i64_0;
                    if !(iOff_0 == sqlite3PendingByte as ::core::ffi::c_longlong) {
                        rc = sqlite3OsRead(
                            (*pMain).pReal,
                            aData as *mut ::core::ffi::c_void,
                            (*pMain).nPagesize as ::core::ffi::c_int,
                            iOff_0,
                        );
                        *(*pMain).aCksum.offset(ii as isize) = genCksum(
                            aData,
                            (*pMain).nPagesize as ::core::ffi::c_int,
                        );
                        if ii + 1 as ::core::ffi::c_int
                            == (*pMain).nPage as ::core::ffi::c_int
                            && rc == SQLITE_IOERR_SHORT_READ
                        {
                            rc = SQLITE_OK;
                        }
                    }
                    ii += 1;
                }
            }
            start_ioerr_simulation(iSave, iSave2);
        }
        sqlite3_free(aData as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn readJournalFile(
    mut p: *mut jt_file,
    mut pMain: *mut jt_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zBuf: [::core::ffi::c_uchar; 28] = [0; 28];
        let mut pReal: *mut sqlite3_file = (*p).pReal;
        let mut iOff: sqlite3_int64 = 0 as sqlite3_int64;
        let mut iSize: sqlite3_int64 = (*p).iMaxOff;
        let mut aPage: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut iSave: ::core::ffi::c_int = 0;
        let mut iSave2: ::core::ffi::c_int = 0;
        aPage = sqlite3_malloc((*pMain).nPagesize as ::core::ffi::c_int)
            as *mut ::core::ffi::c_uchar;
        if aPage.is_null() {
            return SQLITE_IOERR_NOMEM;
        }
        stop_ioerr_simulation(&raw mut iSave, &raw mut iSave2);
        while rc == SQLITE_OK && iOff < iSize {
            let mut nRec: u32_0 = 0;
            let mut nPage: u32_0 = 0;
            let mut nSector: u32_0 = 0;
            let mut nPagesize: u32_0 = 0;
            let mut ii: u32_0 = 0;
            rc = sqlite3OsRead(
                pReal,
                &raw mut zBuf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                28 as ::core::ffi::c_int,
                iOff as i64_0,
            );
            if rc != SQLITE_OK
                || decodeJournalHdr(
                    &raw mut zBuf as *mut ::core::ffi::c_uchar,
                    &raw mut nRec,
                    &raw mut nPage,
                    &raw mut nSector,
                    &raw mut nPagesize,
                ) != 0
            {
                break;
            }
            iOff += nSector as ::core::ffi::c_longlong;
            if nRec == 0 as u32_0 {
                if iSize
                    >= iOff as ::core::ffi::c_longlong
                        + nSector as ::core::ffi::c_longlong
                {
                    rc = sqlite3OsRead(
                        pReal,
                        &raw mut zBuf as *mut ::core::ffi::c_uchar
                            as *mut ::core::ffi::c_void,
                        28 as ::core::ffi::c_int,
                        iOff as i64_0,
                    );
                    if rc != SQLITE_OK
                        || 0 as ::core::ffi::c_int
                            == decodeJournalHdr(
                                &raw mut zBuf as *mut ::core::ffi::c_uchar,
                                ::core::ptr::null_mut::<u32_0>(),
                                ::core::ptr::null_mut::<u32_0>(),
                                ::core::ptr::null_mut::<u32_0>(),
                                ::core::ptr::null_mut::<u32_0>(),
                            )
                    {
                        continue;
                    }
                }
                nRec = ((iSize as ::core::ffi::c_longlong
                    - iOff as ::core::ffi::c_longlong)
                    / (*pMain).nPagesize.wrapping_add(8 as u32_0)
                        as ::core::ffi::c_longlong) as u32_0;
            }
            ii = 0 as u32_0;
            while rc == SQLITE_OK && ii < nRec && iOff < iSize {
                let mut pgno: u32_0 = 0;
                rc = sqlite3OsRead(
                    pReal,
                    &raw mut zBuf as *mut ::core::ffi::c_uchar
                        as *mut ::core::ffi::c_void,
                    4 as ::core::ffi::c_int,
                    iOff as i64_0,
                );
                if rc == SQLITE_OK {
                    pgno = decodeUint32(&raw mut zBuf as *mut ::core::ffi::c_uchar);
                    if pgno > 0 as u32_0 && pgno <= (*pMain).nPage {
                        if 0 as ::core::ffi::c_int
                            == sqlite3BitvecTest((*pMain).pWritable, pgno)
                        {
                            rc = sqlite3OsRead(
                                pReal,
                                aPage as *mut ::core::ffi::c_void,
                                (*pMain).nPagesize as ::core::ffi::c_int,
                                iOff as i64_0 + 4 as i64_0,
                            );
                            if rc == SQLITE_OK {
                                let mut cksum: u32_0 = genCksum(
                                    aPage,
                                    (*pMain).nPagesize as ::core::ffi::c_int,
                                );
                            }
                        }
                        sqlite3BitvecSet((*pMain).pWritable, pgno);
                    }
                    iOff
                        += (8 as u32_0).wrapping_add((*pMain).nPagesize)
                            as ::core::ffi::c_longlong;
                }
                ii = ii.wrapping_add(1);
            }
            iOff = ((iOff as ::core::ffi::c_longlong
                + nSector.wrapping_sub(1 as u32_0) as ::core::ffi::c_longlong)
                / nSector as ::core::ffi::c_longlong
                * nSector as ::core::ffi::c_longlong) as sqlite3_int64;
        }
        start_ioerr_simulation(iSave, iSave2);
        sqlite3_free(aPage as *mut ::core::ffi::c_void);
        if rc == SQLITE_IOERR_SHORT_READ {
            rc = SQLITE_OK;
        }
        return rc;
    }
}
unsafe extern "C" fn jtWrite(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut jt_file = pFile as *mut jt_file;
        if (*p).flags & SQLITE_OPEN_MAIN_JOURNAL != 0 {
            if iOfst == 0 as ::core::ffi::c_longlong {
                let mut pMain: *mut jt_file = locateDatabaseHandle(
                    (*p).zName,
                    0 as ::core::ffi::c_int,
                );
                if iAmt == 28 as ::core::ffi::c_int {
                    closeTransaction(pMain);
                } else if iAmt != 12 as ::core::ffi::c_int {
                    let mut z: *mut u8_0 = zBuf as *mut u8_0;
                    (*pMain).nPage = decodeUint32(
                        z.offset(16 as ::core::ffi::c_int as isize) as *mut u8_0,
                    );
                    (*pMain).nPagesize = decodeUint32(
                        z.offset(24 as ::core::ffi::c_int as isize) as *mut u8_0,
                    );
                    rc = openTransaction(pMain, p);
                    if SQLITE_OK != rc {
                        return rc;
                    }
                }
            }
            if (*p).iMaxOff
                < iOfst as ::core::ffi::c_longlong + iAmt as ::core::ffi::c_longlong
            {
                (*p).iMaxOff = (iOfst as ::core::ffi::c_longlong
                    + iAmt as ::core::ffi::c_longlong) as sqlite3_int64;
            }
        }
        if (*p).flags & SQLITE_OPEN_MAIN_DB != 0 && !(*p).pWritable.is_null() {
            if !(iAmt < (*p).nPagesize as ::core::ffi::c_int
                && (*p).nPagesize.wrapping_rem(iAmt as u32_0) == 0 as u32_0
                && iOfst
                    >= (sqlite3PendingByte + 512 as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong
                && iOfst as ::core::ffi::c_longlong + iAmt as ::core::ffi::c_longlong
                    <= (sqlite3PendingByte as u32_0).wrapping_add((*p).nPagesize)
                        as ::core::ffi::c_longlong)
            {
                let mut pgno: u32_0 = (iOfst as ::core::ffi::c_longlong
                    / (*p).nPagesize as ::core::ffi::c_longlong
                    + 1 as ::core::ffi::c_longlong) as u32_0;
            }
        }
        rc = sqlite3OsWrite((*p).pReal, zBuf, iAmt, iOfst as i64_0);
        if (*p).flags & SQLITE_OPEN_MAIN_JOURNAL != 0 && iAmt == 12 as ::core::ffi::c_int
        {
            let mut pMain_0: *mut jt_file = locateDatabaseHandle(
                (*p).zName,
                0 as ::core::ffi::c_int,
            );
            let mut rc2: ::core::ffi::c_int = readJournalFile(p, pMain_0);
            if rc == SQLITE_OK {
                rc = rc2;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn jtTruncate(
    mut pFile: *mut sqlite3_file,
    mut size: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut jt_file = pFile as *mut jt_file;
        if (*p).flags & SQLITE_OPEN_MAIN_JOURNAL != 0
            && size == 0 as ::core::ffi::c_longlong
        {
            let mut pMain: *mut jt_file = locateDatabaseHandle(
                (*p).zName,
                0 as ::core::ffi::c_int,
            );
            closeTransaction(pMain);
        }
        if (*p).flags & SQLITE_OPEN_MAIN_DB != 0 && !(*p).pWritable.is_null() {
            let mut pgno: u32_0 = 0;
            let mut locking_page: u32_0 = (sqlite3PendingByte as u32_0)
                .wrapping_div((*p).nPagesize)
                .wrapping_add(1 as u32_0);
            pgno = (size as ::core::ffi::c_longlong
                / (*p).nPagesize as ::core::ffi::c_longlong
                + 1 as ::core::ffi::c_longlong) as u32_0;
            while pgno <= (*p).nPage {
                pgno = pgno.wrapping_add(1);
            }
        }
        return sqlite3OsTruncate((*p).pReal, size as i64_0);
    }
}
unsafe extern "C" fn jtSync(
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut jt_file = pFile as *mut jt_file;
        if (*p).flags & SQLITE_OPEN_MAIN_JOURNAL != 0 {
            let mut rc: ::core::ffi::c_int = 0;
            let mut pMain: *mut jt_file = ::core::ptr::null_mut::<jt_file>();
            pMain = locateDatabaseHandle((*p).zName, 0 as ::core::ffi::c_int);
            if !pMain.is_null() && !(*pMain).pWritable.is_null() {
                (*pMain).nSync += 1;
                rc = readJournalFile(p, pMain);
                if rc != SQLITE_OK {
                    return rc;
                }
            }
        }
        return sqlite3OsSync((*p).pReal, flags);
    }
}
unsafe extern "C" fn jtFileSize(
    mut pFile: *mut sqlite3_file,
    mut pSize: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut jt_file = pFile as *mut jt_file;
        return sqlite3OsFileSize((*p).pReal, pSize as *mut i64_0);
    }
}
unsafe extern "C" fn jtLock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut jt_file = pFile as *mut jt_file;
        rc = sqlite3OsLock((*p).pReal, eLock);
        if rc == SQLITE_OK && eLock > (*p).eLock {
            (*p).eLock = eLock;
        }
        return rc;
    }
}
unsafe extern "C" fn jtUnlock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut jt_file = pFile as *mut jt_file;
        rc = sqlite3OsUnlock((*p).pReal, eLock);
        if rc == SQLITE_OK && eLock < (*p).eLock {
            (*p).eLock = eLock;
        }
        return rc;
    }
}
unsafe extern "C" fn jtCheckReservedLock(
    mut pFile: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut jt_file = pFile as *mut jt_file;
        return sqlite3OsCheckReservedLock((*p).pReal, pResOut);
    }
}
unsafe extern "C" fn jtFileControl(
    mut pFile: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut jt_file = pFile as *mut jt_file;
        return (*(*(*p).pReal).pMethods)
            .xFileControl
            .expect("non-null function pointer")((*p).pReal, op, pArg);
    }
}
unsafe extern "C" fn jtSectorSize(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut jt_file = pFile as *mut jt_file;
        return sqlite3OsSectorSize((*p).pReal);
    }
}
unsafe extern "C" fn jtDeviceCharacteristics(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut jt_file = pFile as *mut jt_file;
        return sqlite3OsDeviceCharacteristics((*p).pReal);
    }
}
unsafe extern "C" fn jtOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut jt_file = pFile as *mut jt_file;
        (*pFile).pMethods = ::core::ptr::null::<sqlite3_io_methods>();
        (*p).pReal = p.offset(1 as ::core::ffi::c_int as isize) as *mut jt_file
            as *mut sqlite3_file;
        (*(*p).pReal).pMethods = ::core::ptr::null::<sqlite3_io_methods>();
        rc = sqlite3OsOpen(g.pVfs, zName, (*p).pReal, flags, pOutFlags);
        if rc == SQLITE_OK {
            (*pFile).pMethods = &raw mut jt_io_methods;
            (*p).eLock = 0 as ::core::ffi::c_int;
            (*p).zName = zName;
            (*p).flags = flags;
            (*p).pNext = ::core::ptr::null_mut::<jt_file>();
            (*p).pWritable = ::core::ptr::null_mut::<Bitvec>();
            (*p).aCksum = ::core::ptr::null_mut::<u32_0>();
            enterJtMutex();
            if !zName.is_null() {
                (*p).pNext = g.pList;
                g.pList = p;
            }
            leaveJtMutex();
        }
        return rc;
    }
}
unsafe extern "C" fn jtDelete(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nPath: ::core::ffi::c_int = strlen(zPath) as ::core::ffi::c_int;
        if nPath > 8 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == strcmp(
                    b"-journal\0".as_ptr() as *const ::core::ffi::c_char,
                    zPath.offset((nPath - 8 as ::core::ffi::c_int) as isize)
                        as *const ::core::ffi::c_char,
                )
        {
            let mut pMain: *mut jt_file = locateDatabaseHandle(
                zPath,
                0 as ::core::ffi::c_int,
            );
            if !pMain.is_null() {
                closeTransaction(pMain);
            }
        }
        return sqlite3OsDelete(g.pVfs, zPath, dirSync);
    }
}
unsafe extern "C" fn jtAccess(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsAccess(g.pVfs, zPath, flags, pResOut);
    }
}
unsafe extern "C" fn jtFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsFullPathname(g.pVfs, zPath, nOut, zOut);
    }
}
unsafe extern "C" fn jtDlOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*g.pVfs).xDlOpen.expect("non-null function pointer")(g.pVfs, zPath);
    }
}
unsafe extern "C" fn jtDlError(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zErrMsg: *mut ::core::ffi::c_char,
) {
    unsafe {
        (*g.pVfs).xDlError.expect("non-null function pointer")(g.pVfs, nByte, zErrMsg);
    }
}
unsafe extern "C" fn jtDlSym(
    mut pVfs: *mut sqlite3_vfs,
    mut p: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        return (*g.pVfs).xDlSym.expect("non-null function pointer")(g.pVfs, p, zSym);
    }
}
unsafe extern "C" fn jtDlClose(
    mut pVfs: *mut sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*g.pVfs).xDlClose.expect("non-null function pointer")(g.pVfs, pHandle);
    }
}
unsafe extern "C" fn jtRandomness(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsRandomness(g.pVfs, nByte, zBufOut);
    }
}
unsafe extern "C" fn jtSleep(
    mut pVfs: *mut sqlite3_vfs,
    mut nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsSleep(g.pVfs, nMicro);
    }
}
unsafe extern "C" fn jtCurrentTime(
    mut pVfs: *mut sqlite3_vfs,
    mut pTimeOut: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        return (*g.pVfs)
            .xCurrentTime
            .expect("non-null function pointer")(g.pVfs, pTimeOut);
    }
}
unsafe extern "C" fn jtCurrentTimeInt64(
    mut pVfs: *mut sqlite3_vfs,
    mut pTimeOut: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        return (*g.pVfs)
            .xCurrentTimeInt64
            .expect("non-null function pointer")(g.pVfs, pTimeOut);
    }
}
unsafe extern "C" fn jtGetLastError(
    mut pVfs: *mut sqlite3_vfs,
    mut n: ::core::ffi::c_int,
    mut z: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*g.pVfs).xGetLastError.expect("non-null function pointer")(g.pVfs, n, z);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jt_register(
    mut zWrap: *mut ::core::ffi::c_char,
    mut isDefault: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        g.pVfs = sqlite3_vfs_find(zWrap);
        if g.pVfs.is_null() {
            return SQLITE_ERROR;
        }
        jt_vfs.szOsFile = (::core::mem::size_of::<jt_file>() as usize)
            .wrapping_add((*g.pVfs).szOsFile as usize) as ::core::ffi::c_int;
        if (*g.pVfs).iVersion == 1 as ::core::ffi::c_int {
            jt_vfs.iVersion = 1 as ::core::ffi::c_int;
        } else if (*g.pVfs).xCurrentTimeInt64.is_none() {
            jt_vfs.xCurrentTimeInt64 = None;
        }
        sqlite3_vfs_register(&raw mut jt_vfs, isDefault);
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jt_unregister() {
    unsafe {
        sqlite3_vfs_unregister(&raw mut jt_vfs);
    }
}
