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
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
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
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct amatch_vtab {
    pub base: sqlite3_vtab,
    pub zClassName: *mut ::core::ffi::c_char,
    pub zDb: *mut ::core::ffi::c_char,
    pub zSelf: *mut ::core::ffi::c_char,
    pub zCostTab: *mut ::core::ffi::c_char,
    pub zVocabTab: *mut ::core::ffi::c_char,
    pub zVocabWord: *mut ::core::ffi::c_char,
    pub zVocabLang: *mut ::core::ffi::c_char,
    pub pRule: *mut amatch_rule,
    pub rIns: amatch_cost,
    pub rDel: amatch_cost,
    pub rSub: amatch_cost,
    pub db: *mut sqlite3,
    pub pVCheck: *mut sqlite3_stmt,
    pub nCursor: ::core::ffi::c_int,
}
pub type amatch_cost = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct amatch_rule {
    pub pNext: *mut amatch_rule,
    pub zFrom: *mut ::core::ffi::c_char,
    pub rCost: amatch_cost,
    pub iLang: amatch_langid,
    pub nFrom: amatch_len,
    pub nTo: amatch_len,
    pub zTo: [::core::ffi::c_char; 4],
}
pub type amatch_len = ::core::ffi::c_schar;
pub type amatch_langid = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct amatch_cursor {
    pub base: sqlite3_vtab_cursor,
    pub iRowid: sqlite3_int64,
    pub iLang: amatch_langid,
    pub rLimit: amatch_cost,
    pub nBuf: sqlite3_int64,
    pub oomErr: ::core::ffi::c_int,
    pub nWord: ::core::ffi::c_int,
    pub zBuf: *mut ::core::ffi::c_char,
    pub zInput: *mut ::core::ffi::c_char,
    pub pVtab: *mut amatch_vtab,
    pub pAllWords: *mut amatch_word,
    pub pCurrent: *mut amatch_word,
    pub pCost: *mut amatch_avl,
    pub pWord: *mut amatch_avl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct amatch_avl {
    pub pWord: *mut amatch_word,
    pub zKey: *mut ::core::ffi::c_char,
    pub pBefore: *mut amatch_avl,
    pub pAfter: *mut amatch_avl,
    pub pUp: *mut amatch_avl,
    pub height: ::core::ffi::c_short,
    pub imbalance: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct amatch_word {
    pub pNext: *mut amatch_word,
    pub sCost: amatch_avl,
    pub sWord: amatch_avl,
    pub rCost: amatch_cost,
    pub iSeq: ::core::ffi::c_int,
    pub zCost: [::core::ffi::c_char; 10],
    pub nMatch: ::core::ffi::c_short,
    pub zWord: [::core::ffi::c_char; 4],
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_MATCH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SQLITE_VTAB_INNOCUOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn amatchAvlRecomputeHeight(mut p: *mut amatch_avl) {
    unsafe {
        let mut hBefore: ::core::ffi::c_short = (if !(*p).pBefore.is_null() {
            (*(*p).pBefore).height as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_short;
        let mut hAfter: ::core::ffi::c_short = (if !(*p).pAfter.is_null() {
            (*(*p).pAfter).height as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_short;
        (*p).imbalance = (hBefore as ::core::ffi::c_int - hAfter as ::core::ffi::c_int)
            as ::core::ffi::c_short;
        (*p).height = ((if hBefore as ::core::ffi::c_int > hAfter as ::core::ffi::c_int {
            hBefore as ::core::ffi::c_int
        } else {
            hAfter as ::core::ffi::c_int
        }) + 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
    }
}
unsafe extern "C" fn amatchAvlRotateBefore(mut pP: *mut amatch_avl) -> *mut amatch_avl {
    unsafe {
        let mut pB: *mut amatch_avl = (*pP).pBefore;
        let mut pY: *mut amatch_avl = (*pB).pAfter;
        (*pB).pUp = (*pP).pUp;
        (*pB).pAfter = pP;
        (*pP).pUp = pB;
        (*pP).pBefore = pY;
        if !pY.is_null() {
            (*pY).pUp = pP;
        }
        amatchAvlRecomputeHeight(pP);
        amatchAvlRecomputeHeight(pB);
        return pB;
    }
}
unsafe extern "C" fn amatchAvlRotateAfter(mut pP: *mut amatch_avl) -> *mut amatch_avl {
    unsafe {
        let mut pA: *mut amatch_avl = (*pP).pAfter;
        let mut pY: *mut amatch_avl = (*pA).pBefore;
        (*pA).pUp = (*pP).pUp;
        (*pA).pBefore = pP;
        (*pP).pUp = pA;
        (*pP).pAfter = pY;
        if !pY.is_null() {
            (*pY).pUp = pP;
        }
        amatchAvlRecomputeHeight(pP);
        amatchAvlRecomputeHeight(pA);
        return pA;
    }
}
unsafe extern "C" fn amatchAvlFromPtr(
    mut p: *mut amatch_avl,
    mut pp: *mut *mut amatch_avl,
) -> *mut *mut amatch_avl {
    unsafe {
        let mut pUp: *mut amatch_avl = (*p).pUp;
        if pUp.is_null() {
            return pp;
        }
        if (*pUp).pAfter == p {
            return &raw mut (*pUp).pAfter;
        }
        return &raw mut (*pUp).pBefore;
    }
}
unsafe extern "C" fn amatchAvlBalance(mut p: *mut amatch_avl) -> *mut amatch_avl {
    unsafe {
        let mut pTop: *mut amatch_avl = p;
        let mut pp: *mut *mut amatch_avl = ::core::ptr::null_mut::<*mut amatch_avl>();
        while !p.is_null() {
            amatchAvlRecomputeHeight(p);
            if (*p).imbalance as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
                let mut pB: *mut amatch_avl = (*p).pBefore;
                if ((*pB).imbalance as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    (*p).pBefore = amatchAvlRotateAfter(pB);
                }
                pp = amatchAvlFromPtr(p, &raw mut p);
                *pp = amatchAvlRotateBefore(p);
                p = *pp;
            } else if (*p).imbalance as ::core::ffi::c_int <= -2 as ::core::ffi::c_int {
                let mut pA: *mut amatch_avl = (*p).pAfter;
                if (*pA).imbalance as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    (*p).pAfter = amatchAvlRotateBefore(pA);
                }
                pp = amatchAvlFromPtr(p, &raw mut p);
                *pp = amatchAvlRotateAfter(p);
                p = *pp;
            }
            pTop = p;
            p = (*p).pUp;
        }
        return pTop;
    }
}
unsafe extern "C" fn amatchAvlSearch(
    mut p: *mut amatch_avl,
    mut zKey: *const ::core::ffi::c_char,
) -> *mut amatch_avl {
    unsafe {
        let mut c: ::core::ffi::c_int = 0;
        while !p.is_null()
            && {
                c = strcmp(zKey, (*p).zKey);
                c != 0 as ::core::ffi::c_int
            }
        {
            p = if c < 0 as ::core::ffi::c_int { (*p).pBefore } else { (*p).pAfter };
        }
        return p;
    }
}
unsafe extern "C" fn amatchAvlFirst(mut p: *mut amatch_avl) -> *mut amatch_avl {
    unsafe {
        if !p.is_null() {
            while !(*p).pBefore.is_null() {
                p = (*p).pBefore;
            }
        }
        return p;
    }
}
unsafe extern "C" fn amatchAvlInsert(
    mut ppHead: *mut *mut amatch_avl,
    mut pNew: *mut amatch_avl,
) -> *mut amatch_avl {
    unsafe {
        let mut c: ::core::ffi::c_int = 0;
        let mut p: *mut amatch_avl = *ppHead;
        if p.is_null() {
            p = pNew;
            (*pNew).pUp = ::core::ptr::null_mut::<amatch_avl>();
        } else {
            while !p.is_null() {
                c = strcmp((*pNew).zKey, (*p).zKey);
                if c < 0 as ::core::ffi::c_int {
                    if !(*p).pBefore.is_null() {
                        p = (*p).pBefore;
                    } else {
                        (*p).pBefore = pNew;
                        (*pNew).pUp = p;
                        break;
                    }
                } else if c > 0 as ::core::ffi::c_int {
                    if !(*p).pAfter.is_null() {
                        p = (*p).pAfter;
                    } else {
                        (*p).pAfter = pNew;
                        (*pNew).pUp = p;
                        break;
                    }
                } else {
                    return p
                }
            }
        }
        (*pNew).pBefore = ::core::ptr::null_mut::<amatch_avl>();
        (*pNew).pAfter = ::core::ptr::null_mut::<amatch_avl>();
        (*pNew).height = 1 as ::core::ffi::c_short;
        (*pNew).imbalance = 0 as ::core::ffi::c_short;
        *ppHead = amatchAvlBalance(p);
        return ::core::ptr::null_mut::<amatch_avl>();
    }
}
unsafe extern "C" fn amatchAvlRemove(
    mut ppHead: *mut *mut amatch_avl,
    mut pOld: *mut amatch_avl,
) {
    unsafe {
        let mut ppParent: *mut *mut amatch_avl = ::core::ptr::null_mut::<
            *mut amatch_avl,
        >();
        let mut pBalance: *mut amatch_avl = ::core::ptr::null_mut::<amatch_avl>();
        ppParent = amatchAvlFromPtr(pOld, ppHead);
        if (*pOld).pBefore.is_null() && (*pOld).pAfter.is_null() {
            *ppParent = ::core::ptr::null_mut::<amatch_avl>();
            pBalance = (*pOld).pUp;
        } else if !(*pOld).pBefore.is_null() && !(*pOld).pAfter.is_null() {
            let mut pX: *mut amatch_avl = ::core::ptr::null_mut::<amatch_avl>();
            let mut pY: *mut amatch_avl = ::core::ptr::null_mut::<amatch_avl>();
            pX = amatchAvlFirst((*pOld).pAfter);
            let ref mut c2rust_fresh0 = *amatchAvlFromPtr(
                pX,
                ::core::ptr::null_mut::<*mut amatch_avl>(),
            );
            *c2rust_fresh0 = (*pX).pAfter;
            if !(*pX).pAfter.is_null() {
                (*(*pX).pAfter).pUp = (*pX).pUp;
            }
            pBalance = (*pX).pUp;
            (*pX).pAfter = (*pOld).pAfter;
            if !(*pX).pAfter.is_null() {
                (*(*pX).pAfter).pUp = pX;
            } else {
                pBalance = pX;
            }
            pY = (*pOld).pBefore;
            (*pX).pBefore = pY;
            if !pY.is_null() {
                (*pY).pUp = pX;
            }
            (*pX).pUp = (*pOld).pUp;
            *ppParent = pX;
        } else if (*pOld).pBefore.is_null() {
            pBalance = (*pOld).pAfter;
            *ppParent = pBalance;
            (*pBalance).pUp = (*pOld).pUp;
        } else if (*pOld).pAfter.is_null() {
            pBalance = (*pOld).pBefore;
            *ppParent = pBalance;
            (*pBalance).pUp = (*pOld).pUp;
        }
        *ppHead = amatchAvlBalance(pBalance);
        (*pOld).pUp = ::core::ptr::null_mut::<amatch_avl>();
        (*pOld).pBefore = ::core::ptr::null_mut::<amatch_avl>();
        (*pOld).pAfter = ::core::ptr::null_mut::<amatch_avl>();
    }
}
pub const AMATCH_MX_LENGTH: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const AMATCH_MX_LANGID: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const AMATCH_MX_COST: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
unsafe extern "C" fn amatchMergeRules(
    mut pA: *mut amatch_rule,
    mut pB: *mut amatch_rule,
) -> *mut amatch_rule {
    unsafe {
        let mut head: amatch_rule = amatch_rule {
            pNext: ::core::ptr::null_mut::<amatch_rule>(),
            zFrom: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            rCost: 0,
            iLang: 0,
            nFrom: 0,
            nTo: 0,
            zTo: [0; 4],
        };
        let mut pTail: *mut amatch_rule = ::core::ptr::null_mut::<amatch_rule>();
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
unsafe extern "C" fn amatchLoadOneRule(
    mut p: *mut amatch_vtab,
    mut pStmt: *mut sqlite3_stmt,
    mut ppRule: *mut *mut amatch_rule,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iLang: sqlite3_int64 = sqlite3_column_int64(
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
        let mut rCost: amatch_cost = sqlite3_column_int(pStmt, 3 as ::core::ffi::c_int)
            as amatch_cost;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nFrom: ::core::ffi::c_int = 0;
        let mut nTo: ::core::ffi::c_int = 0;
        let mut pRule: *mut amatch_rule = ::core::ptr::null_mut::<amatch_rule>();
        if zFrom.is_null() {
            zFrom = b"\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if zTo.is_null() {
            zTo = b"\0".as_ptr() as *const ::core::ffi::c_char;
        }
        nFrom = strlen(zFrom) as ::core::ffi::c_int;
        nTo = strlen(zTo) as ::core::ffi::c_int;
        if strcmp(zFrom, zTo) == 0 as ::core::ffi::c_int {
            if *zFrom.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '?' as i32
                && *zFrom.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                if (*p).rSub == 0 as ::core::ffi::c_int || (*p).rSub > rCost {
                    (*p).rSub = rCost;
                }
            }
            *ppRule = ::core::ptr::null_mut::<amatch_rule>();
            return SQLITE_OK;
        }
        if rCost <= 0 as ::core::ffi::c_int || rCost > AMATCH_MX_COST {
            *pzErr = sqlite3_mprintf(
                b"%s: cost must be between 1 and %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zClassName,
                AMATCH_MX_COST,
            );
            rc = SQLITE_ERROR;
        } else if nFrom > AMATCH_MX_LENGTH || nTo > AMATCH_MX_LENGTH {
            *pzErr = sqlite3_mprintf(
                b"%s: maximum string length is %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zClassName,
                AMATCH_MX_LENGTH,
            );
            rc = SQLITE_ERROR;
        } else if iLang < 0 as ::core::ffi::c_longlong
            || iLang > AMATCH_MX_LANGID as ::core::ffi::c_longlong
        {
            *pzErr = sqlite3_mprintf(
                b"%s: iLang must be between 0 and %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zClassName,
                AMATCH_MX_LANGID,
            );
            rc = SQLITE_ERROR;
        } else if strcmp(zFrom, b"\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            && strcmp(zTo, b"?\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            if (*p).rIns == 0 as ::core::ffi::c_int || (*p).rIns > rCost {
                (*p).rIns = rCost;
            }
        } else if strcmp(zFrom, b"?\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            && strcmp(zTo, b"\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            if (*p).rDel == 0 as ::core::ffi::c_int || (*p).rDel > rCost {
                (*p).rDel = rCost;
            }
        } else {
            pRule = sqlite3_malloc64(
                (::core::mem::size_of::<amatch_rule>() as usize)
                    .wrapping_add(nFrom as usize)
                    .wrapping_add(nTo as usize) as sqlite3_uint64,
            ) as *mut amatch_rule;
            if pRule.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                memset(
                    pRule as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<amatch_rule>() as size_t,
                );
                (*pRule).zFrom = (&raw mut (*pRule).zTo as *mut ::core::ffi::c_char)
                    .offset((nTo + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char;
                (*pRule).nFrom = nFrom as amatch_len;
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
                (*pRule).nTo = nTo as amatch_len;
                (*pRule).rCost = rCost;
                (*pRule).iLang = iLang as ::core::ffi::c_int as amatch_langid;
            }
        }
        *ppRule = pRule;
        return rc;
    }
}
unsafe extern "C" fn amatchFreeRules(mut p: *mut amatch_vtab) {
    unsafe {
        while !(*p).pRule.is_null() {
            let mut pRule: *mut amatch_rule = (*p).pRule;
            (*p).pRule = (*pRule).pNext;
            sqlite3_free(pRule as *mut ::core::ffi::c_void);
        }
        (*p).pRule = ::core::ptr::null_mut::<amatch_rule>();
    }
}
unsafe extern "C" fn amatchLoadRules(
    mut db: *mut sqlite3,
    mut p: *mut amatch_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pHead: *mut amatch_rule = ::core::ptr::null_mut::<amatch_rule>();
        zSql = sqlite3_mprintf(
            b"SELECT * FROM %Q.%Q\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).zDb,
            (*p).zCostTab,
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
                    (*p).zCostTab,
                    sqlite3_column_count(pStmt),
                );
                rc = SQLITE_ERROR;
            } else {
                while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pStmt) {
                    let mut pRule: *mut amatch_rule = ::core::ptr::null_mut::<
                        amatch_rule,
                    >();
                    rc = amatchLoadOneRule(p, pStmt, &raw mut pRule, pzErr);
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
            let mut pX: *mut amatch_rule = ::core::ptr::null_mut::<amatch_rule>();
            let mut a: [*mut amatch_rule; 15] = [::core::ptr::null_mut::<
                amatch_rule,
            >(); 15];
            i = 0 as ::core::ffi::c_uint;
            while (i as usize)
                < (::core::mem::size_of::<[*mut amatch_rule; 15]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*mut amatch_rule>() as usize)
            {
                a[i as usize] = ::core::ptr::null_mut::<amatch_rule>();
                i = i.wrapping_add(1);
            }
            loop {
                pX = pHead;
                if pX.is_null() {
                    break;
                }
                pHead = (*pX).pNext;
                (*pX).pNext = ::core::ptr::null_mut::<amatch_rule>();
                i = 0 as ::core::ffi::c_uint;
                while !a[i as usize].is_null()
                    && (i as usize)
                        < (::core::mem::size_of::<[*mut amatch_rule; 15]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<*mut amatch_rule>() as usize,
                            )
                            .wrapping_sub(1 as usize)
                {
                    pX = amatchMergeRules(a[i as usize], pX);
                    a[i as usize] = ::core::ptr::null_mut::<amatch_rule>();
                    i = i.wrapping_add(1);
                }
                a[i as usize] = amatchMergeRules(a[i as usize], pX);
            }
            pX = a[0 as ::core::ffi::c_int as usize];
            i = 1 as ::core::ffi::c_uint;
            while (i as usize)
                < (::core::mem::size_of::<[*mut amatch_rule; 15]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*mut amatch_rule>() as usize)
            {
                pX = amatchMergeRules(a[i as usize], pX);
                i = i.wrapping_add(1);
            }
            (*p).pRule = amatchMergeRules((*p).pRule, pX);
        } else {
            (*p).pRule = pHead;
        }
        return rc;
    }
}
unsafe extern "C" fn amatchDequote(
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
                    let c2rust_fresh1 = iOut;
                    iOut = iOut + 1;
                    *zOut.offset(c2rust_fresh1 as isize) = *zIn.offset(iIn as isize);
                    iIn += 1;
                }
            }
        }
        return zOut;
    }
}
unsafe extern "C" fn amatchVCheckClear(mut p: *mut amatch_vtab) {
    unsafe {
        if !(*p).pVCheck.is_null() {
            sqlite3_finalize((*p).pVCheck);
            (*p).pVCheck = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
    }
}
unsafe extern "C" fn amatchFree(mut p: *mut amatch_vtab) {
    unsafe {
        if !p.is_null() {
            amatchFreeRules(p);
            amatchVCheckClear(p);
            sqlite3_free((*p).zClassName as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zDb as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zCostTab as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zVocabTab as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zVocabWord as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zVocabLang as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zSelf as *mut ::core::ffi::c_void);
            memset(
                p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<amatch_vtab>() as size_t,
            );
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn amatchDisconnect(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut amatch_vtab = pVtab as *mut amatch_vtab;
        amatchFree(p);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn amatchValueOfKey(
    mut zKey: *const ::core::ffi::c_char,
    mut zStr: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut nKey: ::core::ffi::c_int = strlen(zKey) as ::core::ffi::c_int;
        let mut nStr: ::core::ffi::c_int = strlen(zStr) as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        if nStr < nKey + 1 as ::core::ffi::c_int {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        if memcmp(
            zStr as *const ::core::ffi::c_void,
            zKey as *const ::core::ffi::c_void,
            nKey as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        i = nKey;
        while *(*__ctype_b_loc())
            .offset(
                *zStr.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
                    as isize,
            ) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            i += 1;
        }
        if *zStr.offset(i as isize) as ::core::ffi::c_int != '=' as i32 {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        i += 1;
        while *(*__ctype_b_loc())
            .offset(
                *zStr.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
                    as isize,
            ) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            i += 1;
        }
        return zStr.offset(i as isize);
    }
}
unsafe extern "C" fn amatchConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pNew: *mut amatch_vtab = ::core::ptr::null_mut::<amatch_vtab>();
        let mut zModule: *const ::core::ffi::c_char = *argv
            .offset(0 as ::core::ffi::c_int as isize);
        let mut zDb: *const ::core::ffi::c_char = *argv
            .offset(1 as ::core::ffi::c_int as isize);
        let mut zVal: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        *ppVtab = ::core::ptr::null_mut::<sqlite3_vtab>();
        pNew = sqlite3_malloc(
            ::core::mem::size_of::<amatch_vtab>() as ::core::ffi::c_int,
        ) as *mut amatch_vtab;
        if pNew.is_null() {
            return SQLITE_NOMEM;
        }
        rc = SQLITE_NOMEM;
        memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<amatch_vtab>() as size_t,
        );
        (*pNew).db = db;
        (*pNew).zClassName = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            zModule,
        );
        if !(*pNew).zClassName.is_null() {
            (*pNew).zDb = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                zDb,
            );
            if !(*pNew).zDb.is_null() {
                (*pNew).zSelf = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    *argv.offset(2 as ::core::ffi::c_int as isize),
                );
                if !(*pNew).zSelf.is_null() {
                    i = 3 as ::core::ffi::c_int;
                    loop {
                        if !(i < argc) {
                            c2rust_current_block = 8704759739624374314;
                            break;
                        }
                        zVal = amatchValueOfKey(
                            b"vocabulary_table\0".as_ptr() as *const ::core::ffi::c_char,
                            *argv.offset(i as isize),
                        );
                        if !zVal.is_null() {
                            sqlite3_free((*pNew).zVocabTab as *mut ::core::ffi::c_void);
                            (*pNew).zVocabTab = amatchDequote(zVal);
                            if (*pNew).zVocabTab.is_null() {
                                c2rust_current_block = 1619303625927841325;
                                break;
                            }
                        } else {
                            zVal = amatchValueOfKey(
                                b"vocabulary_word\0".as_ptr() as *const ::core::ffi::c_char,
                                *argv.offset(i as isize),
                            );
                            if !zVal.is_null() {
                                sqlite3_free(
                                    (*pNew).zVocabWord as *mut ::core::ffi::c_void,
                                );
                                (*pNew).zVocabWord = amatchDequote(zVal);
                                if (*pNew).zVocabWord.is_null() {
                                    c2rust_current_block = 1619303625927841325;
                                    break;
                                }
                            } else {
                                zVal = amatchValueOfKey(
                                    b"vocabulary_language\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    *argv.offset(i as isize),
                                );
                                if !zVal.is_null() {
                                    sqlite3_free(
                                        (*pNew).zVocabLang as *mut ::core::ffi::c_void,
                                    );
                                    (*pNew).zVocabLang = amatchDequote(zVal);
                                    if (*pNew).zVocabLang.is_null() {
                                        c2rust_current_block = 1619303625927841325;
                                        break;
                                    }
                                } else {
                                    zVal = amatchValueOfKey(
                                        b"edit_distances\0".as_ptr() as *const ::core::ffi::c_char,
                                        *argv.offset(i as isize),
                                    );
                                    if !zVal.is_null() {
                                        sqlite3_free((*pNew).zCostTab as *mut ::core::ffi::c_void);
                                        (*pNew).zCostTab = amatchDequote(zVal);
                                        if (*pNew).zCostTab.is_null() {
                                            c2rust_current_block = 1619303625927841325;
                                            break;
                                        }
                                    } else {
                                        *pzErr = sqlite3_mprintf(
                                            b"unrecognized argument: [%s]\n\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            *argv.offset(i as isize),
                                        );
                                        amatchFree(pNew);
                                        *ppVtab = ::core::ptr::null_mut::<sqlite3_vtab>();
                                        return SQLITE_ERROR;
                                    }
                                }
                            }
                        }
                        i += 1;
                    }
                    match c2rust_current_block {
                        1619303625927841325 => {}
                        _ => {
                            rc = SQLITE_OK;
                            if (*pNew).zCostTab.is_null() {
                                *pzErr = sqlite3_mprintf(
                                    b"no edit_distances table specified\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                rc = SQLITE_ERROR;
                            } else {
                                rc = amatchLoadRules(db, pNew, pzErr);
                            }
                            if rc == SQLITE_OK {
                                sqlite3_vtab_config(db, SQLITE_VTAB_INNOCUOUS);
                                rc = sqlite3_declare_vtab(
                                    db,
                                    b"CREATE TABLE x(word,distance,language,command HIDDEN,nword HIDDEN)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                            }
                            if rc != SQLITE_OK {
                                amatchFree(pNew);
                            }
                            *ppVtab = &raw mut (*pNew).base;
                            return rc;
                        }
                    }
                }
            }
        }
        amatchFree(pNew);
        return rc;
    }
}
pub const AMATCH_COL_WORD: ::core::ffi::c_int = 0;
pub const AMATCH_COL_DISTANCE: ::core::ffi::c_int = 1;
pub const AMATCH_COL_LANGUAGE: ::core::ffi::c_int = 2;
pub const AMATCH_COL_COMMAND: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const AMATCH_COL_NWORD: ::core::ffi::c_int = 4;
unsafe extern "C" fn amatchOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut amatch_vtab = pVTab as *mut amatch_vtab;
        let mut pCur: *mut amatch_cursor = ::core::ptr::null_mut::<amatch_cursor>();
        pCur = sqlite3_malloc(
            ::core::mem::size_of::<amatch_cursor>() as ::core::ffi::c_int,
        ) as *mut amatch_cursor;
        if pCur.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pCur as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<amatch_cursor>() as size_t,
        );
        (*pCur).pVtab = p;
        *ppCursor = &raw mut (*pCur).base;
        (*p).nCursor += 1;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn amatchClearCursor(mut pCur: *mut amatch_cursor) {
    unsafe {
        let mut pWord: *mut amatch_word = ::core::ptr::null_mut::<amatch_word>();
        let mut pNextWord: *mut amatch_word = ::core::ptr::null_mut::<amatch_word>();
        pWord = (*pCur).pAllWords;
        while !pWord.is_null() {
            pNextWord = (*pWord).pNext;
            sqlite3_free(pWord as *mut ::core::ffi::c_void);
            pWord = pNextWord;
        }
        (*pCur).pAllWords = ::core::ptr::null_mut::<amatch_word>();
        sqlite3_free((*pCur).zInput as *mut ::core::ffi::c_void);
        (*pCur).zInput = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sqlite3_free((*pCur).zBuf as *mut ::core::ffi::c_void);
        (*pCur).zBuf = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pCur).nBuf = 0 as sqlite3_int64;
        (*pCur).pCost = ::core::ptr::null_mut::<amatch_avl>();
        (*pCur).pWord = ::core::ptr::null_mut::<amatch_avl>();
        (*pCur).pCurrent = ::core::ptr::null_mut::<amatch_word>();
        (*pCur).rLimit = 1000000 as ::core::ffi::c_int as amatch_cost;
        (*pCur).iLang = 0 as ::core::ffi::c_int as amatch_langid;
        (*pCur).nWord = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn amatchClose(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut amatch_cursor = cur as *mut amatch_cursor;
        amatchClearCursor(pCur);
        (*(*pCur).pVtab).nCursor -= 1;
        sqlite3_free(pCur as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn amatchEncodeInt(
    mut x: ::core::ffi::c_int,
    mut z: *mut ::core::ffi::c_char,
) {
    unsafe {
        static mut a: [::core::ffi::c_char; 65] = unsafe {
            ::core::mem::transmute::<
                [u8; 65],
                [::core::ffi::c_char; 65],
            >(*b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ^abcdefghijklmnopqrstuvwxyz~\0")
        };
        *z.offset(0 as ::core::ffi::c_int as isize) = a[(x >> 18 as ::core::ffi::c_int
            & 0x3f as ::core::ffi::c_int) as usize];
        *z.offset(1 as ::core::ffi::c_int as isize) = a[(x >> 12 as ::core::ffi::c_int
            & 0x3f as ::core::ffi::c_int) as usize];
        *z.offset(2 as ::core::ffi::c_int as isize) = a[(x >> 6 as ::core::ffi::c_int
            & 0x3f as ::core::ffi::c_int) as usize];
        *z.offset(3 as ::core::ffi::c_int as isize) = a[(x & 0x3f as ::core::ffi::c_int)
            as usize];
    }
}
unsafe extern "C" fn amatchWriteCost(mut pWord: *mut amatch_word) {
    unsafe {
        amatchEncodeInt(
            (*pWord).rCost as ::core::ffi::c_int,
            &raw mut (*pWord).zCost as *mut ::core::ffi::c_char,
        );
        amatchEncodeInt(
            (*pWord).iSeq,
            (&raw mut (*pWord).zCost as *mut ::core::ffi::c_char)
                .offset(4 as ::core::ffi::c_int as isize),
        );
        (*pWord).zCost[8 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn amatchStrcpy(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
) {
    unsafe {
        loop {
            let c2rust_fresh2 = src;
            src = src.offset(1);
            let c2rust_fresh3 = dest;
            dest = dest.offset(1);
            *c2rust_fresh3 = *c2rust_fresh2;
            if !(*c2rust_fresh3 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                break;
            }
        };
    }
}
unsafe extern "C" fn amatchStrcat(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
) {
    unsafe {
        while *dest != 0 {
            dest = dest.offset(1);
        }
        amatchStrcpy(dest, src);
    }
}
unsafe extern "C" fn amatchAddWord(
    mut pCur: *mut amatch_cursor,
    mut rCost: amatch_cost,
    mut nMatch: ::core::ffi::c_int,
    mut zWordBase: *const ::core::ffi::c_char,
    mut zWordTail: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut pWord: *mut amatch_word = ::core::ptr::null_mut::<amatch_word>();
        let mut pNode: *mut amatch_avl = ::core::ptr::null_mut::<amatch_avl>();
        let mut pOther: *mut amatch_avl = ::core::ptr::null_mut::<amatch_avl>();
        let mut nBase: ::core::ffi::c_int = 0;
        let mut nTail: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 4] = [0; 4];
        if rCost > (*pCur).rLimit {
            return;
        }
        nBase = strlen(zWordBase) as ::core::ffi::c_int;
        nTail = strlen(zWordTail) as ::core::ffi::c_int;
        if (nBase + nTail + 3 as ::core::ffi::c_int) as ::core::ffi::c_longlong
            > (*pCur).nBuf
        {
            (*pCur).nBuf = (nBase + nTail + 100 as ::core::ffi::c_int) as sqlite3_int64;
            (*pCur).zBuf = sqlite3_realloc64(
                (*pCur).zBuf as *mut ::core::ffi::c_void,
                (*pCur).nBuf as sqlite3_uint64,
            ) as *mut ::core::ffi::c_char;
            if (*pCur).zBuf.is_null() {
                (*pCur).nBuf = 0 as sqlite3_int64;
                return;
            }
        }
        amatchEncodeInt(nMatch, &raw mut zBuf as *mut ::core::ffi::c_char);
        memcpy(
            (*pCur).zBuf as *mut ::core::ffi::c_void,
            (&raw mut zBuf as *mut ::core::ffi::c_char)
                .offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            2 as size_t,
        );
        memcpy(
            (*pCur).zBuf.offset(2 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            zWordBase as *const ::core::ffi::c_void,
            nBase as size_t,
        );
        memcpy(
            (*pCur).zBuf.offset(2 as ::core::ffi::c_int as isize).offset(nBase as isize)
                as *mut ::core::ffi::c_void,
            zWordTail as *const ::core::ffi::c_void,
            (nTail + 1 as ::core::ffi::c_int) as size_t,
        );
        pNode = amatchAvlSearch((*pCur).pWord, (*pCur).zBuf);
        if !pNode.is_null() {
            pWord = (*pNode).pWord;
            if (*pWord).rCost > rCost {
                amatchAvlRemove(&raw mut (*pCur).pCost, &raw mut (*pWord).sCost);
                (*pWord).rCost = rCost;
                amatchWriteCost(pWord);
                pOther = amatchAvlInsert(
                    &raw mut (*pCur).pCost,
                    &raw mut (*pWord).sCost,
                );
            }
            return;
        }
        pWord = sqlite3_malloc64(
            (::core::mem::size_of::<amatch_word>() as usize)
                .wrapping_add(nBase as usize)
                .wrapping_add(nTail as usize)
                .wrapping_sub(1 as usize) as sqlite3_uint64,
        ) as *mut amatch_word;
        if pWord.is_null() {
            return;
        }
        memset(
            pWord as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<amatch_word>() as size_t,
        );
        (*pWord).rCost = rCost;
        let c2rust_fresh4 = (*pCur).nWord;
        (*pCur).nWord = (*pCur).nWord + 1;
        (*pWord).iSeq = c2rust_fresh4;
        amatchWriteCost(pWord);
        (*pWord).nMatch = nMatch as ::core::ffi::c_short;
        (*pWord).pNext = (*pCur).pAllWords;
        (*pCur).pAllWords = pWord;
        (*pWord).sCost.zKey = &raw mut (*pWord).zCost as *mut ::core::ffi::c_char;
        (*pWord).sCost.pWord = pWord;
        pOther = amatchAvlInsert(&raw mut (*pCur).pCost, &raw mut (*pWord).sCost);
        (*pWord).sWord.zKey = &raw mut (*pWord).zWord as *mut ::core::ffi::c_char;
        (*pWord).sWord.pWord = pWord;
        amatchStrcpy(&raw mut (*pWord).zWord as *mut ::core::ffi::c_char, (*pCur).zBuf);
        pOther = amatchAvlInsert(&raw mut (*pCur).pWord, &raw mut (*pWord).sWord);
    }
}
unsafe extern "C" fn amatchNext(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut amatch_cursor = cur as *mut amatch_cursor;
        let mut pWord: *mut amatch_word = ::core::ptr::null_mut::<amatch_word>();
        let mut pNode: *mut amatch_avl = ::core::ptr::null_mut::<amatch_avl>();
        let mut isMatch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p: *mut amatch_vtab = (*pCur).pVtab;
        let mut nWord: sqlite3_int64 = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut zW: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pRule: *mut amatch_rule = ::core::ptr::null_mut::<amatch_rule>();
        let mut zBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nBuf: sqlite3_int64 = 0 as sqlite3_int64;
        let mut zNext: [::core::ffi::c_char; 8] = [0; 8];
        let mut zNextIn: [::core::ffi::c_char; 8] = [0; 8];
        let mut nNextIn: ::core::ffi::c_int = 0;
        if (*p).pVCheck.is_null() {
            let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            if !(*p).zVocabLang.is_null()
                && *(*p).zVocabLang.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int != 0
            {
                zSql = sqlite3_mprintf(
                    b"SELECT \"%w\" FROM \"%w\"\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b" WHERE \"%w\">=?1 AND \"%w\"=?2 ORDER BY 1\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*p).zVocabWord,
                    (*p).zVocabTab,
                    (*p).zVocabWord,
                    (*p).zVocabLang,
                );
            } else {
                zSql = sqlite3_mprintf(
                    b"SELECT \"%w\" FROM \"%w\" WHERE \"%w\">=?1 ORDER BY 1\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*p).zVocabWord,
                    (*p).zVocabTab,
                    (*p).zVocabWord,
                );
            }
            rc = sqlite3_prepare_v2(
                (*p).db,
                zSql,
                -1 as ::core::ffi::c_int,
                &raw mut (*p).pVCheck,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
            if rc != 0 {
                return rc;
            }
        }
        sqlite3_bind_int(
            (*p).pVCheck,
            2 as ::core::ffi::c_int,
            (*pCur).iLang as ::core::ffi::c_int,
        );
        loop {
            pNode = amatchAvlFirst((*pCur).pCost);
            if pNode.is_null() {
                pWord = ::core::ptr::null_mut::<amatch_word>();
                break;
            } else {
                pWord = (*pNode).pWord;
                amatchAvlRemove(&raw mut (*pCur).pCost, &raw mut (*pWord).sCost);
                nWord = strlen(
                    (&raw mut (*pWord).zWord as *mut ::core::ffi::c_char)
                        .offset(2 as ::core::ffi::c_int as isize),
                ) as ::core::ffi::c_int as sqlite3_int64;
                if nWord as ::core::ffi::c_longlong + 20 as ::core::ffi::c_longlong
                    > nBuf
                {
                    nBuf = (nWord as ::core::ffi::c_longlong
                        + 100 as ::core::ffi::c_longlong) as ::core::ffi::c_char
                        as sqlite3_int64;
                    zBuf = sqlite3_realloc64(
                        zBuf as *mut ::core::ffi::c_void,
                        nBuf as sqlite3_uint64,
                    ) as *mut ::core::ffi::c_char;
                    if zBuf.is_null() {
                        return SQLITE_NOMEM;
                    }
                }
                amatchStrcpy(
                    zBuf,
                    (&raw mut (*pWord).zWord as *mut ::core::ffi::c_char)
                        .offset(2 as ::core::ffi::c_int as isize),
                );
                zNext[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                zNextIn[0 as ::core::ffi::c_int as usize] = *(*pCur)
                    .zInput
                    .offset((*pWord).nMatch as isize);
                if zNextIn[0 as ::core::ffi::c_int as usize] != 0 {
                    i = 1 as ::core::ffi::c_int;
                    while i <= 4 as ::core::ffi::c_int
                        && *(*pCur)
                            .zInput
                            .offset(((*pWord).nMatch as ::core::ffi::c_int + i) as isize)
                            as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                            == 0x80 as ::core::ffi::c_int
                    {
                        zNextIn[i as usize] = *(*pCur)
                            .zInput
                            .offset(
                                ((*pWord).nMatch as ::core::ffi::c_int + i) as isize,
                            );
                        i += 1;
                    }
                    zNextIn[i as usize] = 0 as ::core::ffi::c_char;
                    nNextIn = i;
                } else {
                    nNextIn = 0 as ::core::ffi::c_int;
                }
                if zNextIn[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
                    && zNextIn[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        != '*' as i32
                {
                    sqlite3_reset((*p).pVCheck);
                    amatchStrcat(zBuf, &raw mut zNextIn as *mut ::core::ffi::c_char);
                    sqlite3_bind_text(
                        (*p).pVCheck,
                        1 as ::core::ffi::c_int,
                        zBuf,
                        (nWord as ::core::ffi::c_longlong
                            + nNextIn as ::core::ffi::c_longlong) as ::core::ffi::c_int,
                        SQLITE_STATIC,
                    );
                    rc = sqlite3_step((*p).pVCheck);
                    if rc == SQLITE_ROW {
                        zW = sqlite3_column_text((*p).pVCheck, 0 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                        if strncmp(
                            zBuf,
                            zW,
                            (nWord as ::core::ffi::c_longlong
                                + nNextIn as ::core::ffi::c_longlong) as size_t,
                        ) == 0 as ::core::ffi::c_int
                        {
                            amatchAddWord(
                                pCur,
                                (*pWord).rCost,
                                (*pWord).nMatch as ::core::ffi::c_int + nNextIn,
                                zBuf,
                                b"\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    *zBuf.offset(nWord as isize) = 0 as ::core::ffi::c_char;
                }
                loop {
                    amatchStrcpy(
                        zBuf.offset(nWord as isize),
                        &raw mut zNext as *mut ::core::ffi::c_char,
                    );
                    sqlite3_reset((*p).pVCheck);
                    sqlite3_bind_text(
                        (*p).pVCheck,
                        1 as ::core::ffi::c_int,
                        zBuf,
                        -1 as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                    rc = sqlite3_step((*p).pVCheck);
                    if rc != SQLITE_ROW {
                        break;
                    }
                    zW = sqlite3_column_text((*p).pVCheck, 0 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char;
                    amatchStrcpy(
                        zBuf.offset(nWord as isize),
                        &raw mut zNext as *mut ::core::ffi::c_char,
                    );
                    if strncmp(zW, zBuf, nWord as size_t) != 0 as ::core::ffi::c_int {
                        break;
                    }
                    if zNextIn[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == '*' as i32
                        && zNextIn[1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || zNextIn[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && *zW.offset(nWord as isize) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                    {
                        isMatch = 1 as ::core::ffi::c_int;
                        zNextIn[0 as ::core::ffi::c_int as usize] = 0
                            as ::core::ffi::c_char;
                        nNextIn = 0 as ::core::ffi::c_int;
                        break;
                    } else {
                        zNext[0 as ::core::ffi::c_int as usize] = *zW
                            .offset(nWord as isize);
                        i = 1 as ::core::ffi::c_int;
                        while i <= 4 as ::core::ffi::c_int
                            && *zW
                                .offset(
                                    (nWord as ::core::ffi::c_longlong
                                        + i as ::core::ffi::c_longlong) as isize,
                                ) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                                == 0x80 as ::core::ffi::c_int
                        {
                            zNext[i as usize] = *zW
                                .offset(
                                    (nWord as ::core::ffi::c_longlong
                                        + i as ::core::ffi::c_longlong) as isize,
                                );
                            i += 1;
                        }
                        zNext[i as usize] = 0 as ::core::ffi::c_char;
                        *zBuf.offset(nWord as isize) = 0 as ::core::ffi::c_char;
                        if (*p).rIns > 0 as ::core::ffi::c_int {
                            amatchAddWord(
                                pCur,
                                (*pWord).rCost + (*p).rIns,
                                (*pWord).nMatch as ::core::ffi::c_int,
                                zBuf,
                                &raw mut zNext as *mut ::core::ffi::c_char,
                            );
                        }
                        if (*p).rSub > 0 as ::core::ffi::c_int {
                            amatchAddWord(
                                pCur,
                                (*pWord).rCost + (*p).rSub,
                                (*pWord).nMatch as ::core::ffi::c_int + nNextIn,
                                zBuf,
                                &raw mut zNext as *mut ::core::ffi::c_char,
                            );
                        }
                        if (*p).rIns < 0 as ::core::ffi::c_int
                            && (*p).rSub < 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        zNext[(i - 1 as ::core::ffi::c_int) as usize] += 1;
                    }
                }
                sqlite3_reset((*p).pVCheck);
                if (*p).rDel > 0 as ::core::ffi::c_int {
                    *zBuf.offset(nWord as isize) = 0 as ::core::ffi::c_char;
                    amatchAddWord(
                        pCur,
                        (*pWord).rCost + (*p).rDel,
                        (*pWord).nMatch as ::core::ffi::c_int + nNextIn,
                        zBuf,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                pRule = (*p).pRule;
                while !pRule.is_null() {
                    if !((*pRule).iLang != (*pCur).iLang) {
                        if strncmp(
                            (*pRule).zFrom,
                            (*pCur)
                                .zInput
                                .offset((*pWord).nMatch as ::core::ffi::c_int as isize),
                            (*pRule).nFrom as size_t,
                        ) == 0 as ::core::ffi::c_int
                        {
                            amatchAddWord(
                                pCur,
                                (*pWord).rCost + (*pRule).rCost,
                                (*pWord).nMatch as ::core::ffi::c_int
                                    + (*pRule).nFrom as ::core::ffi::c_int,
                                (&raw mut (*pWord).zWord as *mut ::core::ffi::c_char)
                                    .offset(2 as ::core::ffi::c_int as isize),
                                &raw mut (*pRule).zTo as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                    pRule = (*pRule).pNext;
                }
                if !(isMatch == 0) {
                    break;
                }
            }
        }
        (*pCur).pCurrent = pWord;
        sqlite3_free(zBuf as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn amatchFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut amatch_cursor = pVtabCursor as *mut amatch_cursor;
        let mut zWord: *const ::core::ffi::c_char = b"*\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut idx: ::core::ffi::c_int = 0;
        amatchClearCursor(pCur);
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
            (*pCur).iLang = sqlite3_value_int(*argv.offset(idx as isize))
                as amatch_langid;
            idx += 1;
        }
        (*pCur).zInput = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            zWord,
        );
        if (*pCur).zInput.is_null() {
            return SQLITE_NOMEM;
        }
        amatchAddWord(
            pCur,
            0 as amatch_cost,
            0 as ::core::ffi::c_int,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        amatchNext(pVtabCursor);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn amatchColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut amatch_cursor = cur as *mut amatch_cursor;
        match i {
            AMATCH_COL_WORD => {
                sqlite3_result_text(
                    ctx,
                    (&raw mut (*(*pCur).pCurrent).zWord as *mut ::core::ffi::c_char)
                        .offset(2 as ::core::ffi::c_int as isize),
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            AMATCH_COL_DISTANCE => {
                sqlite3_result_int(ctx, (*(*pCur).pCurrent).rCost as ::core::ffi::c_int);
            }
            AMATCH_COL_LANGUAGE => {
                sqlite3_result_int(ctx, (*pCur).iLang as ::core::ffi::c_int);
            }
            AMATCH_COL_NWORD => {
                sqlite3_result_int(ctx, (*pCur).nWord);
            }
            _ => {
                sqlite3_result_null(ctx);
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn amatchRowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut amatch_cursor = cur as *mut amatch_cursor;
        *pRowid = (*pCur).iRowid as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn amatchEof(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut amatch_cursor = cur as *mut amatch_cursor;
        return (*pCur).pCurrent.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn amatchBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iPlan: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iDistTerm: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iLangTerm: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut pConstraint: *const sqlite3_index_constraint = ::core::ptr::null::<
            sqlite3_index_constraint,
        >();
        pConstraint = (*pIdxInfo).aConstraint;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdxInfo).nConstraint {
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
                }
                if iPlan & 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*pConstraint).iColumn == 2 as ::core::ffi::c_int
                    && (*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_EQ
                {
                    iPlan |= 4 as ::core::ffi::c_int;
                    (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).omit = 1
                        as ::core::ffi::c_uchar;
                    iLangTerm = i;
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
            (*(*pIdxInfo).aConstraintUsage.offset(iLangTerm as isize)).argvIndex = idx;
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
        (*pIdxInfo).estimatedCost = 10000 as ::core::ffi::c_int as ::core::ffi::c_double;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn amatchUpdate(
    mut pVTab: *mut sqlite3_vtab,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut amatch_vtab = pVTab as *mut amatch_vtab;
        let mut zCmd: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        if argc == 1 as ::core::ffi::c_int {
            (*pVTab).zErrMsg = sqlite3_mprintf(
                b"DELETE from %s is not allowed\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zSelf,
            );
            return SQLITE_ERROR;
        }
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            != SQLITE_NULL
        {
            (*pVTab).zErrMsg = sqlite3_mprintf(
                b"UPDATE of %s is not allowed\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).zSelf,
            );
            return SQLITE_ERROR;
        }
        if sqlite3_value_type(
            *argv.offset((2 as ::core::ffi::c_int + AMATCH_COL_WORD) as isize),
        ) != SQLITE_NULL
            || sqlite3_value_type(
                *argv.offset((2 as ::core::ffi::c_int + AMATCH_COL_DISTANCE) as isize),
            ) != SQLITE_NULL
            || sqlite3_value_type(
                *argv.offset((2 as ::core::ffi::c_int + AMATCH_COL_LANGUAGE) as isize),
            ) != SQLITE_NULL
        {
            (*pVTab).zErrMsg = sqlite3_mprintf(
                b"INSERT INTO %s allowed for column [command] only\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zSelf,
            );
            return SQLITE_ERROR;
        }
        zCmd = sqlite3_value_text(
            *argv.offset((2 as ::core::ffi::c_int + AMATCH_COL_COMMAND) as isize),
        );
        if zCmd.is_null() {
            return SQLITE_OK;
        }
        return SQLITE_OK;
    }
}
static mut amatchModule: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: Some(
            amatchConnect
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
            amatchConnect
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
            amatchBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            amatchDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            amatchDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            amatchOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            amatchClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            amatchFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            amatchNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            amatchEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            amatchColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            amatchRowid
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: Some(
            amatchUpdate
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
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
pub unsafe extern "C" fn sqlite3_amatch_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        rc = sqlite3_create_module(
            db,
            b"approximate_match\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut amatchModule,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return rc;
    }
}
