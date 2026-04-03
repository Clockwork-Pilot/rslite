unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_str;
    pub type sqlite3_blob;
    pub type sqlite3_backup;
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
    fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
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
    fn sqlite3_prepare(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_value(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const sqlite3_value,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3_column_bytes(
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
    fn sqlite3_create_function_v2(
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
        xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_null(_: *mut sqlite3_context);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
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
    fn sqlite3_vtab_config(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn sqlite3_vtab_on_conflict(_: *mut sqlite3) -> ::core::ffi::c_int;
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
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type u8_0 = ::core::ffi::c_uchar;
pub type u16_0 = ::core::ffi::c_ushort;
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
pub struct EditDist3Cost {
    pub pNext: *mut EditDist3Cost,
    pub nFrom: u8_0,
    pub nTo: u8_0,
    pub iCost: u16_0,
    pub a: [::core::ffi::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EditDist3Config {
    pub nLang: ::core::ffi::c_int,
    pub a: *mut EditDist3Lang,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EditDist3Lang {
    pub iLang: ::core::ffi::c_int,
    pub iInsCost: ::core::ffi::c_int,
    pub iDelCost: ::core::ffi::c_int,
    pub iSubCost: ::core::ffi::c_int,
    pub pCost: *mut EditDist3Cost,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EditDist3From {
    pub nSubst: ::core::ffi::c_int,
    pub nDel: ::core::ffi::c_int,
    pub nByte: ::core::ffi::c_int,
    pub apSubst: *mut *mut EditDist3Cost,
    pub apDel: *mut *mut EditDist3Cost,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EditDist3FromString {
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub isPrefix: ::core::ffi::c_int,
    pub a: *mut EditDist3From,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EditDist3To {
    pub nIns: ::core::ffi::c_int,
    pub nByte: ::core::ffi::c_int,
    pub apIns: *mut *mut EditDist3Cost,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Transliteration {
    pub cFrom: ::core::ffi::c_ushort,
    pub cTo0: ::core::ffi::c_uchar,
    pub cTo1: ::core::ffi::c_uchar,
    pub cTo2: ::core::ffi::c_uchar,
    pub cTo3: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spellfix1_vtab {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub zDbName: *mut ::core::ffi::c_char,
    pub zTableName: *mut ::core::ffi::c_char,
    pub zCostTable: *mut ::core::ffi::c_char,
    pub pConfig3: *mut EditDist3Config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spellfix1_cursor {
    pub base: sqlite3_vtab_cursor,
    pub pVTab: *mut spellfix1_vtab,
    pub zPattern: *mut ::core::ffi::c_char,
    pub idxNum: ::core::ffi::c_int,
    pub nRow: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
    pub iRow: ::core::ffi::c_int,
    pub iLang: ::core::ffi::c_int,
    pub iTop: ::core::ffi::c_int,
    pub iScope: ::core::ffi::c_int,
    pub nSearch: ::core::ffi::c_int,
    pub pFullScan: *mut sqlite3_stmt,
    pub a: *mut spellfix1_row,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spellfix1_row {
    pub iRowid: sqlite3_int64,
    pub zWord: *mut ::core::ffi::c_char,
    pub iRank: ::core::ffi::c_int,
    pub iDistance: ::core::ffi::c_int,
    pub iScore: ::core::ffi::c_int,
    pub iMatchlen: ::core::ffi::c_int,
    pub zHash: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchQuery {
    pub pCur: *mut spellfix1_cursor,
    pub pStmt: *mut sqlite3_stmt,
    pub zHash: [::core::ffi::c_char; 32],
    pub zPattern: *const ::core::ffi::c_char,
    pub nPattern: ::core::ffi::c_int,
    pub pMatchStr3: *mut EditDist3FromString,
    pub pConfig3: *mut EditDist3Config,
    pub pLang: *const EditDist3Lang,
    pub iLang: ::core::ffi::c_int,
    pub iScope: ::core::ffi::c_int,
    pub iMaxDist: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub nRun: ::core::ffi::c_int,
    pub azPrior: [[::core::ffi::c_char; 32]; 1],
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_NOTNULL: ::core::ffi::c_int = SQLITE_CONSTRAINT
    | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_MATCH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SQLITE_VTAB_INNOCUOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CCLASS_SILENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CCLASS_VOWEL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CCLASS_B: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CCLASS_C: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CCLASS_D: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CCLASS_L: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CCLASS_R: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CCLASS_M: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CCLASS_Y: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const CCLASS_DIGIT: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const CCLASS_SPACE: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const CCLASS_OTHER: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
static mut midClass: [::core::ffi::c_uchar; 128] = [
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_SPACE as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_SPACE as ::core::ffi::c_uchar,
    CCLASS_SPACE as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_SPACE as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_SILENT as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_D as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_SILENT as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_L as ::core::ffi::c_uchar,
    CCLASS_M as ::core::ffi::c_uchar,
    CCLASS_M as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_R as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_D as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_D as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_SILENT as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_L as ::core::ffi::c_uchar,
    CCLASS_M as ::core::ffi::c_uchar,
    CCLASS_M as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_R as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_D as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
];
static mut initClass: [::core::ffi::c_uchar; 128] = [
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_SPACE as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_SPACE as ::core::ffi::c_uchar,
    CCLASS_SPACE as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_SPACE as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_DIGIT as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_D as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_SILENT as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_L as ::core::ffi::c_uchar,
    CCLASS_M as ::core::ffi::c_uchar,
    CCLASS_M as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_R as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_D as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_Y as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_D as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_SILENT as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_L as ::core::ffi::c_uchar,
    CCLASS_M as ::core::ffi::c_uchar,
    CCLASS_M as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_R as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_D as ::core::ffi::c_uchar,
    CCLASS_VOWEL as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_B as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_Y as ::core::ffi::c_uchar,
    CCLASS_C as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
    CCLASS_OTHER as ::core::ffi::c_uchar,
];
static mut className: [::core::ffi::c_uchar; 14] = unsafe {
    ::core::mem::transmute::<[u8; 14], [::core::ffi::c_uchar; 14]>(*b".ABCDHLRMY9 ?\0")
};
unsafe extern "C" fn phoneticHash(
    mut zIn: *const ::core::ffi::c_uchar,
    mut nIn: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    unsafe {
        let mut zOut: *mut ::core::ffi::c_uchar = sqlite3_malloc64(
            (nIn + 1 as ::core::ffi::c_int) as sqlite3_uint64,
        ) as *mut ::core::ffi::c_uchar;
        let mut i: ::core::ffi::c_int = 0;
        let mut nOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut cPrev: ::core::ffi::c_char = 0x77 as ::core::ffi::c_char;
        let mut cPrevX: ::core::ffi::c_char = 0x77 as ::core::ffi::c_char;
        let mut aClass: *const ::core::ffi::c_uchar = &raw const initClass
            as *const ::core::ffi::c_uchar;
        if zOut.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if nIn > 2 as ::core::ffi::c_int {
            match *zIn.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
                103 | 107 => {
                    if *zIn.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'n' as i32
                    {
                        zIn = zIn.offset(1);
                        nIn -= 1;
                    }
                }
                _ => {}
            }
        }
        let mut c2rust_current_block_20: u64;
        i = 0 as ::core::ffi::c_int;
        while i < nIn {
            let mut c: ::core::ffi::c_uchar = *zIn.offset(i as isize);
            if (i + 1 as ::core::ffi::c_int) < nIn {
                if c as ::core::ffi::c_int == 'w' as i32
                    && *zIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 'r' as i32
                {
                    c2rust_current_block_20 = 2968425633554183086;
                } else if c as ::core::ffi::c_int == 'd' as i32
                    && (*zIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 'j' as i32
                        || *zIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == 'g' as i32)
                {
                    c2rust_current_block_20 = 2968425633554183086;
                } else if (i + 2 as ::core::ffi::c_int) < nIn {
                    if c as ::core::ffi::c_int == 't' as i32
                        && *zIn.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == 'c' as i32
                        && *zIn.offset((i + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == 'h' as i32
                    {
                        c2rust_current_block_20 = 2968425633554183086;
                    } else {
                        c2rust_current_block_20 = 6057473163062296781;
                    }
                } else {
                    c2rust_current_block_20 = 6057473163062296781;
                }
            } else {
                c2rust_current_block_20 = 6057473163062296781;
            }
            match c2rust_current_block_20 {
                6057473163062296781 => {
                    c = *aClass
                        .offset(
                            (c as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                                as isize,
                        );
                    if !(c as ::core::ffi::c_int == CCLASS_SPACE) {
                        if !(c as ::core::ffi::c_int == CCLASS_OTHER
                            && cPrev as ::core::ffi::c_int != CCLASS_DIGIT)
                        {
                            aClass = &raw const midClass as *const ::core::ffi::c_uchar;
                            if !(c as ::core::ffi::c_int == CCLASS_VOWEL
                                && (cPrevX as ::core::ffi::c_int == CCLASS_R
                                    || cPrevX as ::core::ffi::c_int == CCLASS_L))
                            {
                                if (c as ::core::ffi::c_int == CCLASS_R
                                    || c as ::core::ffi::c_int == CCLASS_L)
                                    && cPrevX as ::core::ffi::c_int == CCLASS_VOWEL
                                {
                                    nOut -= 1;
                                }
                                cPrev = c as ::core::ffi::c_char;
                                if !(c as ::core::ffi::c_int == CCLASS_SILENT) {
                                    cPrevX = c as ::core::ffi::c_char;
                                    c = className[c as usize];
                                    if nOut == 0 as ::core::ffi::c_int
                                        || c as ::core::ffi::c_int
                                            != *zOut.offset((nOut - 1 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_int
                                    {
                                        let c2rust_fresh0 = nOut;
                                        nOut = nOut + 1;
                                        *zOut.offset(c2rust_fresh0 as isize) = c;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            i += 1;
        }
        *zOut.offset(nOut as isize) = 0 as ::core::ffi::c_uchar;
        return zOut;
    }
}
unsafe extern "C" fn phoneticHashSqlFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zIn: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zOut: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        zIn = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        if zIn.is_null() {
            return;
        }
        zOut = phoneticHash(
            zIn,
            sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize)),
        );
        if zOut.is_null() {
            sqlite3_result_error_nomem(context);
        } else {
            sqlite3_result_text(
                context,
                zOut as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        };
    }
}
unsafe extern "C" fn characterClass(
    mut cPrev: ::core::ffi::c_char,
    mut c: ::core::ffi::c_char,
) -> ::core::ffi::c_char {
    unsafe {
        return (if cPrev as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            initClass[(c as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
        } else {
            midClass[(c as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn insertOrDeleteCost(
    mut cPrev: ::core::ffi::c_char,
    mut c: ::core::ffi::c_char,
    mut cNext: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut classC: ::core::ffi::c_char = characterClass(cPrev, c);
        let mut classCprev: ::core::ffi::c_char = 0;
        if classC as ::core::ffi::c_int == CCLASS_SILENT {
            return 1 as ::core::ffi::c_int;
        }
        if cPrev as ::core::ffi::c_int == c as ::core::ffi::c_int {
            return 10 as ::core::ffi::c_int;
        }
        if classC as ::core::ffi::c_int == CCLASS_VOWEL
            && (cPrev as ::core::ffi::c_int == 'r' as i32
                || cNext as ::core::ffi::c_int == 'r' as i32)
        {
            return 20 as ::core::ffi::c_int;
        }
        classCprev = characterClass(cPrev, cPrev);
        if classC as ::core::ffi::c_int == classCprev as ::core::ffi::c_int {
            if classC as ::core::ffi::c_int == CCLASS_VOWEL {
                return 15 as ::core::ffi::c_int
            } else {
                return 50 as ::core::ffi::c_int
            }
        }
        return 100 as ::core::ffi::c_int;
    }
}
pub const FINAL_INS_COST_DIV: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn substituteCost(
    mut cPrev: ::core::ffi::c_char,
    mut cFrom: ::core::ffi::c_char,
    mut cTo: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut classFrom: ::core::ffi::c_char = 0;
        let mut classTo: ::core::ffi::c_char = 0;
        if cFrom as ::core::ffi::c_int == cTo as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if cFrom as ::core::ffi::c_int
            == cTo as ::core::ffi::c_int ^ 0x20 as ::core::ffi::c_int
            && (cTo as ::core::ffi::c_int >= 'A' as i32
                && cTo as ::core::ffi::c_int <= 'Z' as i32
                || cTo as ::core::ffi::c_int >= 'a' as i32
                    && cTo as ::core::ffi::c_int <= 'z' as i32)
        {
            return 0 as ::core::ffi::c_int;
        }
        classFrom = characterClass(cPrev, cFrom);
        classTo = characterClass(cPrev, cTo);
        if classFrom as ::core::ffi::c_int == classTo as ::core::ffi::c_int {
            return 40 as ::core::ffi::c_int;
        }
        if classFrom as ::core::ffi::c_int >= CCLASS_B
            && classFrom as ::core::ffi::c_int <= CCLASS_Y
            && classTo as ::core::ffi::c_int >= CCLASS_B
            && classTo as ::core::ffi::c_int <= CCLASS_Y
        {
            return 75 as ::core::ffi::c_int;
        }
        return 100 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn editdist1(
    mut zA: *const ::core::ffi::c_char,
    mut zB: *const ::core::ffi::c_char,
    mut pnMatch: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nA: ::core::ffi::c_uint = 0;
        let mut nB: ::core::ffi::c_uint = 0;
        let mut xA: ::core::ffi::c_uint = 0;
        let mut xB: ::core::ffi::c_uint = 0;
        let mut cA: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
        let mut cB: ::core::ffi::c_char = 0;
        let mut cAprev: ::core::ffi::c_char = 0;
        let mut cBprev: ::core::ffi::c_char = 0;
        let mut cAnext: ::core::ffi::c_char = 0;
        let mut cBnext: ::core::ffi::c_char = 0;
        let mut d: ::core::ffi::c_int = 0;
        let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut res: ::core::ffi::c_int = 0;
        let mut m: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut cx: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut toFree: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut nMatch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mStack: [::core::ffi::c_int; 75] = [0; 75];
        if zA.is_null() || zB.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        while *zA.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && *zA.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == *zB.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            dc = *zA.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            zA = zA.offset(1);
            zB = zB.offset(1);
            nMatch += 1;
        }
        if !pnMatch.is_null() {
            *pnMatch = nMatch;
        }
        if *zA.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && *zB.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        nA = 0 as ::core::ffi::c_uint;
        while *zA.offset(nA as isize) != 0 {
            if *zA.offset(nA as isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                != 0
            {
                return -2 as ::core::ffi::c_int;
            }
            nA = nA.wrapping_add(1);
        }
        nB = 0 as ::core::ffi::c_uint;
        while *zB.offset(nB as isize) != 0 {
            if *zB.offset(nB as isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                != 0
            {
                return -2 as ::core::ffi::c_int;
            }
            nB = nB.wrapping_add(1);
        }
        if nA == 0 as ::core::ffi::c_uint {
            cBprev = dc as ::core::ffi::c_char;
            res = 0 as ::core::ffi::c_int;
            xB = res as ::core::ffi::c_uint;
            loop {
                cB = *zB.offset(xB as isize);
                if !(cB as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                    break;
                }
                res
                    += insertOrDeleteCost(
                        cBprev,
                        cB,
                        *zB.offset(xB.wrapping_add(1 as ::core::ffi::c_uint) as isize),
                    ) / FINAL_INS_COST_DIV;
                cBprev = cB;
                xB = xB.wrapping_add(1);
            }
            return res;
        }
        if nB == 0 as ::core::ffi::c_uint {
            cAprev = dc as ::core::ffi::c_char;
            res = 0 as ::core::ffi::c_int;
            xA = res as ::core::ffi::c_uint;
            loop {
                cA = *zA.offset(xA as isize);
                if !(cA as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                    break;
                }
                res
                    += insertOrDeleteCost(
                        cAprev,
                        cA,
                        *zA.offset(xA.wrapping_add(1 as ::core::ffi::c_uint) as isize),
                    );
                cAprev = cA;
                xA = xA.wrapping_add(1);
            }
            return res;
        }
        if *zA.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '*' as i32
            && *zA.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if (nB as usize)
            < (::core::mem::size_of::<[::core::ffi::c_int; 75]>() as usize)
                .wrapping_mul(4 as usize)
                .wrapping_div(
                    (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        .wrapping_mul(5 as usize),
                )
        {
            m = &raw mut mStack as *mut ::core::ffi::c_int;
        } else {
            toFree = sqlite3_malloc64(
                (nB
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(5 as ::core::ffi::c_uint) as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    .wrapping_div(4 as usize) as sqlite3_uint64,
            ) as *mut ::core::ffi::c_int;
            m = toFree;
            if m.is_null() {
                return -3 as ::core::ffi::c_int;
            }
        }
        cx = m.offset(nB.wrapping_add(1 as ::core::ffi::c_uint) as isize)
            as *mut ::core::ffi::c_int as *mut ::core::ffi::c_char;
        *m.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
        *cx.offset(0 as ::core::ffi::c_int as isize) = dc as ::core::ffi::c_char;
        cBprev = dc as ::core::ffi::c_char;
        xB = 1 as ::core::ffi::c_uint;
        while xB <= nB {
            cBnext = *zB.offset(xB as isize);
            cB = *zB.offset(xB.wrapping_sub(1 as ::core::ffi::c_uint) as isize);
            *cx.offset(xB as isize) = cB;
            *m.offset(xB as isize) = *m
                .offset(xB.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
                + insertOrDeleteCost(cBprev, cB, cBnext);
            cBprev = cB;
            xB = xB.wrapping_add(1);
        }
        cAprev = dc as ::core::ffi::c_char;
        xA = 1 as ::core::ffi::c_uint;
        while xA <= nA {
            let mut lastA: ::core::ffi::c_int = (xA == nA) as ::core::ffi::c_int;
            cA = *zA.offset(xA.wrapping_sub(1 as ::core::ffi::c_uint) as isize);
            cAnext = *zA.offset(xA as isize);
            if cA as ::core::ffi::c_int == '*' as i32 && lastA != 0 {
                break;
            }
            d = *m.offset(0 as ::core::ffi::c_int as isize);
            dc = *cx.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            *m.offset(0 as ::core::ffi::c_int as isize) = d
                + insertOrDeleteCost(cAprev, cA, cAnext);
            cBprev = 0 as ::core::ffi::c_char;
            xB = 1 as ::core::ffi::c_uint;
            while xB <= nB {
                let mut totalCost: ::core::ffi::c_int = 0;
                let mut insCost: ::core::ffi::c_int = 0;
                let mut delCost: ::core::ffi::c_int = 0;
                let mut subCost: ::core::ffi::c_int = 0;
                let mut ncx: ::core::ffi::c_int = 0;
                cB = *zB.offset(xB.wrapping_sub(1 as ::core::ffi::c_uint) as isize);
                cBnext = *zB.offset(xB as isize);
                insCost = insertOrDeleteCost(
                    *cx.offset(xB.wrapping_sub(1 as ::core::ffi::c_uint) as isize),
                    cB,
                    cBnext,
                );
                if lastA != 0 {
                    insCost /= FINAL_INS_COST_DIV;
                }
                delCost = insertOrDeleteCost(*cx.offset(xB as isize), cA, cBnext);
                subCost = substituteCost(
                    *cx.offset(xB.wrapping_sub(1 as ::core::ffi::c_uint) as isize),
                    cA,
                    cB,
                );
                totalCost = insCost
                    + *m.offset(xB.wrapping_sub(1 as ::core::ffi::c_uint) as isize);
                ncx = cB as ::core::ffi::c_int;
                if delCost + *m.offset(xB as isize) < totalCost {
                    totalCost = delCost + *m.offset(xB as isize);
                    ncx = cA as ::core::ffi::c_int;
                }
                if subCost + d < totalCost {
                    totalCost = subCost + d;
                }
                d = *m.offset(xB as isize);
                dc = *cx.offset(xB as isize) as ::core::ffi::c_int;
                *m.offset(xB as isize) = totalCost;
                *cx.offset(xB as isize) = ncx as ::core::ffi::c_char;
                cBprev = cB;
                xB = xB.wrapping_add(1);
            }
            cAprev = cA;
            xA = xA.wrapping_add(1);
        }
        if cA as ::core::ffi::c_int == '*' as i32 {
            res = *m.offset(1 as ::core::ffi::c_int as isize);
            xB = 1 as ::core::ffi::c_uint;
            while xB <= nB {
                if *m.offset(xB as isize) < res {
                    res = *m.offset(xB as isize);
                    if !pnMatch.is_null() {
                        *pnMatch = xB.wrapping_add(nMatch as ::core::ffi::c_uint)
                            as ::core::ffi::c_int;
                    }
                }
                xB = xB.wrapping_add(1);
            }
        } else {
            res = *m.offset(nB as isize);
        }
        sqlite3_free(toFree as *mut ::core::ffi::c_void);
        return res;
    }
}
unsafe extern "C" fn editdistSqlFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut res: ::core::ffi::c_int = editdist1(
            sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char,
            sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if res < 0 as ::core::ffi::c_int {
            if res == -3 as ::core::ffi::c_int {
                sqlite3_result_error_nomem(context);
            } else if res == -2 as ::core::ffi::c_int {
                sqlite3_result_error(
                    context,
                    b"non-ASCII input to editdist()\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
            } else {
                sqlite3_result_error(
                    context,
                    b"NULL input to editdist()\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
            }
        } else {
            sqlite3_result_int(context, res);
        };
    }
}
static mut editDist3Lang: EditDist3Lang = EditDist3Lang {
    iLang: 0 as ::core::ffi::c_int,
    iInsCost: 100 as ::core::ffi::c_int,
    iDelCost: 100 as ::core::ffi::c_int,
    iSubCost: 150 as ::core::ffi::c_int,
    pCost: ::core::ptr::null::<EditDist3Cost>() as *mut EditDist3Cost,
};
unsafe extern "C" fn editDist3ConfigClear(mut p: *mut EditDist3Config) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if p.is_null() {
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nLang {
            let mut pCost: *mut EditDist3Cost = ::core::ptr::null_mut::<EditDist3Cost>();
            let mut pNext: *mut EditDist3Cost = ::core::ptr::null_mut::<EditDist3Cost>();
            pCost = (*(*p).a.offset(i as isize)).pCost;
            while !pCost.is_null() {
                pNext = (*pCost).pNext;
                sqlite3_free(pCost as *mut ::core::ffi::c_void);
                pCost = pNext;
            }
            i += 1;
        }
        sqlite3_free((*p).a as *mut ::core::ffi::c_void);
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<EditDist3Config>() as size_t,
        );
    }
}
unsafe extern "C" fn editDist3ConfigDelete(mut pIn: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut EditDist3Config = pIn as *mut EditDist3Config;
        editDist3ConfigClear(p);
        sqlite3_free(p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn editDist3CostCompare(
    mut pA: *mut EditDist3Cost,
    mut pB: *mut EditDist3Cost,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: ::core::ffi::c_int = (*pA).nFrom as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0;
        if n > (*pB).nFrom as ::core::ffi::c_int {
            n = (*pB).nFrom as ::core::ffi::c_int;
        }
        rc = strncmp(
            &raw mut (*pA).a as *mut ::core::ffi::c_char,
            &raw mut (*pB).a as *mut ::core::ffi::c_char,
            n as size_t,
        );
        if rc == 0 as ::core::ffi::c_int {
            rc = (*pA).nFrom as ::core::ffi::c_int - (*pB).nFrom as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn editDist3CostMerge(
    mut pA: *mut EditDist3Cost,
    mut pB: *mut EditDist3Cost,
) -> *mut EditDist3Cost {
    unsafe {
        let mut pHead: *mut EditDist3Cost = ::core::ptr::null_mut::<EditDist3Cost>();
        let mut ppTail: *mut *mut EditDist3Cost = &raw mut pHead;
        let mut p: *mut EditDist3Cost = ::core::ptr::null_mut::<EditDist3Cost>();
        while !pA.is_null() && !pB.is_null() {
            if editDist3CostCompare(pA, pB) <= 0 as ::core::ffi::c_int {
                p = pA;
                pA = (*pA).pNext;
            } else {
                p = pB;
                pB = (*pB).pNext;
            }
            *ppTail = p;
            ppTail = &raw mut (*p).pNext;
        }
        if !pA.is_null() {
            *ppTail = pA;
        } else {
            *ppTail = pB;
        }
        return pHead;
    }
}
unsafe extern "C" fn editDist3CostSort(
    mut pList: *mut EditDist3Cost,
) -> *mut EditDist3Cost {
    unsafe {
        let mut ap: [*mut EditDist3Cost; 60] = [::core::ptr::null_mut::<
            EditDist3Cost,
        >(); 60];
        let mut p: *mut EditDist3Cost = ::core::ptr::null_mut::<EditDist3Cost>();
        let mut i: ::core::ffi::c_int = 0;
        let mut mx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        ap[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<EditDist3Cost>();
        ap[1 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<EditDist3Cost>();
        while !pList.is_null() {
            p = pList;
            pList = (*p).pNext;
            (*p).pNext = ::core::ptr::null_mut::<EditDist3Cost>();
            i = 0 as ::core::ffi::c_int;
            while !ap[i as usize].is_null() {
                p = editDist3CostMerge(ap[i as usize], p);
                ap[i as usize] = ::core::ptr::null_mut::<EditDist3Cost>();
                i += 1;
            }
            ap[i as usize] = p;
            if i > mx {
                mx = i;
                ap[(i + 1 as ::core::ffi::c_int) as usize] = ::core::ptr::null_mut::<
                    EditDist3Cost,
                >();
            }
        }
        p = ::core::ptr::null_mut::<EditDist3Cost>();
        i = 0 as ::core::ffi::c_int;
        while i <= mx {
            if !ap[i as usize].is_null() {
                p = editDist3CostMerge(p, ap[i as usize]);
            }
            i += 1;
        }
        return p;
    }
}
unsafe extern "C" fn editDist3ConfigLoad(
    mut p: *mut EditDist3Config,
    mut db: *mut sqlite3,
    mut zTable: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut rc2: ::core::ffi::c_int = 0;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut iLangPrev: ::core::ffi::c_int = -9999 as ::core::ffi::c_int;
        let mut pLang: *mut EditDist3Lang = ::core::ptr::null_mut::<EditDist3Lang>();
        zSql = sqlite3_mprintf(
            b"SELECT iLang, cFrom, cTo, iCost FROM \"%w\" WHERE iLang>=0 ORDER BY iLang\0"
                .as_ptr() as *const ::core::ffi::c_char,
            zTable,
        );
        if zSql.is_null() {
            return SQLITE_NOMEM;
        }
        rc = sqlite3_prepare(
            db,
            zSql,
            -1 as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        if rc != 0 {
            return rc;
        }
        editDist3ConfigClear(p);
        while sqlite3_step(pStmt) == SQLITE_ROW {
            let mut iLang: ::core::ffi::c_int = sqlite3_column_int(
                pStmt,
                0 as ::core::ffi::c_int,
            );
            let mut zFrom: *const ::core::ffi::c_char = sqlite3_column_text(
                pStmt,
                1 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            let mut nFrom: ::core::ffi::c_int = if !zFrom.is_null() {
                sqlite3_column_bytes(pStmt, 1 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            };
            let mut zTo: *const ::core::ffi::c_char = sqlite3_column_text(
                pStmt,
                2 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            let mut nTo: ::core::ffi::c_int = if !zTo.is_null() {
                sqlite3_column_bytes(pStmt, 2 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            };
            let mut iCost: ::core::ffi::c_int = sqlite3_column_int(
                pStmt,
                3 as ::core::ffi::c_int,
            );
            if nFrom > 100 as ::core::ffi::c_int || nTo > 100 as ::core::ffi::c_int {
                continue;
            }
            if iCost < 0 as ::core::ffi::c_int {
                continue;
            }
            if iCost >= 10000 as ::core::ffi::c_int {
                continue;
            }
            if pLang.is_null() || iLang != iLangPrev {
                let mut pNew: *mut EditDist3Lang = ::core::ptr::null_mut::<
                    EditDist3Lang,
                >();
                pNew = sqlite3_realloc64(
                    (*p).a as *mut ::core::ffi::c_void,
                    (((*p).nLang + 1 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<EditDist3Lang>() as usize)
                        as sqlite3_uint64,
                ) as *mut EditDist3Lang;
                if pNew.is_null() {
                    rc = SQLITE_NOMEM;
                    break;
                } else {
                    (*p).a = pNew;
                    pLang = (*p).a.offset((*p).nLang as isize) as *mut EditDist3Lang;
                    (*p).nLang += 1;
                    (*pLang).iLang = iLang;
                    (*pLang).iInsCost = 100 as ::core::ffi::c_int;
                    (*pLang).iDelCost = 100 as ::core::ffi::c_int;
                    (*pLang).iSubCost = 150 as ::core::ffi::c_int;
                    (*pLang).pCost = ::core::ptr::null_mut::<EditDist3Cost>();
                    iLangPrev = iLang;
                }
            }
            if nFrom == 1 as ::core::ffi::c_int
                && *zFrom.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '?' as i32 && nTo == 0 as ::core::ffi::c_int
            {
                (*pLang).iDelCost = iCost;
            } else if nFrom == 0 as ::core::ffi::c_int && nTo == 1 as ::core::ffi::c_int
                && *zTo.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '?' as i32
            {
                (*pLang).iInsCost = iCost;
            } else if nFrom == 1 as ::core::ffi::c_int && nTo == 1 as ::core::ffi::c_int
                && *zFrom.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '?' as i32
                && *zTo.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '?' as i32
            {
                (*pLang).iSubCost = iCost;
            } else {
                let mut pCost: *mut EditDist3Cost = ::core::ptr::null_mut::<
                    EditDist3Cost,
                >();
                let mut nExtra: ::core::ffi::c_int = nFrom + nTo
                    - 4 as ::core::ffi::c_int;
                if nExtra < 0 as ::core::ffi::c_int {
                    nExtra = 0 as ::core::ffi::c_int;
                }
                pCost = sqlite3_malloc64(
                    (::core::mem::size_of::<EditDist3Cost>() as usize)
                        .wrapping_add(nExtra as usize) as sqlite3_uint64,
                ) as *mut EditDist3Cost;
                if pCost.is_null() {
                    rc = SQLITE_NOMEM;
                    break;
                } else {
                    (*pCost).nFrom = nFrom as u8_0;
                    (*pCost).nTo = nTo as u8_0;
                    (*pCost).iCost = iCost as u16_0;
                    memcpy(
                        &raw mut (*pCost).a as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        zFrom as *const ::core::ffi::c_void,
                        nFrom as size_t,
                    );
                    memcpy(
                        (&raw mut (*pCost).a as *mut ::core::ffi::c_char)
                            .offset(nFrom as isize) as *mut ::core::ffi::c_void,
                        zTo as *const ::core::ffi::c_void,
                        nTo as size_t,
                    );
                    (*pCost).pNext = (*pLang).pCost;
                    (*pLang).pCost = pCost;
                }
            }
        }
        rc2 = sqlite3_finalize(pStmt);
        if rc == SQLITE_OK {
            rc = rc2;
        }
        if rc == SQLITE_OK {
            let mut iLang_0: ::core::ffi::c_int = 0;
            iLang_0 = 0 as ::core::ffi::c_int;
            while iLang_0 < (*p).nLang {
                let ref mut c2rust_fresh1 = (*(*p).a.offset(iLang_0 as isize)).pCost;
                *c2rust_fresh1 = editDist3CostSort(
                    (*(*p).a.offset(iLang_0 as isize)).pCost,
                );
                iLang_0 += 1;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn utf8Len(
    mut c: ::core::ffi::c_uchar,
    mut N: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut len: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if c as ::core::ffi::c_int > 0x7f as ::core::ffi::c_int {
            if c as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                len = 2 as ::core::ffi::c_int;
            } else if c as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                == 0xe0 as ::core::ffi::c_int
            {
                len = 3 as ::core::ffi::c_int;
            } else {
                len = 4 as ::core::ffi::c_int;
            }
        }
        if len > N {
            len = N;
        }
        return len;
    }
}
unsafe extern "C" fn matchTo(
    mut p: *mut EditDist3Cost,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).a[(*p).nFrom as usize] as ::core::ffi::c_int
            != *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*p).nTo as ::core::ffi::c_int > n {
            return 0 as ::core::ffi::c_int;
        }
        if strncmp(
            (&raw mut (*p).a as *mut ::core::ffi::c_char)
                .offset((*p).nFrom as ::core::ffi::c_int as isize),
            z,
            (*p).nTo as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn matchFrom(
    mut p: *mut EditDist3Cost,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).nFrom != 0 {
            if (*p).a[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                != *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            if strncmp(
                &raw mut (*p).a as *mut ::core::ffi::c_char,
                z,
                (*p).nFrom as size_t,
            ) != 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn matchFromTo(
    mut pStr: *mut EditDist3FromString,
    mut n1: ::core::ffi::c_int,
    mut z2: *const ::core::ffi::c_char,
    mut n2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut b1: ::core::ffi::c_int = (*(*pStr).a.offset(n1 as isize)).nByte;
        if b1 > n2 {
            return 0 as ::core::ffi::c_int;
        }
        if *(*pStr).z.offset(n1 as isize) as ::core::ffi::c_int
            != *z2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if strncmp((*pStr).z.offset(n1 as isize), z2, b1 as size_t)
            != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn editDist3FromStringDelete(mut p: *mut EditDist3FromString) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if !p.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*p).n {
                sqlite3_free(
                    (*(*p).a.offset(i as isize)).apDel as *mut ::core::ffi::c_void,
                );
                sqlite3_free(
                    (*(*p).a.offset(i as isize)).apSubst as *mut ::core::ffi::c_void,
                );
                i += 1;
            }
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn editDist3FromStringNew(
    mut pLang: *const EditDist3Lang,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> *mut EditDist3FromString {
    unsafe {
        let mut pStr: *mut EditDist3FromString = ::core::ptr::null_mut::<
            EditDist3FromString,
        >();
        let mut p: *mut EditDist3Cost = ::core::ptr::null_mut::<EditDist3Cost>();
        let mut i: ::core::ffi::c_int = 0;
        if z.is_null() {
            return ::core::ptr::null_mut::<EditDist3FromString>();
        }
        if n < 0 as ::core::ffi::c_int {
            n = strlen(z) as ::core::ffi::c_int;
        }
        pStr = sqlite3_malloc64(
            (::core::mem::size_of::<EditDist3FromString>() as usize)
                .wrapping_add(
                    (::core::mem::size_of::<EditDist3From>() as usize)
                        .wrapping_mul(n as usize),
                )
                .wrapping_add(n as usize)
                .wrapping_add(1 as usize) as sqlite3_uint64,
        ) as *mut EditDist3FromString;
        if pStr.is_null() {
            return ::core::ptr::null_mut::<EditDist3FromString>();
        }
        (*pStr).a = pStr.offset(1 as ::core::ffi::c_int as isize)
            as *mut EditDist3FromString as *mut EditDist3From;
        memset(
            (*pStr).a as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<EditDist3From>() as size_t).wrapping_mul(n as size_t),
        );
        (*pStr).n = n;
        (*pStr).z = (*pStr).a.offset(n as isize) as *mut EditDist3From
            as *mut ::core::ffi::c_char;
        memcpy(
            (*pStr).z as *mut ::core::ffi::c_void,
            z as *const ::core::ffi::c_void,
            (n + 1 as ::core::ffi::c_int) as size_t,
        );
        if n != 0
            && *z.offset((n - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == '*' as i32
        {
            (*pStr).isPrefix = 1 as ::core::ffi::c_int;
            n -= 1;
            (*pStr).n -= 1;
            *(*pStr).z.offset(n as isize) = 0 as ::core::ffi::c_char;
        } else {
            (*pStr).isPrefix = 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < n {
            let mut pFrom: *mut EditDist3From = (*pStr).a.offset(i as isize)
                as *mut EditDist3From;
            memset(
                pFrom as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<EditDist3From>() as size_t,
            );
            (*pFrom).nByte = utf8Len(
                *z.offset(i as isize) as ::core::ffi::c_uchar,
                n - i,
            );
            p = (*pLang).pCost;
            while !p.is_null() {
                let mut apNew: *mut *mut EditDist3Cost = ::core::ptr::null_mut::<
                    *mut EditDist3Cost,
                >();
                if !(i + (*p).nFrom as ::core::ffi::c_int > n) {
                    if !(matchFrom(p, z.offset(i as isize), n - i)
                        == 0 as ::core::ffi::c_int)
                    {
                        if (*p).nTo as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            apNew = sqlite3_realloc64(
                                (*pFrom).apDel as *mut ::core::ffi::c_void,
                                (::core::mem::size_of::<*mut EditDist3Cost>() as usize)
                                    .wrapping_mul(
                                        ((*pFrom).nDel + 1 as ::core::ffi::c_int) as usize,
                                    ) as sqlite3_uint64,
                            ) as *mut *mut EditDist3Cost;
                            if apNew.is_null() {
                                break;
                            }
                            (*pFrom).apDel = apNew;
                            let c2rust_fresh2 = (*pFrom).nDel;
                            (*pFrom).nDel = (*pFrom).nDel + 1;
                            let ref mut c2rust_fresh3 = *apNew
                                .offset(c2rust_fresh2 as isize);
                            *c2rust_fresh3 = p;
                        } else {
                            apNew = sqlite3_realloc64(
                                (*pFrom).apSubst as *mut ::core::ffi::c_void,
                                (::core::mem::size_of::<*mut EditDist3Cost>() as usize)
                                    .wrapping_mul(
                                        ((*pFrom).nSubst + 1 as ::core::ffi::c_int) as usize,
                                    ) as sqlite3_uint64,
                            ) as *mut *mut EditDist3Cost;
                            if apNew.is_null() {
                                break;
                            }
                            (*pFrom).apSubst = apNew;
                            let c2rust_fresh4 = (*pFrom).nSubst;
                            (*pFrom).nSubst = (*pFrom).nSubst + 1;
                            let ref mut c2rust_fresh5 = *apNew
                                .offset(c2rust_fresh4 as isize);
                            *c2rust_fresh5 = p;
                        }
                    }
                }
                p = (*p).pNext;
            }
            if !p.is_null() {
                editDist3FromStringDelete(pStr);
                pStr = ::core::ptr::null_mut::<EditDist3FromString>();
                break;
            } else {
                i += 1;
            }
        }
        return pStr;
    }
}
unsafe extern "C" fn updateCost(
    mut m: *mut ::core::ffi::c_uint,
    mut i: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
    mut iCost: ::core::ffi::c_int,
) {
    unsafe {
        let mut b: ::core::ffi::c_uint = 0;
        b = (*m.offset(j as isize)).wrapping_add(iCost as ::core::ffi::c_uint);
        if b < *m.offset(i as isize) {
            *m.offset(i as isize) = b;
        }
    }
}
unsafe extern "C" fn editDist3Core(
    mut pFrom: *mut EditDist3FromString,
    mut z2: *const ::core::ffi::c_char,
    mut n2: ::core::ffi::c_int,
    mut pLang: *const EditDist3Lang,
    mut pnMatch: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut k: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut i1: ::core::ffi::c_int = 0;
        let mut b1: ::core::ffi::c_int = 0;
        let mut i2: ::core::ffi::c_int = 0;
        let mut b2: ::core::ffi::c_int = 0;
        let mut f: EditDist3FromString = *pFrom;
        let mut a2: *mut EditDist3To = ::core::ptr::null_mut::<EditDist3To>();
        let mut m: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<
            ::core::ffi::c_uint,
        >();
        let mut pToFree: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<
            ::core::ffi::c_uint,
        >();
        let mut szRow: ::core::ffi::c_int = 0;
        let mut p: *mut EditDist3Cost = ::core::ptr::null_mut::<EditDist3Cost>();
        let mut res: ::core::ffi::c_int = 0;
        let mut nByte: sqlite3_uint64 = 0;
        let mut stackSpace: [::core::ffi::c_uint; 256] = [0; 256];
        n = (f.n + 1 as ::core::ffi::c_int) * (n2 + 1 as ::core::ffi::c_int);
        n = n + 1 as ::core::ffi::c_int & !(1 as ::core::ffi::c_int);
        nByte = (n as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as usize)
            .wrapping_add(
                (::core::mem::size_of::<EditDist3To>() as usize)
                    .wrapping_mul(n2 as usize),
            ) as sqlite3_uint64;
        if nByte
            <= ::core::mem::size_of::<[::core::ffi::c_uint; 256]>()
                as ::core::ffi::c_ulonglong
        {
            m = &raw mut stackSpace as *mut ::core::ffi::c_uint;
            pToFree = ::core::ptr::null_mut::<::core::ffi::c_uint>();
        } else {
            pToFree = sqlite3_malloc64(nByte) as *mut ::core::ffi::c_uint;
            m = pToFree;
            if m.is_null() {
                return -1 as ::core::ffi::c_int;
            }
        }
        a2 = m.offset(n as isize) as *mut ::core::ffi::c_uint as *mut EditDist3To;
        memset(
            a2 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<EditDist3To>() as size_t).wrapping_mul(n2 as size_t),
        );
        i2 = 0 as ::core::ffi::c_int;
        's_71: loop {
            if !(i2 < n2) {
                c2rust_current_block = 14648156034262866959;
                break;
            }
            (*a2.offset(i2 as isize)).nByte = utf8Len(
                *z2.offset(i2 as isize) as ::core::ffi::c_uchar,
                n2 - i2,
            );
            p = (*pLang).pCost;
            while !p.is_null() {
                let mut apNew: *mut *mut EditDist3Cost = ::core::ptr::null_mut::<
                    *mut EditDist3Cost,
                >();
                if (*p).nFrom as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    break;
                }
                if !(i2 + (*p).nTo as ::core::ffi::c_int > n2) {
                    if (*p).a[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        > *z2.offset(i2 as isize) as ::core::ffi::c_int
                    {
                        break;
                    }
                    if !(matchTo(p, z2.offset(i2 as isize), n2 - i2)
                        == 0 as ::core::ffi::c_int)
                    {
                        let ref mut c2rust_fresh6 = (*a2.offset(i2 as isize)).nIns;
                        *c2rust_fresh6 += 1;
                        apNew = sqlite3_realloc64(
                            (*a2.offset(i2 as isize)).apIns as *mut ::core::ffi::c_void,
                            (::core::mem::size_of::<*mut EditDist3Cost>() as usize)
                                .wrapping_mul((*a2.offset(i2 as isize)).nIns as usize)
                                as sqlite3_uint64,
                        ) as *mut *mut EditDist3Cost;
                        if apNew.is_null() {
                            res = -1 as ::core::ffi::c_int;
                            c2rust_current_block = 10659019050092516261;
                            break 's_71;
                        } else {
                            let ref mut c2rust_fresh7 = (*a2.offset(i2 as isize)).apIns;
                            *c2rust_fresh7 = apNew;
                            let ref mut c2rust_fresh8 = *(*a2.offset(i2 as isize))
                                .apIns
                                .offset(
                                    ((*a2.offset(i2 as isize)).nIns - 1 as ::core::ffi::c_int)
                                        as isize,
                                );
                            *c2rust_fresh8 = p;
                        }
                    }
                }
                p = (*p).pNext;
            }
            i2 += 1;
        }
        match c2rust_current_block {
            14648156034262866959 => {
                szRow = f.n + 1 as ::core::ffi::c_int;
                memset(
                    m as *mut ::core::ffi::c_void,
                    0x1 as ::core::ffi::c_int,
                    (((n2 + 1 as ::core::ffi::c_int) * szRow) as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<::core::ffi::c_uint>() as size_t,
                        ),
                );
                *m.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_uint;
                i1 = 0 as ::core::ffi::c_int;
                while i1 < f.n {
                    b1 = (*f.a.offset(i1 as isize)).nByte;
                    updateCost(m, i1 + b1, i1, (*pLang).iDelCost);
                    k = 0 as ::core::ffi::c_int;
                    while k < (*f.a.offset(i1 as isize)).nDel {
                        p = *(*f.a.offset(i1 as isize)).apDel.offset(k as isize);
                        updateCost(
                            m,
                            i1 + (*p).nFrom as ::core::ffi::c_int,
                            i1,
                            (*p).iCost as ::core::ffi::c_int,
                        );
                        k += 1;
                    }
                    i1 += b1;
                }
                i2 = 0 as ::core::ffi::c_int;
                while i2 < n2 {
                    let mut rx: ::core::ffi::c_int = 0;
                    let mut rxp: ::core::ffi::c_int = 0;
                    b2 = (*a2.offset(i2 as isize)).nByte;
                    rx = szRow * (i2 + b2);
                    rxp = szRow * i2;
                    updateCost(m, rx, rxp, (*pLang).iInsCost);
                    k = 0 as ::core::ffi::c_int;
                    while k < (*a2.offset(i2 as isize)).nIns {
                        p = *(*a2.offset(i2 as isize)).apIns.offset(k as isize);
                        updateCost(
                            m,
                            szRow * (i2 + (*p).nTo as ::core::ffi::c_int),
                            rxp,
                            (*p).iCost as ::core::ffi::c_int,
                        );
                        k += 1;
                    }
                    i1 = 0 as ::core::ffi::c_int;
                    while i1 < f.n {
                        let mut cx: ::core::ffi::c_int = 0;
                        let mut cxp: ::core::ffi::c_int = 0;
                        let mut cxd: ::core::ffi::c_int = 0;
                        let mut cxu: ::core::ffi::c_int = 0;
                        b1 = (*f.a.offset(i1 as isize)).nByte;
                        cxp = rx + i1;
                        cx = cxp + b1;
                        cxd = rxp + i1;
                        cxu = cxd + b1;
                        updateCost(m, cx, cxp, (*pLang).iDelCost);
                        k = 0 as ::core::ffi::c_int;
                        while k < (*f.a.offset(i1 as isize)).nDel {
                            p = *(*f.a.offset(i1 as isize)).apDel.offset(k as isize);
                            updateCost(
                                m,
                                cxp + (*p).nFrom as ::core::ffi::c_int,
                                cxp,
                                (*p).iCost as ::core::ffi::c_int,
                            );
                            k += 1;
                        }
                        updateCost(m, cx, cxu, (*pLang).iInsCost);
                        if matchFromTo(&raw mut f, i1, z2.offset(i2 as isize), n2 - i2)
                            != 0
                        {
                            updateCost(m, cx, cxd, 0 as ::core::ffi::c_int);
                        }
                        updateCost(m, cx, cxd, (*pLang).iSubCost);
                        k = 0 as ::core::ffi::c_int;
                        while k < (*f.a.offset(i1 as isize)).nSubst {
                            p = *(*f.a.offset(i1 as isize)).apSubst.offset(k as isize);
                            if matchTo(p, z2.offset(i2 as isize), n2 - i2) != 0 {
                                updateCost(
                                    m,
                                    cxd + (*p).nFrom as ::core::ffi::c_int
                                        + szRow * (*p).nTo as ::core::ffi::c_int,
                                    cxd,
                                    (*p).iCost as ::core::ffi::c_int,
                                );
                            }
                            k += 1;
                        }
                        i1 += b1;
                    }
                    i2 += b2;
                }
                res = *m
                    .offset(
                        (szRow * (n2 + 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
                n = n2;
                if f.isPrefix != 0 {
                    i2 = 1 as ::core::ffi::c_int;
                    while i2 <= n2 {
                        let mut b: ::core::ffi::c_int = *m
                            .offset((szRow * i2 - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int;
                        if b <= res {
                            res = b;
                            n = i2 - 1 as ::core::ffi::c_int;
                        }
                        i2 += 1;
                    }
                }
                if !pnMatch.is_null() {
                    let mut nExtra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    k = 0 as ::core::ffi::c_int;
                    while k < n {
                        if *z2.offset(k as isize) as ::core::ffi::c_int
                            & 0xc0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int
                        {
                            nExtra += 1;
                        }
                        k += 1;
                    }
                    *pnMatch = n - nExtra;
                }
            }
            _ => {}
        }
        i2 = 0 as ::core::ffi::c_int;
        while i2 < n2 {
            sqlite3_free((*a2.offset(i2 as isize)).apIns as *mut ::core::ffi::c_void);
            i2 += 1;
        }
        sqlite3_free(pToFree as *mut ::core::ffi::c_void);
        return res;
    }
}
unsafe extern "C" fn editDist3FindLang(
    mut pConfig: *mut EditDist3Config,
    mut iLang: ::core::ffi::c_int,
) -> *const EditDist3Lang {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pConfig).nLang {
            if (*(*pConfig).a.offset(i as isize)).iLang == iLang {
                return (*pConfig).a.offset(i as isize) as *mut EditDist3Lang;
            }
            i += 1;
        }
        return &raw const editDist3Lang;
    }
}
unsafe extern "C" fn editDist3SqlFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pConfig: *mut EditDist3Config = sqlite3_user_data(context)
            as *mut EditDist3Config;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut rc: ::core::ffi::c_int = 0;
        if argc == 1 as ::core::ffi::c_int {
            let mut zTable: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            ) as *const ::core::ffi::c_char;
            rc = editDist3ConfigLoad(pConfig, db, zTable);
            if rc != 0 {
                sqlite3_result_error_code(context, rc);
            }
        } else {
            let mut zA: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            ) as *const ::core::ffi::c_char;
            let mut zB: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(1 as ::core::ffi::c_int as isize),
            ) as *const ::core::ffi::c_char;
            let mut nA: ::core::ffi::c_int = sqlite3_value_bytes(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            let mut nB: ::core::ffi::c_int = sqlite3_value_bytes(
                *argv.offset(1 as ::core::ffi::c_int as isize),
            );
            let mut iLang: ::core::ffi::c_int = if argc == 3 as ::core::ffi::c_int {
                sqlite3_value_int(*argv.offset(2 as ::core::ffi::c_int as isize))
            } else {
                0 as ::core::ffi::c_int
            };
            let mut pLang: *const EditDist3Lang = editDist3FindLang(pConfig, iLang);
            let mut pFrom: *mut EditDist3FromString = ::core::ptr::null_mut::<
                EditDist3FromString,
            >();
            let mut dist: ::core::ffi::c_int = 0;
            pFrom = editDist3FromStringNew(pLang, zA, nA);
            if pFrom.is_null() {
                sqlite3_result_error_nomem(context);
                return;
            }
            dist = editDist3Core(
                pFrom,
                zB,
                nB,
                pLang,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            editDist3FromStringDelete(pFrom);
            if dist == -1 as ::core::ffi::c_int {
                sqlite3_result_error_nomem(context);
            } else {
                sqlite3_result_int(context, dist);
            }
        };
    }
}
unsafe extern "C" fn editDist3Install(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pConfig: *mut EditDist3Config = sqlite3_malloc64(
            ::core::mem::size_of::<EditDist3Config>() as sqlite3_uint64,
        ) as *mut EditDist3Config;
        if pConfig.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pConfig as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<EditDist3Config>() as size_t,
        );
        rc = sqlite3_create_function_v2(
            db,
            b"editdist3\0".as_ptr() as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
            SQLITE_UTF8 | SQLITE_DETERMINISTIC,
            pConfig as *mut ::core::ffi::c_void,
            Some(
                editDist3SqlFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
            None,
        );
        if rc == SQLITE_OK {
            rc = sqlite3_create_function_v2(
                db,
                b"editdist3\0".as_ptr() as *const ::core::ffi::c_char,
                3 as ::core::ffi::c_int,
                SQLITE_UTF8 | SQLITE_DETERMINISTIC,
                pConfig as *mut ::core::ffi::c_void,
                Some(
                    editDist3SqlFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                None,
                None,
                None,
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3_create_function_v2(
                db,
                b"editdist3\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8 | SQLITE_DETERMINISTIC,
                pConfig as *mut ::core::ffi::c_void,
                Some(
                    editDist3SqlFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                None,
                None,
                Some(
                    editDist3ConfigDelete
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        } else {
            sqlite3_free(pConfig as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
static mut sqlite3Utf8Trans1: [::core::ffi::c_uchar; 64] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1b as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
unsafe extern "C" fn utf8Read(
    mut z: *const ::core::ffi::c_uchar,
    mut n: ::core::ffi::c_int,
    mut pSize: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        c = *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        if c >= 0xc0 as ::core::ffi::c_int {
            c = sqlite3Utf8Trans1[(c - 0xc0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int;
            while i < n
                && *z.offset(i as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int
            {
                let c2rust_fresh9 = i;
                i = i + 1;
                c = (c << 6 as ::core::ffi::c_int)
                    + (0x3f as ::core::ffi::c_int
                        & *z.offset(c2rust_fresh9 as isize) as ::core::ffi::c_int);
            }
        }
        *pSize = i;
        return c;
    }
}
unsafe extern "C" fn utf8Charlen(
    mut zIn: *const ::core::ffi::c_char,
    mut nIn: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut nChar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < nIn {
            let mut sz: ::core::ffi::c_int = 0;
            utf8Read(
                zIn.offset(i as isize) as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_uchar,
                nIn - i,
                &raw mut sz,
            );
            i += sz;
            nChar += 1;
        }
        return nChar;
    }
}
static mut translit: [Transliteration; 389] = [
    Transliteration {
        cFrom: 0xa0 as ::core::ffi::c_ushort,
        cTo0: 0x20 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xb5 as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xc0 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xc1 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xc2 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xc3 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xc4 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0x65 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xc5 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0x61 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xc6 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0x45 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xc7 as ::core::ffi::c_ushort,
        cTo0: 0x43 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xc8 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xc9 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xca as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xcb as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xcc as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xcd as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xce as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xcf as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xd0 as ::core::ffi::c_ushort,
        cTo0: 0x44 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xd1 as ::core::ffi::c_ushort,
        cTo0: 0x4e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xd2 as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xd3 as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xd4 as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xd5 as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xd6 as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0x65 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xd7 as ::core::ffi::c_ushort,
        cTo0: 0x78 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xd8 as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xd9 as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xda as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xdb as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xdc as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0x65 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xdd as ::core::ffi::c_ushort,
        cTo0: 0x59 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xde as ::core::ffi::c_ushort,
        cTo0: 0x54 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xdf as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0x73 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xe0 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xe1 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xe2 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xe3 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xe4 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0x65 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xe5 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0x61 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xe6 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0x65 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xe7 as ::core::ffi::c_ushort,
        cTo0: 0x63 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xe8 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xe9 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xea as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xeb as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xec as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xed as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xee as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xef as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xf0 as ::core::ffi::c_ushort,
        cTo0: 0x64 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xf1 as ::core::ffi::c_ushort,
        cTo0: 0x6e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xf2 as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xf3 as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xf4 as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xf5 as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xf6 as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0x65 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xf7 as ::core::ffi::c_ushort,
        cTo0: 0x3a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xf8 as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xf9 as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xfa as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xfb as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xfc as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0x65 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xfd as ::core::ffi::c_ushort,
        cTo0: 0x79 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xfe as ::core::ffi::c_ushort,
        cTo0: 0x74 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xff as ::core::ffi::c_ushort,
        cTo0: 0x79 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x100 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x101 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x102 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x103 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x104 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x105 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x106 as ::core::ffi::c_ushort,
        cTo0: 0x43 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x107 as ::core::ffi::c_ushort,
        cTo0: 0x63 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x108 as ::core::ffi::c_ushort,
        cTo0: 0x43 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x109 as ::core::ffi::c_ushort,
        cTo0: 0x63 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x10a as ::core::ffi::c_ushort,
        cTo0: 0x43 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x10b as ::core::ffi::c_ushort,
        cTo0: 0x63 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x10c as ::core::ffi::c_ushort,
        cTo0: 0x43 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x10d as ::core::ffi::c_ushort,
        cTo0: 0x63 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x10e as ::core::ffi::c_ushort,
        cTo0: 0x44 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x10f as ::core::ffi::c_ushort,
        cTo0: 0x64 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x110 as ::core::ffi::c_ushort,
        cTo0: 0x44 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x111 as ::core::ffi::c_ushort,
        cTo0: 0x64 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x112 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x113 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x114 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x115 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x116 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x117 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x118 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x119 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x11a as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x11b as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x11c as ::core::ffi::c_ushort,
        cTo0: 0x47 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x11d as ::core::ffi::c_ushort,
        cTo0: 0x67 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x11e as ::core::ffi::c_ushort,
        cTo0: 0x47 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x11f as ::core::ffi::c_ushort,
        cTo0: 0x67 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x120 as ::core::ffi::c_ushort,
        cTo0: 0x47 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x121 as ::core::ffi::c_ushort,
        cTo0: 0x67 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x122 as ::core::ffi::c_ushort,
        cTo0: 0x47 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x123 as ::core::ffi::c_ushort,
        cTo0: 0x67 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x124 as ::core::ffi::c_ushort,
        cTo0: 0x48 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x125 as ::core::ffi::c_ushort,
        cTo0: 0x68 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x126 as ::core::ffi::c_ushort,
        cTo0: 0x48 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x127 as ::core::ffi::c_ushort,
        cTo0: 0x68 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x128 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x129 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x12a as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x12b as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x12c as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x12d as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x12e as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x12f as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x130 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x131 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x132 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0x4a as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x133 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0x6a as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x134 as ::core::ffi::c_ushort,
        cTo0: 0x4a as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x135 as ::core::ffi::c_ushort,
        cTo0: 0x6a as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x136 as ::core::ffi::c_ushort,
        cTo0: 0x4b as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x137 as ::core::ffi::c_ushort,
        cTo0: 0x6b as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x138 as ::core::ffi::c_ushort,
        cTo0: 0x6b as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x139 as ::core::ffi::c_ushort,
        cTo0: 0x4c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x13a as ::core::ffi::c_ushort,
        cTo0: 0x6c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x13b as ::core::ffi::c_ushort,
        cTo0: 0x4c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x13c as ::core::ffi::c_ushort,
        cTo0: 0x6c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x13d as ::core::ffi::c_ushort,
        cTo0: 0x4c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x13e as ::core::ffi::c_ushort,
        cTo0: 0x6c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x13f as ::core::ffi::c_ushort,
        cTo0: 0x4c as ::core::ffi::c_uchar,
        cTo1: 0x2e as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x140 as ::core::ffi::c_ushort,
        cTo0: 0x6c as ::core::ffi::c_uchar,
        cTo1: 0x2e as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x141 as ::core::ffi::c_ushort,
        cTo0: 0x4c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x142 as ::core::ffi::c_ushort,
        cTo0: 0x6c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x143 as ::core::ffi::c_ushort,
        cTo0: 0x4e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x144 as ::core::ffi::c_ushort,
        cTo0: 0x6e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x145 as ::core::ffi::c_ushort,
        cTo0: 0x4e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x146 as ::core::ffi::c_ushort,
        cTo0: 0x6e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x147 as ::core::ffi::c_ushort,
        cTo0: 0x4e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x148 as ::core::ffi::c_ushort,
        cTo0: 0x6e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x149 as ::core::ffi::c_ushort,
        cTo0: 0x27 as ::core::ffi::c_uchar,
        cTo1: 0x6e as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x14a as ::core::ffi::c_ushort,
        cTo0: 0x4e as ::core::ffi::c_uchar,
        cTo1: 0x47 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x14b as ::core::ffi::c_ushort,
        cTo0: 0x6e as ::core::ffi::c_uchar,
        cTo1: 0x67 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x14c as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x14d as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x14e as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x14f as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x150 as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x151 as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x152 as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0x45 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x153 as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0x65 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x154 as ::core::ffi::c_ushort,
        cTo0: 0x52 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x155 as ::core::ffi::c_ushort,
        cTo0: 0x72 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x156 as ::core::ffi::c_ushort,
        cTo0: 0x52 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x157 as ::core::ffi::c_ushort,
        cTo0: 0x72 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x158 as ::core::ffi::c_ushort,
        cTo0: 0x52 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x159 as ::core::ffi::c_ushort,
        cTo0: 0x72 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x15a as ::core::ffi::c_ushort,
        cTo0: 0x53 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x15b as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x15c as ::core::ffi::c_ushort,
        cTo0: 0x53 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x15d as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x15e as ::core::ffi::c_ushort,
        cTo0: 0x53 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x15f as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x160 as ::core::ffi::c_ushort,
        cTo0: 0x53 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x161 as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x162 as ::core::ffi::c_ushort,
        cTo0: 0x54 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x163 as ::core::ffi::c_ushort,
        cTo0: 0x74 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x164 as ::core::ffi::c_ushort,
        cTo0: 0x54 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x165 as ::core::ffi::c_ushort,
        cTo0: 0x74 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x166 as ::core::ffi::c_ushort,
        cTo0: 0x54 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x167 as ::core::ffi::c_ushort,
        cTo0: 0x74 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x168 as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x169 as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x16a as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x16b as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x16c as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x16d as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x16e as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x16f as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x170 as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x171 as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x172 as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x173 as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x174 as ::core::ffi::c_ushort,
        cTo0: 0x57 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x175 as ::core::ffi::c_ushort,
        cTo0: 0x77 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x176 as ::core::ffi::c_ushort,
        cTo0: 0x59 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x177 as ::core::ffi::c_ushort,
        cTo0: 0x79 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x178 as ::core::ffi::c_ushort,
        cTo0: 0x59 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x179 as ::core::ffi::c_ushort,
        cTo0: 0x5a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x17a as ::core::ffi::c_ushort,
        cTo0: 0x7a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x17b as ::core::ffi::c_ushort,
        cTo0: 0x5a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x17c as ::core::ffi::c_ushort,
        cTo0: 0x7a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x17d as ::core::ffi::c_ushort,
        cTo0: 0x5a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x17e as ::core::ffi::c_ushort,
        cTo0: 0x7a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x17f as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x192 as ::core::ffi::c_ushort,
        cTo0: 0x66 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x218 as ::core::ffi::c_ushort,
        cTo0: 0x53 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x219 as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x21a as ::core::ffi::c_ushort,
        cTo0: 0x54 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x21b as ::core::ffi::c_ushort,
        cTo0: 0x74 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x386 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x388 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x389 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x38a as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x38c as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x38e as ::core::ffi::c_ushort,
        cTo0: 0x59 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x38f as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x390 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x391 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x392 as ::core::ffi::c_ushort,
        cTo0: 0x42 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x393 as ::core::ffi::c_ushort,
        cTo0: 0x47 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x394 as ::core::ffi::c_ushort,
        cTo0: 0x44 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x395 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x396 as ::core::ffi::c_ushort,
        cTo0: 0x5a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x397 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x398 as ::core::ffi::c_ushort,
        cTo0: 0x54 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x399 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x39a as ::core::ffi::c_ushort,
        cTo0: 0x4b as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x39b as ::core::ffi::c_ushort,
        cTo0: 0x4c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x39c as ::core::ffi::c_ushort,
        cTo0: 0x4d as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x39d as ::core::ffi::c_ushort,
        cTo0: 0x4e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x39e as ::core::ffi::c_ushort,
        cTo0: 0x58 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x39f as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3a0 as ::core::ffi::c_ushort,
        cTo0: 0x50 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3a1 as ::core::ffi::c_ushort,
        cTo0: 0x52 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3a3 as ::core::ffi::c_ushort,
        cTo0: 0x53 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3a4 as ::core::ffi::c_ushort,
        cTo0: 0x54 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3a5 as ::core::ffi::c_ushort,
        cTo0: 0x59 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3a6 as ::core::ffi::c_ushort,
        cTo0: 0x46 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3a7 as ::core::ffi::c_ushort,
        cTo0: 0x43 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3a8 as ::core::ffi::c_ushort,
        cTo0: 0x50 as ::core::ffi::c_uchar,
        cTo1: 0x73 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3a9 as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3aa as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3ab as ::core::ffi::c_ushort,
        cTo0: 0x59 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3ac as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3ad as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3ae as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3af as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3b1 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3b2 as ::core::ffi::c_ushort,
        cTo0: 0x62 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3b3 as ::core::ffi::c_ushort,
        cTo0: 0x67 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3b4 as ::core::ffi::c_ushort,
        cTo0: 0x64 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3b5 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3b6 as ::core::ffi::c_ushort,
        cTo0: 0x7a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3b7 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3b8 as ::core::ffi::c_ushort,
        cTo0: 0x74 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3b9 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3ba as ::core::ffi::c_ushort,
        cTo0: 0x6b as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3bb as ::core::ffi::c_ushort,
        cTo0: 0x6c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3bc as ::core::ffi::c_ushort,
        cTo0: 0x6d as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3bd as ::core::ffi::c_ushort,
        cTo0: 0x6e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3be as ::core::ffi::c_ushort,
        cTo0: 0x78 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3bf as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3c0 as ::core::ffi::c_ushort,
        cTo0: 0x70 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3c1 as ::core::ffi::c_ushort,
        cTo0: 0x72 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3c3 as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3c4 as ::core::ffi::c_ushort,
        cTo0: 0x74 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3c5 as ::core::ffi::c_ushort,
        cTo0: 0x79 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3c6 as ::core::ffi::c_ushort,
        cTo0: 0x66 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3c7 as ::core::ffi::c_ushort,
        cTo0: 0x63 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3c8 as ::core::ffi::c_ushort,
        cTo0: 0x70 as ::core::ffi::c_uchar,
        cTo1: 0x73 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3c9 as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3ca as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3cb as ::core::ffi::c_ushort,
        cTo0: 0x79 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3cc as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3cd as ::core::ffi::c_ushort,
        cTo0: 0x79 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x3ce as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x400 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x401 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x402 as ::core::ffi::c_ushort,
        cTo0: 0x44 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x403 as ::core::ffi::c_ushort,
        cTo0: 0x47 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x404 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x405 as ::core::ffi::c_ushort,
        cTo0: 0x5a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x406 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x407 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x408 as ::core::ffi::c_ushort,
        cTo0: 0x4a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x409 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x40a as ::core::ffi::c_ushort,
        cTo0: 0x4e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x40b as ::core::ffi::c_ushort,
        cTo0: 0x44 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x40c as ::core::ffi::c_ushort,
        cTo0: 0x4b as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x40d as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x40e as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x40f as ::core::ffi::c_ushort,
        cTo0: 0x44 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x410 as ::core::ffi::c_ushort,
        cTo0: 0x41 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x411 as ::core::ffi::c_ushort,
        cTo0: 0x42 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x412 as ::core::ffi::c_ushort,
        cTo0: 0x56 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x413 as ::core::ffi::c_ushort,
        cTo0: 0x47 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x414 as ::core::ffi::c_ushort,
        cTo0: 0x44 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x415 as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x416 as ::core::ffi::c_ushort,
        cTo0: 0x5a as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x417 as ::core::ffi::c_ushort,
        cTo0: 0x5a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x418 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x419 as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x41a as ::core::ffi::c_ushort,
        cTo0: 0x4b as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x41b as ::core::ffi::c_ushort,
        cTo0: 0x4c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x41c as ::core::ffi::c_ushort,
        cTo0: 0x4d as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x41d as ::core::ffi::c_ushort,
        cTo0: 0x4e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x41e as ::core::ffi::c_ushort,
        cTo0: 0x4f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x41f as ::core::ffi::c_ushort,
        cTo0: 0x50 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x420 as ::core::ffi::c_ushort,
        cTo0: 0x52 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x421 as ::core::ffi::c_ushort,
        cTo0: 0x53 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x422 as ::core::ffi::c_ushort,
        cTo0: 0x54 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x423 as ::core::ffi::c_ushort,
        cTo0: 0x55 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x424 as ::core::ffi::c_ushort,
        cTo0: 0x46 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x425 as ::core::ffi::c_ushort,
        cTo0: 0x4b as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x426 as ::core::ffi::c_ushort,
        cTo0: 0x54 as ::core::ffi::c_uchar,
        cTo1: 0x63 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x427 as ::core::ffi::c_ushort,
        cTo0: 0x43 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x428 as ::core::ffi::c_ushort,
        cTo0: 0x53 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x429 as ::core::ffi::c_ushort,
        cTo0: 0x53 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0x63 as ::core::ffi::c_uchar,
        cTo3: 0x68 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x42a as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x42b as ::core::ffi::c_ushort,
        cTo0: 0x59 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x42c as ::core::ffi::c_ushort,
        cTo0: 0x59 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x42d as ::core::ffi::c_ushort,
        cTo0: 0x45 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x42e as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0x75 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x42f as ::core::ffi::c_ushort,
        cTo0: 0x49 as ::core::ffi::c_uchar,
        cTo1: 0x61 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x430 as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x431 as ::core::ffi::c_ushort,
        cTo0: 0x62 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x432 as ::core::ffi::c_ushort,
        cTo0: 0x76 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x433 as ::core::ffi::c_ushort,
        cTo0: 0x67 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x434 as ::core::ffi::c_ushort,
        cTo0: 0x64 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x435 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x436 as ::core::ffi::c_ushort,
        cTo0: 0x7a as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x437 as ::core::ffi::c_ushort,
        cTo0: 0x7a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x438 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x439 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x43a as ::core::ffi::c_ushort,
        cTo0: 0x6b as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x43b as ::core::ffi::c_ushort,
        cTo0: 0x6c as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x43c as ::core::ffi::c_ushort,
        cTo0: 0x6d as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x43d as ::core::ffi::c_ushort,
        cTo0: 0x6e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x43e as ::core::ffi::c_ushort,
        cTo0: 0x6f as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x43f as ::core::ffi::c_ushort,
        cTo0: 0x70 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x440 as ::core::ffi::c_ushort,
        cTo0: 0x72 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x441 as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x442 as ::core::ffi::c_ushort,
        cTo0: 0x74 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x443 as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x444 as ::core::ffi::c_ushort,
        cTo0: 0x66 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x445 as ::core::ffi::c_ushort,
        cTo0: 0x6b as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x446 as ::core::ffi::c_ushort,
        cTo0: 0x74 as ::core::ffi::c_uchar,
        cTo1: 0x63 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x447 as ::core::ffi::c_ushort,
        cTo0: 0x63 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x448 as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x449 as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0x68 as ::core::ffi::c_uchar,
        cTo2: 0x63 as ::core::ffi::c_uchar,
        cTo3: 0x68 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x44a as ::core::ffi::c_ushort,
        cTo0: 0x61 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x44b as ::core::ffi::c_ushort,
        cTo0: 0x79 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x44c as ::core::ffi::c_ushort,
        cTo0: 0x79 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x44d as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x44e as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0x75 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x44f as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0x61 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x450 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x451 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x452 as ::core::ffi::c_ushort,
        cTo0: 0x64 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x453 as ::core::ffi::c_ushort,
        cTo0: 0x67 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x454 as ::core::ffi::c_ushort,
        cTo0: 0x65 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x455 as ::core::ffi::c_ushort,
        cTo0: 0x7a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x456 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x457 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x458 as ::core::ffi::c_ushort,
        cTo0: 0x6a as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x459 as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x45a as ::core::ffi::c_ushort,
        cTo0: 0x6e as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x45b as ::core::ffi::c_ushort,
        cTo0: 0x64 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x45c as ::core::ffi::c_ushort,
        cTo0: 0x6b as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x45d as ::core::ffi::c_ushort,
        cTo0: 0x69 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x45e as ::core::ffi::c_ushort,
        cTo0: 0x75 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x45f as ::core::ffi::c_ushort,
        cTo0: 0x64 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e02 as ::core::ffi::c_ushort,
        cTo0: 0x42 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e03 as ::core::ffi::c_ushort,
        cTo0: 0x62 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e0a as ::core::ffi::c_ushort,
        cTo0: 0x44 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e0b as ::core::ffi::c_ushort,
        cTo0: 0x64 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e1e as ::core::ffi::c_ushort,
        cTo0: 0x46 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e1f as ::core::ffi::c_ushort,
        cTo0: 0x66 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e40 as ::core::ffi::c_ushort,
        cTo0: 0x4d as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e41 as ::core::ffi::c_ushort,
        cTo0: 0x6d as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e56 as ::core::ffi::c_ushort,
        cTo0: 0x50 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e57 as ::core::ffi::c_ushort,
        cTo0: 0x70 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e60 as ::core::ffi::c_ushort,
        cTo0: 0x53 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e61 as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e6a as ::core::ffi::c_ushort,
        cTo0: 0x54 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e6b as ::core::ffi::c_ushort,
        cTo0: 0x74 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e80 as ::core::ffi::c_ushort,
        cTo0: 0x57 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e81 as ::core::ffi::c_ushort,
        cTo0: 0x77 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e82 as ::core::ffi::c_ushort,
        cTo0: 0x57 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e83 as ::core::ffi::c_ushort,
        cTo0: 0x77 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e84 as ::core::ffi::c_ushort,
        cTo0: 0x57 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1e85 as ::core::ffi::c_ushort,
        cTo0: 0x77 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1ef2 as ::core::ffi::c_ushort,
        cTo0: 0x59 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0x1ef3 as ::core::ffi::c_ushort,
        cTo0: 0x79 as ::core::ffi::c_uchar,
        cTo1: 0 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xfb00 as ::core::ffi::c_ushort,
        cTo0: 0x66 as ::core::ffi::c_uchar,
        cTo1: 0x66 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xfb01 as ::core::ffi::c_ushort,
        cTo0: 0x66 as ::core::ffi::c_uchar,
        cTo1: 0x69 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xfb02 as ::core::ffi::c_ushort,
        cTo0: 0x66 as ::core::ffi::c_uchar,
        cTo1: 0x6c as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xfb05 as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0x74 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
    Transliteration {
        cFrom: 0xfb06 as ::core::ffi::c_ushort,
        cTo0: 0x73 as ::core::ffi::c_uchar,
        cTo1: 0x74 as ::core::ffi::c_uchar,
        cTo2: 0 as ::core::ffi::c_uchar,
        cTo3: 0 as ::core::ffi::c_uchar,
    },
];
unsafe extern "C" fn spellfixFindTranslit(
    mut c: ::core::ffi::c_int,
    mut pxTop: *mut ::core::ffi::c_int,
) -> *const Transliteration {
    unsafe {
        *pxTop = (::core::mem::size_of::<[Transliteration; 389]>() as usize)
            .wrapping_div(::core::mem::size_of::<Transliteration>() as usize)
            .wrapping_sub(1 as usize) as ::core::ffi::c_int;
        return &raw const translit as *const Transliteration;
    }
}
unsafe extern "C" fn transliterate(
    mut zIn: *const ::core::ffi::c_uchar,
    mut nIn: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    unsafe {
        let mut zOut: *mut ::core::ffi::c_uchar = sqlite3_malloc64(
            (nIn * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as sqlite3_uint64,
        ) as *mut ::core::ffi::c_uchar;
        let mut c: ::core::ffi::c_int = 0;
        let mut sz: ::core::ffi::c_int = 0;
        let mut nOut: ::core::ffi::c_int = 0;
        if zOut.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        nOut = 0 as ::core::ffi::c_int;
        while nIn > 0 as ::core::ffi::c_int {
            c = utf8Read(zIn, nIn, &raw mut sz);
            zIn = zIn.offset(sz as isize);
            nIn -= sz;
            if c <= 127 as ::core::ffi::c_int {
                let c2rust_fresh10 = nOut;
                nOut = nOut + 1;
                *zOut.offset(c2rust_fresh10 as isize) = c as ::core::ffi::c_uchar;
            } else {
                let mut xTop: ::core::ffi::c_int = 0;
                let mut xBtm: ::core::ffi::c_int = 0;
                let mut x: ::core::ffi::c_int = 0;
                let mut tbl: *const Transliteration = spellfixFindTranslit(
                    c,
                    &raw mut xTop,
                );
                xBtm = 0 as ::core::ffi::c_int;
                while xTop >= xBtm {
                    x = (xTop + xBtm) / 2 as ::core::ffi::c_int;
                    if (*tbl.offset(x as isize)).cFrom as ::core::ffi::c_int == c {
                        let c2rust_fresh11 = nOut;
                        nOut = nOut + 1;
                        *zOut.offset(c2rust_fresh11 as isize) = (*tbl.offset(x as isize))
                            .cTo0;
                        if (*tbl.offset(x as isize)).cTo1 != 0 {
                            let c2rust_fresh12 = nOut;
                            nOut = nOut + 1;
                            *zOut.offset(c2rust_fresh12 as isize) = (*tbl
                                .offset(x as isize))
                                .cTo1;
                            if (*tbl.offset(x as isize)).cTo2 != 0 {
                                let c2rust_fresh13 = nOut;
                                nOut = nOut + 1;
                                *zOut.offset(c2rust_fresh13 as isize) = (*tbl
                                    .offset(x as isize))
                                    .cTo2;
                                if (*tbl.offset(x as isize)).cTo3 != 0 {
                                    let c2rust_fresh14 = nOut;
                                    nOut = nOut + 1;
                                    *zOut.offset(c2rust_fresh14 as isize) = (*tbl
                                        .offset(x as isize))
                                        .cTo3;
                                }
                            }
                        }
                        c = 0 as ::core::ffi::c_int;
                        break;
                    } else if (*tbl.offset(x as isize)).cFrom as ::core::ffi::c_int > c {
                        xTop = x - 1 as ::core::ffi::c_int;
                    } else {
                        xBtm = x + 1 as ::core::ffi::c_int;
                    }
                }
                if c != 0 {
                    let c2rust_fresh15 = nOut;
                    nOut = nOut + 1;
                    *zOut.offset(c2rust_fresh15 as isize) = '?' as i32
                        as ::core::ffi::c_uchar;
                }
            }
        }
        *zOut.offset(nOut as isize) = 0 as ::core::ffi::c_uchar;
        return zOut;
    }
}
unsafe extern "C" fn translen_to_charlen(
    mut zIn: *const ::core::ffi::c_char,
    mut nIn: ::core::ffi::c_int,
    mut nTrans: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        let mut sz: ::core::ffi::c_int = 0;
        let mut nOut: ::core::ffi::c_int = 0;
        let mut nChar: ::core::ffi::c_int = 0;
        nOut = 0 as ::core::ffi::c_int;
        i = nOut;
        nChar = 0 as ::core::ffi::c_int;
        while i < nIn && nOut < nTrans {
            c = utf8Read(
                zIn.offset(i as isize) as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_uchar,
                nIn - i,
                &raw mut sz,
            );
            i += sz;
            nOut += 1;
            if c >= 128 as ::core::ffi::c_int {
                let mut xTop: ::core::ffi::c_int = 0;
                let mut xBtm: ::core::ffi::c_int = 0;
                let mut x: ::core::ffi::c_int = 0;
                let mut tbl: *const Transliteration = spellfixFindTranslit(
                    c,
                    &raw mut xTop,
                );
                xBtm = 0 as ::core::ffi::c_int;
                while xTop >= xBtm {
                    x = (xTop + xBtm) / 2 as ::core::ffi::c_int;
                    if (*tbl.offset(x as isize)).cFrom as ::core::ffi::c_int == c {
                        if (*tbl.offset(x as isize)).cTo1 != 0 {
                            nOut += 1;
                            if (*tbl.offset(x as isize)).cTo2 != 0 {
                                nOut += 1;
                                if (*tbl.offset(x as isize)).cTo3 != 0 {
                                    nOut += 1;
                                }
                            }
                        }
                        break;
                    } else if (*tbl.offset(x as isize)).cFrom as ::core::ffi::c_int > c {
                        xTop = x - 1 as ::core::ffi::c_int;
                    } else {
                        xBtm = x + 1 as ::core::ffi::c_int;
                    }
                }
            }
            nChar += 1;
        }
        return nChar;
    }
}
unsafe extern "C" fn transliterateSqlFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zIn: *const ::core::ffi::c_uchar = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        let mut nIn: ::core::ffi::c_int = sqlite3_value_bytes(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        let mut zOut: *mut ::core::ffi::c_uchar = transliterate(zIn, nIn);
        if zOut.is_null() {
            sqlite3_result_error_nomem(context);
        } else {
            sqlite3_result_text(
                context,
                zOut as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        };
    }
}
unsafe extern "C" fn scriptCodeSqlFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zIn: *const ::core::ffi::c_uchar = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        let mut nIn: ::core::ffi::c_int = sqlite3_value_bytes(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        let mut c: ::core::ffi::c_int = 0;
        let mut sz: ::core::ffi::c_int = 0;
        let mut scriptMask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut res: ::core::ffi::c_int = 0;
        let mut seenDigit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while nIn > 0 as ::core::ffi::c_int {
            c = utf8Read(zIn, nIn, &raw mut sz);
            zIn = zIn.offset(sz as isize);
            nIn -= sz;
            if c < 0x2af as ::core::ffi::c_int {
                if c >= 0x80 as ::core::ffi::c_int
                    || (midClass[(c & 0x7f as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int) < CCLASS_DIGIT
                {
                    scriptMask |= SCRIPT_LATIN;
                } else if c >= '0' as i32 && c <= '9' as i32 {
                    seenDigit = 1 as ::core::ffi::c_int;
                }
            } else if c >= 0x400 as ::core::ffi::c_int
                && c <= 0x4ff as ::core::ffi::c_int
            {
                scriptMask |= SCRIPT_CYRILLIC;
            } else if c >= 0x386 as ::core::ffi::c_int
                && c <= 0x3ce as ::core::ffi::c_int
            {
                scriptMask |= SCRIPT_GREEK;
            } else if c >= 0x590 as ::core::ffi::c_int
                && c <= 0x5ff as ::core::ffi::c_int
            {
                scriptMask |= SCRIPT_HEBREW;
            } else if c >= 0x600 as ::core::ffi::c_int
                && c <= 0x6ff as ::core::ffi::c_int
            {
                scriptMask |= SCRIPT_ARABIC;
            }
        }
        if scriptMask == 0 as ::core::ffi::c_int && seenDigit != 0 {
            scriptMask = SCRIPT_LATIN;
        }
        match scriptMask {
            0 => {
                res = 999 as ::core::ffi::c_int;
            }
            SCRIPT_LATIN => {
                res = 215 as ::core::ffi::c_int;
            }
            SCRIPT_CYRILLIC => {
                res = 220 as ::core::ffi::c_int;
            }
            SCRIPT_GREEK => {
                res = 200 as ::core::ffi::c_int;
            }
            SCRIPT_HEBREW => {
                res = 125 as ::core::ffi::c_int;
            }
            SCRIPT_ARABIC => {
                res = 160 as ::core::ffi::c_int;
            }
            _ => {
                res = 998 as ::core::ffi::c_int;
            }
        }
        sqlite3_result_int(context, res);
    }
}
pub const SCRIPT_LATIN: ::core::ffi::c_int = 1;
pub const SCRIPT_CYRILLIC: ::core::ffi::c_int = 2;
pub const SCRIPT_GREEK: ::core::ffi::c_int = 4;
pub const SCRIPT_HEBREW: ::core::ffi::c_int = 8;
pub const SCRIPT_ARABIC: ::core::ffi::c_int = 16;
pub const SPELLFIX_MX_HASH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C" fn spellfix1DbExec(
    mut pRc: *mut ::core::ffi::c_int,
    mut db: *mut sqlite3,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if *pRc != 0 {
            return;
        }
        let mut ap = c2rust_args.clone();
        zSql = sqlite3_vmprintf(zFormat, ap);
        if zSql.is_null() {
            *pRc = SQLITE_NOMEM;
        } else {
            *pRc = sqlite3_exec(
                db,
                zSql,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
        };
    }
}
unsafe extern "C" fn spellfix1Uninit(
    mut isDestroy: ::core::ffi::c_int,
    mut pVTab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut spellfix1_vtab = pVTab as *mut spellfix1_vtab;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if isDestroy != 0 {
            let mut db: *mut sqlite3 = (*p).db;
            spellfix1DbExec(
                &raw mut rc,
                db,
                b"DROP TABLE IF EXISTS \"%w\".\"%w_vocab\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zDbName,
                (*p).zTableName,
            );
        }
        if rc == SQLITE_OK {
            sqlite3_free((*p).zTableName as *mut ::core::ffi::c_void);
            editDist3ConfigDelete((*p).pConfig3 as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zCostTable as *mut ::core::ffi::c_void);
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn spellfix1Disconnect(
    mut pVTab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        return spellfix1Uninit(0 as ::core::ffi::c_int, pVTab);
    }
}
unsafe extern "C" fn spellfix1Destroy(
    mut pVTab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        return spellfix1Uninit(1 as ::core::ffi::c_int, pVTab);
    }
}
unsafe extern "C" fn spellfix1Dequote(
    mut zIn: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_char = 0;
        while *(*__ctype_b_loc())
            .offset(
                *zIn.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                    as ::core::ffi::c_int as isize,
            ) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            zIn = zIn.offset(1);
        }
        zOut = sqlite3_mprintf(b"%s\0".as_ptr() as *const ::core::ffi::c_char, zIn);
        if zOut.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        i = strlen(zOut) as ::core::ffi::c_int;
        *zOut.offset(i as isize) = 0 as ::core::ffi::c_char;
        c = *zOut.offset(0 as ::core::ffi::c_int as isize);
        if c as ::core::ffi::c_int == '\'' as i32
            || c as ::core::ffi::c_int == '"' as i32
        {
            i = 1 as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            loop {
                let c2rust_fresh16 = j;
                j = j + 1;
                *zOut.offset(c2rust_fresh16 as isize) = *zOut.offset(i as isize);
                if *zOut.offset(i as isize) as ::core::ffi::c_int
                    == c as ::core::ffi::c_int
                {
                    if *zOut.offset((i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == c as ::core::ffi::c_int
                    {
                        i += 1;
                    } else {
                        *zOut.offset((j - 1 as ::core::ffi::c_int) as isize) = 0
                            as ::core::ffi::c_char;
                        break;
                    }
                }
                i += 1;
            }
        }
        return zOut;
    }
}
unsafe extern "C" fn spellfix1Init(
    mut isCreate: ::core::ffi::c_int,
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVTab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNew: *mut spellfix1_vtab = ::core::ptr::null_mut::<spellfix1_vtab>();
        let mut zDbName: *const ::core::ffi::c_char = *argv
            .offset(1 as ::core::ffi::c_int as isize);
        let mut zTableName: *const ::core::ffi::c_char = *argv
            .offset(2 as ::core::ffi::c_int as isize);
        let mut nDbName: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
        nDbName = strlen(zDbName) as ::core::ffi::c_int;
        pNew = sqlite3_malloc64(
            (::core::mem::size_of::<spellfix1_vtab>() as usize)
                .wrapping_add(nDbName as usize)
                .wrapping_add(1 as usize) as sqlite3_uint64,
        ) as *mut spellfix1_vtab;
        if pNew.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            memset(
                pNew as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<spellfix1_vtab>() as size_t,
            );
            (*pNew).zDbName = pNew.offset(1 as ::core::ffi::c_int as isize)
                as *mut spellfix1_vtab as *mut ::core::ffi::c_char;
            memcpy(
                (*pNew).zDbName as *mut ::core::ffi::c_void,
                zDbName as *const ::core::ffi::c_void,
                (nDbName + 1 as ::core::ffi::c_int) as size_t,
            );
            (*pNew).zTableName = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                zTableName,
            );
            (*pNew).db = db;
            if (*pNew).zTableName.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                sqlite3_vtab_config(db, SQLITE_VTAB_INNOCUOUS);
                rc = sqlite3_declare_vtab(
                    db,
                    b"CREATE TABLE x(word,rank,distance,langid, score, matchlen, phonehash HIDDEN, top HIDDEN, scope HIDDEN, srchcnt HIDDEN, soundslike HIDDEN, command HIDDEN)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if rc == SQLITE_OK && isCreate != 0 {
                spellfix1DbExec(
                    &raw mut rc,
                    db,
                    b"CREATE TABLE IF NOT EXISTS \"%w\".\"%w_vocab\"(\n  id INTEGER PRIMARY KEY,\n  rank INT,\n  langid INT,\n  word TEXT,\n  k1 TEXT,\n  k2 TEXT\n);\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    zDbName,
                    zTableName,
                );
                spellfix1DbExec(
                    &raw mut rc,
                    db,
                    b"CREATE INDEX IF NOT EXISTS \"%w\".\"%w_vocab_index_langid_k2\" ON \"%w_vocab\"(langid,k2);\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    zDbName,
                    zTableName,
                    zTableName,
                );
            }
            i = 3 as ::core::ffi::c_int;
            while rc == SQLITE_OK && i < argc {
                if strncmp(
                    *argv.offset(i as isize),
                    b"edit_cost_table=\0".as_ptr() as *const ::core::ffi::c_char,
                    16 as size_t,
                ) == 0 as ::core::ffi::c_int && (*pNew).zCostTable.is_null()
                {
                    (*pNew).zCostTable = spellfix1Dequote(
                        (*argv.offset(i as isize))
                            .offset(16 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_char,
                    );
                    if (*pNew).zCostTable.is_null() {
                        rc = SQLITE_NOMEM;
                    }
                } else {
                    *pzErr = sqlite3_mprintf(
                        b"bad argument to spellfix1(): \"%s\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        *argv.offset(i as isize),
                    );
                    rc = SQLITE_ERROR;
                }
                i += 1;
            }
        }
        if rc != 0 && !pNew.is_null() {
            *ppVTab = ::core::ptr::null_mut::<sqlite3_vtab>();
            spellfix1Uninit(0 as ::core::ffi::c_int, &raw mut (*pNew).base);
        } else {
            *ppVTab = pNew as *mut sqlite3_vtab;
        }
        return rc;
    }
}
pub const SPELLFIX_COL_WORD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SPELLFIX_COL_RANK: ::core::ffi::c_int = 1;
pub const SPELLFIX_COL_DISTANCE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SPELLFIX_COL_LANGID: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SPELLFIX_COL_SCORE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SPELLFIX_COL_MATCHLEN: ::core::ffi::c_int = 5;
pub const SPELLFIX_COL_PHONEHASH: ::core::ffi::c_int = 6;
pub const SPELLFIX_COL_TOP: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SPELLFIX_COL_SCOPE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SPELLFIX_COL_SRCHCNT: ::core::ffi::c_int = 9;
pub const SPELLFIX_COL_SOUNDSLIKE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SPELLFIX_COL_COMMAND: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
unsafe extern "C" fn spellfix1Connect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVTab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return spellfix1Init(
            0 as ::core::ffi::c_int,
            db,
            pAux,
            argc,
            argv,
            ppVTab,
            pzErr,
        );
    }
}
unsafe extern "C" fn spellfix1Create(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVTab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return spellfix1Init(
            1 as ::core::ffi::c_int,
            db,
            pAux,
            argc,
            argv,
            ppVTab,
            pzErr,
        );
    }
}
unsafe extern "C" fn spellfix1ResetCursor(mut pCur: *mut spellfix1_cursor) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pCur).nRow {
            sqlite3_free(
                (*(*pCur).a.offset(i as isize)).zWord as *mut ::core::ffi::c_void,
            );
            i += 1;
        }
        (*pCur).nRow = 0 as ::core::ffi::c_int;
        (*pCur).iRow = 0 as ::core::ffi::c_int;
        (*pCur).nSearch = 0 as ::core::ffi::c_int;
        if !(*pCur).pFullScan.is_null() {
            sqlite3_finalize((*pCur).pFullScan);
            (*pCur).pFullScan = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
    }
}
unsafe extern "C" fn spellfix1ResizeCursor(
    mut pCur: *mut spellfix1_cursor,
    mut N: ::core::ffi::c_int,
) {
    unsafe {
        let mut aNew: *mut spellfix1_row = ::core::ptr::null_mut::<spellfix1_row>();
        aNew = sqlite3_realloc64(
            (*pCur).a as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<spellfix1_row>() as usize).wrapping_mul(N as usize)
                as sqlite3_uint64,
        ) as *mut spellfix1_row;
        if aNew.is_null() && N > 0 as ::core::ffi::c_int {
            spellfix1ResetCursor(pCur);
            sqlite3_free((*pCur).a as *mut ::core::ffi::c_void);
            (*pCur).nAlloc = 0 as ::core::ffi::c_int;
            (*pCur).a = ::core::ptr::null_mut::<spellfix1_row>();
        } else {
            (*pCur).nAlloc = N;
            (*pCur).a = aNew as *mut spellfix1_row;
        };
    }
}
unsafe extern "C" fn spellfix1Close(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut spellfix1_cursor = cur as *mut spellfix1_cursor;
        spellfix1ResetCursor(pCur);
        spellfix1ResizeCursor(pCur, 0 as ::core::ffi::c_int);
        sqlite3_free((*pCur).zPattern as *mut ::core::ffi::c_void);
        sqlite3_free(pCur as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
pub const SPELLFIX_IDXNUM_MATCH: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SPELLFIX_IDXNUM_LANGID: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SPELLFIX_IDXNUM_TOP: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SPELLFIX_IDXNUM_SCOPE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SPELLFIX_IDXNUM_DISTLT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SPELLFIX_IDXNUM_DISTLE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SPELLFIX_IDXNUM_ROWID: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SPELLFIX_IDXNUM_DIST: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int
    | 0x20 as ::core::ffi::c_int;
unsafe extern "C" fn spellfix1BestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iPlan: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iLangTerm: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iTopTerm: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iScopeTerm: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iDistTerm: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iRowidTerm: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut pConstraint: *const sqlite3_index_constraint = ::core::ptr::null::<
            sqlite3_index_constraint,
        >();
        pConstraint = (*pIdxInfo).aConstraint;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdxInfo).nConstraint {
            if !((*pConstraint).usable as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            {
                if iPlan & SPELLFIX_IDXNUM_MATCH == 0 as ::core::ffi::c_int
                    && (*pConstraint).iColumn == SPELLFIX_COL_WORD
                    && (*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_MATCH
                {
                    iPlan |= SPELLFIX_IDXNUM_MATCH;
                    (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).argvIndex = 1
                        as ::core::ffi::c_int;
                    (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).omit = 1
                        as ::core::ffi::c_uchar;
                }
                if iPlan & SPELLFIX_IDXNUM_LANGID == 0 as ::core::ffi::c_int
                    && (*pConstraint).iColumn == SPELLFIX_COL_LANGID
                    && (*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_EQ
                {
                    iPlan |= SPELLFIX_IDXNUM_LANGID;
                    iLangTerm = i;
                }
                if iPlan & SPELLFIX_IDXNUM_TOP == 0 as ::core::ffi::c_int
                    && (*pConstraint).iColumn == SPELLFIX_COL_TOP
                    && (*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_EQ
                {
                    iPlan |= SPELLFIX_IDXNUM_TOP;
                    iTopTerm = i;
                }
                if iPlan & SPELLFIX_IDXNUM_SCOPE == 0 as ::core::ffi::c_int
                    && (*pConstraint).iColumn == SPELLFIX_COL_SCOPE
                    && (*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_EQ
                {
                    iPlan |= SPELLFIX_IDXNUM_SCOPE;
                    iScopeTerm = i;
                }
                if iPlan & SPELLFIX_IDXNUM_DIST == 0 as ::core::ffi::c_int
                    && (*pConstraint).iColumn == SPELLFIX_COL_DISTANCE
                    && ((*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_LT
                        || (*pConstraint).op as ::core::ffi::c_int
                            == SQLITE_INDEX_CONSTRAINT_LE)
                {
                    if (*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_LT
                    {
                        iPlan |= SPELLFIX_IDXNUM_DISTLT;
                    } else {
                        iPlan |= SPELLFIX_IDXNUM_DISTLE;
                    }
                    iDistTerm = i;
                }
                if iPlan & SPELLFIX_IDXNUM_ROWID == 0 as ::core::ffi::c_int
                    && (*pConstraint).iColumn < 0 as ::core::ffi::c_int
                    && (*pConstraint).op as ::core::ffi::c_int
                        == SQLITE_INDEX_CONSTRAINT_EQ
                {
                    iPlan |= SPELLFIX_IDXNUM_ROWID;
                    iRowidTerm = i;
                }
            }
            i += 1;
            pConstraint = pConstraint.offset(1);
        }
        if iPlan & SPELLFIX_IDXNUM_MATCH != 0 {
            let mut idx: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
            (*pIdxInfo).idxNum = iPlan;
            if (*pIdxInfo).nOrderBy == 1 as ::core::ffi::c_int
                && (*(*pIdxInfo).aOrderBy.offset(0 as ::core::ffi::c_int as isize))
                    .iColumn == SPELLFIX_COL_SCORE
                && (*(*pIdxInfo).aOrderBy.offset(0 as ::core::ffi::c_int as isize)).desc
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                (*pIdxInfo).orderByConsumed = 1 as ::core::ffi::c_int;
            }
            if iPlan & SPELLFIX_IDXNUM_LANGID != 0 {
                let c2rust_fresh17 = idx;
                idx = idx + 1;
                (*(*pIdxInfo).aConstraintUsage.offset(iLangTerm as isize)).argvIndex = c2rust_fresh17;
                (*(*pIdxInfo).aConstraintUsage.offset(iLangTerm as isize)).omit = 1
                    as ::core::ffi::c_uchar;
            }
            if iPlan & SPELLFIX_IDXNUM_TOP != 0 {
                let c2rust_fresh18 = idx;
                idx = idx + 1;
                (*(*pIdxInfo).aConstraintUsage.offset(iTopTerm as isize)).argvIndex = c2rust_fresh18;
                (*(*pIdxInfo).aConstraintUsage.offset(iTopTerm as isize)).omit = 1
                    as ::core::ffi::c_uchar;
            }
            if iPlan & SPELLFIX_IDXNUM_SCOPE != 0 {
                let c2rust_fresh19 = idx;
                idx = idx + 1;
                (*(*pIdxInfo).aConstraintUsage.offset(iScopeTerm as isize)).argvIndex = c2rust_fresh19;
                (*(*pIdxInfo).aConstraintUsage.offset(iScopeTerm as isize)).omit = 1
                    as ::core::ffi::c_uchar;
            }
            if iPlan & SPELLFIX_IDXNUM_DIST != 0 {
                let c2rust_fresh20 = idx;
                idx = idx + 1;
                (*(*pIdxInfo).aConstraintUsage.offset(iDistTerm as isize)).argvIndex = c2rust_fresh20;
                (*(*pIdxInfo).aConstraintUsage.offset(iDistTerm as isize)).omit = 1
                    as ::core::ffi::c_uchar;
            }
            (*pIdxInfo).estimatedCost = 1e5f64;
        } else if iPlan & SPELLFIX_IDXNUM_ROWID != 0 {
            (*pIdxInfo).idxNum = SPELLFIX_IDXNUM_ROWID;
            (*(*pIdxInfo).aConstraintUsage.offset(iRowidTerm as isize)).argvIndex = 1
                as ::core::ffi::c_int;
            (*(*pIdxInfo).aConstraintUsage.offset(iRowidTerm as isize)).omit = 1
                as ::core::ffi::c_uchar;
            (*pIdxInfo).estimatedCost = 5 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            (*pIdxInfo).idxNum = 0 as ::core::ffi::c_int;
            (*pIdxInfo).estimatedCost = 1e50f64;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn spellfix1Open(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut spellfix1_vtab = pVTab as *mut spellfix1_vtab;
        let mut pCur: *mut spellfix1_cursor = ::core::ptr::null_mut::<
            spellfix1_cursor,
        >();
        pCur = sqlite3_malloc64(
            ::core::mem::size_of::<spellfix1_cursor>() as sqlite3_uint64,
        ) as *mut spellfix1_cursor;
        if pCur.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pCur as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<spellfix1_cursor>() as size_t,
        );
        (*pCur).pVTab = p;
        *ppCursor = &raw mut (*pCur).base;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn spellfix1Score(
    mut iDistance: ::core::ffi::c_int,
    mut iRank: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iLog2: ::core::ffi::c_int = 0;
        iLog2 = 0 as ::core::ffi::c_int;
        while iRank > 0 as ::core::ffi::c_int {
            iLog2 += 1;
            iRank >>= 1 as ::core::ffi::c_int;
        }
        return iDistance + 32 as ::core::ffi::c_int - iLog2;
    }
}
unsafe extern "C" fn spellfix1RowCompare(
    mut A: *const ::core::ffi::c_void,
    mut B: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: *const spellfix1_row = A as *const spellfix1_row;
        let mut b: *const spellfix1_row = B as *const spellfix1_row;
        return (*a).iScore - (*b).iScore;
    }
}
unsafe extern "C" fn spellfix1RunQuery(
    mut p: *mut MatchQuery,
    mut zQuery: *const ::core::ffi::c_char,
    mut nQuery: ::core::ffi::c_int,
) {
    unsafe {
        let mut zK1: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zWord: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut iDist: ::core::ffi::c_int = 0;
        let mut iRank: ::core::ffi::c_int = 0;
        let mut iScore: ::core::ffi::c_int = 0;
        let mut iWorst: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut idx: ::core::ffi::c_int = 0;
        let mut idxWorst: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut iScope: ::core::ffi::c_int = (*p).iScope;
        let mut pCur: *mut spellfix1_cursor = (*p).pCur;
        let mut pStmt: *mut sqlite3_stmt = (*p).pStmt;
        let mut zHash1: [::core::ffi::c_char; 32] = [0; 32];
        let mut zHash2: [::core::ffi::c_char; 32] = [0; 32];
        let mut zClass: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nClass: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if (*pCur).a.is_null() || (*p).rc != 0 {
            return;
        }
        zClass = phoneticHash(zQuery as *mut ::core::ffi::c_uchar, nQuery)
            as *mut ::core::ffi::c_char;
        if zClass.is_null() {
            (*p).rc = SQLITE_NOMEM;
            return;
        }
        nClass = strlen(zClass) as ::core::ffi::c_int;
        if nClass > SPELLFIX_MX_HASH - 2 as ::core::ffi::c_int {
            nClass = SPELLFIX_MX_HASH - 2 as ::core::ffi::c_int;
            *zClass.offset(nClass as isize) = 0 as ::core::ffi::c_char;
        }
        if nClass <= iScope {
            if nClass > 2 as ::core::ffi::c_int {
                iScope = nClass - 1 as ::core::ffi::c_int;
            } else {
                iScope = nClass;
            }
        }
        memcpy(
            &raw mut zHash1 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            zClass as *const ::core::ffi::c_void,
            iScope as size_t,
        );
        sqlite3_free(zClass as *mut ::core::ffi::c_void);
        zHash1[iScope as usize] = 0 as ::core::ffi::c_char;
        memcpy(
            &raw mut zHash2 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw mut zHash1 as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            iScope as size_t,
        );
        zHash2[iScope as usize] = 'Z' as i32 as ::core::ffi::c_char;
        zHash2[(iScope + 1 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_char;
        let c2rust_fresh21 = (*p).nRun;
        (*p).nRun = (*p).nRun + 1;
        memcpy(
            &raw mut *(&raw mut (*p).azPrior as *mut [::core::ffi::c_char; 32])
                .offset(c2rust_fresh21 as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            &raw mut zHash1 as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            (iScope + 1 as ::core::ffi::c_int) as size_t,
        );
        if sqlite3_bind_text(
            pStmt,
            1 as ::core::ffi::c_int,
            &raw mut zHash1 as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        ) == SQLITE_NOMEM
            || sqlite3_bind_text(
                pStmt,
                2 as ::core::ffi::c_int,
                &raw mut zHash2 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            ) == SQLITE_NOMEM
        {
            (*p).rc = SQLITE_NOMEM;
            return;
        }
        while sqlite3_step(pStmt) == SQLITE_ROW {
            let mut iMatchlen: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
            iRank = sqlite3_column_int(pStmt, 2 as ::core::ffi::c_int);
            if !(*p).pMatchStr3.is_null() {
                let mut nWord: ::core::ffi::c_int = sqlite3_column_bytes(
                    pStmt,
                    1 as ::core::ffi::c_int,
                );
                zWord = sqlite3_column_text(pStmt, 1 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
                iDist = editDist3Core(
                    (*p).pMatchStr3,
                    zWord,
                    nWord,
                    (*p).pLang,
                    &raw mut iMatchlen,
                );
            } else {
                zK1 = sqlite3_column_text(pStmt, 3 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
                if zK1.is_null() {
                    continue;
                }
                iDist = editdist1(
                    (*p).zPattern,
                    zK1,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
            }
            if iDist < 0 as ::core::ffi::c_int {
                (*p).rc = SQLITE_NOMEM;
                break;
            } else {
                (*pCur).nSearch += 1;
                if (*p).iMaxDist >= 0 as ::core::ffi::c_int {
                    if iDist > (*p).iMaxDist {
                        continue;
                    }
                    if (*pCur).nRow >= (*pCur).nAlloc
                        && (*pCur).idxNum & SPELLFIX_IDXNUM_TOP
                            == 0 as ::core::ffi::c_int
                    {
                        spellfix1ResizeCursor(
                            pCur,
                            (*pCur).nAlloc * 2 as ::core::ffi::c_int
                                + 10 as ::core::ffi::c_int,
                        );
                        if (*pCur).a.is_null() {
                            break;
                        }
                    }
                }
                iScore = spellfix1Score(iDist, iRank);
                if (*pCur).nRow < (*pCur).nAlloc {
                    idx = (*pCur).nRow;
                } else {
                    if !(iScore < iWorst) {
                        continue;
                    }
                    idx = idxWorst;
                    sqlite3_free(
                        (*(*pCur).a.offset(idx as isize)).zWord
                            as *mut ::core::ffi::c_void,
                    );
                }
                let ref mut c2rust_fresh22 = (*(*pCur).a.offset(idx as isize)).zWord;
                *c2rust_fresh22 = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_column_text(pStmt, 1 as ::core::ffi::c_int),
                );
                if (*(*pCur).a.offset(idx as isize)).zWord.is_null() {
                    (*p).rc = SQLITE_NOMEM;
                    break;
                } else {
                    (*(*pCur).a.offset(idx as isize)).iRowid = sqlite3_column_int64(
                        pStmt,
                        0 as ::core::ffi::c_int,
                    );
                    (*(*pCur).a.offset(idx as isize)).iRank = iRank;
                    (*(*pCur).a.offset(idx as isize)).iDistance = iDist;
                    (*(*pCur).a.offset(idx as isize)).iScore = iScore;
                    (*(*pCur).a.offset(idx as isize)).iMatchlen = iMatchlen;
                    memcpy(
                        &raw mut (*(*pCur).a.offset(idx as isize)).zHash
                            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        &raw mut zHash1 as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (iScope + 1 as ::core::ffi::c_int) as size_t,
                    );
                    if (*pCur).nRow < (*pCur).nAlloc {
                        (*pCur).nRow += 1;
                    }
                    if (*pCur).nRow == (*pCur).nAlloc {
                        iWorst = (*(*pCur).a.offset(0 as ::core::ffi::c_int as isize))
                            .iScore;
                        idxWorst = 0 as ::core::ffi::c_int;
                        i = 1 as ::core::ffi::c_int;
                        while i < (*pCur).nRow {
                            iScore = (*(*pCur).a.offset(i as isize)).iScore;
                            if iWorst < iScore {
                                iWorst = iScore;
                                idxWorst = i;
                            }
                            i += 1;
                        }
                    }
                }
            }
        }
        rc = sqlite3_reset(pStmt);
        if rc != 0 {
            (*p).rc = rc;
        }
    }
}
unsafe extern "C" fn spellfix1FilterForMatch(
    mut pCur: *mut spellfix1_cursor,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut idxNum: ::core::ffi::c_int = (*pCur).idxNum;
        let mut zMatchThis: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut pMatchStr3: *mut EditDist3FromString = ::core::ptr::null_mut::<
            EditDist3FromString,
        >();
        let mut zPattern: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nPattern: ::core::ffi::c_int = 0;
        let mut iLimit: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
        let mut iScope: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
        let mut iLang: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut idx: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut p: *mut spellfix1_vtab = (*pCur).pVTab;
        let mut x: MatchQuery = MatchQuery {
            pCur: ::core::ptr::null_mut::<spellfix1_cursor>(),
            pStmt: ::core::ptr::null_mut::<sqlite3_stmt>(),
            zHash: [0; 32],
            zPattern: ::core::ptr::null::<::core::ffi::c_char>(),
            nPattern: 0,
            pMatchStr3: ::core::ptr::null_mut::<EditDist3FromString>(),
            pConfig3: ::core::ptr::null_mut::<EditDist3Config>(),
            pLang: ::core::ptr::null::<EditDist3Lang>(),
            iLang: 0,
            iScope: 0,
            iMaxDist: 0,
            rc: 0,
            nRun: 0,
            azPrior: [[0; 32]; 1],
        };
        if !(*p).zCostTable.is_null() && (*p).pConfig3.is_null() {
            (*p).pConfig3 = sqlite3_malloc64(
                ::core::mem::size_of::<EditDist3Config>() as sqlite3_uint64,
            ) as *mut EditDist3Config;
            if (*p).pConfig3.is_null() {
                return SQLITE_NOMEM;
            }
            memset(
                (*p).pConfig3 as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<EditDist3Config>() as size_t,
            );
            rc = editDist3ConfigLoad((*p).pConfig3, (*p).db, (*p).zCostTable);
            if rc != 0 {
                return rc;
            }
        }
        memset(
            &raw mut x as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<MatchQuery>() as size_t,
        );
        x.iScope = 3 as ::core::ffi::c_int;
        x.iMaxDist = -1 as ::core::ffi::c_int;
        if idxNum & 2 as ::core::ffi::c_int != 0 {
            let c2rust_fresh23 = idx;
            idx = idx + 1;
            iLang = sqlite3_value_int(*argv.offset(c2rust_fresh23 as isize));
        }
        if idxNum & 4 as ::core::ffi::c_int != 0 {
            let c2rust_fresh24 = idx;
            idx = idx + 1;
            iLimit = sqlite3_value_int(*argv.offset(c2rust_fresh24 as isize));
            if iLimit < 1 as ::core::ffi::c_int {
                iLimit = 1 as ::core::ffi::c_int;
            }
        }
        if idxNum & 8 as ::core::ffi::c_int != 0 {
            let c2rust_fresh25 = idx;
            idx = idx + 1;
            x.iScope = sqlite3_value_int(*argv.offset(c2rust_fresh25 as isize));
            if x.iScope < 1 as ::core::ffi::c_int {
                x.iScope = 1 as ::core::ffi::c_int;
            }
            if x.iScope > SPELLFIX_MX_HASH - 2 as ::core::ffi::c_int {
                x.iScope = SPELLFIX_MX_HASH - 2 as ::core::ffi::c_int;
            }
        }
        if idxNum & (16 as ::core::ffi::c_int | 32 as ::core::ffi::c_int) != 0 {
            let c2rust_fresh26 = idx;
            idx = idx + 1;
            x.iMaxDist = sqlite3_value_int(*argv.offset(c2rust_fresh26 as isize));
            if idxNum & 16 as ::core::ffi::c_int != 0 {
                x.iMaxDist -= 1;
            }
            if x.iMaxDist < 0 as ::core::ffi::c_int {
                x.iMaxDist = 0 as ::core::ffi::c_int;
            }
        }
        spellfix1ResetCursor(pCur);
        spellfix1ResizeCursor(pCur, iLimit);
        zMatchThis = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        if zMatchThis.is_null() {
            return SQLITE_OK;
        }
        if !(*p).pConfig3.is_null() {
            x.pLang = editDist3FindLang((*p).pConfig3, iLang);
            pMatchStr3 = editDist3FromStringNew(
                x.pLang,
                zMatchThis as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            );
            if pMatchStr3.is_null() {
                x.rc = SQLITE_NOMEM;
                c2rust_current_block = 5663109960237200556;
            } else {
                c2rust_current_block = 3934796541983872331;
            }
        } else {
            x.pLang = ::core::ptr::null::<EditDist3Lang>();
            c2rust_current_block = 3934796541983872331;
        }
        match c2rust_current_block {
            3934796541983872331 => {
                zPattern = transliterate(
                    zMatchThis,
                    sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize)),
                ) as *mut ::core::ffi::c_char;
                sqlite3_free((*pCur).zPattern as *mut ::core::ffi::c_void);
                (*pCur).zPattern = zPattern;
                if zPattern.is_null() {
                    x.rc = SQLITE_NOMEM;
                } else {
                    nPattern = strlen(zPattern) as ::core::ffi::c_int;
                    if *zPattern.offset((nPattern - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == '*' as i32
                    {
                        nPattern -= 1;
                    }
                    zSql = sqlite3_mprintf(
                        b"SELECT id, word, rank, coalesce(k1,word)  FROM \"%w\".\"%w_vocab\" WHERE langid=%d AND k2>=?1 AND k2<?2\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*p).zDbName,
                        (*p).zTableName,
                        iLang,
                    );
                    if zSql.is_null() {
                        x.rc = SQLITE_NOMEM;
                        pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                    } else {
                        rc = sqlite3_prepare_v2(
                            (*p).db,
                            zSql,
                            -1 as ::core::ffi::c_int,
                            &raw mut pStmt,
                            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                        );
                        sqlite3_free(zSql as *mut ::core::ffi::c_void);
                        (*pCur).iLang = iLang;
                        x.pCur = pCur;
                        x.pStmt = pStmt;
                        x.zPattern = zPattern;
                        x.nPattern = nPattern;
                        x.pMatchStr3 = pMatchStr3;
                        x.iLang = iLang;
                        x.rc = rc;
                        x.pConfig3 = (*p).pConfig3;
                        if x.rc == SQLITE_OK {
                            spellfix1RunQuery(&raw mut x, zPattern, nPattern);
                        }
                        if !(*pCur).a.is_null() {
                            qsort(
                                (*pCur).a as *mut ::core::ffi::c_void,
                                (*pCur).nRow as size_t,
                                ::core::mem::size_of::<spellfix1_row>() as size_t,
                                Some(
                                    spellfix1RowCompare
                                        as unsafe extern "C" fn(
                                            *const ::core::ffi::c_void,
                                            *const ::core::ffi::c_void,
                                        ) -> ::core::ffi::c_int,
                                ),
                            );
                            (*pCur).iTop = iLimit;
                            (*pCur).iScope = iScope;
                        } else {
                            x.rc = SQLITE_NOMEM;
                        }
                    }
                }
            }
            _ => {}
        }
        sqlite3_finalize(pStmt);
        editDist3FromStringDelete(pMatchStr3);
        return x.rc;
    }
}
unsafe extern "C" fn spellfix1FilterForFullScan(
    mut pCur: *mut spellfix1_cursor,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut idxNum: ::core::ffi::c_int = (*pCur).idxNum;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pVTab: *mut spellfix1_vtab = (*pCur).pVTab;
        spellfix1ResetCursor(pCur);
        zSql = sqlite3_mprintf(
            b"SELECT word, rank, NULL, langid, id FROM \"%w\".\"%w_vocab\"%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*pVTab).zDbName,
            (*pVTab).zTableName,
            if idxNum & 64 as ::core::ffi::c_int != 0 {
                b" WHERE rowid=?\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if zSql.is_null() {
            return SQLITE_NOMEM;
        }
        rc = sqlite3_prepare_v2(
            (*pVTab).db,
            zSql,
            -1 as ::core::ffi::c_int,
            &raw mut (*pCur).pFullScan,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        if rc == SQLITE_OK && idxNum & 64 as ::core::ffi::c_int != 0 {
            rc = sqlite3_bind_value(
                (*pCur).pFullScan,
                1 as ::core::ffi::c_int,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
        }
        (*pCur).iRow = 0 as ::core::ffi::c_int;
        (*pCur).nRow = (*pCur).iRow;
        if rc == SQLITE_OK {
            rc = sqlite3_step((*pCur).pFullScan);
            if rc == SQLITE_ROW {
                (*pCur).iRow = -1 as ::core::ffi::c_int;
                rc = SQLITE_OK;
            }
            if rc == SQLITE_DONE {
                rc = SQLITE_OK;
            }
        } else {
            (*pCur).iRow = 0 as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn spellfix1Filter(
    mut cur: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut spellfix1_cursor = cur as *mut spellfix1_cursor;
        let mut rc: ::core::ffi::c_int = 0;
        (*pCur).idxNum = idxNum;
        if idxNum & 1 as ::core::ffi::c_int != 0 {
            rc = spellfix1FilterForMatch(pCur, argc, argv);
        } else {
            rc = spellfix1FilterForFullScan(pCur, argc, argv);
        }
        return rc;
    }
}
unsafe extern "C" fn spellfix1Next(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut spellfix1_cursor = cur as *mut spellfix1_cursor;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pCur).iRow < (*pCur).nRow {
            if !(*pCur).pFullScan.is_null() {
                rc = sqlite3_step((*pCur).pFullScan);
                if rc != SQLITE_ROW {
                    (*pCur).iRow = (*pCur).nRow;
                }
                if rc == SQLITE_ROW || rc == SQLITE_DONE {
                    rc = SQLITE_OK;
                }
            } else {
                (*pCur).iRow += 1;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn spellfix1Eof(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut spellfix1_cursor = cur as *mut spellfix1_cursor;
        return ((*pCur).iRow >= (*pCur).nRow) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn spellfix1Column(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut spellfix1_cursor = cur as *mut spellfix1_cursor;
        if !(*pCur).pFullScan.is_null() {
            if i <= SPELLFIX_COL_LANGID {
                sqlite3_result_value(ctx, sqlite3_column_value((*pCur).pFullScan, i));
            } else {
                sqlite3_result_null(ctx);
            }
            return SQLITE_OK;
        }
        match i {
            SPELLFIX_COL_WORD => {
                sqlite3_result_text(
                    ctx,
                    (*(*pCur).a.offset((*pCur).iRow as isize)).zWord,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            SPELLFIX_COL_RANK => {
                sqlite3_result_int(
                    ctx,
                    (*(*pCur).a.offset((*pCur).iRow as isize)).iRank,
                );
            }
            SPELLFIX_COL_DISTANCE => {
                sqlite3_result_int(
                    ctx,
                    (*(*pCur).a.offset((*pCur).iRow as isize)).iDistance,
                );
            }
            SPELLFIX_COL_LANGID => {
                sqlite3_result_int(ctx, (*pCur).iLang);
            }
            SPELLFIX_COL_SCORE => {
                sqlite3_result_int(
                    ctx,
                    (*(*pCur).a.offset((*pCur).iRow as isize)).iScore,
                );
            }
            SPELLFIX_COL_MATCHLEN => {
                let mut iMatchlen: ::core::ffi::c_int = (*(*pCur)
                    .a
                    .offset((*pCur).iRow as isize))
                    .iMatchlen;
                if iMatchlen < 0 as ::core::ffi::c_int {
                    let mut nPattern: ::core::ffi::c_int = strlen((*pCur).zPattern)
                        as ::core::ffi::c_int;
                    let mut zWord: *mut ::core::ffi::c_char = (*(*pCur)
                        .a
                        .offset((*pCur).iRow as isize))
                        .zWord;
                    let mut nWord: ::core::ffi::c_int = strlen(zWord)
                        as ::core::ffi::c_int;
                    if nPattern > 0 as ::core::ffi::c_int
                        && *(*pCur)
                            .zPattern
                            .offset((nPattern - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == '*' as i32
                    {
                        let mut zTranslit: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        let mut res: ::core::ffi::c_int = 0;
                        zTranslit = transliterate(
                            zWord as *mut ::core::ffi::c_uchar,
                            nWord,
                        ) as *mut ::core::ffi::c_char;
                        if zTranslit.is_null() {
                            return SQLITE_NOMEM;
                        }
                        res = editdist1((*pCur).zPattern, zTranslit, &raw mut iMatchlen);
                        sqlite3_free(zTranslit as *mut ::core::ffi::c_void);
                        if res < 0 as ::core::ffi::c_int {
                            return SQLITE_NOMEM;
                        }
                        iMatchlen = translen_to_charlen(zWord, nWord, iMatchlen);
                    } else {
                        iMatchlen = utf8Charlen(zWord, nWord);
                    }
                }
                sqlite3_result_int(ctx, iMatchlen);
            }
            SPELLFIX_COL_PHONEHASH => {
                sqlite3_result_text(
                    ctx,
                    &raw mut (*(*pCur).a.offset((*pCur).iRow as isize)).zHash
                        as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            SPELLFIX_COL_TOP => {
                sqlite3_result_int(ctx, (*pCur).iTop);
            }
            SPELLFIX_COL_SCOPE => {
                sqlite3_result_int(ctx, (*pCur).iScope);
            }
            SPELLFIX_COL_SRCHCNT => {
                sqlite3_result_int(ctx, (*pCur).nSearch);
            }
            _ => {
                sqlite3_result_null(ctx);
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn spellfix1Rowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut spellfix1_cursor = cur as *mut spellfix1_cursor;
        if !(*pCur).pFullScan.is_null() {
            *pRowid = sqlite3_column_int64((*pCur).pFullScan, 4 as ::core::ffi::c_int)
                as sqlite_int64;
        } else {
            *pRowid = (*(*pCur).a.offset((*pCur).iRow as isize)).iRowid as sqlite_int64;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn spellfix1GetConflict(
    mut db: *mut sqlite3,
) -> *const ::core::ffi::c_char {
    unsafe {
        static mut azConflict: [*const ::core::ffi::c_char; 5] = [
            b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char,
            b"IGNORE\0".as_ptr() as *const ::core::ffi::c_char,
            b"ABORT\0".as_ptr() as *const ::core::ffi::c_char,
            b"ABORT\0".as_ptr() as *const ::core::ffi::c_char,
            b"REPLACE\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut eConflict: ::core::ffi::c_int = sqlite3_vtab_on_conflict(db);
        return azConflict[(eConflict - 1 as ::core::ffi::c_int) as usize];
    }
}
unsafe extern "C" fn spellfix1Update(
    mut pVTab: *mut sqlite3_vtab,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut rowid: sqlite3_int64 = 0;
        let mut newRowid: sqlite3_int64 = 0;
        let mut p: *mut spellfix1_vtab = pVTab as *mut spellfix1_vtab;
        let mut db: *mut sqlite3 = (*p).db;
        if argc == 1 as ::core::ffi::c_int {
            *pRowid = sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize))
                as sqlite_int64;
            rowid = *pRowid as sqlite3_int64;
            spellfix1DbExec(
                &raw mut rc,
                db,
                b"DELETE FROM \"%w\".\"%w_vocab\"  WHERE id=%lld\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zDbName,
                (*p).zTableName,
                rowid,
            );
        } else {
            let mut zWord: *const ::core::ffi::c_uchar = sqlite3_value_text(
                *argv.offset((SPELLFIX_COL_WORD + 2 as ::core::ffi::c_int) as isize),
            );
            let mut nWord: ::core::ffi::c_int = sqlite3_value_bytes(
                *argv.offset((SPELLFIX_COL_WORD + 2 as ::core::ffi::c_int) as isize),
            );
            let mut iLang: ::core::ffi::c_int = sqlite3_value_int(
                *argv.offset((SPELLFIX_COL_LANGID + 2 as ::core::ffi::c_int) as isize),
            );
            let mut iRank: ::core::ffi::c_int = sqlite3_value_int(
                *argv.offset((SPELLFIX_COL_RANK + 2 as ::core::ffi::c_int) as isize),
            );
            let mut zSoundslike: *const ::core::ffi::c_uchar = sqlite3_value_text(
                *argv
                    .offset((SPELLFIX_COL_SOUNDSLIKE + 2 as ::core::ffi::c_int) as isize),
            );
            let mut nSoundslike: ::core::ffi::c_int = sqlite3_value_bytes(
                *argv
                    .offset((SPELLFIX_COL_SOUNDSLIKE + 2 as ::core::ffi::c_int) as isize),
            );
            let mut zK1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut zK2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut i: ::core::ffi::c_int = 0;
            let mut c: ::core::ffi::c_char = 0;
            let mut zConflict: *const ::core::ffi::c_char = spellfix1GetConflict(db);
            if zWord.is_null() {
                let mut zCmd: *const ::core::ffi::c_char = sqlite3_value_text(
                    *argv
                        .offset(
                            (SPELLFIX_COL_COMMAND + 2 as ::core::ffi::c_int) as isize,
                        ),
                ) as *const ::core::ffi::c_char;
                if zCmd.is_null() {
                    (*pVTab).zErrMsg = sqlite3_mprintf(
                        b"NOT NULL constraint failed: %s.word\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*p).zTableName,
                    );
                    return SQLITE_CONSTRAINT_NOTNULL;
                }
                if strcmp(zCmd, b"reset\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    editDist3ConfigDelete((*p).pConfig3 as *mut ::core::ffi::c_void);
                    (*p).pConfig3 = ::core::ptr::null_mut::<EditDist3Config>();
                    return SQLITE_OK;
                }
                if strncmp(
                    zCmd,
                    b"edit_cost_table=\0".as_ptr() as *const ::core::ffi::c_char,
                    16 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    editDist3ConfigDelete((*p).pConfig3 as *mut ::core::ffi::c_void);
                    (*p).pConfig3 = ::core::ptr::null_mut::<EditDist3Config>();
                    sqlite3_free((*p).zCostTable as *mut ::core::ffi::c_void);
                    (*p).zCostTable = spellfix1Dequote(
                        zCmd.offset(16 as ::core::ffi::c_int as isize),
                    );
                    if (*p).zCostTable.is_null() {
                        return SQLITE_NOMEM;
                    }
                    if *(*p).zCostTable.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || sqlite3_stricmp(
                            (*p).zCostTable,
                            b"null\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        sqlite3_free((*p).zCostTable as *mut ::core::ffi::c_void);
                        (*p).zCostTable = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    return SQLITE_OK;
                }
                (*pVTab).zErrMsg = sqlite3_mprintf(
                    b"unknown value for %s.command: \"%w\"\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*p).zTableName,
                    zCmd,
                );
                return SQLITE_ERROR;
            }
            if iRank < 1 as ::core::ffi::c_int {
                iRank = 1 as ::core::ffi::c_int;
            }
            if !zSoundslike.is_null() {
                zK1 = transliterate(zSoundslike, nSoundslike)
                    as *mut ::core::ffi::c_char;
            } else {
                zK1 = transliterate(zWord, nWord) as *mut ::core::ffi::c_char;
            }
            if zK1.is_null() {
                return SQLITE_NOMEM;
            }
            i = 0 as ::core::ffi::c_int;
            loop {
                c = *zK1.offset(i as isize);
                if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                    break;
                }
                if c as ::core::ffi::c_int >= 'A' as i32
                    && c as ::core::ffi::c_int <= 'Z' as i32
                {
                    let ref mut c2rust_fresh27 = *zK1.offset(i as isize);
                    *c2rust_fresh27 = (*c2rust_fresh27 as ::core::ffi::c_int
                        + ('a' as i32 - 'A' as i32)) as ::core::ffi::c_char;
                }
                i += 1;
            }
            zK2 = phoneticHash(zK1 as *const ::core::ffi::c_uchar, i)
                as *mut ::core::ffi::c_char;
            if zK2.is_null() {
                sqlite3_free(zK1 as *mut ::core::ffi::c_void);
                return SQLITE_NOMEM;
            }
            if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
                == SQLITE_NULL
            {
                if sqlite3_value_type(*argv.offset(1 as ::core::ffi::c_int as isize))
                    == SQLITE_NULL
                {
                    spellfix1DbExec(
                        &raw mut rc,
                        db,
                        b"INSERT INTO \"%w\".\"%w_vocab\"(rank,langid,word,k1,k2) VALUES(%d,%d,%Q,nullif(%Q,%Q),%Q)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*p).zDbName,
                        (*p).zTableName,
                        iRank,
                        iLang,
                        zWord,
                        zK1,
                        zWord,
                        zK2,
                    );
                } else {
                    newRowid = sqlite3_value_int64(
                        *argv.offset(1 as ::core::ffi::c_int as isize),
                    );
                    spellfix1DbExec(
                        &raw mut rc,
                        db,
                        b"INSERT OR %s INTO \"%w\".\"%w_vocab\"(id,rank,langid,word,k1,k2) VALUES(%lld,%d,%d,%Q,nullif(%Q,%Q),%Q)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        zConflict,
                        (*p).zDbName,
                        (*p).zTableName,
                        newRowid,
                        iRank,
                        iLang,
                        zWord,
                        zK1,
                        zWord,
                        zK2,
                    );
                }
                *pRowid = sqlite3_last_insert_rowid(db) as sqlite_int64;
            } else {
                rowid = sqlite3_value_int64(
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
                *pRowid = sqlite3_value_int64(
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                ) as sqlite_int64;
                newRowid = *pRowid as sqlite3_int64;
                spellfix1DbExec(
                    &raw mut rc,
                    db,
                    b"UPDATE OR %s \"%w\".\"%w_vocab\" SET id=%lld, rank=%d, langid=%d, word=%Q, k1=nullif(%Q,%Q), k2=%Q WHERE id=%lld\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    zConflict,
                    (*p).zDbName,
                    (*p).zTableName,
                    newRowid,
                    iRank,
                    iLang,
                    zWord,
                    zK1,
                    zWord,
                    zK2,
                    rowid,
                );
            }
            sqlite3_free(zK1 as *mut ::core::ffi::c_void);
            sqlite3_free(zK2 as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn spellfix1Rename(
    mut pVTab: *mut sqlite3_vtab,
    mut zNew: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut spellfix1_vtab = pVTab as *mut spellfix1_vtab;
        let mut db: *mut sqlite3 = (*p).db;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zNewName: *mut ::core::ffi::c_char = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            zNew,
        );
        if zNewName.is_null() {
            return SQLITE_NOMEM;
        }
        spellfix1DbExec(
            &raw mut rc,
            db,
            b"ALTER TABLE \"%w\".\"%w_vocab\" RENAME TO \"%w_vocab\"\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*p).zDbName,
            (*p).zTableName,
            zNewName,
        );
        if rc == SQLITE_OK {
            sqlite3_free((*p).zTableName as *mut ::core::ffi::c_void);
            (*p).zTableName = zNewName;
        } else {
            sqlite3_free(zNewName as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
static mut spellfix1Module: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: Some(
            spellfix1Create
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
            spellfix1Connect
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
            spellfix1BestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            spellfix1Disconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            spellfix1Destroy
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            spellfix1Open
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            spellfix1Close
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            spellfix1Filter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            spellfix1Next
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            spellfix1Eof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            spellfix1Column
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            spellfix1Rowid
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: Some(
            spellfix1Update
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
        xRename: Some(
            spellfix1Rename
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSavepoint: None,
        xRelease: None,
        xRollbackTo: None,
        xShadowName: None,
        xIntegrity: None,
    }
};
unsafe extern "C" fn spellfix1Register(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_uint = 0;
        rc = sqlite3_create_function(
            db,
            b"spellfix1_translit\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            SQLITE_UTF8 | SQLITE_DETERMINISTIC,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                transliterateSqlFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
        if rc == SQLITE_OK {
            rc = sqlite3_create_function(
                db,
                b"spellfix1_editdist\0".as_ptr() as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
                SQLITE_UTF8 | SQLITE_DETERMINISTIC,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    editdistSqlFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                None,
                None,
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3_create_function(
                db,
                b"spellfix1_phonehash\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8 | SQLITE_DETERMINISTIC,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    phoneticHashSqlFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                None,
                None,
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3_create_function(
                db,
                b"spellfix1_scriptcode\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8 | SQLITE_DETERMINISTIC,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    scriptCodeSqlFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                None,
                None,
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3_create_module(
                db,
                b"spellfix1\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut spellfix1Module,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        if rc == SQLITE_OK {
            rc = editDist3Install(db);
        }
        i = 0 as ::core::ffi::c_uint;
        while (i as usize)
            < (::core::mem::size_of::<[Transliteration; 389]>() as usize)
                .wrapping_div(::core::mem::size_of::<Transliteration>() as usize)
                .wrapping_sub(1 as usize)
        {
            i = i.wrapping_add(1);
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_spellfix_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        return spellfix1Register(db);
    }
}
