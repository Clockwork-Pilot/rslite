unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_str;
    pub type sqlite3_blob;
    pub type sqlite3_backup;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
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
    fn sqlite3_create_window_function(
        db: *mut sqlite3,
        zFunctionName: *const ::core::ffi::c_char,
        nArg: ::core::ffi::c_int,
        eTextRep: ::core::ffi::c_int,
        pApp: *mut ::core::ffi::c_void,
        xStep: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
        xValue: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
        xInverse: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_aggregate_context(
        _: *mut sqlite3_context,
        nBytes: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_null(_: *mut sqlite3_context);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_create_collation(
        _: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
        eTextRep: ::core::ffi::c_int,
        pArg: *mut ::core::ffi::c_void,
        xCompare: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
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
pub struct Decimal {
    pub sign: ::core::ffi::c_char,
    pub oom: ::core::ffi::c_char,
    pub isNull: ::core::ffi::c_char,
    pub isInit: ::core::ffi::c_char,
    pub nDigit: ::core::ffi::c_int,
    pub nFrac: ::core::ffi::c_int,
    pub a: *mut ::core::ffi::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub zFuncName: *const ::core::ffi::c_char,
    pub nArg: ::core::ffi::c_int,
    pub iArg: ::core::ffi::c_int,
    pub xFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4;
pub const SQLITE_NULL: ::core::ffi::c_int = 5;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_INNOCUOUS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
unsafe extern "C" fn decimal_clear(mut p: *mut Decimal) {
    unsafe {
        sqlite3_free((*p).a as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn decimal_free(mut p: *mut Decimal) {
    unsafe {
        if !p.is_null() {
            decimal_clear(p);
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn decimalNewFromText(
    mut zIn: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> *mut Decimal {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut p: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        let mut i: ::core::ffi::c_int = 0;
        let mut iExp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        p = sqlite3_malloc(::core::mem::size_of::<Decimal>() as ::core::ffi::c_int)
            as *mut Decimal;
        if !p.is_null() {
            (*p).sign = 0 as ::core::ffi::c_char;
            (*p).oom = 0 as ::core::ffi::c_char;
            (*p).isInit = 1 as ::core::ffi::c_char;
            (*p).isNull = 0 as ::core::ffi::c_char;
            (*p).nDigit = 0 as ::core::ffi::c_int;
            (*p).nFrac = 0 as ::core::ffi::c_int;
            (*p).a = sqlite3_malloc64((n + 1 as ::core::ffi::c_int) as sqlite3_uint64)
                as *mut ::core::ffi::c_schar;
            if !(*p).a.is_null() {
                i = 0 as ::core::ffi::c_int;
                while *(*__ctype_b_loc())
                    .offset(
                        *zIn.offset(i as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int as isize,
                    ) as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
                {
                    i += 1;
                }
                if *zIn.offset(i as isize) as ::core::ffi::c_int == '-' as i32 {
                    (*p).sign = 1 as ::core::ffi::c_char;
                    i += 1;
                } else if *zIn.offset(i as isize) as ::core::ffi::c_int == '+' as i32 {
                    i += 1;
                }
                while i < n
                    && *zIn.offset(i as isize) as ::core::ffi::c_int == '0' as i32
                {
                    i += 1;
                }
                while i < n {
                    let mut c: ::core::ffi::c_char = *zIn.offset(i as isize);
                    if c as ::core::ffi::c_int >= '0' as i32
                        && c as ::core::ffi::c_int <= '9' as i32
                    {
                        let c2rust_fresh0 = (*p).nDigit;
                        (*p).nDigit = (*p).nDigit + 1;
                        *(*p).a.offset(c2rust_fresh0 as isize) = (c as ::core::ffi::c_int
                            - '0' as i32) as ::core::ffi::c_schar;
                    } else if c as ::core::ffi::c_int == '.' as i32 {
                        (*p).nFrac = (*p).nDigit + 1 as ::core::ffi::c_int;
                    } else if c as ::core::ffi::c_int == 'e' as i32
                        || c as ::core::ffi::c_int == 'E' as i32
                    {
                        let mut j: ::core::ffi::c_int = i + 1 as ::core::ffi::c_int;
                        let mut neg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        if j >= n {
                            break;
                        }
                        if *zIn.offset(j as isize) as ::core::ffi::c_int == '-' as i32 {
                            neg = 1 as ::core::ffi::c_int;
                            j += 1;
                        } else if *zIn.offset(j as isize) as ::core::ffi::c_int
                            == '+' as i32
                        {
                            j += 1;
                        }
                        while j < n && iExp < 1000000 as ::core::ffi::c_int {
                            if *zIn.offset(j as isize) as ::core::ffi::c_int
                                >= '0' as i32
                                && *zIn.offset(j as isize) as ::core::ffi::c_int
                                    <= '9' as i32
                            {
                                iExp = iExp * 10 as ::core::ffi::c_int
                                    + *zIn.offset(j as isize) as ::core::ffi::c_int
                                    - '0' as i32;
                            }
                            j += 1;
                        }
                        if neg != 0 {
                            iExp = -iExp;
                        }
                        break;
                    }
                    i += 1;
                }
                if (*p).nFrac != 0 {
                    (*p).nFrac = (*p).nDigit - ((*p).nFrac - 1 as ::core::ffi::c_int);
                }
                if iExp > 0 as ::core::ffi::c_int {
                    if (*p).nFrac > 0 as ::core::ffi::c_int {
                        if iExp <= (*p).nFrac {
                            (*p).nFrac -= iExp;
                            iExp = 0 as ::core::ffi::c_int;
                        } else {
                            iExp -= (*p).nFrac;
                            (*p).nFrac = 0 as ::core::ffi::c_int;
                        }
                    }
                    if iExp > 0 as ::core::ffi::c_int {
                        (*p).a = sqlite3_realloc64(
                            (*p).a as *mut ::core::ffi::c_void,
                            ((*p).nDigit as ::core::ffi::c_longlong
                                + iExp as ::core::ffi::c_longlong
                                + 1 as ::core::ffi::c_longlong) as sqlite3_uint64,
                        ) as *mut ::core::ffi::c_schar;
                        if (*p).a.is_null() {
                            c2rust_current_block = 8992291143372979280;
                        } else {
                            memset(
                                (*p).a.offset((*p).nDigit as isize)
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                iExp as size_t,
                            );
                            (*p).nDigit += iExp;
                            c2rust_current_block = 7419121793134201633;
                        }
                    } else {
                        c2rust_current_block = 7419121793134201633;
                    }
                } else if iExp < 0 as ::core::ffi::c_int {
                    let mut nExtra: ::core::ffi::c_int = 0;
                    iExp = -iExp;
                    nExtra = (*p).nDigit - (*p).nFrac - 1 as ::core::ffi::c_int;
                    if nExtra != 0 {
                        if nExtra >= iExp {
                            (*p).nFrac += iExp;
                            iExp = 0 as ::core::ffi::c_int;
                        } else {
                            iExp -= nExtra;
                            (*p).nFrac = (*p).nDigit - 1 as ::core::ffi::c_int;
                        }
                    }
                    if iExp > 0 as ::core::ffi::c_int {
                        (*p).a = sqlite3_realloc64(
                            (*p).a as *mut ::core::ffi::c_void,
                            ((*p).nDigit as ::core::ffi::c_longlong
                                + iExp as ::core::ffi::c_longlong
                                + 1 as ::core::ffi::c_longlong) as sqlite3_uint64,
                        ) as *mut ::core::ffi::c_schar;
                        if (*p).a.is_null() {
                            c2rust_current_block = 8992291143372979280;
                        } else {
                            memmove(
                                (*p).a.offset(iExp as isize) as *mut ::core::ffi::c_void,
                                (*p).a as *const ::core::ffi::c_void,
                                (*p).nDigit as size_t,
                            );
                            memset(
                                (*p).a as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                iExp as size_t,
                            );
                            (*p).nDigit += iExp;
                            (*p).nFrac += iExp;
                            c2rust_current_block = 7419121793134201633;
                        }
                    } else {
                        c2rust_current_block = 7419121793134201633;
                    }
                } else {
                    c2rust_current_block = 7419121793134201633;
                }
                match c2rust_current_block {
                    8992291143372979280 => {}
                    _ => {
                        if (*p).sign != 0 {
                            i = 0 as ::core::ffi::c_int;
                            while i < (*p).nDigit
                                && *(*p).a.offset(i as isize) as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                            {
                                i += 1;
                            }
                            if i >= (*p).nDigit {
                                (*p).sign = 0 as ::core::ffi::c_char;
                            }
                        }
                        return p;
                    }
                }
            }
        }
        if !p.is_null() {
            if !(*p).a.is_null() {
                sqlite3_free((*p).a as *mut ::core::ffi::c_void);
            }
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<Decimal>();
    }
}
unsafe extern "C" fn decimal_new(
    mut pCtx: *mut sqlite3_context,
    mut pIn: *mut sqlite3_value,
    mut bTextOnly: ::core::ffi::c_int,
) -> *mut Decimal {
    unsafe {
        let mut p: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        let mut eType: ::core::ffi::c_int = sqlite3_value_type(pIn);
        if bTextOnly != 0 && (eType == SQLITE_FLOAT || eType == SQLITE_BLOB) {
            eType = SQLITE_TEXT;
        }
        match eType {
            SQLITE_TEXT | SQLITE_INTEGER => {
                let mut zIn: *const ::core::ffi::c_char = sqlite3_value_text(pIn)
                    as *const ::core::ffi::c_char;
                let mut n: ::core::ffi::c_int = sqlite3_value_bytes(pIn);
                p = decimalNewFromText(zIn, n);
                if p.is_null() {
                    if !pCtx.is_null() {
                        sqlite3_result_error_nomem(pCtx);
                    }
                    sqlite3_free(p as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<Decimal>();
                }
            }
            SQLITE_FLOAT => {
                p = decimalFromDouble(sqlite3_value_double(pIn));
            }
            SQLITE_BLOB => {
                let mut x: *const ::core::ffi::c_uchar = ::core::ptr::null::<
                    ::core::ffi::c_uchar,
                >();
                let mut i: ::core::ffi::c_uint = 0;
                let mut v: sqlite3_uint64 = 0 as sqlite3_uint64;
                let mut r: ::core::ffi::c_double = 0.;
                if !(sqlite3_value_bytes(pIn) as usize
                    != ::core::mem::size_of::<::core::ffi::c_double>() as usize)
                {
                    x = sqlite3_value_blob(pIn) as *const ::core::ffi::c_uchar;
                    i = 0 as ::core::ffi::c_uint;
                    while (i as usize)
                        < ::core::mem::size_of::<::core::ffi::c_double>() as usize
                    {
                        v = ((v as ::core::ffi::c_ulonglong) << 8 as ::core::ffi::c_int
                            | *x.offset(i as isize) as ::core::ffi::c_ulonglong)
                            as sqlite3_uint64;
                        i = i.wrapping_add(1);
                    }
                    memcpy(
                        &raw mut r as *mut ::core::ffi::c_void,
                        &raw mut v as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                    );
                    p = decimalFromDouble(r);
                }
            }
            SQLITE_NULL | _ => {}
        }
        return p;
    }
}
unsafe extern "C" fn decimal_result(
    mut pCtx: *mut sqlite3_context,
    mut p: *mut Decimal,
) {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        if p.is_null() || (*p).oom as ::core::ffi::c_int != 0 {
            sqlite3_result_error_nomem(pCtx);
            return;
        }
        if (*p).isNull != 0 {
            sqlite3_result_null(pCtx);
            return;
        }
        z = sqlite3_malloc64(
            ((*p).nDigit as ::core::ffi::c_longlong + 4 as ::core::ffi::c_longlong)
                as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if z.is_null() {
            sqlite3_result_error_nomem(pCtx);
            return;
        }
        i = 0 as ::core::ffi::c_int;
        if (*p).nDigit == 0 as ::core::ffi::c_int
            || (*p).nDigit == 1 as ::core::ffi::c_int
                && *(*p).a.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
        {
            (*p).sign = 0 as ::core::ffi::c_char;
        }
        if (*p).sign != 0 {
            *z.offset(0 as ::core::ffi::c_int as isize) = '-' as i32
                as ::core::ffi::c_char;
            i = 1 as ::core::ffi::c_int;
        }
        n = (*p).nDigit - (*p).nFrac;
        if n <= 0 as ::core::ffi::c_int {
            let c2rust_fresh2 = i;
            i = i + 1;
            *z.offset(c2rust_fresh2 as isize) = '0' as i32 as ::core::ffi::c_char;
        }
        j = 0 as ::core::ffi::c_int;
        while n > 1 as ::core::ffi::c_int
            && *(*p).a.offset(j as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            j += 1;
            n -= 1;
        }
        while n > 0 as ::core::ffi::c_int {
            let c2rust_fresh3 = i;
            i = i + 1;
            *z.offset(c2rust_fresh3 as isize) = (*(*p).a.offset(j as isize)
                as ::core::ffi::c_int + '0' as i32) as ::core::ffi::c_char;
            j += 1;
            n -= 1;
        }
        if (*p).nFrac != 0 {
            let c2rust_fresh4 = i;
            i = i + 1;
            *z.offset(c2rust_fresh4 as isize) = '.' as i32 as ::core::ffi::c_char;
            loop {
                let c2rust_fresh5 = i;
                i = i + 1;
                *z.offset(c2rust_fresh5 as isize) = (*(*p).a.offset(j as isize)
                    as ::core::ffi::c_int + '0' as i32) as ::core::ffi::c_char;
                j += 1;
                if !(j < (*p).nDigit) {
                    break;
                }
            }
        }
        *z.offset(i as isize) = 0 as ::core::ffi::c_char;
        sqlite3_result_text(
            pCtx,
            z,
            i,
            Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    }
}
unsafe extern "C" fn decimal_result_sci(
    mut pCtx: *mut sqlite3_context,
    mut p: *mut Decimal,
) {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut nZero: ::core::ffi::c_int = 0;
        let mut nDigit: ::core::ffi::c_int = 0;
        let mut nFrac: ::core::ffi::c_int = 0;
        let mut exp: ::core::ffi::c_int = 0;
        let mut zero: ::core::ffi::c_schar = 0;
        let mut a: *mut ::core::ffi::c_schar = ::core::ptr::null_mut::<
            ::core::ffi::c_schar,
        >();
        if p.is_null() || (*p).oom as ::core::ffi::c_int != 0 {
            sqlite3_result_error_nomem(pCtx);
            return;
        }
        if (*p).isNull != 0 {
            sqlite3_result_null(pCtx);
            return;
        }
        nDigit = (*p).nDigit;
        while nDigit > 0 as ::core::ffi::c_int
            && *(*p).a.offset((nDigit - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            nDigit -= 1;
        }
        nZero = 0 as ::core::ffi::c_int;
        while nZero < nDigit
            && *(*p).a.offset(nZero as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            nZero += 1;
        }
        nFrac = (*p).nFrac + (nDigit - (*p).nDigit);
        nDigit -= nZero;
        z = sqlite3_malloc64(
            (nDigit as ::core::ffi::c_longlong + 20 as ::core::ffi::c_longlong)
                as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if z.is_null() {
            sqlite3_result_error_nomem(pCtx);
            return;
        }
        if nDigit == 0 as ::core::ffi::c_int {
            zero = 0 as ::core::ffi::c_schar;
            a = &raw mut zero;
            nDigit = 1 as ::core::ffi::c_int;
            nFrac = 0 as ::core::ffi::c_int;
        } else {
            a = (*p).a.offset(nZero as isize) as *mut ::core::ffi::c_schar;
        }
        if (*p).sign as ::core::ffi::c_int != 0 && nDigit > 0 as ::core::ffi::c_int {
            *z.offset(0 as ::core::ffi::c_int as isize) = '-' as i32
                as ::core::ffi::c_char;
        } else {
            *z.offset(0 as ::core::ffi::c_int as isize) = '+' as i32
                as ::core::ffi::c_char;
        }
        *z.offset(1 as ::core::ffi::c_int as isize) = (*a
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int + '0' as i32)
            as ::core::ffi::c_char;
        *z.offset(2 as ::core::ffi::c_int as isize) = '.' as i32 as ::core::ffi::c_char;
        if nDigit == 1 as ::core::ffi::c_int {
            *z.offset(3 as ::core::ffi::c_int as isize) = '0' as i32
                as ::core::ffi::c_char;
            i = 4 as ::core::ffi::c_int;
        } else {
            i = 1 as ::core::ffi::c_int;
            while i < nDigit {
                *z.offset((2 as ::core::ffi::c_int + i) as isize) = (*a
                    .offset(i as isize) as ::core::ffi::c_int + '0' as i32)
                    as ::core::ffi::c_char;
                i += 1;
            }
            i = nDigit + 2 as ::core::ffi::c_int;
        }
        exp = nDigit - nFrac - 1 as ::core::ffi::c_int;
        sqlite3_snprintf(
            nDigit + 20 as ::core::ffi::c_int - i,
            z.offset(i as isize) as *mut ::core::ffi::c_char,
            b"e%+03d\0".as_ptr() as *const ::core::ffi::c_char,
            exp,
        );
        sqlite3_result_text(
            pCtx,
            z,
            -1 as ::core::ffi::c_int,
            Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    }
}
unsafe extern "C" fn decimal_cmp(
    mut pA: *mut Decimal,
    mut pB: *mut Decimal,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nASig: ::core::ffi::c_int = 0;
        let mut nBSig: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        while (*pA).nFrac > 0 as ::core::ffi::c_int
            && *(*pA).a.offset(((*pA).nDigit - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*pA).nDigit -= 1;
            (*pA).nFrac -= 1;
        }
        while (*pB).nFrac > 0 as ::core::ffi::c_int
            && *(*pB).a.offset(((*pB).nDigit - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*pB).nDigit -= 1;
            (*pB).nFrac -= 1;
        }
        if (*pA).sign as ::core::ffi::c_int != (*pB).sign as ::core::ffi::c_int {
            return if (*pA).sign as ::core::ffi::c_int != 0 {
                -1 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
        }
        if (*pA).sign != 0 {
            let mut pTemp: *mut Decimal = pA;
            pA = pB;
            pB = pTemp;
        }
        nASig = (*pA).nDigit - (*pA).nFrac;
        nBSig = (*pB).nDigit - (*pB).nFrac;
        if nASig != nBSig {
            return nASig - nBSig;
        }
        n = (*pA).nDigit;
        if n > (*pB).nDigit {
            n = (*pB).nDigit;
        }
        rc = memcmp(
            (*pA).a as *const ::core::ffi::c_void,
            (*pB).a as *const ::core::ffi::c_void,
            n as size_t,
        );
        if rc == 0 as ::core::ffi::c_int {
            rc = (*pA).nDigit - (*pB).nDigit;
        }
        return rc;
    }
}
unsafe extern "C" fn decimalCmpFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pA: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        let mut pB: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        let mut rc: ::core::ffi::c_int = 0;
        pA = decimal_new(
            context,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        if !(pA.is_null() || (*pA).isNull as ::core::ffi::c_int != 0) {
            pB = decimal_new(
                context,
                *argv.offset(1 as ::core::ffi::c_int as isize),
                1 as ::core::ffi::c_int,
            );
            if !(pB.is_null() || (*pB).isNull as ::core::ffi::c_int != 0) {
                rc = decimal_cmp(pA, pB);
                if rc < 0 as ::core::ffi::c_int {
                    rc = -1 as ::core::ffi::c_int;
                } else if rc > 0 as ::core::ffi::c_int {
                    rc = 1 as ::core::ffi::c_int;
                }
                sqlite3_result_int(context, rc);
            }
        }
        decimal_free(pA);
        decimal_free(pB);
    }
}
unsafe extern "C" fn decimal_expand(
    mut p: *mut Decimal,
    mut nDigit: ::core::ffi::c_int,
    mut nFrac: ::core::ffi::c_int,
) {
    unsafe {
        let mut nAddSig: ::core::ffi::c_int = 0;
        let mut nAddFrac: ::core::ffi::c_int = 0;
        if p.is_null() {
            return;
        }
        nAddFrac = nFrac - (*p).nFrac;
        nAddSig = nDigit - (*p).nDigit - nAddFrac;
        if nAddFrac == 0 as ::core::ffi::c_int && nAddSig == 0 as ::core::ffi::c_int {
            return;
        }
        (*p).a = sqlite3_realloc64(
            (*p).a as *mut ::core::ffi::c_void,
            (nDigit + 1 as ::core::ffi::c_int) as sqlite3_uint64,
        ) as *mut ::core::ffi::c_schar;
        if (*p).a.is_null() {
            (*p).oom = 1 as ::core::ffi::c_char;
            return;
        }
        if nAddSig != 0 {
            memmove(
                (*p).a.offset(nAddSig as isize) as *mut ::core::ffi::c_void,
                (*p).a as *const ::core::ffi::c_void,
                (*p).nDigit as size_t,
            );
            memset(
                (*p).a as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nAddSig as size_t,
            );
            (*p).nDigit += nAddSig;
        }
        if nAddFrac != 0 {
            memset(
                (*p).a.offset((*p).nDigit as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nAddFrac as size_t,
            );
            (*p).nDigit += nAddFrac;
            (*p).nFrac += nAddFrac;
        }
    }
}
unsafe extern "C" fn decimal_add(mut pA: *mut Decimal, mut pB: *mut Decimal) {
    unsafe {
        let mut nSig: ::core::ffi::c_int = 0;
        let mut nFrac: ::core::ffi::c_int = 0;
        let mut nDigit: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if pA.is_null() {
            return;
        }
        if (*pA).oom as ::core::ffi::c_int != 0 || pB.is_null()
            || (*pB).oom as ::core::ffi::c_int != 0
        {
            (*pA).oom = 1 as ::core::ffi::c_char;
            return;
        }
        if (*pA).isNull as ::core::ffi::c_int != 0
            || (*pB).isNull as ::core::ffi::c_int != 0
        {
            (*pA).isNull = 1 as ::core::ffi::c_char;
            return;
        }
        nSig = (*pA).nDigit - (*pA).nFrac;
        if nSig != 0
            && *(*pA).a.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            nSig -= 1;
        }
        if nSig < (*pB).nDigit - (*pB).nFrac {
            nSig = (*pB).nDigit - (*pB).nFrac;
        }
        nFrac = (*pA).nFrac;
        if nFrac < (*pB).nFrac {
            nFrac = (*pB).nFrac;
        }
        nDigit = nSig + nFrac + 1 as ::core::ffi::c_int;
        decimal_expand(pA, nDigit, nFrac);
        decimal_expand(pB, nDigit, nFrac);
        if (*pA).oom as ::core::ffi::c_int != 0 || (*pB).oom as ::core::ffi::c_int != 0 {
            (*pA).oom = 1 as ::core::ffi::c_char;
        } else if (*pA).sign as ::core::ffi::c_int == (*pB).sign as ::core::ffi::c_int {
            let mut carry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            i = nDigit - 1 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                let mut x: ::core::ffi::c_int = *(*pA).a.offset(i as isize)
                    as ::core::ffi::c_int
                    + *(*pB).a.offset(i as isize) as ::core::ffi::c_int + carry;
                if x >= 10 as ::core::ffi::c_int {
                    carry = 1 as ::core::ffi::c_int;
                    *(*pA).a.offset(i as isize) = (x - 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_schar;
                } else {
                    carry = 0 as ::core::ffi::c_int;
                    *(*pA).a.offset(i as isize) = x as ::core::ffi::c_schar;
                }
                i -= 1;
            }
        } else {
            let mut aA: *mut ::core::ffi::c_schar = ::core::ptr::null_mut::<
                ::core::ffi::c_schar,
            >();
            let mut aB: *mut ::core::ffi::c_schar = ::core::ptr::null_mut::<
                ::core::ffi::c_schar,
            >();
            let mut borrow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = memcmp(
                (*pA).a as *const ::core::ffi::c_void,
                (*pB).a as *const ::core::ffi::c_void,
                nDigit as size_t,
            );
            if rc < 0 as ::core::ffi::c_int {
                aA = (*pB).a;
                aB = (*pA).a;
                (*pA).sign = ((*pA).sign == 0) as ::core::ffi::c_int
                    as ::core::ffi::c_char;
            } else {
                aA = (*pA).a;
                aB = (*pB).a;
            }
            i = nDigit - 1 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                let mut x_0: ::core::ffi::c_int = *aA.offset(i as isize)
                    as ::core::ffi::c_int - *aB.offset(i as isize) as ::core::ffi::c_int
                    - borrow;
                if x_0 < 0 as ::core::ffi::c_int {
                    *(*pA).a.offset(i as isize) = (x_0 + 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_schar;
                    borrow = 1 as ::core::ffi::c_int;
                } else {
                    *(*pA).a.offset(i as isize) = x_0 as ::core::ffi::c_schar;
                    borrow = 0 as ::core::ffi::c_int;
                }
                i -= 1;
            }
        };
    }
}
unsafe extern "C" fn decimalMul(mut pA: *mut Decimal, mut pB: *mut Decimal) {
    unsafe {
        let mut acc: *mut ::core::ffi::c_schar = ::core::ptr::null_mut::<
            ::core::ffi::c_schar,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut minFrac: ::core::ffi::c_int = 0;
        if !(pA.is_null() || (*pA).oom as ::core::ffi::c_int != 0
            || (*pA).isNull as ::core::ffi::c_int != 0 || pB.is_null()
            || (*pB).oom as ::core::ffi::c_int != 0
            || (*pB).isNull as ::core::ffi::c_int != 0)
        {
            acc = sqlite3_malloc64(
                ((*pA).nDigit as ::core::ffi::c_longlong
                    + (*pB).nDigit as ::core::ffi::c_longlong
                    + 2 as ::core::ffi::c_longlong) as sqlite3_uint64,
            ) as *mut ::core::ffi::c_schar;
            if acc.is_null() {
                (*pA).oom = 1 as ::core::ffi::c_char;
            } else {
                memset(
                    acc as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ((*pA).nDigit + (*pB).nDigit + 2 as ::core::ffi::c_int) as size_t,
                );
                minFrac = (*pA).nFrac;
                if (*pB).nFrac < minFrac {
                    minFrac = (*pB).nFrac;
                }
                i = (*pA).nDigit - 1 as ::core::ffi::c_int;
                while i >= 0 as ::core::ffi::c_int {
                    let mut f: ::core::ffi::c_schar = *(*pA).a.offset(i as isize);
                    let mut carry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut x: ::core::ffi::c_int = 0;
                    j = (*pB).nDigit - 1 as ::core::ffi::c_int;
                    k = i + j + 3 as ::core::ffi::c_int;
                    while j >= 0 as ::core::ffi::c_int {
                        x = *acc.offset(k as isize) as ::core::ffi::c_int
                            + f as ::core::ffi::c_int
                                * *(*pB).a.offset(j as isize) as ::core::ffi::c_int + carry;
                        *acc.offset(k as isize) = (x % 10 as ::core::ffi::c_int)
                            as ::core::ffi::c_schar;
                        carry = x / 10 as ::core::ffi::c_int;
                        j -= 1;
                        k -= 1;
                    }
                    x = *acc.offset(k as isize) as ::core::ffi::c_int + carry;
                    *acc.offset(k as isize) = (x % 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_schar;
                    let ref mut c2rust_fresh1 = *acc
                        .offset((k - 1 as ::core::ffi::c_int) as isize);
                    *c2rust_fresh1 = (*c2rust_fresh1 as ::core::ffi::c_int
                        + x / 10 as ::core::ffi::c_int) as ::core::ffi::c_schar;
                    i -= 1;
                }
                sqlite3_free((*pA).a as *mut ::core::ffi::c_void);
                (*pA).a = acc;
                acc = ::core::ptr::null_mut::<::core::ffi::c_schar>();
                (*pA).nDigit += (*pB).nDigit + 2 as ::core::ffi::c_int;
                (*pA).nFrac += (*pB).nFrac;
                (*pA).sign = ((*pA).sign as ::core::ffi::c_int
                    ^ (*pB).sign as ::core::ffi::c_int) as ::core::ffi::c_char;
                while (*pA).nFrac > minFrac
                    && *(*pA).a.offset(((*pA).nDigit - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    (*pA).nFrac -= 1;
                    (*pA).nDigit -= 1;
                }
            }
        }
        sqlite3_free(acc as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn decimalPow2(mut N: ::core::ffi::c_int) -> *mut Decimal {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pA: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        let mut pX: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        if !(N < -20000 as ::core::ffi::c_int || N > 20000 as ::core::ffi::c_int) {
            pA = decimalNewFromText(
                b"1.0\0".as_ptr() as *const ::core::ffi::c_char,
                3 as ::core::ffi::c_int,
            );
            if !(pA.is_null() || (*pA).oom as ::core::ffi::c_int != 0) {
                if N == 0 as ::core::ffi::c_int {
                    return pA;
                }
                if N > 0 as ::core::ffi::c_int {
                    pX = decimalNewFromText(
                        b"2.0\0".as_ptr() as *const ::core::ffi::c_char,
                        3 as ::core::ffi::c_int,
                    );
                } else {
                    N = -N;
                    pX = decimalNewFromText(
                        b"0.5\0".as_ptr() as *const ::core::ffi::c_char,
                        3 as ::core::ffi::c_int,
                    );
                }
                if !(pX.is_null() || (*pX).oom as ::core::ffi::c_int != 0) {
                    loop {
                        if N & 1 as ::core::ffi::c_int != 0 {
                            decimalMul(pA, pX);
                            if (*pA).oom != 0 {
                                c2rust_current_block = 5491477955474942809;
                                break;
                            }
                        }
                        N >>= 1 as ::core::ffi::c_int;
                        if N == 0 as ::core::ffi::c_int {
                            c2rust_current_block = 15976848397966268834;
                            break;
                        }
                        decimalMul(pX, pX);
                    }
                    match c2rust_current_block {
                        5491477955474942809 => {}
                        _ => {
                            decimal_free(pX);
                            return pA;
                        }
                    }
                }
            }
        }
        decimal_free(pA);
        decimal_free(pX);
        return ::core::ptr::null_mut::<Decimal>();
    }
}
unsafe extern "C" fn decimalFromDouble(mut r: ::core::ffi::c_double) -> *mut Decimal {
    unsafe {
        let mut m: sqlite3_int64 = 0;
        let mut a: sqlite3_int64 = 0;
        let mut e: ::core::ffi::c_int = 0;
        let mut isNeg: ::core::ffi::c_int = 0;
        let mut pA: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        let mut pX: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        let mut zNum: [::core::ffi::c_char; 100] = [0; 100];
        if r < 0.0f64 {
            isNeg = 1 as ::core::ffi::c_int;
            r = -r;
        } else {
            isNeg = 0 as ::core::ffi::c_int;
        }
        memcpy(
            &raw mut a as *mut ::core::ffi::c_void,
            &raw mut r as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sqlite3_int64>() as size_t,
        );
        if a == 0 as ::core::ffi::c_longlong
            || a == 0x8000000000000000 as ::core::ffi::c_ulonglong as sqlite3_int64
        {
            e = 0 as ::core::ffi::c_int;
            m = 0 as sqlite3_int64;
        } else {
            e = (a >> 52 as ::core::ffi::c_int) as ::core::ffi::c_int;
            m = (a as ::core::ffi::c_longlong
                & ((1 as ::core::ffi::c_int as ::core::ffi::c_longlong)
                    << 52 as ::core::ffi::c_int) - 1 as ::core::ffi::c_longlong)
                as sqlite3_int64;
            if e == 0 as ::core::ffi::c_int {
                m <<= 1 as ::core::ffi::c_int;
            } else {
                m
                    |= ((1 as ::core::ffi::c_int as sqlite3_int64)
                        << 52 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
            }
            while e < 1075 as ::core::ffi::c_int && m > 0 as ::core::ffi::c_longlong
                && m as ::core::ffi::c_longlong & 1 as ::core::ffi::c_longlong
                    == 0 as ::core::ffi::c_longlong
            {
                m >>= 1 as ::core::ffi::c_int;
                e += 1;
            }
            if isNeg != 0 {
                m = -m;
            }
            e = e - 1075 as ::core::ffi::c_int;
            if e > 971 as ::core::ffi::c_int {
                return ::core::ptr::null_mut::<Decimal>();
            }
        }
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zNum as *mut ::core::ffi::c_char,
            b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
            m,
        );
        pA = decimalNewFromText(
            &raw mut zNum as *mut ::core::ffi::c_char,
            strlen(&raw mut zNum as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
        );
        pX = decimalPow2(e);
        decimalMul(pA, pX);
        decimal_free(pX);
        return pA;
    }
}
unsafe extern "C" fn decimalFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut Decimal = decimal_new(
            context,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            0 as ::core::ffi::c_int,
        );
        if !p.is_null() {
            if !sqlite3_user_data(context).is_null() {
                decimal_result_sci(context, p);
            } else {
                decimal_result(context, p);
            }
            decimal_free(p);
        }
    }
}
unsafe extern "C" fn decimalCollFunc(
    mut notUsed: *mut ::core::ffi::c_void,
    mut nKey1: ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut nKey2: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zA: *const ::core::ffi::c_uchar = pKey1 as *const ::core::ffi::c_uchar;
        let mut zB: *const ::core::ffi::c_uchar = pKey2 as *const ::core::ffi::c_uchar;
        let mut pA: *mut Decimal = decimalNewFromText(
            zA as *const ::core::ffi::c_char,
            nKey1,
        );
        let mut pB: *mut Decimal = decimalNewFromText(
            zB as *const ::core::ffi::c_char,
            nKey2,
        );
        let mut rc: ::core::ffi::c_int = 0;
        if pA.is_null() || pB.is_null() {
            rc = 0 as ::core::ffi::c_int;
        } else {
            rc = decimal_cmp(pA, pB);
        }
        decimal_free(pA);
        decimal_free(pB);
        return rc;
    }
}
unsafe extern "C" fn decimalAddFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pA: *mut Decimal = decimal_new(
            context,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        let mut pB: *mut Decimal = decimal_new(
            context,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        decimal_add(pA, pB);
        decimal_result(context, pA);
        decimal_free(pA);
        decimal_free(pB);
    }
}
unsafe extern "C" fn decimalSubFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pA: *mut Decimal = decimal_new(
            context,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        let mut pB: *mut Decimal = decimal_new(
            context,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        if !pB.is_null() {
            (*pB).sign = ((*pB).sign == 0) as ::core::ffi::c_int as ::core::ffi::c_char;
            decimal_add(pA, pB);
            decimal_result(context, pA);
        }
        decimal_free(pA);
        decimal_free(pB);
    }
}
unsafe extern "C" fn decimalSumStep(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        let mut pArg: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        p = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<Decimal>() as ::core::ffi::c_int,
        ) as *mut Decimal;
        if p.is_null() {
            return;
        }
        if (*p).isInit == 0 {
            (*p).isInit = 1 as ::core::ffi::c_char;
            (*p).a = sqlite3_malloc(2 as ::core::ffi::c_int)
                as *mut ::core::ffi::c_schar;
            if (*p).a.is_null() {
                (*p).oom = 1 as ::core::ffi::c_char;
            } else {
                *(*p).a.offset(0 as ::core::ffi::c_int as isize) = 0
                    as ::core::ffi::c_schar;
            }
            (*p).nDigit = 1 as ::core::ffi::c_int;
            (*p).nFrac = 0 as ::core::ffi::c_int;
        }
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_NULL
        {
            return;
        }
        pArg = decimal_new(
            context,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        decimal_add(p, pArg);
        decimal_free(pArg);
    }
}
unsafe extern "C" fn decimalSumInverse(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        let mut pArg: *mut Decimal = ::core::ptr::null_mut::<Decimal>();
        p = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<Decimal>() as ::core::ffi::c_int,
        ) as *mut Decimal;
        if p.is_null() {
            return;
        }
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_NULL
        {
            return;
        }
        pArg = decimal_new(
            context,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        if !pArg.is_null() {
            (*pArg).sign = ((*pArg).sign == 0) as ::core::ffi::c_int
                as ::core::ffi::c_char;
        }
        decimal_add(p, pArg);
        decimal_free(pArg);
    }
}
unsafe extern "C" fn decimalSumValue(mut context: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut Decimal = sqlite3_aggregate_context(
            context,
            0 as ::core::ffi::c_int,
        ) as *mut Decimal;
        if p.is_null() {
            return;
        }
        decimal_result(context, p);
    }
}
unsafe extern "C" fn decimalSumFinalize(mut context: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut Decimal = sqlite3_aggregate_context(
            context,
            0 as ::core::ffi::c_int,
        ) as *mut Decimal;
        if p.is_null() {
            return;
        }
        decimal_result(context, p);
        decimal_clear(p);
    }
}
unsafe extern "C" fn decimalMulFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pA: *mut Decimal = decimal_new(
            context,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        let mut pB: *mut Decimal = decimal_new(
            context,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
        );
        if !(pA.is_null() || (*pA).oom as ::core::ffi::c_int != 0
            || (*pA).isNull as ::core::ffi::c_int != 0 || pB.is_null()
            || (*pB).oom as ::core::ffi::c_int != 0
            || (*pB).isNull as ::core::ffi::c_int != 0)
        {
            decimalMul(pA, pB);
            if !((*pA).oom != 0) {
                decimal_result(context, pA);
            }
        }
        decimal_free(pA);
        decimal_free(pB);
    }
}
unsafe extern "C" fn decimalPow2Func(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_INTEGER
        {
            let mut pA: *mut Decimal = decimalPow2(
                sqlite3_value_int(*argv.offset(0 as ::core::ffi::c_int as isize)),
            );
            decimal_result_sci(context, pA);
            decimal_free(pA);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_decimal_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        static mut aFunc: [C2Rust_Unnamed_0; 7] = unsafe {
            [
                C2Rust_Unnamed_0 {
                    zFuncName: b"decimal\0".as_ptr() as *const ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_int,
                    iArg: 0 as ::core::ffi::c_int,
                    xFunc: Some(
                        decimalFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_0 {
                    zFuncName: b"decimal_exp\0".as_ptr() as *const ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_int,
                    iArg: 1 as ::core::ffi::c_int,
                    xFunc: Some(
                        decimalFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_0 {
                    zFuncName: b"decimal_cmp\0".as_ptr() as *const ::core::ffi::c_char,
                    nArg: 2 as ::core::ffi::c_int,
                    iArg: 0 as ::core::ffi::c_int,
                    xFunc: Some(
                        decimalCmpFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_0 {
                    zFuncName: b"decimal_add\0".as_ptr() as *const ::core::ffi::c_char,
                    nArg: 2 as ::core::ffi::c_int,
                    iArg: 0 as ::core::ffi::c_int,
                    xFunc: Some(
                        decimalAddFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_0 {
                    zFuncName: b"decimal_sub\0".as_ptr() as *const ::core::ffi::c_char,
                    nArg: 2 as ::core::ffi::c_int,
                    iArg: 0 as ::core::ffi::c_int,
                    xFunc: Some(
                        decimalSubFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_0 {
                    zFuncName: b"decimal_mul\0".as_ptr() as *const ::core::ffi::c_char,
                    nArg: 2 as ::core::ffi::c_int,
                    iArg: 0 as ::core::ffi::c_int,
                    xFunc: Some(
                        decimalMulFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_0 {
                    zFuncName: b"decimal_pow2\0".as_ptr() as *const ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_int,
                    iArg: 0 as ::core::ffi::c_int,
                    xFunc: Some(
                        decimalPow2Func
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
            ]
        };
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i
            < (::core::mem::size_of::<[C2Rust_Unnamed_0; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_0>() as usize)
                as ::core::ffi::c_int as ::core::ffi::c_uint && rc == SQLITE_OK
        {
            rc = sqlite3_create_function(
                db,
                aFunc[i as usize].zFuncName,
                aFunc[i as usize].nArg,
                SQLITE_UTF8 | SQLITE_INNOCUOUS | SQLITE_DETERMINISTIC,
                (if aFunc[i as usize].iArg != 0 {
                    db
                } else {
                    ::core::ptr::null_mut::<sqlite3>()
                }) as *mut ::core::ffi::c_void,
                aFunc[i as usize].xFunc,
                None,
                None,
            );
            i = i.wrapping_add(1);
        }
        if rc == SQLITE_OK {
            rc = sqlite3_create_window_function(
                db,
                b"decimal_sum\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8 | SQLITE_INNOCUOUS | SQLITE_DETERMINISTIC,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    decimalSumStep
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                Some(
                    decimalSumFinalize
                        as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                ),
                Some(
                    decimalSumValue as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                ),
                Some(
                    decimalSumInverse
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                None,
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3_create_collation(
                db,
                b"decimal\0".as_ptr() as *const ::core::ffi::c_char,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    decimalCollFunc
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
        }
        return rc;
    }
}
