unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sqlite3_libversion() -> *const ::core::ffi::c_char;
    fn sqlite3_sourceid() -> *const ::core::ffi::c_char;
    fn sqlite3_libversion_number() -> ::core::ffi::c_int;
    fn sqlite3_compileoption_used(
        zOptName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_close(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_initialize() -> ::core::ffi::c_int;
    fn sqlite3_config(_: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3_db_config(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_vmprintf(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_trace(
        _: *mut sqlite3,
        xTrace: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
        _: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_open_v2(
        filename: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
        flags: ::core::ffi::c_int,
        zVfs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const ::core::ffi::c_char;
    fn sqlite3_expanded_sql(pStmt: *mut sqlite3_stmt) -> *mut ::core::ffi::c_char;
    fn sqlite3_bind_double(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: sqlite3_uint64,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        encoding: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_blob(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    fn sqlite3_column_bytes(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_type(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_db_release_memory(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        op: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_test_control(op: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3_status(
        op: ::core::ffi::c_int,
        pCurrent: *mut ::core::ffi::c_int,
        pHighwater: *mut ::core::ffi::c_int,
        resetFlag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_db_status(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        pCur: *mut ::core::ffi::c_int,
        pHiwtr: *mut ::core::ffi::c_int,
        resetFlg: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strglob(
        zGlob: *const ::core::ffi::c_char,
        zStr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_rtree_geometry_callback(
        db: *mut sqlite3,
        zGeom: *const ::core::ffi::c_char,
        xGeom: Option<
            unsafe extern "C" fn(
                *mut sqlite3_rtree_geometry,
                ::core::ffi::c_int,
                *mut sqlite3_rtree_dbl,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pContext: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type va_list = __builtin_va_list;
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
pub struct sqlite3_rtree_geometry {
    pub pContext: *mut ::core::ffi::c_void,
    pub nParam: ::core::ffi::c_int,
    pub aParam: *mut sqlite3_rtree_dbl,
    pub pUser: *mut ::core::ffi::c_void,
    pub xDelUser: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub type sqlite3_rtree_dbl = ::core::ffi::c_double;
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
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
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2Rust_Unnamed = 8;
pub const _ISpunct: C2Rust_Unnamed = 4;
pub const _IScntrl: C2Rust_Unnamed = 2;
pub const _ISblank: C2Rust_Unnamed = 1;
pub const _ISgraph: C2Rust_Unnamed = 32768;
pub const _ISprint: C2Rust_Unnamed = 16384;
pub const _ISspace: C2Rust_Unnamed = 8192;
pub const _ISxdigit: C2Rust_Unnamed = 4096;
pub const _ISdigit: C2Rust_Unnamed = 2048;
pub const _ISalpha: C2Rust_Unnamed = 1024;
pub const _ISlower: C2Rust_Unnamed = 512;
pub const _ISupper: C2Rust_Unnamed = 256;
pub type u64_0 = sqlite3_uint64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashContext {
    pub isInit: ::core::ffi::c_uchar,
    pub i: ::core::ffi::c_uchar,
    pub j: ::core::ffi::c_uchar,
    pub s: [::core::ffi::c_uchar; 256],
    pub r: [::core::ffi::c_uchar; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Global {
    pub db: *mut sqlite3,
    pub zDbName: *const ::core::ffi::c_char,
    pub zVfs: *const ::core::ffi::c_char,
    pub pStmt: *mut sqlite3_stmt,
    pub iStart: sqlite3_int64,
    pub iTotal: sqlite3_int64,
    pub bWithoutRowid: ::core::ffi::c_int,
    pub bReprepare: ::core::ffi::c_int,
    pub bSqlOnly: ::core::ffi::c_int,
    pub bExplain: ::core::ffi::c_int,
    pub bVerify: ::core::ffi::c_int,
    pub bMemShrink: ::core::ffi::c_int,
    pub eTemp: ::core::ffi::c_int,
    pub szTest: ::core::ffi::c_int,
    pub szBase: ::core::ffi::c_int,
    pub nRepeat: ::core::ffi::c_int,
    pub doCheckpoint: ::core::ffi::c_int,
    pub nReserve: ::core::ffi::c_int,
    pub stmtScanStatus: ::core::ffi::c_int,
    pub doBigTransactions: ::core::ffi::c_int,
    pub zWR: *const ::core::ffi::c_char,
    pub zNN: *const ::core::ffi::c_char,
    pub zPK: *const ::core::ffi::c_char,
    pub x: ::core::ffi::c_uint,
    pub y: ::core::ffi::c_uint,
    pub nResByte: u64_0,
    pub nResult: ::core::ffi::c_int,
    pub zResult: [::core::ffi::c_char; 3000],
    pub pScript: *mut FILE,
    pub hashFile: *mut FILE,
    pub hash: HashContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub zSuffix: *mut ::core::ffi::c_char,
    pub iMult: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub zPattern: *const ::core::ffi::c_char,
    pub zDesc: *const ::core::ffi::c_char,
}
static mut zHelp: [::core::ffi::c_char; 2897] = unsafe {
    ::core::mem::transmute::<
        [u8; 2897],
        [::core::ffi::c_char; 2897],
    >(
        *b"Usage: %s [--options] DATABASE\nOptions:\n  --autovacuum        Enable AUTOVACUUM mode\n  --big-transactions  Add BEGIN/END around all large tests\n  --cachesize N       Set PRAGMA cache_size=N. Note: N is pages, not bytes\n  --checkpoint        Run PRAGMA wal_checkpoint after each test case\n  --exclusive         Enable locking_mode=EXCLUSIVE\n  --explain           Like --sqlonly but with added EXPLAIN keywords\n  --fullfsync         Enable fullfsync=TRUE\n  --hard-heap-limit N The hard limit on the maximum heap size\n  --heap SZ MIN       Memory allocator uses SZ bytes & min allocation MIN\n  --incrvacuum        Enable incremenatal vacuum mode\n  --journal M         Set the journal_mode to M\n  --key KEY           Set the encryption key to KEY\n  --lookaside N SZ    Configure lookaside for N slots of SZ bytes each\n  --memdb             Use an in-memory database\n  --mmap SZ           MMAP the first SZ bytes of the database file\n  --multithread       Set multithreaded mode\n  --nomemstat         Disable memory statistics\n  --nomutex           Open db with SQLITE_OPEN_NOMUTEX\n  --nosync            Set PRAGMA synchronous=OFF\n  --notnull           Add NOT NULL constraints to table columns\n  --output FILE       Store SQL output in FILE\n  --pagesize N        Set the page size to N\n  --pcache N SZ       Configure N pages of pagecache each of size SZ bytes\n  --primarykey        Use PRIMARY KEY instead of UNIQUE where appropriate\n  --repeat N          Repeat each SELECT N times (default: 1)\n  --reprepare         Reprepare each statement upon every invocation\n  --reserve N         Reserve N bytes on each database page\n  --script FILE       Write an SQL script for the test into FILE\n  --serialized        Set serialized threading mode\n  --singlethread      Set single-threaded mode - disables all mutexing\n  --sqlonly           No-op.  Only show the SQL that would have been run.\n  --shrink-memory     Invoke sqlite3_db_release_memory() frequently.\n  --size N            Relative test size.  Default=100\n  --soft-heap-limit N The soft limit on the maximum heap size\n  --strict            Use STRICT table where appropriate\n  --stats             Show statistics at the end\n  --stmtscanstatus    Activate SQLITE_DBCONFIG_STMT_SCANSTATUS\n  --temp N            N from 0 to 9.  0: no temp table. 9: all temp tables\n  --testset T         Run test-set T (main, cte, rtree, orm, fp, json,\n                      star, app, debug).  Can be a comma-separated list\n                      of values, with /SCALE suffixes or macro \"mix1\"\n  --trace             Turn on SQL tracing\n  --threads N         Use up to N threads for sorting\n  --utf16be           Set text encoding to UTF-16BE\n  --utf16le           Set text encoding to UTF-16LE\n  --verify            Run additional verification steps\n  --vfs NAME          Use the given (preinstalled) VFS\n  --without-rowid     Use WITHOUT ROWID where appropriate\n\0",
    )
};
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_NOMUTEX: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_RESERVE_BYTES: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_SINGLETHREAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_MULTITHREAD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_SERIALIZED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_PAGECACHE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_HEAP: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_MEMSTATUS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_LOOKASIDE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_LOOKASIDE: ::core::ffi::c_int = 1001 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_STMT_SCANSTATUS: ::core::ffi::c_int = 1018
    as ::core::ffi::c_int;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_TESTCTRL_PRNG_SEED: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const SQLITE_STATUS_MEMORY_USED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_STATUS_PAGECACHE_OVERFLOW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_STATUS_MALLOC_SIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATUS_PAGECACHE_SIZE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_STATUS_MALLOC_COUNT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_LOOKASIDE_USED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_USED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_SCHEMA_USED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_STMT_USED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_LOOKASIDE_HIT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE: ::core::ffi::c_int = 5
    as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL: ::core::ffi::c_int = 6
    as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_HIT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_MISS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_WRITE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
static mut g: Global = Global {
    db: ::core::ptr::null::<sqlite3>() as *mut sqlite3,
    zDbName: ::core::ptr::null::<::core::ffi::c_char>(),
    zVfs: ::core::ptr::null::<::core::ffi::c_char>(),
    pStmt: ::core::ptr::null::<sqlite3_stmt>() as *mut sqlite3_stmt,
    iStart: 0,
    iTotal: 0,
    bWithoutRowid: 0,
    bReprepare: 0,
    bSqlOnly: 0,
    bExplain: 0,
    bVerify: 0,
    bMemShrink: 0,
    eTemp: 0,
    szTest: 0,
    szBase: 0,
    nRepeat: 0,
    doCheckpoint: 0,
    nReserve: 0,
    stmtScanStatus: 0,
    doBigTransactions: 0,
    zWR: ::core::ptr::null::<::core::ffi::c_char>(),
    zNN: ::core::ptr::null::<::core::ffi::c_char>(),
    zPK: ::core::ptr::null::<::core::ffi::c_char>(),
    x: 0,
    y: 0,
    nResByte: 0,
    nResult: 0,
    zResult: [0; 3000],
    pScript: ::core::ptr::null::<FILE>() as *mut FILE,
    hashFile: ::core::ptr::null::<FILE>() as *mut FILE,
    hash: HashContext {
        isInit: 0,
        i: 0,
        j: 0,
        s: [0; 256],
        r: [0; 32],
    },
};
unsafe extern "C" fn isTemp(mut N: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    unsafe {
        return if g.eTemp >= N {
            b" TEMP\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        };
    }
}
unsafe extern "C" fn fatal_error(
    mut zMsg: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut ap: ::core::ffi::VaList<'_>;
        ap = c2rust_args.clone();
        vfprintf(stderr, zMsg, ap);
        exit(1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn HashInit() {
    unsafe {
        let mut k: ::core::ffi::c_uint = 0;
        g.hash.i = 0 as ::core::ffi::c_uchar;
        g.hash.j = 0 as ::core::ffi::c_uchar;
        k = 0 as ::core::ffi::c_uint;
        while k < 256 as ::core::ffi::c_uint {
            g.hash.s[k as usize] = k as ::core::ffi::c_uchar;
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn HashUpdate(
    mut aData: *const ::core::ffi::c_uchar,
    mut nData: ::core::ffi::c_uint,
) {
    unsafe {
        let mut t: ::core::ffi::c_uchar = 0;
        let mut i: ::core::ffi::c_uchar = g.hash.i;
        let mut j: ::core::ffi::c_uchar = g.hash.j;
        let mut k: ::core::ffi::c_uint = 0;
        if !g.hashFile.is_null() {
            fwrite(
                aData as *const ::core::ffi::c_void,
                1 as size_t,
                nData as size_t,
                g.hashFile,
            );
        }
        k = 0 as ::core::ffi::c_uint;
        while k < nData {
            j = (j as ::core::ffi::c_int
                + (g.hash.s[i as usize] as ::core::ffi::c_int
                    + *aData.offset(k as isize) as ::core::ffi::c_int))
                as ::core::ffi::c_uchar;
            t = g.hash.s[j as usize];
            g.hash.s[j as usize] = g.hash.s[i as usize];
            g.hash.s[i as usize] = t;
            i = i.wrapping_add(1);
            k = k.wrapping_add(1);
        }
        g.hash.i = i;
        g.hash.j = j;
    }
}
unsafe extern "C" fn HashFinal() {
    unsafe {
        let mut k: ::core::ffi::c_uint = 0;
        let mut t: ::core::ffi::c_uchar = 0;
        let mut i: ::core::ffi::c_uchar = 0;
        let mut j: ::core::ffi::c_uchar = 0;
        i = g.hash.i;
        j = g.hash.j;
        k = 0 as ::core::ffi::c_uint;
        while k < 32 as ::core::ffi::c_uint {
            i = i.wrapping_add(1);
            t = g.hash.s[i as usize];
            j = (j as ::core::ffi::c_int + t as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            g.hash.s[i as usize] = g.hash.s[j as usize];
            g.hash.s[j as usize] = t;
            t = (t as ::core::ffi::c_int + g.hash.s[i as usize] as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            g.hash.r[k as usize] = g.hash.s[t as usize];
            k = k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn hexDigitValue(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32
        {
            return c as ::core::ffi::c_int - '0' as i32;
        }
        if c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'f' as i32
        {
            return c as ::core::ffi::c_int - 'a' as i32 + 10 as ::core::ffi::c_int;
        }
        if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32
        {
            return c as ::core::ffi::c_int - 'A' as i32 + 10 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn integerValue(
    mut zArg: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: sqlite3_int64 = 0 as sqlite3_int64;
        static mut aMult: [C2Rust_Unnamed_0; 9] = [
            C2Rust_Unnamed_0 {
                zSuffix: b"KiB\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iMult: 1024 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_0 {
                zSuffix: b"MiB\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iMult: 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_0 {
                zSuffix: b"GiB\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iMult: 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int
                    * 1024 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_0 {
                zSuffix: b"KB\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iMult: 1000 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_0 {
                zSuffix: b"MB\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iMult: 1000000 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_0 {
                zSuffix: b"GB\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iMult: 1000000000 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_0 {
                zSuffix: b"K\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iMult: 1000 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_0 {
                zSuffix: b"M\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iMult: 1000000 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_0 {
                zSuffix: b"G\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                iMult: 1000000000 as ::core::ffi::c_int,
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let mut isNeg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if *zArg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32
        {
            isNeg = 1 as ::core::ffi::c_int;
            zArg = zArg.offset(1);
        } else if *zArg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '+' as i32
        {
            zArg = zArg.offset(1);
        }
        if *zArg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '0' as i32
            && *zArg.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'x' as i32
        {
            let mut x: ::core::ffi::c_int = 0;
            zArg = zArg.offset(2 as ::core::ffi::c_int as isize);
            loop {
                x = hexDigitValue(*zArg.offset(0 as ::core::ffi::c_int as isize));
                if !(x >= 0 as ::core::ffi::c_int) {
                    break;
                }
                v = (((v as ::core::ffi::c_longlong) << 4 as ::core::ffi::c_int)
                    + x as ::core::ffi::c_longlong) as sqlite3_int64;
                zArg = zArg.offset(1);
            }
        } else {
            while *(*__ctype_b_loc())
                .offset(
                    *zArg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        as isize,
                ) as ::core::ffi::c_int
                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
            {
                v = (v as ::core::ffi::c_longlong * 10 as ::core::ffi::c_longlong
                    + *zArg.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_longlong
                    - '0' as i32 as ::core::ffi::c_longlong) as sqlite3_int64;
                zArg = zArg.offset(1);
            }
        }
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_0; 9]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_0>() as usize)
        {
            if sqlite3_stricmp(aMult[i as usize].zSuffix, zArg)
                == 0 as ::core::ffi::c_int
            {
                v *= aMult[i as usize].iMult as ::core::ffi::c_longlong;
                break;
            } else {
                i += 1;
            }
        }
        if v > 0x7fffffff as ::core::ffi::c_longlong {
            fatal_error(
                b"parameter too large - max 2147483648\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        return (if isNeg != 0 {
            -(v as ::core::ffi::c_longlong)
        } else {
            v as ::core::ffi::c_longlong
        }) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_timestamp() -> sqlite3_int64 {
    unsafe {
        static mut clockVfs: *mut sqlite3_vfs = ::core::ptr::null::<sqlite3_vfs>()
            as *mut sqlite3_vfs;
        let mut t: sqlite3_int64 = 0;
        if clockVfs.is_null() {
            clockVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        }
        if (*clockVfs).iVersion >= 2 as ::core::ffi::c_int
            && (*clockVfs).xCurrentTimeInt64.is_some()
        {
            (*clockVfs)
                .xCurrentTimeInt64
                .expect("non-null function pointer")(clockVfs, &raw mut t);
        } else {
            let mut r: ::core::ffi::c_double = 0.;
            (*clockVfs)
                .xCurrentTime
                .expect("non-null function pointer")(clockVfs, &raw mut r);
            t = (r * 86400000.0f64) as sqlite3_int64;
        }
        return t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_random() -> ::core::ffi::c_uint {
    unsafe {
        g.x = g.x >> 1 as ::core::ffi::c_int
            ^ (1 as ::core::ffi::c_uint).wrapping_add(!(g.x & 1 as ::core::ffi::c_uint))
                & 0xd0000001 as ::core::ffi::c_uint;
        g.y = g
            .y
            .wrapping_mul(1103515245 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_add(12345 as ::core::ffi::c_uint);
        return g.x ^ g.y;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn swizzle(
    mut in_0: ::core::ffi::c_uint,
    mut limit: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut out: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while limit != 0 {
            out = out << 1 as ::core::ffi::c_int | in_0 & 1 as ::core::ffi::c_uint;
            in_0 >>= 1 as ::core::ffi::c_int;
            limit >>= 1 as ::core::ffi::c_int;
        }
        return out;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn roundup_allones(
    mut limit: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut m: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
        while m < limit {
            m = (m << 1 as ::core::ffi::c_int).wrapping_add(1 as ::core::ffi::c_uint);
        }
        return m;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_numbername(
    mut n: ::core::ffi::c_uint,
    mut zOut: *mut ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        static mut ones: [*const ::core::ffi::c_char; 20] = [
            b"zero\0".as_ptr() as *const ::core::ffi::c_char,
            b"one\0".as_ptr() as *const ::core::ffi::c_char,
            b"two\0".as_ptr() as *const ::core::ffi::c_char,
            b"three\0".as_ptr() as *const ::core::ffi::c_char,
            b"four\0".as_ptr() as *const ::core::ffi::c_char,
            b"five\0".as_ptr() as *const ::core::ffi::c_char,
            b"six\0".as_ptr() as *const ::core::ffi::c_char,
            b"seven\0".as_ptr() as *const ::core::ffi::c_char,
            b"eight\0".as_ptr() as *const ::core::ffi::c_char,
            b"nine\0".as_ptr() as *const ::core::ffi::c_char,
            b"ten\0".as_ptr() as *const ::core::ffi::c_char,
            b"eleven\0".as_ptr() as *const ::core::ffi::c_char,
            b"twelve\0".as_ptr() as *const ::core::ffi::c_char,
            b"thirteen\0".as_ptr() as *const ::core::ffi::c_char,
            b"fourteen\0".as_ptr() as *const ::core::ffi::c_char,
            b"fifteen\0".as_ptr() as *const ::core::ffi::c_char,
            b"sixteen\0".as_ptr() as *const ::core::ffi::c_char,
            b"seventeen\0".as_ptr() as *const ::core::ffi::c_char,
            b"eighteen\0".as_ptr() as *const ::core::ffi::c_char,
            b"nineteen\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        static mut tens: [*const ::core::ffi::c_char; 10] = [
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"ten\0".as_ptr() as *const ::core::ffi::c_char,
            b"twenty\0".as_ptr() as *const ::core::ffi::c_char,
            b"thirty\0".as_ptr() as *const ::core::ffi::c_char,
            b"forty\0".as_ptr() as *const ::core::ffi::c_char,
            b"fifty\0".as_ptr() as *const ::core::ffi::c_char,
            b"sixty\0".as_ptr() as *const ::core::ffi::c_char,
            b"seventy\0".as_ptr() as *const ::core::ffi::c_char,
            b"eighty\0".as_ptr() as *const ::core::ffi::c_char,
            b"ninety\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if n >= 1000000000 as ::core::ffi::c_int as ::core::ffi::c_uint {
            i
                += speedtest1_numbername(
                    n
                        .wrapping_div(
                            1000000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        ),
                    zOut.offset(i as isize),
                    nOut - i,
                );
            sqlite3_snprintf(
                nOut - i,
                zOut.offset(i as isize),
                b" billion\0".as_ptr() as *const ::core::ffi::c_char,
            );
            i += strlen(zOut.offset(i as isize)) as ::core::ffi::c_int;
            n = n.wrapping_rem(1000000000 as ::core::ffi::c_int as ::core::ffi::c_uint);
        }
        if n >= 1000000 as ::core::ffi::c_int as ::core::ffi::c_uint {
            if i != 0 && i < nOut - 1 as ::core::ffi::c_int {
                let c2rust_fresh0 = i;
                i = i + 1;
                *zOut.offset(c2rust_fresh0 as isize) = ' ' as i32 as ::core::ffi::c_char;
            }
            i
                += speedtest1_numbername(
                    n.wrapping_div(1000000 as ::core::ffi::c_int as ::core::ffi::c_uint),
                    zOut.offset(i as isize),
                    nOut - i,
                );
            sqlite3_snprintf(
                nOut - i,
                zOut.offset(i as isize),
                b" million\0".as_ptr() as *const ::core::ffi::c_char,
            );
            i += strlen(zOut.offset(i as isize)) as ::core::ffi::c_int;
            n = n.wrapping_rem(1000000 as ::core::ffi::c_int as ::core::ffi::c_uint);
        }
        if n >= 1000 as ::core::ffi::c_uint {
            if i != 0 && i < nOut - 1 as ::core::ffi::c_int {
                let c2rust_fresh1 = i;
                i = i + 1;
                *zOut.offset(c2rust_fresh1 as isize) = ' ' as i32 as ::core::ffi::c_char;
            }
            i
                += speedtest1_numbername(
                    n.wrapping_div(1000 as ::core::ffi::c_uint),
                    zOut.offset(i as isize),
                    nOut - i,
                );
            sqlite3_snprintf(
                nOut - i,
                zOut.offset(i as isize),
                b" thousand\0".as_ptr() as *const ::core::ffi::c_char,
            );
            i += strlen(zOut.offset(i as isize)) as ::core::ffi::c_int;
            n = n.wrapping_rem(1000 as ::core::ffi::c_uint);
        }
        if n >= 100 as ::core::ffi::c_uint {
            if i != 0 && i < nOut - 1 as ::core::ffi::c_int {
                let c2rust_fresh2 = i;
                i = i + 1;
                *zOut.offset(c2rust_fresh2 as isize) = ' ' as i32 as ::core::ffi::c_char;
            }
            sqlite3_snprintf(
                nOut - i,
                zOut.offset(i as isize),
                b"%s hundred\0".as_ptr() as *const ::core::ffi::c_char,
                ones[n.wrapping_div(100 as ::core::ffi::c_uint) as usize],
            );
            i += strlen(zOut.offset(i as isize)) as ::core::ffi::c_int;
            n = n.wrapping_rem(100 as ::core::ffi::c_uint);
        }
        if n >= 20 as ::core::ffi::c_uint {
            if i != 0 && i < nOut - 1 as ::core::ffi::c_int {
                let c2rust_fresh3 = i;
                i = i + 1;
                *zOut.offset(c2rust_fresh3 as isize) = ' ' as i32 as ::core::ffi::c_char;
            }
            sqlite3_snprintf(
                nOut - i,
                zOut.offset(i as isize),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                tens[n.wrapping_div(10 as ::core::ffi::c_uint) as usize],
            );
            i += strlen(zOut.offset(i as isize)) as ::core::ffi::c_int;
            n = n.wrapping_rem(10 as ::core::ffi::c_uint);
        }
        if n > 0 as ::core::ffi::c_uint {
            if i != 0 && i < nOut - 1 as ::core::ffi::c_int {
                let c2rust_fresh4 = i;
                i = i + 1;
                *zOut.offset(c2rust_fresh4 as isize) = ' ' as i32 as ::core::ffi::c_char;
            }
            sqlite3_snprintf(
                nOut - i,
                zOut.offset(i as isize),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                ones[n as usize],
            );
            i += strlen(zOut.offset(i as isize)) as ::core::ffi::c_int;
        }
        if i == 0 as ::core::ffi::c_int {
            sqlite3_snprintf(
                nOut - i,
                zOut.offset(i as isize),
                b"zero\0".as_ptr() as *const ::core::ffi::c_char,
            );
            i += strlen(zOut.offset(i as isize)) as ::core::ffi::c_int;
        }
        return i;
    }
}
pub const NAMEWIDTH: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
static mut zDots: [::core::ffi::c_char; 72] = unsafe {
    ::core::mem::transmute::<
        [u8; 72],
        [::core::ffi::c_char; 72],
    >(*b".......................................................................\0")
};
static mut iTestNumber: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_begin_test(
    mut iTestNum: ::core::ffi::c_int,
    mut zTestName: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut n: ::core::ffi::c_int = strlen(zTestName) as ::core::ffi::c_int;
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ap: ::core::ffi::VaList<'_>;
        iTestNumber = iTestNum;
        ap = c2rust_args.clone();
        zName = sqlite3_vmprintf(zTestName, ap);
        n = strlen(zName) as ::core::ffi::c_int;
        if n > NAMEWIDTH {
            *zName.offset(NAMEWIDTH as isize) = 0 as ::core::ffi::c_char;
            n = NAMEWIDTH;
        }
        if !g.pScript.is_null() {
            fprintf(
                g.pScript,
                b"-- begin test %d %.*s\n\0".as_ptr() as *const ::core::ffi::c_char,
                iTestNumber,
                n,
                zName,
            );
        }
        if g.bSqlOnly != 0 {
            printf(
                b"/* %4d - %s%.*s */\n\0".as_ptr() as *const ::core::ffi::c_char,
                iTestNum,
                zName,
                NAMEWIDTH - n,
                &raw const zDots as *const ::core::ffi::c_char,
            );
        } else {
            printf(
                b"%4d - %s%.*s \0".as_ptr() as *const ::core::ffi::c_char,
                iTestNum,
                zName,
                NAMEWIDTH - n,
                &raw const zDots as *const ::core::ffi::c_char,
            );
            fflush(stdout);
        }
        sqlite3_free(zName as *mut ::core::ffi::c_void);
        g.nResult = 0 as ::core::ffi::c_int;
        g.iStart = speedtest1_timestamp();
        g.x = 0xad131d0b as ::core::ffi::c_uint;
        g.y = 0x44f9eac8 as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_end_test() {
    unsafe {
        let mut iElapseTime: sqlite3_int64 = speedtest1_timestamp() - g.iStart;
        if g.doCheckpoint != 0 {
            speedtest1_exec(
                b"PRAGMA wal_checkpoint;\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        '_c2rust_label: {
            if iTestNumber > 0 as ::core::ffi::c_int {} else {
                __assert_fail(
                    b"iTestNumber > 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/workspace/c2rust/crust-speedtest1/speedtest1.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    456 as ::core::ffi::c_uint,
                    b"void speedtest1_end_test(void)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if !g.pScript.is_null() {
            fprintf(
                g.pScript,
                b"-- end test %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                iTestNumber,
            );
        }
        if g.bSqlOnly == 0 {
            g.iTotal += iElapseTime as ::core::ffi::c_longlong;
            printf(
                b"%4d.%03ds\n\0".as_ptr() as *const ::core::ffi::c_char,
                (iElapseTime as ::core::ffi::c_longlong
                    / 1000 as ::core::ffi::c_longlong) as ::core::ffi::c_int,
                (iElapseTime as ::core::ffi::c_longlong
                    % 1000 as ::core::ffi::c_longlong) as ::core::ffi::c_int,
            );
        }
        if !g.pStmt.is_null() {
            sqlite3_finalize(g.pStmt);
            g.pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
        iTestNumber = 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_final() {
    unsafe {
        if g.bSqlOnly == 0 {
            printf(
                b"       TOTAL%.*s %4d.%03ds\n\0".as_ptr() as *const ::core::ffi::c_char,
                NAMEWIDTH - 5 as ::core::ffi::c_int,
                &raw const zDots as *const ::core::ffi::c_char,
                (g.iTotal as ::core::ffi::c_longlong / 1000 as ::core::ffi::c_longlong)
                    as ::core::ffi::c_int,
                (g.iTotal as ::core::ffi::c_longlong % 1000 as ::core::ffi::c_longlong)
                    as ::core::ffi::c_int,
            );
        }
        if g.bVerify != 0 {
            let mut i: ::core::ffi::c_int = 0;
            printf(
                b"Verification Hash: %llu \0".as_ptr() as *const ::core::ffi::c_char,
                g.nResByte,
            );
            HashUpdate(
                b"\n\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_uchar,
                1 as ::core::ffi::c_uint,
            );
            HashFinal();
            i = 0 as ::core::ffi::c_int;
            while i < 24 as ::core::ffi::c_int {
                printf(
                    b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                    g.hash.r[i as usize] as ::core::ffi::c_int,
                );
                i += 1;
            }
            if !g.hashFile.is_null() && g.hashFile != stdout {
                fclose(g.hashFile);
            }
            printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
}
unsafe extern "C" fn printSql(mut zSql: *const ::core::ffi::c_char) {
    unsafe {
        let mut n: ::core::ffi::c_int = strlen(zSql) as ::core::ffi::c_int;
        while n > 0 as ::core::ffi::c_int
            && (*zSql.offset((n - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == ';' as i32
                || *(*__ctype_b_loc())
                    .offset(
                        *zSql.offset((n - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                    ) as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0)
        {
            n -= 1;
        }
        if g.bExplain != 0 {
            printf(b"EXPLAIN \0".as_ptr() as *const ::core::ffi::c_char);
        }
        printf(b"%.*s;\n\0".as_ptr() as *const ::core::ffi::c_char, n, zSql);
        if g.bExplain != 0
            && (sqlite3_strglob(
                b"CREATE *\0".as_ptr() as *const ::core::ffi::c_char,
                zSql,
            ) == 0 as ::core::ffi::c_int
                || sqlite3_strglob(
                    b"DROP *\0".as_ptr() as *const ::core::ffi::c_char,
                    zSql,
                ) == 0 as ::core::ffi::c_int
                || sqlite3_strglob(
                    b"ALTER *\0".as_ptr() as *const ::core::ffi::c_char,
                    zSql,
                ) == 0 as ::core::ffi::c_int)
        {
            printf(b"%.*s;\n\0".as_ptr() as *const ::core::ffi::c_char, n, zSql);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_shrink_memory() {
    unsafe {
        if g.bMemShrink != 0 {
            sqlite3_db_release_memory(g.db);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_exec(
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut ap: ::core::ffi::VaList<'_>;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        ap = c2rust_args.clone();
        zSql = sqlite3_vmprintf(zFormat, ap);
        if g.bSqlOnly != 0 {
            printSql(zSql);
        } else {
            let mut zErrMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut rc: ::core::ffi::c_int = 0;
            if !g.pScript.is_null() {
                fprintf(
                    g.pScript,
                    b"%s;\n\0".as_ptr() as *const ::core::ffi::c_char,
                    zSql,
                );
            }
            rc = sqlite3_exec(
                g.db,
                zSql,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                &raw mut zErrMsg,
            );
            if !zErrMsg.is_null() {
                fatal_error(
                    b"SQL error: %s\n%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    zErrMsg,
                    zSql,
                );
            }
            if rc != SQLITE_OK {
                fatal_error(
                    b"exec error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg(g.db),
                );
            }
        }
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        speedtest1_shrink_memory();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_once(
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut ap: ::core::ffi::VaList<'_>;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zResult: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        ap = c2rust_args.clone();
        zSql = sqlite3_vmprintf(zFormat, ap);
        if g.bSqlOnly != 0 {
            printSql(zSql);
        } else {
            let mut rc_0: ::core::ffi::c_int = sqlite3_prepare_v2(
                g.db,
                zSql,
                -1 as ::core::ffi::c_int,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc_0 != 0 {
                fatal_error(
                    b"SQL error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg(g.db),
                );
            }
            if !g.pScript.is_null() {
                let mut z: *mut ::core::ffi::c_char = sqlite3_expanded_sql(pStmt);
                fprintf(g.pScript, b"%s\n\0".as_ptr() as *const ::core::ffi::c_char, z);
                sqlite3_free(z as *mut ::core::ffi::c_void);
            }
            if sqlite3_step(pStmt) == SQLITE_ROW {
                let mut z_0: *const ::core::ffi::c_char = sqlite3_column_text(
                    pStmt,
                    0 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                if !z_0.is_null() {
                    zResult = sqlite3_mprintf(
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        z_0,
                    );
                }
            }
            rc_0 = sqlite3_reset(pStmt);
            if rc_0 != SQLITE_OK {
                fatal_error(
                    b"%s\nError code %d: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_sql(pStmt),
                    rc_0,
                    sqlite3_errmsg(g.db),
                );
            }
            sqlite3_finalize(pStmt);
        }
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        speedtest1_shrink_memory();
        return zResult;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_prepare(
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut ap: ::core::ffi::VaList<'_>;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        ap = c2rust_args.clone();
        zSql = sqlite3_vmprintf(zFormat, ap);
        if g.bSqlOnly != 0 {
            printSql(zSql);
        } else {
            let mut rc: ::core::ffi::c_int = 0;
            if !g.pStmt.is_null() {
                sqlite3_finalize(g.pStmt);
            }
            rc = sqlite3_prepare_v2(
                g.db,
                zSql,
                -1 as ::core::ffi::c_int,
                &raw mut g.pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc != 0 {
                fatal_error(
                    b"SQL error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg(g.db),
                );
            }
        }
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_run() {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut len: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if g.bSqlOnly != 0 {
            return;
        }
        '_c2rust_label: {
            if !g.pStmt.is_null() {} else {
                __assert_fail(
                    b"g.pStmt\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/workspace/c2rust/crust-speedtest1/speedtest1.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    609 as ::core::ffi::c_uint,
                    b"void speedtest1_run(void)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        g.nResult = 0 as ::core::ffi::c_int;
        if !g.pScript.is_null() {
            let mut z: *mut ::core::ffi::c_char = sqlite3_expanded_sql(g.pStmt);
            fprintf(g.pScript, b"%s\n\0".as_ptr() as *const ::core::ffi::c_char, z);
            sqlite3_free(z as *mut ::core::ffi::c_void);
        }
        while sqlite3_step(g.pStmt) == SQLITE_ROW {
            n = sqlite3_column_count(g.pStmt);
            i = 0 as ::core::ffi::c_int;
            while i < n {
                let mut z_0: *const ::core::ffi::c_char = sqlite3_column_text(g.pStmt, i)
                    as *const ::core::ffi::c_char;
                if z_0.is_null() {
                    z_0 = b"nil\0".as_ptr() as *const ::core::ffi::c_char;
                }
                len = strlen(z_0) as ::core::ffi::c_int;
                if g.bVerify != 0 {
                    let mut eType: ::core::ffi::c_int = sqlite3_column_type(g.pStmt, i);
                    let mut zPrefix: [::core::ffi::c_uchar; 2] = [0; 2];
                    zPrefix[0 as ::core::ffi::c_int as usize] = '\n' as i32
                        as ::core::ffi::c_uchar;
                    zPrefix[1 as ::core::ffi::c_int as usize] = ::core::mem::transmute::<
                        [u8; 7],
                        [::core::ffi::c_char; 7],
                    >(*b"-IFTBN\0")[eType as usize] as ::core::ffi::c_uchar;
                    if g.nResByte != 0 {
                        HashUpdate(
                            &raw mut zPrefix as *mut ::core::ffi::c_uchar,
                            2 as ::core::ffi::c_uint,
                        );
                    } else {
                        HashUpdate(
                            (&raw mut zPrefix as *mut ::core::ffi::c_uchar)
                                .offset(1 as ::core::ffi::c_int as isize),
                            1 as ::core::ffi::c_uint,
                        );
                    }
                    if eType == SQLITE_FLOAT {
                        g.nResByte = (g.nResByte as ::core::ffi::c_ulonglong)
                            .wrapping_add(2 as ::core::ffi::c_ulonglong) as u64_0
                            as u64_0;
                    } else if eType == SQLITE_BLOB {
                        let mut nBlob: ::core::ffi::c_int = sqlite3_column_bytes(
                            g.pStmt,
                            i,
                        );
                        let mut iBlob: ::core::ffi::c_int = 0;
                        let mut zChar: [::core::ffi::c_uchar; 2] = [0; 2];
                        let mut aBlob: *const ::core::ffi::c_uchar = sqlite3_column_blob(
                            g.pStmt,
                            i,
                        ) as *const ::core::ffi::c_uchar;
                        iBlob = 0 as ::core::ffi::c_int;
                        while iBlob < nBlob {
                            zChar[0 as ::core::ffi::c_int as usize] = ::core::mem::transmute::<
                                [u8; 17],
                                [::core::ffi::c_char; 17],
                            >(
                                *b"0123456789abcdef\0",
                            )[(*aBlob.offset(iBlob as isize) as ::core::ffi::c_int
                                >> 4 as ::core::ffi::c_int) as usize]
                                as ::core::ffi::c_uchar;
                            zChar[1 as ::core::ffi::c_int as usize] = ::core::mem::transmute::<
                                [u8; 17],
                                [::core::ffi::c_char; 17],
                            >(
                                *b"0123456789abcdef\0",
                            )[(*aBlob.offset(iBlob as isize) as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                as ::core::ffi::c_uchar;
                            HashUpdate(
                                &raw mut zChar as *mut ::core::ffi::c_uchar,
                                2 as ::core::ffi::c_uint,
                            );
                            iBlob += 1;
                        }
                        g.nResByte = (g.nResByte as ::core::ffi::c_ulonglong)
                            .wrapping_add(
                                (nBlob * 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_ulonglong,
                            ) as u64_0 as u64_0;
                    } else {
                        HashUpdate(
                            z_0 as *mut ::core::ffi::c_uchar,
                            len as ::core::ffi::c_uint,
                        );
                        g.nResByte = (g.nResByte as ::core::ffi::c_ulonglong)
                            .wrapping_add(
                                (len + 2 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong,
                            ) as u64_0 as u64_0;
                    }
                }
                if ((g.nResult + len) as usize)
                    < (::core::mem::size_of::<[::core::ffi::c_char; 3000]>() as usize)
                        .wrapping_sub(2 as usize)
                {
                    if g.nResult > 0 as ::core::ffi::c_int {
                        let c2rust_fresh5 = g.nResult;
                        g.nResult = g.nResult + 1;
                        g.zResult[c2rust_fresh5 as usize] = ' ' as i32
                            as ::core::ffi::c_char;
                    }
                    memcpy(
                        (&raw mut g.zResult as *mut ::core::ffi::c_char)
                            .offset(g.nResult as isize) as *mut ::core::ffi::c_void,
                        z_0 as *const ::core::ffi::c_void,
                        (len + 1 as ::core::ffi::c_int) as size_t,
                    );
                    g.nResult += len;
                }
                i += 1;
            }
        }
        if g.bReprepare != 0 {
            let mut pNew: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            sqlite3_prepare_v2(
                g.db,
                sqlite3_sql(g.pStmt),
                -1 as ::core::ffi::c_int,
                &raw mut pNew,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            rc = sqlite3_finalize(g.pStmt);
            if rc != SQLITE_OK {
                fatal_error(
                    b"%s\nError code %d: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_sql(pNew),
                    rc,
                    sqlite3_errmsg(g.db),
                );
            }
            g.pStmt = pNew;
        } else {
            rc = sqlite3_reset(g.pStmt);
            if rc != SQLITE_OK {
                fatal_error(
                    b"%s\nError code %d: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_sql(g.pStmt),
                    rc,
                    sqlite3_errmsg(g.db),
                );
            }
        }
        speedtest1_shrink_memory();
    }
}
unsafe extern "C" fn traceCallback(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut zSql: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut n: ::core::ffi::c_int = strlen(zSql) as ::core::ffi::c_int;
        while n > 0 as ::core::ffi::c_int
            && (*zSql.offset((n - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == ';' as i32
                || *(*__ctype_b_loc())
                    .offset(
                        *zSql.offset((n - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                    ) as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0)
        {
            n -= 1;
        }
        fprintf(stderr, b"%.*s;\n\0".as_ptr() as *const ::core::ffi::c_char, n, zSql);
    }
}
unsafe extern "C" fn randomFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: ::core::ffi::c_int,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_result_int64(context, speedtest1_random() as sqlite3_int64);
    }
}
unsafe extern "C" fn est_square_root(mut x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let mut y0: ::core::ffi::c_int = x / 2 as ::core::ffi::c_int;
        let mut y1: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        n = 0 as ::core::ffi::c_int;
        while y0 > 0 as ::core::ffi::c_int && n < 10 as ::core::ffi::c_int {
            y1 = (y0 + x / y0) / 2 as ::core::ffi::c_int;
            if y1 == y0 {
                break;
            }
            y0 = y1;
            n += 1;
        }
        return y0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testset_main() {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut sz: ::core::ffi::c_int = 0;
        let mut maxb: ::core::ffi::c_int = 0;
        let mut x1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut x2: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zNum: [::core::ffi::c_char; 2000] = [0; 2000];
        n = g.szTest * 500 as ::core::ffi::c_int;
        sz = n;
        zNum[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        maxb = roundup_allones(sz as ::core::ffi::c_uint) as ::core::ffi::c_int;
        speedtest1_begin_test(
            100 as ::core::ffi::c_int,
            b"%d INSERTs into table with no index\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"CREATE%s TABLE z1(a INTEGER %s, b INTEGER %s, c TEXT %s);\0".as_ptr()
                as *const ::core::ffi::c_char,
            isTemp(9 as ::core::ffi::c_int),
            g.zNN,
            g.zNN,
            g.zNN,
        );
        speedtest1_prepare(
            b"INSERT INTO z1 VALUES(?1,?2,?3); --  %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = swizzle(i as ::core::ffi::c_uint, maxb as ::core::ffi::c_uint);
            speedtest1_numbername(
                x1,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            sqlite3_bind_int64(g.pStmt, 1 as ::core::ffi::c_int, x1 as sqlite3_int64);
            sqlite3_bind_int(g.pStmt, 2 as ::core::ffi::c_int, i);
            sqlite3_bind_text(
                g.pStmt,
                3 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = sz;
        speedtest1_begin_test(
            110 as ::core::ffi::c_int,
            b"%d ordered INSERTS with one index/PK\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"CREATE%s TABLE z2(a INTEGER %s %s, b INTEGER %s, c TEXT %s) %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            isTemp(5 as ::core::ffi::c_int),
            g.zNN,
            g.zPK,
            g.zNN,
            g.zNN,
            g.zWR,
        );
        speedtest1_prepare(
            b"INSERT INTO z2 VALUES(?1,?2,?3); -- %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = swizzle(i as ::core::ffi::c_uint, maxb as ::core::ffi::c_uint);
            speedtest1_numbername(
                x1,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, i);
            sqlite3_bind_int64(g.pStmt, 2 as ::core::ffi::c_int, x1 as sqlite3_int64);
            sqlite3_bind_text(
                g.pStmt,
                3 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = sz;
        speedtest1_begin_test(
            120 as ::core::ffi::c_int,
            b"%d unordered INSERTS with one index/PK\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"CREATE%s TABLE t3(a INTEGER %s %s, b INTEGER %s, c TEXT %s) %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            isTemp(3 as ::core::ffi::c_int),
            g.zNN,
            g.zPK,
            g.zNN,
            g.zNN,
            g.zWR,
        );
        speedtest1_prepare(
            b"INSERT INTO t3 VALUES(?1,?2,?3); -- %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = swizzle(i as ::core::ffi::c_uint, maxb as ::core::ffi::c_uint);
            speedtest1_numbername(
                x1,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            sqlite3_bind_int(g.pStmt, 2 as ::core::ffi::c_int, i);
            sqlite3_bind_int64(g.pStmt, 1 as ::core::ffi::c_int, x1 as sqlite3_int64);
            sqlite3_bind_text(
                g.pStmt,
                3 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = 25 as ::core::ffi::c_int;
        speedtest1_begin_test(
            130 as ::core::ffi::c_int,
            b"%d SELECTS, numeric BETWEEN, unindexed\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"SELECT count(*), avg(b), sum(length(c)), group_concat(c) FROM z1\n WHERE b BETWEEN ?1 AND ?2; -- %d times\0"
                .as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            if (i - 1 as ::core::ffi::c_int) % g.nRepeat == 0 as ::core::ffi::c_int {
                x1 = speedtest1_random().wrapping_rem(maxb as ::core::ffi::c_uint);
                x2 = speedtest1_random()
                    .wrapping_rem(10 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (sz / 5000 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                    )
                    .wrapping_add(x1);
            }
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, x1 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 2 as ::core::ffi::c_int, x2 as ::core::ffi::c_int);
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = 10 as ::core::ffi::c_int;
        speedtest1_begin_test(
            140 as ::core::ffi::c_int,
            b"%d SELECTS, LIKE, unindexed\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"SELECT count(*), avg(b), sum(length(c)), group_concat(c) FROM z1\n WHERE c LIKE ?1; -- %d times\0"
                .as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            if (i - 1 as ::core::ffi::c_int) % g.nRepeat == 0 as ::core::ffi::c_int {
                x1 = speedtest1_random().wrapping_rem(maxb as ::core::ffi::c_uint);
                zNum[0 as ::core::ffi::c_int as usize] = '%' as i32
                    as ::core::ffi::c_char;
                len = speedtest1_numbername(
                    i as ::core::ffi::c_uint,
                    (&raw mut zNum as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    (::core::mem::size_of::<[::core::ffi::c_char; 2000]>() as usize)
                        .wrapping_sub(2 as usize) as ::core::ffi::c_int,
                );
                zNum[len as usize] = '%' as i32 as ::core::ffi::c_char;
                zNum[(len + 1 as ::core::ffi::c_int) as usize] = 0
                    as ::core::ffi::c_char;
            }
            sqlite3_bind_text(
                g.pStmt,
                1 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                len + 1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = 10 as ::core::ffi::c_int;
        speedtest1_begin_test(
            142 as ::core::ffi::c_int,
            b"%d SELECTS w/ORDER BY, unindexed\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"SELECT a, b, c FROM z1 WHERE c LIKE ?1\n ORDER BY a; -- %d times\0"
                .as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            if (i - 1 as ::core::ffi::c_int) % g.nRepeat == 0 as ::core::ffi::c_int {
                x1 = speedtest1_random().wrapping_rem(maxb as ::core::ffi::c_uint);
                zNum[0 as ::core::ffi::c_int as usize] = '%' as i32
                    as ::core::ffi::c_char;
                len = speedtest1_numbername(
                    i as ::core::ffi::c_uint,
                    (&raw mut zNum as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    (::core::mem::size_of::<[::core::ffi::c_char; 2000]>() as usize)
                        .wrapping_sub(2 as usize) as ::core::ffi::c_int,
                );
                zNum[len as usize] = '%' as i32 as ::core::ffi::c_char;
                zNum[(len + 1 as ::core::ffi::c_int) as usize] = 0
                    as ::core::ffi::c_char;
            }
            sqlite3_bind_text(
                g.pStmt,
                1 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                len + 1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = 10 as ::core::ffi::c_int;
        speedtest1_begin_test(
            145 as ::core::ffi::c_int,
            b"%d SELECTS w/ORDER BY and LIMIT, unindexed\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"SELECT a, b, c FROM z1 WHERE c LIKE ?1\n ORDER BY a LIMIT 10; -- %d times\0"
                .as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            if (i - 1 as ::core::ffi::c_int) % g.nRepeat == 0 as ::core::ffi::c_int {
                x1 = speedtest1_random().wrapping_rem(maxb as ::core::ffi::c_uint);
                zNum[0 as ::core::ffi::c_int as usize] = '%' as i32
                    as ::core::ffi::c_char;
                len = speedtest1_numbername(
                    i as ::core::ffi::c_uint,
                    (&raw mut zNum as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    (::core::mem::size_of::<[::core::ffi::c_char; 2000]>() as usize)
                        .wrapping_sub(2 as usize) as ::core::ffi::c_int,
                );
                zNum[len as usize] = '%' as i32 as ::core::ffi::c_char;
                zNum[(len + 1 as ::core::ffi::c_int) as usize] = 0
                    as ::core::ffi::c_char;
            }
            sqlite3_bind_text(
                g.pStmt,
                1 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                len + 1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        speedtest1_begin_test(
            150 as ::core::ffi::c_int,
            b"CREATE INDEX five times\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(b"BEGIN;\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"CREATE UNIQUE INDEX t1b ON z1(b);\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"CREATE INDEX t1c ON z1(c);\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"CREATE UNIQUE INDEX t2b ON z2(b);\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"CREATE INDEX t2c ON z2(c DESC);\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"CREATE INDEX t3bc ON t3(b,c);\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = sz / 5 as ::core::ffi::c_int;
        speedtest1_begin_test(
            160 as ::core::ffi::c_int,
            b"%d SELECTS, numeric BETWEEN, indexed\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"SELECT count(*), avg(b), sum(length(c)), group_concat(a) FROM z1\n WHERE b BETWEEN ?1 AND ?2; -- %d times\0"
                .as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            if (i - 1 as ::core::ffi::c_int) % g.nRepeat == 0 as ::core::ffi::c_int {
                x1 = speedtest1_random().wrapping_rem(maxb as ::core::ffi::c_uint);
                x2 = speedtest1_random()
                    .wrapping_rem(10 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (sz / 5000 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                    )
                    .wrapping_add(x1);
            }
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, x1 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 2 as ::core::ffi::c_int, x2 as ::core::ffi::c_int);
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = sz / 5 as ::core::ffi::c_int;
        speedtest1_begin_test(
            161 as ::core::ffi::c_int,
            b"%d SELECTS, numeric BETWEEN, PK\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"SELECT count(*), avg(b), sum(length(c)), group_concat(a) FROM z2\n WHERE a BETWEEN ?1 AND ?2; -- %d times\0"
                .as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            if (i - 1 as ::core::ffi::c_int) % g.nRepeat == 0 as ::core::ffi::c_int {
                x1 = speedtest1_random().wrapping_rem(maxb as ::core::ffi::c_uint);
                x2 = speedtest1_random()
                    .wrapping_rem(10 as ::core::ffi::c_uint)
                    .wrapping_add(
                        (sz / 5000 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                    )
                    .wrapping_add(x1);
            }
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, x1 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 2 as ::core::ffi::c_int, x2 as ::core::ffi::c_int);
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = sz / 5 as ::core::ffi::c_int;
        speedtest1_begin_test(
            170 as ::core::ffi::c_int,
            b"%d SELECTS, text BETWEEN, indexed\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"SELECT count(*), avg(b), sum(length(c)), group_concat(a) FROM z1\n WHERE c BETWEEN ?1 AND (?1||'~'); -- %d times\0"
                .as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            if (i - 1 as ::core::ffi::c_int) % g.nRepeat == 0 as ::core::ffi::c_int {
                x1 = swizzle(i as ::core::ffi::c_uint, maxb as ::core::ffi::c_uint);
                len = speedtest1_numbername(
                    x1,
                    &raw mut zNum as *mut ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 2000]>() as usize)
                        .wrapping_sub(1 as usize) as ::core::ffi::c_int,
                );
            }
            sqlite3_bind_text(
                g.pStmt,
                1 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                len,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = sz;
        speedtest1_begin_test(
            180 as ::core::ffi::c_int,
            b"%d INSERTS with three indexes\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"CREATE%s TABLE t4(\n  a INTEGER %s %s,\n  b INTEGER %s,\n  c TEXT %s\n) %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            isTemp(1 as ::core::ffi::c_int),
            g.zNN,
            g.zPK,
            g.zNN,
            g.zNN,
            g.zWR,
        );
        speedtest1_exec(
            b"CREATE INDEX t4b ON t4(b)\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"CREATE INDEX t4c ON t4(c)\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"INSERT INTO t4 SELECT * FROM z1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = sz;
        speedtest1_begin_test(
            190 as ::core::ffi::c_int,
            b"DELETE and REFILL one table\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"DELETE FROM z2;\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"INSERT INTO z2 SELECT * FROM z1;\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            200 as ::core::ffi::c_int,
            b"VACUUM\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(b"VACUUM\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        speedtest1_begin_test(
            210 as ::core::ffi::c_int,
            b"ALTER TABLE ADD COLUMN, and query\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"ALTER TABLE z2 ADD COLUMN d INT DEFAULT 123\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"SELECT sum(d) FROM z2\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        n = sz / 5 as ::core::ffi::c_int;
        speedtest1_begin_test(
            230 as ::core::ffi::c_int,
            b"%d UPDATES, numeric BETWEEN, indexed\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"UPDATE z2 SET d=b*2 WHERE b BETWEEN ?1 AND ?2; -- %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = speedtest1_random().wrapping_rem(maxb as ::core::ffi::c_uint);
            x2 = speedtest1_random()
                .wrapping_rem(10 as ::core::ffi::c_uint)
                .wrapping_add((sz / 5000 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_add(x1);
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, x1 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 2 as ::core::ffi::c_int, x2 as ::core::ffi::c_int);
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = sz;
        speedtest1_begin_test(
            240 as ::core::ffi::c_int,
            b"%d UPDATES of individual rows\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"UPDATE z2 SET d=b*3 WHERE a=?1; -- %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = speedtest1_random()
                .wrapping_rem(sz as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint);
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, x1 as ::core::ffi::c_int);
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        speedtest1_begin_test(
            250 as ::core::ffi::c_int,
            b"One big UPDATE of the whole %d-row table\0".as_ptr()
                as *const ::core::ffi::c_char,
            sz,
        );
        speedtest1_exec(b"UPDATE z2 SET d=b*4\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        speedtest1_begin_test(
            260 as ::core::ffi::c_int,
            b"Query added column after filling\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"SELECT sum(d) FROM z2\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        n = sz / 5 as ::core::ffi::c_int;
        speedtest1_begin_test(
            270 as ::core::ffi::c_int,
            b"%d DELETEs, numeric BETWEEN, indexed\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"DELETE FROM z2 WHERE b BETWEEN ?1 AND ?2; -- %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = speedtest1_random()
                .wrapping_rem(maxb as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint);
            x2 = speedtest1_random()
                .wrapping_rem(10 as ::core::ffi::c_uint)
                .wrapping_add((sz / 5000 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                .wrapping_add(x1);
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, x1 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 2 as ::core::ffi::c_int, x2 as ::core::ffi::c_int);
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = sz;
        speedtest1_begin_test(
            280 as ::core::ffi::c_int,
            b"%d DELETEs of individual rows\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"DELETE FROM t3 WHERE a=?1; -- %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = speedtest1_random()
                .wrapping_rem(sz as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint);
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, x1 as ::core::ffi::c_int);
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        speedtest1_begin_test(
            290 as ::core::ffi::c_int,
            b"Refill two %d-row tables using REPLACE\0".as_ptr()
                as *const ::core::ffi::c_char,
            sz,
        );
        speedtest1_exec(
            b"REPLACE INTO z2(a,b,c) SELECT a,b,c FROM z1\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"REPLACE INTO t3(a,b,c) SELECT a,b,c FROM z1\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            300 as ::core::ffi::c_int,
            b"Refill a %d-row table using (b&1)==(a&1)\0".as_ptr()
                as *const ::core::ffi::c_char,
            sz,
        );
        speedtest1_exec(b"DELETE FROM z2;\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"INSERT INTO z2(a,b,c)\n SELECT a,b,c FROM z1  WHERE (b&1)==(a&1);\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"INSERT INTO z2(a,b,c)\n SELECT a,b,c FROM z1  WHERE (b&1)<>(a&1);\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        n = sz / 5 as ::core::ffi::c_int;
        speedtest1_begin_test(
            310 as ::core::ffi::c_int,
            b"%d four-ways joins\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"SELECT z1.c FROM z1, z2, t3, t4\n WHERE t4.a BETWEEN ?1 AND ?2\n   AND t3.a=t4.b\n   AND z2.a=t3.b\n   AND z1.c=z2.c;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = speedtest1_random()
                .wrapping_rem(sz as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint);
            x2 = speedtest1_random()
                .wrapping_rem(10 as ::core::ffi::c_uint)
                .wrapping_add(x1)
                .wrapping_add(4 as ::core::ffi::c_uint);
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, x1 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 2 as ::core::ffi::c_int, x2 as ::core::ffi::c_int);
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        speedtest1_begin_test(
            320 as ::core::ffi::c_int,
            b"subquery in result set\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"SELECT sum(a), max(c),\n       avg((SELECT a FROM z2 WHERE 5+z2.b=z1.b) AND rowid<?1), max(c)\n FROM z1 WHERE rowid<?1;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3_bind_int(
            g.pStmt,
            1 as ::core::ffi::c_int,
            est_square_root(g.szTest) * 50 as ::core::ffi::c_int,
        );
        speedtest1_run();
        speedtest1_end_test();
        n = g.szTest * 700 as ::core::ffi::c_int;
        sz = n;
        zNum[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        maxb = roundup_allones((sz / 3 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        speedtest1_begin_test(
            400 as ::core::ffi::c_int,
            b"%d REPLACE ops on an IPK\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"CREATE%s TABLE t5(a INTEGER PRIMARY KEY, b %s);\0".as_ptr()
                as *const ::core::ffi::c_char,
            isTemp(9 as ::core::ffi::c_int),
            g.zNN,
        );
        speedtest1_prepare(
            b"REPLACE INTO t5 VALUES(?1,?2); --  %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = swizzle(i as ::core::ffi::c_uint, maxb as ::core::ffi::c_uint);
            speedtest1_numbername(
                i as ::core::ffi::c_uint,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                x1 as sqlite3_int64 as ::core::ffi::c_int,
            );
            sqlite3_bind_text(
                g.pStmt,
                2 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        speedtest1_begin_test(
            410 as ::core::ffi::c_int,
            b"%d SELECTS on an IPK\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        if g.doBigTransactions != 0 {
            speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        }
        speedtest1_prepare(
            b"SELECT b FROM t5 WHERE a=?1; --  %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = swizzle(i as ::core::ffi::c_uint, maxb as ::core::ffi::c_uint);
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                x1 as sqlite3_int64 as ::core::ffi::c_int,
            );
            speedtest1_run();
            i += 1;
        }
        if g.doBigTransactions != 0 {
            speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        }
        speedtest1_end_test();
        n = g.szTest * 700 as ::core::ffi::c_int;
        sz = n;
        zNum[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        maxb = roundup_allones((sz / 3 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        speedtest1_begin_test(
            500 as ::core::ffi::c_int,
            b"%d REPLACE on TEXT PK\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"CREATE%s TABLE t6(a TEXT PRIMARY KEY, b %s)%s;\0".as_ptr()
                as *const ::core::ffi::c_char,
            isTemp(9 as ::core::ffi::c_int),
            g.zNN,
            if sqlite3_libversion_number() >= 3008002 as ::core::ffi::c_int {
                b"WITHOUT ROWID\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        speedtest1_prepare(
            b"REPLACE INTO t6 VALUES(?1,?2); --  %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = swizzle(i as ::core::ffi::c_uint, maxb as ::core::ffi::c_uint);
            speedtest1_numbername(
                x1,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            sqlite3_bind_int(g.pStmt, 2 as ::core::ffi::c_int, i);
            sqlite3_bind_text(
                g.pStmt,
                1 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        speedtest1_begin_test(
            510 as ::core::ffi::c_int,
            b"%d SELECTS on a TEXT PK\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        if g.doBigTransactions != 0 {
            speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        }
        speedtest1_prepare(
            b"SELECT b FROM t6 WHERE a=?1; --  %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            x1 = swizzle(i as ::core::ffi::c_uint, maxb as ::core::ffi::c_uint);
            speedtest1_numbername(
                x1,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            sqlite3_bind_text(
                g.pStmt,
                1 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        if g.doBigTransactions != 0 {
            speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            520 as ::core::ffi::c_int,
            b"%d SELECT DISTINCT\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(
            b"SELECT DISTINCT b FROM t5;\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"SELECT DISTINCT b FROM t6;\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            980 as ::core::ffi::c_int,
            b"PRAGMA integrity_check\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"PRAGMA integrity_check\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            990 as ::core::ffi::c_int,
            b"ANALYZE\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(b"ANALYZE\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testset_cte() {
    unsafe {
        static mut azPuzzle: [*const ::core::ffi::c_char; 3] = [
            b"534...9..67.195....98....6.8...6...34..8.3..1....2...6.6....28....419..5...28..79\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"53....9..6..195....98....6.8...6...34..8.3..1....2...6.6....28....419..5....8..79\0"
                .as_ptr() as *const ::core::ffi::c_char,
            b"53.......6..195....98....6.8...6...34..8.3..1....2...6.6....28....419..5....8..79\0"
                .as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut zPuz: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rSpacing: ::core::ffi::c_double = 0.;
        let mut nElem: ::core::ffi::c_int = 0;
        if g.szTest < 25 as ::core::ffi::c_int {
            zPuz = azPuzzle[0 as ::core::ffi::c_int as usize];
        } else if g.szTest < 70 as ::core::ffi::c_int {
            zPuz = azPuzzle[1 as ::core::ffi::c_int as usize];
        } else {
            zPuz = azPuzzle[2 as ::core::ffi::c_int as usize];
        }
        speedtest1_begin_test(
            100 as ::core::ffi::c_int,
            b"Sudoku with recursive 'digits'\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"WITH RECURSIVE\n  input(sud) AS (VALUES(?1)),\n  digits(z,lp) AS (\n    VALUES('1', 1)\n    UNION ALL\n    SELECT CAST(lp+1 AS TEXT), lp+1 FROM digits WHERE lp<9\n  ),\n  x(s, ind) AS (\n    SELECT sud, instr(sud, '.') FROM input\n    UNION ALL\n    SELECT\n      substr(s, 1, ind-1) || z || substr(s, ind+1),\n      instr( substr(s, 1, ind-1) || z || substr(s, ind+1), '.' )\n     FROM x, digits AS z\n    WHERE ind>0\n      AND NOT EXISTS (\n            SELECT 1\n              FROM digits AS lp\n             WHERE z.z = substr(s, ((ind-1)/9)*9 + lp, 1)\n                OR z.z = substr(s, ((ind-1)%%9) + (lp-1)*9 + 1, 1)\n                OR z.z = substr(s, (((ind-1)/3) %% 3) * 3\n                        + ((ind-1)/27) * 27 + lp\n                        + ((lp-1) / 3) * 6, 1)\n         )\n  )\nSELECT s FROM x WHERE ind=0;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3_bind_text(
            g.pStmt,
            1 as ::core::ffi::c_int,
            zPuz,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        speedtest1_run();
        speedtest1_end_test();
        speedtest1_begin_test(
            200 as ::core::ffi::c_int,
            b"Sudoku with VALUES 'digits'\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"WITH RECURSIVE\n  input(sud) AS (VALUES(?1)),\n  digits(z,lp) AS (VALUES('1',1),('2',2),('3',3),('4',4),('5',5),\n                         ('6',6),('7',7),('8',8),('9',9)),\n  x(s, ind) AS (\n    SELECT sud, instr(sud, '.') FROM input\n    UNION ALL\n    SELECT\n      substr(s, 1, ind-1) || z || substr(s, ind+1),\n      instr( substr(s, 1, ind-1) || z || substr(s, ind+1), '.' )\n     FROM x, digits AS z\n    WHERE ind>0\n      AND NOT EXISTS (\n            SELECT 1\n              FROM digits AS lp\n             WHERE z.z = substr(s, ((ind-1)/9)*9 + lp, 1)\n                OR z.z = substr(s, ((ind-1)%%9) + (lp-1)*9 + 1, 1)\n                OR z.z = substr(s, (((ind-1)/3) %% 3) * 3\n                        + ((ind-1)/27) * 27 + lp\n                        + ((lp-1) / 3) * 6, 1)\n         )\n  )\nSELECT s FROM x WHERE ind=0;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3_bind_text(
            g.pStmt,
            1 as ::core::ffi::c_int,
            zPuz,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        speedtest1_run();
        speedtest1_end_test();
        rSpacing = 5.0f64 / g.szTest as ::core::ffi::c_double;
        speedtest1_begin_test(
            300 as ::core::ffi::c_int,
            b"Mandelbrot Set with spacing=%f\0".as_ptr() as *const ::core::ffi::c_char,
            rSpacing,
        );
        speedtest1_prepare(
            b"WITH RECURSIVE \n  xaxis(x) AS (VALUES(-2.0) UNION ALL SELECT x+?1 FROM xaxis WHERE x<1.2),\n  yaxis(y) AS (VALUES(-1.0) UNION ALL SELECT y+?2 FROM yaxis WHERE y<1.0),\n  m(iter, cx, cy, x, y) AS (\n    SELECT 0, x, y, 0.0, 0.0 FROM xaxis, yaxis\n    UNION ALL\n    SELECT iter+1, cx, cy, x*x-y*y + cx, 2.0*x*y + cy FROM m \n     WHERE (x*x + y*y) < 4.0 AND iter<28\n  ),\n  m2(iter, cx, cy) AS (\n    SELECT max(iter), cx, cy FROM m GROUP BY cx, cy\n  ),\n  a(t) AS (\n    SELECT group_concat( substr(' .+*#', 1+min(iter/7,4), 1), '') \n    FROM m2 GROUP BY cy\n  )\nSELECT group_concat(rtrim(t),x'0a') FROM a;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3_bind_double(g.pStmt, 1 as ::core::ffi::c_int, rSpacing * 0.05f64);
        sqlite3_bind_double(g.pStmt, 2 as ::core::ffi::c_int, rSpacing);
        speedtest1_run();
        speedtest1_end_test();
        nElem = 10000 as ::core::ffi::c_int * g.szTest;
        speedtest1_begin_test(
            400 as ::core::ffi::c_int,
            b"EXCEPT operator on %d-element tables\0".as_ptr()
                as *const ::core::ffi::c_char,
            nElem,
        );
        speedtest1_prepare(
            b"WITH RECURSIVE \n  z1(x) AS (VALUES(2) UNION ALL SELECT x+2 FROM z1 WHERE x<%d),\n  z2(y) AS (VALUES(3) UNION ALL SELECT y+3 FROM z2 WHERE y<%d)\nSELECT count(x), avg(x) FROM (\n  SELECT x FROM z1 EXCEPT SELECT y FROM z2 ORDER BY 1\n);\0"
                .as_ptr() as *const ::core::ffi::c_char,
            nElem,
            nElem,
        );
        speedtest1_run();
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_random_ascii_fp(mut zFP: *mut ::core::ffi::c_char) {
    unsafe {
        let mut x: ::core::ffi::c_int = speedtest1_random() as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = speedtest1_random() as ::core::ffi::c_int;
        let mut z: ::core::ffi::c_int = 0;
        z = y % 10 as ::core::ffi::c_int;
        if z < 0 as ::core::ffi::c_int {
            z = -z;
        }
        y /= 10 as ::core::ffi::c_int;
        sqlite3_snprintf(
            100 as ::core::ffi::c_int,
            zFP,
            b"%d.%de%d\0".as_ptr() as *const ::core::ffi::c_char,
            y,
            z,
            x % 200 as ::core::ffi::c_int,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testset_fp() {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut zFP1: [::core::ffi::c_char; 100] = [0; 100];
        let mut zFP2: [::core::ffi::c_char; 100] = [0; 100];
        n = g.szTest * 5000 as ::core::ffi::c_int;
        speedtest1_begin_test(
            100 as ::core::ffi::c_int,
            b"Fill a table with %d FP values\0".as_ptr() as *const ::core::ffi::c_char,
            n * 2 as ::core::ffi::c_int,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"CREATE%s TABLE z1(a REAL %s, b REAL %s);\0".as_ptr()
                as *const ::core::ffi::c_char,
            isTemp(1 as ::core::ffi::c_int),
            g.zNN,
            g.zNN,
        );
        speedtest1_prepare(
            b"INSERT INTO z1 VALUES(?1,?2); -- %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            speedtest1_random_ascii_fp(&raw mut zFP1 as *mut ::core::ffi::c_char);
            speedtest1_random_ascii_fp(&raw mut zFP2 as *mut ::core::ffi::c_char);
            sqlite3_bind_text(
                g.pStmt,
                1 as ::core::ffi::c_int,
                &raw mut zFP1 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            sqlite3_bind_text(
                g.pStmt,
                2 as ::core::ffi::c_int,
                &raw mut zFP2 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = g.szTest / 25 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
        speedtest1_begin_test(
            110 as ::core::ffi::c_int,
            b"%d range queries\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"SELECT sum(b) FROM z1 WHERE a BETWEEN ?1 AND ?2\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            speedtest1_random_ascii_fp(&raw mut zFP1 as *mut ::core::ffi::c_char);
            speedtest1_random_ascii_fp(&raw mut zFP2 as *mut ::core::ffi::c_char);
            sqlite3_bind_text(
                g.pStmt,
                1 as ::core::ffi::c_int,
                &raw mut zFP1 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            sqlite3_bind_text(
                g.pStmt,
                2 as ::core::ffi::c_int,
                &raw mut zFP2 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            120 as ::core::ffi::c_int,
            b"CREATE INDEX three times\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(b"BEGIN;\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"CREATE INDEX t1a ON z1(a);\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"CREATE INDEX t1b ON z1(b);\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"CREATE INDEX t1ab ON z1(a,b);\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = g.szTest / 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
        speedtest1_begin_test(
            130 as ::core::ffi::c_int,
            b"%d indexed range queries\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"SELECT sum(b) FROM z1 WHERE a BETWEEN ?1 AND ?2\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            speedtest1_random_ascii_fp(&raw mut zFP1 as *mut ::core::ffi::c_char);
            speedtest1_random_ascii_fp(&raw mut zFP2 as *mut ::core::ffi::c_char);
            sqlite3_bind_text(
                g.pStmt,
                1 as ::core::ffi::c_int,
                &raw mut zFP1 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            sqlite3_bind_text(
                g.pStmt,
                2 as ::core::ffi::c_int,
                &raw mut zFP2 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            i += 1;
        }
        speedtest1_end_test();
        n = g.szTest * 5000 as ::core::ffi::c_int;
        speedtest1_begin_test(
            140 as ::core::ffi::c_int,
            b"%d calls to round()\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(
            b"SELECT sum(round(a,2)+round(b,4)) FROM z1;\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            150 as ::core::ffi::c_int,
            b"%d printf() calls\0".as_ptr() as *const ::core::ffi::c_char,
            n * 4 as ::core::ffi::c_int,
        );
        speedtest1_exec(
            b"WITH c(fmt) AS (VALUES('%%g'),('%%e'),('%%!g'),('%%.20f'))SELECT sum(printf(fmt,a)) FROM z1, c\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testset_star() {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        n = g.szTest * 50 as ::core::ffi::c_int;
        speedtest1_begin_test(
            100 as ::core::ffi::c_int,
            b"Create a fact table with %d entries\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(
            b"CREATE TABLE facttab( attr01 INT, attr02 INT, attr03 INT, data01 TEXT, attr04 INT, attr05 INT, attr06 INT, attr07 INT, attr08 INT, factid INTEGER PRIMARY KEY, data02 TEXT);\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"WITH RECURSIVE counter(nnn) AS(VALUES(1) UNION ALL SELECT nnn+1 FROM counter WHERE nnn<%d)INSERT INTO facttab(attr01,attr02,attr03,attr04,attr05,attr06,attr07,attr08,data01,data02)SELECT random()%%12, random()%%13, random()%%14, random()%%15,random()%%16, random()%%17, random()%%18, random()%%19,concat('data-',nnn), format('%%x',random()) FROM counter;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            110 as ::core::ffi::c_int,
            b"Create indexes on all attributes columns\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= 8 as ::core::ffi::c_int {
            speedtest1_exec(
                b"CREATE INDEX fact_attr%02d ON facttab(attr%02d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i,
                i,
            );
            i += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            120 as ::core::ffi::c_int,
            b"Create dimension tables\0".as_ptr() as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= 8 as ::core::ffi::c_int {
            speedtest1_exec(
                b"CREATE TABLE dimension%02d(beta%02d INT, content%02d TEXT, rate%02d REAL)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                i,
                i,
                i,
                i,
            );
            speedtest1_exec(
                b"WITH RECURSIVE ctr(nn) AS (VALUES(1) UNION ALL SELECT nn+1 FROM ctr WHERE nn<%d) INSERT INTO dimension%02d   SELECT nn%%(%d), concat('content-%02d-',nn), (random()%%10000)*0.125 FROM ctr;\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                4 as ::core::ffi::c_int * (i + 1 as ::core::ffi::c_int),
                i,
                2 as ::core::ffi::c_int * (i + 1 as ::core::ffi::c_int),
                i,
            );
            if i & 2 as ::core::ffi::c_int != 0 {
                speedtest1_exec(
                    b"CREATE INDEX dim%02d ON dimension%02d(beta%02d);\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    i,
                    i,
                    i,
                );
            } else {
                speedtest1_exec(
                    b"CREATE INDEX dim%02d ON dimension%02d(beta%02d,content%02d);\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    i,
                    i,
                    i,
                    i,
                );
            }
            i += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            130 as ::core::ffi::c_int,
            b"Star query over the entire fact table\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"SELECT count(*), max(content04), min(content03), sum(rate04), avg(rate05) FROM facttab, dimension01, dimension02, dimension03, dimension04, dimension05, dimension06, dimension07, dimension08 WHERE attr01=beta01 AND attr02=beta02 AND attr03=beta03 AND attr04=beta04 AND attr05=beta05 AND attr06=beta06 AND attr07=beta07 AND attr08=beta08;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            130 as ::core::ffi::c_int,
            b"Star query with LEFT JOINs\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"SELECT count(*), max(content04), min(content03), sum(rate04), avg(rate05) FROM facttab LEFT JOIN dimension01 ON attr01=beta01 LEFT JOIN dimension02 ON attr02=beta02 JOIN dimension03 ON attr03=beta03 JOIN dimension04 ON attr04=beta04 JOIN dimension05 ON attr05=beta05 LEFT JOIN dimension06 ON attr06=beta06 JOIN dimension07 ON attr07=beta07 JOIN dimension08 ON attr08=beta08 WHERE facttab.data01 LIKE 'data-9%%';\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
    }
}
unsafe extern "C" fn testset_app() {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        speedtest1_begin_test(
            100 as ::core::ffi::c_int,
            b"Generate a Fossil-like database schema\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"BEGIN;CREATE TABLE blob(\n  rid INTEGER PRIMARY KEY,\n  rcvid INTEGER,\n  size INTEGER,\n  uuid TEXT UNIQUE NOT NULL,\n  content BLOB,\n  CHECK( length(uuid)>=40 AND rid>0 )\n);\nCREATE TABLE delta(\n  rid INTEGER PRIMARY KEY,\n  srcid INTEGER NOT NULL REFERENCES blob\n);\nCREATE TABLE rcvfrom(\n  rcvid INTEGER PRIMARY KEY,\n  uid INTEGER REFERENCES user,\n  mtime DATETIME,\n  nonce TEXT UNIQUE,\n  ipaddr TEXT\n);\nCREATE TABLE private(rid INTEGER PRIMARY KEY);\nCREATE TABLE accesslog(\n  uname TEXT,\n  ipaddr TEXT,\n  success BOOLEAN,\n  mtime TIMESTAMP\n);\nCREATE TABLE user(\n  uid INTEGER PRIMARY KEY,\n  login TEXT UNIQUE,\n  pw TEXT,\n  cap TEXT,\n  cookie TEXT,\n  ipaddr TEXT,\n  cexpire DATETIME,\n  info TEXT,\n  mtime DATE,\n  photo BLOB\n, jx TEXT DEFAULT '{}');\nCREATE TABLE reportfmt(\n   rn INTEGER PRIMARY KEY,\n   owner TEXT,\n   title TEXT UNIQUE,\n   mtime INTEGER,\n   cols TEXT,\n   sqlcode TEXT\n, jx TEXT DEFAULT '{}');\nCREATE TABLE config(\n  name TEXT PRIMARY KEY NOT NULL,\n  value CLOB, mtime INTEGER,\n  CHECK( typeof(name)='text' AND length(name)>=1 )\n) WITHOUT ROWID;\nCREATE TABLE shun(uuid PRIMARY KEY, mtime INTEGER, scom TEXT)\n  WITHOUT ROWID;\nCREATE TABLE concealed(\n  hash TEXT PRIMARY KEY,\n  content TEXT\n, mtime INTEGER) WITHOUT ROWID;\nCREATE TABLE admin_log(\n id INTEGER PRIMARY KEY,\n time INTEGER, -- Seconds since 1970\n page TEXT,    -- path of page\n who TEXT,     -- User who made the change\n  what TEXT     -- What changed\n);\nCREATE TABLE unversioned(\n  name TEXT PRIMARY KEY,\n  rcvid INTEGER,\n  mtime DATETIME,\n  hash TEXT,\n  sz INTEGER,\n  encoding INT,\n  content BLOB\n) WITHOUT ROWID;\nCREATE TABLE subscriber(\n  subscriberId INTEGER PRIMARY KEY,\n  subscriberCode BLOB DEFAULT (randomblob(32)) UNIQUE,\n  semail TEXT UNIQUE COLLATE nocase,\n  suname TEXT,\n  sverified BOOLEAN DEFAULT true,\n  sdonotcall BOOLEAN,\n  sdigest BOOLEAN,\n  ssub TEXT,\n  sctime INTDATE,\n  mtime INTDATE,\n  smip TEXT\n, lastContact INT);\nCREATE TABLE pending_alert(\n  eventid TEXT PRIMARY KEY,\n  sentSep BOOLEAN DEFAULT false,\n  sentDigest BOOLEAN DEFAULT false\n, sentMod BOOLEAN DEFAULT false) WITHOUT ROWID;\nCREATE TABLE filename(\n  fnid INTEGER PRIMARY KEY,\n  name TEXT UNIQUE\n) STRICT;\nCREATE TABLE mlink(\n  mid INTEGER,\n  fid INTEGER,\n  pmid INTEGER,\n  pid INTEGER,\n  fnid INTEGER REFERENCES filename,\n  pfnid INTEGER,\n  mperm INTEGER,\n  isaux INT DEFAULT 0\n) STRICT;\nCREATE TABLE plink(\n  pid INTEGER REFERENCES blob,\n  cid INTEGER REFERENCES blob,\n  isprim INT,\n  mtime REAL,\n  baseid INTEGER REFERENCES blob,\n  UNIQUE(pid, cid)\n) STRICT;\nCREATE TABLE leaf(rid INTEGER PRIMARY KEY);\nCREATE TABLE event(\n  type TEXT,\n  mtime REAL,\n  objid INTEGER PRIMARY KEY,\n  tagid INTEGER,\n  uid INTEGER REFERENCES user,\n  bgcolor TEXT,\n  euser TEXT,\n  user TEXT,\n  ecomment TEXT,\n  comment TEXT,\n  brief TEXT,\n  omtime REAL\n) STRICT;\nCREATE TABLE phantom(\n  rid INTEGER PRIMARY KEY\n);\nCREATE TABLE orphan(\n  rid INTEGER PRIMARY KEY,\n  baseline INTEGER\n) STRICT;\nCREATE TABLE unclustered(\n  rid INTEGER PRIMARY KEY\n);\nCREATE TABLE unsent(\n  rid INTEGER PRIMARY KEY\n);\nCREATE TABLE tag(\n  tagid INTEGER PRIMARY KEY,\n  tagname TEXT UNIQUE\n) STRICT;\nCREATE TABLE tagxref(\n  tagid INTEGER REFERENCES tag,\n  tagtype INTEGER,\n  srcid INTEGER REFERENCES blob,\n  origid INTEGER REFERENCES blob,\n  value TEXT,\n  mtime REAL,\n  rid INTEGER REFERENCES blob,\n  UNIQUE(rid, tagid)\n) STRICT;\nCREATE TABLE backlink(\n  target TEXT,\n  srctype INT,\n  srcid INT,\n  mtime REAL,\n  UNIQUE(target, srctype, srcid)\n) STRICT;\nCREATE TABLE attachment(\n  attachid INTEGER PRIMARY KEY,\n  isLatest INT DEFAULT 0,\n  mtime REAL,\n  src TEXT,\n  target TEXT,\n  filename TEXT,\n  comment TEXT,\n  user TEXT\n) STRICT;\nCREATE TABLE cherrypick(\n  parentid INT,\n  childid INT,\n  isExclude INT DEFAULT false,\n  PRIMARY KEY(parentid, childid)\n) WITHOUT ROWID, STRICT;\nCREATE TABLE vcache(\n  vid INTEGER,         -- check-in ID\n  fname TEXT,          -- filename\n  rid INTEGER,         -- artifact ID\n  PRIMARY KEY(vid,fname)\n) WITHOUT ROWID;\nCREATE TABLE synclog(\n  sfrom TEXT,\n  sto TEXT,\n  stime INT NOT NULL,\n  stype TEXT,\n  PRIMARY KEY(sfrom,sto)\n) WITHOUT ROWID;\nCREATE TABLE chat(\n  msgid INTEGER PRIMARY KEY AUTOINCREMENT,\n  mtime JULIANDAY,\n  lmtime TEXT,\n  xfrom TEXT,\n  xmsg  TEXT,\n  fname TEXT,\n  fmime TEXT,\n  mdel INT,\n  file  BLOB\n);\nCREATE TABLE ftsdocs(\n  rowid INTEGER PRIMARY KEY,\n  type CHAR(1),\n  rid INTEGER,\n  name TEXT,\n  idxed BOOLEAN,\n  label TEXT,\n  url TEXT,\n  mtime DATE,\n  bx TEXT,\n  UNIQUE(type,rid)\n);\nCREATE TABLE ticket(\n  -- Do not change any column that begins with tkt_\n  tkt_id INTEGER PRIMARY KEY,\n  tkt_uuid TEXT UNIQUE,\n  tkt_mtime DATE,\n  tkt_ctime DATE,\n  -- Add as many fields as required below this line\n  type TEXT,\n  status TEXT,\n  subsystem TEXT,\n  priority TEXT,\n  severity TEXT,\n  foundin TEXT,\n  private_contact TEXT,\n  resolution TEXT,\n  title TEXT,\n  comment TEXT\n);\nCREATE TABLE ticketchng(\n  -- Do not change any column that begins with tkt_\n  tkt_id INTEGER REFERENCES ticket,\n  tkt_rid INTEGER REFERENCES blob,\n  tkt_mtime DATE,\n  tkt_user TEXT,\n  -- Add as many fields as required below this line\n  login TEXT,\n  username TEXT,\n  mimetype TEXT,\n  icomment TEXT\n);\nCREATE TABLE forumpost(\n  fpid INTEGER PRIMARY KEY,\n  froot INT,\n  fprev INT,\n  firt INT,\n  fmtime REAL\n);\nCREATE INDEX delta_i1 ON delta(srcid);\nCREATE INDEX blob_rcvid ON blob(rcvid);\nCREATE INDEX subscriberUname\n  ON subscriber(suname) WHERE suname IS NOT NULL;\nCREATE INDEX mlink_i1 ON mlink(mid);\nCREATE INDEX mlink_i2 ON mlink(fnid);\nCREATE INDEX mlink_i3 ON mlink(fid);\nCREATE INDEX mlink_i4 ON mlink(pid);\nCREATE INDEX plink_i2 ON plink(cid,pid);\nCREATE INDEX event_i1 ON event(mtime);\nCREATE INDEX orphan_baseline ON orphan(baseline);\nCREATE INDEX tagxref_i1 ON tagxref(tagid, mtime);\nCREATE INDEX backlink_src ON backlink(srcid, srctype);\nCREATE INDEX attachment_idx1 ON attachment(target, filename, mtime);\nCREATE INDEX attachment_idx2 ON attachment(src);\nCREATE INDEX cherrypick_cid ON cherrypick(childid);\nCREATE INDEX ftsdocIdxed ON ftsdocs(type,rid,name) WHERE idxed==0;\nCREATE INDEX ftsdocName ON ftsdocs(name) WHERE type='w';\nCREATE INDEX ticketchng_idx1 ON ticketchng(tkt_id, tkt_mtime);\nCREATE INDEX forumthread ON forumpost(froot,fmtime);\nCREATE VIEW artifact(rid,rcvid,size,atype,srcid,hash,content) AS\n  SELECT blob.rid,rcvid,size,1,srcid,uuid,content\n    FROM blob LEFT JOIN delta ON (blob.rid=delta.rid);\nCREATE VIEW ftscontent AS\n  SELECT rowid, type, rid, name, idxed, label, url, mtime,\n         title(type,rid,name) AS 'title', body(type,rid,name) AS 'body'\n    FROM ftsdocs;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        if sqlite3_compileoption_used(
            b"ENABLE_FTS5\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
            speedtest1_exec(
                b"CREATE VIRTUAL TABLE ftsidx\n  USING fts5(content=\"ftscontent\", title, body);\nCREATE VIRTUAL TABLE chatfts1 USING fts5(\n  xmsg, content=chat, content_rowid=msgid,tokenize=porter);\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            speedtest1_exec(
                b"CREATE TABLE ftsidx_data(id INTEGER PRIMARY KEY, block BLOB);\nCREATE TABLE ftsidx_idx(segid, term, pgno, PRIMARY KEY(segid, term))\n  WITHOUT ROWID;\nCREATE TABLE ftsidx_docsize(id INTEGER PRIMARY KEY, sz BLOB);\nCREATE TABLE ftsidx_config(k PRIMARY KEY, v) WITHOUT ROWID;\nCREATE TABLE chatfts1_data(id INTEGER PRIMARY KEY, block BLOB);\nCREATE TABLE chatfts1_idx(segid, term, pgno, PRIMARY KEY(segid, term))\n  WITHOUT ROWID;\nCREATE TABLE chatfts1_docsize(id INTEGER PRIMARY KEY, sz BLOB);\nCREATE TABLE chatfts1_config(k PRIMARY KEY, v) WITHOUT ROWID;\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        speedtest1_exec(
            b"ANALYZE sqlite_schema;\nINSERT INTO sqlite_stat1(tbl,idx,stat) VALUES\n  ('ftsidx_config','ftsidx_config','1 1'),\n  ('ftsidx_idx','ftsidx_idx','4215 401 1'),\n  ('user','sqlite_autoindex_user_1','25 1'),\n  ('phantom',NULL,'26'),\n  ('reportfmt','sqlite_autoindex_reportfmt_1','9 1'),\n  ('rcvfrom','sqlite_autoindex_rcvfrom_1','18445 401'),\n  ('private',NULL,'99'),\n  ('mlink','mlink_i4','116678 401'),\n  ('mlink','mlink_i3','121212 2'),\n  ('mlink','mlink_i2','106372 401'),\n  ('mlink','mlink_i1','99298 5'),\n  ('ftsidx_data',NULL,'3795'),\n  ('leaf',NULL,'1559'),\n  ('delta','delta_i1','66340 1'),\n  ('unversioned','unversioned','3 1'),\n  ('pending_alert','pending_alert','3 1'),\n  ('cherrypick','cherrypick_cid','680 2'),\n  ('cherrypick','cherrypick','628 1 1'),\n  ('config','config','128 1'),\n  ('ftsidx_docsize',NULL,'33848'),\n  ('event','event_i1','36096 1'),\n  ('plink','plink_i2','38236 1 1'),\n  ('plink','sqlite_autoindex_plink_1','38357 1 1'),\n  ('shun','shun','10 1'),\n  ('concealed','concealed','110 1'),\n  ('vcache','vcache','1888 401 1'),\n  ('ftsdocs','ftsdocName','19 1'),\n  ('ftsdocs','ftsdocIdxed','168 84 1 1'),\n  ('ftsdocs','sqlite_autoindex_ftsdocs_1','37312 401 1'),\n  ('subscriber','subscriberUname','5 1'),\n  ('subscriber','sqlite_autoindex_subscriber_2','37 1'),\n  ('subscriber','sqlite_autoindex_subscriber_1','37 1'),\n  ('tag','sqlite_autoindex_tag_1','2990 1'),\n  ('filename','sqlite_autoindex_filename_1','3168 1'),\n  ('chat',NULL,'56124'),\n  ('tagxref','tagxref_i1','40992 401 2'),\n  ('tagxref','sqlite_autoindex_tagxref_1','79233 3 1'),\n  ('attachment','attachment_idx2','11 1'),\n  ('attachment','attachment_idx1','11 2 2 1'),\n  ('blob','blob_rcvid','128240 201'),\n  ('blob','sqlite_autoindex_blob_1','126480 1'),\n  ('synclog','synclog','12 3 1'),\n  ('backlink','backlink_src','2160 2 2'),\n  ('backlink','sqlite_autoindex_backlink_1','2340 2 2 1'),\n  ('accesslog',NULL,'38'),\n  ('chatfts1_config','chatfts1_config','1 1'),\n  ('chatfts1_idx','chatfts1_idx','688 230 1'),\n  ('ticket','sqlite_autoindex_ticket_1','794 1'),\n  ('ticketchng','ticketchng_idx1','2089 3 1'),\n  ('forumpost','forumthread','4 4 1'),\n  ('unclustered',NULL,'12');\nCOMMIT;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        n = g.szTest * 3 as ::core::ffi::c_int;
        speedtest1_begin_test(
            110 as ::core::ffi::c_int,
            b"Open and use the database %d times\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        i = 0 as ::core::ffi::c_int;
        while i < n {
            let mut dbMain: *mut sqlite3 = g.db;
            let mut dbAux: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
            if !g.zDbName.is_null()
                && *g.zDbName.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int != 0
            {
                if sqlite3_open_v2(
                    g.zDbName,
                    &raw mut dbAux,
                    SQLITE_OPEN_READWRITE,
                    g.zVfs,
                ) != 0
                {
                    fatal_error(
                        b"Cannot open database file: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        g.zDbName,
                    );
                }
                g.db = dbAux;
            }
            speedtest1_exec(
                b"SELECT name FROM pragma_table_list /*scan*/ WHERE schema='repository' AND type IN ('table','virtual') AND name NOT IN ('admin_log', 'blob','delta','rcvfrom','user','alias','config','shun','private','reportfmt','concealed','accesslog','modreq','purgeevent','purgeitem','unversioned','subscriber','pending_alert','chat') AND name NOT GLOB 'sqlite_*' AND name NOT GLOB 'fx_*';SELECT 1 FROM pragma_table_xinfo('ticket') WHERE name = 'mimetype';\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            speedtest1_exec(
                b"SELECT name, value, unixepoch()/86400-value, date(value*86400,'unixepoch') FROM config WHERE name in ('email-renew-warning','email-renew-cutoff');SELECT count(*) FROM pending_alert WHERE NOT sentDigest;\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            speedtest1_exec(
                b"WITH priors(rid,who) AS (  SELECT firt, coalesce(euser,user)    FROM forumpost LEFT JOIN event ON fpid=objid   WHERE fpid=12345  UNION ALL  SELECT firt, coalesce(euser,user)    FROM priors, forumpost LEFT JOIN event ON fpid=objid   WHERE fpid=rid)SELECT ','||group_concat(DISTINCT 'u'||who)||','||group_concat(rid) FROM priors;\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            speedtest1_exec(
                b"CREATE TEMP TABLE IF NOT EXISTS ok(rid INTEGER PRIMARY KEY);\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            speedtest1_exec(
                b"WITH RECURSIVE\n  parent(pid,cid,isCP) AS (\n    SELECT plink.pid, plink.cid, 0 AS xisCP FROM plink\n    UNION ALL\n    SELECT parentid, childid, 1 FROM cherrypick WHERE NOT isExclude\n  ),\n  ancestor(rid, mtime, isCP) AS (\n    SELECT 123, mtime, 0 FROM event WHERE objid=$object\n    UNION\n    SELECT parent.pid, event.mtime, parent.isCP\n      FROM ancestor, parent, event\n     WHERE parent.cid=ancestor.rid\n       AND event.objid=parent.pid\n       AND NOT ancestor.isCP\n       AND (event.mtime>=$date OR parent.pid=$pid)\n     ORDER BY mtime DESC LIMIT 10\n  )\n  INSERT OR IGNORE INTO ok SELECT rid FROM ancestor;\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            sqlite3_close(dbAux);
            g.db = dbMain;
            i += 1;
        }
        speedtest1_end_test();
    }
}
unsafe extern "C" fn twoCoords(
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
    mut mx: ::core::ffi::c_uint,
    mut pX0: *mut ::core::ffi::c_uint,
    mut pX1: *mut ::core::ffi::c_uint,
) {
    unsafe {
        let mut d: ::core::ffi::c_uint = 0;
        let mut x0: ::core::ffi::c_uint = 0;
        let mut x1: ::core::ffi::c_uint = 0;
        let mut span: ::core::ffi::c_uint = 0;
        span = mx
            .wrapping_div(100 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint);
        if speedtest1_random().wrapping_rem(3 as ::core::ffi::c_uint)
            == 0 as ::core::ffi::c_uint
        {
            span = span.wrapping_mul(p1 as ::core::ffi::c_uint);
        }
        if speedtest1_random().wrapping_rem(p2 as ::core::ffi::c_uint)
            == 0 as ::core::ffi::c_uint
        {
            span = mx.wrapping_div(2 as ::core::ffi::c_uint);
        }
        d = speedtest1_random()
            .wrapping_rem(span)
            .wrapping_add(1 as ::core::ffi::c_uint);
        x0 = speedtest1_random()
            .wrapping_rem(mx.wrapping_sub(d))
            .wrapping_add(1 as ::core::ffi::c_uint);
        x1 = x0.wrapping_add(d);
        *pX0 = x0;
        *pX1 = x1;
    }
}
unsafe extern "C" fn xsliceGeometryCallback(
    mut p: *mut sqlite3_rtree_geometry,
    mut nCoord: ::core::ffi::c_int,
    mut aCoord: *mut ::core::ffi::c_double,
    mut pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        *pRes = (*aCoord.offset(3 as ::core::ffi::c_int as isize)
            >= *(*p).aParam.offset(0 as ::core::ffi::c_int as isize)
            && *aCoord.offset(2 as ::core::ffi::c_int as isize)
                <= *(*p).aParam.offset(1 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testset_rtree(
    mut p1: ::core::ffi::c_int,
    mut p2: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut n: ::core::ffi::c_uint = 0;
        let mut mxCoord: ::core::ffi::c_uint = 0;
        let mut x0: ::core::ffi::c_uint = 0;
        let mut x1: ::core::ffi::c_uint = 0;
        let mut y0: ::core::ffi::c_uint = 0;
        let mut y1: ::core::ffi::c_uint = 0;
        let mut z0: ::core::ffi::c_uint = 0;
        let mut z1: ::core::ffi::c_uint = 0;
        let mut iStep: ::core::ffi::c_uint = 0;
        let mut mxRowid: ::core::ffi::c_uint = 0;
        let mut aCheck: *mut ::core::ffi::c_int = sqlite3_malloc(
            (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                .wrapping_mul(g.szTest as usize)
                .wrapping_mul(500 as usize) as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_int;
        mxCoord = 15000 as ::core::ffi::c_uint;
        n = (g.szTest * 500 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        mxRowid = n;
        speedtest1_begin_test(
            100 as ::core::ffi::c_int,
            b"%d INSERTs into an r-tree\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"CREATE VIRTUAL TABLE rt1 USING rtree(id,x0,x1,y0,y1,z0,z1)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"INSERT INTO rt1(id,x0,x1,y0,y1,z0,z1)VALUES(?1,?2,?3,?4,?5,?6,?7)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_uint;
        while i <= n {
            twoCoords(p1, p2, mxCoord, &raw mut x0, &raw mut x1);
            twoCoords(p1, p2, mxCoord, &raw mut y0, &raw mut y1);
            twoCoords(p1, p2, mxCoord, &raw mut z0, &raw mut z1);
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, i as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 2 as ::core::ffi::c_int, x0 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 3 as ::core::ffi::c_int, x1 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 4 as ::core::ffi::c_int, y0 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 5 as ::core::ffi::c_int, y1 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 6 as ::core::ffi::c_int, z0 as ::core::ffi::c_int);
            sqlite3_bind_int(g.pStmt, 7 as ::core::ffi::c_int, z1 as ::core::ffi::c_int);
            speedtest1_run();
            i = i.wrapping_add(1);
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        speedtest1_begin_test(
            101 as ::core::ffi::c_int,
            b"Copy from rtree to a regular table\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"CREATE TABLE z1(id INTEGER PRIMARY KEY,x0,x1,y0,y1,z0,z1)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"INSERT INTO z1 SELECT * FROM rt1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        n = (g.szTest * 200 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        speedtest1_begin_test(
            110 as ::core::ffi::c_int,
            b"%d one-dimensional intersect slice queries\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"SELECT count(*) FROM rt1 WHERE x0>=?1 AND x1<=?2\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        iStep = mxCoord.wrapping_div(n);
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                i.wrapping_mul(iStep) as ::core::ffi::c_int,
            );
            sqlite3_bind_int(
                g.pStmt,
                2 as ::core::ffi::c_int,
                i.wrapping_add(1 as ::core::ffi::c_uint).wrapping_mul(iStep)
                    as ::core::ffi::c_int,
            );
            speedtest1_run();
            *aCheck.offset(i as isize) = atoi(
                &raw mut g.zResult as *mut ::core::ffi::c_char,
            );
            i = i.wrapping_add(1);
        }
        speedtest1_end_test();
        if g.bVerify != 0 {
            n = (g.szTest * 200 as ::core::ffi::c_int) as ::core::ffi::c_uint;
            speedtest1_begin_test(
                111 as ::core::ffi::c_int,
                b"Verify result from 1-D intersect slice queries\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            speedtest1_prepare(
                b"SELECT count(*) FROM z1 WHERE x0>=?1 AND x1<=?2\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            iStep = mxCoord.wrapping_div(n);
            i = 0 as ::core::ffi::c_uint;
            while i < n {
                sqlite3_bind_int(
                    g.pStmt,
                    1 as ::core::ffi::c_int,
                    i.wrapping_mul(iStep) as ::core::ffi::c_int,
                );
                sqlite3_bind_int(
                    g.pStmt,
                    2 as ::core::ffi::c_int,
                    i.wrapping_add(1 as ::core::ffi::c_uint).wrapping_mul(iStep)
                        as ::core::ffi::c_int,
                );
                speedtest1_run();
                if *aCheck.offset(i as isize)
                    != atoi(&raw mut g.zResult as *mut ::core::ffi::c_char)
                {
                    fatal_error(
                        b"Count disagree step %d: %d..%d.  %d vs %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        i,
                        i.wrapping_mul(iStep),
                        i.wrapping_add(1 as ::core::ffi::c_uint).wrapping_mul(iStep),
                        *aCheck.offset(i as isize),
                        atoi(&raw mut g.zResult as *mut ::core::ffi::c_char),
                    );
                }
                i = i.wrapping_add(1);
            }
            speedtest1_end_test();
        }
        n = (g.szTest * 200 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        speedtest1_begin_test(
            120 as ::core::ffi::c_int,
            b"%d one-dimensional overlap slice queries\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"SELECT count(*) FROM rt1 WHERE y1>=?1 AND y0<=?2\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        iStep = mxCoord.wrapping_div(n);
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                i.wrapping_mul(iStep) as ::core::ffi::c_int,
            );
            sqlite3_bind_int(
                g.pStmt,
                2 as ::core::ffi::c_int,
                i.wrapping_add(1 as ::core::ffi::c_uint).wrapping_mul(iStep)
                    as ::core::ffi::c_int,
            );
            speedtest1_run();
            *aCheck.offset(i as isize) = atoi(
                &raw mut g.zResult as *mut ::core::ffi::c_char,
            );
            i = i.wrapping_add(1);
        }
        speedtest1_end_test();
        if g.bVerify != 0 {
            n = (g.szTest * 200 as ::core::ffi::c_int) as ::core::ffi::c_uint;
            speedtest1_begin_test(
                121 as ::core::ffi::c_int,
                b"Verify result from 1-D overlap slice queries\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            speedtest1_prepare(
                b"SELECT count(*) FROM z1 WHERE y1>=?1 AND y0<=?2\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            iStep = mxCoord.wrapping_div(n);
            i = 0 as ::core::ffi::c_uint;
            while i < n {
                sqlite3_bind_int(
                    g.pStmt,
                    1 as ::core::ffi::c_int,
                    i.wrapping_mul(iStep) as ::core::ffi::c_int,
                );
                sqlite3_bind_int(
                    g.pStmt,
                    2 as ::core::ffi::c_int,
                    i.wrapping_add(1 as ::core::ffi::c_uint).wrapping_mul(iStep)
                        as ::core::ffi::c_int,
                );
                speedtest1_run();
                if *aCheck.offset(i as isize)
                    != atoi(&raw mut g.zResult as *mut ::core::ffi::c_char)
                {
                    fatal_error(
                        b"Count disagree step %d: %d..%d.  %d vs %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        i,
                        i.wrapping_mul(iStep),
                        i.wrapping_add(1 as ::core::ffi::c_uint).wrapping_mul(iStep),
                        *aCheck.offset(i as isize),
                        atoi(&raw mut g.zResult as *mut ::core::ffi::c_char),
                    );
                }
                i = i.wrapping_add(1);
            }
            speedtest1_end_test();
        }
        n = (g.szTest * 200 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        speedtest1_begin_test(
            125 as ::core::ffi::c_int,
            b"%d custom geometry callback queries\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        sqlite3_rtree_geometry_callback(
            g.db,
            b"xslice\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                xsliceGeometryCallback
                    as unsafe extern "C" fn(
                        *mut sqlite3_rtree_geometry,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        speedtest1_prepare(
            b"SELECT count(*) FROM rt1 WHERE id MATCH xslice(?1,?2)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        iStep = mxCoord.wrapping_div(n);
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                i.wrapping_mul(iStep) as ::core::ffi::c_int,
            );
            sqlite3_bind_int(
                g.pStmt,
                2 as ::core::ffi::c_int,
                i.wrapping_add(1 as ::core::ffi::c_uint).wrapping_mul(iStep)
                    as ::core::ffi::c_int,
            );
            speedtest1_run();
            if *aCheck.offset(i as isize)
                != atoi(&raw mut g.zResult as *mut ::core::ffi::c_char)
            {
                fatal_error(
                    b"Count disagree step %d: %d..%d.  %d vs %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    i,
                    i.wrapping_mul(iStep),
                    i.wrapping_add(1 as ::core::ffi::c_uint).wrapping_mul(iStep),
                    *aCheck.offset(i as isize),
                    atoi(&raw mut g.zResult as *mut ::core::ffi::c_char),
                );
            }
            i = i.wrapping_add(1);
        }
        speedtest1_end_test();
        n = (g.szTest * 400 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        speedtest1_begin_test(
            130 as ::core::ffi::c_int,
            b"%d three-dimensional intersect box queries\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"SELECT count(*) FROM rt1 WHERE x1>=?1 AND x0<=?2 AND y1>=?1 AND y0<=?2 AND z1>=?1 AND z0<=?2\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        iStep = mxCoord.wrapping_div(n);
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                i.wrapping_mul(iStep) as ::core::ffi::c_int,
            );
            sqlite3_bind_int(
                g.pStmt,
                2 as ::core::ffi::c_int,
                i.wrapping_add(1 as ::core::ffi::c_uint).wrapping_mul(iStep)
                    as ::core::ffi::c_int,
            );
            speedtest1_run();
            *aCheck.offset(i as isize) = atoi(
                &raw mut g.zResult as *mut ::core::ffi::c_char,
            );
            i = i.wrapping_add(1);
        }
        speedtest1_end_test();
        n = (g.szTest * 500 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        speedtest1_begin_test(
            140 as ::core::ffi::c_int,
            b"%d rowid queries\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"SELECT * FROM rt1 WHERE id=?1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_uint;
        while i <= n {
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, i as ::core::ffi::c_int);
            speedtest1_run();
            i = i.wrapping_add(1);
        }
        speedtest1_end_test();
        n = (g.szTest * 50 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        speedtest1_begin_test(
            150 as ::core::ffi::c_int,
            b"%d UPDATEs using rowid\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"UPDATE rt1 SET x0=x0+100, x1=x1+100 WHERE id=?1\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_uint;
        while i <= n {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                i
                    .wrapping_mul(251 as ::core::ffi::c_uint)
                    .wrapping_rem(mxRowid)
                    .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            );
            speedtest1_run();
            i = i.wrapping_add(1);
        }
        speedtest1_end_test();
        n = (g.szTest * 5 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        speedtest1_begin_test(
            155 as ::core::ffi::c_int,
            b"%d UPDATEs using one-dimensional overlap\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"UPDATE rt1 SET x0=x0-100, x1=x1-100 WHERE y1>=?1 AND y0<=?1+5\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        iStep = mxCoord.wrapping_div(n);
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                i.wrapping_mul(iStep) as ::core::ffi::c_int,
            );
            speedtest1_run();
            *aCheck.offset(i as isize) = atoi(
                &raw mut g.zResult as *mut ::core::ffi::c_char,
            );
            i = i.wrapping_add(1);
        }
        speedtest1_end_test();
        n = (g.szTest * 50 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        speedtest1_begin_test(
            160 as ::core::ffi::c_int,
            b"%d DELETEs using rowid\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"DELETE FROM rt1 WHERE id=?1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        i = 1 as ::core::ffi::c_uint;
        while i <= n {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                i
                    .wrapping_mul(257 as ::core::ffi::c_uint)
                    .wrapping_rem(mxRowid)
                    .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            );
            speedtest1_run();
            i = i.wrapping_add(1);
        }
        speedtest1_end_test();
        n = (g.szTest * 5 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        speedtest1_begin_test(
            165 as ::core::ffi::c_int,
            b"%d DELETEs using one-dimensional overlap\0".as_ptr()
                as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"DELETE FROM rt1 WHERE y1>=?1 AND y0<=?1+5\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        iStep = mxCoord.wrapping_div(n);
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                i.wrapping_mul(iStep) as ::core::ffi::c_int,
            );
            speedtest1_run();
            *aCheck.offset(i as isize) = atoi(
                &raw mut g.zResult as *mut ::core::ffi::c_char,
            );
            i = i.wrapping_add(1);
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            170 as ::core::ffi::c_int,
            b"Restore deleted entries using INSERT OR IGNORE\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"INSERT OR IGNORE INTO rt1 SELECT * FROM z1\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testset_orm() {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        let mut n: ::core::ffi::c_uint = 0;
        let mut nRow: ::core::ffi::c_uint = 0;
        let mut x1: ::core::ffi::c_uint = 0;
        let mut len: ::core::ffi::c_uint = 0;
        let mut zNum: [::core::ffi::c_char; 2000] = [0; 2000];
        static mut zType: [::core::ffi::c_char; 120] = unsafe {
            ::core::mem::transmute::<
                [u8; 120],
                [::core::ffi::c_char; 120],
            >(
                *b"IBBIIITIVVITBTBFBFITTFBTBVBVIFTBBFITFFVBIFIVBVVVBTVTIBBFFIVIBTBTVTTFTVTVFFIITIFBITFTTFFFVBIIBTTITFTFFVVVFIIITVBBVFFTVVB\0",
            )
        };
        n = (g.szTest * 250 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        nRow = n;
        speedtest1_begin_test(
            100 as ::core::ffi::c_int,
            b"Fill %d rows\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_exec(
            b"BEGIN;CREATE TABLE ZLOOKSLIKECOREDATA (  ZPK INTEGER PRIMARY KEY,  ZTERMFITTINGHOUSINGCOMMAND INTEGER,  ZBRIEFGOBYDODGERHEIGHT BLOB,  ZCAPABLETRIPDOORALMOND BLOB,  ZDEPOSITPAIRCOLLEGECOMET INTEGER,  ZFRAMEENTERSIMPLEMOUTH INTEGER,  ZHOPEFULGATEHOLECHALK INTEGER,  ZSLEEPYUSERGRANDBOWL TIMESTAMP,  ZDEWPEACHCAREERCELERY INTEGER,  ZHANGERLITHIUMDINNERMEET VARCHAR,  ZCLUBRELEASELIZARDADVICE VARCHAR,  ZCHARGECLICKHUMANEHIRE INTEGER,  ZFINGERDUEPIZZAOPTION TIMESTAMP,  ZFLYINGDOCTORTABLEMELODY BLOB,  ZLONGFINLEAVEIMAGEOIL TIMESTAMP,  ZFAMILYVISUALOWNERMATTER BLOB,  ZGOLDYOUNGINITIALNOSE FLOAT,  ZCAUSESALAMITERMCYAN BLOB,  ZSPREADMOTORBISCUITBACON FLOAT,  ZGIFTICEFISHGLUEHAIR INTEGER,  ZNOTICEPEARPOLICYJUICE TIMESTAMP,  ZBANKBUFFALORECOVERORBIT TIMESTAMP,  ZLONGDIETESSAYNATURE FLOAT,  ZACTIONRANGEELEGANTNEUTRON BLOB,  ZCADETBRIGHTPLANETBANK TIMESTAMP,  ZAIRFORGIVEHEADFROG BLOB,  ZSHARKJUSTFRUITMOVIE VARCHAR,  ZFARMERMORNINGMIRRORCONCERN BLOB,  ZWOODPOETRYCOBBLERBENCH VARCHAR,  ZHAFNIUMSCRIPTSALADMOTOR INTEGER,  ZPROBLEMCLUBPOPOVERJELLY FLOAT,  ZEIGHTLEADERWORKERMOST TIMESTAMP,  ZGLASSRESERVEBARIUMMEAL BLOB,  ZCLAMBITARUGULAFAJITA BLOB,  ZDECADEJOYOUSWAVEHABIT FLOAT,  ZCOMPANYSUMMERFIBERELF INTEGER,  ZTREATTESTQUILLCHARGE TIMESTAMP,  ZBROWBALANCEKEYCHOWDER FLOAT,  ZPEACHCOPPERDINNERLAKE FLOAT,  ZDRYWALLBEYONDBROWNBOWL VARCHAR,  ZBELLYCRASHITEMLACK BLOB,  ZTENNISCYCLEBILLOFFICER INTEGER,  ZMALLEQUIPTHANKSGLUE FLOAT,  ZMISSREPLYHUMANLIVING INTEGER,  ZKIWIVISUALPRIDEAPPLE VARCHAR,  ZWISHHITSKINMOTOR BLOB,  ZCALMRACCOONPROGRAMDEBIT VARCHAR,  ZSHINYASSISTLIVINGCRAB VARCHAR,  ZRESOLVEWRISTWRAPAPPLE VARCHAR,  ZAPPEALSIMPLESECONDHOUSING BLOB,  ZCORNERANCHORTAPEDIVER TIMESTAMP,  ZMEMORYREQUESTSOURCEBIG VARCHAR,  ZTRYFACTKEEPMILK TIMESTAMP,  ZDIVERPAINTLEATHEREASY INTEGER,  ZSORTMISTYQUOTECABBAGE BLOB,  ZTUNEGASBUFFALOCAPITAL BLOB,  ZFILLSTOPLAWJOYFUL FLOAT,  ZSTEELCAREFULPLATENUMBER FLOAT,  ZGIVEVIVIDDIVINEMEANING INTEGER,  ZTREATPACKFUTURECONVERT VARCHAR,  ZCALMLYGEMFINISHEFFECT INTEGER,  ZCABBAGESOCKEASEMINUTE BLOB,  ZPLANETFAMILYPUREMEMORY TIMESTAMP,  ZMERRYCRACKTRAINLEADER BLOB,  ZMINORWAYPAPERCLASSY TIMESTAMP,  ZEAGLELINEMINEMAIL VARCHAR,  ZRESORTYARDGREENLET TIMESTAMP,  ZYARDOREGANOVIVIDJEWEL TIMESTAMP,  ZPURECAKEVIVIDNEATLY FLOAT,  ZASKCONTACTMONITORFUN TIMESTAMP,  ZMOVEWHOGAMMAINCH VARCHAR,  ZLETTUCEBIRDMEETDEBATE TIMESTAMP,  ZGENENATURALHEARINGKITE VARCHAR,  ZMUFFINDRYERDRAWFORTUNE FLOAT,  ZGRAYSURVEYWIRELOVE FLOAT,  ZPLIERSPRINTASKOREGANO INTEGER,  ZTRAVELDRIVERCONTESTLILY INTEGER,  ZHUMORSPICESANDKIDNEY TIMESTAMP,  ZARSENICSAMPLEWAITMUON INTEGER,  ZLACEADDRESSGROUNDCAREFUL FLOAT,  ZBAMBOOMESSWASABIEVENING BLOB,  ZONERELEASEAVERAGENURSE INTEGER,  ZRADIANTWHENTRYCARD TIMESTAMP,  ZREWARDINSIDEMANGOINTENSE FLOAT,  ZNEATSTEWPARTIRON TIMESTAMP,  ZOUTSIDEPEAHENCOUNTICE TIMESTAMP,  ZCREAMEVENINGLIPBRANCH FLOAT,  ZWHALEMATHAVOCADOCOPPER FLOAT,  ZLIFEUSELEAFYBELL FLOAT,  ZWEALTHLINENGLEEFULDAY VARCHAR,  ZFACEINVITETALKGOLD BLOB,  ZWESTAMOUNTAFFECTHEARING INTEGER,  ZDELAYOUTCOMEHORNAGENCY INTEGER,  ZBIGTHINKCONVERTECONOMY BLOB,  ZBASEGOUDAREGULARFORGIVE TIMESTAMP,  ZPATTERNCLORINEGRANDCOLBY TIMESTAMP,  ZCYANBASEFEEDADROIT INTEGER,  ZCARRYFLOORMINNOWDRAGON TIMESTAMP,  ZIMAGEPENCILOTHERBOTTOM FLOAT,  ZXENONFLIGHTPALEAPPLE TIMESTAMP,  ZHERRINGJOKEFEATUREHOPEFUL FLOAT,  ZCAPYEARLYRIVETBRUSH FLOAT,  ZAGEREEDFROGBASKET VARCHAR,  ZUSUALBODYHALIBUTDIAMOND VARCHAR,  ZFOOTTAPWORDENTRY VARCHAR,  ZDISHKEEPBLESTMONITOR FLOAT,  ZBROADABLESOLIDCASUAL INTEGER,  ZSQUAREGLEEFULCHILDLIGHT INTEGER,  ZHOLIDAYHEADPONYDETAIL INTEGER,  ZGENERALRESORTSKYOPEN TIMESTAMP,  ZGLADSPRAYKIDNEYGUPPY VARCHAR,  ZSWIMHEAVYMENTIONKIND BLOB,  ZMESSYSULFURDREAMFESTIVE BLOB,  ZSKYSKYCLASSICBRIEF VARCHAR,  ZDILLASKHOKILEMON FLOAT,  ZJUNIORSHOWPRESSNOVA FLOAT,  ZSIZETOEAWARDFRESH TIMESTAMP,  ZKEYFAILAPRICOTMETAL VARCHAR,  ZHANDYREPAIRPROTONAIRPORT VARCHAR,  ZPOSTPROTEINHANDLEACTOR BLOB);\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"INSERT INTO ZLOOKSLIKECOREDATA(ZPK,ZAIRFORGIVEHEADFROG,ZGIFTICEFISHGLUEHAIR,ZDELAYOUTCOMEHORNAGENCY,ZSLEEPYUSERGRANDBOWL,ZGLASSRESERVEBARIUMMEAL,ZBRIEFGOBYDODGERHEIGHT,ZBAMBOOMESSWASABIEVENING,ZFARMERMORNINGMIRRORCONCERN,ZTREATPACKFUTURECONVERT,ZCAUSESALAMITERMCYAN,ZCALMRACCOONPROGRAMDEBIT,ZHOLIDAYHEADPONYDETAIL,ZWOODPOETRYCOBBLERBENCH,ZHAFNIUMSCRIPTSALADMOTOR,ZUSUALBODYHALIBUTDIAMOND,ZOUTSIDEPEAHENCOUNTICE,ZDIVERPAINTLEATHEREASY,ZWESTAMOUNTAFFECTHEARING,ZSIZETOEAWARDFRESH,ZDEWPEACHCAREERCELERY,ZSTEELCAREFULPLATENUMBER,ZCYANBASEFEEDADROIT,ZCALMLYGEMFINISHEFFECT,ZHANDYREPAIRPROTONAIRPORT,ZGENENATURALHEARINGKITE,ZBROADABLESOLIDCASUAL,ZPOSTPROTEINHANDLEACTOR,ZLACEADDRESSGROUNDCAREFUL,ZIMAGEPENCILOTHERBOTTOM,ZPROBLEMCLUBPOPOVERJELLY,ZPATTERNCLORINEGRANDCOLBY,ZNEATSTEWPARTIRON,ZAPPEALSIMPLESECONDHOUSING,ZMOVEWHOGAMMAINCH,ZTENNISCYCLEBILLOFFICER,ZSHARKJUSTFRUITMOVIE,ZKEYFAILAPRICOTMETAL,ZCOMPANYSUMMERFIBERELF,ZTERMFITTINGHOUSINGCOMMAND,ZRESORTYARDGREENLET,ZCABBAGESOCKEASEMINUTE,ZSQUAREGLEEFULCHILDLIGHT,ZONERELEASEAVERAGENURSE,ZBIGTHINKCONVERTECONOMY,ZPLIERSPRINTASKOREGANO,ZDECADEJOYOUSWAVEHABIT,ZDRYWALLBEYONDBROWNBOWL,ZCLUBRELEASELIZARDADVICE,ZWHALEMATHAVOCADOCOPPER,ZBELLYCRASHITEMLACK,ZLETTUCEBIRDMEETDEBATE,ZCAPABLETRIPDOORALMOND,ZRADIANTWHENTRYCARD,ZCAPYEARLYRIVETBRUSH,ZAGEREEDFROGBASKET,ZSWIMHEAVYMENTIONKIND,ZTRAVELDRIVERCONTESTLILY,ZGLADSPRAYKIDNEYGUPPY,ZBANKBUFFALORECOVERORBIT,ZFINGERDUEPIZZAOPTION,ZCLAMBITARUGULAFAJITA,ZLONGFINLEAVEIMAGEOIL,ZLONGDIETESSAYNATURE,ZJUNIORSHOWPRESSNOVA,ZHOPEFULGATEHOLECHALK,ZDEPOSITPAIRCOLLEGECOMET,ZWEALTHLINENGLEEFULDAY,ZFILLSTOPLAWJOYFUL,ZTUNEGASBUFFALOCAPITAL,ZGRAYSURVEYWIRELOVE,ZCORNERANCHORTAPEDIVER,ZREWARDINSIDEMANGOINTENSE,ZCADETBRIGHTPLANETBANK,ZPLANETFAMILYPUREMEMORY,ZTREATTESTQUILLCHARGE,ZCREAMEVENINGLIPBRANCH,ZSKYSKYCLASSICBRIEF,ZARSENICSAMPLEWAITMUON,ZBROWBALANCEKEYCHOWDER,ZFLYINGDOCTORTABLEMELODY,ZHANGERLITHIUMDINNERMEET,ZNOTICEPEARPOLICYJUICE,ZSHINYASSISTLIVINGCRAB,ZLIFEUSELEAFYBELL,ZFACEINVITETALKGOLD,ZGENERALRESORTSKYOPEN,ZPURECAKEVIVIDNEATLY,ZKIWIVISUALPRIDEAPPLE,ZMESSYSULFURDREAMFESTIVE,ZCHARGECLICKHUMANEHIRE,ZHERRINGJOKEFEATUREHOPEFUL,ZYARDOREGANOVIVIDJEWEL,ZFOOTTAPWORDENTRY,ZWISHHITSKINMOTOR,ZBASEGOUDAREGULARFORGIVE,ZMUFFINDRYERDRAWFORTUNE,ZACTIONRANGEELEGANTNEUTRON,ZTRYFACTKEEPMILK,ZPEACHCOPPERDINNERLAKE,ZFRAMEENTERSIMPLEMOUTH,ZMERRYCRACKTRAINLEADER,ZMEMORYREQUESTSOURCEBIG,ZCARRYFLOORMINNOWDRAGON,ZMINORWAYPAPERCLASSY,ZDILLASKHOKILEMON,ZRESOLVEWRISTWRAPAPPLE,ZASKCONTACTMONITORFUN,ZGIVEVIVIDDIVINEMEANING,ZEIGHTLEADERWORKERMOST,ZMISSREPLYHUMANLIVING,ZXENONFLIGHTPALEAPPLE,ZSORTMISTYQUOTECABBAGE,ZEAGLELINEMINEMAIL,ZFAMILYVISUALOWNERMATTER,ZSPREADMOTORBISCUITBACON,ZDISHKEEPBLESTMONITOR,ZMALLEQUIPTHANKSGLUE,ZGOLDYOUNGINITIALNOSE,ZHUMORSPICESANDKIDNEY)VALUES(?1,?26,?20,?93,?8,?33,?3,?81,?28,?60,?18,?47,?109,?29,?30,?104,?86,?54,?92,?117,?9,?58,?97,?61,?119,?73,?107,?120,?80,?99,?31,?96,?85,?50,?71,?42,?27,?118,?36,?2,?67,?62,?108,?82,?94,?76,?35,?40,?11,?88,?41,?72,?4,?83,?102,?103,?112,?77,?111,?22,?13,?34,?15,?23,?116,?7,?5,?90,?57,?56,?75,?51,?84,?25,?63,?37,?87,?114,?79,?38,?14,?10,?21,?48,?89,?91,?110,?69,?45,?113,?12,?101,?68,?105,?46,?95,?74,?24,?53,?39,?6,?64,?52,?98,?65,?115,?49,?70,?59,?32,?44,?100,?55,?66,?16,?19,?106,?43,?17,?78);\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            x1 = speedtest1_random();
            speedtest1_numbername(
                x1.wrapping_rem(1000 as ::core::ffi::c_uint),
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            len = strlen(&raw mut zNum as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                as ::core::ffi::c_uint;
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                (i ^ 0xf as ::core::ffi::c_uint) as ::core::ffi::c_int,
            );
            j = 0 as ::core::ffi::c_uint;
            while zType[j as usize] != 0 {
                match zType[j as usize] as ::core::ffi::c_int {
                    73 | 84 => {
                        sqlite3_bind_int64(
                            g.pStmt,
                            j.wrapping_add(2 as ::core::ffi::c_uint)
                                as ::core::ffi::c_int,
                            x1 as sqlite3_int64,
                        );
                    }
                    70 => {
                        sqlite3_bind_double(
                            g.pStmt,
                            j.wrapping_add(2 as ::core::ffi::c_uint)
                                as ::core::ffi::c_int,
                            x1 as ::core::ffi::c_double,
                        );
                    }
                    86 | 66 => {
                        sqlite3_bind_text64(
                            g.pStmt,
                            j.wrapping_add(2 as ::core::ffi::c_uint)
                                as ::core::ffi::c_int,
                            &raw mut zNum as *mut ::core::ffi::c_char,
                            len as sqlite3_uint64,
                            SQLITE_STATIC,
                            SQLITE_UTF8 as ::core::ffi::c_uchar,
                        );
                    }
                    _ => {}
                }
                j = j.wrapping_add(1);
            }
            speedtest1_run();
            i = i.wrapping_add(1);
        }
        speedtest1_exec(b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        n = (g.szTest * 250 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        speedtest1_begin_test(
            110 as ::core::ffi::c_int,
            b"Query %d rows by rowid\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        speedtest1_prepare(
            b"SELECT ZCYANBASEFEEDADROIT,ZJUNIORSHOWPRESSNOVA,ZCAUSESALAMITERMCYAN,ZHOPEFULGATEHOLECHALK,ZHUMORSPICESANDKIDNEY,ZSWIMHEAVYMENTIONKIND,ZMOVEWHOGAMMAINCH,ZAPPEALSIMPLESECONDHOUSING,ZHAFNIUMSCRIPTSALADMOTOR,ZNEATSTEWPARTIRON,ZLONGFINLEAVEIMAGEOIL,ZDEWPEACHCAREERCELERY,ZXENONFLIGHTPALEAPPLE,ZCALMRACCOONPROGRAMDEBIT,ZUSUALBODYHALIBUTDIAMOND,ZTRYFACTKEEPMILK,ZWEALTHLINENGLEEFULDAY,ZLONGDIETESSAYNATURE,ZLIFEUSELEAFYBELL,ZTREATPACKFUTURECONVERT,ZMEMORYREQUESTSOURCEBIG,ZYARDOREGANOVIVIDJEWEL,ZDEPOSITPAIRCOLLEGECOMET,ZSLEEPYUSERGRANDBOWL,ZBRIEFGOBYDODGERHEIGHT,ZCLUBRELEASELIZARDADVICE,ZCAPABLETRIPDOORALMOND,ZDRYWALLBEYONDBROWNBOWL,ZASKCONTACTMONITORFUN,ZKIWIVISUALPRIDEAPPLE,ZNOTICEPEARPOLICYJUICE,ZPEACHCOPPERDINNERLAKE,ZSTEELCAREFULPLATENUMBER,ZGLADSPRAYKIDNEYGUPPY,ZCOMPANYSUMMERFIBERELF,ZTENNISCYCLEBILLOFFICER,ZIMAGEPENCILOTHERBOTTOM,ZWESTAMOUNTAFFECTHEARING,ZDIVERPAINTLEATHEREASY,ZSKYSKYCLASSICBRIEF,ZMESSYSULFURDREAMFESTIVE,ZMERRYCRACKTRAINLEADER,ZBROADABLESOLIDCASUAL,ZGLASSRESERVEBARIUMMEAL,ZTUNEGASBUFFALOCAPITAL,ZBANKBUFFALORECOVERORBIT,ZTREATTESTQUILLCHARGE,ZBAMBOOMESSWASABIEVENING,ZREWARDINSIDEMANGOINTENSE,ZEAGLELINEMINEMAIL,ZCALMLYGEMFINISHEFFECT,ZKEYFAILAPRICOTMETAL,ZFINGERDUEPIZZAOPTION,ZCADETBRIGHTPLANETBANK,ZGOLDYOUNGINITIALNOSE,ZMISSREPLYHUMANLIVING,ZEIGHTLEADERWORKERMOST,ZFRAMEENTERSIMPLEMOUTH,ZBIGTHINKCONVERTECONOMY,ZFACEINVITETALKGOLD,ZPOSTPROTEINHANDLEACTOR,ZHERRINGJOKEFEATUREHOPEFUL,ZCABBAGESOCKEASEMINUTE,ZMUFFINDRYERDRAWFORTUNE,ZPROBLEMCLUBPOPOVERJELLY,ZGIVEVIVIDDIVINEMEANING,ZGENENATURALHEARINGKITE,ZGENERALRESORTSKYOPEN,ZLETTUCEBIRDMEETDEBATE,ZBASEGOUDAREGULARFORGIVE,ZCHARGECLICKHUMANEHIRE,ZPLANETFAMILYPUREMEMORY,ZMINORWAYPAPERCLASSY,ZCAPYEARLYRIVETBRUSH,ZSIZETOEAWARDFRESH,ZARSENICSAMPLEWAITMUON,ZSQUAREGLEEFULCHILDLIGHT,ZSHINYASSISTLIVINGCRAB,ZCORNERANCHORTAPEDIVER,ZDECADEJOYOUSWAVEHABIT,ZTRAVELDRIVERCONTESTLILY,ZFLYINGDOCTORTABLEMELODY,ZSHARKJUSTFRUITMOVIE,ZFAMILYVISUALOWNERMATTER,ZFARMERMORNINGMIRRORCONCERN,ZGIFTICEFISHGLUEHAIR,ZOUTSIDEPEAHENCOUNTICE,ZSPREADMOTORBISCUITBACON,ZWISHHITSKINMOTOR,ZHOLIDAYHEADPONYDETAIL,ZWOODPOETRYCOBBLERBENCH,ZAIRFORGIVEHEADFROG,ZBROWBALANCEKEYCHOWDER,ZDISHKEEPBLESTMONITOR,ZCLAMBITARUGULAFAJITA,ZPLIERSPRINTASKOREGANO,ZRADIANTWHENTRYCARD,ZDELAYOUTCOMEHORNAGENCY,ZPURECAKEVIVIDNEATLY,ZPATTERNCLORINEGRANDCOLBY,ZHANDYREPAIRPROTONAIRPORT,ZAGEREEDFROGBASKET,ZSORTMISTYQUOTECABBAGE,ZFOOTTAPWORDENTRY,ZRESOLVEWRISTWRAPAPPLE,ZDILLASKHOKILEMON,ZFILLSTOPLAWJOYFUL,ZACTIONRANGEELEGANTNEUTRON,ZRESORTYARDGREENLET,ZCREAMEVENINGLIPBRANCH,ZWHALEMATHAVOCADOCOPPER,ZGRAYSURVEYWIRELOVE,ZBELLYCRASHITEMLACK,ZHANGERLITHIUMDINNERMEET,ZCARRYFLOORMINNOWDRAGON,ZMALLEQUIPTHANKSGLUE,ZTERMFITTINGHOUSINGCOMMAND,ZONERELEASEAVERAGENURSE,ZLACEADDRESSGROUNDCAREFUL FROM ZLOOKSLIKECOREDATA WHERE ZPK=?1;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            x1 = speedtest1_random().wrapping_rem(nRow);
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, x1 as ::core::ffi::c_int);
            speedtest1_run();
            i = i.wrapping_add(1);
        }
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testset_trigger() {
    unsafe {
        let mut jj: ::core::ffi::c_int = 0;
        let mut ii: ::core::ffi::c_int = 0;
        let mut zNum: [::core::ffi::c_char; 2000] = [0; 2000];
        let NROW: ::core::ffi::c_int = 500 as ::core::ffi::c_int * g.szTest;
        let NROW2: ::core::ffi::c_int = 100 as ::core::ffi::c_int * g.szTest;
        speedtest1_exec(
            b"BEGIN;CREATE TABLE z1(rowid INTEGER PRIMARY KEY, i INTEGER, t TEXT);CREATE TABLE z2(rowid INTEGER PRIMARY KEY, i INTEGER, t TEXT);CREATE TABLE t3(rowid INTEGER PRIMARY KEY, i INTEGER, t TEXT);CREATE VIEW v1 AS SELECT rowid, i, t FROM z1;CREATE VIEW v2 AS SELECT rowid, i, t FROM z2;CREATE VIEW v3 AS SELECT rowid, i, t FROM t3;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        jj = 1 as ::core::ffi::c_int;
        while jj <= 3 as ::core::ffi::c_int {
            speedtest1_prepare(
                b"INSERT INTO t%d VALUES(NULL,?1,?2)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                jj,
            );
            ii = 0 as ::core::ffi::c_int;
            while ii < NROW {
                let mut x1: ::core::ffi::c_int = speedtest1_random()
                    .wrapping_rem(NROW as ::core::ffi::c_uint) as ::core::ffi::c_int;
                speedtest1_numbername(
                    x1 as ::core::ffi::c_uint,
                    &raw mut zNum as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                        as ::core::ffi::c_int,
                );
                sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, x1);
                sqlite3_bind_text(
                    g.pStmt,
                    2 as ::core::ffi::c_int,
                    &raw mut zNum as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                speedtest1_run();
                ii += 1;
            }
            jj += 1;
        }
        speedtest1_exec(
            b"CREATE INDEX i1 ON z1(t);CREATE INDEX i2 ON z2(t);CREATE INDEX i3 ON t3(t);COMMIT;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_begin_test(
            100 as ::core::ffi::c_int,
            b"speed4p-join1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"SELECT * FROM z1, z2, t3 WHERE z1.oid = z2.oid AND z2.oid = t3.oid\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_run();
        speedtest1_end_test();
        speedtest1_begin_test(
            110 as ::core::ffi::c_int,
            b"speed4p-join2\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"SELECT * FROM z1, z2, t3 WHERE z1.t = z2.t AND z2.t = t3.t\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_run();
        speedtest1_end_test();
        speedtest1_begin_test(
            120 as ::core::ffi::c_int,
            b"speed4p-view1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        jj = 1 as ::core::ffi::c_int;
        while jj <= 3 as ::core::ffi::c_int {
            speedtest1_prepare(
                b"SELECT * FROM v%d WHERE rowid = ?\0".as_ptr()
                    as *const ::core::ffi::c_char,
                jj,
            );
            ii = 0 as ::core::ffi::c_int;
            while ii < NROW2 {
                sqlite3_bind_int(
                    g.pStmt,
                    1 as ::core::ffi::c_int,
                    ii * 3 as ::core::ffi::c_int,
                );
                speedtest1_run();
                ii += 3 as ::core::ffi::c_int;
            }
            jj += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            130 as ::core::ffi::c_int,
            b"speed4p-table1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        jj = 1 as ::core::ffi::c_int;
        while jj <= 3 as ::core::ffi::c_int {
            speedtest1_prepare(
                b"SELECT * FROM t%d WHERE rowid = ?\0".as_ptr()
                    as *const ::core::ffi::c_char,
                jj,
            );
            ii = 0 as ::core::ffi::c_int;
            while ii < NROW2 {
                sqlite3_bind_int(
                    g.pStmt,
                    1 as ::core::ffi::c_int,
                    ii * 3 as ::core::ffi::c_int,
                );
                speedtest1_run();
                ii += 3 as ::core::ffi::c_int;
            }
            jj += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            140 as ::core::ffi::c_int,
            b"speed4p-table1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        jj = 1 as ::core::ffi::c_int;
        while jj <= 3 as ::core::ffi::c_int {
            speedtest1_prepare(
                b"SELECT * FROM t%d WHERE rowid = ?\0".as_ptr()
                    as *const ::core::ffi::c_char,
                jj,
            );
            ii = 0 as ::core::ffi::c_int;
            while ii < NROW2 {
                sqlite3_bind_int(
                    g.pStmt,
                    1 as ::core::ffi::c_int,
                    ii * 3 as ::core::ffi::c_int,
                );
                speedtest1_run();
                ii += 3 as ::core::ffi::c_int;
            }
            jj += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            150 as ::core::ffi::c_int,
            b"speed4p-subselect1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"SELECT (SELECT t FROM z1 WHERE rowid = ?1),(SELECT t FROM z2 WHERE rowid = ?1),(SELECT t FROM t3 WHERE rowid = ?1)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        jj = 0 as ::core::ffi::c_int;
        while jj < NROW2 {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                jj * 3 as ::core::ffi::c_int,
            );
            speedtest1_run();
            jj += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            160 as ::core::ffi::c_int,
            b"speed4p-rowid-update\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_prepare(
            b"UPDATE z1 SET i=i+1 WHERE rowid=?1\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        jj = 0 as ::core::ffi::c_int;
        while jj < NROW2 {
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, jj);
            speedtest1_run();
            jj += 1;
        }
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_end_test();
        speedtest1_exec(
            b"CREATE TABLE t5(t TEXT PRIMARY KEY, i INTEGER);\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_begin_test(
            170 as ::core::ffi::c_int,
            b"speed4p-insert-ignore\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"INSERT OR IGNORE INTO t5 SELECT t, i FROM z1\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_exec(
            b"CREATE TABLE log(op TEXT, r INTEGER, i INTEGER, t TEXT);CREATE TABLE t4(rowid INTEGER PRIMARY KEY, i INTEGER, t TEXT);CREATE TRIGGER t4_trigger1 AFTER INSERT ON t4 BEGIN  INSERT INTO log VALUES('INSERT INTO t4', new.rowid, new.i, new.t);END;CREATE TRIGGER t4_trigger2 AFTER UPDATE ON t4 BEGIN  INSERT INTO log VALUES('UPDATE OF t4', new.rowid, new.i, new.t);END;CREATE TRIGGER t4_trigger3 AFTER DELETE ON t4 BEGIN  INSERT INTO log VALUES('DELETE OF t4', old.rowid, old.i, old.t);END;BEGIN;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_begin_test(
            180 as ::core::ffi::c_int,
            b"speed4p-trigger1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"INSERT INTO t4 VALUES(NULL, ?1, ?2)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        jj = 0 as ::core::ffi::c_int;
        while jj < NROW2 {
            speedtest1_numbername(
                jj as ::core::ffi::c_uint,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, jj);
            sqlite3_bind_text(
                g.pStmt,
                2 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            jj += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            190 as ::core::ffi::c_int,
            b"speed4p-trigger2\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"UPDATE t4 SET i = ?1, t = ?2 WHERE rowid = ?3\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        jj = 1 as ::core::ffi::c_int;
        while jj <= NROW2 * 2 as ::core::ffi::c_int {
            speedtest1_numbername(
                (jj * 2 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                jj * 2 as ::core::ffi::c_int,
            );
            sqlite3_bind_text(
                g.pStmt,
                2 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            sqlite3_bind_int(g.pStmt, 3 as ::core::ffi::c_int, jj);
            speedtest1_run();
            jj += 2 as ::core::ffi::c_int;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            200 as ::core::ffi::c_int,
            b"speed4p-trigger3\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"DELETE FROM t4 WHERE rowid = ?1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        jj = 1 as ::core::ffi::c_int;
        while jj <= NROW2 * 2 as ::core::ffi::c_int {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                jj * 2 as ::core::ffi::c_int,
            );
            speedtest1_run();
            jj += 2 as ::core::ffi::c_int;
        }
        speedtest1_end_test();
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
        speedtest1_exec(
            b"DROP TABLE t4;DROP TABLE log;VACUUM;CREATE TABLE t4(rowid INTEGER PRIMARY KEY, i INTEGER, t TEXT);BEGIN;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_begin_test(
            210 as ::core::ffi::c_int,
            b"speed4p-notrigger1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"INSERT INTO t4 VALUES(NULL, ?1, ?2)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        jj = 0 as ::core::ffi::c_int;
        while jj < NROW2 {
            speedtest1_numbername(
                jj as ::core::ffi::c_uint,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            sqlite3_bind_int(g.pStmt, 1 as ::core::ffi::c_int, jj);
            sqlite3_bind_text(
                g.pStmt,
                2 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            speedtest1_run();
            jj += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            210 as ::core::ffi::c_int,
            b"speed4p-notrigger2\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"UPDATE t4 SET i = ?1, t = ?2 WHERE rowid = ?3\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        jj = 1 as ::core::ffi::c_int;
        while jj <= NROW2 * 2 as ::core::ffi::c_int {
            speedtest1_numbername(
                (jj * 2 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                jj * 2 as ::core::ffi::c_int,
            );
            sqlite3_bind_text(
                g.pStmt,
                2 as ::core::ffi::c_int,
                &raw mut zNum as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            sqlite3_bind_int(g.pStmt, 3 as ::core::ffi::c_int, jj);
            speedtest1_run();
            jj += 2 as ::core::ffi::c_int;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            220 as ::core::ffi::c_int,
            b"speed4p-notrigger3\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_prepare(
            b"DELETE FROM t4 WHERE rowid = ?1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        jj = 1 as ::core::ffi::c_int;
        while jj <= NROW2 * 2 as ::core::ffi::c_int {
            sqlite3_bind_int(
                g.pStmt,
                1 as ::core::ffi::c_int,
                jj * 2 as ::core::ffi::c_int,
            );
            speedtest1_run();
            jj += 2 as ::core::ffi::c_int;
        }
        speedtest1_end_test();
        speedtest1_exec(b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testset_debug1() {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut n: ::core::ffi::c_uint = 0;
        let mut x1: ::core::ffi::c_uint = 0;
        let mut x2: ::core::ffi::c_uint = 0;
        let mut zNum: [::core::ffi::c_char; 2000] = [0; 2000];
        n = g.szTest as ::core::ffi::c_uint;
        i = 1 as ::core::ffi::c_uint;
        while i <= n {
            x1 = swizzle(i, n);
            x2 = swizzle(x1, n);
            speedtest1_numbername(
                x1,
                &raw mut zNum as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2000]>()
                    as ::core::ffi::c_int,
            );
            printf(
                b"%5d %5d %5d %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                i,
                x1,
                x2,
                &raw mut zNum as *mut ::core::ffi::c_char,
            );
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testset_json() {
    unsafe {
        let mut r: ::core::ffi::c_uint = 0x12345678 as ::core::ffi::c_int
            as ::core::ffi::c_uint;
        sqlite3_test_control(SQLITE_TESTCTRL_PRNG_SEED, r, g.db);
        speedtest1_begin_test(
            100 as ::core::ffi::c_int,
            b"table J1 is %d rows of JSONB\0".as_ptr() as *const ::core::ffi::c_char,
            g.szTest * 5 as ::core::ffi::c_int,
        );
        speedtest1_exec(
            b"CREATE TABLE j1(x JSONB);\nWITH RECURSIVE\n  jval(n,j) AS (\n    VALUES(0,'{}'),(1,'[]'),(2,'true'),(3,'false'),(4,'null'),\n          (5,'{x:1,y:2}'),(6,'0.0'),(7,'3.14159'),(8,'-99.9'),\n          (9,'[1,2,\"\\n\\u2192\\\"\\u2190\",4]')\n  ),\n  c(x) AS (VALUES(1) UNION ALL SELECT x+1 FROM c WHERE x<26*26-1),\n  array1(y) AS MATERIALIZED (\n    SELECT jsonb_group_array(\n      jsonb_object('x',x,\n                  'y',jsonb(coalesce(j,random()%%10000)),\n                  'z',hex(randomblob(50)))\n    )\n    FROM c LEFT JOIN jval ON (x%%20)=n\n  ),\n  object1(z) AS MATERIALIZED (\n    SELECT jsonb_group_object(char(0x61+x%%26,0x61+(x/26)%%26),\n                      jsonb( coalesce(j,random()%%10000)))\n      FROM c LEFT JOIN jval ON (x%%20)=n\n  ),\n  c2(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM c2 WHERE n<%d)\nINSERT INTO j1(x)\n  SELECT jsonb_object('a',n,'b',n+10000,'c',jsonb(y),'d',jsonb(z),\n                     'e',n+20000,'f',n+30000)\n    FROM array1, object1, c2;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            g.szTest * 5 as ::core::ffi::c_int,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            110 as ::core::ffi::c_int,
            b"table J2 is %d rows from J1 converted to text\0".as_ptr()
                as *const ::core::ffi::c_char,
            g.szTest,
        );
        speedtest1_exec(
            b"CREATE TABLE j2(x JSON TEXT);\nINSERT INTO j2(x) SELECT json(x) FROM j1 LIMIT %d\0"
                .as_ptr() as *const ::core::ffi::c_char,
            g.szTest,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            120 as ::core::ffi::c_int,
            b"create indexes on JSON expressions on J1\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"BEGIN;\nCREATE INDEX j1x1 ON j1(x->>'a');\nCREATE INDEX j1x2 ON j1(x->>'b');\nCREATE INDEX j1x3 ON j1(x->>'f');\nCOMMIT;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            130 as ::core::ffi::c_int,
            b"create indexes on JSON expressions on J2\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"BEGIN;\nCREATE INDEX j2x1 ON j2(x->>'a');\nCREATE INDEX j2x2 ON j2(x->>'b');\nCREATE INDEX j2x3 ON j2(x->>'f');\nCOMMIT;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            140 as ::core::ffi::c_int,
            b"queries against J1\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"WITH c(n) AS (VALUES(0) UNION ALL SELECT n+1 FROM c WHERE n<7)\n  SELECT sum(x->>format('$.c[%%d].x',n)) FROM c, j1;\nWITH c(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM c WHERE n<5)\n  SELECT sum(x->>format('$.\"c\"[#-%%d].y',n)) FROM c, j1;\nSELECT sum(x->>'$.d.ez' + x->>'$.d.\"xz\"' + x->>'a' + x->>'$.c[10].y') FROM j1;\nSELECT x->>'$.d.tz[2]', x->'$.d.tz' FROM j1;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            141 as ::core::ffi::c_int,
            b"queries involving json_type()\0".as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"WITH c(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM c WHERE n<20)\n  SELECT json_type(x,format('$.c[#-%%d].y',n)), count(*)\n    FROM c, j1\n   WHERE j1.rowid=1\n   GROUP BY 1 ORDER BY 2;\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            150 as ::core::ffi::c_int,
            b"json_insert()/set()/remove() on every row of J1\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"BEGIN;\nUPDATE j1 SET x=jsonb_insert(x,'$.g',(x->>'f')+1,'$.h',3.14159,'$.i','hello',\n                               '$.j',json('{x:99}'),'$.k','{y:98}');\nUPDATE j1 SET x=jsonb_set(x,'$.e',(x->>'f')-1);\nUPDATE j1 SET x=jsonb_remove(x,'$.d');\nCOMMIT;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
        speedtest1_begin_test(
            160 as ::core::ffi::c_int,
            b"json_insert()/set()/remove() on every row of J2\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        speedtest1_exec(
            b"BEGIN;\nUPDATE j2 SET x=json_insert(x,'$.g',(x->>'f')+1);\nUPDATE j2 SET x=json_set(x,'$.e',(x->>'f')-1);\nUPDATE j2 SET x=json_remove(x,'$.d');\nCOMMIT;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testset_parsenumber() {
    unsafe {
        let mut zSql1: *const ::core::ffi::c_char = b"SELECT 1, 12, 123, 1234, 12345, 123456\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut zSql2: *const ::core::ffi::c_char = b"SELECT 8227256643844975616, 7932208612563860480, 2010730661871032832, 9138463067404021760, 2557616153664746496, 2557616153664746496\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut zSql3: *const ::core::ffi::c_char = b"SELECT 1.0, 1.2, 1.23, 123.4, 1.2345, 1.23456\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut zSql4: *const ::core::ffi::c_char = b"SELECT 8.227256643844975616, 7.932208612563860480, 2.010730661871032832, 9.138463067404021760, 2.557616153664746496, 2.557616153664746496\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let NROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int * g.szTest;
        let mut ii: ::core::ffi::c_int = 0;
        speedtest1_begin_test(
            100 as ::core::ffi::c_int,
            b"parsing %d small integers\0".as_ptr() as *const ::core::ffi::c_char,
            NROW,
        );
        ii = 0 as ::core::ffi::c_int;
        while ii < NROW {
            sqlite3_exec(
                g.db,
                zSql1,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            ii += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            110 as ::core::ffi::c_int,
            b"parsing %d large integers\0".as_ptr() as *const ::core::ffi::c_char,
            NROW,
        );
        ii = 0 as ::core::ffi::c_int;
        while ii < NROW {
            sqlite3_exec(
                g.db,
                zSql2,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            ii += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            200 as ::core::ffi::c_int,
            b"parsing %d small reals\0".as_ptr() as *const ::core::ffi::c_char,
            NROW,
        );
        ii = 0 as ::core::ffi::c_int;
        while ii < NROW {
            sqlite3_exec(
                g.db,
                zSql3,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            ii += 1;
        }
        speedtest1_end_test();
        speedtest1_begin_test(
            210 as ::core::ffi::c_int,
            b"parsing %d large reals\0".as_ptr() as *const ::core::ffi::c_char,
            NROW,
        );
        ii = 0 as ::core::ffi::c_int;
        while ii < NROW {
            sqlite3_exec(
                g.db,
                zSql4,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            ii += 1;
        }
        speedtest1_end_test();
    }
}
unsafe extern "C" fn displayLinuxIoStats(mut out: *mut FILE) {
    unsafe {
        let mut in_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut z: [::core::ffi::c_char; 200] = [0; 200];
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as ::core::ffi::c_int,
            &raw mut z as *mut ::core::ffi::c_char,
            b"/proc/%d/io\0".as_ptr() as *const ::core::ffi::c_char,
            getpid(),
        );
        in_0 = fopen(
            &raw mut z as *mut ::core::ffi::c_char,
            b"rb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if in_0.is_null() {
            return;
        }
        while !fgets(
                &raw mut z as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 200]>()
                    as ::core::ffi::c_int,
                in_0,
            )
            .is_null()
        {
            static mut aTrans: [C2Rust_Unnamed_1; 7] = [
                C2Rust_Unnamed_1 {
                    zPattern: b"rchar: \0".as_ptr() as *const ::core::ffi::c_char,
                    zDesc: b"Bytes received by read():\0".as_ptr()
                        as *const ::core::ffi::c_char,
                },
                C2Rust_Unnamed_1 {
                    zPattern: b"wchar: \0".as_ptr() as *const ::core::ffi::c_char,
                    zDesc: b"Bytes sent to write():\0".as_ptr()
                        as *const ::core::ffi::c_char,
                },
                C2Rust_Unnamed_1 {
                    zPattern: b"syscr: \0".as_ptr() as *const ::core::ffi::c_char,
                    zDesc: b"Read() system calls:\0".as_ptr()
                        as *const ::core::ffi::c_char,
                },
                C2Rust_Unnamed_1 {
                    zPattern: b"syscw: \0".as_ptr() as *const ::core::ffi::c_char,
                    zDesc: b"Write() system calls:\0".as_ptr()
                        as *const ::core::ffi::c_char,
                },
                C2Rust_Unnamed_1 {
                    zPattern: b"read_bytes: \0".as_ptr() as *const ::core::ffi::c_char,
                    zDesc: b"Bytes rcvd from storage:\0".as_ptr()
                        as *const ::core::ffi::c_char,
                },
                C2Rust_Unnamed_1 {
                    zPattern: b"write_bytes: \0".as_ptr() as *const ::core::ffi::c_char,
                    zDesc: b"Bytes sent to storage:\0".as_ptr()
                        as *const ::core::ffi::c_char,
                },
                C2Rust_Unnamed_1 {
                    zPattern: b"cancelled_write_bytes: \0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zDesc: b"Cancelled write bytes:\0".as_ptr()
                        as *const ::core::ffi::c_char,
                },
            ];
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while (i as usize)
                < (::core::mem::size_of::<[C2Rust_Unnamed_1; 7]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_1>() as usize)
            {
                let mut n: ::core::ffi::c_int = strlen(aTrans[i as usize].zPattern)
                    as ::core::ffi::c_int;
                if strncmp(
                    aTrans[i as usize].zPattern,
                    &raw mut z as *mut ::core::ffi::c_char,
                    n as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    fprintf(
                        out,
                        b"-- %-28s %s\0".as_ptr() as *const ::core::ffi::c_char,
                        aTrans[i as usize].zDesc,
                        (&raw mut z as *mut ::core::ffi::c_char).offset(n as isize)
                            as *mut ::core::ffi::c_char,
                    );
                    break;
                } else {
                    i += 1;
                }
            }
        }
        fclose(in_0);
    }
}
unsafe extern "C" fn xCompileOptions(
    mut pCtx: *mut ::core::ffi::c_void,
    mut nVal: ::core::ffi::c_int,
    mut azVal: *mut *mut ::core::ffi::c_char,
    mut azCol: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        printf(
            b"-- Compile option: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            *azVal.offset(0 as ::core::ffi::c_int as isize),
        );
        return SQLITE_OK;
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut doAutovac: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut cacheSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut doExclusive: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut doFullFSync: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nHeap: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mnHeap: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut doIncrvac: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zJMode: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zKey: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nHardHeapLmt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nSoftHeapLmt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nLook: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut szLook: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut noSync: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pageSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nPCache: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut szPCache: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut doPCache: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut showStats: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nThread: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mmapSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut memDb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut openFlags: ::core::ffi::c_int = SQLITE_OPEN_READWRITE
            | SQLITE_OPEN_CREATE;
        let mut zTSet: *mut ::core::ffi::c_char = b"mix1\0".as_ptr()
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        let mut doTrace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zEncoding: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pHeap: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut pLook: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut pPCache: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut iCur: ::core::ffi::c_int = 0;
        let mut iHi: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        static mut zMix1Tests: [::core::ffi::c_char; 62] = unsafe {
            ::core::mem::transmute::<
                [u8; 62],
                [::core::ffi::c_char; 62],
            >(*b"main,orm/25,cte/20,json,fp/3,parsenumber/25,rtree/10,star,app\0")
        };
        printf(
            b"-- Speedtest1 for SQLite %s %.48s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_libversion(),
            sqlite3_sourceid(),
        );
        g.zDbName = ::core::ptr::null::<::core::ffi::c_char>();
        g.zVfs = ::core::ptr::null::<::core::ffi::c_char>();
        g.zWR = b"\0".as_ptr() as *const ::core::ffi::c_char;
        g.zNN = b"\0".as_ptr() as *const ::core::ffi::c_char;
        g.zPK = b"UNIQUE\0".as_ptr() as *const ::core::ffi::c_char;
        g.szTest = 100 as ::core::ffi::c_int;
        g.szBase = 100 as ::core::ffi::c_int;
        g.nRepeat = 1 as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while i < argc {
            let mut z: *const ::core::ffi::c_char = *argv.offset(i as isize);
            if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32
            {
                loop {
                    z = z.offset(1);
                    if !(*z.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '-' as i32)
                    {
                        break;
                    }
                }
                if strcmp(z, b"autovacuum\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    doAutovac = 1 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"big-transactions\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    g.doBigTransactions = 1 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"cachesize\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    cacheSize = integerValue(*argv.offset(i as isize));
                } else if strcmp(
                    z,
                    b"exclusive\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    doExclusive = 1 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"fullfsync\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    doFullFSync = 1 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"checkpoint\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    g.doCheckpoint = 1 as ::core::ffi::c_int;
                } else if strcmp(z, b"explain\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    g.bSqlOnly = 1 as ::core::ffi::c_int;
                    g.bExplain = 1 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"hard-heap-limit\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    nHardHeapLmt = integerValue(
                        *argv.offset((i + 1 as ::core::ffi::c_int) as isize),
                    );
                    i += 1 as ::core::ffi::c_int;
                } else if strcmp(z, b"heap\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 2 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    nHeap = integerValue(
                        *argv.offset((i + 1 as ::core::ffi::c_int) as isize),
                    );
                    mnHeap = integerValue(
                        *argv.offset((i + 2 as ::core::ffi::c_int) as isize),
                    );
                    i += 2 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"incrvacuum\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    doIncrvac = 1 as ::core::ffi::c_int;
                } else if strcmp(z, b"journal\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    zJMode = *argv.offset(i as isize);
                } else if strcmp(z, b"key\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    zKey = *argv.offset(i as isize);
                } else if strcmp(
                    z,
                    b"lookaside\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 2 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    nLook = integerValue(
                        *argv.offset((i + 1 as ::core::ffi::c_int) as isize),
                    );
                    szLook = integerValue(
                        *argv.offset((i + 2 as ::core::ffi::c_int) as isize),
                    );
                    i += 2 as ::core::ffi::c_int;
                } else if strcmp(z, b"memdb\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    memDb = 1 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"multithread\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    sqlite3_config(SQLITE_CONFIG_MULTITHREAD);
                } else if strcmp(
                    z,
                    b"nomemstat\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    sqlite3_config(SQLITE_CONFIG_MEMSTATUS, 0 as ::core::ffi::c_int);
                } else if strcmp(z, b"mmap\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    mmapSize = integerValue(*argv.offset(i as isize));
                } else if strcmp(z, b"nomutex\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    openFlags |= SQLITE_OPEN_NOMUTEX;
                } else if strcmp(z, b"nosync\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    noSync = 1 as ::core::ffi::c_int;
                } else if strcmp(z, b"notnull\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    g.zNN = b"NOT NULL\0".as_ptr() as *const ::core::ffi::c_char;
                } else if strcmp(z, b"output\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    if strcmp(
                        *argv.offset(i as isize),
                        b"-\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        g.hashFile = stdout;
                    } else {
                        g.hashFile = fopen(
                            *argv.offset(i as isize),
                            b"wb\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if g.hashFile.is_null() {
                            fatal_error(
                                b"cannot open \"%s\" for writing\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                *argv.offset(i as isize),
                            );
                        }
                    }
                } else if strcmp(z, b"pagesize\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    pageSize = integerValue(*argv.offset(i as isize));
                } else if strcmp(z, b"pcache\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 2 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    nPCache = integerValue(
                        *argv.offset((i + 1 as ::core::ffi::c_int) as isize),
                    );
                    szPCache = integerValue(
                        *argv.offset((i + 2 as ::core::ffi::c_int) as isize),
                    );
                    doPCache = 1 as ::core::ffi::c_int;
                    i += 2 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"primarykey\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    g.zPK = b"PRIMARY KEY\0".as_ptr() as *const ::core::ffi::c_char;
                } else if strcmp(z, b"repeat\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    g.nRepeat = integerValue(*argv.offset(i as isize));
                } else if strcmp(
                    z,
                    b"reprepare\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    g.bReprepare = 1 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"serialized\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    sqlite3_config(SQLITE_CONFIG_SERIALIZED);
                } else if strcmp(
                    z,
                    b"singlethread\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    sqlite3_config(SQLITE_CONFIG_SINGLETHREAD);
                } else if strcmp(z, b"script\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    if !g.pScript.is_null() {
                        fclose(g.pScript);
                    }
                    i += 1;
                    g.pScript = fopen(
                        *argv.offset(i as isize),
                        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if g.pScript.is_null() {
                        fatal_error(
                            b"unable to open output file \"%s\"\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                } else if strcmp(z, b"sqlonly\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    g.bSqlOnly = 1 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"shrink-memory\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    g.bMemShrink = 1 as ::core::ffi::c_int;
                } else if strcmp(z, b"size\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    g.szBase = integerValue(*argv.offset(i as isize));
                    g.szTest = g.szBase;
                } else if strcmp(
                    z,
                    b"soft-heap-limit\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    nSoftHeapLmt = integerValue(
                        *argv.offset((i + 1 as ::core::ffi::c_int) as isize),
                    );
                    i += 1 as ::core::ffi::c_int;
                } else if strcmp(z, b"stats\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    showStats = 1 as ::core::ffi::c_int;
                } else if strcmp(z, b"temp\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    if (*(*argv.offset(i as isize))
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        < '0' as i32
                        || *(*argv.offset(i as isize))
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int > '9' as i32
                        || *(*argv.offset(i as isize))
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    {
                        fatal_error(
                            b"argument to --temp should be integer between 0 and 9\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    g.eTemp = *(*argv.offset(i as isize))
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - '0' as i32;
                } else if strcmp(z, b"testset\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    zTSet = *argv.offset(i as isize);
                } else if strcmp(z, b"trace\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    doTrace = 1 as ::core::ffi::c_int;
                } else if strcmp(z, b"threads\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    nThread = integerValue(*argv.offset(i as isize));
                } else if strcmp(z, b"utf16le\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    zEncoding = b"utf16le\0".as_ptr() as *const ::core::ffi::c_char;
                } else if strcmp(z, b"utf16be\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    zEncoding = b"utf16be\0".as_ptr() as *const ::core::ffi::c_char;
                } else if strcmp(z, b"verify\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    g.bVerify = 1 as ::core::ffi::c_int;
                    HashInit();
                } else if strcmp(z, b"vfs\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    g.zVfs = *argv.offset(i as isize);
                } else if strcmp(z, b"reserve\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if i >= argc - 1 as ::core::ffi::c_int {
                        fatal_error(
                            b"missing argument on %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                    }
                    i += 1;
                    g.nReserve = atoi(*argv.offset(i as isize));
                } else if strcmp(
                    z,
                    b"stmtscanstatus\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    g.stmtScanStatus = 1 as ::core::ffi::c_int;
                } else if strcmp(
                    z,
                    b"without-rowid\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if strstr(g.zWR, b"WITHOUT\0".as_ptr() as *const ::core::ffi::c_char)
                        .is_null()
                    {
                        if !strstr(
                                g.zWR,
                                b"STRICT\0".as_ptr() as *const ::core::ffi::c_char,
                            )
                            .is_null()
                        {
                            g.zWR = b"WITHOUT ROWID,STRICT\0".as_ptr()
                                as *const ::core::ffi::c_char;
                        } else {
                            g.zWR = b"WITHOUT ROWID\0".as_ptr()
                                as *const ::core::ffi::c_char;
                        }
                    }
                    g.zPK = b"PRIMARY KEY\0".as_ptr() as *const ::core::ffi::c_char;
                } else if strcmp(z, b"strict\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if strstr(g.zWR, b"STRICT\0".as_ptr() as *const ::core::ffi::c_char)
                        .is_null()
                    {
                        if !strstr(
                                g.zWR,
                                b"WITHOUT\0".as_ptr() as *const ::core::ffi::c_char,
                            )
                            .is_null()
                        {
                            g.zWR = b"WITHOUT ROWID,STRICT\0".as_ptr()
                                as *const ::core::ffi::c_char;
                        } else {
                            g.zWR = b"STRICT\0".as_ptr() as *const ::core::ffi::c_char;
                        }
                    }
                } else if strcmp(z, b"help\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(z, b"?\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    printf(
                        &raw const zHelp as *const ::core::ffi::c_char,
                        *argv.offset(0 as ::core::ffi::c_int as isize),
                    );
                    exit(0 as ::core::ffi::c_int);
                } else {
                    fatal_error(
                        b"unknown option: %s\nUse \"%s -?\" for help\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        *argv.offset(i as isize),
                        *argv.offset(0 as ::core::ffi::c_int as isize),
                    );
                }
            } else if g.zDbName.is_null() {
                g.zDbName = *argv.offset(i as isize);
            } else {
                fatal_error(
                    b"surplus argument: %s\nUse \"%s -?\" for help\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *argv.offset(i as isize),
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
            }
            i += 1;
        }
        if nHeap > 0 as ::core::ffi::c_int {
            pHeap = malloc(nHeap as size_t);
            if pHeap.is_null() {
                fatal_error(
                    b"cannot allocate %d-byte heap\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    nHeap,
                );
            }
            rc = sqlite3_config(SQLITE_CONFIG_HEAP, pHeap, nHeap, mnHeap);
            if rc != 0 {
                fatal_error(
                    b"heap configuration failed: %d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    rc,
                );
            }
        }
        if doPCache != 0 {
            if nPCache > 0 as ::core::ffi::c_int && szPCache > 0 as ::core::ffi::c_int {
                pPCache = malloc(
                    (nPCache as sqlite3_int64 * szPCache as sqlite3_int64) as size_t,
                );
                if pPCache.is_null() {
                    fatal_error(
                        b"cannot allocate %lld-byte pcache\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        nPCache as sqlite3_int64 * szPCache as sqlite3_int64,
                    );
                }
            }
            rc = sqlite3_config(SQLITE_CONFIG_PAGECACHE, pPCache, szPCache, nPCache);
            if rc != 0 {
                fatal_error(
                    b"pcache configuration failed: %d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    rc,
                );
            }
        }
        if nLook >= 0 as ::core::ffi::c_int {
            sqlite3_config(
                SQLITE_CONFIG_LOOKASIDE,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        sqlite3_initialize();
        if !g.zDbName.is_null() {
            let mut pVfs: *mut sqlite3_vfs = sqlite3_vfs_find(g.zVfs);
            if !pVfs.is_null() {
                (*pVfs)
                    .xDelete
                    .expect(
                        "non-null function pointer",
                    )(pVfs, g.zDbName, 1 as ::core::ffi::c_int);
            }
            unlink(g.zDbName);
        }
        if sqlite3_open_v2(
            if memDb != 0 {
                b":memory:\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                g.zDbName
            },
            &raw mut g.db,
            openFlags,
            g.zVfs,
        ) != 0
        {
            fatal_error(
                b"Cannot open database file: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                g.zDbName,
            );
        }
        if nLook > 0 as ::core::ffi::c_int && szLook > 0 as ::core::ffi::c_int {
            pLook = malloc((nLook * szLook) as size_t);
            rc = sqlite3_db_config(
                g.db,
                SQLITE_DBCONFIG_LOOKASIDE,
                pLook,
                szLook,
                nLook,
            );
            if rc != 0 {
                fatal_error(
                    b"lookaside configuration failed: %d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    rc,
                );
            }
        }
        if g.nReserve > 0 as ::core::ffi::c_int {
            sqlite3_file_control(
                g.db,
                ::core::ptr::null::<::core::ffi::c_char>(),
                SQLITE_FCNTL_RESERVE_BYTES,
                &raw mut g.nReserve as *mut ::core::ffi::c_void,
            );
        }
        if g.stmtScanStatus != 0 {
            sqlite3_db_config(
                g.db,
                SQLITE_DBCONFIG_STMT_SCANSTATUS,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        sqlite3_create_function(
            g.db,
            b"random\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                randomFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
        if doTrace != 0 {
            sqlite3_trace(
                g.db,
                Some(
                    traceCallback
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                        ) -> (),
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        if memDb > 0 as ::core::ffi::c_int {
            speedtest1_exec(
                b"PRAGMA temp_store=memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if mmapSize > 0 as ::core::ffi::c_int {
            speedtest1_exec(
                b"PRAGMA mmap_size=%d\0".as_ptr() as *const ::core::ffi::c_char,
                mmapSize,
            );
        }
        speedtest1_exec(
            b"PRAGMA threads=%d\0".as_ptr() as *const ::core::ffi::c_char,
            nThread,
        );
        if !zKey.is_null() {
            speedtest1_exec(
                b"PRAGMA key('%s')\0".as_ptr() as *const ::core::ffi::c_char,
                zKey,
            );
        }
        if !zEncoding.is_null() {
            speedtest1_exec(
                b"PRAGMA encoding=%s\0".as_ptr() as *const ::core::ffi::c_char,
                zEncoding,
            );
        }
        if doAutovac != 0 {
            speedtest1_exec(
                b"PRAGMA auto_vacuum=FULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if doIncrvac != 0 {
            speedtest1_exec(
                b"PRAGMA auto_vacuum=INCREMENTAL\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if pageSize != 0 {
            speedtest1_exec(
                b"PRAGMA page_size=%d\0".as_ptr() as *const ::core::ffi::c_char,
                pageSize,
            );
        }
        if cacheSize != 0 {
            speedtest1_exec(
                b"PRAGMA cache_size=%d\0".as_ptr() as *const ::core::ffi::c_char,
                cacheSize,
            );
        }
        if noSync != 0 {
            speedtest1_exec(
                b"PRAGMA synchronous=OFF\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if doFullFSync != 0 {
            speedtest1_exec(
                b"PRAGMA fullfsync=ON\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if doExclusive != 0 {
            speedtest1_exec(
                b"PRAGMA locking_mode=EXCLUSIVE\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !zJMode.is_null() {
            speedtest1_exec(
                b"PRAGMA journal_mode=%s\0".as_ptr() as *const ::core::ffi::c_char,
                zJMode,
            );
        }
        if nHardHeapLmt > 0 as ::core::ffi::c_int {
            speedtest1_exec(
                b"PRAGMA hard_heap_limit=%d\0".as_ptr() as *const ::core::ffi::c_char,
                nHardHeapLmt,
            );
        }
        if nSoftHeapLmt > 0 as ::core::ffi::c_int {
            speedtest1_exec(
                b"PRAGMA soft_heap_limit=%d\0".as_ptr() as *const ::core::ffi::c_char,
                nSoftHeapLmt,
            );
        }
        if !zJMode.is_null() {
            speedtest1_exec(
                b"PRAGMA journal_mode=%s\0".as_ptr() as *const ::core::ffi::c_char,
                zJMode,
            );
        }
        if g.bExplain != 0 {
            printf(b".explain\n.echo on\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if strcmp(zTSet, b"mix1\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            zTSet = &raw mut zMix1Tests as *mut ::core::ffi::c_char;
        }
        loop {
            let mut zThisTest: *mut ::core::ffi::c_char = zTSet;
            let mut zSep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut zComma: *mut ::core::ffi::c_char = strchr(zThisTest, ',' as i32);
            if !zComma.is_null() {
                *zComma = 0 as ::core::ffi::c_char;
                zTSet = zComma.offset(1 as ::core::ffi::c_int as isize);
            } else {
                zTSet = b"\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            zSep = strchr(zThisTest, '/' as i32);
            if !zSep.is_null() {
                let mut kk: ::core::ffi::c_int = 0;
                kk = 1 as ::core::ffi::c_int;
                while *zSep.offset(kk as isize) as ::core::ffi::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(
                            *zSep.offset(kk as isize) as ::core::ffi::c_uchar
                                as ::core::ffi::c_int as isize,
                        ) as ::core::ffi::c_int
                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int != 0
                {
                    kk += 1;
                }
                if kk == 1 as ::core::ffi::c_int
                    || *zSep.offset(kk as isize) as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                {
                    fatal_error(
                        b"bad modifier on testset name: \"%s\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zThisTest,
                    );
                }
                g.szTest = g.szBase
                    * integerValue(zSep.offset(1 as ::core::ffi::c_int as isize))
                    / 100 as ::core::ffi::c_int;
                if g.szTest <= 0 as ::core::ffi::c_int {
                    g.szTest = 1 as ::core::ffi::c_int;
                }
                *zSep.offset(0 as ::core::ffi::c_int as isize) = 0
                    as ::core::ffi::c_char;
            } else {
                g.szTest = g.szBase;
            }
            if g.iTotal > 0 as ::core::ffi::c_longlong || zComma.is_null() {
                printf(
                    b"       Begin testset \"%s\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zThisTest,
                );
            }
            if strcmp(zThisTest, b"main\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                testset_main();
            } else if strcmp(
                zThisTest,
                b"debug1\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                testset_debug1();
            } else if strcmp(zThisTest, b"orm\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                testset_orm();
            } else if strcmp(zThisTest, b"cte\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                testset_cte();
            } else if strcmp(zThisTest, b"star\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                testset_star();
            } else if strcmp(zThisTest, b"app\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                testset_app();
            } else if strcmp(zThisTest, b"fp\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                testset_fp();
            } else if strcmp(zThisTest, b"json\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                testset_json();
            } else if strcmp(
                zThisTest,
                b"trigger\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                testset_trigger();
            } else if strcmp(
                zThisTest,
                b"parsenumber\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                testset_parsenumber();
            } else if strcmp(
                zThisTest,
                b"rtree\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                testset_rtree(6 as ::core::ffi::c_int, 147 as ::core::ffi::c_int);
            } else {
                fatal_error(
                    b"unknown testset: \"%s\"\nChoices: cte debug1 fp main orm rtree trigger\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    zThisTest,
                );
            }
            if *zTSet.offset(0 as ::core::ffi::c_int as isize) != 0 {
                let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zObj: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                speedtest1_begin_test(
                    999 as ::core::ffi::c_int,
                    b"Reset the database\0".as_ptr() as *const ::core::ffi::c_char,
                );
                loop {
                    zObj = speedtest1_once(
                        b"SELECT name FROM main.sqlite_master WHERE sql LIKE 'CREATE %%TABLE%%'\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    if zObj.is_null() {
                        break;
                    }
                    zSql = sqlite3_mprintf(
                        b"DROP TABLE main.\"%w\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zObj,
                    );
                    speedtest1_exec(zSql);
                    sqlite3_free(zSql as *mut ::core::ffi::c_void);
                    sqlite3_free(zObj as *mut ::core::ffi::c_void);
                }
                loop {
                    zObj = speedtest1_once(
                        b"SELECT name FROM temp.sqlite_master WHERE sql LIKE 'CREATE %%TABLE%%'\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    if zObj.is_null() {
                        break;
                    }
                    zSql = sqlite3_mprintf(
                        b"DROP TABLE main.\"%w\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zObj,
                    );
                    speedtest1_exec(zSql);
                    sqlite3_free(zSql as *mut ::core::ffi::c_void);
                    sqlite3_free(zObj as *mut ::core::ffi::c_void);
                }
                speedtest1_end_test();
            }
            if !(*zTSet.offset(0 as ::core::ffi::c_int as isize) != 0) {
                break;
            }
        }
        speedtest1_final();
        if showStats != 0 {
            sqlite3_exec(
                g.db,
                b"PRAGMA compile_options\0".as_ptr() as *const ::core::ffi::c_char,
                Some(
                    xCompileOptions
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *mut *mut ::core::ffi::c_char,
                            *mut *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
        if showStats != 0 {
            sqlite3_db_status(
                g.db,
                SQLITE_DBSTATUS_LOOKASIDE_USED,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Lookaside Slots Used:        %d (max %d)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iCur,
                iHi,
            );
            sqlite3_db_status(
                g.db,
                SQLITE_DBSTATUS_LOOKASIDE_HIT,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Successful lookasides:       %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iHi,
            );
            sqlite3_db_status(
                g.db,
                SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Lookaside size faults:       %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iHi,
            );
            sqlite3_db_status(
                g.db,
                SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Lookaside OOM faults:        %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iHi,
            );
            sqlite3_db_status(
                g.db,
                SQLITE_DBSTATUS_CACHE_USED,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Pager Heap Usage:            %d bytes\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iCur,
            );
            sqlite3_db_status(
                g.db,
                SQLITE_DBSTATUS_CACHE_HIT,
                &raw mut iCur,
                &raw mut iHi,
                1 as ::core::ffi::c_int,
            );
            printf(
                b"-- Page cache hits:             %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iCur,
            );
            sqlite3_db_status(
                g.db,
                SQLITE_DBSTATUS_CACHE_MISS,
                &raw mut iCur,
                &raw mut iHi,
                1 as ::core::ffi::c_int,
            );
            printf(
                b"-- Page cache misses:           %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iCur,
            );
            sqlite3_db_status(
                g.db,
                SQLITE_DBSTATUS_CACHE_WRITE,
                &raw mut iCur,
                &raw mut iHi,
                1 as ::core::ffi::c_int,
            );
            printf(
                b"-- Page cache writes:           %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iCur,
            );
            sqlite3_db_status(
                g.db,
                SQLITE_DBSTATUS_SCHEMA_USED,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Schema Heap Usage:           %d bytes\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iCur,
            );
            sqlite3_db_status(
                g.db,
                SQLITE_DBSTATUS_STMT_USED,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Statement Heap Usage:        %d bytes\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iCur,
            );
        }
        sqlite3_close(g.db);
        if showStats != 0 {
            sqlite3_status(
                SQLITE_STATUS_MEMORY_USED,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Memory Used (bytes):         %d (max %d)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iCur,
                iHi,
            );
            sqlite3_status(
                SQLITE_STATUS_MALLOC_COUNT,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Outstanding Allocations:     %d (max %d)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iCur,
                iHi,
            );
            sqlite3_status(
                SQLITE_STATUS_PAGECACHE_OVERFLOW,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Pcache Overflow Bytes:       %d (max %d)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iCur,
                iHi,
            );
            sqlite3_status(
                SQLITE_STATUS_MALLOC_SIZE,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Largest Allocation:          %d bytes\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iHi,
            );
            sqlite3_status(
                SQLITE_STATUS_PAGECACHE_SIZE,
                &raw mut iCur,
                &raw mut iHi,
                0 as ::core::ffi::c_int,
            );
            printf(
                b"-- Largest Pcache Allocation:   %d bytes\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iHi,
            );
        }
        if showStats != 0 {
            displayLinuxIoStats(stdout);
        }
        if !g.pScript.is_null() {
            fclose(g.pScript);
        }
        free(pLook);
        free(pPCache);
        free(pHeap);
        return 0 as ::core::ffi::c_int;
    }
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(
            main_0(
                (args_ptrs.len() - 1) as ::core::ffi::c_int,
                args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
            ) as i32,
        )
    }
}
