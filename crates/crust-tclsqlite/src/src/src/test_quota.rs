use ::libc;
unsafe extern "C" {
    pub type sqlite3_mutex;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Tcl_Command_;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
    fn sqlite3_mutex_alloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3_mutex_free(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    fn rewind(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
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
    fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ftruncate(__fd: ::core::ffi::c_int, __length: __off_t) -> ::core::ffi::c_int;
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
    fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_NewStringObj(
        bytes: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_BackgroundError(interp: *mut Tcl_Interp);
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_ObjGetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_ObjSetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        newValuePtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_SetResult(
        interp: *mut Tcl_Interp,
        result: *mut ::core::ffi::c_char,
        freeProc: Option<Tcl_FreeProc>,
    );
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_UnsetVar2(
        interp: *mut Tcl_Interp,
        part1: *const ::core::ffi::c_char,
        part2: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_AppendObjToObj(objPtr: *mut Tcl_Obj, appendObjPtr: *mut Tcl_Obj);
    fn Tcl_EvalObjEx(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn Tcl_GetWideIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        widePtr: *mut Tcl_WideInt,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3TestTextToPtr(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
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
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub pOrigVfs: *mut sqlite3_vfs,
    pub sThisVfs: sqlite3_vfs,
    pub sIoMethodsV1: sqlite3_io_methods,
    pub sIoMethodsV2: sqlite3_io_methods,
    pub isInitialized: ::core::ffi::c_int,
    pub pMutex: *mut sqlite3_mutex,
    pub pGroup: *mut quotaGroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quotaGroup {
    pub zPattern: *const ::core::ffi::c_char,
    pub iLimit: sqlite3_int64,
    pub iSize: sqlite3_int64,
    pub xCallback: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *mut sqlite3_int64,
            sqlite3_int64,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub pArg: *mut ::core::ffi::c_void,
    pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pNext: *mut quotaGroup,
    pub ppPrev: *mut *mut quotaGroup,
    pub pFiles: *mut quotaFile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quotaFile {
    pub zFilename: *mut ::core::ffi::c_char,
    pub pGroup: *mut quotaGroup,
    pub iSize: sqlite3_int64,
    pub nRef: ::core::ffi::c_int,
    pub deleteOnClose: ::core::ffi::c_int,
    pub pNext: *mut quotaFile,
    pub ppPrev: *mut *mut quotaFile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quotaConn {
    pub base: sqlite3_file,
    pub pFile: *mut quotaFile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quota_FILE {
    pub f: *mut FILE,
    pub iOfst: sqlite3_int64,
    pub pFile: *mut quotaFile,
}
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
    pub internalRep: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2Rust_Unnamed_2,
    pub ptrAndLongRep: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
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
pub type Tcl_FreeProc = unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TclQuotaCallback {
    pub interp: *mut Tcl_Interp,
    pub pScript: *mut Tcl_Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_FULL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_CANTOPEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_OPEN_DELETEONCLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_WAL: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_VFSNAME: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_FAST: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut gQuota: C2Rust_Unnamed = C2Rust_Unnamed {
    pOrigVfs: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
    sThisVfs: sqlite3_vfs {
        iVersion: 0,
        szOsFile: 0,
        mxPathname: 0,
        pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
        zName: ::core::ptr::null::<::core::ffi::c_char>(),
        pAppData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        xOpen: None,
        xDelete: None,
        xAccess: None,
        xFullPathname: None,
        xDlOpen: None,
        xDlError: None,
        xDlSym: None,
        xDlClose: None,
        xRandomness: None,
        xSleep: None,
        xCurrentTime: None,
        xGetLastError: None,
        xCurrentTimeInt64: None,
        xSetSystemCall: None,
        xGetSystemCall: None,
        xNextSystemCall: None,
    },
    sIoMethodsV1: sqlite3_io_methods {
        iVersion: 0,
        xClose: None,
        xRead: None,
        xWrite: None,
        xTruncate: None,
        xSync: None,
        xFileSize: None,
        xLock: None,
        xUnlock: None,
        xCheckReservedLock: None,
        xFileControl: None,
        xSectorSize: None,
        xDeviceCharacteristics: None,
        xShmMap: None,
        xShmLock: None,
        xShmBarrier: None,
        xShmUnmap: None,
        xFetch: None,
        xUnfetch: None,
    },
    sIoMethodsV2: sqlite3_io_methods {
        iVersion: 0,
        xClose: None,
        xRead: None,
        xWrite: None,
        xTruncate: None,
        xSync: None,
        xFileSize: None,
        xLock: None,
        xUnlock: None,
        xCheckReservedLock: None,
        xFileControl: None,
        xSectorSize: None,
        xDeviceCharacteristics: None,
        xShmMap: None,
        xShmLock: None,
        xShmBarrier: None,
        xShmUnmap: None,
        xFetch: None,
        xUnfetch: None,
    },
    isInitialized: 0,
    pMutex: ::core::ptr::null::<sqlite3_mutex>() as *mut sqlite3_mutex,
    pGroup: ::core::ptr::null::<quotaGroup>() as *mut quotaGroup,
};
unsafe extern "C" fn quotaEnter() {
    unsafe {
        sqlite3_mutex_enter(gQuota.pMutex);
    }
}
unsafe extern "C" fn quotaLeave() {
    unsafe {
        sqlite3_mutex_leave(gQuota.pMutex);
    }
}
unsafe extern "C" fn quotaGroupOpenFileCount(
    mut pGroup: *mut quotaGroup,
) -> ::core::ffi::c_int {
    unsafe {
        let mut N: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pFile: *mut quotaFile = (*pGroup).pFiles;
        while !pFile.is_null() {
            if (*pFile).nRef != 0 {
                N += 1;
            }
            pFile = (*pFile).pNext;
        }
        return N;
    }
}
unsafe extern "C" fn quotaRemoveFile(mut pFile: *mut quotaFile) {
    unsafe {
        let mut pGroup: *mut quotaGroup = (*pFile).pGroup;
        (*pGroup).iSize -= (*pFile).iSize as ::core::ffi::c_longlong;
        *(*pFile).ppPrev = (*pFile).pNext;
        if !(*pFile).pNext.is_null() {
            (*(*pFile).pNext).ppPrev = (*pFile).ppPrev;
        }
        sqlite3_free(pFile as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn quotaRemoveAllFiles(mut pGroup: *mut quotaGroup) {
    unsafe {
        while !(*pGroup).pFiles.is_null() {
            quotaRemoveFile((*pGroup).pFiles);
        }
    }
}
unsafe extern "C" fn quotaGroupDeref(mut pGroup: *mut quotaGroup) {
    unsafe {
        if (*pGroup).iLimit == 0 as ::core::ffi::c_longlong
            && quotaGroupOpenFileCount(pGroup) == 0 as ::core::ffi::c_int
        {
            quotaRemoveAllFiles(pGroup);
            *(*pGroup).ppPrev = (*pGroup).pNext;
            if !(*pGroup).pNext.is_null() {
                (*(*pGroup).pNext).ppPrev = (*pGroup).ppPrev;
            }
            if (*pGroup).xDestroy.is_some() {
                (*pGroup).xDestroy.expect("non-null function pointer")((*pGroup).pArg);
            }
            sqlite3_free(pGroup as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn quotaStrglob(
    mut zGlob: *const ::core::ffi::c_char,
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: ::core::ffi::c_int = 0;
        let mut c2: ::core::ffi::c_int = 0;
        let mut cx: ::core::ffi::c_int = 0;
        let mut invert: ::core::ffi::c_int = 0;
        let mut seen: ::core::ffi::c_int = 0;
        loop {
            let c2rust_fresh1 = zGlob;
            zGlob = zGlob.offset(1);
            c = *c2rust_fresh1 as ::core::ffi::c_int;
            if !(c != 0 as ::core::ffi::c_int) {
                break;
            }
            if c == '*' as i32 {
                loop {
                    let c2rust_fresh2 = zGlob;
                    zGlob = zGlob.offset(1);
                    c = *c2rust_fresh2 as ::core::ffi::c_int;
                    if !(c == '*' as i32 || c == '?' as i32) {
                        break;
                    }
                    if c == '?' as i32
                        && {
                            let c2rust_fresh3 = z;
                            z = z.offset(1);
                            *c2rust_fresh3 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        }
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                if c == 0 as ::core::ffi::c_int {
                    return 1 as ::core::ffi::c_int
                } else if c == '[' as i32 {
                    while *z as ::core::ffi::c_int != 0
                        && quotaStrglob(
                            zGlob.offset(-(1 as ::core::ffi::c_int as isize)),
                            z,
                        ) == 0 as ::core::ffi::c_int
                    {
                        z = z.offset(1);
                    }
                    return (*z as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                }
                cx = if c == '/' as i32 { '\\' as i32 } else { c };
                loop {
                    let c2rust_fresh4 = z;
                    z = z.offset(1);
                    c2 = *c2rust_fresh4 as ::core::ffi::c_int;
                    if !(c2 != 0 as ::core::ffi::c_int) {
                        break;
                    }
                    while c2 != c && c2 != cx {
                        let c2rust_fresh5 = z;
                        z = z.offset(1);
                        c2 = *c2rust_fresh5 as ::core::ffi::c_int;
                        if c2 == 0 as ::core::ffi::c_int {
                            return 0 as ::core::ffi::c_int;
                        }
                    }
                    if quotaStrglob(zGlob, z) != 0 {
                        return 1 as ::core::ffi::c_int;
                    }
                }
                return 0 as ::core::ffi::c_int;
            } else if c == '?' as i32 {
                let c2rust_fresh6 = z;
                z = z.offset(1);
                if *c2rust_fresh6 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
            } else if c == '[' as i32 {
                let mut prior_c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                seen = 0 as ::core::ffi::c_int;
                invert = 0 as ::core::ffi::c_int;
                let c2rust_fresh7 = z;
                z = z.offset(1);
                c = *c2rust_fresh7 as ::core::ffi::c_int;
                if c == 0 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
                let c2rust_fresh8 = zGlob;
                zGlob = zGlob.offset(1);
                c2 = *c2rust_fresh8 as ::core::ffi::c_int;
                if c2 == '^' as i32 {
                    invert = 1 as ::core::ffi::c_int;
                    let c2rust_fresh9 = zGlob;
                    zGlob = zGlob.offset(1);
                    c2 = *c2rust_fresh9 as ::core::ffi::c_int;
                }
                if c2 == ']' as i32 {
                    if c == ']' as i32 {
                        seen = 1 as ::core::ffi::c_int;
                    }
                    let c2rust_fresh10 = zGlob;
                    zGlob = zGlob.offset(1);
                    c2 = *c2rust_fresh10 as ::core::ffi::c_int;
                }
                while c2 != 0 && c2 != ']' as i32 {
                    if c2 == '-' as i32
                        && *zGlob.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int != ']' as i32
                        && *zGlob.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                        && prior_c > 0 as ::core::ffi::c_int
                    {
                        let c2rust_fresh11 = zGlob;
                        zGlob = zGlob.offset(1);
                        c2 = *c2rust_fresh11 as ::core::ffi::c_int;
                        if c >= prior_c && c <= c2 {
                            seen = 1 as ::core::ffi::c_int;
                        }
                        prior_c = 0 as ::core::ffi::c_int;
                    } else {
                        if c == c2 {
                            seen = 1 as ::core::ffi::c_int;
                        }
                        prior_c = c2;
                    }
                    let c2rust_fresh12 = zGlob;
                    zGlob = zGlob.offset(1);
                    c2 = *c2rust_fresh12 as ::core::ffi::c_int;
                }
                if c2 == 0 as ::core::ffi::c_int
                    || seen ^ invert == 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
            } else if c == '/' as i32 {
                if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '/' as i32
                    && *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != '\\' as i32
                {
                    return 0 as ::core::ffi::c_int;
                }
                z = z.offset(1);
            } else {
                let c2rust_fresh13 = z;
                z = z.offset(1);
                if c != *c2rust_fresh13 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        return (*z as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn quotaGroupFind(
    mut zFilename: *const ::core::ffi::c_char,
) -> *mut quotaGroup {
    unsafe {
        let mut p: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        p = gQuota.pGroup;
        while !p.is_null()
            && quotaStrglob((*p).zPattern, zFilename) == 0 as ::core::ffi::c_int
        {
            p = (*p).pNext;
        }
        return p;
    }
}
unsafe extern "C" fn quotaSubOpen(mut pConn: *mut sqlite3_file) -> *mut sqlite3_file {
    unsafe {
        let mut p: *mut quotaConn = pConn as *mut quotaConn;
        return p.offset(1 as ::core::ffi::c_int as isize) as *mut quotaConn
            as *mut sqlite3_file;
    }
}
unsafe extern "C" fn quotaFindFile(
    mut pGroup: *mut quotaGroup,
    mut zName: *const ::core::ffi::c_char,
    mut createFlag: ::core::ffi::c_int,
) -> *mut quotaFile {
    unsafe {
        let mut pFile: *mut quotaFile = (*pGroup).pFiles;
        while !pFile.is_null()
            && strcmp((*pFile).zFilename, zName) != 0 as ::core::ffi::c_int
        {
            pFile = (*pFile).pNext;
        }
        if pFile.is_null() && createFlag != 0 {
            let mut nName: ::core::ffi::c_int = (strlen(zName)
                & 0x3fffffff as ::core::ffi::c_int as size_t) as ::core::ffi::c_int;
            pFile = sqlite3_malloc(
                (::core::mem::size_of::<quotaFile>() as usize)
                    .wrapping_add(nName as usize)
                    .wrapping_add(1 as usize) as ::core::ffi::c_int,
            ) as *mut quotaFile;
            if !pFile.is_null() {
                memset(
                    pFile as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<quotaFile>() as size_t,
                );
                (*pFile).zFilename = pFile.offset(1 as ::core::ffi::c_int as isize)
                    as *mut quotaFile as *mut ::core::ffi::c_char;
                memcpy(
                    (*pFile).zFilename as *mut ::core::ffi::c_void,
                    zName as *const ::core::ffi::c_void,
                    (nName + 1 as ::core::ffi::c_int) as size_t,
                );
                (*pFile).pNext = (*pGroup).pFiles;
                if !(*pGroup).pFiles.is_null() {
                    (*(*pGroup).pFiles).ppPrev = &raw mut (*pFile).pNext;
                }
                (*pFile).ppPrev = &raw mut (*pGroup).pFiles;
                (*pGroup).pFiles = pFile;
                (*pFile).pGroup = pGroup;
            }
        }
        return pFile;
    }
}
unsafe extern "C" fn quota_utf8_to_mbcs(
    mut zUtf8: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        return zUtf8 as *mut ::core::ffi::c_char;
    }
}
unsafe extern "C" fn quota_mbcs_free(mut zOld: *mut ::core::ffi::c_char) {}
unsafe extern "C" fn quotaOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pConn: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pQuotaOpen: *mut quotaConn = ::core::ptr::null_mut::<quotaConn>();
        let mut pFile: *mut quotaFile = ::core::ptr::null_mut::<quotaFile>();
        let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        let mut pSubOpen: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut pOrigVfs: *mut sqlite3_vfs = gQuota.pOrigVfs;
        if flags & (SQLITE_OPEN_MAIN_DB | SQLITE_OPEN_WAL) == 0 as ::core::ffi::c_int {
            return (*pOrigVfs)
                .xOpen
                .expect(
                    "non-null function pointer",
                )(pOrigVfs, zName as sqlite3_filename, pConn, flags, pOutFlags);
        }
        quotaEnter();
        pGroup = quotaGroupFind(zName);
        if pGroup.is_null() {
            rc = (*pOrigVfs)
                .xOpen
                .expect(
                    "non-null function pointer",
                )(pOrigVfs, zName as sqlite3_filename, pConn, flags, pOutFlags);
        } else {
            pQuotaOpen = pConn as *mut quotaConn;
            pSubOpen = quotaSubOpen(pConn);
            rc = (*pOrigVfs)
                .xOpen
                .expect(
                    "non-null function pointer",
                )(pOrigVfs, zName as sqlite3_filename, pSubOpen, flags, pOutFlags);
            if rc == SQLITE_OK {
                pFile = quotaFindFile(pGroup, zName, 1 as ::core::ffi::c_int);
                if pFile.is_null() {
                    quotaLeave();
                    (*(*pSubOpen).pMethods)
                        .xClose
                        .expect("non-null function pointer")(pSubOpen);
                    return SQLITE_NOMEM;
                }
                (*pFile).deleteOnClose = (flags & SQLITE_OPEN_DELETEONCLOSE
                    != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                (*pFile).nRef += 1;
                (*pQuotaOpen).pFile = pFile;
                if (*(*pSubOpen).pMethods).iVersion == 1 as ::core::ffi::c_int {
                    (*pQuotaOpen).base.pMethods = &raw mut gQuota.sIoMethodsV1;
                } else {
                    (*pQuotaOpen).base.pMethods = &raw mut gQuota.sIoMethodsV2;
                }
            }
        }
        quotaLeave();
        return rc;
    }
}
unsafe extern "C" fn quotaDelete(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut syncDir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pFile: *mut quotaFile = ::core::ptr::null_mut::<quotaFile>();
        let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        let mut pOrigVfs: *mut sqlite3_vfs = gQuota.pOrigVfs;
        rc = (*pOrigVfs)
            .xDelete
            .expect("non-null function pointer")(pOrigVfs, zName, syncDir);
        if rc == SQLITE_OK {
            quotaEnter();
            pGroup = quotaGroupFind(zName);
            if !pGroup.is_null() {
                pFile = quotaFindFile(pGroup, zName, 0 as ::core::ffi::c_int);
                if !pFile.is_null() {
                    if (*pFile).nRef != 0 {
                        (*pFile).deleteOnClose = 1 as ::core::ffi::c_int;
                    } else {
                        quotaRemoveFile(pFile);
                        quotaGroupDeref(pGroup);
                    }
                }
            }
            quotaLeave();
        }
        return rc;
    }
}
unsafe extern "C" fn quotaClose(mut pConn: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quotaConn = pConn as *mut quotaConn;
        let mut pFile: *mut quotaFile = (*p).pFile;
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        let mut rc: ::core::ffi::c_int = 0;
        rc = (*(*pSubOpen).pMethods)
            .xClose
            .expect("non-null function pointer")(pSubOpen);
        quotaEnter();
        (*pFile).nRef -= 1;
        if (*pFile).nRef == 0 as ::core::ffi::c_int {
            let mut pGroup: *mut quotaGroup = (*pFile).pGroup;
            if (*pFile).deleteOnClose != 0 {
                (*gQuota.pOrigVfs)
                    .xDelete
                    .expect(
                        "non-null function pointer",
                    )(gQuota.pOrigVfs, (*pFile).zFilename, 0 as ::core::ffi::c_int);
                quotaRemoveFile(pFile);
            }
            quotaGroupDeref(pGroup);
        }
        quotaLeave();
        return rc;
    }
}
unsafe extern "C" fn quotaRead(
    mut pConn: *mut sqlite3_file,
    mut pBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        return (*(*pSubOpen).pMethods)
            .xRead
            .expect("non-null function pointer")(pSubOpen, pBuf, iAmt, iOfst);
    }
}
unsafe extern "C" fn quotaWrite(
    mut pConn: *mut sqlite3_file,
    mut pBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quotaConn = pConn as *mut quotaConn;
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        let mut iEnd: sqlite3_int64 = iOfst + iAmt as sqlite3_int64;
        let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        let mut pFile: *mut quotaFile = (*p).pFile;
        let mut szNew: sqlite3_int64 = 0;
        if (*pFile).iSize < iEnd {
            pGroup = (*pFile).pGroup;
            quotaEnter();
            szNew = (*pGroup).iSize - (*pFile).iSize + iEnd;
            if szNew > (*pGroup).iLimit
                && (*pGroup).iLimit > 0 as ::core::ffi::c_longlong
            {
                if (*pGroup).xCallback.is_some() {
                    (*pGroup)
                        .xCallback
                        .expect(
                            "non-null function pointer",
                        )(
                        (*pFile).zFilename,
                        &raw mut (*pGroup).iLimit,
                        szNew,
                        (*pGroup).pArg,
                    );
                }
                if szNew > (*pGroup).iLimit
                    && (*pGroup).iLimit > 0 as ::core::ffi::c_longlong
                {
                    quotaLeave();
                    return SQLITE_FULL;
                }
            }
            (*pGroup).iSize = szNew;
            (*pFile).iSize = iEnd;
            quotaLeave();
        }
        return (*(*pSubOpen).pMethods)
            .xWrite
            .expect("non-null function pointer")(pSubOpen, pBuf, iAmt, iOfst);
    }
}
unsafe extern "C" fn quotaTruncate(
    mut pConn: *mut sqlite3_file,
    mut size: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quotaConn = pConn as *mut quotaConn;
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        let mut rc: ::core::ffi::c_int = (*(*pSubOpen).pMethods)
            .xTruncate
            .expect("non-null function pointer")(pSubOpen, size);
        let mut pFile: *mut quotaFile = (*p).pFile;
        let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        if rc == SQLITE_OK {
            quotaEnter();
            pGroup = (*pFile).pGroup;
            (*pGroup).iSize -= (*pFile).iSize as ::core::ffi::c_longlong;
            (*pFile).iSize = size;
            (*pGroup).iSize += size as ::core::ffi::c_longlong;
            quotaLeave();
        }
        return rc;
    }
}
unsafe extern "C" fn quotaSync(
    mut pConn: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        return (*(*pSubOpen).pMethods)
            .xSync
            .expect("non-null function pointer")(pSubOpen, flags);
    }
}
unsafe extern "C" fn quotaFileSize(
    mut pConn: *mut sqlite3_file,
    mut pSize: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quotaConn = pConn as *mut quotaConn;
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        let mut pFile: *mut quotaFile = (*p).pFile;
        let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        let mut sz: sqlite3_int64 = 0;
        let mut rc: ::core::ffi::c_int = 0;
        rc = (*(*pSubOpen).pMethods)
            .xFileSize
            .expect("non-null function pointer")(pSubOpen, &raw mut sz);
        if rc == SQLITE_OK {
            quotaEnter();
            pGroup = (*pFile).pGroup;
            (*pGroup).iSize -= (*pFile).iSize as ::core::ffi::c_longlong;
            (*pFile).iSize = sz;
            (*pGroup).iSize += sz as ::core::ffi::c_longlong;
            quotaLeave();
            *pSize = sz;
        }
        return rc;
    }
}
unsafe extern "C" fn quotaLock(
    mut pConn: *mut sqlite3_file,
    mut lock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        return (*(*pSubOpen).pMethods)
            .xLock
            .expect("non-null function pointer")(pSubOpen, lock);
    }
}
unsafe extern "C" fn quotaUnlock(
    mut pConn: *mut sqlite3_file,
    mut lock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        return (*(*pSubOpen).pMethods)
            .xUnlock
            .expect("non-null function pointer")(pSubOpen, lock);
    }
}
unsafe extern "C" fn quotaCheckReservedLock(
    mut pConn: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        return (*(*pSubOpen).pMethods)
            .xCheckReservedLock
            .expect("non-null function pointer")(pSubOpen, pResOut);
    }
}
unsafe extern "C" fn quotaFileControl(
    mut pConn: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        let mut rc: ::core::ffi::c_int = (*(*pSubOpen).pMethods)
            .xFileControl
            .expect("non-null function pointer")(pSubOpen, op, pArg);
        if op == SQLITE_FCNTL_VFSNAME && rc == SQLITE_OK {
            let ref mut c2rust_fresh0 = *(pArg as *mut *mut ::core::ffi::c_char);
            *c2rust_fresh0 = sqlite3_mprintf(
                b"quota/%z\0".as_ptr() as *const ::core::ffi::c_char,
                *(pArg as *mut *mut ::core::ffi::c_char),
            );
        }
        return rc;
    }
}
unsafe extern "C" fn quotaSectorSize(
    mut pConn: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        return (*(*pSubOpen).pMethods)
            .xSectorSize
            .expect("non-null function pointer")(pSubOpen);
    }
}
unsafe extern "C" fn quotaDeviceCharacteristics(
    mut pConn: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        return (*(*pSubOpen).pMethods)
            .xDeviceCharacteristics
            .expect("non-null function pointer")(pSubOpen);
    }
}
unsafe extern "C" fn quotaShmMap(
    mut pConn: *mut sqlite3_file,
    mut iRegion: ::core::ffi::c_int,
    mut szRegion: ::core::ffi::c_int,
    mut bExtend: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        return (*(*pSubOpen).pMethods)
            .xShmMap
            .expect(
                "non-null function pointer",
            )(pSubOpen, iRegion, szRegion, bExtend, pp);
    }
}
unsafe extern "C" fn quotaShmLock(
    mut pConn: *mut sqlite3_file,
    mut ofst: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        return (*(*pSubOpen).pMethods)
            .xShmLock
            .expect("non-null function pointer")(pSubOpen, ofst, n, flags);
    }
}
unsafe extern "C" fn quotaShmBarrier(mut pConn: *mut sqlite3_file) {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        (*(*pSubOpen).pMethods)
            .xShmBarrier
            .expect("non-null function pointer")(pSubOpen);
    }
}
unsafe extern "C" fn quotaShmUnmap(
    mut pConn: *mut sqlite3_file,
    mut deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = quotaSubOpen(pConn);
        return (*(*pSubOpen).pMethods)
            .xShmUnmap
            .expect("non-null function pointer")(pSubOpen, deleteFlag);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_initialize(
    mut zOrigVfsName: *const ::core::ffi::c_char,
    mut makeDefault: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pOrigVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        if gQuota.isInitialized != 0 {
            return SQLITE_MISUSE;
        }
        pOrigVfs = sqlite3_vfs_find(zOrigVfsName);
        if pOrigVfs.is_null() {
            return SQLITE_ERROR;
        }
        gQuota.pMutex = sqlite3_mutex_alloc(SQLITE_MUTEX_FAST);
        if gQuota.pMutex.is_null() {
            return SQLITE_NOMEM;
        }
        gQuota.isInitialized = 1 as ::core::ffi::c_int;
        gQuota.pOrigVfs = pOrigVfs;
        gQuota.sThisVfs = *pOrigVfs;
        gQuota.sThisVfs.xOpen = Some(
            quotaOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    sqlite3_filename,
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sThisVfs.xDelete = Some(
            quotaDelete
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sThisVfs.szOsFile = (gQuota.sThisVfs.szOsFile as ::core::ffi::c_ulong)
            .wrapping_add(
                ::core::mem::size_of::<quotaConn>() as usize as ::core::ffi::c_ulong,
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        gQuota.sThisVfs.zName = b"quota\0".as_ptr() as *const ::core::ffi::c_char;
        gQuota.sIoMethodsV1.iVersion = 1 as ::core::ffi::c_int;
        gQuota.sIoMethodsV1.xClose = Some(
            quotaClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>;
        gQuota.sIoMethodsV1.xRead = Some(
            quotaRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV1.xWrite = Some(
            quotaWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV1.xTruncate = Some(
            quotaTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV1.xSync = Some(
            quotaSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV1.xFileSize = Some(
            quotaFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV1.xLock = Some(
            quotaLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV1.xUnlock = Some(
            quotaUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV1.xCheckReservedLock = Some(
            quotaCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV1.xFileControl = Some(
            quotaFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV1.xSectorSize = Some(
            quotaSectorSize
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>;
        gQuota.sIoMethodsV1.xDeviceCharacteristics = Some(
            quotaDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>;
        gQuota.sIoMethodsV2 = gQuota.sIoMethodsV1;
        gQuota.sIoMethodsV2.iVersion = 2 as ::core::ffi::c_int;
        gQuota.sIoMethodsV2.xShmMap = Some(
            quotaShmMap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV2.xShmLock = Some(
            quotaShmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gQuota.sIoMethodsV2.xShmBarrier = Some(
            quotaShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
        ) as Option<unsafe extern "C" fn(*mut sqlite3_file) -> ()>;
        gQuota.sIoMethodsV2.xShmUnmap = Some(
            quotaShmUnmap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        sqlite3_vfs_register(&raw mut gQuota.sThisVfs, makeDefault);
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_shutdown() -> ::core::ffi::c_int {
    unsafe {
        let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        if gQuota.isInitialized == 0 as ::core::ffi::c_int {
            return SQLITE_MISUSE;
        }
        pGroup = gQuota.pGroup;
        while !pGroup.is_null() {
            if quotaGroupOpenFileCount(pGroup) > 0 as ::core::ffi::c_int {
                return SQLITE_MISUSE;
            }
            pGroup = (*pGroup).pNext;
        }
        while !gQuota.pGroup.is_null() {
            pGroup = gQuota.pGroup;
            gQuota.pGroup = (*pGroup).pNext;
            (*pGroup).iLimit = 0 as sqlite3_int64;
            quotaGroupDeref(pGroup);
        }
        gQuota.isInitialized = 0 as ::core::ffi::c_int;
        sqlite3_mutex_free(gQuota.pMutex);
        sqlite3_vfs_unregister(&raw mut gQuota.sThisVfs);
        memset(
            &raw mut gQuota as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<C2Rust_Unnamed>() as size_t,
        );
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_set(
    mut zPattern: *const ::core::ffi::c_char,
    mut iLimit: sqlite3_int64,
    mut xCallback: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *mut sqlite3_int64,
            sqlite3_int64,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut pArg: *mut ::core::ffi::c_void,
    mut xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        quotaEnter();
        pGroup = gQuota.pGroup;
        while !pGroup.is_null()
            && strcmp((*pGroup).zPattern, zPattern) != 0 as ::core::ffi::c_int
        {
            pGroup = (*pGroup).pNext;
        }
        if pGroup.is_null() {
            let mut nPattern: ::core::ffi::c_int = (strlen(zPattern)
                & 0x3fffffff as ::core::ffi::c_int as size_t) as ::core::ffi::c_int;
            if iLimit <= 0 as ::core::ffi::c_longlong {
                quotaLeave();
                return SQLITE_OK;
            }
            pGroup = sqlite3_malloc(
                (::core::mem::size_of::<quotaGroup>() as usize)
                    .wrapping_add(nPattern as usize)
                    .wrapping_add(1 as usize) as ::core::ffi::c_int,
            ) as *mut quotaGroup;
            if pGroup.is_null() {
                quotaLeave();
                return SQLITE_NOMEM;
            }
            memset(
                pGroup as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<quotaGroup>() as size_t,
            );
            (*pGroup).zPattern = pGroup.offset(1 as ::core::ffi::c_int as isize)
                as *mut quotaGroup as *mut ::core::ffi::c_char;
            memcpy(
                (*pGroup).zPattern as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                zPattern as *const ::core::ffi::c_void,
                (nPattern + 1 as ::core::ffi::c_int) as size_t,
            );
            if !gQuota.pGroup.is_null() {
                (*gQuota.pGroup).ppPrev = &raw mut (*pGroup).pNext;
            }
            (*pGroup).pNext = gQuota.pGroup;
            (*pGroup).ppPrev = &raw mut gQuota.pGroup;
            gQuota.pGroup = pGroup;
        }
        (*pGroup).iLimit = iLimit;
        (*pGroup).xCallback = xCallback;
        if (*pGroup).xDestroy.is_some() && (*pGroup).pArg != pArg {
            (*pGroup).xDestroy.expect("non-null function pointer")((*pGroup).pArg);
        }
        (*pGroup).pArg = pArg;
        (*pGroup).xDestroy = xDestroy;
        quotaGroupDeref(pGroup);
        quotaLeave();
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_file(
    mut zFilename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zFull: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut fd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut outFlags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iSize: sqlite3_int64 = 0;
        let mut nAlloc: ::core::ffi::c_int = gQuota.sThisVfs.szOsFile
            + gQuota.sThisVfs.mxPathname + 2 as ::core::ffi::c_int;
        fd = sqlite3_malloc(nAlloc) as *mut sqlite3_file;
        if fd.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            zFull = (fd as *mut ::core::ffi::c_char)
                .offset(gQuota.sThisVfs.szOsFile as isize) as *mut ::core::ffi::c_char;
            rc = (*gQuota.pOrigVfs)
                .xFullPathname
                .expect(
                    "non-null function pointer",
                )(
                gQuota.pOrigVfs,
                zFilename,
                gQuota.sThisVfs.mxPathname + 1 as ::core::ffi::c_int,
                zFull,
            );
        }
        if rc == SQLITE_OK {
            *zFull.offset(strlen(zFull).wrapping_add(1 as size_t) as isize) = '\0' as i32
                as ::core::ffi::c_char;
            rc = quotaOpen(
                &raw mut gQuota.sThisVfs,
                zFull,
                fd,
                SQLITE_OPEN_READONLY | SQLITE_OPEN_MAIN_DB,
                &raw mut outFlags,
            );
            if rc == SQLITE_OK {
                (*(*fd).pMethods)
                    .xFileSize
                    .expect("non-null function pointer")(fd, &raw mut iSize);
                (*(*fd).pMethods).xClose.expect("non-null function pointer")(fd);
            } else if rc == SQLITE_CANTOPEN {
                let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
                let mut pFile: *mut quotaFile = ::core::ptr::null_mut::<quotaFile>();
                quotaEnter();
                pGroup = quotaGroupFind(zFull);
                if !pGroup.is_null() {
                    pFile = quotaFindFile(pGroup, zFull, 0 as ::core::ffi::c_int);
                    if !pFile.is_null() {
                        quotaRemoveFile(pFile);
                    }
                }
                quotaLeave();
            }
        }
        sqlite3_free(fd as *mut ::core::ffi::c_void);
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_fopen(
    mut zFilename: *const ::core::ffi::c_char,
    mut zMode: *const ::core::ffi::c_char,
) -> *mut quota_FILE {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut zFull: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zFullTranslated: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        let mut pFile: *mut quotaFile = ::core::ptr::null_mut::<quotaFile>();
        zFull = sqlite3_malloc(gQuota.sThisVfs.mxPathname + 1 as ::core::ffi::c_int)
            as *mut ::core::ffi::c_char;
        if zFull.is_null() {
            return ::core::ptr::null_mut::<quota_FILE>();
        }
        rc = (*gQuota.pOrigVfs)
            .xFullPathname
            .expect(
                "non-null function pointer",
            )(
            gQuota.pOrigVfs,
            zFilename,
            gQuota.sThisVfs.mxPathname + 1 as ::core::ffi::c_int,
            zFull,
        );
        if !(rc != 0) {
            p = sqlite3_malloc(
                ::core::mem::size_of::<quota_FILE>() as ::core::ffi::c_int,
            ) as *mut quota_FILE;
            if !p.is_null() {
                memset(
                    p as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<quota_FILE>() as size_t,
                );
                zFullTranslated = quota_utf8_to_mbcs(zFull);
                if !zFullTranslated.is_null() {
                    (*p).f = fopen(zFullTranslated, zMode);
                    if !(*p).f.is_null() {
                        quotaEnter();
                        pGroup = quotaGroupFind(zFull);
                        if !pGroup.is_null() {
                            pFile = quotaFindFile(
                                pGroup,
                                zFull,
                                1 as ::core::ffi::c_int,
                            );
                            if pFile.is_null() {
                                quotaLeave();
                                c2rust_current_block = 11733108213561214315;
                            } else {
                                (*pFile).nRef += 1;
                                (*p).pFile = pFile;
                                c2rust_current_block = 17833034027772472439;
                            }
                        } else {
                            c2rust_current_block = 17833034027772472439;
                        }
                        match c2rust_current_block {
                            11733108213561214315 => {}
                            _ => {
                                quotaLeave();
                                sqlite3_free(zFull as *mut ::core::ffi::c_void);
                                return p;
                            }
                        }
                    }
                }
            }
        }
        quota_mbcs_free(zFullTranslated);
        sqlite3_free(zFull as *mut ::core::ffi::c_void);
        if !p.is_null() && !(*p).f.is_null() {
            fclose((*p).f);
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<quota_FILE>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_fread(
    mut pBuf: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut p: *mut quota_FILE,
) -> size_t {
    unsafe {
        return fread(pBuf, size, nmemb, (*p).f) as size_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_fwrite(
    mut pBuf: *const ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut p: *mut quota_FILE,
) -> size_t {
    unsafe {
        let mut iOfst: sqlite3_int64 = 0;
        let mut iEnd: sqlite3_int64 = 0;
        let mut szNew: sqlite3_int64 = 0;
        let mut pFile: *mut quotaFile = ::core::ptr::null_mut::<quotaFile>();
        let mut rc: size_t = 0;
        iOfst = ftell((*p).f) as sqlite3_int64;
        iEnd = (iOfst as ::core::ffi::c_ulonglong)
            .wrapping_add(size.wrapping_mul(nmemb) as ::core::ffi::c_ulonglong)
            as sqlite3_int64;
        pFile = (*p).pFile;
        if !pFile.is_null() && (*pFile).iSize < iEnd {
            let mut pGroup: *mut quotaGroup = (*pFile).pGroup;
            quotaEnter();
            szNew = (*pGroup).iSize - (*pFile).iSize + iEnd;
            if szNew > (*pGroup).iLimit
                && (*pGroup).iLimit > 0 as ::core::ffi::c_longlong
            {
                if (*pGroup).xCallback.is_some() {
                    (*pGroup)
                        .xCallback
                        .expect(
                            "non-null function pointer",
                        )(
                        (*pFile).zFilename,
                        &raw mut (*pGroup).iLimit,
                        szNew,
                        (*pGroup).pArg,
                    );
                }
                if szNew > (*pGroup).iLimit
                    && (*pGroup).iLimit > 0 as ::core::ffi::c_longlong
                {
                    iEnd = (*pGroup).iLimit - (*pGroup).iSize + (*pFile).iSize;
                    nmemb = ((iEnd - iOfst) as ::core::ffi::c_ulonglong)
                        .wrapping_div(size as ::core::ffi::c_ulonglong) as size_t;
                    iEnd = (iOfst as ::core::ffi::c_ulonglong)
                        .wrapping_add(
                            size.wrapping_mul(nmemb) as ::core::ffi::c_ulonglong,
                        ) as sqlite3_int64;
                    szNew = (*pGroup).iSize - (*pFile).iSize + iEnd;
                }
            }
            (*pGroup).iSize = szNew;
            (*pFile).iSize = iEnd;
            quotaLeave();
        } else {
            pFile = ::core::ptr::null_mut::<quotaFile>();
        }
        rc = fwrite(pBuf, size, nmemb, (*p).f) as size_t;
        if rc < nmemb && !pFile.is_null() {
            let mut nWritten: size_t = rc;
            let mut iNewEnd: sqlite3_int64 = (iOfst as ::core::ffi::c_ulonglong)
                .wrapping_add(size.wrapping_mul(nWritten) as ::core::ffi::c_ulonglong)
                as sqlite3_int64;
            if iNewEnd < iEnd {
                iNewEnd = iEnd;
            }
            quotaEnter();
            (*(*pFile).pGroup).iSize
                += (iNewEnd - (*pFile).iSize) as ::core::ffi::c_longlong;
            (*pFile).iSize = iNewEnd;
            quotaLeave();
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_fclose(
    mut p: *mut quota_FILE,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pFile: *mut quotaFile = ::core::ptr::null_mut::<quotaFile>();
        rc = fclose((*p).f);
        pFile = (*p).pFile;
        if !pFile.is_null() {
            quotaEnter();
            (*pFile).nRef -= 1;
            if (*pFile).nRef == 0 as ::core::ffi::c_int {
                let mut pGroup: *mut quotaGroup = (*pFile).pGroup;
                if (*pFile).deleteOnClose != 0 {
                    (*gQuota.pOrigVfs)
                        .xDelete
                        .expect(
                            "non-null function pointer",
                        )(gQuota.pOrigVfs, (*pFile).zFilename, 0 as ::core::ffi::c_int);
                    quotaRemoveFile(pFile);
                }
                quotaGroupDeref(pGroup);
            }
            quotaLeave();
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_fflush(
    mut p: *mut quota_FILE,
    mut doFsync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = fflush((*p).f);
        if rc == 0 as ::core::ffi::c_int && doFsync != 0 {
            rc = fsync(fileno((*p).f));
        }
        return (rc != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_fseek(
    mut p: *mut quota_FILE,
    mut offset: ::core::ffi::c_long,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return fseek((*p).f, offset, whence);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_rewind(mut p: *mut quota_FILE) {
    unsafe {
        rewind((*p).f);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_ftell(
    mut p: *mut quota_FILE,
) -> ::core::ffi::c_long {
    unsafe {
        return ftell((*p).f);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_ferror(
    mut p: *mut quota_FILE,
) -> ::core::ffi::c_int {
    unsafe {
        return ferror((*p).f);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_ftruncate(
    mut p: *mut quota_FILE,
    mut szNew: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFile: *mut quotaFile = (*p).pFile;
        let mut rc: ::core::ffi::c_int = 0;
        pFile = (*p).pFile;
        if !pFile.is_null() && (*pFile).iSize < szNew {
            let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
            if (*pFile).iSize < szNew {
                return -1 as ::core::ffi::c_int;
            }
            pGroup = (*pFile).pGroup;
            quotaEnter();
            (*pGroup).iSize += (szNew - (*pFile).iSize) as ::core::ffi::c_longlong;
            quotaLeave();
        }
        rc = ftruncate(fileno((*p).f), szNew as __off_t);
        if !pFile.is_null() && rc == 0 as ::core::ffi::c_int {
            let mut pGroup_0: *mut quotaGroup = (*pFile).pGroup;
            quotaEnter();
            (*pGroup_0).iSize += (szNew - (*pFile).iSize) as ::core::ffi::c_longlong;
            (*pFile).iSize = szNew;
            quotaLeave();
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_file_mtime(
    mut p: *mut quota_FILE,
    mut pTime: *mut time_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        rc = fstat(fileno((*p).f), &raw mut buf);
        if rc == 0 as ::core::ffi::c_int {
            *pTime = buf.st_mtim.tv_sec as time_t;
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_file_truesize(
    mut p: *mut quota_FILE,
) -> sqlite3_int64 {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        rc = fstat(fileno((*p).f), &raw mut buf);
        return (if rc == 0 as ::core::ffi::c_int {
            buf.st_size as ::core::ffi::c_long
        } else {
            -1 as ::core::ffi::c_int as ::core::ffi::c_long
        }) as sqlite3_int64;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_file_size(
    mut p: *mut quota_FILE,
) -> sqlite3_int64 {
    unsafe {
        return if !(*p).pFile.is_null() {
            (*(*p).pFile).iSize
        } else {
            -1 as ::core::ffi::c_int as sqlite3_int64
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_file_available(
    mut p: *mut quota_FILE,
) -> ::core::ffi::c_long {
    unsafe {
        let mut f: *mut FILE = (*p).f;
        let mut pos1: ::core::ffi::c_long = 0;
        let mut pos2: ::core::ffi::c_long = 0;
        let mut rc: ::core::ffi::c_int = 0;
        pos1 = ftell(f);
        if pos1 < 0 as ::core::ffi::c_long {
            return -1 as ::core::ffi::c_int as ::core::ffi::c_long;
        }
        rc = fseek(f, 0 as ::core::ffi::c_long, SEEK_END);
        if rc != 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int as ::core::ffi::c_long;
        }
        pos2 = ftell(f);
        if pos2 < 0 as ::core::ffi::c_long {
            return -1 as ::core::ffi::c_int as ::core::ffi::c_long;
        }
        rc = fseek(f, pos1, SEEK_SET);
        if rc != 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int as ::core::ffi::c_long;
        }
        return pos2 - pos1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_quota_remove(
    mut zFilename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zFull: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nFull: size_t = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        let mut pFile: *mut quotaFile = ::core::ptr::null_mut::<quotaFile>();
        let mut pNextFile: *mut quotaFile = ::core::ptr::null_mut::<quotaFile>();
        let mut diff: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_char = 0;
        zFull = sqlite3_malloc(gQuota.sThisVfs.mxPathname + 1 as ::core::ffi::c_int)
            as *mut ::core::ffi::c_char;
        if zFull.is_null() {
            return SQLITE_NOMEM;
        }
        rc = (*gQuota.pOrigVfs)
            .xFullPathname
            .expect(
                "non-null function pointer",
            )(
            gQuota.pOrigVfs,
            zFilename,
            gQuota.sThisVfs.mxPathname + 1 as ::core::ffi::c_int,
            zFull,
        );
        if rc != 0 {
            sqlite3_free(zFull as *mut ::core::ffi::c_void);
            return rc;
        }
        nFull = strlen(zFull);
        if nFull > 0 as size_t
            && (*zFull.offset(nFull.wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int == '/' as i32
                || *zFull.offset(nFull.wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_int == '\\' as i32)
        {
            nFull = nFull.wrapping_sub(1);
            *zFull.offset(nFull as isize) = 0 as ::core::ffi::c_char;
        }
        quotaEnter();
        pGroup = quotaGroupFind(zFull);
        if !pGroup.is_null() {
            pFile = (*pGroup).pFiles;
            while !pFile.is_null() && rc == SQLITE_OK {
                pNextFile = (*pFile).pNext;
                diff = strncmp(zFull, (*pFile).zFilename, nFull);
                if diff == 0 as ::core::ffi::c_int
                    && {
                        c = *(*pFile).zFilename.offset(nFull as isize);
                        c as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            || c as ::core::ffi::c_int == '/' as i32
                            || c as ::core::ffi::c_int == '\\' as i32
                    }
                {
                    if (*pFile).nRef != 0 {
                        (*pFile).deleteOnClose = 1 as ::core::ffi::c_int;
                    } else {
                        rc = (*gQuota.pOrigVfs)
                            .xDelete
                            .expect(
                                "non-null function pointer",
                            )(
                            gQuota.pOrigVfs,
                            (*pFile).zFilename,
                            0 as ::core::ffi::c_int,
                        );
                        quotaRemoveFile(pFile);
                        quotaGroupDeref(pGroup);
                    }
                }
                pFile = pNextFile;
            }
        }
        quotaLeave();
        sqlite3_free(zFull as *mut ::core::ffi::c_void);
        return rc;
    }
}
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_EVAL_GLOBAL: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const TCL_STATIC: Option<Tcl_FreeProc> = None;
unsafe extern "C" fn tclQuotaCallback(
    mut zFilename: *const ::core::ffi::c_char,
    mut piLimit: *mut sqlite3_int64,
    mut iSize: sqlite3_int64,
    mut pArg: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut p: *mut TclQuotaCallback = ::core::ptr::null_mut::<TclQuotaCallback>();
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pVarname: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut rnd: ::core::ffi::c_uint = 0;
        let mut rc: ::core::ffi::c_int = 0;
        p = pArg as *mut TclQuotaCallback;
        if p.is_null() {
            return;
        }
        pVarname = Tcl_NewStringObj(
            b"::piLimit_\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
        );
        (*pVarname).refCount += 1;
        sqlite3_randomness(
            ::core::mem::size_of::<::core::ffi::c_uint>() as ::core::ffi::c_int,
            &raw mut rnd as *mut ::core::ffi::c_void,
        );
        Tcl_AppendObjToObj(
            pVarname,
            Tcl_NewIntObj(
                (rnd & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as ::core::ffi::c_int,
            ),
        );
        Tcl_ObjSetVar2(
            (*p).interp,
            pVarname,
            ::core::ptr::null_mut::<Tcl_Obj>(),
            Tcl_NewWideIntObj(*piLimit),
            0 as ::core::ffi::c_int,
        );
        pEval = Tcl_DuplicateObj((*p).pScript);
        (*pEval).refCount += 1;
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pEval,
            Tcl_NewStringObj(zFilename, -1 as ::core::ffi::c_int),
        );
        Tcl_ListObjAppendElement(::core::ptr::null_mut::<Tcl_Interp>(), pEval, pVarname);
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pEval,
            Tcl_NewWideIntObj(iSize as Tcl_WideInt),
        );
        rc = Tcl_EvalObjEx((*p).interp, pEval, TCL_EVAL_GLOBAL);
        if rc == TCL_OK {
            let mut x: Tcl_WideInt = 0;
            let mut pLimit: *mut Tcl_Obj = Tcl_ObjGetVar2(
                (*p).interp,
                pVarname,
                ::core::ptr::null_mut::<Tcl_Obj>(),
                0 as ::core::ffi::c_int,
            );
            rc = Tcl_GetWideIntFromObj((*p).interp, pLimit, &raw mut x);
            *piLimit = x as sqlite3_int64;
            Tcl_UnsetVar2(
                (*p).interp,
                Tcl_GetString(pVarname),
                ::core::ptr::null::<::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
            );
        }
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh14 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh14 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        let mut _objPtr_0: *mut Tcl_Obj = pVarname;
        let c2rust_fresh15 = (*_objPtr_0).refCount;
        (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
        if c2rust_fresh15 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr_0);
        }
        if rc != TCL_OK {
            Tcl_BackgroundError((*p).interp);
        }
    }
}
unsafe extern "C" fn tclCallbackDestructor(mut pObj: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut TclQuotaCallback = pObj as *mut TclQuotaCallback;
        if !p.is_null() {
            let mut _objPtr: *mut Tcl_Obj = (*p).pScript;
            let c2rust_fresh16 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh16 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
            sqlite3_free(p as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn test_quota_initialize(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut makeDefault: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"NAME MAKEDEFAULT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zName = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        if Tcl_GetBooleanFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut makeDefault,
        ) != 0
        {
            return TCL_ERROR;
        }
        if *zName.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
        {
            zName = ::core::ptr::null::<::core::ffi::c_char>();
        }
        rc = sqlite3_quota_initialize(zName, makeDefault);
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_shutdown(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = sqlite3_quota_shutdown();
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_set(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zPattern: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut iLimit: Tcl_WideInt = 0;
        let mut pScript: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut TclQuotaCallback = ::core::ptr::null_mut::<TclQuotaCallback>();
        let mut nScript: ::core::ffi::c_int = 0;
        let mut xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = None;
        let mut xCallback: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *mut sqlite3_int64,
                sqlite3_int64,
                *mut ::core::ffi::c_void,
            ) -> (),
        > = None;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"PATTERN LIMIT SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zPattern = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        if Tcl_GetWideIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut iLimit,
        ) != 0
        {
            return TCL_ERROR;
        }
        pScript = *objv.offset(3 as ::core::ffi::c_int as isize);
        Tcl_GetStringFromObj(pScript, &raw mut nScript);
        if nScript > 0 as ::core::ffi::c_int {
            p = sqlite3_malloc(
                ::core::mem::size_of::<TclQuotaCallback>() as ::core::ffi::c_int,
            ) as *mut TclQuotaCallback;
            if p.is_null() {
                Tcl_SetResult(
                    interp,
                    b"SQLITE_NOMEM\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    TCL_STATIC,
                );
                return TCL_OK;
            }
            memset(
                p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<TclQuotaCallback>() as size_t,
            );
            (*p).interp = interp;
            (*pScript).refCount += 1;
            (*p).pScript = pScript;
            xDestroy = Some(
                tclCallbackDestructor
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
            xCallback = Some(
                tclQuotaCallback
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut sqlite3_int64,
                        sqlite3_int64,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut sqlite3_int64,
                        sqlite3_int64,
                        *mut ::core::ffi::c_void,
                    ) -> (),
                >;
        } else {
            p = ::core::ptr::null_mut::<TclQuotaCallback>();
            xDestroy = None;
            xCallback = None;
        }
        rc = sqlite3_quota_set(
            zPattern,
            iLimit as sqlite3_int64,
            xCallback,
            p as *mut ::core::ffi::c_void,
            xDestroy,
        );
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_file(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zFilename: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zFilename = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        rc = sqlite3_quota_file(zFilename);
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_dump(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pResult: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pGroupTerm: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pFileTerm: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pGroup: *mut quotaGroup = ::core::ptr::null_mut::<quotaGroup>();
        let mut pFile: *mut quotaFile = ::core::ptr::null_mut::<quotaFile>();
        pResult = Tcl_NewObj();
        quotaEnter();
        pGroup = gQuota.pGroup;
        while !pGroup.is_null() {
            pGroupTerm = Tcl_NewObj();
            Tcl_ListObjAppendElement(
                interp,
                pGroupTerm,
                Tcl_NewStringObj((*pGroup).zPattern, -1 as ::core::ffi::c_int),
            );
            Tcl_ListObjAppendElement(
                interp,
                pGroupTerm,
                Tcl_NewWideIntObj((*pGroup).iLimit as Tcl_WideInt),
            );
            Tcl_ListObjAppendElement(
                interp,
                pGroupTerm,
                Tcl_NewWideIntObj((*pGroup).iSize as Tcl_WideInt),
            );
            pFile = (*pGroup).pFiles;
            while !pFile.is_null() {
                let mut i: ::core::ffi::c_int = 0;
                let mut zTemp: [::core::ffi::c_char; 1000] = [0; 1000];
                pFileTerm = Tcl_NewObj();
                sqlite3_snprintf(
                    ::core::mem::size_of::<[::core::ffi::c_char; 1000]>()
                        as ::core::ffi::c_int,
                    &raw mut zTemp as *mut ::core::ffi::c_char,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pFile).zFilename,
                );
                i = 0 as ::core::ffi::c_int;
                while zTemp[i as usize] != 0 {
                    if zTemp[i as usize] as ::core::ffi::c_int == '\\' as i32 {
                        zTemp[i as usize] = '/' as i32 as ::core::ffi::c_char;
                    }
                    i += 1;
                }
                Tcl_ListObjAppendElement(
                    interp,
                    pFileTerm,
                    Tcl_NewStringObj(
                        &raw mut zTemp as *mut ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    ),
                );
                Tcl_ListObjAppendElement(
                    interp,
                    pFileTerm,
                    Tcl_NewWideIntObj((*pFile).iSize as Tcl_WideInt),
                );
                Tcl_ListObjAppendElement(
                    interp,
                    pFileTerm,
                    Tcl_NewWideIntObj((*pFile).nRef as Tcl_WideInt),
                );
                Tcl_ListObjAppendElement(
                    interp,
                    pFileTerm,
                    Tcl_NewWideIntObj((*pFile).deleteOnClose as Tcl_WideInt),
                );
                Tcl_ListObjAppendElement(interp, pGroupTerm, pFileTerm);
                pFile = (*pFile).pNext;
            }
            Tcl_ListObjAppendElement(interp, pResult, pGroupTerm);
            pGroup = (*pGroup).pNext;
        }
        quotaLeave();
        Tcl_SetObjResult(interp, pResult);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_fopen(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zFilename: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zMode: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut zReturn: [::core::ffi::c_char; 50] = [0; 50];
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"FILENAME MODE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zFilename = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        zMode = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        p = sqlite3_quota_fopen(zFilename, zMode);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 50]>() as ::core::ffi::c_int,
            &raw mut zReturn as *mut ::core::ffi::c_char,
            b"%p\0".as_ptr() as *const ::core::ffi::c_char,
            p,
        );
        Tcl_SetResult(
            interp,
            &raw mut zReturn as *mut ::core::ffi::c_char,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<Tcl_FreeProc>,
            >(1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_fread(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut zBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut sz: ::core::ffi::c_int = 0;
        let mut nElem: ::core::ffi::c_int = 0;
        let mut got: size_t = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE SIZE NELEM\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut sz,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut nElem,
        ) != 0
        {
            return TCL_ERROR;
        }
        zBuf = sqlite3_malloc(sz * nElem + 1 as ::core::ffi::c_int)
            as *mut ::core::ffi::c_char;
        if zBuf.is_null() {
            Tcl_SetResult(
                interp,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                TCL_STATIC,
            );
            return TCL_ERROR;
        }
        got = sqlite3_quota_fread(
            zBuf as *mut ::core::ffi::c_void,
            sz as size_t,
            nElem as size_t,
            p,
        );
        *zBuf.offset(got.wrapping_mul(sz as size_t) as isize) = 0 as ::core::ffi::c_char;
        Tcl_SetResult(
            interp,
            zBuf,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<Tcl_FreeProc>,
            >(1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_free(zBuf as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_fwrite(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut zBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut sz: ::core::ffi::c_int = 0;
        let mut nElem: ::core::ffi::c_int = 0;
        let mut got: size_t = 0;
        if objc != 5 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE SIZE NELEM CONTENT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut sz,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut nElem,
        ) != 0
        {
            return TCL_ERROR;
        }
        zBuf = Tcl_GetString(*objv.offset(4 as ::core::ffi::c_int as isize));
        got = sqlite3_quota_fwrite(
            zBuf as *const ::core::ffi::c_void,
            sz as size_t,
            nElem as size_t,
            p,
        );
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(got as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_fclose(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        rc = sqlite3_quota_fclose(p);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_fflush(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut doSync: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE ?HARDSYNC?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        if objc == 3 as ::core::ffi::c_int {
            if Tcl_GetBooleanFromObj(
                interp,
                *objv.offset(2 as ::core::ffi::c_int as isize),
                &raw mut doSync,
            ) != 0
            {
                return TCL_ERROR;
            }
        }
        rc = sqlite3_quota_fflush(p, doSync);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_fseek(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut ofst: ::core::ffi::c_int = 0;
        let mut zWhence: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut whence: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE OFFSET WHENCE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut ofst,
        ) != 0
        {
            return TCL_ERROR;
        }
        zWhence = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        if strcmp(zWhence, b"SEEK_SET\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            whence = SEEK_SET;
        } else if strcmp(zWhence, b"SEEK_CUR\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            whence = SEEK_CUR;
        } else if strcmp(zWhence, b"SEEK_END\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            whence = SEEK_END;
        } else {
            Tcl_AppendResult(
                interp,
                b"WHENCE should be SEEK_SET, SEEK_CUR, or SEEK_END\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        rc = sqlite3_quota_fseek(p, ofst as ::core::ffi::c_long, whence);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_rewind(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        sqlite3_quota_rewind(p);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_ftell(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut x: sqlite3_int64 = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        x = sqlite3_quota_ftell(p) as sqlite3_int64;
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(x as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_ftruncate(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut x: sqlite3_int64 = 0;
        let mut w: Tcl_WideInt = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE SIZE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        if Tcl_GetWideIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut w,
        ) != 0
        {
            return TCL_ERROR;
        }
        x = w;
        rc = sqlite3_quota_ftruncate(p, x);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_file_size(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut x: sqlite3_int64 = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        x = sqlite3_quota_file_size(p);
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(x as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_file_truesize(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut x: sqlite3_int64 = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        x = sqlite3_quota_file_truesize(p);
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(x as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_file_mtime(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut t: time_t = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        t = 0 as time_t;
        sqlite3_quota_file_mtime(p, &raw mut t);
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(t as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_remove(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zFilename: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zFilename = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        rc = sqlite3_quota_remove(zFilename);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_glob(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zPattern: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zText: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"PATTERN TEXT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zPattern = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        zText = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        rc = quotaStrglob(zPattern, zText);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_file_available(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut x: sqlite3_int64 = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        x = sqlite3_quota_file_available(p) as sqlite3_int64;
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(x as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_quota_ferror(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut quota_FILE = ::core::ptr::null_mut::<quota_FILE>();
        let mut x: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut quota_FILE;
        x = sqlite3_quota_ferror(p);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(x));
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitequota_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aCmd: [C2Rust_Unnamed_3; 21] = unsafe {
            [
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_initialize\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_initialize
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_shutdown\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_shutdown
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_set\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_set
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_file\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_file
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_dump\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_dump
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_fopen\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_fopen
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_fread\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_fread
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_fwrite\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_fwrite
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_fclose\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_fclose
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_fflush\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_fflush
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_fseek\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_fseek
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_rewind\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_rewind
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_ftell\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_ftell
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_ftruncate\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_ftruncate
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_file_size\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_file_size
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_file_truesize\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_file_truesize
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_file_mtime\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_file_mtime
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_remove\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_remove
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_glob\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_glob
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_file_available\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_file_available
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_quota_ferror\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_quota_ferror
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
            ]
        };
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_3; 21]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_3>() as usize)
        {
            Tcl_CreateObjCommand(
                interp,
                aCmd[i as usize].zName,
                aCmd[i as usize].xProc,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            i += 1;
        }
        return TCL_OK;
    }
}
