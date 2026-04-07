use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Tcl_Command_;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
        p: *const sqlite3_module,
        pClientData: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_declare_vtab(
        _: *mut sqlite3,
        zSQL: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_GetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        infoPtr: *mut Tcl_CmdInfo,
    ) -> ::core::ffi::c_int;
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
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: ::core::ffi::c_int,
    pub zErrMsg: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xConnect: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xBestIndex: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut sqlite3_index_info,
        ) -> ::core::ffi::c_int,
    >,
    pub xDisconnect: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut *mut sqlite3_vtab_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xFilter: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xEof: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xColumn: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xUpdate: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xBegin: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xSync: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xCommit: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xRollback: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xFindFunction: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *mut Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xRename: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSavepoint: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xRelease: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xRollbackTo: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xShadowName: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub xIntegrity: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_info {
    pub nConstraint: ::core::ffi::c_int,
    pub aConstraint: *mut sqlite3_index_constraint,
    pub nOrderBy: ::core::ffi::c_int,
    pub aOrderBy: *mut sqlite3_index_orderby,
    pub aConstraintUsage: *mut sqlite3_index_constraint_usage,
    pub idxNum: ::core::ffi::c_int,
    pub idxStr: *mut ::core::ffi::c_char,
    pub needToFreeIdxStr: ::core::ffi::c_int,
    pub orderByConsumed: ::core::ffi::c_int,
    pub estimatedCost: ::core::ffi::c_double,
    pub estimatedRows: sqlite3_int64,
    pub idxFlags: ::core::ffi::c_int,
    pub colUsed: sqlite3_uint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint_usage {
    pub argvIndex: ::core::ffi::c_int,
    pub omit: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_orderby {
    pub iColumn: ::core::ffi::c_int,
    pub desc: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint {
    pub iColumn: ::core::ffi::c_int,
    pub op: ::core::ffi::c_uchar,
    pub usable: ::core::ffi::c_uchar,
    pub iTermOffset: ::core::ffi::c_int,
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfslogVfs {
    pub base: sqlite3_vfs,
    pub pVfs: *mut sqlite3_vfs,
    pub iNextFileId: ::core::ffi::c_int,
    pub pLog: *mut sqlite3_file,
    pub iOffset: sqlite3_int64,
    pub nBuf: ::core::ffi::c_int,
    pub aBuf: [::core::ffi::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfslogFile {
    pub base: sqlite3_file,
    pub pReal: *mut sqlite3_file,
    pub pVfslog: *mut sqlite3_vfs,
    pub iFileId: ::core::ffi::c_int,
}
pub type __time_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __suseconds_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfslogVtab {
    pub base: sqlite3_vtab,
    pub pFd: *mut sqlite3_file,
    pub nByte: sqlite3_int64,
    pub zFile: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfslogCsr {
    pub base: sqlite3_vtab_cursor,
    pub iRowid: sqlite3_int64,
    pub iOffset: sqlite3_int64,
    pub zTransient: *mut ::core::ffi::c_char,
    pub nFile: ::core::ffi::c_int,
    pub azFile: *mut *mut ::core::ffi::c_char,
    pub aBuf: [::core::ffi::c_uchar; 1024],
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
pub type Tcl_CmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;
pub type Tcl_NamespaceDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Namespace {
    pub name: *mut ::core::ffi::c_char,
    pub fullName: *mut ::core::ffi::c_char,
    pub clientData: ClientData,
    pub deleteProc: Option<Tcl_NamespaceDeleteProc>,
    pub parentPtr: *mut Tcl_Namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_CmdInfo {
    pub isNativeObjectProc: ::core::ffi::c_int,
    pub objProc: Option<Tcl_ObjCmdProc>,
    pub objClientData: ClientData,
    pub proc: Option<Tcl_CmdProc>,
    pub clientData: ClientData,
    pub deleteProc: Option<Tcl_CmdDeleteProc>,
    pub deleteData: ClientData,
    pub namespacePtr: *mut Tcl_Namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqliteDb {
    pub db: *mut sqlite3,
}
pub const VL_REGISTER: VL_enum = 3;
pub const VL_NEW: VL_enum = 2;
pub const VL_FINALIZE: VL_enum = 1;
pub const VL_ANNOTATE: VL_enum = 0;
pub type VL_enum = ::core::ffi::c_uint;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SUPER_JOURNAL: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_VFSNAME: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const INST_MAX_PATHNAME: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const OS_ACCESS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OS_CHECKRESERVEDLOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OS_CLOSE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OS_CURRENTTIME: ::core::ffi::c_int = 4;
pub const OS_DELETE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OS_DEVCHAR: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const OS_FILECONTROL: ::core::ffi::c_int = 7;
pub const OS_FILESIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OS_FULLPATHNAME: ::core::ffi::c_int = 9;
pub const OS_LOCK: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const OS_OPEN: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const OS_RANDOMNESS: ::core::ffi::c_int = 13;
pub const OS_READ: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const OS_SECTORSIZE: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const OS_SLEEP: ::core::ffi::c_int = 16;
pub const OS_SYNC: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const OS_TRUNCATE: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const OS_UNLOCK: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const OS_WRITE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const OS_SHMUNMAP: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const OS_SHMMAP: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const OS_SHMLOCK: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const OS_SHMBARRIER: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const OS_ANNOTATE: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
static mut vfslog_vfs: sqlite3_vfs = unsafe {
    sqlite3_vfs {
        iVersion: 1 as ::core::ffi::c_int,
        szOsFile: ::core::mem::size_of::<VfslogFile>() as ::core::ffi::c_int,
        mxPathname: INST_MAX_PATHNAME,
        pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
        zName: ::core::ptr::null::<::core::ffi::c_char>(),
        pAppData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        xOpen: Some(
            vfslogOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xDelete: Some(
            vfslogDelete
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xAccess: Some(
            vfslogAccess
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFullPathname: Some(
            vfslogFullPathname
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xDlOpen: Some(
            vfslogDlOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_void,
        ),
        xDlError: Some(
            vfslogDlError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> (),
        ),
        xDlSym: Some(
            vfslogDlSym
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> Option<unsafe extern "C" fn() -> ()>,
        ),
        xDlClose: Some(
            vfslogDlClose
                as unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
        ),
        xRandomness: Some(
            vfslogRandomness
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSleep: Some(
            vfslogSleep
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCurrentTime: Some(
            vfslogCurrentTime
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
        ),
        xGetLastError: Some(
            vfslogGetLastError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xCurrentTimeInt64: Some(
            vfslogCurrentTimeInt64
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
static mut vfslog_io_methods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 2 as ::core::ffi::c_int,
        xClose: Some(
            vfslogClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            vfslogRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            vfslogWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            vfslogTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            vfslogSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            vfslogFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            vfslogLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            vfslogUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            vfslogCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            vfslogFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            vfslogSectorSize
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            vfslogDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: Some(
            vfslogShmMap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xShmLock: Some(
            vfslogShmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShmBarrier: Some(
            vfslogShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
        ),
        xShmUnmap: Some(
            vfslogShmUnmap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFetch: None,
        xUnfetch: None,
    }
};
unsafe extern "C" fn vfslog_time() -> sqlite3_uint64 {
    unsafe {
        let mut sTime: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        gettimeofday(&raw mut sTime, ::core::ptr::null_mut::<::core::ffi::c_void>());
        return (sTime.tv_usec as sqlite3_uint64)
            .wrapping_add(
                (sTime.tv_sec as sqlite3_uint64).wrapping_mul(1000000 as sqlite3_uint64),
            );
    }
}
unsafe extern "C" fn vfslogClose(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut t: sqlite3_uint64 = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        if !(*(*p).pReal).pMethods.is_null() {
            rc = (*(*(*p).pReal).pMethods)
                .xClose
                .expect("non-null function pointer")((*p).pReal);
        }
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_CLOSE,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogRead(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xRead
            .expect(
                "non-null function pointer",
            )((*p).pReal, zBuf, iAmt, iOfst as sqlite3_int64);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_READ,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            iAmt,
            iOfst as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogWrite(
    mut pFile: *mut sqlite3_file,
    mut z: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xWrite
            .expect(
                "non-null function pointer",
            )((*p).pReal, z, iAmt, iOfst as sqlite3_int64);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_WRITE,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            iAmt,
            iOfst as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogTruncate(
    mut pFile: *mut sqlite3_file,
    mut size: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xTruncate
            .expect("non-null function pointer")((*p).pReal, size as sqlite3_int64);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_TRUNCATE,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            0 as ::core::ffi::c_int,
            size as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogSync(
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xSync
            .expect("non-null function pointer")((*p).pReal, flags);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_SYNC,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            flags,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogFileSize(
    mut pFile: *mut sqlite3_file,
    mut pSize: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xFileSize
            .expect(
                "non-null function pointer",
            )((*p).pReal, pSize as *mut sqlite3_int64);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_FILESIZE,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            0 as ::core::ffi::c_int,
            *pSize as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogLock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xLock
            .expect("non-null function pointer")((*p).pReal, eLock);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_LOCK,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            eLock,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogUnlock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xUnlock
            .expect("non-null function pointer")((*p).pReal, eLock);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_UNLOCK,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            eLock,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogCheckReservedLock(
    mut pFile: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xCheckReservedLock
            .expect("non-null function pointer")((*p).pReal, pResOut);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_CHECKRESERVEDLOCK,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            *pResOut,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogFileControl(
    mut pFile: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        let mut rc: ::core::ffi::c_int = (*(*(*p).pReal).pMethods)
            .xFileControl
            .expect("non-null function pointer")((*p).pReal, op, pArg);
        if op == SQLITE_FCNTL_VFSNAME && rc == SQLITE_OK {
            let ref mut c2rust_fresh0 = *(pArg as *mut *mut ::core::ffi::c_char);
            *c2rust_fresh0 = sqlite3_mprintf(
                b"vfslog/%z\0".as_ptr() as *const ::core::ffi::c_char,
                *(pArg as *mut *mut ::core::ffi::c_char),
            );
        }
        return rc;
    }
}
unsafe extern "C" fn vfslogSectorSize(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xSectorSize
            .expect("non-null function pointer")((*p).pReal);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_SECTORSIZE,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogDeviceCharacteristics(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xDeviceCharacteristics
            .expect("non-null function pointer")((*p).pReal);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_DEVCHAR,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogShmLock(
    mut pFile: *mut sqlite3_file,
    mut ofst: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xShmLock
            .expect("non-null function pointer")((*p).pReal, ofst, n, flags);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_SHMLOCK,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogShmMap(
    mut pFile: *mut sqlite3_file,
    mut iRegion: ::core::ffi::c_int,
    mut szRegion: ::core::ffi::c_int,
    mut isWrite: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xShmMap
            .expect(
                "non-null function pointer",
            )((*p).pReal, iRegion, szRegion, isWrite, pp);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_SHMMAP,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogShmBarrier(mut pFile: *mut sqlite3_file) {
    unsafe {
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        (*(*(*p).pReal).pMethods)
            .xShmBarrier
            .expect("non-null function pointer")((*p).pReal);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_SHMBARRIER,
            (*p).iFileId,
            t as sqlite3_int64,
            SQLITE_OK,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn vfslogShmUnmap(
    mut pFile: *mut sqlite3_file,
    mut deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        t = vfslog_time();
        rc = (*(*(*p).pReal).pMethods)
            .xShmUnmap
            .expect("non-null function pointer")((*p).pReal, deleteFlag);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            (*p).pVfslog,
            OS_SHMUNMAP,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        return rc;
    }
}
unsafe extern "C" fn vfslogOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        let mut p: *mut VfslogFile = pFile as *mut VfslogFile;
        let mut pLog: *mut VfslogVfs = pVfs as *mut VfslogVfs;
        (*pFile).pMethods = &raw mut vfslog_io_methods;
        (*p).pReal = p.offset(1 as ::core::ffi::c_int as isize) as *mut VfslogFile
            as *mut sqlite3_file;
        (*p).pVfslog = pVfs;
        (*pLog).iNextFileId += 1;
        (*p).iFileId = (*pLog).iNextFileId;
        t = vfslog_time();
        rc = (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xOpen
            .expect(
                "non-null function pointer",
            )(
            (*(pVfs as *mut VfslogVfs)).pVfs,
            zName as sqlite3_filename,
            (*p).pReal,
            flags,
            pOutFlags,
        );
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            pVfs,
            OS_OPEN,
            (*p).iFileId,
            t as sqlite3_int64,
            rc,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        vfslog_string(pVfs, zName);
        return rc;
    }
}
unsafe extern "C" fn vfslogDelete(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        t = vfslog_time();
        rc = (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xDelete
            .expect(
                "non-null function pointer",
            )((*(pVfs as *mut VfslogVfs)).pVfs, zPath, dirSync);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            pVfs,
            OS_DELETE,
            0 as ::core::ffi::c_int,
            t as sqlite3_int64,
            rc,
            dirSync,
            0 as ::core::ffi::c_int,
        );
        vfslog_string(pVfs, zPath);
        return rc;
    }
}
unsafe extern "C" fn vfslogAccess(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut t: sqlite3_uint64 = 0;
        t = vfslog_time();
        rc = (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xAccess
            .expect(
                "non-null function pointer",
            )((*(pVfs as *mut VfslogVfs)).pVfs, zPath, flags, pResOut);
        t = vfslog_time().wrapping_sub(t);
        vfslog_call(
            pVfs,
            OS_ACCESS,
            0 as ::core::ffi::c_int,
            t as sqlite3_int64,
            rc,
            flags,
            *pResOut,
        );
        vfslog_string(pVfs, zPath);
        return rc;
    }
}
unsafe extern "C" fn vfslogFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xFullPathname
            .expect(
                "non-null function pointer",
            )((*(pVfs as *mut VfslogVfs)).pVfs, zPath, nOut, zOut);
    }
}
unsafe extern "C" fn vfslogDlOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xDlOpen
            .expect(
                "non-null function pointer",
            )((*(pVfs as *mut VfslogVfs)).pVfs, zPath);
    }
}
unsafe extern "C" fn vfslogDlError(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zErrMsg: *mut ::core::ffi::c_char,
) {
    unsafe {
        (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xDlError
            .expect(
                "non-null function pointer",
            )((*(pVfs as *mut VfslogVfs)).pVfs, nByte, zErrMsg);
    }
}
unsafe extern "C" fn vfslogDlSym(
    mut pVfs: *mut sqlite3_vfs,
    mut p: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        return (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xDlSym
            .expect(
                "non-null function pointer",
            )((*(pVfs as *mut VfslogVfs)).pVfs, p, zSym);
    }
}
unsafe extern "C" fn vfslogDlClose(
    mut pVfs: *mut sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xDlClose
            .expect(
                "non-null function pointer",
            )((*(pVfs as *mut VfslogVfs)).pVfs, pHandle);
    }
}
unsafe extern "C" fn vfslogRandomness(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xRandomness
            .expect(
                "non-null function pointer",
            )((*(pVfs as *mut VfslogVfs)).pVfs, nByte, zBufOut);
    }
}
unsafe extern "C" fn vfslogSleep(
    mut pVfs: *mut sqlite3_vfs,
    mut nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xSleep
            .expect(
                "non-null function pointer",
            )((*(pVfs as *mut VfslogVfs)).pVfs, nMicro);
    }
}
unsafe extern "C" fn vfslogCurrentTime(
    mut pVfs: *mut sqlite3_vfs,
    mut pTimeOut: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xCurrentTime
            .expect(
                "non-null function pointer",
            )((*(pVfs as *mut VfslogVfs)).pVfs, pTimeOut);
    }
}
unsafe extern "C" fn vfslogGetLastError(
    mut pVfs: *mut sqlite3_vfs,
    mut a: ::core::ffi::c_int,
    mut b: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xGetLastError
            .expect("non-null function pointer")((*(pVfs as *mut VfslogVfs)).pVfs, a, b);
    }
}
unsafe extern "C" fn vfslogCurrentTimeInt64(
    mut pVfs: *mut sqlite3_vfs,
    mut p: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*(pVfs as *mut VfslogVfs)).pVfs)
            .xCurrentTimeInt64
            .expect("non-null function pointer")((*(pVfs as *mut VfslogVfs)).pVfs, p);
    }
}
unsafe extern "C" fn vfslog_flush(mut p: *mut VfslogVfs) {
    unsafe {
        unsafe extern "C" {
            static mut sqlite3_io_error_pending: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_io_error_persist: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_diskfull_pending: ::core::ffi::c_int;
        }
        let mut pending: ::core::ffi::c_int = sqlite3_io_error_pending;
        let mut persist: ::core::ffi::c_int = sqlite3_io_error_persist;
        let mut diskfull: ::core::ffi::c_int = sqlite3_diskfull_pending;
        sqlite3_io_error_pending = 0 as ::core::ffi::c_int;
        sqlite3_io_error_persist = 0 as ::core::ffi::c_int;
        sqlite3_diskfull_pending = 0 as ::core::ffi::c_int;
        if (*p).nBuf != 0 {
            (*(*(*p).pLog).pMethods)
                .xWrite
                .expect(
                    "non-null function pointer",
                )(
                (*p).pLog,
                &raw mut (*p).aBuf as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                (*p).nBuf,
                (*p).iOffset,
            );
            (*p).iOffset += (*p).nBuf as ::core::ffi::c_longlong;
            (*p).nBuf = 0 as ::core::ffi::c_int;
        }
        sqlite3_io_error_pending = pending;
        sqlite3_io_error_persist = persist;
        sqlite3_diskfull_pending = diskfull;
    }
}
unsafe extern "C" fn put32bits(
    mut p: *mut ::core::ffi::c_uchar,
    mut v: ::core::ffi::c_uint,
) {
    unsafe {
        *p.offset(0 as ::core::ffi::c_int as isize) = (v >> 24 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        *p.offset(1 as ::core::ffi::c_int as isize) = (v >> 16 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        *p.offset(2 as ::core::ffi::c_int as isize) = (v >> 8 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        *p.offset(3 as ::core::ffi::c_int as isize) = v as ::core::ffi::c_uchar;
    }
}
unsafe extern "C" fn vfslog_call(
    mut pVfs: *mut sqlite3_vfs,
    mut eEvent: ::core::ffi::c_int,
    mut iFileid: ::core::ffi::c_int,
    mut nClick: sqlite3_int64,
    mut return_code: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut VfslogVfs = pVfs as *mut VfslogVfs;
        let mut zRec: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        if (24 as ::core::ffi::c_int + (*p).nBuf) as usize
            > ::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as usize
        {
            vfslog_flush(p);
        }
        zRec = (&raw mut (*p).aBuf as *mut ::core::ffi::c_char)
            .offset((*p).nBuf as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar;
        put32bits(
            zRec.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar,
            eEvent as ::core::ffi::c_uint,
        );
        put32bits(
            zRec.offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar,
            iFileid as ::core::ffi::c_uint,
        );
        put32bits(
            zRec.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar,
            (nClick as ::core::ffi::c_longlong & 0xffff as ::core::ffi::c_longlong)
                as ::core::ffi::c_uint,
        );
        put32bits(
            zRec.offset(12 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar,
            return_code as ::core::ffi::c_uint,
        );
        put32bits(
            zRec.offset(16 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar,
            size as ::core::ffi::c_uint,
        );
        put32bits(
            zRec.offset(20 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar,
            offset as ::core::ffi::c_uint,
        );
        (*p).nBuf += 24 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn vfslog_string(
    mut pVfs: *mut sqlite3_vfs,
    mut zStr: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut p: *mut VfslogVfs = pVfs as *mut VfslogVfs;
        let mut zRec: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut nStr: ::core::ffi::c_int = if !zStr.is_null() {
            strlen(zStr) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        if (4 as ::core::ffi::c_int + nStr + (*p).nBuf) as usize
            > ::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as usize
        {
            vfslog_flush(p);
        }
        zRec = (&raw mut (*p).aBuf as *mut ::core::ffi::c_char)
            .offset((*p).nBuf as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_uchar;
        put32bits(
            zRec.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar,
            nStr as ::core::ffi::c_uint,
        );
        if !zStr.is_null() {
            memcpy(
                zRec.offset(4 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                zStr as *const ::core::ffi::c_void,
                nStr as size_t,
            );
        }
        (*p).nBuf += 4 as ::core::ffi::c_int + nStr;
    }
}
unsafe extern "C" fn vfslog_finalize(mut p: *mut VfslogVfs) {
    unsafe {
        if !(*(*p).pLog).pMethods.is_null() {
            vfslog_flush(p);
            (*(*(*p).pLog).pMethods)
                .xClose
                .expect("non-null function pointer")((*p).pLog);
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vfslog_finalize(
    mut zVfs: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        pVfs = sqlite3_vfs_find(zVfs);
        if pVfs.is_null()
            || (*pVfs).xOpen
                != Some(
                    vfslogOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                )
        {
            return SQLITE_ERROR;
        }
        sqlite3_vfs_unregister(pVfs);
        vfslog_finalize(pVfs as *mut VfslogVfs);
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vfslog_new(
    mut zVfs: *const ::core::ffi::c_char,
    mut zParentVfs: *const ::core::ffi::c_char,
    mut zLog: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut VfslogVfs = ::core::ptr::null_mut::<VfslogVfs>();
        let mut pParent: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut nByte: ::core::ffi::c_int = 0;
        let mut flags: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut zFile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nVfs: ::core::ffi::c_int = 0;
        pParent = sqlite3_vfs_find(zParentVfs);
        if pParent.is_null() {
            return SQLITE_ERROR;
        }
        nVfs = strlen(zVfs) as ::core::ffi::c_int;
        nByte = (::core::mem::size_of::<VfslogVfs>() as usize)
            .wrapping_add((*pParent).szOsFile as usize)
            .wrapping_add(nVfs as usize)
            .wrapping_add(1 as usize)
            .wrapping_add((*pParent).mxPathname as usize)
            .wrapping_add(1 as usize) as ::core::ffi::c_int;
        p = sqlite3_malloc(nByte) as *mut VfslogVfs;
        memset(p as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, nByte as size_t);
        (*p).pVfs = pParent;
        (*p).pLog = p.offset(1 as ::core::ffi::c_int as isize) as *mut VfslogVfs
            as *mut sqlite3_file;
        memcpy(
            &raw mut (*p).base as *mut ::core::ffi::c_void,
            &raw mut vfslog_vfs as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sqlite3_vfs>() as size_t,
        );
        (*p).base.zName = ((*p).pLog as *mut ::core::ffi::c_char)
            .offset((*pParent).szOsFile as isize) as *mut ::core::ffi::c_char;
        (*p).base.szOsFile += (*pParent).szOsFile;
        memcpy(
            (*p).base.zName as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            zVfs as *const ::core::ffi::c_void,
            nVfs as size_t,
        );
        zFile = (*p).base.zName.offset((nVfs + 1 as ::core::ffi::c_int) as isize)
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        (*pParent)
            .xFullPathname
            .expect(
                "non-null function pointer",
            )(pParent, zLog, (*pParent).mxPathname, zFile);
        flags = SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_SUPER_JOURNAL;
        (*pParent)
            .xDelete
            .expect(
                "non-null function pointer",
            )(pParent, zFile, 0 as ::core::ffi::c_int);
        rc = (*pParent)
            .xOpen
            .expect(
                "non-null function pointer",
            )(pParent, zFile as sqlite3_filename, (*p).pLog, flags, &raw mut flags);
        if rc == SQLITE_OK {
            memcpy(
                &raw mut (*p).aBuf as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                b"sqlite_ostrace1.....\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                20 as size_t,
            );
            (*p).iOffset = 0 as sqlite3_int64;
            (*p).nBuf = 20 as ::core::ffi::c_int;
            rc = sqlite3_vfs_register(p as *mut sqlite3_vfs, 1 as ::core::ffi::c_int);
        }
        if rc != 0 {
            vfslog_finalize(p);
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vfslog_annotate(
    mut zVfs: *const ::core::ffi::c_char,
    mut zMsg: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        pVfs = sqlite3_vfs_find(zVfs);
        if pVfs.is_null()
            || (*pVfs).xOpen
                != Some(
                    vfslogOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                )
        {
            return SQLITE_ERROR;
        }
        vfslog_call(
            pVfs,
            OS_ANNOTATE,
            0 as ::core::ffi::c_int,
            0 as sqlite3_int64,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        vfslog_string(pVfs, zMsg);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn vfslog_eventname(
    mut eEvent: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut zEvent: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        match eEvent {
            OS_CLOSE => {
                zEvent = b"xClose\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_READ => {
                zEvent = b"xRead\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_WRITE => {
                zEvent = b"xWrite\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_TRUNCATE => {
                zEvent = b"xTruncate\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_SYNC => {
                zEvent = b"xSync\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_FILESIZE => {
                zEvent = b"xFilesize\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_LOCK => {
                zEvent = b"xLock\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_UNLOCK => {
                zEvent = b"xUnlock\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_CHECKRESERVEDLOCK => {
                zEvent = b"xCheckResLock\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_FILECONTROL => {
                zEvent = b"xFileControl\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_SECTORSIZE => {
                zEvent = b"xSectorSize\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_DEVCHAR => {
                zEvent = b"xDeviceChar\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_OPEN => {
                zEvent = b"xOpen\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_DELETE => {
                zEvent = b"xDelete\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_ACCESS => {
                zEvent = b"xAccess\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_FULLPATHNAME => {
                zEvent = b"xFullPathname\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_RANDOMNESS => {
                zEvent = b"xRandomness\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_SLEEP => {
                zEvent = b"xSleep\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_CURRENTTIME => {
                zEvent = b"xCurrentTime\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_SHMUNMAP => {
                zEvent = b"xShmUnmap\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_SHMLOCK => {
                zEvent = b"xShmLock\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_SHMBARRIER => {
                zEvent = b"xShmBarrier\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_SHMMAP => {
                zEvent = b"xShmMap\0".as_ptr() as *const ::core::ffi::c_char;
            }
            OS_ANNOTATE => {
                zEvent = b"annotation\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        return zEvent;
    }
}
unsafe extern "C" fn get32bits(mut p: *mut ::core::ffi::c_uchar) -> ::core::ffi::c_uint {
    unsafe {
        return (((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 24 as ::core::ffi::c_int)
            + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int)
            + ((*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int)
            + *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
    }
}
unsafe extern "C" fn dequote(mut z: *mut ::core::ffi::c_char) {
    unsafe {
        let mut quote: ::core::ffi::c_char = 0;
        quote = *z.offset(0 as ::core::ffi::c_int as isize);
        if quote as ::core::ffi::c_int == '[' as i32
            || quote as ::core::ffi::c_int == '\'' as i32
            || quote as ::core::ffi::c_int == '"' as i32
            || quote as ::core::ffi::c_int == '`' as i32
        {
            let mut iIn: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if quote as ::core::ffi::c_int == '[' as i32 {
                quote = ']' as i32 as ::core::ffi::c_char;
            }
            while *z.offset(iIn as isize) != 0 {
                if *z.offset(iIn as isize) as ::core::ffi::c_int
                    == quote as ::core::ffi::c_int
                {
                    if *z.offset((iIn + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int != quote as ::core::ffi::c_int
                    {
                        break;
                    }
                    let c2rust_fresh1 = iOut;
                    iOut = iOut + 1;
                    *z.offset(c2rust_fresh1 as isize) = quote;
                    iIn += 2 as ::core::ffi::c_int;
                } else {
                    let c2rust_fresh2 = iIn;
                    iIn = iIn + 1;
                    let c2rust_fresh3 = iOut;
                    iOut = iOut + 1;
                    *z.offset(c2rust_fresh3 as isize) = *z
                        .offset(c2rust_fresh2 as isize);
                }
            }
            *z.offset(iOut as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
    }
}
unsafe extern "C" fn vlogConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut flags: ::core::ffi::c_int = 0;
        let mut p: *mut VfslogVtab = ::core::ptr::null_mut::<VfslogVtab>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut nByte: ::core::ffi::c_int = 0;
        let mut zFile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        *ppVtab = ::core::ptr::null_mut::<sqlite3_vtab>();
        pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        nByte = (::core::mem::size_of::<VfslogVtab>() as usize)
            .wrapping_add((*pVfs).szOsFile as usize)
            .wrapping_add((*pVfs).mxPathname as usize) as ::core::ffi::c_int;
        p = sqlite3_malloc(nByte) as *mut VfslogVtab;
        if p.is_null() {
            return SQLITE_NOMEM;
        }
        memset(p as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, nByte as size_t);
        (*p).pFd = p.offset(1 as ::core::ffi::c_int as isize) as *mut VfslogVtab
            as *mut sqlite3_file;
        (*p).zFile = ((*p).pFd as *mut ::core::ffi::c_char)
            .offset((*pVfs).szOsFile as isize) as *mut ::core::ffi::c_char;
        zFile = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(3 as ::core::ffi::c_int as isize),
        );
        if zFile.is_null() {
            sqlite3_free(p as *mut ::core::ffi::c_void);
            return SQLITE_NOMEM;
        }
        dequote(zFile);
        (*pVfs)
            .xFullPathname
            .expect(
                "non-null function pointer",
            )(pVfs, zFile, (*pVfs).mxPathname, (*p).zFile);
        sqlite3_free(zFile as *mut ::core::ffi::c_void);
        flags = SQLITE_OPEN_READWRITE | SQLITE_OPEN_SUPER_JOURNAL;
        rc = (*pVfs)
            .xOpen
            .expect(
                "non-null function pointer",
            )(pVfs, (*p).zFile as sqlite3_filename, (*p).pFd, flags, &raw mut flags);
        if rc == SQLITE_OK {
            (*(*(*p).pFd).pMethods)
                .xFileSize
                .expect("non-null function pointer")((*p).pFd, &raw mut (*p).nByte);
            sqlite3_declare_vtab(
                db,
                b"CREATE TABLE xxx(event, file, click, rc, size, offset)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            *ppVtab = &raw mut (*p).base;
        } else {
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn vlogBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        (*pIdxInfo).estimatedCost = 10.0f64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn vlogDisconnect(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut VfslogVtab = pVtab as *mut VfslogVtab;
        if !(*(*p).pFd).pMethods.is_null() {
            (*(*(*p).pFd).pMethods).xClose.expect("non-null function pointer")((*p).pFd);
            (*(*p).pFd).pMethods = ::core::ptr::null::<sqlite3_io_methods>();
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn vlogOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut VfslogCsr = ::core::ptr::null_mut::<VfslogCsr>();
        pCsr = sqlite3_malloc(::core::mem::size_of::<VfslogCsr>() as ::core::ffi::c_int)
            as *mut VfslogCsr;
        if pCsr.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pCsr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VfslogCsr>() as size_t,
        );
        *ppCursor = &raw mut (*pCsr).base;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn vlogClose(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut VfslogCsr = pCursor as *mut VfslogCsr;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nFile {
            sqlite3_free(*(*p).azFile.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        sqlite3_free((*p).azFile as *mut ::core::ffi::c_void);
        sqlite3_free((*p).zTransient as *mut ::core::ffi::c_void);
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn vlogNext(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut VfslogCsr = pCursor as *mut VfslogCsr;
        let mut p: *mut VfslogVtab = (*pCursor).pVtab as *mut VfslogVtab;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nRead: ::core::ffi::c_int = 0;
        sqlite3_free((*pCsr).zTransient as *mut ::core::ffi::c_void);
        (*pCsr).zTransient = ::core::ptr::null_mut::<::core::ffi::c_char>();
        nRead = 24 as ::core::ffi::c_int;
        if (*pCsr).iOffset as ::core::ffi::c_longlong + nRead as ::core::ffi::c_longlong
            <= (*p).nByte
        {
            let mut eEvent: ::core::ffi::c_int = 0;
            rc = (*(*(*p).pFd).pMethods)
                .xRead
                .expect(
                    "non-null function pointer",
                )(
                (*p).pFd,
                &raw mut (*pCsr).aBuf as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                nRead,
                (*pCsr).iOffset,
            );
            eEvent = get32bits(&raw mut (*pCsr).aBuf as *mut ::core::ffi::c_uchar)
                as ::core::ffi::c_int;
            if rc == SQLITE_OK
                && (eEvent == OS_OPEN || eEvent == OS_DELETE || eEvent == OS_ACCESS)
            {
                let mut buf: [::core::ffi::c_char; 4] = [0; 4];
                rc = (*(*(*p).pFd).pMethods)
                    .xRead
                    .expect(
                        "non-null function pointer",
                    )(
                    (*p).pFd,
                    &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    4 as ::core::ffi::c_int,
                    (*pCsr).iOffset + nRead as sqlite3_int64,
                );
                nRead += 4 as ::core::ffi::c_int;
                if rc == SQLITE_OK {
                    let mut nStr: ::core::ffi::c_int = get32bits(
                        &raw mut buf as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_uchar,
                    ) as ::core::ffi::c_int;
                    let mut zStr: *mut ::core::ffi::c_char = sqlite3_malloc(
                        nStr + 1 as ::core::ffi::c_int,
                    ) as *mut ::core::ffi::c_char;
                    rc = (*(*(*p).pFd).pMethods)
                        .xRead
                        .expect(
                            "non-null function pointer",
                        )(
                        (*p).pFd,
                        zStr as *mut ::core::ffi::c_void,
                        nStr,
                        (*pCsr).iOffset + nRead as sqlite3_int64,
                    );
                    *zStr.offset(nStr as isize) = '\0' as i32 as ::core::ffi::c_char;
                    nRead += nStr;
                    if eEvent == OS_OPEN {
                        let mut iFileid: ::core::ffi::c_int = get32bits(
                            (&raw mut (*pCsr).aBuf as *mut ::core::ffi::c_uchar)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_uchar,
                        ) as ::core::ffi::c_int;
                        if iFileid >= (*pCsr).nFile {
                            let mut nNew: ::core::ffi::c_int = (::core::mem::size_of::<
                                *mut ::core::ffi::c_char,
                            >() as usize)
                                .wrapping_mul((iFileid + 1 as ::core::ffi::c_int) as usize)
                                as ::core::ffi::c_int;
                            (*pCsr).azFile = sqlite3_realloc(
                                (*pCsr).azFile as *mut ::core::ffi::c_void,
                                nNew,
                            ) as *mut *mut ::core::ffi::c_char;
                            nNew = (nNew as ::core::ffi::c_ulong)
                                .wrapping_sub(
                                    (::core::mem::size_of::<*mut ::core::ffi::c_char>()
                                        as usize)
                                        .wrapping_mul((*pCsr).nFile as usize)
                                        as ::core::ffi::c_ulong,
                                ) as ::core::ffi::c_int as ::core::ffi::c_int;
                            memset(
                                (*pCsr).azFile.offset((*pCsr).nFile as isize)
                                    as *mut *mut ::core::ffi::c_char
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                nNew as size_t,
                            );
                            (*pCsr).nFile = iFileid + 1 as ::core::ffi::c_int;
                        }
                        sqlite3_free(
                            *(*pCsr).azFile.offset(iFileid as isize)
                                as *mut ::core::ffi::c_void,
                        );
                        let ref mut c2rust_fresh4 = *(*pCsr)
                            .azFile
                            .offset(iFileid as isize);
                        *c2rust_fresh4 = zStr;
                    } else {
                        (*pCsr).zTransient = zStr;
                    }
                }
            }
        }
        (*pCsr).iRowid += 1 as ::core::ffi::c_longlong;
        (*pCsr).iOffset += nRead as ::core::ffi::c_longlong;
        return rc;
    }
}
unsafe extern "C" fn vlogEof(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut VfslogCsr = pCursor as *mut VfslogCsr;
        let mut p: *mut VfslogVtab = (*pCursor).pVtab as *mut VfslogVtab;
        return ((*pCsr).iOffset >= (*p).nByte) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn vlogFilter(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut VfslogCsr = pCursor as *mut VfslogCsr;
        (*pCsr).iRowid = 0 as sqlite3_int64;
        (*pCsr).iOffset = 20 as sqlite3_int64;
        return vlogNext(pCursor);
    }
}
unsafe extern "C" fn vlogColumn(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut val: ::core::ffi::c_uint = 0;
        let mut pCsr: *mut VfslogCsr = pCursor as *mut VfslogCsr;
        val = get32bits(
            (&raw mut (*pCsr).aBuf as *mut ::core::ffi::c_uchar)
                .offset((4 as ::core::ffi::c_int * i) as isize)
                as *mut ::core::ffi::c_uchar,
        );
        match i {
            0 => {
                sqlite3_result_text(
                    ctx,
                    vfslog_eventname(val as ::core::ffi::c_int),
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            1 => {
                let mut zStr: *mut ::core::ffi::c_char = (*pCsr).zTransient;
                if val != 0 as ::core::ffi::c_uint
                    && val < (*pCsr).nFile as ::core::ffi::c_uint
                {
                    zStr = *(*pCsr).azFile.offset(val as isize);
                }
                sqlite3_result_text(
                    ctx,
                    zStr,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
            _ => {
                sqlite3_result_int(ctx, val as ::core::ffi::c_int);
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn vlogRowid(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut VfslogCsr = pCursor as *mut VfslogCsr;
        *pRowid = (*pCsr).iRowid as sqlite_int64;
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vfslog_register(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        static mut vfslog_module: sqlite3_module = unsafe {
            sqlite3_module {
                iVersion: 0 as ::core::ffi::c_int,
                xCreate: Some(
                    vlogConnect
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const *const ::core::ffi::c_char,
                            *mut *mut sqlite3_vtab,
                            *mut *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                xConnect: Some(
                    vlogConnect
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const *const ::core::ffi::c_char,
                            *mut *mut sqlite3_vtab,
                            *mut *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                xBestIndex: Some(
                    vlogBestIndex
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut sqlite3_index_info,
                        ) -> ::core::ffi::c_int,
                ),
                xDisconnect: Some(
                    vlogDisconnect
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xDestroy: Some(
                    vlogDisconnect
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xOpen: Some(
                    vlogOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xClose: Some(
                    vlogClose
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xFilter: Some(
                    vlogFilter
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> ::core::ffi::c_int,
                ),
                xNext: Some(
                    vlogNext
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xEof: Some(
                    vlogEof
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xColumn: Some(
                    vlogColumn
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xRowid: Some(
                    vlogRowid
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            *mut sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xUpdate: None,
                xBegin: None,
                xSync: None,
                xCommit: None,
                xRollback: None,
                xFindFunction: None,
                xRename: None,
                xSavepoint: None,
                xRelease: None,
                xRollbackTo: None,
                xShadowName: None,
                xIntegrity: None,
            }
        };
        sqlite3_create_module(
            db,
            b"vfslog\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut vfslog_module,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return SQLITE_OK;
    }
}
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn test_vfslog(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut cmdInfo: Tcl_CmdInfo = Tcl_CmdInfo {
            isNativeObjectProc: 0,
            objProc: None,
            objClientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            proc: None,
            clientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            deleteProc: None,
            deleteData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            namespacePtr: ::core::ptr::null_mut::<Tcl_Namespace>(),
        };
        let mut rc: ::core::ffi::c_int = SQLITE_ERROR;
        static mut strs: [*const ::core::ffi::c_char; 5] = [
            b"annotate\0".as_ptr() as *const ::core::ffi::c_char,
            b"finalize\0".as_ptr() as *const ::core::ffi::c_char,
            b"new\0".as_ptr() as *const ::core::ffi::c_char,
            b"register\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut iSub: ::core::ffi::c_int = 0;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUB-COMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut strs as *mut *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as ::core::ffi::c_int,
            b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iSub,
        ) != 0
        {
            return TCL_ERROR;
        }
        match iSub as VL_enum as ::core::ffi::c_uint {
            0 => {
                let mut zVfs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                if objc != 4 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        3 as ::core::ffi::c_int,
                        objv,
                        b"VFS\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                zVfs = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
                zMsg = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
                rc = sqlite3_vfslog_annotate(zVfs, zMsg);
                if rc != SQLITE_OK {
                    Tcl_AppendResult(
                        interp,
                        b"failed\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
            }
            1 => {
                let mut zVfs_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"VFS\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                zVfs_0 = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
                rc = sqlite3_vfslog_finalize(zVfs_0);
                if rc != SQLITE_OK {
                    Tcl_AppendResult(
                        interp,
                        b"failed\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
            }
            2 => {
                let mut zVfs_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zParent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zLog: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                if objc != 5 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"VFS PARENT LOGFILE\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                zVfs_1 = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
                zParent = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
                zLog = Tcl_GetString(*objv.offset(4 as ::core::ffi::c_int as isize));
                if *zParent as ::core::ffi::c_int == '\0' as i32 {
                    zParent = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                rc = sqlite3_vfslog_new(zVfs_1, zParent, zLog);
                if rc != SQLITE_OK {
                    Tcl_AppendResult(
                        interp,
                        b"failed\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
            }
            3 => {
                let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"DB\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                zDb = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
                if Tcl_GetCommandInfo(interp, zDb, &raw mut cmdInfo) != 0 {
                    db = (*(cmdInfo.objClientData as *mut SqliteDb)).db;
                    rc = sqlite3_vfslog_register(db);
                }
                if rc != SQLITE_OK {
                    Tcl_AppendResult(
                        interp,
                        b"bad sqlite3 handle: \0".as_ptr() as *const ::core::ffi::c_char,
                        zDb,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    );
                    return TCL_ERROR;
                }
            }
            _ => {}
        }
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SqlitetestOsinst_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_CreateObjCommand(
            interp,
            b"vfslog\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                test_vfslog
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
