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
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
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
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_null(_: *mut sqlite3_context);
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
    fn sqlite3_vtab_config(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        ...
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
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
pub struct fuzzer_vtab {
    pub base: sqlite3_vtab,
    pub zClassName: *mut ::core::ffi::c_char,
    pub pRule: *mut fuzzer_rule,
    pub nCursor: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuzzer_rule {
    pub pNext: *mut fuzzer_rule,
    pub zFrom: *mut ::core::ffi::c_char,
    pub rCost: fuzzer_cost,
    pub nFrom: fuzzer_len,
    pub nTo: fuzzer_len,
    pub iRuleset: fuzzer_ruleid,
    pub zTo: [::core::ffi::c_char; 4],
}
pub type fuzzer_ruleid = ::core::ffi::c_int;
pub type fuzzer_len = ::core::ffi::c_schar;
pub type fuzzer_cost = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuzzer_cursor {
    pub base: sqlite3_vtab_cursor,
    pub iRowid: sqlite3_int64,
    pub pVtab: *mut fuzzer_vtab,
    pub rLimit: fuzzer_cost,
    pub pStem: *mut fuzzer_stem,
    pub pDone: *mut fuzzer_stem,
    pub aQueue: [*mut fuzzer_stem; 20],
    pub mxQueue: ::core::ffi::c_int,
    pub zBuf: *mut ::core::ffi::c_char,
    pub nBuf: ::core::ffi::c_int,
    pub nStem: ::core::ffi::c_int,
    pub iRuleset: ::core::ffi::c_int,
    pub nullRule: fuzzer_rule,
    pub apHash: [*mut fuzzer_stem; 4001],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuzzer_stem {
    pub zBasis: *mut ::core::ffi::c_char,
    pub pRule: *const fuzzer_rule,
    pub pNext: *mut fuzzer_stem,
    pub pHash: *mut fuzzer_stem,
    pub rBaseCost: fuzzer_cost,
    pub rCostX: fuzzer_cost,
    pub nBasis: fuzzer_len,
    pub n: fuzzer_len,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_MATCH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SQLITE_VTAB_INNOCUOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FUZZER_MX_LENGTH: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const FUZZER_MX_RULEID: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const FUZZER_MX_COST: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const FUZZER_MX_OUTPUT_LENGTH: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const FUZZER_HASH: ::core::ffi::c_int = 4001 as ::core::ffi::c_int;
pub const FUZZER_NQUEUE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
unsafe extern "C" fn fuzzerMergeRules(
    mut pA: *mut fuzzer_rule,
    mut pB: *mut fuzzer_rule,
) -> *mut fuzzer_rule {
    unsafe {
        let mut head: fuzzer_rule = fuzzer_rule {
            pNext: ::core::ptr::null_mut::<fuzzer_rule>(),
            zFrom: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            rCost: 0,
            nFrom: 0,
            nTo: 0,
            iRuleset: 0,
            zTo: [0; 4],
        };
        let mut pTail: *mut fuzzer_rule = ::core::ptr::null_mut::<fuzzer_rule>();
        pTail = &raw mut head;
        while !pA.is_null() && !pB.is_null() {
            if (*pA).rCost <= (*pB).rCost {
                (*pTail).pNext = pA;
                pTail = pA;
                pA = (*pA).pNext;
            } else {
                (*pTail).pNext = pB;
                pTail = pB;
                pB = (*pB).pNext;
            }
        }
        if pA.is_null() {
            (*pTail).pNext = pB;
        } else {
            (*pTail).pNext = pA;
        }
        return head.pNext;
    }
}
unsafe extern "C" fn fuzzerLoadOneRule(
    mut p: *mut fuzzer_vtab,
    mut pStmt: *mut sqlite3_stmt,
    mut ppRule: *mut *mut fuzzer_rule,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iRuleset: sqlite3_int64 = sqlite3_column_int64(
            pStmt,
            0 as ::core::ffi::c_int,
        );
        let mut zFrom: *const ::core::ffi::c_char = sqlite3_column_text(
            pStmt,
            1 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        let mut zTo: *const ::core::ffi::c_char = sqlite3_column_text(
            pStmt,
            2 as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        let mut nCost: ::core::ffi::c_int = sqlite3_column_int(
            pStmt,
            3 as ::core::ffi::c_int,
        );
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nFrom: ::core::ffi::c_int = 0;
        let mut nTo: ::core::ffi::c_int = 0;
        let mut pRule: *mut fuzzer_rule = ::core::ptr::null_mut::<fuzzer_rule>();
        if zFrom.is_null() {
            zFrom = b"\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if zTo.is_null() {
            zTo = b"\0".as_ptr() as *const ::core::ffi::c_char;
        }
        nFrom = strlen(zFrom) as ::core::ffi::c_int;
        nTo = strlen(zTo) as ::core::ffi::c_int;
        if strcmp(zFrom, zTo) == 0 as ::core::ffi::c_int {
            *ppRule = ::core::ptr::null_mut::<fuzzer_rule>();
            return SQLITE_OK;
        }
        if nCost <= 0 as ::core::ffi::c_int || nCost > FUZZER_MX_COST {
            *pzErr = sqlite3_mprintf(
                b"%s: cost must be between 1 and %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zClassName,
                FUZZER_MX_COST,
            );
            rc = SQLITE_ERROR;
        } else if nFrom > FUZZER_MX_LENGTH || nTo > FUZZER_MX_LENGTH {
            *pzErr = sqlite3_mprintf(
                b"%s: maximum string length is %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zClassName,
                FUZZER_MX_LENGTH,
            );
            rc = SQLITE_ERROR;
        } else if iRuleset < 0 as ::core::ffi::c_longlong
            || iRuleset > FUZZER_MX_RULEID as ::core::ffi::c_longlong
        {
            *pzErr = sqlite3_mprintf(
                b"%s: ruleset must be between 0 and %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zClassName,
                FUZZER_MX_RULEID,
            );
            rc = SQLITE_ERROR;
        } else {
            pRule = sqlite3_malloc64(
                (::core::mem::size_of::<fuzzer_rule>() as usize)
                    .wrapping_add(nFrom as usize)
                    .wrapping_add(nTo as usize) as sqlite3_uint64,
            ) as *mut fuzzer_rule;
            if pRule.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                memset(
                    pRule as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<fuzzer_rule>() as size_t,
                );
                (*pRule).zFrom = &raw mut (*pRule).zTo as *mut ::core::ffi::c_char;
                (*pRule).zFrom = (*pRule)
                    .zFrom
                    .offset((nTo + 1 as ::core::ffi::c_int) as isize);
                (*pRule).nFrom = nFrom as fuzzer_len;
                memcpy(
                    (*pRule).zFrom as *mut ::core::ffi::c_void,
                    zFrom as *const ::core::ffi::c_void,
                    (nFrom + 1 as ::core::ffi::c_int) as size_t,
                );
                memcpy(
                    &raw mut (*pRule).zTo as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    zTo as *const ::core::ffi::c_void,
                    (nTo + 1 as ::core::ffi::c_int) as size_t,
                );
                (*pRule).nTo = nTo as fuzzer_len;
                (*pRule).rCost = nCost as fuzzer_cost;
                (*pRule).iRuleset = iRuleset as ::core::ffi::c_int as fuzzer_ruleid;
            }
        }
        *ppRule = pRule;
        return rc;
    }
}
unsafe extern "C" fn fuzzerLoadRules(
    mut db: *mut sqlite3,
    mut p: *mut fuzzer_vtab,
    mut zDb: *const ::core::ffi::c_char,
    mut zData: *const ::core::ffi::c_char,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pHead: *mut fuzzer_rule = ::core::ptr::null_mut::<fuzzer_rule>();
        zSql = sqlite3_mprintf(
            b"SELECT * FROM %Q.%Q\0".as_ptr() as *const ::core::ffi::c_char,
            zDb,
            zData,
        );
        if zSql.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            let mut rc2: ::core::ffi::c_int = 0;
            let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            rc = sqlite3_prepare_v2(
                db,
                zSql,
                -1 as ::core::ffi::c_int,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc != SQLITE_OK {
                *pzErr = sqlite3_mprintf(
                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).zClassName,
                    sqlite3_errmsg(db),
                );
            } else if sqlite3_column_count(pStmt) != 4 as ::core::ffi::c_int {
                *pzErr = sqlite3_mprintf(
                    b"%s: %s has %d columns, expected 4\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*p).zClassName,
                    zData,
                    sqlite3_column_count(pStmt),
                );
                rc = SQLITE_ERROR;
            } else {
                while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pStmt) {
                    let mut pRule: *mut fuzzer_rule = ::core::ptr::null_mut::<
                        fuzzer_rule,
                    >();
                    rc = fuzzerLoadOneRule(p, pStmt, &raw mut pRule, pzErr);
                    if !pRule.is_null() {
                        (*pRule).pNext = pHead;
                        pHead = pRule;
                    }
                }
            }
            rc2 = sqlite3_finalize(pStmt);
            if rc == SQLITE_OK {
                rc = rc2;
            }
        }
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        if rc == SQLITE_OK {
            let mut i: ::core::ffi::c_uint = 0;
            let mut pX: *mut fuzzer_rule = ::core::ptr::null_mut::<fuzzer_rule>();
            let mut a: [*mut fuzzer_rule; 15] = [::core::ptr::null_mut::<
                fuzzer_rule,
            >(); 15];
            i = 0 as ::core::ffi::c_uint;
            while (i as usize)
                < (::core::mem::size_of::<[*mut fuzzer_rule; 15]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*mut fuzzer_rule>() as usize)
            {
                a[i as usize] = ::core::ptr::null_mut::<fuzzer_rule>();
                i = i.wrapping_add(1);
            }
            loop {
                pX = pHead;
                if pX.is_null() {
                    break;
                }
                pHead = (*pX).pNext;
                (*pX).pNext = ::core::ptr::null_mut::<fuzzer_rule>();
                i = 0 as ::core::ffi::c_uint;
                while !a[i as usize].is_null()
                    && (i as usize)
                        < (::core::mem::size_of::<[*mut fuzzer_rule; 15]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut fuzzer_rule>() as usize,
                            )
                            .wrapping_sub(1 as usize)
                {
                    pX = fuzzerMergeRules(a[i as usize], pX);
                    a[i as usize] = ::core::ptr::null_mut::<fuzzer_rule>();
                    i = i.wrapping_add(1);
                }
                a[i as usize] = fuzzerMergeRules(a[i as usize], pX);
            }
            pX = a[0 as ::core::ffi::c_int as usize];
            i = 1 as ::core::ffi::c_uint;
            while (i as usize)
                < (::core::mem::size_of::<[*mut fuzzer_rule; 15]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*mut fuzzer_rule>() as usize)
            {
                pX = fuzzerMergeRules(a[i as usize], pX);
                i = i.wrapping_add(1);
            }
            (*p).pRule = fuzzerMergeRules((*p).pRule, pX);
        } else {
            (*p).pRule = pHead;
        }
        return rc;
    }
}
unsafe extern "C" fn fuzzerDequote(
    mut zIn: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut nIn: sqlite3_int64 = 0;
        let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        nIn = strlen(zIn) as sqlite3_int64;
        zOut = sqlite3_malloc64(
            (nIn as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if !zOut.is_null() {
            let mut q: ::core::ffi::c_char = *zIn
                .offset(0 as ::core::ffi::c_int as isize);
            if q as ::core::ffi::c_int != '[' as i32
                && q as ::core::ffi::c_int != '\'' as i32
                && q as ::core::ffi::c_int != '"' as i32
                && q as ::core::ffi::c_int != '`' as i32
            {
                memcpy(
                    zOut as *mut ::core::ffi::c_void,
                    zIn as *const ::core::ffi::c_void,
                    (nIn as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                        as size_t,
                );
            } else {
                let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iIn: ::core::ffi::c_int = 0;
                if q as ::core::ffi::c_int == '[' as i32 {
                    q = ']' as i32 as ::core::ffi::c_char;
                }
                iIn = 1 as ::core::ffi::c_int;
                while (iIn as ::core::ffi::c_longlong) < nIn {
                    if *zIn.offset(iIn as isize) as ::core::ffi::c_int
                        == q as ::core::ffi::c_int
                    {
                        iIn += 1;
                    }
                    let c2rust_fresh0 = iOut;
                    iOut = iOut + 1;
                    *zOut.offset(c2rust_fresh0 as isize) = *zIn.offset(iIn as isize);
                    iIn += 1;
                }
            }
        }
        return zOut;
    }
}
unsafe extern "C" fn fuzzerDisconnect(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut fuzzer_vtab = pVtab as *mut fuzzer_vtab;
        while !(*p).pRule.is_null() {
            let mut pRule: *mut fuzzer_rule = (*p).pRule;
            (*p).pRule = (*pRule).pNext;
            sqlite3_free(pRule as *mut ::core::ffi::c_void);
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fuzzerConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pNew: *mut fuzzer_vtab = ::core::ptr::null_mut::<fuzzer_vtab>();
        let mut zModule: *const ::core::ffi::c_char = *argv
            .offset(0 as ::core::ffi::c_int as isize);
        let mut zDb: *const ::core::ffi::c_char = *argv
            .offset(1 as ::core::ffi::c_int as isize);
        if argc != 4 as ::core::ffi::c_int {
            *pzErr = sqlite3_mprintf(
                b"%s: wrong number of CREATE VIRTUAL TABLE arguments\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zModule,
            );
            rc = SQLITE_ERROR;
        } else {
            let mut nModule: sqlite3_int64 = 0;
            nModule = strlen(zModule) as sqlite3_int64;
            pNew = sqlite3_malloc64(
                (::core::mem::size_of::<fuzzer_vtab>() as sqlite3_uint64)
                    .wrapping_add(nModule as sqlite3_uint64)
                    .wrapping_add(1 as sqlite3_uint64),
            ) as *mut fuzzer_vtab;
            if pNew.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                let mut zTab: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                memset(
                    pNew as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<fuzzer_vtab>() as size_t,
                );
                (*pNew).zClassName = pNew.offset(1 as ::core::ffi::c_int as isize)
                    as *mut fuzzer_vtab as *mut ::core::ffi::c_char;
                memcpy(
                    (*pNew).zClassName as *mut ::core::ffi::c_void,
                    zModule as *const ::core::ffi::c_void,
                    (nModule as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                        as size_t,
                );
                zTab = fuzzerDequote(*argv.offset(3 as ::core::ffi::c_int as isize));
                if zTab.is_null() {
                    rc = SQLITE_NOMEM;
                } else {
                    rc = fuzzerLoadRules(db, pNew, zDb, zTab, pzErr);
                    sqlite3_free(zTab as *mut ::core::ffi::c_void);
                }
                if rc == SQLITE_OK {
                    rc = sqlite3_declare_vtab(
                        db,
                        b"CREATE TABLE x(word,distance,ruleset)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                if rc != SQLITE_OK {
                    fuzzerDisconnect(pNew as *mut sqlite3_vtab);
                    pNew = ::core::ptr::null_mut::<fuzzer_vtab>();
                } else {
                    sqlite3_vtab_config(db, SQLITE_VTAB_INNOCUOUS);
                }
            }
        }
        *ppVtab = pNew as *mut sqlite3_vtab;
        return rc;
    }
}
unsafe extern "C" fn fuzzerOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut fuzzer_vtab = pVTab as *mut fuzzer_vtab;
        let mut pCur: *mut fuzzer_cursor = ::core::ptr::null_mut::<fuzzer_cursor>();
        pCur = sqlite3_malloc(
            ::core::mem::size_of::<fuzzer_cursor>() as ::core::ffi::c_int,
        ) as *mut fuzzer_cursor;
        if pCur.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pCur as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<fuzzer_cursor>() as size_t,
        );
        (*pCur).pVtab = p;
        *ppCursor = &raw mut (*pCur).base;
        (*p).nCursor += 1;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fuzzerClearStemList(mut pStem: *mut fuzzer_stem) {
    unsafe {
        while !pStem.is_null() {
            let mut pNext: *mut fuzzer_stem = (*pStem).pNext;
            sqlite3_free(pStem as *mut ::core::ffi::c_void);
            pStem = pNext;
        }
    }
}
unsafe extern "C" fn fuzzerClearCursor(
    mut pCur: *mut fuzzer_cursor,
    mut clearHash: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        fuzzerClearStemList((*pCur).pStem);
        fuzzerClearStemList((*pCur).pDone);
        i = 0 as ::core::ffi::c_int;
        while i < FUZZER_NQUEUE {
            fuzzerClearStemList((*pCur).aQueue[i as usize]);
            i += 1;
        }
        (*pCur).rLimit = 0 as ::core::ffi::c_int;
        if clearHash != 0 && (*pCur).nStem != 0 {
            (*pCur).mxQueue = 0 as ::core::ffi::c_int;
            (*pCur).pStem = ::core::ptr::null_mut::<fuzzer_stem>();
            (*pCur).pDone = ::core::ptr::null_mut::<fuzzer_stem>();
            memset(
                &raw mut (*pCur).aQueue as *mut *mut fuzzer_stem
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<[*mut fuzzer_stem; 20]>() as size_t,
            );
            memset(
                &raw mut (*pCur).apHash as *mut *mut fuzzer_stem
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<[*mut fuzzer_stem; 4001]>() as size_t,
            );
        }
        (*pCur).nStem = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fuzzerClose(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut fuzzer_cursor = cur as *mut fuzzer_cursor;
        fuzzerClearCursor(pCur, 0 as ::core::ffi::c_int);
        sqlite3_free((*pCur).zBuf as *mut ::core::ffi::c_void);
        (*(*pCur).pVtab).nCursor -= 1;
        sqlite3_free(pCur as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fuzzerRender(
    mut pStem: *mut fuzzer_stem,
    mut pzBuf: *mut *mut ::core::ffi::c_char,
    mut pnBuf: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pRule: *const fuzzer_rule = (*pStem).pRule;
        let mut n: ::core::ffi::c_int = 0;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        n = (*pStem).nBasis as ::core::ffi::c_int + (*pRule).nTo as ::core::ffi::c_int
            - (*pRule).nFrom as ::core::ffi::c_int;
        if *pnBuf < n + 1 as ::core::ffi::c_int {
            *pzBuf = sqlite3_realloc(
                *pzBuf as *mut ::core::ffi::c_void,
                n + 100 as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char;
            if (*pzBuf).is_null() {
                return SQLITE_NOMEM;
            }
            *pnBuf = n + 100 as ::core::ffi::c_int;
        }
        n = (*pStem).n as ::core::ffi::c_int;
        z = *pzBuf;
        if n < 0 as ::core::ffi::c_int {
            memcpy(
                z as *mut ::core::ffi::c_void,
                (*pStem).zBasis as *const ::core::ffi::c_void,
                ((*pStem).nBasis as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as size_t,
            );
        } else {
            memcpy(
                z as *mut ::core::ffi::c_void,
                (*pStem).zBasis as *const ::core::ffi::c_void,
                n as size_t,
            );
            memcpy(
                z.offset(n as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                &raw const (*pRule).zTo as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                (*pRule).nTo as size_t,
            );
            memcpy(
                z.offset((n + (*pRule).nTo as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (*pStem)
                    .zBasis
                    .offset((n + (*pRule).nFrom as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                ((*pStem).nBasis as ::core::ffi::c_int - n
                    - (*pRule).nFrom as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as size_t,
            );
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fuzzerHash(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while *z != 0 {
            let c2rust_fresh1 = z;
            z = z.offset(1);
            h = h << 3 as ::core::ffi::c_int ^ h >> 29 as ::core::ffi::c_int
                ^ *c2rust_fresh1 as ::core::ffi::c_uint;
        }
        return h.wrapping_rem(FUZZER_HASH as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn fuzzerCost(mut pStem: *mut fuzzer_stem) -> fuzzer_cost {
    unsafe {
        (*pStem).rCostX = (*pStem).rBaseCost + (*(*pStem).pRule).rCost;
        return (*pStem).rCostX;
    }
}
unsafe extern "C" fn fuzzerSeen(
    mut pCur: *mut fuzzer_cursor,
    mut pStem: *mut fuzzer_stem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut h: ::core::ffi::c_uint = 0;
        let mut pLookup: *mut fuzzer_stem = ::core::ptr::null_mut::<fuzzer_stem>();
        if fuzzerRender(pStem, &raw mut (*pCur).zBuf, &raw mut (*pCur).nBuf)
            == SQLITE_NOMEM
        {
            return -1 as ::core::ffi::c_int;
        }
        h = fuzzerHash((*pCur).zBuf);
        pLookup = (*pCur).apHash[h as usize];
        while !pLookup.is_null()
            && strcmp((*pLookup).zBasis, (*pCur).zBuf) != 0 as ::core::ffi::c_int
        {
            pLookup = (*pLookup).pHash;
        }
        return !pLookup.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fuzzerSkipRule(
    mut pRule: *const fuzzer_rule,
    mut pStem: *mut fuzzer_stem,
    mut iRuleset: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (!pRule.is_null()
            && ((*pRule).iRuleset != iRuleset
                || (*pStem).nBasis as ::core::ffi::c_int
                    + (*pRule).nTo as ::core::ffi::c_int
                    - (*pRule).nFrom as ::core::ffi::c_int > FUZZER_MX_OUTPUT_LENGTH))
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fuzzerAdvance(
    mut pCur: *mut fuzzer_cursor,
    mut pStem: *mut fuzzer_stem,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pRule: *const fuzzer_rule = ::core::ptr::null::<fuzzer_rule>();
        loop {
            pRule = (*pStem).pRule;
            if pRule.is_null() {
                break;
            }
            while ((*pStem).n as ::core::ffi::c_int)
                < (*pStem).nBasis as ::core::ffi::c_int
                    - (*pRule).nFrom as ::core::ffi::c_int
            {
                (*pStem).n += 1;
                if (*pRule).nFrom as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || memcmp(
                        (*pStem).zBasis.offset((*pStem).n as isize)
                            as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                        (*pRule).zFrom as *const ::core::ffi::c_void,
                        (*pRule).nFrom as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    let mut rc: ::core::ffi::c_int = fuzzerSeen(pCur, pStem);
                    if rc < 0 as ::core::ffi::c_int {
                        return -1 as ::core::ffi::c_int;
                    }
                    if rc == 0 as ::core::ffi::c_int {
                        fuzzerCost(pStem);
                        return 1 as ::core::ffi::c_int;
                    }
                }
            }
            (*pStem).n = -1 as ::core::ffi::c_int as fuzzer_len;
            loop {
                pRule = (*pRule).pNext;
                if !(fuzzerSkipRule(pRule, pStem, (*pCur).iRuleset) != 0) {
                    break;
                }
            }
            (*pStem).pRule = pRule;
            if !pRule.is_null() && fuzzerCost(pStem) > (*pCur).rLimit {
                (*pStem).pRule = ::core::ptr::null::<fuzzer_rule>();
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fuzzerMergeStems(
    mut pA: *mut fuzzer_stem,
    mut pB: *mut fuzzer_stem,
) -> *mut fuzzer_stem {
    unsafe {
        let mut head: fuzzer_stem = fuzzer_stem {
            zBasis: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pRule: ::core::ptr::null::<fuzzer_rule>(),
            pNext: ::core::ptr::null_mut::<fuzzer_stem>(),
            pHash: ::core::ptr::null_mut::<fuzzer_stem>(),
            rBaseCost: 0,
            rCostX: 0,
            nBasis: 0,
            n: 0,
        };
        let mut pTail: *mut fuzzer_stem = ::core::ptr::null_mut::<fuzzer_stem>();
        pTail = &raw mut head;
        while !pA.is_null() && !pB.is_null() {
            if (*pA).rCostX <= (*pB).rCostX {
                (*pTail).pNext = pA;
                pTail = pA;
                pA = (*pA).pNext;
            } else {
                (*pTail).pNext = pB;
                pTail = pB;
                pB = (*pB).pNext;
            }
        }
        if pA.is_null() {
            (*pTail).pNext = pB;
        } else {
            (*pTail).pNext = pA;
        }
        return head.pNext;
    }
}
unsafe extern "C" fn fuzzerLowestCostStem(
    mut pCur: *mut fuzzer_cursor,
) -> *mut fuzzer_stem {
    unsafe {
        let mut pBest: *mut fuzzer_stem = ::core::ptr::null_mut::<fuzzer_stem>();
        let mut pX: *mut fuzzer_stem = ::core::ptr::null_mut::<fuzzer_stem>();
        let mut iBest: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        if (*pCur).pStem.is_null() {
            iBest = -1 as ::core::ffi::c_int;
            pBest = ::core::ptr::null_mut::<fuzzer_stem>();
            i = 0 as ::core::ffi::c_int;
            while i <= (*pCur).mxQueue {
                pX = (*pCur).aQueue[i as usize];
                if !pX.is_null() {
                    if pBest.is_null() || (*pBest).rCostX > (*pX).rCostX {
                        pBest = pX;
                        iBest = i;
                    }
                }
                i += 1;
            }
            if !pBest.is_null() {
                (*pCur).aQueue[iBest as usize] = (*pBest).pNext;
                (*pBest).pNext = ::core::ptr::null_mut::<fuzzer_stem>();
                (*pCur).pStem = pBest;
            }
        }
        return (*pCur).pStem;
    }
}
unsafe extern "C" fn fuzzerInsert(
    mut pCur: *mut fuzzer_cursor,
    mut pNew: *mut fuzzer_stem,
) -> *mut fuzzer_stem {
    unsafe {
        let mut pX: *mut fuzzer_stem = ::core::ptr::null_mut::<fuzzer_stem>();
        let mut i: ::core::ffi::c_int = 0;
        pX = (*pCur).pStem;
        if !pX.is_null() && (*pX).rCostX > (*pNew).rCostX {
            (*pNew).pNext = ::core::ptr::null_mut::<fuzzer_stem>();
            (*pCur).pStem = pNew;
            pNew = pX;
        }
        (*pNew).pNext = ::core::ptr::null_mut::<fuzzer_stem>();
        pX = pNew;
        i = 0 as ::core::ffi::c_int;
        while i <= (*pCur).mxQueue {
            if !(*pCur).aQueue[i as usize].is_null() {
                pX = fuzzerMergeStems(pX, (*pCur).aQueue[i as usize]);
                (*pCur).aQueue[i as usize] = ::core::ptr::null_mut::<fuzzer_stem>();
                i += 1;
            } else {
                (*pCur).aQueue[i as usize] = pX;
                break;
            }
        }
        if i > (*pCur).mxQueue {
            if i < FUZZER_NQUEUE {
                (*pCur).mxQueue = i;
                (*pCur).aQueue[i as usize] = pX;
            } else {
                pX = fuzzerMergeStems(
                    pX,
                    (*pCur).aQueue[(FUZZER_NQUEUE - 1 as ::core::ffi::c_int) as usize],
                );
                (*pCur).aQueue[(FUZZER_NQUEUE - 1 as ::core::ffi::c_int) as usize] = pX;
            }
        }
        return fuzzerLowestCostStem(pCur);
    }
}
unsafe extern "C" fn fuzzerNewStem(
    mut pCur: *mut fuzzer_cursor,
    mut zWord: *const ::core::ffi::c_char,
    mut rBaseCost: fuzzer_cost,
) -> *mut fuzzer_stem {
    unsafe {
        let mut pNew: *mut fuzzer_stem = ::core::ptr::null_mut::<fuzzer_stem>();
        let mut pRule: *mut fuzzer_rule = ::core::ptr::null_mut::<fuzzer_rule>();
        let mut h: ::core::ffi::c_uint = 0;
        pNew = sqlite3_malloc64(
            (::core::mem::size_of::<fuzzer_stem>() as usize)
                .wrapping_add(strlen(zWord) as usize)
                .wrapping_add(1 as usize) as sqlite3_uint64,
        ) as *mut fuzzer_stem;
        if pNew.is_null() {
            return ::core::ptr::null_mut::<fuzzer_stem>();
        }
        memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<fuzzer_stem>() as size_t,
        );
        (*pNew).zBasis = pNew.offset(1 as ::core::ffi::c_int as isize)
            as *mut fuzzer_stem as *mut ::core::ffi::c_char;
        (*pNew).nBasis = strlen(zWord) as fuzzer_len;
        memcpy(
            (*pNew).zBasis as *mut ::core::ffi::c_void,
            zWord as *const ::core::ffi::c_void,
            ((*pNew).nBasis as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        );
        pRule = (*(*pCur).pVtab).pRule;
        while fuzzerSkipRule(pRule, pNew, (*pCur).iRuleset) != 0 {
            pRule = (*pRule).pNext;
        }
        (*pNew).pRule = pRule;
        (*pNew).n = -1 as ::core::ffi::c_int as fuzzer_len;
        (*pNew).rCostX = rBaseCost;
        (*pNew).rBaseCost = (*pNew).rCostX;
        h = fuzzerHash((*pNew).zBasis);
        (*pNew).pHash = (*pCur).apHash[h as usize];
        (*pCur).apHash[h as usize] = pNew;
        (*pCur).nStem += 1;
        return pNew;
    }
}
unsafe extern "C" fn fuzzerNext(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut fuzzer_cursor = cur as *mut fuzzer_cursor;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pStem: *mut fuzzer_stem = ::core::ptr::null_mut::<fuzzer_stem>();
        let mut pNew: *mut fuzzer_stem = ::core::ptr::null_mut::<fuzzer_stem>();
        (*pCur).iRowid += 1;
        pStem = (*pCur).pStem;
        if (*pStem).rCostX > 0 as ::core::ffi::c_int {
            rc = fuzzerRender(pStem, &raw mut (*pCur).zBuf, &raw mut (*pCur).nBuf);
            if rc == SQLITE_NOMEM {
                return SQLITE_NOMEM;
            }
            pNew = fuzzerNewStem(pCur, (*pCur).zBuf, (*pStem).rCostX);
            if !pNew.is_null() {
                if fuzzerAdvance(pCur, pNew) == 0 as ::core::ffi::c_int {
                    (*pNew).pNext = (*pCur).pDone;
                    (*pCur).pDone = pNew;
                } else if fuzzerInsert(pCur, pNew) == pNew {
                    return SQLITE_OK
                }
            } else {
                return SQLITE_NOMEM
            }
        }
        loop {
            pStem = (*pCur).pStem;
            if pStem.is_null() {
                break;
            }
            let mut res: ::core::ffi::c_int = fuzzerAdvance(pCur, pStem);
            if res < 0 as ::core::ffi::c_int {
                return SQLITE_NOMEM
            } else if res > 0 as ::core::ffi::c_int {
                (*pCur).pStem = ::core::ptr::null_mut::<fuzzer_stem>();
                pStem = fuzzerInsert(pCur, pStem);
                rc = fuzzerSeen(pCur, pStem);
                if rc != 0 as ::core::ffi::c_int {
                    if rc < 0 as ::core::ffi::c_int {
                        return SQLITE_NOMEM;
                    }
                } else {
                    return SQLITE_OK
                }
            } else {
                (*pCur).pStem = ::core::ptr::null_mut::<fuzzer_stem>();
                (*pStem).pNext = (*pCur).pDone;
                (*pCur).pDone = pStem;
                if !fuzzerLowestCostStem(pCur).is_null() {
                    rc = fuzzerSeen(pCur, (*pCur).pStem);
                    if rc < 0 as ::core::ffi::c_int {
                        return SQLITE_NOMEM;
                    }
                    if rc == 0 as ::core::ffi::c_int {
                        return SQLITE_OK;
                    }
                }
            }
        }
        (*pCur).rLimit = 0 as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fuzzerFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut fuzzer_cursor = pVtabCursor as *mut fuzzer_cursor;
        let mut zWord: *const ::core::ffi::c_char = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut pStem: *mut fuzzer_stem = ::core::ptr::null_mut::<fuzzer_stem>();
        let mut idx: ::core::ffi::c_int = 0;
        fuzzerClearCursor(pCur, 1 as ::core::ffi::c_int);
        (*pCur).rLimit = 2147483647 as ::core::ffi::c_int as fuzzer_cost;
        idx = 0 as ::core::ffi::c_int;
        if idxNum & 1 as ::core::ffi::c_int != 0 {
            zWord = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
            idx += 1;
        }
        if idxNum & 2 as ::core::ffi::c_int != 0 {
            (*pCur).rLimit = sqlite3_value_int(*argv.offset(idx as isize));
            idx += 1;
        }
        if idxNum & 4 as ::core::ffi::c_int != 0 {
            (*pCur).iRuleset = sqlite3_value_int(*argv.offset(idx as isize))
                as ::core::ffi::c_int;
            idx += 1;
        }
        (*pCur).nullRule.pNext = (*(*pCur).pVtab).pRule;
        (*pCur).nullRule.rCost = 0 as ::core::ffi::c_int as fuzzer_cost;
        (*pCur).nullRule.nFrom = 0 as fuzzer_len;
        (*pCur).nullRule.nTo = 0 as fuzzer_len;
        (*pCur).nullRule.zFrom = b"\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
        (*pCur).iRowid = 1 as sqlite3_int64;
        if (strlen(zWord) as ::core::ffi::c_int) < FUZZER_MX_OUTPUT_LENGTH {
            pStem = fuzzerNewStem(pCur, zWord, 0 as ::core::ffi::c_int);
            (*pCur).pStem = pStem;
            if pStem.is_null() {
                return SQLITE_NOMEM;
            }
            (*pStem).pRule = &raw mut (*pCur).nullRule;
            (*pStem).n = (*pStem).nBasis;
        } else {
            (*pCur).rLimit = 0 as ::core::ffi::c_int as fuzzer_cost;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fuzzerColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut fuzzer_cursor = cur as *mut fuzzer_cursor;
        if i == 0 as ::core::ffi::c_int {
            if fuzzerRender((*pCur).pStem, &raw mut (*pCur).zBuf, &raw mut (*pCur).nBuf)
                == SQLITE_NOMEM
            {
                return SQLITE_NOMEM;
            }
            sqlite3_result_text(
                ctx,
                (*pCur).zBuf,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else if i == 1 as ::core::ffi::c_int {
            sqlite3_result_int(ctx, (*(*pCur).pStem).rCostX as ::core::ffi::c_int);
        } else {
            sqlite3_result_null(ctx);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fuzzerRowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut fuzzer_cursor = cur as *mut fuzzer_cursor;
        *pRowid = (*pCur).iRowid as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fuzzerEof(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut fuzzer_cursor = cur as *mut fuzzer_cursor;
        return ((*pCur).rLimit <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fuzzerBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iPlan: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iDistTerm: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iRulesetTerm: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut seenMatch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pConstraint: *const sqlite3_index_constraint = ::core::ptr::null::<
            sqlite3_index_constraint,
        >();
        let mut rCost: ::core::ffi::c_double = 1e12f64;
        pConstraint = (*pIdxInfo).aConstraint;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdxInfo).nConstraint {
            if (*pConstraint).iColumn == 0 as ::core::ffi::c_int
                && (*pConstraint).op as ::core::ffi::c_int
                    == SQLITE_INDEX_CONSTRAINT_MATCH
            {
                seenMatch = 1 as ::core::ffi::c_int;
            }
            if !((*pConstraint).usable as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            {
                if iPlan & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*pConstraint).iColumn == 0 as ::core::ffi::c_int
                    && (*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_MATCH
                {
                    iPlan |= 1 as ::core::ffi::c_int;
                    (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).argvIndex = 1
                        as ::core::ffi::c_int;
                    (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).omit = 1
                        as ::core::ffi::c_uchar;
                    rCost /= 1e6f64;
                }
                if iPlan & 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*pConstraint).iColumn == 1 as ::core::ffi::c_int
                    && ((*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_LT
                        || (*pConstraint).op as ::core::ffi::c_int
                            == SQLITE_INDEX_CONSTRAINT_LE)
                {
                    iPlan |= 2 as ::core::ffi::c_int;
                    iDistTerm = i;
                    rCost /= 10.0f64;
                }
                if iPlan & 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*pConstraint).iColumn == 2 as ::core::ffi::c_int
                    && (*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_EQ
                {
                    iPlan |= 4 as ::core::ffi::c_int;
                    (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).omit = 1
                        as ::core::ffi::c_uchar;
                    iRulesetTerm = i;
                    rCost /= 10.0f64;
                }
            }
            i += 1;
            pConstraint = pConstraint.offset(1);
        }
        if iPlan & 2 as ::core::ffi::c_int != 0 {
            (*(*pIdxInfo).aConstraintUsage.offset(iDistTerm as isize)).argvIndex = 1
                as ::core::ffi::c_int
                + (iPlan & 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
        }
        if iPlan & 4 as ::core::ffi::c_int != 0 {
            let mut idx: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            if iPlan & 1 as ::core::ffi::c_int != 0 {
                idx += 1;
            }
            if iPlan & 2 as ::core::ffi::c_int != 0 {
                idx += 1;
            }
            (*(*pIdxInfo).aConstraintUsage.offset(iRulesetTerm as isize)).argvIndex = idx;
        }
        (*pIdxInfo).idxNum = iPlan;
        if (*pIdxInfo).nOrderBy == 1 as ::core::ffi::c_int
            && (*(*pIdxInfo).aOrderBy.offset(0 as ::core::ffi::c_int as isize)).iColumn
                == 1 as ::core::ffi::c_int
            && (*(*pIdxInfo).aOrderBy.offset(0 as ::core::ffi::c_int as isize)).desc
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*pIdxInfo).orderByConsumed = 1 as ::core::ffi::c_int;
        }
        if seenMatch != 0 && iPlan & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            rCost = 1e99f64;
        }
        (*pIdxInfo).estimatedCost = rCost;
        return SQLITE_OK;
    }
}
static mut fuzzerModule: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: Some(
            fuzzerConnect
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
            fuzzerConnect
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
            fuzzerBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            fuzzerDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            fuzzerDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            fuzzerOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            fuzzerClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            fuzzerFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            fuzzerNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            fuzzerEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            fuzzerColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            fuzzerRowid
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_fuzzer_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        rc = sqlite3_create_module(
            db,
            b"fuzzer\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut fuzzerModule,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return rc;
    }
}
