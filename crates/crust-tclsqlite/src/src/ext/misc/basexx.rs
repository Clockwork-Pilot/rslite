unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_str;
    pub type sqlite3_blob;
    pub type sqlite3_backup;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_errcode(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_limit(
        _: *mut sqlite3,
        id: ::core::ffi::c_int,
        newVal: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_null(_: *mut sqlite3_context);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_zeroblob(_: *mut sqlite3_context, n: ::core::ffi::c_int);
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
pub type u8_0 = ::core::ffi::c_uchar;
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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4;
pub const SQLITE_NULL: ::core::ffi::c_int = 5;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_DIRECTONLY: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_INNOCUOUS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
unsafe extern "C" fn init_api_ptr(mut pApi: *const sqlite3_api_routines) {}
pub const PC: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const WS: ::core::ffi::c_int = 0x81 as ::core::ffi::c_int;
pub const ND: ::core::ffi::c_int = 0x82 as ::core::ffi::c_int;
pub const PAD_CHAR: ::core::ffi::c_int = '=' as i32;
static mut b64DigitValues: [u8_0; 128] = [
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    WS as u8_0,
    WS as u8_0,
    WS as u8_0,
    WS as u8_0,
    WS as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    WS as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    62 as ::core::ffi::c_int as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    63 as ::core::ffi::c_int as u8_0,
    52 as ::core::ffi::c_int as u8_0,
    53 as ::core::ffi::c_int as u8_0,
    54 as ::core::ffi::c_int as u8_0,
    55 as ::core::ffi::c_int as u8_0,
    56 as ::core::ffi::c_int as u8_0,
    57 as ::core::ffi::c_int as u8_0,
    58 as ::core::ffi::c_int as u8_0,
    59 as ::core::ffi::c_int as u8_0,
    60 as ::core::ffi::c_int as u8_0,
    61 as ::core::ffi::c_int as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    PC as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    0 as ::core::ffi::c_int as u8_0,
    1 as ::core::ffi::c_int as u8_0,
    2 as ::core::ffi::c_int as u8_0,
    3 as ::core::ffi::c_int as u8_0,
    4 as ::core::ffi::c_int as u8_0,
    5 as ::core::ffi::c_int as u8_0,
    6 as ::core::ffi::c_int as u8_0,
    7 as ::core::ffi::c_int as u8_0,
    8 as ::core::ffi::c_int as u8_0,
    9 as ::core::ffi::c_int as u8_0,
    10 as ::core::ffi::c_int as u8_0,
    11 as ::core::ffi::c_int as u8_0,
    12 as ::core::ffi::c_int as u8_0,
    13 as ::core::ffi::c_int as u8_0,
    14 as ::core::ffi::c_int as u8_0,
    15 as ::core::ffi::c_int as u8_0,
    16 as ::core::ffi::c_int as u8_0,
    17 as ::core::ffi::c_int as u8_0,
    18 as ::core::ffi::c_int as u8_0,
    19 as ::core::ffi::c_int as u8_0,
    20 as ::core::ffi::c_int as u8_0,
    21 as ::core::ffi::c_int as u8_0,
    22 as ::core::ffi::c_int as u8_0,
    23 as ::core::ffi::c_int as u8_0,
    24 as ::core::ffi::c_int as u8_0,
    25 as ::core::ffi::c_int as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    26 as ::core::ffi::c_int as u8_0,
    27 as ::core::ffi::c_int as u8_0,
    28 as ::core::ffi::c_int as u8_0,
    29 as ::core::ffi::c_int as u8_0,
    30 as ::core::ffi::c_int as u8_0,
    31 as ::core::ffi::c_int as u8_0,
    32 as ::core::ffi::c_int as u8_0,
    33 as ::core::ffi::c_int as u8_0,
    34 as ::core::ffi::c_int as u8_0,
    35 as ::core::ffi::c_int as u8_0,
    36 as ::core::ffi::c_int as u8_0,
    37 as ::core::ffi::c_int as u8_0,
    38 as ::core::ffi::c_int as u8_0,
    39 as ::core::ffi::c_int as u8_0,
    40 as ::core::ffi::c_int as u8_0,
    41 as ::core::ffi::c_int as u8_0,
    42 as ::core::ffi::c_int as u8_0,
    43 as ::core::ffi::c_int as u8_0,
    44 as ::core::ffi::c_int as u8_0,
    45 as ::core::ffi::c_int as u8_0,
    46 as ::core::ffi::c_int as u8_0,
    47 as ::core::ffi::c_int as u8_0,
    48 as ::core::ffi::c_int as u8_0,
    49 as ::core::ffi::c_int as u8_0,
    50 as ::core::ffi::c_int as u8_0,
    51 as ::core::ffi::c_int as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
    ND as u8_0,
];
static mut b64Numerals: [::core::ffi::c_char; 65] = unsafe {
    ::core::mem::transmute::<
        [u8; 65],
        [::core::ffi::c_char; 65],
    >(*b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0")
};
pub const B64_DARK_MAX: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
unsafe extern "C" fn toBase64(
    mut pIn: *mut u8_0,
    mut nbIn: ::core::ffi::c_int,
    mut pOut: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while nbIn >= 3 as ::core::ffi::c_int {
            *pOut.offset(0 as ::core::ffi::c_int as isize) = b64Numerals[(*pIn
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as u8_0 as usize];
            *pOut.offset(1 as ::core::ffi::c_int as isize) = b64Numerals[(((*pIn
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int
                | *pIn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int) & 0x3f as ::core::ffi::c_int) as u8_0
                as usize];
            *pOut.offset(2 as ::core::ffi::c_int as isize) = b64Numerals[((*pIn
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | *pIn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int) as u8_0 as usize];
            *pOut.offset(3 as ::core::ffi::c_int as isize) = b64Numerals[(*pIn
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int) as u8_0 as usize];
            pOut = pOut.offset(4 as ::core::ffi::c_int as isize);
            nbIn -= 3 as ::core::ffi::c_int;
            pIn = pIn.offset(3 as ::core::ffi::c_int as isize);
            nCol += 4 as ::core::ffi::c_int;
            if nCol >= B64_DARK_MAX || nbIn <= 0 as ::core::ffi::c_int {
                let c2rust_fresh0 = pOut;
                pOut = pOut.offset(1);
                *c2rust_fresh0 = '\n' as i32 as ::core::ffi::c_char;
                nCol = 0 as ::core::ffi::c_int;
            }
        }
        if nbIn > 0 as ::core::ffi::c_int {
            let mut nco: ::core::ffi::c_schar = (nbIn + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_schar;
            let mut nbe: ::core::ffi::c_int = 0;
            let c2rust_fresh1 = pIn;
            pIn = pIn.offset(1);
            let mut qv: ::core::ffi::c_ulong = *c2rust_fresh1 as ::core::ffi::c_ulong;
            nbe = 1 as ::core::ffi::c_int;
            while nbe < 3 as ::core::ffi::c_int {
                qv <<= 8 as ::core::ffi::c_int;
                if nbe < nbIn {
                    let c2rust_fresh2 = pIn;
                    pIn = pIn.offset(1);
                    qv |= *c2rust_fresh2 as ::core::ffi::c_ulong;
                }
                nbe += 1;
            }
            nbe = 3 as ::core::ffi::c_int;
            while nbe >= 0 as ::core::ffi::c_int {
                let mut ce: ::core::ffi::c_char = (if nbe < nco as ::core::ffi::c_int {
                    b64Numerals[(qv & 0x3f as ::core::ffi::c_ulong) as u8_0 as usize]
                        as ::core::ffi::c_int
                } else {
                    PAD_CHAR
                }) as ::core::ffi::c_char;
                qv >>= 6 as ::core::ffi::c_int;
                *pOut.offset(nbe as isize) = ce;
                nbe -= 1;
            }
            pOut = pOut.offset(4 as ::core::ffi::c_int as isize);
            let c2rust_fresh3 = pOut;
            pOut = pOut.offset(1);
            *c2rust_fresh3 = '\n' as i32 as ::core::ffi::c_char;
        }
        *pOut = 0 as ::core::ffi::c_char;
        return pOut;
    }
}
unsafe extern "C" fn skipNonB64(
    mut s: *mut ::core::ffi::c_char,
    mut nc: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut c: ::core::ffi::c_char = 0;
        loop {
            let c2rust_fresh4 = nc;
            nc = nc - 1;
            if !(c2rust_fresh4 > 0 as ::core::ffi::c_int
                && {
                    c = *s;
                    c as ::core::ffi::c_int != 0
                }
                && !(((if (c as u8_0 as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
                {
                    b64DigitValues[c as u8_0 as usize] as ::core::ffi::c_int
                } else {
                    0x80 as ::core::ffi::c_int
                }) as u8_0 as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int))
            {
                break;
            }
            s = s.offset(1);
        }
        return s;
    }
}
unsafe extern "C" fn fromBase64(
    mut pIn: *mut ::core::ffi::c_char,
    mut ncIn: ::core::ffi::c_int,
    mut pOut: *mut u8_0,
) -> *mut u8_0 {
    unsafe {
        if ncIn > 0 as ::core::ffi::c_int
            && *pIn.offset((ncIn - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == '\n' as i32
        {
            ncIn -= 1;
        }
        while ncIn > 0 as ::core::ffi::c_int && *pIn as ::core::ffi::c_int != PAD_CHAR {
            static mut nboi: [::core::ffi::c_schar; 5] = [
                0 as ::core::ffi::c_int as ::core::ffi::c_schar,
                0 as ::core::ffi::c_int as ::core::ffi::c_schar,
                1 as ::core::ffi::c_int as ::core::ffi::c_schar,
                2 as ::core::ffi::c_int as ::core::ffi::c_schar,
                3 as ::core::ffi::c_int as ::core::ffi::c_schar,
            ];
            let mut pUse: *mut ::core::ffi::c_char = skipNonB64(pIn, ncIn);
            let mut qv: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
            let mut nti: ::core::ffi::c_int = 0;
            let mut nbo: ::core::ffi::c_int = 0;
            let mut nac: ::core::ffi::c_int = 0;
            ncIn = (ncIn as ::core::ffi::c_long
                - pUse.offset_from(pIn) as ::core::ffi::c_long) as ::core::ffi::c_int;
            pIn = pUse;
            nti = if ncIn > 4 as ::core::ffi::c_int {
                4 as ::core::ffi::c_int
            } else {
                ncIn
            };
            ncIn -= nti;
            nbo = nboi[nti as usize] as ::core::ffi::c_int;
            if nbo == 0 as ::core::ffi::c_int {
                break;
            }
            nac = 0 as ::core::ffi::c_int;
            while nac < 4 as ::core::ffi::c_int {
                let mut c: ::core::ffi::c_char = (if nac < nti {
                    let c2rust_fresh5 = pIn;
                    pIn = pIn.offset(1);
                    *c2rust_fresh5 as ::core::ffi::c_int
                } else {
                    b64Numerals[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                }) as ::core::ffi::c_char;
                let mut bdp: u8_0 = (if (c as u8_0 as ::core::ffi::c_int)
                    < 0x80 as ::core::ffi::c_int
                {
                    b64DigitValues[c as u8_0 as usize] as ::core::ffi::c_int
                } else {
                    0x80 as ::core::ffi::c_int
                }) as u8_0;
                let mut c2rust_current_block_15: u64;
                match bdp as ::core::ffi::c_int {
                    ND => {
                        ncIn = 0 as ::core::ffi::c_int;
                        c2rust_current_block_15 = 2090640532801442485;
                    }
                    WS => {
                        c2rust_current_block_15 = 2090640532801442485;
                    }
                    PC => {
                        c2rust_current_block_15 = 16610272218197223412;
                    }
                    _ => {
                        c2rust_current_block_15 = 12496723122765518624;
                    }
                }
                match c2rust_current_block_15 {
                    2090640532801442485 => {
                        nti = nac;
                        c2rust_current_block_15 = 16610272218197223412;
                    }
                    _ => {}
                }
                match c2rust_current_block_15 {
                    16610272218197223412 => {
                        bdp = 0 as u8_0;
                        nbo -= 1;
                    }
                    _ => {}
                }
                qv = qv << 6 as ::core::ffi::c_int | bdp as ::core::ffi::c_ulong;
                nac += 1;
            }
            let mut c2rust_current_block_23: u64;
            match nbo {
                3 => {
                    *pOut.offset(2 as ::core::ffi::c_int as isize) = (qv
                        & 0xff as ::core::ffi::c_ulong) as u8_0;
                    c2rust_current_block_23 = 14308627023463179144;
                }
                2 => {
                    c2rust_current_block_23 = 14308627023463179144;
                }
                1 => {
                    c2rust_current_block_23 = 8086608799474531834;
                }
                _ => {
                    c2rust_current_block_23 = 15768484401365413375;
                }
            }
            match c2rust_current_block_23 {
                14308627023463179144 => {
                    *pOut.offset(1 as ::core::ffi::c_int as isize) = (qv
                        >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_ulong)
                        as u8_0;
                    c2rust_current_block_23 = 8086608799474531834;
                }
                _ => {}
            }
            match c2rust_current_block_23 {
                8086608799474531834 => {
                    *pOut.offset(0 as ::core::ffi::c_int as isize) = (qv
                        >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_ulong)
                        as u8_0;
                }
                _ => {}
            }
            pOut = pOut.offset(nbo as isize);
        }
        return pOut;
    }
}
unsafe extern "C" fn base64(
    mut context: *mut sqlite3_context,
    mut na: ::core::ffi::c_int,
    mut av: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut nb: sqlite3_int64 = 0;
        let mut nv: sqlite3_int64 = sqlite3_value_bytes(
            *av.offset(0 as ::core::ffi::c_int as isize),
        ) as sqlite3_int64;
        let mut nc: sqlite3_int64 = 0;
        let mut nvMax: ::core::ffi::c_int = sqlite3_limit(
            sqlite3_context_db_handle(context),
            SQLITE_LIMIT_LENGTH,
            -1 as ::core::ffi::c_int,
        );
        let mut cBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut bBuf: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        match sqlite3_value_type(*av.offset(0 as ::core::ffi::c_int as isize)) {
            SQLITE_BLOB => {
                nb = nv;
                nc = (4 as ::core::ffi::c_longlong
                    * ((nv as ::core::ffi::c_longlong + 2 as ::core::ffi::c_longlong)
                        / 3 as ::core::ffi::c_longlong)) as sqlite3_int64;
                nc
                    += (nc as ::core::ffi::c_longlong
                        + (B64_DARK_MAX - 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_longlong)
                        / B64_DARK_MAX as ::core::ffi::c_longlong
                        + 1 as ::core::ffi::c_longlong;
                if (nvMax as ::core::ffi::c_longlong) < nc {
                    sqlite3_result_error(
                        context,
                        b"blob expanded to base64 too big\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                    return;
                }
                bBuf = sqlite3_value_blob(*av.offset(0 as ::core::ffi::c_int as isize))
                    as *mut u8_0;
                if bBuf.is_null() {
                    if SQLITE_NOMEM
                        == sqlite3_errcode(sqlite3_context_db_handle(context))
                    {
                        c2rust_current_block = 10269022505599344878;
                    } else {
                        sqlite3_result_text(
                            context,
                            b"\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                            SQLITE_STATIC,
                        );
                        c2rust_current_block = 17281240262373992796;
                    }
                } else {
                    cBuf = sqlite3_malloc(nc as ::core::ffi::c_int)
                        as *mut ::core::ffi::c_char;
                    if cBuf.is_null() {
                        c2rust_current_block = 10269022505599344878;
                    } else {
                        nc = toBase64(bBuf, nb as ::core::ffi::c_int, cBuf)
                            .offset_from(cBuf) as ::core::ffi::c_long
                            as ::core::ffi::c_int as sqlite3_int64;
                        sqlite3_result_text(
                            context,
                            cBuf,
                            nc as ::core::ffi::c_int,
                            Some(
                                sqlite3_free
                                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                            ),
                        );
                        c2rust_current_block = 17281240262373992796;
                    }
                }
            }
            SQLITE_TEXT => {
                nc = nv;
                nb = (3 as ::core::ffi::c_longlong
                    * ((nv as ::core::ffi::c_longlong + 3 as ::core::ffi::c_longlong)
                        / 4 as ::core::ffi::c_longlong)) as sqlite3_int64;
                if (nvMax as ::core::ffi::c_longlong) < nb {
                    sqlite3_result_error(
                        context,
                        b"blob from base64 may be too big\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                    return;
                } else if nb < 1 as ::core::ffi::c_longlong {
                    nb = 1 as sqlite3_int64;
                }
                cBuf = sqlite3_value_text(*av.offset(0 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_char;
                if cBuf.is_null() {
                    if SQLITE_NOMEM
                        == sqlite3_errcode(sqlite3_context_db_handle(context))
                    {
                        c2rust_current_block = 10269022505599344878;
                    } else {
                        sqlite3_result_zeroblob(context, 0 as ::core::ffi::c_int);
                        c2rust_current_block = 17281240262373992796;
                    }
                } else {
                    bBuf = sqlite3_malloc(nb as ::core::ffi::c_int) as *mut u8_0;
                    if bBuf.is_null() {
                        c2rust_current_block = 10269022505599344878;
                    } else {
                        nb = fromBase64(cBuf, nc as ::core::ffi::c_int, bBuf)
                            .offset_from(bBuf) as ::core::ffi::c_long
                            as ::core::ffi::c_int as sqlite3_int64;
                        sqlite3_result_blob(
                            context,
                            bBuf as *const ::core::ffi::c_void,
                            nb as ::core::ffi::c_int,
                            Some(
                                sqlite3_free
                                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                            ),
                        );
                        c2rust_current_block = 17281240262373992796;
                    }
                }
            }
            _ => {
                sqlite3_result_error(
                    context,
                    b"base64 accepts only blob or text\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
        }
        match c2rust_current_block {
            17281240262373992796 => return,
            _ => {
                sqlite3_result_error(
                    context,
                    b"base64 OOM\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_base64_init(
    mut db: *mut sqlite3,
    mut pzErr: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3_create_function(
            db,
            b"base64\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            SQLITE_DETERMINISTIC | SQLITE_INNOCUOUS | SQLITE_DIRECTONLY | SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                base64
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
}
static mut b85_cOffset: [u8_0; 5] = [
    0 as ::core::ffi::c_int as u8_0,
    '#' as i32 as u8_0,
    0 as ::core::ffi::c_int as u8_0,
    ('*' as i32 - 4 as ::core::ffi::c_int) as u8_0,
    0 as ::core::ffi::c_int as u8_0,
];
pub const B85_DARK_MAX: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
unsafe extern "C" fn skipNonB85(
    mut s: *mut ::core::ffi::c_char,
    mut nc: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut c: ::core::ffi::c_char = 0;
        loop {
            let c2rust_fresh6 = nc;
            nc = nc - 1;
            if !(c2rust_fresh6 > 0 as ::core::ffi::c_int
                && {
                    c = *s;
                    c as ::core::ffi::c_int != 0
                }
                && (c as ::core::ffi::c_int >= '#' as i32) as ::core::ffi::c_int
                    + (c as ::core::ffi::c_int > '&' as i32) as ::core::ffi::c_int
                    + (c as ::core::ffi::c_int >= '*' as i32) as ::core::ffi::c_int
                    + (c as ::core::ffi::c_int > 'z' as i32) as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int == 0)
            {
                break;
            }
            s = s.offset(1);
        }
        return s;
    }
}
unsafe extern "C" fn putcs(
    mut pc: *mut ::core::ffi::c_char,
    mut s: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut c: ::core::ffi::c_char = 0;
        loop {
            let c2rust_fresh7 = s;
            s = s.offset(1);
            c = *c2rust_fresh7;
            if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                break;
            }
            let c2rust_fresh8 = pc;
            pc = pc.offset(1);
            *c2rust_fresh8 = c;
        }
        return pc;
    }
}
unsafe extern "C" fn toBase85(
    mut pIn: *mut u8_0,
    mut nbIn: ::core::ffi::c_int,
    mut pOut: *mut ::core::ffi::c_char,
    mut pSep: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while nbIn >= 4 as ::core::ffi::c_int {
            let mut nco: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
            let mut qbv: ::core::ffi::c_ulong = (*pIn
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong)
                << 24 as ::core::ffi::c_int
                | ((*pIn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 16 as ::core::ffi::c_int) as ::core::ffi::c_ulong
                | ((*pIn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong
                | *pIn.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong;
            while nco > 0 as ::core::ffi::c_int {
                let mut nqv: ::core::ffi::c_uint = qbv
                    .wrapping_div(85 as ::core::ffi::c_ulong) as ::core::ffi::c_uint;
                let mut dv: ::core::ffi::c_uchar = qbv
                    .wrapping_sub(
                        (85 as ::core::ffi::c_ulong)
                            .wrapping_mul(nqv as ::core::ffi::c_ulong),
                    ) as ::core::ffi::c_uchar;
                qbv = nqv as ::core::ffi::c_ulong;
                nco -= 1;
                *pOut.offset(nco as isize) = (if (dv as ::core::ffi::c_int)
                    < 4 as ::core::ffi::c_int
                {
                    (dv as ::core::ffi::c_int + '#' as i32) as ::core::ffi::c_char
                        as ::core::ffi::c_int
                } else {
                    (dv as ::core::ffi::c_int - 4 as ::core::ffi::c_int + '*' as i32)
                        as ::core::ffi::c_char as ::core::ffi::c_int
                }) as ::core::ffi::c_char;
            }
            nbIn -= 4 as ::core::ffi::c_int;
            pIn = pIn.offset(4 as ::core::ffi::c_int as isize);
            pOut = pOut.offset(5 as ::core::ffi::c_int as isize);
            if !pSep.is_null()
                && {
                    nCol += 5 as ::core::ffi::c_int;
                    nCol >= B85_DARK_MAX
                }
            {
                pOut = putcs(pOut, pSep);
                nCol = 0 as ::core::ffi::c_int;
            }
        }
        if nbIn > 0 as ::core::ffi::c_int {
            let mut nco_0: ::core::ffi::c_int = nbIn + 1 as ::core::ffi::c_int;
            let c2rust_fresh9 = pIn;
            pIn = pIn.offset(1);
            let mut qv: ::core::ffi::c_ulong = *c2rust_fresh9 as ::core::ffi::c_ulong;
            let mut nbe: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            loop {
                let c2rust_fresh10 = nbe;
                nbe = nbe + 1;
                if !(c2rust_fresh10 < nbIn) {
                    break;
                }
                let c2rust_fresh11 = pIn;
                pIn = pIn.offset(1);
                qv = qv << 8 as ::core::ffi::c_int
                    | *c2rust_fresh11 as ::core::ffi::c_ulong;
            }
            nCol += nco_0;
            while nco_0 > 0 as ::core::ffi::c_int {
                let mut dv_0: u8_0 = qv.wrapping_rem(85 as ::core::ffi::c_ulong) as u8_0;
                qv = qv.wrapping_div(85 as ::core::ffi::c_ulong);
                nco_0 -= 1;
                *pOut.offset(nco_0 as isize) = (if (dv_0 as ::core::ffi::c_int)
                    < 4 as ::core::ffi::c_int
                {
                    (dv_0 as ::core::ffi::c_int + '#' as i32) as ::core::ffi::c_char
                        as ::core::ffi::c_int
                } else {
                    (dv_0 as ::core::ffi::c_int - 4 as ::core::ffi::c_int + '*' as i32)
                        as ::core::ffi::c_char as ::core::ffi::c_int
                }) as ::core::ffi::c_char;
            }
            pOut = pOut.offset((nbIn + 1 as ::core::ffi::c_int) as isize);
        }
        if !pSep.is_null() && nCol > 0 as ::core::ffi::c_int {
            pOut = putcs(pOut, pSep);
        }
        *pOut = 0 as ::core::ffi::c_char;
        return pOut;
    }
}
unsafe extern "C" fn fromBase85(
    mut pIn: *mut ::core::ffi::c_char,
    mut ncIn: ::core::ffi::c_int,
    mut pOut: *mut u8_0,
) -> *mut u8_0 {
    unsafe {
        if ncIn > 0 as ::core::ffi::c_int
            && *pIn.offset((ncIn - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int == '\n' as i32
        {
            ncIn -= 1;
        }
        while ncIn > 0 as ::core::ffi::c_int {
            static mut nboi: [::core::ffi::c_schar; 6] = [
                0 as ::core::ffi::c_int as ::core::ffi::c_schar,
                0 as ::core::ffi::c_int as ::core::ffi::c_schar,
                1 as ::core::ffi::c_int as ::core::ffi::c_schar,
                2 as ::core::ffi::c_int as ::core::ffi::c_schar,
                3 as ::core::ffi::c_int as ::core::ffi::c_schar,
                4 as ::core::ffi::c_int as ::core::ffi::c_schar,
            ];
            let mut pUse: *mut ::core::ffi::c_char = skipNonB85(pIn, ncIn);
            let mut qv: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
            let mut nti: ::core::ffi::c_int = 0;
            let mut nbo: ::core::ffi::c_int = 0;
            ncIn = (ncIn as ::core::ffi::c_long
                - pUse.offset_from(pIn) as ::core::ffi::c_long) as ::core::ffi::c_int;
            pIn = pUse;
            nti = if ncIn > 5 as ::core::ffi::c_int {
                5 as ::core::ffi::c_int
            } else {
                ncIn
            };
            nbo = nboi[nti as usize] as ::core::ffi::c_int;
            if nbo == 0 as ::core::ffi::c_int {
                break;
            }
            while nti > 0 as ::core::ffi::c_int {
                let c2rust_fresh12 = pIn;
                pIn = pIn.offset(1);
                let mut c: ::core::ffi::c_char = *c2rust_fresh12;
                let mut cdo: u8_0 = b85_cOffset[((c as ::core::ffi::c_int >= '#' as i32)
                    as ::core::ffi::c_int
                    + (c as ::core::ffi::c_int > '&' as i32) as ::core::ffi::c_int
                    + (c as ::core::ffi::c_int >= '*' as i32) as ::core::ffi::c_int
                    + (c as ::core::ffi::c_int > 'z' as i32) as ::core::ffi::c_int)
                    as usize];
                ncIn -= 1;
                if cdo as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
                qv = (85 as ::core::ffi::c_ulong)
                    .wrapping_mul(qv)
                    .wrapping_add(
                        (c as ::core::ffi::c_int - cdo as ::core::ffi::c_int)
                            as ::core::ffi::c_ulong,
                    );
                nti -= 1;
            }
            nbo -= nti;
            let mut c2rust_current_block_14: u64;
            match nbo {
                4 => {
                    let c2rust_fresh13 = pOut;
                    pOut = pOut.offset(1);
                    *c2rust_fresh13 = (qv >> 24 as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_ulong) as u8_0;
                    c2rust_current_block_14 = 14428124661266369284;
                }
                3 => {
                    c2rust_current_block_14 = 14428124661266369284;
                }
                2 => {
                    c2rust_current_block_14 = 8328256794720041877;
                }
                1 => {
                    c2rust_current_block_14 = 494466633303020227;
                }
                0 | _ => {
                    c2rust_current_block_14 = 15904375183555213903;
                }
            }
            match c2rust_current_block_14 {
                14428124661266369284 => {
                    let c2rust_fresh14 = pOut;
                    pOut = pOut.offset(1);
                    *c2rust_fresh14 = (qv >> 16 as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_ulong) as u8_0;
                    c2rust_current_block_14 = 8328256794720041877;
                }
                _ => {}
            }
            match c2rust_current_block_14 {
                8328256794720041877 => {
                    let c2rust_fresh15 = pOut;
                    pOut = pOut.offset(1);
                    *c2rust_fresh15 = (qv >> 8 as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_ulong) as u8_0;
                    c2rust_current_block_14 = 494466633303020227;
                }
                _ => {}
            }
            match c2rust_current_block_14 {
                494466633303020227 => {
                    let c2rust_fresh16 = pOut;
                    pOut = pOut.offset(1);
                    *c2rust_fresh16 = (qv & 0xff as ::core::ffi::c_ulong) as u8_0;
                }
                _ => {}
            }
        }
        return pOut;
    }
}
unsafe extern "C" fn allBase85(
    mut p: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: ::core::ffi::c_char = 0;
        loop {
            let c2rust_fresh17 = len;
            len = len - 1;
            if !(c2rust_fresh17 > 0 as ::core::ffi::c_int
                && {
                    let c2rust_fresh18 = p;
                    p = p.offset(1);
                    c = *c2rust_fresh18;
                    c as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                })
            {
                break;
            }
            if (c as ::core::ffi::c_int >= '#' as i32) as ::core::ffi::c_int
                + (c as ::core::ffi::c_int > '&' as i32) as ::core::ffi::c_int
                + (c as ::core::ffi::c_int >= '*' as i32) as ::core::ffi::c_int
                + (c as ::core::ffi::c_int > 'z' as i32) as ::core::ffi::c_int
                & 1 as ::core::ffi::c_int == 0
                && *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn is_base85(
    mut context: *mut sqlite3_context,
    mut na: ::core::ffi::c_int,
    mut av: *mut *mut sqlite3_value,
) {
    unsafe {
        match sqlite3_value_type(*av.offset(0 as ::core::ffi::c_int as isize)) {
            SQLITE_TEXT => {
                let mut rv: ::core::ffi::c_int = allBase85(
                    sqlite3_value_text(*av.offset(0 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_char,
                    sqlite3_value_bytes(*av.offset(0 as ::core::ffi::c_int as isize)),
                );
                sqlite3_result_int(context, rv);
            }
            SQLITE_NULL => {
                sqlite3_result_null(context);
            }
            _ => {
                sqlite3_result_error(
                    context,
                    b"is_base85 accepts only text or NULL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
        };
    }
}
unsafe extern "C" fn base85(
    mut context: *mut sqlite3_context,
    mut na: ::core::ffi::c_int,
    mut av: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut nb: sqlite3_int64 = 0;
        let mut nc: sqlite3_int64 = 0;
        let mut nv: sqlite3_int64 = sqlite3_value_bytes(
            *av.offset(0 as ::core::ffi::c_int as isize),
        ) as sqlite3_int64;
        let mut nvMax: ::core::ffi::c_int = sqlite3_limit(
            sqlite3_context_db_handle(context),
            SQLITE_LIMIT_LENGTH,
            -1 as ::core::ffi::c_int,
        );
        let mut cBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut bBuf: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        match sqlite3_value_type(*av.offset(0 as ::core::ffi::c_int as isize)) {
            SQLITE_BLOB => {
                nb = nv;
                nc = (5 as ::core::ffi::c_longlong
                    * (nv as ::core::ffi::c_longlong / 4 as ::core::ffi::c_longlong)
                    + nv as ::core::ffi::c_longlong % 4 as ::core::ffi::c_longlong
                    + nv as ::core::ffi::c_longlong / 64 as ::core::ffi::c_longlong
                    + 1 as ::core::ffi::c_longlong + 2 as ::core::ffi::c_longlong)
                    as sqlite3_int64;
                if (nvMax as ::core::ffi::c_longlong) < nc {
                    sqlite3_result_error(
                        context,
                        b"blob expanded to base85 too big\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                    return;
                }
                bBuf = sqlite3_value_blob(*av.offset(0 as ::core::ffi::c_int as isize))
                    as *mut u8_0;
                if bBuf.is_null() {
                    if SQLITE_NOMEM
                        == sqlite3_errcode(sqlite3_context_db_handle(context))
                    {
                        c2rust_current_block = 10120260591233090538;
                    } else {
                        sqlite3_result_text(
                            context,
                            b"\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                            SQLITE_STATIC,
                        );
                        c2rust_current_block = 7205609094909031804;
                    }
                } else {
                    cBuf = sqlite3_malloc(nc as ::core::ffi::c_int)
                        as *mut ::core::ffi::c_char;
                    if cBuf.is_null() {
                        c2rust_current_block = 10120260591233090538;
                    } else {
                        nc = toBase85(
                                bBuf,
                                nb as ::core::ffi::c_int,
                                cBuf,
                                b"\n\0".as_ptr() as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            )
                            .offset_from(cBuf) as ::core::ffi::c_long
                            as ::core::ffi::c_int as sqlite3_int64;
                        sqlite3_result_text(
                            context,
                            cBuf,
                            nc as ::core::ffi::c_int,
                            Some(
                                sqlite3_free
                                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                            ),
                        );
                        c2rust_current_block = 7205609094909031804;
                    }
                }
            }
            SQLITE_TEXT => {
                nc = nv;
                nb = (4 as ::core::ffi::c_longlong
                    * (nv as ::core::ffi::c_longlong / 5 as ::core::ffi::c_longlong)
                    + nv as ::core::ffi::c_longlong % 5 as ::core::ffi::c_longlong)
                    as sqlite3_int64;
                if (nvMax as ::core::ffi::c_longlong) < nb {
                    sqlite3_result_error(
                        context,
                        b"blob from base85 may be too big\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                    return;
                } else if nb < 1 as ::core::ffi::c_longlong {
                    nb = 1 as sqlite3_int64;
                }
                cBuf = sqlite3_value_text(*av.offset(0 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_char;
                if cBuf.is_null() {
                    if SQLITE_NOMEM
                        == sqlite3_errcode(sqlite3_context_db_handle(context))
                    {
                        c2rust_current_block = 10120260591233090538;
                    } else {
                        sqlite3_result_zeroblob(context, 0 as ::core::ffi::c_int);
                        c2rust_current_block = 7205609094909031804;
                    }
                } else {
                    bBuf = sqlite3_malloc(nb as ::core::ffi::c_int) as *mut u8_0;
                    if bBuf.is_null() {
                        c2rust_current_block = 10120260591233090538;
                    } else {
                        nb = fromBase85(cBuf, nc as ::core::ffi::c_int, bBuf)
                            .offset_from(bBuf) as ::core::ffi::c_long
                            as ::core::ffi::c_int as sqlite3_int64;
                        sqlite3_result_blob(
                            context,
                            bBuf as *const ::core::ffi::c_void,
                            nb as ::core::ffi::c_int,
                            Some(
                                sqlite3_free
                                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                            ),
                        );
                        c2rust_current_block = 7205609094909031804;
                    }
                }
            }
            _ => {
                sqlite3_result_error(
                    context,
                    b"base85 accepts only blob or text.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
        }
        match c2rust_current_block {
            7205609094909031804 => return,
            _ => {
                sqlite3_result_error(
                    context,
                    b"base85 OOM\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_base85_init(
    mut db: *mut sqlite3,
    mut pzErr: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_create_function(
            db,
            b"is_base85\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            SQLITE_DETERMINISTIC | SQLITE_INNOCUOUS | SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                is_base85
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
        if rc != SQLITE_OK {
            return rc;
        }
        return sqlite3_create_function(
            db,
            b"base85\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            SQLITE_DETERMINISTIC | SQLITE_INNOCUOUS | SQLITE_DIRECTONLY | SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                base85
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
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_basexx_init(
    mut db: *mut sqlite3,
    mut pzErr: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc1: ::core::ffi::c_int = 0;
        let mut rc2: ::core::ffi::c_int = 0;
        init_api_ptr(pApi);
        rc1 = sqlite3_base64_init(
            db,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ::core::ptr::null::<sqlite3_api_routines>(),
        );
        rc2 = sqlite3_base85_init(
            db,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ::core::ptr::null::<sqlite3_api_routines>(),
        );
        if rc1 == SQLITE_OK && rc2 == SQLITE_OK {
            return SQLITE_OK
        } else {
            return SQLITE_ERROR
        };
    }
}
