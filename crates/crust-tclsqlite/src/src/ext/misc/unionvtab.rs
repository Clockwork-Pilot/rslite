use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_str;
    pub type sqlite3_blob;
    pub type sqlite3_backup;
    fn sqlite3_close(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_vmprintf(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_open_v2(
        filename: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
        flags: ::core::ffi::c_int,
        zVfs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_errcode(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_parameter_index(
        _: *mut sqlite3_stmt,
        zName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_int64(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> sqlite3_int64;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    fn sqlite3_column_value(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *mut sqlite3_value;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
    fn sqlite3_db_handle(_: *mut sqlite3_stmt) -> *mut sqlite3;
    fn sqlite3_table_column_metadata(
        db: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        zTableName: *const ::core::ffi::c_char,
        zColumnName: *const ::core::ffi::c_char,
        pzDataType: *mut *const ::core::ffi::c_char,
        pzCollSeq: *mut *const ::core::ffi::c_char,
        pNotNull: *mut ::core::ffi::c_int,
        pPrimaryKey: *mut ::core::ffi::c_int,
        pAutoinc: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub type sqlite3_callback = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        *mut *mut ::core::ffi::c_char,
        *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_api_routines {
    pub aggregate_context: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub aggregate_count: Option<
        unsafe extern "C" fn(*mut sqlite3_context) -> ::core::ffi::c_int,
    >,
    pub bind_blob: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_double: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            ::core::ffi::c_double,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_int: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_int64: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            sqlite_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_null: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub bind_parameter_count: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int,
    >,
    pub bind_parameter_index: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_parameter_name: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub bind_text: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_text16: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_value: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            *const sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub busy_handler: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub busy_timeout: Option<
        unsafe extern "C" fn(*mut sqlite3, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub changes: Option<unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int>,
    pub collation_needed: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut sqlite3,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                ) -> (),
            >,
        ) -> ::core::ffi::c_int,
    >,
    pub collation_needed16: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut sqlite3,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> (),
            >,
        ) -> ::core::ffi::c_int,
    >,
    pub column_blob: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_bytes: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub column_bytes16: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub column_count: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int,
    >,
    pub column_database_name: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub column_database_name16: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_decltype: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub column_decltype16: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_double: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_double,
    >,
    pub column_int: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub column_int64: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt, ::core::ffi::c_int) -> sqlite_int64,
    >,
    pub column_name: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub column_name16: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_origin_name: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub column_origin_name16: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_table_name: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub column_table_name16: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_text: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_uchar,
    >,
    pub column_text16: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_type: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub column_value: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt, ::core::ffi::c_int) -> *mut sqlite3_value,
    >,
    pub commit_hook: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub complete: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub complete16: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub create_collation: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
    pub create_collation16: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
    pub create_function: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub create_function16: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub create_module: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            *const sqlite3_module,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub data_count: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int,
    >,
    pub db_handle: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> *mut sqlite3>,
    pub declare_vtab: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub enable_shared_cache: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub errcode: Option<unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int>,
    pub errmsg: Option<unsafe extern "C" fn(*mut sqlite3) -> *const ::core::ffi::c_char>,
    pub errmsg16: Option<
        unsafe extern "C" fn(*mut sqlite3) -> *const ::core::ffi::c_void,
    >,
    pub exec: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            sqlite3_callback,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub expired: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int>,
    pub finalize: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int>,
    pub free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub free_table: Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_char) -> ()>,
    pub get_autocommit: Option<unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int>,
    pub get_auxdata: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub get_table: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            *mut *mut *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub global_recover: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub interruptx: Option<unsafe extern "C" fn(*mut sqlite3) -> ()>,
    pub last_insert_rowid: Option<unsafe extern "C" fn(*mut sqlite3) -> sqlite_int64>,
    pub libversion: Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
    pub libversion_number: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub malloc: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut ::core::ffi::c_void,
    >,
    pub mprintf: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char,
    >,
    pub open: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *mut *mut sqlite3,
        ) -> ::core::ffi::c_int,
    >,
    pub open16: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *mut *mut sqlite3,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_stmt,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare16: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut *mut sqlite3_stmt,
            *mut *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub profile: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    sqlite_uint64,
                ) -> (),
            >,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub progress_handler: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            ::core::ffi::c_int,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub realloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int>,
    pub result_blob: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> (),
    >,
    pub result_double: Option<
        unsafe extern "C" fn(*mut sqlite3_context, ::core::ffi::c_double) -> (),
    >,
    pub result_error: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub result_error16: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub result_int: Option<
        unsafe extern "C" fn(*mut sqlite3_context, ::core::ffi::c_int) -> (),
    >,
    pub result_int64: Option<
        unsafe extern "C" fn(*mut sqlite3_context, sqlite_int64) -> (),
    >,
    pub result_null: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    pub result_text: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> (),
    >,
    pub result_text16: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> (),
    >,
    pub result_text16be: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> (),
    >,
    pub result_text16le: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> (),
    >,
    pub result_value: Option<
        unsafe extern "C" fn(*mut sqlite3_context, *mut sqlite3_value) -> (),
    >,
    pub rollback_hook: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub set_authorizer: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub set_auxdata: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> (),
    >,
    pub xsnprintf: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ...
        ) -> *mut ::core::ffi::c_char,
    >,
    pub step: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int>,
    pub table_column_metadata: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub thread_cleanup: Option<unsafe extern "C" fn() -> ()>,
    pub total_changes: Option<unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int>,
    pub trace: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> (),
            >,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub transfer_bindings: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt, *mut sqlite3_stmt) -> ::core::ffi::c_int,
    >,
    pub update_hook: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    sqlite_int64,
                ) -> (),
            >,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub user_data: Option<
        unsafe extern "C" fn(*mut sqlite3_context) -> *mut ::core::ffi::c_void,
    >,
    pub value_blob: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> *const ::core::ffi::c_void,
    >,
    pub value_bytes: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> ::core::ffi::c_int,
    >,
    pub value_bytes16: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> ::core::ffi::c_int,
    >,
    pub value_double: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> ::core::ffi::c_double,
    >,
    pub value_int: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> ::core::ffi::c_int,
    >,
    pub value_int64: Option<unsafe extern "C" fn(*mut sqlite3_value) -> sqlite_int64>,
    pub value_numeric_type: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> ::core::ffi::c_int,
    >,
    pub value_text: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> *const ::core::ffi::c_uchar,
    >,
    pub value_text16: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> *const ::core::ffi::c_void,
    >,
    pub value_text16be: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> *const ::core::ffi::c_void,
    >,
    pub value_text16le: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> *const ::core::ffi::c_void,
    >,
    pub value_type: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> ::core::ffi::c_int,
    >,
    pub vmprintf: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            ::core::ffi::VaList,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub overload_function: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare_v2: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_stmt,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare16_v2: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut *mut sqlite3_stmt,
            *mut *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub clear_bindings: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int,
    >,
    pub create_module_v2: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            *const sqlite3_module,
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_zeroblob: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub blob_bytes: Option<
        unsafe extern "C" fn(*mut sqlite3_blob) -> ::core::ffi::c_int,
    >,
    pub blob_close: Option<
        unsafe extern "C" fn(*mut sqlite3_blob) -> ::core::ffi::c_int,
    >,
    pub blob_open: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            sqlite3_int64,
            ::core::ffi::c_int,
            *mut *mut sqlite3_blob,
        ) -> ::core::ffi::c_int,
    >,
    pub blob_read: Option<
        unsafe extern "C" fn(
            *mut sqlite3_blob,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub blob_write: Option<
        unsafe extern "C" fn(
            *mut sqlite3_blob,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub create_collation_v2: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub file_control: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub memory_highwater: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> sqlite3_int64,
    >,
    pub memory_used: Option<unsafe extern "C" fn() -> sqlite3_int64>,
    pub mutex_alloc: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut sqlite3_mutex,
    >,
    pub mutex_enter: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub mutex_free: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub mutex_leave: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub mutex_try: Option<
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
    >,
    pub open_v2: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *mut *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub release_memory: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub result_error_nomem: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    pub result_error_toobig: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    pub sleep: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub soft_heap_limit: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    pub vfs_find: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut sqlite3_vfs,
    >,
    pub vfs_register: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub vfs_unregister: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs) -> ::core::ffi::c_int,
    >,
    pub xthreadsafe: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub result_zeroblob: Option<
        unsafe extern "C" fn(*mut sqlite3_context, ::core::ffi::c_int) -> (),
    >,
    pub result_error_code: Option<
        unsafe extern "C" fn(*mut sqlite3_context, ::core::ffi::c_int) -> (),
    >,
    pub test_control: Option<
        unsafe extern "C" fn(::core::ffi::c_int, ...) -> ::core::ffi::c_int,
    >,
    pub randomness: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void) -> (),
    >,
    pub context_db_handle: Option<
        unsafe extern "C" fn(*mut sqlite3_context) -> *mut sqlite3,
    >,
    pub extended_result_codes: Option<
        unsafe extern "C" fn(*mut sqlite3, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub limit: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub next_stmt: Option<
        unsafe extern "C" fn(*mut sqlite3, *mut sqlite3_stmt) -> *mut sqlite3_stmt,
    >,
    pub sql: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt) -> *const ::core::ffi::c_char,
    >,
    pub status: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub backup_finish: Option<
        unsafe extern "C" fn(*mut sqlite3_backup) -> ::core::ffi::c_int,
    >,
    pub backup_init: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            *mut sqlite3,
            *const ::core::ffi::c_char,
        ) -> *mut sqlite3_backup,
    >,
    pub backup_pagecount: Option<
        unsafe extern "C" fn(*mut sqlite3_backup) -> ::core::ffi::c_int,
    >,
    pub backup_remaining: Option<
        unsafe extern "C" fn(*mut sqlite3_backup) -> ::core::ffi::c_int,
    >,
    pub backup_step: Option<
        unsafe extern "C" fn(
            *mut sqlite3_backup,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub compileoption_get: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *const ::core::ffi::c_char,
    >,
    pub compileoption_used: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub create_function_v2: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub db_config: Option<
        unsafe extern "C" fn(*mut sqlite3, ::core::ffi::c_int, ...) -> ::core::ffi::c_int,
    >,
    pub db_mutex: Option<unsafe extern "C" fn(*mut sqlite3) -> *mut sqlite3_mutex>,
    pub db_status: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub extended_errcode: Option<
        unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int,
    >,
    pub log: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char, ...) -> (),
    >,
    pub soft_heap_limit64: Option<unsafe extern "C" fn(sqlite3_int64) -> sqlite3_int64>,
    pub sourceid: Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
    pub stmt_status: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub strnicmp: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub unlock_notify: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            Option<
                unsafe extern "C" fn(
                    *mut *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> (),
            >,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub wal_autocheckpoint: Option<
        unsafe extern "C" fn(*mut sqlite3, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub wal_checkpoint: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub wal_hook: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut sqlite3,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub blob_reopen: Option<
        unsafe extern "C" fn(*mut sqlite3_blob, sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub vtab_config: Option<
        unsafe extern "C" fn(*mut sqlite3, ::core::ffi::c_int, ...) -> ::core::ffi::c_int,
    >,
    pub vtab_on_conflict: Option<
        unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int,
    >,
    pub close_v2: Option<unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int>,
    pub db_filename: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >,
    pub db_readonly: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub db_release_memory: Option<
        unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int,
    >,
    pub errstr: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *const ::core::ffi::c_char,
    >,
    pub stmt_busy: Option<unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int>,
    pub stmt_readonly: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int,
    >,
    pub stricmp: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub uri_boolean: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub uri_int64: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            sqlite3_int64,
        ) -> sqlite3_int64,
    >,
    pub uri_parameter: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >,
    pub xvsnprintf: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::VaList,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub wal_checkpoint_v2: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub auto_extension: Option<
        unsafe extern "C" fn(Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int,
    >,
    pub bind_blob64: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            sqlite3_uint64,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_text64: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            sqlite3_uint64,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ::core::ffi::c_uchar,
        ) -> ::core::ffi::c_int,
    >,
    pub cancel_auto_extension: Option<
        unsafe extern "C" fn(Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int,
    >,
    pub load_extension: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub malloc64: Option<
        unsafe extern "C" fn(sqlite3_uint64) -> *mut ::core::ffi::c_void,
    >,
    pub msize: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> sqlite3_uint64>,
    pub realloc64: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            sqlite3_uint64,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub reset_auto_extension: Option<unsafe extern "C" fn() -> ()>,
    pub result_blob64: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            *const ::core::ffi::c_void,
            sqlite3_uint64,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> (),
    >,
    pub result_text64: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            *const ::core::ffi::c_char,
            sqlite3_uint64,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            ::core::ffi::c_uchar,
        ) -> (),
    >,
    pub strglob: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub value_dup: Option<
        unsafe extern "C" fn(*const sqlite3_value) -> *mut sqlite3_value,
    >,
    pub value_free: Option<unsafe extern "C" fn(*mut sqlite3_value) -> ()>,
    pub result_zeroblob64: Option<
        unsafe extern "C" fn(*mut sqlite3_context, sqlite3_uint64) -> ::core::ffi::c_int,
    >,
    pub bind_zeroblob64: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            sqlite3_uint64,
        ) -> ::core::ffi::c_int,
    >,
    pub value_subtype: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> ::core::ffi::c_uint,
    >,
    pub result_subtype: Option<
        unsafe extern "C" fn(*mut sqlite3_context, ::core::ffi::c_uint) -> (),
    >,
    pub status64: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut sqlite3_int64,
            *mut sqlite3_int64,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub strlike: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
    pub db_cacheflush: Option<unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int>,
    pub system_errno: Option<unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int>,
    pub trace_v2: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            ::core::ffi::c_uint,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_uint,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub expanded_sql: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt) -> *mut ::core::ffi::c_char,
    >,
    pub set_last_insert_rowid: Option<
        unsafe extern "C" fn(*mut sqlite3, sqlite3_int64) -> (),
    >,
    pub prepare_v3: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_uint,
            *mut *mut sqlite3_stmt,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare16_v3: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_uint,
            *mut *mut sqlite3_stmt,
            *mut *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_pointer: Option<
        unsafe extern "C" fn(
            *mut sqlite3_stmt,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub result_pointer: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> (),
    >,
    pub value_pointer: Option<
        unsafe extern "C" fn(
            *mut sqlite3_value,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub vtab_nochange: Option<
        unsafe extern "C" fn(*mut sqlite3_context) -> ::core::ffi::c_int,
    >,
    pub value_nochange: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> ::core::ffi::c_int,
    >,
    pub vtab_collation: Option<
        unsafe extern "C" fn(
            *mut sqlite3_index_info,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub keyword_count: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub keyword_name: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub keyword_check: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub str_new: Option<unsafe extern "C" fn(*mut sqlite3) -> *mut sqlite3_str>,
    pub str_finish: Option<
        unsafe extern "C" fn(*mut sqlite3_str) -> *mut ::core::ffi::c_char,
    >,
    pub str_appendf: Option<
        unsafe extern "C" fn(*mut sqlite3_str, *const ::core::ffi::c_char, ...) -> (),
    >,
    pub str_vappendf: Option<
        unsafe extern "C" fn(
            *mut sqlite3_str,
            *const ::core::ffi::c_char,
            ::core::ffi::VaList,
        ) -> (),
    >,
    pub str_append: Option<
        unsafe extern "C" fn(
            *mut sqlite3_str,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub str_appendall: Option<
        unsafe extern "C" fn(*mut sqlite3_str, *const ::core::ffi::c_char) -> (),
    >,
    pub str_appendchar: Option<
        unsafe extern "C" fn(
            *mut sqlite3_str,
            ::core::ffi::c_int,
            ::core::ffi::c_char,
        ) -> (),
    >,
    pub str_reset: Option<unsafe extern "C" fn(*mut sqlite3_str) -> ()>,
    pub str_errcode: Option<
        unsafe extern "C" fn(*mut sqlite3_str) -> ::core::ffi::c_int,
    >,
    pub str_length: Option<unsafe extern "C" fn(*mut sqlite3_str) -> ::core::ffi::c_int>,
    pub str_value: Option<
        unsafe extern "C" fn(*mut sqlite3_str) -> *mut ::core::ffi::c_char,
    >,
    pub create_window_function: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
            Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub normalized_sql: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt) -> *const ::core::ffi::c_char,
    >,
    pub stmt_isexplain: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt) -> ::core::ffi::c_int,
    >,
    pub value_frombind: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> ::core::ffi::c_int,
    >,
    pub drop_modules: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub hard_heap_limit64: Option<unsafe extern "C" fn(sqlite3_int64) -> sqlite3_int64>,
    pub uri_key: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub filename_database: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> *const ::core::ffi::c_char,
    >,
    pub filename_journal: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> *const ::core::ffi::c_char,
    >,
    pub filename_wal: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> *const ::core::ffi::c_char,
    >,
    pub create_filename: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >,
    pub free_filename: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>,
    pub database_file_object: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut sqlite3_file,
    >,
    pub txn_state: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub changes64: Option<unsafe extern "C" fn(*mut sqlite3) -> sqlite3_int64>,
    pub total_changes64: Option<unsafe extern "C" fn(*mut sqlite3) -> sqlite3_int64>,
    pub autovacuum_pages: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                ) -> ::core::ffi::c_uint,
            >,
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub error_offset: Option<unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int>,
    pub vtab_rhs_value: Option<
        unsafe extern "C" fn(
            *mut sqlite3_index_info,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub vtab_distinct: Option<
        unsafe extern "C" fn(*mut sqlite3_index_info) -> ::core::ffi::c_int,
    >,
    pub vtab_in: Option<
        unsafe extern "C" fn(
            *mut sqlite3_index_info,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub vtab_in_first: Option<
        unsafe extern "C" fn(
            *mut sqlite3_value,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub vtab_in_next: Option<
        unsafe extern "C" fn(
            *mut sqlite3_value,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub deserialize: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_uchar,
            sqlite3_int64,
            sqlite3_int64,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
    pub serialize: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            *mut sqlite3_int64,
            ::core::ffi::c_uint,
        ) -> *mut ::core::ffi::c_uchar,
    >,
    pub db_name: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub value_encoding: Option<
        unsafe extern "C" fn(*mut sqlite3_value) -> ::core::ffi::c_int,
    >,
    pub is_interrupted: Option<unsafe extern "C" fn(*mut sqlite3) -> ::core::ffi::c_int>,
    pub stmt_explain: Option<
        unsafe extern "C" fn(*mut sqlite3_stmt, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub get_clientdata: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub set_clientdata: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub setlk_timeout: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub set_errmsg: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub db_status64: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            ::core::ffi::c_int,
            *mut sqlite3_int64,
            *mut sqlite3_int64,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
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
pub type sqlite3_filename = *const ::core::ffi::c_char;
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
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: ::core::ffi::c_int,
    pub zErrMsg: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
}
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnionCsr {
    pub base: sqlite3_vtab_cursor,
    pub pStmt: *mut sqlite3_stmt,
    pub iMaxRowid: sqlite3_int64,
    pub iTab: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnionTab {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub bSwarm: ::core::ffi::c_int,
    pub iPK: ::core::ffi::c_int,
    pub nSrc: ::core::ffi::c_int,
    pub aSrc: *mut UnionSrc,
    pub bHasContext: ::core::ffi::c_int,
    pub zSourceStr: *mut ::core::ffi::c_char,
    pub pNotFound: *mut sqlite3_stmt,
    pub pOpenClose: *mut sqlite3_stmt,
    pub pClosable: *mut UnionSrc,
    pub nOpen: ::core::ffi::c_int,
    pub nMaxOpen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnionSrc {
    pub zDb: *mut ::core::ffi::c_char,
    pub zTab: *mut ::core::ffi::c_char,
    pub iMin: sqlite3_int64,
    pub iMax: sqlite3_int64,
    pub zFile: *mut ::core::ffi::c_char,
    pub zContext: *mut ::core::ffi::c_char,
    pub nUser: ::core::ffi::c_int,
    pub db: *mut sqlite3,
    pub pNextClosable: *mut UnionSrc,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_OPEN_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_SCAN_UNIQUE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_GT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_GE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const LARGEST_INT64: sqlite3_int64 = 0xffffffff as sqlite3_int64
    | (0x7fffffff as ::core::ffi::c_int as sqlite3_int64) << 32 as ::core::ffi::c_int;
pub const SMALLEST_INT64: sqlite3_int64 = -1 as ::core::ffi::c_int as sqlite3_int64
    - LARGEST_INT64;
pub const SWARMVTAB_MAX_OPEN: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
unsafe extern "C" fn unionMalloc(
    mut pRc: *mut ::core::ffi::c_int,
    mut nByte: sqlite3_int64,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pRet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if *pRc == SQLITE_OK {
            pRet = sqlite3_malloc64(nByte as sqlite3_uint64);
            if !pRet.is_null() {
                memset(pRet, 0 as ::core::ffi::c_int, nByte as size_t);
            } else {
                *pRc = SQLITE_NOMEM;
            }
        } else {
            pRet = ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        return pRet;
    }
}
unsafe extern "C" fn unionStrdup(
    mut pRc: *mut ::core::ffi::c_int,
    mut zIn: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if !zIn.is_null() {
            let mut nByte: sqlite3_int64 = strlen(zIn).wrapping_add(1 as size_t)
                as sqlite3_int64;
            zRet = unionMalloc(pRc, nByte) as *mut ::core::ffi::c_char;
            if !zRet.is_null() {
                memcpy(
                    zRet as *mut ::core::ffi::c_void,
                    zIn as *const ::core::ffi::c_void,
                    nByte as size_t,
                );
            }
        }
        return zRet;
    }
}
unsafe extern "C" fn unionDequote(mut z: *mut ::core::ffi::c_char) {
    unsafe {
        if !z.is_null() {
            let mut q: ::core::ffi::c_char = *z.offset(0 as ::core::ffi::c_int as isize);
            if q as ::core::ffi::c_int == '[' as i32
                || q as ::core::ffi::c_int == '\'' as i32
                || q as ::core::ffi::c_int == '"' as i32
                || q as ::core::ffi::c_int == '`' as i32
            {
                let mut iIn: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if q as ::core::ffi::c_int == '[' as i32 {
                    q = ']' as i32 as ::core::ffi::c_char;
                }
                while *z.offset(iIn as isize) != 0 {
                    if *z.offset(iIn as isize) as ::core::ffi::c_int
                        == q as ::core::ffi::c_int
                    {
                        if *z.offset((iIn + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int != q as ::core::ffi::c_int
                        {
                            iIn += 1;
                            break;
                        } else {
                            iIn += 2 as ::core::ffi::c_int;
                            let c2rust_fresh0 = iOut;
                            iOut = iOut + 1;
                            *z.offset(c2rust_fresh0 as isize) = q;
                        }
                    } else {
                        let c2rust_fresh1 = iIn;
                        iIn = iIn + 1;
                        let c2rust_fresh2 = iOut;
                        iOut = iOut + 1;
                        *z.offset(c2rust_fresh2 as isize) = *z
                            .offset(c2rust_fresh1 as isize);
                    }
                }
                *z.offset(iOut as isize) = '\0' as i32 as ::core::ffi::c_char;
            }
        }
    }
}
unsafe extern "C" fn unionPrepare(
    mut pRc: *mut ::core::ffi::c_int,
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> *mut sqlite3_stmt {
    unsafe {
        let mut pRet: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if *pRc == SQLITE_OK {
            let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
                db,
                zSql,
                -1 as ::core::ffi::c_int,
                &raw mut pRet,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc != SQLITE_OK {
                *pzErr = sqlite3_mprintf(
                    b"sql error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg(db),
                );
                *pRc = rc;
            }
        }
        return pRet;
    }
}
unsafe extern "C" fn unionPreparePrintf(
    mut pRc: *mut ::core::ffi::c_int,
    mut pzErr: *mut *mut ::core::ffi::c_char,
    mut db: *mut sqlite3,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut sqlite3_stmt {
    unsafe {
        let mut pRet: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ap = c2rust_args.clone();
        zSql = sqlite3_vmprintf(zFmt, ap);
        if *pRc == SQLITE_OK {
            if zSql.is_null() {
                *pRc = SQLITE_NOMEM;
            } else {
                pRet = unionPrepare(pRc, db, zSql, pzErr);
            }
        }
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        return pRet;
    }
}
unsafe extern "C" fn unionFinalize(
    mut pRc: *mut ::core::ffi::c_int,
    mut pStmt: *mut sqlite3_stmt,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_db_handle(pStmt);
        let mut rc: ::core::ffi::c_int = sqlite3_finalize(pStmt);
        if *pRc == SQLITE_OK {
            *pRc = rc;
            if rc != 0 {
                *pzErr = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg(db),
                );
            }
        }
    }
}
unsafe extern "C" fn unionInvokeOpenClose(
    mut pTab: *mut UnionTab,
    mut pSrc: *mut UnionSrc,
    mut bClose: ::core::ffi::c_int,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !(*pTab).pOpenClose.is_null() {
            sqlite3_bind_text(
                (*pTab).pOpenClose,
                1 as ::core::ffi::c_int,
                (*pSrc).zFile,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            if (*pTab).bHasContext != 0 {
                sqlite3_bind_text(
                    (*pTab).pOpenClose,
                    2 as ::core::ffi::c_int,
                    (*pSrc).zContext,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            sqlite3_bind_int(
                (*pTab).pOpenClose,
                2 as ::core::ffi::c_int + (*pTab).bHasContext,
                bClose,
            );
            sqlite3_step((*pTab).pOpenClose);
            rc = sqlite3_reset((*pTab).pOpenClose);
            if SQLITE_OK != rc {
                if !pzErr.is_null() {
                    *pzErr = sqlite3_mprintf(
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        sqlite3_errmsg((*pTab).db),
                    );
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn unionCloseSources(
    mut pTab: *mut UnionTab,
    mut nMax: ::core::ffi::c_int,
) {
    unsafe {
        while !(*pTab).pClosable.is_null() && (*pTab).nOpen > nMax {
            let mut p: *mut UnionSrc = ::core::ptr::null_mut::<UnionSrc>();
            let mut pp: *mut *mut UnionSrc = ::core::ptr::null_mut::<*mut UnionSrc>();
            pp = &raw mut (*pTab).pClosable;
            while !(**pp).pNextClosable.is_null() {
                pp = &raw mut (**pp).pNextClosable;
            }
            p = *pp;
            sqlite3_close((*p).db);
            (*p).db = ::core::ptr::null_mut::<sqlite3>();
            *pp = ::core::ptr::null_mut::<UnionSrc>();
            (*pTab).nOpen -= 1;
            unionInvokeOpenClose(
                pTab,
                p,
                1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
    }
}
unsafe extern "C" fn unionDisconnect(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        if !pVtab.is_null() {
            let mut pTab: *mut UnionTab = pVtab as *mut UnionTab;
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pTab).nSrc {
                let mut pSrc: *mut UnionSrc = (*pTab).aSrc.offset(i as isize)
                    as *mut UnionSrc;
                let mut bHaveSrcDb: ::core::ffi::c_int = !(*pSrc).db.is_null()
                    as ::core::ffi::c_int;
                sqlite3_close((*pSrc).db);
                if bHaveSrcDb != 0 {
                    unionInvokeOpenClose(
                        pTab,
                        pSrc,
                        1 as ::core::ffi::c_int,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    );
                }
                sqlite3_free((*pSrc).zDb as *mut ::core::ffi::c_void);
                sqlite3_free((*pSrc).zTab as *mut ::core::ffi::c_void);
                sqlite3_free((*pSrc).zFile as *mut ::core::ffi::c_void);
                sqlite3_free((*pSrc).zContext as *mut ::core::ffi::c_void);
                i += 1;
            }
            sqlite3_finalize((*pTab).pNotFound);
            sqlite3_finalize((*pTab).pOpenClose);
            sqlite3_free((*pTab).zSourceStr as *mut ::core::ffi::c_void);
            sqlite3_free((*pTab).aSrc as *mut ::core::ffi::c_void);
            sqlite3_free(pTab as *mut ::core::ffi::c_void);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn unionIsIntkeyTable(
    mut db: *mut sqlite3,
    mut pSrc: *mut UnionSrc,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut bPk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        sqlite3_table_column_metadata(
            db,
            (*pSrc).zDb,
            (*pSrc).zTab,
            b"_rowid_\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut zType,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            &raw mut bPk,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        rc = sqlite3_errcode(db);
        if rc == SQLITE_ERROR
            || rc == SQLITE_OK
                && (bPk == 0
                    || sqlite3_stricmp(
                        b"integer\0".as_ptr() as *const ::core::ffi::c_char,
                        zType,
                    ) != 0)
        {
            rc = SQLITE_ERROR;
            *pzErr = sqlite3_mprintf(
                b"no such rowid table: %s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                if !(*pSrc).zDb.is_null() {
                    (*pSrc).zDb as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                if !(*pSrc).zDb.is_null() {
                    b".\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                (*pSrc).zTab,
            );
        }
        return rc;
    }
}
unsafe extern "C" fn unionSourceToStr(
    mut pRc: *mut ::core::ffi::c_int,
    mut pTab: *mut UnionTab,
    mut pSrc: *mut UnionSrc,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if *pRc == SQLITE_OK {
            let mut db: *mut sqlite3 = if (*pTab).bSwarm != 0 {
                (*pSrc).db
            } else {
                (*pTab).db
            };
            let mut rc: ::core::ffi::c_int = unionIsIntkeyTable(db, pSrc, pzErr);
            let mut pStmt: *mut sqlite3_stmt = unionPrepare(
                &raw mut rc,
                db,
                b"SELECT group_concat(quote(name) || '.' || quote(type)) FROM pragma_table_info(?, ?)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                pzErr,
            );
            if rc == SQLITE_OK {
                sqlite3_bind_text(
                    pStmt,
                    1 as ::core::ffi::c_int,
                    (*pSrc).zTab,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                sqlite3_bind_text(
                    pStmt,
                    2 as ::core::ffi::c_int,
                    (*pSrc).zDb,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                if SQLITE_ROW == sqlite3_step(pStmt) {
                    let mut z: *const ::core::ffi::c_char = sqlite3_column_text(
                        pStmt,
                        0 as ::core::ffi::c_int,
                    ) as *const ::core::ffi::c_char;
                    zRet = unionStrdup(&raw mut rc, z);
                }
                unionFinalize(&raw mut rc, pStmt, pzErr);
            }
            *pRc = rc;
        }
        return zRet;
    }
}
unsafe extern "C" fn unionSourceCheck(
    mut pTab: *mut UnionTab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut z0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        z0 = unionSourceToStr(
            &raw mut rc,
            pTab,
            (*pTab).aSrc.offset(0 as ::core::ffi::c_int as isize) as *mut UnionSrc,
            pzErr,
        );
        i = 1 as ::core::ffi::c_int;
        while i < (*pTab).nSrc {
            let mut z: *mut ::core::ffi::c_char = unionSourceToStr(
                &raw mut rc,
                pTab,
                (*pTab).aSrc.offset(i as isize) as *mut UnionSrc,
                pzErr,
            );
            if rc == SQLITE_OK && sqlite3_stricmp(z, z0) != 0 {
                *pzErr = sqlite3_mprintf(
                    b"source table schema mismatch\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                rc = SQLITE_ERROR;
            }
            sqlite3_free(z as *mut ::core::ffi::c_void);
            i += 1;
        }
        sqlite3_free(z0 as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn unionOpenDatabaseInner(
    mut pTab: *mut UnionTab,
    mut pSrc: *mut UnionSrc,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        static mut openFlags: ::core::ffi::c_int = SQLITE_OPEN_READONLY
            | SQLITE_OPEN_URI;
        let mut rc: ::core::ffi::c_int = 0;
        rc = unionInvokeOpenClose(pTab, pSrc, 0 as ::core::ffi::c_int, pzErr);
        if rc != SQLITE_OK {
            return rc;
        }
        rc = sqlite3_open_v2(
            (*pSrc).zFile,
            &raw mut (*pSrc).db,
            openFlags,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            return rc;
        }
        if !(*pTab).pNotFound.is_null() {
            sqlite3_close((*pSrc).db);
            (*pSrc).db = ::core::ptr::null_mut::<sqlite3>();
            sqlite3_bind_text(
                (*pTab).pNotFound,
                1 as ::core::ffi::c_int,
                (*pSrc).zFile,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            if (*pTab).bHasContext != 0 {
                sqlite3_bind_text(
                    (*pTab).pNotFound,
                    2 as ::core::ffi::c_int,
                    (*pSrc).zContext,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            sqlite3_step((*pTab).pNotFound);
            rc = sqlite3_reset((*pTab).pNotFound);
            if SQLITE_OK != rc {
                *pzErr = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg((*pTab).db),
                );
                return rc;
            }
            rc = sqlite3_open_v2(
                (*pSrc).zFile,
                &raw mut (*pSrc).db,
                openFlags,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        if rc != SQLITE_OK {
            *pzErr = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg((*pSrc).db),
            );
        }
        return rc;
    }
}
unsafe extern "C" fn unionOpenDatabase(
    mut pTab: *mut UnionTab,
    mut iSrc: ::core::ffi::c_int,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pSrc: *mut UnionSrc = (*pTab).aSrc.offset(iSrc as isize)
            as *mut UnionSrc;
        if (*pSrc).db.is_null() {
            unionCloseSources(pTab, (*pTab).nMaxOpen - 1 as ::core::ffi::c_int);
            rc = unionOpenDatabaseInner(pTab, pSrc, pzErr);
            if rc == SQLITE_OK {
                let mut z: *mut ::core::ffi::c_char = unionSourceToStr(
                    &raw mut rc,
                    pTab,
                    pSrc,
                    pzErr,
                );
                if rc == SQLITE_OK {
                    if (*pTab).zSourceStr.is_null() {
                        (*pTab).zSourceStr = z;
                    } else {
                        if sqlite3_stricmp(z, (*pTab).zSourceStr) != 0 {
                            *pzErr = sqlite3_mprintf(
                                b"source table schema mismatch\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            rc = SQLITE_ERROR;
                        }
                        sqlite3_free(z as *mut ::core::ffi::c_void);
                    }
                }
            }
            if rc == SQLITE_OK {
                (*pSrc).pNextClosable = (*pTab).pClosable;
                (*pTab).pClosable = pSrc;
                (*pTab).nOpen += 1;
            } else {
                sqlite3_close((*pSrc).db);
                (*pSrc).db = ::core::ptr::null_mut::<sqlite3>();
                unionInvokeOpenClose(
                    pTab,
                    pSrc,
                    1 as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
            }
        }
        return rc;
    }
}
unsafe extern "C" fn unionIncrRefcount(
    mut pTab: *mut UnionTab,
    mut iTab: ::core::ffi::c_int,
) {
    unsafe {
        if (*pTab).bSwarm != 0 {
            let mut pSrc: *mut UnionSrc = (*pTab).aSrc.offset(iTab as isize)
                as *mut UnionSrc;
            if (*pSrc).nUser == 0 as ::core::ffi::c_int {
                let mut pp: *mut *mut UnionSrc = ::core::ptr::null_mut::<
                    *mut UnionSrc,
                >();
                pp = &raw mut (*pTab).pClosable;
                while *pp != pSrc {
                    pp = &raw mut (**pp).pNextClosable;
                }
                *pp = (*pSrc).pNextClosable;
                (*pSrc).pNextClosable = ::core::ptr::null_mut::<UnionSrc>();
            }
            (*pSrc).nUser += 1;
        }
    }
}
unsafe extern "C" fn unionFinalizeCsrStmt(
    mut pCsr: *mut UnionCsr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !(*pCsr).pStmt.is_null() {
            let mut pTab: *mut UnionTab = (*pCsr).base.pVtab as *mut UnionTab;
            let mut pSrc: *mut UnionSrc = (*pTab).aSrc.offset((*pCsr).iTab as isize)
                as *mut UnionSrc;
            rc = sqlite3_finalize((*pCsr).pStmt);
            (*pCsr).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            if (*pTab).bSwarm != 0 {
                (*pSrc).nUser -= 1;
                if (*pSrc).nUser == 0 as ::core::ffi::c_int {
                    (*pSrc).pNextClosable = (*pTab).pClosable;
                    (*pTab).pClosable = pSrc;
                }
                unionCloseSources(pTab, (*pTab).nMaxOpen);
            }
        }
        return rc;
    }
}
unsafe extern "C" fn union_isspace(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        return (c as ::core::ffi::c_int == ' ' as i32
            || c as ::core::ffi::c_int == '\n' as i32
            || c as ::core::ffi::c_int == '\r' as i32
            || c as ::core::ffi::c_int == '\t' as i32) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn union_isidchar(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        return (c as ::core::ffi::c_int >= 'a' as i32
            && c as ::core::ffi::c_int <= 'z' as i32
            || c as ::core::ffi::c_int >= 'A' as i32
                && (c as ::core::ffi::c_int) < 'Z' as i32
            || c as ::core::ffi::c_int >= '0' as i32
                && c as ::core::ffi::c_int <= '9' as i32) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn unionConfigureVtab(
    mut pRc: *mut ::core::ffi::c_int,
    mut pTab: *mut UnionTab,
    mut pStmt: *mut sqlite3_stmt,
    mut nArg: ::core::ffi::c_int,
    mut azArg: *const *const ::core::ffi::c_char,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut rc: ::core::ffi::c_int = *pRc;
        let mut i: ::core::ffi::c_int = 0;
        if rc == SQLITE_OK {
            (*pTab).bHasContext = (sqlite3_column_count(pStmt) > 4 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && i < nArg {
            let mut zArg: *mut ::core::ffi::c_char = unionStrdup(
                &raw mut rc,
                *azArg.offset(i as isize),
            );
            if !zArg.is_null() {
                let mut nOpt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut zOpt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zVal: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                unionDequote(zArg);
                zOpt = zArg;
                while union_isspace(*zOpt) != 0 {
                    zOpt = zOpt.offset(1);
                }
                zVal = zOpt;
                if *zVal as ::core::ffi::c_int == ':' as i32 {
                    zVal = zVal.offset(1);
                }
                while union_isidchar(*zVal) != 0 {
                    zVal = zVal.offset(1);
                }
                nOpt = zVal.offset_from(zOpt) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
                while union_isspace(*zVal) != 0 {
                    zVal = zVal.offset(1);
                }
                if *zVal as ::core::ffi::c_int == '=' as i32 {
                    *zOpt.offset(nOpt as isize) = '\0' as i32 as ::core::ffi::c_char;
                    zVal = zVal.offset(1);
                    while union_isspace(*zVal) != 0 {
                        zVal = zVal.offset(1);
                    }
                    zVal = unionStrdup(&raw mut rc, zVal);
                    if !zVal.is_null() {
                        unionDequote(zVal);
                        if *zOpt.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == ':' as i32
                        {
                            let mut iParam: ::core::ffi::c_int = sqlite3_bind_parameter_index(
                                pStmt,
                                zOpt,
                            );
                            if iParam == 0 as ::core::ffi::c_int {
                                *pzErr = sqlite3_mprintf(
                                    b"swarmvtab: no such SQL parameter: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    zOpt,
                                );
                                rc = SQLITE_ERROR;
                            } else {
                                rc = sqlite3_bind_text(
                                    pStmt,
                                    iParam,
                                    zVal,
                                    -1 as ::core::ffi::c_int,
                                    ::core::mem::transmute::<
                                        ::libc::intptr_t,
                                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                                );
                            }
                        } else if nOpt == 7 as ::core::ffi::c_int
                            && 0 as ::core::ffi::c_int
                                == sqlite3_strnicmp(
                                    zOpt,
                                    b"maxopen\0".as_ptr() as *const ::core::ffi::c_char,
                                    7 as ::core::ffi::c_int,
                                )
                        {
                            (*pTab).nMaxOpen = atoi(zVal);
                            if (*pTab).nMaxOpen <= 0 as ::core::ffi::c_int {
                                *pzErr = sqlite3_mprintf(
                                    b"swarmvtab: illegal maxopen value\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                rc = SQLITE_ERROR;
                            }
                        } else if nOpt == 7 as ::core::ffi::c_int
                            && 0 as ::core::ffi::c_int
                                == sqlite3_strnicmp(
                                    zOpt,
                                    b"missing\0".as_ptr() as *const ::core::ffi::c_char,
                                    7 as ::core::ffi::c_int,
                                )
                        {
                            if !(*pTab).pNotFound.is_null() {
                                *pzErr = sqlite3_mprintf(
                                    b"swarmvtab: duplicate \"missing\" option\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                rc = SQLITE_ERROR;
                            } else {
                                (*pTab).pNotFound = unionPreparePrintf(
                                    &raw mut rc,
                                    pzErr,
                                    (*pTab).db,
                                    b"SELECT \"%w\"(?%s)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    zVal,
                                    if (*pTab).bHasContext != 0 {
                                        b",?\0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"\0".as_ptr() as *const ::core::ffi::c_char
                                    },
                                );
                            }
                        } else if nOpt == 9 as ::core::ffi::c_int
                            && 0 as ::core::ffi::c_int
                                == sqlite3_strnicmp(
                                    zOpt,
                                    b"openclose\0".as_ptr() as *const ::core::ffi::c_char,
                                    9 as ::core::ffi::c_int,
                                )
                        {
                            if !(*pTab).pOpenClose.is_null() {
                                *pzErr = sqlite3_mprintf(
                                    b"swarmvtab: duplicate \"openclose\" option\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                rc = SQLITE_ERROR;
                            } else {
                                (*pTab).pOpenClose = unionPreparePrintf(
                                    &raw mut rc,
                                    pzErr,
                                    (*pTab).db,
                                    b"SELECT \"%w\"(?,?%s)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    zVal,
                                    if (*pTab).bHasContext != 0 {
                                        b",?\0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"\0".as_ptr() as *const ::core::ffi::c_char
                                    },
                                );
                            }
                        } else {
                            *pzErr = sqlite3_mprintf(
                                b"swarmvtab: unrecognized option: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                zOpt,
                            );
                            rc = SQLITE_ERROR;
                        }
                        sqlite3_free(zVal as *mut ::core::ffi::c_void);
                    }
                } else if i == 0 as ::core::ffi::c_int && nArg == 1 as ::core::ffi::c_int
                {
                    (*pTab).pNotFound = unionPreparePrintf(
                        &raw mut rc,
                        pzErr,
                        (*pTab).db,
                        b"SELECT \"%w\"(?)\0".as_ptr() as *const ::core::ffi::c_char,
                        zArg,
                    );
                } else {
                    *pzErr = sqlite3_mprintf(
                        b"swarmvtab: parse error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        *azArg.offset(i as isize),
                    );
                    rc = SQLITE_ERROR;
                }
                sqlite3_free(zArg as *mut ::core::ffi::c_void);
            }
            i += 1;
        }
        *pRc = rc;
    }
}
unsafe extern "C" fn unionConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut UnionTab = ::core::ptr::null_mut::<UnionTab>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut bSwarm: ::core::ffi::c_int = if pAux.is_null() {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        let mut zVtab: *const ::core::ffi::c_char = if bSwarm != 0 {
            b"swarmvtab\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"unionvtab\0".as_ptr() as *const ::core::ffi::c_char
        };
        if sqlite3_stricmp(
            b"temp\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(1 as ::core::ffi::c_int as isize),
        ) != 0
        {
            *pzErr = sqlite3_mprintf(
                b"%s tables must be created in TEMP schema\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zVtab,
            );
            rc = SQLITE_ERROR;
        } else if argc < 4 as ::core::ffi::c_int
            || argc > 4 as ::core::ffi::c_int && bSwarm == 0 as ::core::ffi::c_int
        {
            *pzErr = sqlite3_mprintf(
                b"wrong number of arguments for %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zVtab,
            );
            rc = SQLITE_ERROR;
        } else {
            let mut nAlloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            let mut zArg: *mut ::core::ffi::c_char = unionStrdup(
                &raw mut rc,
                *argv.offset(3 as ::core::ffi::c_int as isize),
            );
            unionDequote(zArg);
            pStmt = unionPreparePrintf(
                &raw mut rc,
                pzErr,
                db,
                b"SELECT * FROM (%z) ORDER BY 3\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zArg,
            );
            pTab = unionMalloc(
                &raw mut rc,
                ::core::mem::size_of::<UnionTab>() as sqlite3_int64,
            ) as *mut UnionTab;
            if !pTab.is_null() {
                (*pTab).db = db;
                (*pTab).bSwarm = bSwarm;
                (*pTab).nMaxOpen = SWARMVTAB_MAX_OPEN;
            }
            if bSwarm != 0 {
                unionConfigureVtab(
                    &raw mut rc,
                    pTab,
                    pStmt,
                    argc - 4 as ::core::ffi::c_int,
                    argv.offset(4 as ::core::ffi::c_int as isize)
                        as *const *const ::core::ffi::c_char,
                    pzErr,
                );
            }
            while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pStmt) {
                let mut zDb: *const ::core::ffi::c_char = sqlite3_column_text(
                    pStmt,
                    0 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                let mut zTab: *const ::core::ffi::c_char = sqlite3_column_text(
                    pStmt,
                    1 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                let mut iMin: sqlite3_int64 = sqlite3_column_int64(
                    pStmt,
                    2 as ::core::ffi::c_int,
                );
                let mut iMax: sqlite3_int64 = sqlite3_column_int64(
                    pStmt,
                    3 as ::core::ffi::c_int,
                );
                let mut pSrc: *mut UnionSrc = ::core::ptr::null_mut::<UnionSrc>();
                if nAlloc <= (*pTab).nSrc {
                    let mut nNew: ::core::ffi::c_int = if nAlloc != 0 {
                        nAlloc * 2 as ::core::ffi::c_int
                    } else {
                        8 as ::core::ffi::c_int
                    };
                    let mut aNew: *mut UnionSrc = sqlite3_realloc64(
                        (*pTab).aSrc as *mut ::core::ffi::c_void,
                        (nNew as usize)
                            .wrapping_mul(::core::mem::size_of::<UnionSrc>() as usize)
                            as sqlite3_uint64,
                    ) as *mut UnionSrc;
                    if aNew.is_null() {
                        rc = SQLITE_NOMEM;
                        break;
                    } else {
                        memset(
                            aNew.offset((*pTab).nSrc as isize) as *mut UnionSrc
                                as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ((nNew - (*pTab).nSrc) as size_t)
                                .wrapping_mul(::core::mem::size_of::<UnionSrc>() as size_t),
                        );
                        (*pTab).aSrc = aNew;
                        nAlloc = nNew;
                    }
                }
                if iMax < iMin
                    || (*pTab).nSrc > 0 as ::core::ffi::c_int
                        && iMin
                            <= (*(*pTab)
                                .aSrc
                                .offset(((*pTab).nSrc - 1 as ::core::ffi::c_int) as isize))
                                .iMax
                {
                    *pzErr = sqlite3_mprintf(
                        b"rowid range mismatch error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    rc = SQLITE_ERROR;
                }
                if rc == SQLITE_OK {
                    let c2rust_fresh3 = (*pTab).nSrc;
                    (*pTab).nSrc = (*pTab).nSrc + 1;
                    pSrc = (*pTab).aSrc.offset(c2rust_fresh3 as isize) as *mut UnionSrc;
                    (*pSrc).zTab = unionStrdup(&raw mut rc, zTab);
                    (*pSrc).iMin = iMin;
                    (*pSrc).iMax = iMax;
                    if bSwarm != 0 {
                        (*pSrc).zFile = unionStrdup(&raw mut rc, zDb);
                    } else {
                        (*pSrc).zDb = unionStrdup(&raw mut rc, zDb);
                    }
                    if (*pTab).bHasContext != 0 {
                        let mut zContext: *const ::core::ffi::c_char = sqlite3_column_text(
                            pStmt,
                            4 as ::core::ffi::c_int,
                        ) as *const ::core::ffi::c_char;
                        (*pSrc).zContext = unionStrdup(&raw mut rc, zContext);
                    }
                }
            }
            unionFinalize(&raw mut rc, pStmt, pzErr);
            pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            if rc == SQLITE_OK && (*pTab).nSrc == 0 as ::core::ffi::c_int {
                *pzErr = sqlite3_mprintf(
                    b"no source tables configured\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                rc = SQLITE_ERROR;
            }
            if rc == SQLITE_OK {
                if bSwarm != 0 {
                    rc = unionOpenDatabase(pTab, 0 as ::core::ffi::c_int, pzErr);
                } else {
                    rc = unionSourceCheck(pTab, pzErr);
                }
            }
            if rc == SQLITE_OK {
                let mut pSrc_0: *mut UnionSrc = (*pTab)
                    .aSrc
                    .offset(0 as ::core::ffi::c_int as isize) as *mut UnionSrc;
                let mut tdb: *mut sqlite3 = if (*pTab).bSwarm != 0 {
                    (*pSrc_0).db
                } else {
                    (*pTab).db
                };
                pStmt = unionPreparePrintf(
                    &raw mut rc,
                    pzErr,
                    tdb,
                    b"SELECT 'CREATE TABLE xyz('    || group_concat(quote(name) || ' ' || type, ', ')    || ')',max((cid+1) * (type='INTEGER' COLLATE nocase AND pk=1))-1 FROM pragma_table_info(%Q, ?)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*pSrc_0).zTab,
                    (*pSrc_0).zDb,
                );
            }
            if rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pStmt) {
                let mut zDecl: *const ::core::ffi::c_char = sqlite3_column_text(
                    pStmt,
                    0 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                rc = sqlite3_declare_vtab(db, zDecl);
                (*pTab).iPK = sqlite3_column_int(pStmt, 1 as ::core::ffi::c_int);
            }
            unionFinalize(&raw mut rc, pStmt, pzErr);
        }
        if rc != SQLITE_OK {
            unionDisconnect(pTab as *mut sqlite3_vtab);
            pTab = ::core::ptr::null_mut::<UnionTab>();
        }
        *ppVtab = pTab as *mut sqlite3_vtab;
        return rc;
    }
}
unsafe extern "C" fn unionOpen(
    mut p: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut UnionCsr = ::core::ptr::null_mut::<UnionCsr>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        pCsr = unionMalloc(
            &raw mut rc,
            ::core::mem::size_of::<UnionCsr>() as sqlite3_int64,
        ) as *mut UnionCsr;
        *ppCursor = &raw mut (*pCsr).base;
        return rc;
    }
}
unsafe extern "C" fn unionClose(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut UnionCsr = cur as *mut UnionCsr;
        unionFinalizeCsrStmt(pCsr);
        sqlite3_free(pCsr as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn doUnionNext(mut pCsr: *mut UnionCsr) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if sqlite3_step((*pCsr).pStmt) != SQLITE_ROW {
            let mut pTab: *mut UnionTab = (*pCsr).base.pVtab as *mut UnionTab;
            rc = unionFinalizeCsrStmt(pCsr);
            if rc == SQLITE_OK && (*pTab).bSwarm != 0 {
                (*pCsr).iTab += 1;
                if (*pCsr).iTab < (*pTab).nSrc {
                    let mut pSrc: *mut UnionSrc = (*pTab)
                        .aSrc
                        .offset((*pCsr).iTab as isize) as *mut UnionSrc;
                    if (*pCsr).iMaxRowid >= (*pSrc).iMin {
                        rc = unionOpenDatabase(
                            pTab,
                            (*pCsr).iTab,
                            &raw mut (*pTab).base.zErrMsg,
                        );
                        (*pCsr).pStmt = unionPreparePrintf(
                            &raw mut rc,
                            &raw mut (*pTab).base.zErrMsg,
                            (*pSrc).db,
                            b"SELECT rowid, * FROM %Q %s %lld\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*pSrc).zTab,
                            if (*pSrc).iMax > (*pCsr).iMaxRowid {
                                b"WHERE _rowid_ <=\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"-- \0".as_ptr() as *const ::core::ffi::c_char
                            },
                            (*pCsr).iMaxRowid,
                        );
                        if rc == SQLITE_OK {
                            unionIncrRefcount(pTab, (*pCsr).iTab);
                            rc = SQLITE_ROW;
                        }
                    }
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn unionNext(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        loop {
            rc = doUnionNext(cur as *mut UnionCsr);
            if !(rc == SQLITE_ROW) {
                break;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn unionColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut UnionCsr = cur as *mut UnionCsr;
        sqlite3_result_value(
            ctx,
            sqlite3_column_value((*pCsr).pStmt, i + 1 as ::core::ffi::c_int),
        );
        return SQLITE_OK;
    }
}
unsafe extern "C" fn unionRowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut UnionCsr = cur as *mut UnionCsr;
        *pRowid = sqlite3_column_int64((*pCsr).pStmt, 0 as ::core::ffi::c_int)
            as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn unionEof(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut UnionCsr = cur as *mut UnionCsr;
        return (*pCsr).pStmt.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn unionFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut UnionTab = (*pVtabCursor).pVtab as *mut UnionTab;
        let mut pCsr: *mut UnionCsr = pVtabCursor as *mut UnionCsr;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut bZero: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iMin: sqlite3_int64 = SMALLEST_INT64;
        let mut iMax: sqlite3_int64 = LARGEST_INT64;
        if idxNum == SQLITE_INDEX_CONSTRAINT_EQ {
            iMax = sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize));
            iMin = iMax;
        } else {
            if idxNum & (SQLITE_INDEX_CONSTRAINT_LE | SQLITE_INDEX_CONSTRAINT_LT) != 0 {
                iMax = sqlite3_value_int64(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
                if idxNum & SQLITE_INDEX_CONSTRAINT_LT != 0 {
                    if iMax == SMALLEST_INT64 {
                        bZero = 1 as ::core::ffi::c_int;
                    } else {
                        iMax -= 1;
                    }
                }
            }
            if idxNum & (SQLITE_INDEX_CONSTRAINT_GE | SQLITE_INDEX_CONSTRAINT_GT) != 0 {
                iMin = sqlite3_value_int64(
                    *argv.offset((argc - 1 as ::core::ffi::c_int) as isize),
                );
                if idxNum & SQLITE_INDEX_CONSTRAINT_GT != 0 {
                    if iMin == LARGEST_INT64 {
                        bZero = 1 as ::core::ffi::c_int;
                    } else {
                        iMin += 1;
                    }
                }
            }
        }
        unionFinalizeCsrStmt(pCsr);
        if bZero != 0 {
            return SQLITE_OK;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pTab).nSrc {
            let mut pSrc: *mut UnionSrc = (*pTab).aSrc.offset(i as isize)
                as *mut UnionSrc;
            if !(iMin > (*pSrc).iMax || iMax < (*pSrc).iMin) {
                zSql = sqlite3_mprintf(
                    b"%z%sSELECT rowid, * FROM %s%q%s%Q\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zSql,
                    if !zSql.is_null() {
                        b" UNION ALL \0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    if !(*pSrc).zDb.is_null() {
                        b"'\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    if !(*pSrc).zDb.is_null() {
                        (*pSrc).zDb as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    if !(*pSrc).zDb.is_null() {
                        b"'.\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    (*pSrc).zTab,
                );
                if zSql.is_null() {
                    rc = SQLITE_NOMEM;
                    break;
                } else {
                    if iMin == iMax {
                        zSql = sqlite3_mprintf(
                            b"%z WHERE rowid=%lld\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            zSql,
                            iMin,
                        );
                    } else {
                        let mut zWhere: *const ::core::ffi::c_char = b"WHERE\0".as_ptr()
                            as *const ::core::ffi::c_char;
                        if iMin != SMALLEST_INT64 && iMin > (*pSrc).iMin {
                            zSql = sqlite3_mprintf(
                                b"%z WHERE rowid>=%lld\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                zSql,
                                iMin,
                            );
                            zWhere = b"AND\0".as_ptr() as *const ::core::ffi::c_char;
                        }
                        if iMax != LARGEST_INT64 && iMax < (*pSrc).iMax {
                            zSql = sqlite3_mprintf(
                                b"%z %s rowid<=%lld\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                zSql,
                                zWhere,
                                iMax,
                            );
                        }
                    }
                    if (*pTab).bSwarm != 0 {
                        (*pCsr).iTab = i;
                        (*pCsr).iMaxRowid = iMax;
                        rc = unionOpenDatabase(pTab, i, &raw mut (*pTab).base.zErrMsg);
                        break;
                    }
                }
            }
            i += 1;
        }
        if zSql.is_null() {
            return rc
        } else {
            let mut db: *mut sqlite3 = if (*pTab).bSwarm != 0 {
                (*(*pTab).aSrc.offset((*pCsr).iTab as isize)).db
            } else {
                (*pTab).db
            };
            (*pCsr).pStmt = unionPrepare(
                &raw mut rc,
                db,
                zSql,
                &raw mut (*pTab).base.zErrMsg,
            );
            if !(*pCsr).pStmt.is_null() {
                unionIncrRefcount(pTab, (*pCsr).iTab);
            }
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
        }
        if rc != SQLITE_OK {
            return rc;
        }
        return unionNext(pVtabCursor);
    }
}
unsafe extern "C" fn unionBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut UnionTab = tab as *mut UnionTab;
        let mut iEq: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iLt: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iGt: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdxInfo).nConstraint {
            let mut p: *mut sqlite3_index_constraint = (*pIdxInfo)
                .aConstraint
                .offset(i as isize) as *mut sqlite3_index_constraint;
            if (*p).usable as ::core::ffi::c_int != 0
                && ((*p).iColumn < 0 as ::core::ffi::c_int
                    || (*p).iColumn == (*pTab).iPK)
            {
                match (*p).op as ::core::ffi::c_int {
                    SQLITE_INDEX_CONSTRAINT_EQ => {
                        iEq = i;
                    }
                    SQLITE_INDEX_CONSTRAINT_LE | SQLITE_INDEX_CONSTRAINT_LT => {
                        iLt = i;
                    }
                    SQLITE_INDEX_CONSTRAINT_GE | SQLITE_INDEX_CONSTRAINT_GT => {
                        iGt = i;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        if iEq >= 0 as ::core::ffi::c_int {
            (*pIdxInfo).estimatedRows = 1 as sqlite3_int64;
            (*pIdxInfo).idxFlags = SQLITE_INDEX_SCAN_UNIQUE;
            (*pIdxInfo).estimatedCost = 3.0f64;
            (*pIdxInfo).idxNum = SQLITE_INDEX_CONSTRAINT_EQ;
            (*(*pIdxInfo).aConstraintUsage.offset(iEq as isize)).argvIndex = 1
                as ::core::ffi::c_int;
            (*(*pIdxInfo).aConstraintUsage.offset(iEq as isize)).omit = 1
                as ::core::ffi::c_uchar;
        } else {
            let mut iCons: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut idxNum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nRow: sqlite3_int64 = 1000000 as sqlite3_int64;
            if iLt >= 0 as ::core::ffi::c_int {
                nRow = (nRow as ::core::ffi::c_longlong / 2 as ::core::ffi::c_longlong)
                    as sqlite3_int64;
                let c2rust_fresh4 = iCons;
                iCons = iCons + 1;
                (*(*pIdxInfo).aConstraintUsage.offset(iLt as isize)).argvIndex = c2rust_fresh4;
                (*(*pIdxInfo).aConstraintUsage.offset(iLt as isize)).omit = 1
                    as ::core::ffi::c_uchar;
                idxNum
                    |= (*(*pIdxInfo).aConstraint.offset(iLt as isize)).op
                        as ::core::ffi::c_int;
            }
            if iGt >= 0 as ::core::ffi::c_int {
                nRow = (nRow as ::core::ffi::c_longlong / 2 as ::core::ffi::c_longlong)
                    as sqlite3_int64;
                let c2rust_fresh5 = iCons;
                iCons = iCons + 1;
                (*(*pIdxInfo).aConstraintUsage.offset(iGt as isize)).argvIndex = c2rust_fresh5;
                (*(*pIdxInfo).aConstraintUsage.offset(iGt as isize)).omit = 1
                    as ::core::ffi::c_uchar;
                idxNum
                    |= (*(*pIdxInfo).aConstraint.offset(iGt as isize)).op
                        as ::core::ffi::c_int;
            }
            (*pIdxInfo).estimatedRows = nRow;
            (*pIdxInfo).estimatedCost = 3.0f64 * nRow as ::core::ffi::c_double;
            (*pIdxInfo).idxNum = idxNum;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn createUnionVtab(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        static mut unionModule: sqlite3_module = unsafe {
            sqlite3_module {
                iVersion: 0 as ::core::ffi::c_int,
                xCreate: Some(
                    unionConnect
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
                    unionConnect
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
                    unionBestIndex
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut sqlite3_index_info,
                        ) -> ::core::ffi::c_int,
                ),
                xDisconnect: Some(
                    unionDisconnect
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xDestroy: Some(
                    unionDisconnect
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xOpen: Some(
                    unionOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xClose: Some(
                    unionClose
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xFilter: Some(
                    unionFilter
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> ::core::ffi::c_int,
                ),
                xNext: Some(
                    unionNext
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xEof: Some(
                    unionEof
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xColumn: Some(
                    unionColumn
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xRowid: Some(
                    unionRowid
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
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3_create_module(
            db,
            b"unionvtab\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut unionModule,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        if rc == SQLITE_OK {
            rc = sqlite3_create_module(
                db,
                b"swarmvtab\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut unionModule,
                db as *mut ::core::ffi::c_void,
            );
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_unionvtab_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        rc = createUnionVtab(db);
        return rc;
    }
}
