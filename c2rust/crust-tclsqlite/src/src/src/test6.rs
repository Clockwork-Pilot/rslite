unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Tcl_Command_;
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
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
    fn sqlite3OsOpen(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_Free(ptr: *mut ::core::ffi::c_char);
    fn Tcl_DuplicateObj(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_GetBooleanFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetStringFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_ListObjGetElements(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objcPtr: *mut ::core::ffi::c_int,
        objvPtr: *mut *mut *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_GetIndexFromObjStruct(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        tablePtr: *const ::core::ffi::c_void,
        offset: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        indexPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_UtfToLower(src: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn Tcl_AttemptAlloc(size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char;
    fn Tcl_AttemptRealloc(
        ptr: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_char;
}
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type u8_0 = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
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
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ClientData = *mut ::core::ffi::c_void;
pub type Tcl_WideInt = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut ::core::ffi::c_char,
    pub freeProcDontUse: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ()>,
    pub errorLineDontUse: ::core::ffi::c_int,
}
pub type Tcl_Command = *mut Tcl_Command_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Obj {
    pub refCount: ::core::ffi::c_int,
    pub bytes: *mut ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub typePtr: *const Tcl_ObjType,
    pub internalRep: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2Rust_Unnamed_1,
    pub ptrAndLongRep: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub ptr1: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_ObjType {
    pub name: *const ::core::ffi::c_char,
    pub freeIntRepProc: Option<Tcl_FreeInternalRepProc>,
    pub dupIntRepProc: Option<Tcl_DupInternalRepProc>,
    pub updateStringProc: Option<Tcl_UpdateStringProc>,
    pub setFromAnyProc: Option<Tcl_SetFromAnyProc>,
}
pub type Tcl_SetFromAnyProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
) -> ::core::ffi::c_int;
pub type Tcl_UpdateStringProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_DupInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> ();
pub type Tcl_FreeInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_CmdDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrashFile {
    pub pMethod: *const sqlite3_io_methods,
    pub pRealFile: *mut sqlite3_file,
    pub zName: *mut ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int,
    pub zData: *mut u8_0,
    pub nData: ::core::ffi::c_int,
    pub iSize: i64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrashGlobal {
    pub pWriteList: *mut WriteBuffer,
    pub pWriteListEnd: *mut WriteBuffer,
    pub iSectorSize: ::core::ffi::c_int,
    pub iDeviceCharacteristics: ::core::ffi::c_int,
    pub iCrash: ::core::ffi::c_int,
    pub zCrashFile: [::core::ffi::c_char; 500],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WriteBuffer {
    pub iOffset: i64_0,
    pub nBuf: ::core::ffi::c_int,
    pub zBuf: *mut u8_0,
    pub pFile: *mut CrashFile,
    pub pNext: *mut WriteBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceFlag {
    pub zName: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC512: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC1K: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC2K: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC4K: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC8K: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC16K: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC32K: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC64K: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SAFE_APPEND: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SEQUENTIAL: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 0x1000
    as ::core::ffi::c_int;
pub const SQLITE_IOCAP_BATCH_ATOMIC: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_SIZE_HINT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const SQLITE_DEFAULT_SECTOR_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut g: CrashGlobal = CrashGlobal {
    pWriteList: ::core::ptr::null::<WriteBuffer>() as *mut WriteBuffer,
    pWriteListEnd: ::core::ptr::null::<WriteBuffer>() as *mut WriteBuffer,
    iSectorSize: SQLITE_DEFAULT_SECTOR_SIZE,
    iDeviceCharacteristics: 0 as ::core::ffi::c_int,
    iCrash: 0 as ::core::ffi::c_int,
    zCrashFile: [0; 500],
};
static mut sqlite3CrashTestEnable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn crash_malloc(
    mut nByte: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return Tcl_AttemptAlloc(nByte as size_t as ::core::ffi::c_uint)
            as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn crash_free(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        Tcl_Free(p as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn crash_realloc(
    mut p: *mut ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return Tcl_AttemptRealloc(
            p as *mut ::core::ffi::c_char,
            n as size_t as ::core::ffi::c_uint,
        ) as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn writeDbFile(
    mut p: *mut CrashFile,
    mut z: *mut u8_0,
    mut iAmt: i64_0,
    mut iOff: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut iSkip: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if iAmt as ::core::ffi::c_longlong - iSkip as ::core::ffi::c_longlong
            > 0 as ::core::ffi::c_longlong
        {
            rc = sqlite3OsWrite(
                (*p).pRealFile,
                z.offset(iSkip as isize) as *mut u8_0 as *const ::core::ffi::c_void,
                (iAmt as ::core::ffi::c_longlong - iSkip as ::core::ffi::c_longlong)
                    as ::core::ffi::c_int,
                iOff + iSkip as i64_0,
            );
        }
        return rc;
    }
}
unsafe extern "C" fn writeListSync(
    mut pFile: *mut CrashFile,
    mut isCrash: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut iDc: ::core::ffi::c_int = g.iDeviceCharacteristics;
        let mut pWrite: *mut WriteBuffer = ::core::ptr::null_mut::<WriteBuffer>();
        let mut ppPtr: *mut *mut WriteBuffer = ::core::ptr::null_mut::<
            *mut WriteBuffer,
        >();
        let mut pFinal: *mut WriteBuffer = ::core::ptr::null_mut::<WriteBuffer>();
        if isCrash == 0 {
            pWrite = g.pWriteList;
            while !pWrite.is_null() {
                if (*pWrite).pFile == pFile {
                    pFinal = pWrite;
                }
                pWrite = (*pWrite).pNext;
            }
        } else if iDc & (SQLITE_IOCAP_SEQUENTIAL | SQLITE_IOCAP_SAFE_APPEND) != 0 {
            let mut nWrite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iFinal: ::core::ffi::c_int = 0;
            pWrite = g.pWriteList;
            while !pWrite.is_null() {
                nWrite += 1;
                pWrite = (*pWrite).pNext;
            }
            sqlite3_randomness(
                ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
                &raw mut iFinal as *mut ::core::ffi::c_void,
            );
            iFinal = (if iFinal < 0 as ::core::ffi::c_int {
                -1 as ::core::ffi::c_int * iFinal
            } else {
                iFinal
            }) % nWrite;
            pWrite = g.pWriteList;
            while iFinal > 0 as ::core::ffi::c_int {
                iFinal -= 1;
                pWrite = (*pWrite).pNext;
            }
            pFinal = pWrite;
        }
        ppPtr = &raw mut g.pWriteList;
        pWrite = *ppPtr;
        while rc == SQLITE_OK && !pWrite.is_null() {
            let mut pRealFile: *mut sqlite3_file = (*(*pWrite).pFile).pRealFile;
            let mut eAction: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if isCrash == 0 {
                eAction = 2 as ::core::ffi::c_int;
                if (*pWrite).pFile == pFile || iDc & SQLITE_IOCAP_SEQUENTIAL != 0 {
                    eAction = 1 as ::core::ffi::c_int;
                }
            } else {
                let mut random: ::core::ffi::c_char = 0;
                sqlite3_randomness(
                    1 as ::core::ffi::c_int,
                    &raw mut random as *mut ::core::ffi::c_void,
                );
                if iDc & SQLITE_IOCAP_ATOMIC != 0 || (*pWrite).zBuf.is_null() {
                    random = (random as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                }
                if iDc & SQLITE_IOCAP_SEQUENTIAL != 0 && pWrite != pFinal {
                    random = 0 as ::core::ffi::c_char;
                }
                if iDc & SQLITE_IOCAP_SAFE_APPEND != 0 && !(*pWrite).zBuf.is_null() {
                    let mut iSize: i64_0 = 0;
                    sqlite3OsFileSize(pRealFile, &raw mut iSize);
                    if iSize == (*pWrite).iOffset {
                        random = 0 as ::core::ffi::c_char;
                    }
                }
                if random as ::core::ffi::c_int & 0x6 as ::core::ffi::c_int
                    == 0x6 as ::core::ffi::c_int
                {
                    eAction = 3 as ::core::ffi::c_int;
                } else {
                    eAction = if random as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
                        != 0
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    };
                }
            }
            match eAction {
                1 => {
                    if !(*pWrite).zBuf.is_null() {
                        rc = writeDbFile(
                            (*pWrite).pFile,
                            (*pWrite).zBuf,
                            (*pWrite).nBuf as i64_0,
                            (*pWrite).iOffset,
                        );
                    } else {
                        rc = sqlite3OsTruncate(pRealFile, (*pWrite).iOffset);
                    }
                    *ppPtr = (*pWrite).pNext;
                    crash_free(pWrite as *mut ::core::ffi::c_void);
                }
                2 => {
                    ppPtr = &raw mut (*pWrite).pNext;
                }
                3 => {
                    let mut zGarbage: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                    let mut iFirst: ::core::ffi::c_int = ((*pWrite).iOffset
                        as ::core::ffi::c_longlong
                        / g.iSectorSize as ::core::ffi::c_longlong)
                        as ::core::ffi::c_int;
                    let mut iLast: ::core::ffi::c_int = (((*pWrite).iOffset
                        as ::core::ffi::c_longlong
                        + (*pWrite).nBuf as ::core::ffi::c_longlong
                        - 1 as ::core::ffi::c_longlong)
                        / g.iSectorSize as ::core::ffi::c_longlong)
                        as ::core::ffi::c_int;
                    zGarbage = crash_malloc(g.iSectorSize) as *mut u8_0;
                    if !zGarbage.is_null() {
                        let mut i: sqlite3_int64 = 0;
                        i = iFirst as sqlite3_int64;
                        while rc == SQLITE_OK && i <= iLast as ::core::ffi::c_longlong {
                            sqlite3_randomness(
                                g.iSectorSize,
                                zGarbage as *mut ::core::ffi::c_void,
                            );
                            rc = writeDbFile(
                                (*pWrite).pFile,
                                zGarbage,
                                g.iSectorSize as i64_0,
                                i as i64_0 * g.iSectorSize as i64_0,
                            );
                            i += 1;
                        }
                        crash_free(zGarbage as *mut ::core::ffi::c_void);
                    } else {
                        rc = SQLITE_NOMEM;
                    }
                    ppPtr = &raw mut (*pWrite).pNext;
                }
                _ => {}
            }
            if pWrite == pFinal {
                break;
            }
            pWrite = *ppPtr;
        }
        if rc == SQLITE_OK && isCrash != 0 {
            exit(-1 as ::core::ffi::c_int);
        }
        pWrite = g.pWriteList;
        while !pWrite.is_null() && !(*pWrite).pNext.is_null() {
            pWrite = (*pWrite).pNext;
        }
        g.pWriteListEnd = pWrite;
        return rc;
    }
}
unsafe extern "C" fn writeListAppend(
    mut pFile: *mut sqlite3_file,
    mut iOffset: sqlite3_int64,
    mut zBuf: *const u8_0,
    mut nBuf: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNew: *mut WriteBuffer = ::core::ptr::null_mut::<WriteBuffer>();
        pNew = crash_malloc(
            (::core::mem::size_of::<WriteBuffer>() as usize).wrapping_add(nBuf as usize)
                as ::core::ffi::c_int,
        ) as *mut WriteBuffer;
        if pNew.is_null() {
            fprintf(
                stderr,
                b"out of memory in the crash simulator\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<WriteBuffer>() as size_t)
                .wrapping_add(nBuf as size_t),
        );
        (*pNew).iOffset = iOffset as i64_0;
        (*pNew).nBuf = nBuf;
        (*pNew).pFile = pFile as *mut CrashFile;
        if !zBuf.is_null() {
            (*pNew).zBuf = pNew.offset(1 as ::core::ffi::c_int as isize)
                as *mut WriteBuffer as *mut u8_0;
            memcpy(
                (*pNew).zBuf as *mut ::core::ffi::c_void,
                zBuf as *const ::core::ffi::c_void,
                nBuf as size_t,
            );
        }
        if !g.pWriteList.is_null() {
            (*g.pWriteListEnd).pNext = pNew;
        } else {
            g.pWriteList = pNew;
        }
        g.pWriteListEnd = pNew;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn cfClose(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut pCrash: *mut CrashFile = pFile as *mut CrashFile;
        writeListSync(pCrash, 0 as ::core::ffi::c_int);
        sqlite3OsClose((*pCrash).pRealFile);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn cfRead(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCrash: *mut CrashFile = pFile as *mut CrashFile;
        let mut nCopy: ::core::ffi::c_int = (if (iAmt as i64_0)
            < (*pCrash).iSize as sqlite_int64 - iOfst
        {
            iAmt as ::core::ffi::c_longlong
        } else {
            (*pCrash).iSize as ::core::ffi::c_longlong - iOfst as ::core::ffi::c_longlong
        }) as ::core::ffi::c_int;
        if nCopy > 0 as ::core::ffi::c_int {
            memcpy(
                zBuf,
                (*pCrash).zData.offset(iOfst as isize) as *mut u8_0
                    as *const ::core::ffi::c_void,
                nCopy as size_t,
            );
        }
        if nCopy < iAmt {
            return SQLITE_IOERR_SHORT_READ;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn cfWrite(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCrash: *mut CrashFile = pFile as *mut CrashFile;
        if iAmt as sqlite_int64 + iOfst > (*pCrash).iSize {
            (*pCrash).iSize = (iAmt as sqlite_int64 + iOfst) as ::core::ffi::c_int
                as i64_0;
        }
        while (*pCrash).iSize > (*pCrash).nData as ::core::ffi::c_longlong {
            let mut zNew: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
            let mut nNew: ::core::ffi::c_int = (*pCrash).nData * 2 as ::core::ffi::c_int
                + 4096 as ::core::ffi::c_int;
            zNew = crash_realloc((*pCrash).zData as *mut ::core::ffi::c_void, nNew)
                as *mut u8_0;
            if zNew.is_null() {
                return SQLITE_NOMEM;
            }
            memset(
                zNew.offset((*pCrash).nData as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (nNew - (*pCrash).nData) as size_t,
            );
            (*pCrash).nData = nNew;
            (*pCrash).zData = zNew;
        }
        memcpy(
            (*pCrash).zData.offset(iOfst as isize) as *mut u8_0
                as *mut ::core::ffi::c_void,
            zBuf,
            iAmt as size_t,
        );
        return writeListAppend(pFile, iOfst as sqlite3_int64, zBuf as *const u8_0, iAmt);
    }
}
unsafe extern "C" fn cfTruncate(
    mut pFile: *mut sqlite3_file,
    mut size: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCrash: *mut CrashFile = pFile as *mut CrashFile;
        if (*pCrash).iSize > size {
            (*pCrash).iSize = size as ::core::ffi::c_int as i64_0;
        }
        return writeListAppend(
            pFile,
            size as sqlite3_int64,
            ::core::ptr::null::<u8_0>(),
            0 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn cfSync(
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCrash: *mut CrashFile = pFile as *mut CrashFile;
        let mut isCrash: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zName: *const ::core::ffi::c_char = (*pCrash).zName;
        let mut zCrashFile: *const ::core::ffi::c_char = &raw mut g.zCrashFile
            as *mut ::core::ffi::c_char;
        let mut nName: ::core::ffi::c_int = strlen(zName) as ::core::ffi::c_int;
        let mut nCrashFile: ::core::ffi::c_int = strlen(zCrashFile)
            as ::core::ffi::c_int;
        if nCrashFile > 0 as ::core::ffi::c_int
            && *zCrashFile.offset((nCrashFile - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == '*' as i32
        {
            nCrashFile -= 1;
            if nName > nCrashFile {
                nName = nCrashFile;
            }
        }
        if nName == nCrashFile
            && 0 as ::core::ffi::c_int
                == memcmp(
                    zName as *const ::core::ffi::c_void,
                    zCrashFile as *const ::core::ffi::c_void,
                    nName as size_t,
                )
        {
            g.iCrash -= 1;
            if g.iCrash == 0 as ::core::ffi::c_int {
                isCrash = 1 as ::core::ffi::c_int;
            }
        }
        return writeListSync(pCrash, isCrash);
    }
}
unsafe extern "C" fn cfFileSize(
    mut pFile: *mut sqlite3_file,
    mut pSize: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCrash: *mut CrashFile = pFile as *mut CrashFile;
        *pSize = (*pCrash).iSize as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn cfLock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsLock((*(pFile as *mut CrashFile)).pRealFile, eLock);
    }
}
unsafe extern "C" fn cfUnlock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsUnlock((*(pFile as *mut CrashFile)).pRealFile, eLock);
    }
}
unsafe extern "C" fn cfCheckReservedLock(
    mut pFile: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsCheckReservedLock(
            (*(pFile as *mut CrashFile)).pRealFile,
            pResOut,
        );
    }
}
unsafe extern "C" fn cfFileControl(
    mut pFile: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        if op == SQLITE_FCNTL_SIZE_HINT {
            let mut pCrash: *mut CrashFile = pFile as *mut CrashFile;
            let mut nByte: i64_0 = *(pArg as *mut i64_0);
            if nByte > (*pCrash).iSize {
                if SQLITE_OK
                    == writeListAppend(
                        pFile,
                        nByte as sqlite3_int64,
                        ::core::ptr::null::<u8_0>(),
                        0 as ::core::ffi::c_int,
                    )
                {
                    (*pCrash).iSize = nByte as ::core::ffi::c_int as i64_0;
                }
            }
            return SQLITE_OK;
        }
        return sqlite3OsFileControl((*(pFile as *mut CrashFile)).pRealFile, op, pArg);
    }
}
unsafe extern "C" fn cfSectorSize(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        return g.iSectorSize;
    }
}
unsafe extern "C" fn cfDeviceCharacteristics(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        return g.iDeviceCharacteristics;
    }
}
unsafe extern "C" fn cfShmLock(
    mut pFile: *mut sqlite3_file,
    mut ofst: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pReal: *mut sqlite3_file = (*(pFile as *mut CrashFile)).pRealFile;
        return (*(*pReal).pMethods)
            .xShmLock
            .expect("non-null function pointer")(pReal, ofst, n, flags);
    }
}
unsafe extern "C" fn cfShmBarrier(mut pFile: *mut sqlite3_file) {
    unsafe {
        let mut pReal: *mut sqlite3_file = (*(pFile as *mut CrashFile)).pRealFile;
        (*(*pReal).pMethods).xShmBarrier.expect("non-null function pointer")(pReal);
    }
}
unsafe extern "C" fn cfShmUnmap(
    mut pFile: *mut sqlite3_file,
    mut delFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pReal: *mut sqlite3_file = (*(pFile as *mut CrashFile)).pRealFile;
        return (*(*pReal).pMethods)
            .xShmUnmap
            .expect("non-null function pointer")(pReal, delFlag);
    }
}
unsafe extern "C" fn cfShmMap(
    mut pFile: *mut sqlite3_file,
    mut iRegion: ::core::ffi::c_int,
    mut sz: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pReal: *mut sqlite3_file = (*(pFile as *mut CrashFile)).pRealFile;
        return (*(*pReal).pMethods)
            .xShmMap
            .expect("non-null function pointer")(pReal, iRegion, sz, w, pp);
    }
}
static mut CrashFileVtab: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 2 as ::core::ffi::c_int,
        xClose: Some(
            cfClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            cfRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            cfWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            cfTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            cfSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            cfFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            cfLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            cfUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            cfCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            cfFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            cfSectorSize as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            cfDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: Some(
            cfShmMap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xShmLock: Some(
            cfShmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShmBarrier: Some(cfShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> ()),
        xShmUnmap: Some(
            cfShmUnmap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFetch: None,
        xUnfetch: None,
    }
};
unsafe extern "C" fn cfOpen(
    mut pCfVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pWrapper: *mut CrashFile = pFile as *mut CrashFile;
        let mut pReal: *mut sqlite3_file = pWrapper
            .offset(1 as ::core::ffi::c_int as isize) as *mut CrashFile
            as *mut sqlite3_file;
        memset(
            pWrapper as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<CrashFile>() as size_t,
        );
        rc = sqlite3OsOpen(pVfs, zName, pReal, flags, pOutFlags);
        if rc == SQLITE_OK {
            let mut iSize: i64_0 = 0;
            (*pWrapper).pMethod = &raw const CrashFileVtab;
            (*pWrapper).zName = zName as *mut ::core::ffi::c_char;
            (*pWrapper).pRealFile = pReal;
            rc = sqlite3OsFileSize(pReal, &raw mut iSize);
            (*pWrapper).iSize = iSize as ::core::ffi::c_int as i64_0;
            (*pWrapper).flags = flags;
        }
        if rc == SQLITE_OK {
            (*pWrapper).nData = (4096 as i64_0 + (*pWrapper).iSize)
                as ::core::ffi::c_int;
            (*pWrapper).zData = crash_malloc((*pWrapper).nData) as *mut u8_0;
            if !(*pWrapper).zData.is_null() {
                let mut iOff: i64_0 = 0;
                memset(
                    (*pWrapper).zData as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (*pWrapper).nData as size_t,
                );
                iOff = 0 as i64_0;
                while iOff < (*pWrapper).iSize {
                    let mut nRead: ::core::ffi::c_int = ((*pWrapper).iSize - iOff)
                        as ::core::ffi::c_int;
                    if nRead > 512 as ::core::ffi::c_int {
                        nRead = 512 as ::core::ffi::c_int;
                    }
                    rc = sqlite3OsRead(
                        pReal,
                        (*pWrapper).zData.offset(iOff as isize) as *mut u8_0
                            as *mut ::core::ffi::c_void,
                        nRead,
                        iOff,
                    );
                    iOff += 512 as ::core::ffi::c_longlong;
                }
            } else {
                rc = SQLITE_NOMEM;
            }
        }
        if rc != SQLITE_OK && !(*pWrapper).pMethod.is_null() {
            sqlite3OsClose(pFile);
        }
        return rc;
    }
}
unsafe extern "C" fn cfDelete(
    mut pCfVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        return (*pVfs).xDelete.expect("non-null function pointer")(pVfs, zPath, dirSync);
    }
}
unsafe extern "C" fn cfAccess(
    mut pCfVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        return (*pVfs)
            .xAccess
            .expect("non-null function pointer")(pVfs, zPath, flags, pResOut);
    }
}
unsafe extern "C" fn cfFullPathname(
    mut pCfVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nPathOut: ::core::ffi::c_int,
    mut zPathOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        return (*pVfs)
            .xFullPathname
            .expect("non-null function pointer")(pVfs, zPath, nPathOut, zPathOut);
    }
}
unsafe extern "C" fn cfDlOpen(
    mut pCfVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        return (*pVfs).xDlOpen.expect("non-null function pointer")(pVfs, zPath);
    }
}
unsafe extern "C" fn cfDlError(
    mut pCfVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zErrMsg: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        (*pVfs).xDlError.expect("non-null function pointer")(pVfs, nByte, zErrMsg);
    }
}
unsafe extern "C" fn cfDlSym(
    mut pCfVfs: *mut sqlite3_vfs,
    mut pH: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        return (*pVfs).xDlSym.expect("non-null function pointer")(pVfs, pH, zSym);
    }
}
unsafe extern "C" fn cfDlClose(
    mut pCfVfs: *mut sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        (*pVfs).xDlClose.expect("non-null function pointer")(pVfs, pHandle);
    }
}
unsafe extern "C" fn cfRandomness(
    mut pCfVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        return (*pVfs)
            .xRandomness
            .expect("non-null function pointer")(pVfs, nByte, zBufOut);
    }
}
unsafe extern "C" fn cfSleep(
    mut pCfVfs: *mut sqlite3_vfs,
    mut nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        return (*pVfs).xSleep.expect("non-null function pointer")(pVfs, nMicro);
    }
}
unsafe extern "C" fn cfCurrentTime(
    mut pCfVfs: *mut sqlite3_vfs,
    mut pTimeOut: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        return (*pVfs).xCurrentTime.expect("non-null function pointer")(pVfs, pTimeOut);
    }
}
unsafe extern "C" fn cfGetLastError(
    mut pCfVfs: *mut sqlite3_vfs,
    mut n: ::core::ffi::c_int,
    mut z: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = (*pCfVfs).pAppData as *mut sqlite3_vfs;
        return (*pVfs).xGetLastError.expect("non-null function pointer")(pVfs, n, z);
    }
}
unsafe extern "C" fn processDevSymArgs(
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
    mut piDeviceChar: *mut ::core::ffi::c_int,
    mut piSectorSize: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aFlag: [DeviceFlag; 14] = [
            DeviceFlag {
                zName: b"atomic\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_ATOMIC,
            },
            DeviceFlag {
                zName: b"atomic512\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_ATOMIC512,
            },
            DeviceFlag {
                zName: b"atomic1k\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_ATOMIC1K,
            },
            DeviceFlag {
                zName: b"atomic2k\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_ATOMIC2K,
            },
            DeviceFlag {
                zName: b"atomic4k\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_ATOMIC4K,
            },
            DeviceFlag {
                zName: b"atomic8k\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_ATOMIC8K,
            },
            DeviceFlag {
                zName: b"atomic16k\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_ATOMIC16K,
            },
            DeviceFlag {
                zName: b"atomic32k\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_ATOMIC32K,
            },
            DeviceFlag {
                zName: b"atomic64k\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_ATOMIC64K,
            },
            DeviceFlag {
                zName: b"sequential\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_SEQUENTIAL,
            },
            DeviceFlag {
                zName: b"safe_append\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_SAFE_APPEND,
            },
            DeviceFlag {
                zName: b"powersafe_overwrite\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_POWERSAFE_OVERWRITE,
            },
            DeviceFlag {
                zName: b"batch-atomic\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iValue: SQLITE_IOCAP_BATCH_ATOMIC,
            },
            DeviceFlag {
                zName: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                iValue: 0 as ::core::ffi::c_int,
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let mut iDc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iSectorSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut setSectorsize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut setDeviceChar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < objc {
            let mut nOpt: ::core::ffi::c_int = 0;
            let mut zOpt: *mut ::core::ffi::c_char = Tcl_GetStringFromObj(
                *objv.offset(i as isize),
                &raw mut nOpt,
            );
            if (nOpt > 11 as ::core::ffi::c_int || nOpt < 2 as ::core::ffi::c_int
                || strncmp(
                    b"-sectorsize\0".as_ptr() as *const ::core::ffi::c_char,
                    zOpt,
                    nOpt as size_t,
                ) != 0)
                && (nOpt > 16 as ::core::ffi::c_int || nOpt < 2 as ::core::ffi::c_int
                    || strncmp(
                        b"-characteristics\0".as_ptr() as *const ::core::ffi::c_char,
                        zOpt,
                        nOpt as size_t,
                    ) != 0)
            {
                Tcl_AppendResult(
                    interp,
                    b"Bad option: \"\0".as_ptr() as *const ::core::ffi::c_char,
                    zOpt,
                    b"\" - must be \"-characteristics\" or \"-sectorsize\"\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    NULL,
                );
                return TCL_ERROR;
            }
            if i == objc - 1 as ::core::ffi::c_int {
                Tcl_AppendResult(
                    interp,
                    b"Option requires an argument: \"\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zOpt,
                    b"\"\0".as_ptr() as *const ::core::ffi::c_char,
                    NULL,
                );
                return TCL_ERROR;
            }
            if *zOpt.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 's' as i32
            {
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset((i + 1 as ::core::ffi::c_int) as isize),
                    &raw mut iSectorSize,
                ) != 0
                {
                    return TCL_ERROR;
                }
                setSectorsize = 1 as ::core::ffi::c_int;
            } else {
                let mut j: ::core::ffi::c_int = 0;
                let mut apObj: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<
                    *mut Tcl_Obj,
                >();
                let mut nObj: ::core::ffi::c_int = 0;
                if Tcl_ListObjGetElements(
                    interp,
                    *objv.offset((i + 1 as ::core::ffi::c_int) as isize),
                    &raw mut nObj,
                    &raw mut apObj,
                ) != 0
                {
                    return TCL_ERROR;
                }
                j = 0 as ::core::ffi::c_int;
                while j < nObj {
                    let mut rc: ::core::ffi::c_int = 0;
                    let mut iChoice: ::core::ffi::c_int = 0;
                    let mut pFlag: *mut Tcl_Obj = Tcl_DuplicateObj(
                        *apObj.offset(j as isize),
                    );
                    (*pFlag).refCount += 1;
                    Tcl_UtfToLower(Tcl_GetString(pFlag));
                    rc = Tcl_GetIndexFromObjStruct(
                        interp,
                        pFlag,
                        &raw mut aFlag as *mut DeviceFlag as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<DeviceFlag>() as ::core::ffi::c_int,
                        b"no such flag\0".as_ptr() as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                        &raw mut iChoice,
                    );
                    let mut _objPtr: *mut Tcl_Obj = pFlag;
                    let c2rust_fresh0 = (*_objPtr).refCount;
                    (*_objPtr).refCount = (*_objPtr).refCount - 1;
                    if c2rust_fresh0 <= 1 as ::core::ffi::c_int {
                        TclFreeObj(_objPtr);
                    }
                    if rc != 0 {
                        return TCL_ERROR;
                    }
                    iDc |= aFlag[iChoice as usize].iValue;
                    j += 1;
                }
                setDeviceChar = 1 as ::core::ffi::c_int;
            }
            i += 2 as ::core::ffi::c_int;
        }
        if setDeviceChar != 0 {
            *piDeviceChar = iDc;
        }
        if setSectorsize != 0 {
            *piSectorSize = iSectorSize;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn crashNowCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        writeListSync(::core::ptr::null_mut::<CrashFile>(), 1 as ::core::ffi::c_int);
        return TCL_OK;
    }
}
unsafe extern "C" fn crashEnableCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut isEnable: ::core::ffi::c_int = 0;
        let mut isDefault: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        static mut crashVfs: sqlite3_vfs = unsafe {
            sqlite3_vfs {
                iVersion: 2 as ::core::ffi::c_int,
                szOsFile: 0 as ::core::ffi::c_int,
                mxPathname: 0 as ::core::ffi::c_int,
                pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
                zName: b"crash\0".as_ptr() as *const ::core::ffi::c_char,
                pAppData: ::core::ptr::null::<::core::ffi::c_void>()
                    as *mut ::core::ffi::c_void,
                xOpen: Some(
                    cfOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xDelete: Some(
                    cfDelete
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xAccess: Some(
                    cfAccess
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xFullPathname: Some(
                    cfFullPathname
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                xDlOpen: Some(
                    cfDlOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        ) -> *mut ::core::ffi::c_void,
                ),
                xDlError: Some(
                    cfDlError
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> (),
                ),
                xDlSym: Some(
                    cfDlSym
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                        ) -> Option<unsafe extern "C" fn() -> ()>,
                ),
                xDlClose: Some(
                    cfDlClose
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                xRandomness: Some(
                    cfRandomness
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                xSleep: Some(
                    cfSleep
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xCurrentTime: Some(
                    cfCurrentTime
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *mut ::core::ffi::c_double,
                        ) -> ::core::ffi::c_int,
                ),
                xGetLastError: Some(
                    cfGetLastError
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                xCurrentTimeInt64: None,
                xSetSystemCall: None,
                xGetSystemCall: None,
                xNextSystemCall: None,
            }
        };
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"ENABLE ?DEFAULT?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetBooleanFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut isEnable,
        ) != 0
        {
            return TCL_ERROR;
        }
        if objc == 3 as ::core::ffi::c_int
            && Tcl_GetBooleanFromObj(
                interp,
                *objv.offset(2 as ::core::ffi::c_int as isize),
                &raw mut isDefault,
            ) != 0
        {
            return TCL_ERROR;
        }
        if isEnable != 0 && !crashVfs.pAppData.is_null()
            || isEnable == 0 && crashVfs.pAppData.is_null()
        {
            return TCL_OK;
        }
        if crashVfs.pAppData.is_null() {
            let mut pOriginalVfs: *mut sqlite3_vfs = sqlite3_vfs_find(
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            crashVfs.mxPathname = (*pOriginalVfs).mxPathname;
            crashVfs.pAppData = pOriginalVfs as *mut ::core::ffi::c_void;
            crashVfs.szOsFile = (::core::mem::size_of::<CrashFile>() as usize)
                .wrapping_add((*pOriginalVfs).szOsFile as usize) as ::core::ffi::c_int;
            sqlite3_vfs_register(&raw mut crashVfs, isDefault);
        } else {
            crashVfs.pAppData = ::core::ptr::null_mut::<::core::ffi::c_void>();
            sqlite3_vfs_unregister(&raw mut crashVfs);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn crashParamsObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iDelay: ::core::ffi::c_int = 0;
        let mut zCrashFile: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nCrashFile: ::core::ffi::c_int = 0;
        let mut iDc: ::core::ffi::c_int = 0;
        let mut iSectorSize: ::core::ffi::c_int = 0;
        iDc = -1 as ::core::ffi::c_int;
        iSectorSize = -1 as ::core::ffi::c_int;
        if objc < 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?OPTIONS? DELAY CRASHFILE\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            zCrashFile = Tcl_GetStringFromObj(
                *objv.offset((objc - 1 as ::core::ffi::c_int) as isize),
                &raw mut nCrashFile,
            );
            if nCrashFile as usize
                >= ::core::mem::size_of::<[::core::ffi::c_char; 500]>() as usize
            {
                Tcl_AppendResult(
                    interp,
                    b"Filename is too long: \"\0".as_ptr() as *const ::core::ffi::c_char,
                    zCrashFile,
                    b"\"\0".as_ptr() as *const ::core::ffi::c_char,
                    NULL,
                );
            } else if !(Tcl_GetIntFromObj(
                interp,
                *objv.offset((objc - 2 as ::core::ffi::c_int) as isize),
                &raw mut iDelay,
            ) != 0)
            {
                if processDevSymArgs(
                    interp,
                    objc - 3 as ::core::ffi::c_int,
                    objv.offset(1 as ::core::ffi::c_int as isize) as *const *mut Tcl_Obj,
                    &raw mut iDc,
                    &raw mut iSectorSize,
                ) != 0
                {
                    return TCL_ERROR;
                }
                if iDc >= 0 as ::core::ffi::c_int {
                    g.iDeviceCharacteristics = iDc;
                }
                if iSectorSize >= 0 as ::core::ffi::c_int {
                    g.iSectorSize = iSectorSize;
                }
                g.iCrash = iDelay;
                memcpy(
                    &raw mut g.zCrashFile as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    zCrashFile as *const ::core::ffi::c_void,
                    (nCrashFile + 1 as ::core::ffi::c_int) as size_t,
                );
                sqlite3CrashTestEnable = 1 as ::core::ffi::c_int;
                return TCL_OK;
            }
        }
        return TCL_ERROR;
    }
}
unsafe extern "C" fn devSymObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            #[link_name = "devsym_register"]
            fn devsym_register_0(
                iDeviceChar: ::core::ffi::c_int,
                iSectorSize: ::core::ffi::c_int,
            );
        }
        let mut iDc: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iSectorSize: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        if processDevSymArgs(
            interp,
            objc - 1 as ::core::ffi::c_int,
            objv.offset(1 as ::core::ffi::c_int as isize) as *const *mut Tcl_Obj,
            &raw mut iDc,
            &raw mut iSectorSize,
        ) != 0
        {
            return TCL_ERROR;
        }
        devsym_register_0(iDc, iSectorSize);
        return TCL_OK;
    }
}
unsafe extern "C" fn writeCrashObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            #[link_name = "devsym_crash_on_write"]
            fn devsym_crash_on_write_0(_: ::core::ffi::c_int);
        }
        let mut nWrite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"NWRITE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut nWrite,
        ) != 0
        {
            return TCL_ERROR;
        }
        devsym_crash_on_write_0(nWrite);
        return TCL_OK;
    }
}
unsafe extern "C" fn dsUnregisterObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            #[link_name = "devsym_unregister"]
            fn devsym_unregister_0();
        }
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        devsym_unregister_0();
        return TCL_OK;
    }
}
unsafe extern "C" fn jtObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            #[link_name = "jt_register"]
            fn jt_register_0(
                _: *mut ::core::ffi::c_char,
                _: ::core::ffi::c_int,
            ) -> ::core::ffi::c_int;
        }
        let mut zParent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?-default? PARENT-VFS\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zParent = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        if objc == 3 as ::core::ffi::c_int {
            if strcmp(zParent, b"-default\0".as_ptr() as *const ::core::ffi::c_char) != 0
            {
                Tcl_AppendResult(
                    interp,
                    b"bad option \"\0".as_ptr() as *const ::core::ffi::c_char,
                    zParent,
                    b"\": must be -default\0".as_ptr() as *const ::core::ffi::c_char,
                    NULL,
                );
                return TCL_ERROR;
            }
            zParent = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        }
        if *zParent == 0 {
            zParent = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if jt_register_0(
            zParent,
            (objc == 3 as ::core::ffi::c_int) as ::core::ffi::c_int,
        ) != 0
        {
            Tcl_AppendResult(
                interp,
                b"Error in jt_register\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn jtUnregisterObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            #[link_name = "jt_unregister"]
            fn jt_unregister_0();
        }
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        jt_unregister_0();
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest6_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_crash_enable\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                crashEnableCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_crashparams\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                crashParamsObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_crash_now\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                crashNowCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_simulate_device\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                devSymObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_crash_on_write\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                writeCrashObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"unregister_devsim\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                dsUnregisterObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"register_jt_vfs\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                jtObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"unregister_jt_vfs\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                jtUnregisterObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        return TCL_OK;
    }
}
