unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_str;
    pub type sqlite3_blob;
    pub type sqlite3_backup;
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
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
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_get_auxdata(
        _: *mut sqlite3_context,
        N: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_set_auxdata(
        _: *mut sqlite3_context,
        N: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
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
pub type ReStateNumber = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReStateSet {
    pub nState: ::core::ffi::c_uint,
    pub aState: *mut ReStateNumber,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReInput {
    pub z: *const ::core::ffi::c_uchar,
    pub i: ::core::ffi::c_int,
    pub mx: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReCompiled {
    pub sIn: ReInput,
    pub zErr: *const ::core::ffi::c_char,
    pub aOp: *mut ::core::ffi::c_char,
    pub aArg: *mut ::core::ffi::c_int,
    pub xNextChar: Option<unsafe extern "C" fn(*mut ReInput) -> ::core::ffi::c_uint>,
    pub zInit: [::core::ffi::c_uchar; 12],
    pub nInit: ::core::ffi::c_int,
    pub nState: ::core::ffi::c_uint,
    pub nAlloc: ::core::ffi::c_uint,
    pub mxAlloc: ::core::ffi::c_uint,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LIKE_PATTERN_LENGTH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_INNOCUOUS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const RE_EOF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RE_START: ::core::ffi::c_int = 0xfffffff as ::core::ffi::c_int;
pub const RE_OP_MATCH: ::core::ffi::c_int = 1;
pub const RE_OP_ANY: ::core::ffi::c_int = 2;
pub const RE_OP_ANYSTAR: ::core::ffi::c_int = 3;
pub const RE_OP_FORK: ::core::ffi::c_int = 4;
pub const RE_OP_GOTO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const RE_OP_ACCEPT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const RE_OP_CC_INC: ::core::ffi::c_int = 7;
pub const RE_OP_CC_EXC: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RE_OP_CC_VALUE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const RE_OP_CC_RANGE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const RE_OP_WORD: ::core::ffi::c_int = 11;
pub const RE_OP_NOTWORD: ::core::ffi::c_int = 12;
pub const RE_OP_DIGIT: ::core::ffi::c_int = 13;
pub const RE_OP_NOTDIGIT: ::core::ffi::c_int = 14;
pub const RE_OP_SPACE: ::core::ffi::c_int = 15;
pub const RE_OP_NOTSPACE: ::core::ffi::c_int = 16;
pub const RE_OP_BOUNDARY: ::core::ffi::c_int = 17;
pub const RE_OP_ATSTART: ::core::ffi::c_int = 18;
unsafe extern "C" fn re_add_state(
    mut pSet: *mut ReStateSet,
    mut newState: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < (*pSet).nState {
            if *(*pSet).aState.offset(i as isize) as ::core::ffi::c_int == newState {
                return;
            }
            i = i.wrapping_add(1);
        }
        let c2rust_fresh0 = (*pSet).nState;
        (*pSet).nState = (*pSet).nState.wrapping_add(1);
        *(*pSet).aState.offset(c2rust_fresh0 as isize) = newState as ReStateNumber;
    }
}
unsafe extern "C" fn re_next_char(mut p: *mut ReInput) -> ::core::ffi::c_uint {
    unsafe {
        let mut c: ::core::ffi::c_uint = 0;
        if (*p).i >= (*p).mx {
            return 0 as ::core::ffi::c_uint;
        }
        let c2rust_fresh1 = (*p).i;
        (*p).i = (*p).i + 1;
        c = *(*p).z.offset(c2rust_fresh1 as isize) as ::core::ffi::c_uint;
        if c >= 0x80 as ::core::ffi::c_uint {
            if c & 0xe0 as ::core::ffi::c_uint == 0xc0 as ::core::ffi::c_uint
                && (*p).i < (*p).mx
                && *(*p).z.offset((*p).i as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int
            {
                let c2rust_fresh2 = (*p).i;
                (*p).i = (*p).i + 1;
                c = (c & 0x1f as ::core::ffi::c_uint) << 6 as ::core::ffi::c_int
                    | (*(*p).z.offset(c2rust_fresh2 as isize) as ::core::ffi::c_int
                        & 0x3f as ::core::ffi::c_int) as ::core::ffi::c_uint;
                if c < 0x80 as ::core::ffi::c_uint {
                    c = 0xfffd as ::core::ffi::c_uint;
                }
            } else if c & 0xf0 as ::core::ffi::c_uint == 0xe0 as ::core::ffi::c_uint
                && ((*p).i + 1 as ::core::ffi::c_int) < (*p).mx
                && *(*p).z.offset((*p).i as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int
                && *(*p).z.offset(((*p).i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int
            {
                c = (c & 0xf as ::core::ffi::c_uint) << 12 as ::core::ffi::c_int
                    | ((*(*p).z.offset((*p).i as isize) as ::core::ffi::c_int
                        & 0x3f as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                    | (*(*p).z.offset(((*p).i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                        as ::core::ffi::c_uint;
                (*p).i += 2 as ::core::ffi::c_int;
                if c <= 0x7ff as ::core::ffi::c_uint
                    || c >= 0xd800 as ::core::ffi::c_uint
                        && c <= 0xdfff as ::core::ffi::c_uint
                {
                    c = 0xfffd as ::core::ffi::c_uint;
                }
            } else if c & 0xf8 as ::core::ffi::c_uint == 0xf0 as ::core::ffi::c_uint
                && ((*p).i + 2 as ::core::ffi::c_int) < (*p).mx
                && *(*p).z.offset((*p).i as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int
                && *(*p).z.offset(((*p).i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int
                && *(*p).z.offset(((*p).i + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int
            {
                c = (c & 0x7 as ::core::ffi::c_uint) << 18 as ::core::ffi::c_int
                    | ((*(*p).z.offset((*p).i as isize) as ::core::ffi::c_int
                        & 0x3f as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                    | ((*(*p).z.offset(((*p).i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | (*(*p).z.offset(((*p).i + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                        as ::core::ffi::c_uint;
                (*p).i += 3 as ::core::ffi::c_int;
                if c <= 0xffff as ::core::ffi::c_uint
                    || c > 0x10ffff as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    c = 0xfffd as ::core::ffi::c_uint;
                }
            } else {
                c = 0xfffd as ::core::ffi::c_uint;
            }
        }
        return c;
    }
}
unsafe extern "C" fn re_next_char_nocase(mut p: *mut ReInput) -> ::core::ffi::c_uint {
    unsafe {
        let mut c: ::core::ffi::c_uint = re_next_char(p);
        if c >= 'A' as i32 as ::core::ffi::c_uint
            && c <= 'Z' as i32 as ::core::ffi::c_uint
        {
            c = c.wrapping_add(('a' as i32 - 'A' as i32) as ::core::ffi::c_uint);
        }
        return c;
    }
}
unsafe extern "C" fn re_word_char(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return (c >= '0' as i32 && c <= '9' as i32 || c >= 'a' as i32 && c <= 'z' as i32
            || c >= 'A' as i32 && c <= 'Z' as i32 || c == '_' as i32)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn re_digit_char(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return (c >= '0' as i32 && c <= '9' as i32) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn re_space_char(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return (c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32
            || c == '\r' as i32 || c == '\u{b}' as i32 || c == '\u{c}' as i32)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn sqlite3re_match(
    mut pRe: *mut ReCompiled,
    mut zIn: *const ::core::ffi::c_uchar,
    mut nIn: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut aStateSet: [ReStateSet; 2] = [ReStateSet {
            nState: 0,
            aState: ::core::ptr::null_mut::<ReStateNumber>(),
        }; 2];
        let mut pThis: *mut ReStateSet = ::core::ptr::null_mut::<ReStateSet>();
        let mut pNext: *mut ReStateSet = ::core::ptr::null_mut::<ReStateSet>();
        let mut aSpace: [ReStateNumber; 100] = [0; 100];
        let mut pToFree: *mut ReStateNumber = ::core::ptr::null_mut::<ReStateNumber>();
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut iSwap: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut c: ::core::ffi::c_int = RE_START;
        let mut cPrev: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut in_0: ReInput = ReInput {
            z: ::core::ptr::null::<::core::ffi::c_uchar>(),
            i: 0,
            mx: 0,
        };
        in_0.z = zIn;
        in_0.i = 0 as ::core::ffi::c_int;
        in_0.mx = if nIn >= 0 as ::core::ffi::c_int {
            nIn
        } else {
            strlen(zIn as *const ::core::ffi::c_char) as ::core::ffi::c_int
        };
        if (*pRe).nInit != 0 {
            let mut x: ::core::ffi::c_uchar = (*pRe)
                .zInit[0 as ::core::ffi::c_int as usize];
            while in_0.i + (*pRe).nInit <= in_0.mx
                && (*zIn.offset(in_0.i as isize) as ::core::ffi::c_int
                    != x as ::core::ffi::c_int
                    || strncmp(
                        (zIn as *const ::core::ffi::c_char).offset(in_0.i as isize),
                        &raw mut (*pRe).zInit as *mut ::core::ffi::c_uchar
                            as *const ::core::ffi::c_char,
                        (*pRe).nInit as size_t,
                    ) != 0 as ::core::ffi::c_int)
            {
                in_0.i += 1;
            }
            if in_0.i + (*pRe).nInit > in_0.mx {
                return 0 as ::core::ffi::c_int;
            }
            c = RE_START - 1 as ::core::ffi::c_int;
        }
        if (*pRe).nState as usize
            <= (::core::mem::size_of::<[ReStateNumber; 100]>() as usize)
                .wrapping_div(
                    (::core::mem::size_of::<ReStateNumber>() as usize)
                        .wrapping_mul(2 as usize),
                )
        {
            pToFree = ::core::ptr::null_mut::<ReStateNumber>();
            aStateSet[0 as ::core::ffi::c_int as usize].aState = &raw mut aSpace
                as *mut ReStateNumber;
        } else {
            pToFree = sqlite3_malloc64(
                (::core::mem::size_of::<ReStateNumber>() as usize)
                    .wrapping_mul(2 as usize)
                    .wrapping_mul((*pRe).nState as usize) as sqlite3_uint64,
            ) as *mut ReStateNumber;
            if pToFree.is_null() {
                return -1 as ::core::ffi::c_int;
            }
            aStateSet[0 as ::core::ffi::c_int as usize].aState = pToFree;
        }
        aStateSet[1 as ::core::ffi::c_int as usize].aState = (*(&raw mut aStateSet
            as *mut ReStateSet)
            .offset(0 as ::core::ffi::c_int as isize))
            .aState
            .offset((*pRe).nState as isize) as *mut ReStateNumber;
        pNext = (&raw mut aStateSet as *mut ReStateSet)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ReStateSet;
        (*pNext).nState = 0 as ::core::ffi::c_uint;
        re_add_state(pNext, 0 as ::core::ffi::c_int);
        's_107: loop {
            if !(c != RE_EOF && (*pNext).nState > 0 as ::core::ffi::c_uint) {
                c2rust_current_block = 15855550149339537395;
                break;
            }
            cPrev = c;
            c = (*pRe).xNextChar.expect("non-null function pointer")(&raw mut in_0)
                as ::core::ffi::c_int;
            pThis = pNext;
            pNext = (&raw mut aStateSet as *mut ReStateSet).offset(iSwap as isize)
                as *mut ReStateSet;
            iSwap = (1 as ::core::ffi::c_uint).wrapping_sub(iSwap);
            (*pNext).nState = 0 as ::core::ffi::c_uint;
            i = 0 as ::core::ffi::c_uint;
            while i < (*pThis).nState {
                let mut x_0: ::core::ffi::c_int = *(*pThis).aState.offset(i as isize)
                    as ::core::ffi::c_int;
                match *(*pRe).aOp.offset(x_0 as isize) as ::core::ffi::c_int {
                    RE_OP_MATCH => {
                        if *(*pRe).aArg.offset(x_0 as isize) == c {
                            re_add_state(pNext, x_0 + 1 as ::core::ffi::c_int);
                        }
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_ATSTART => {
                        if cPrev == RE_START {
                            re_add_state(pThis, x_0 + 1 as ::core::ffi::c_int);
                        }
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_ANY => {
                        if c != 0 as ::core::ffi::c_int {
                            re_add_state(pNext, x_0 + 1 as ::core::ffi::c_int);
                        }
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_WORD => {
                        if re_word_char(c) != 0 {
                            re_add_state(pNext, x_0 + 1 as ::core::ffi::c_int);
                        }
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_NOTWORD => {
                        if re_word_char(c) == 0 && c != 0 as ::core::ffi::c_int {
                            re_add_state(pNext, x_0 + 1 as ::core::ffi::c_int);
                        }
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_DIGIT => {
                        if re_digit_char(c) != 0 {
                            re_add_state(pNext, x_0 + 1 as ::core::ffi::c_int);
                        }
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_NOTDIGIT => {
                        if re_digit_char(c) == 0 && c != 0 as ::core::ffi::c_int {
                            re_add_state(pNext, x_0 + 1 as ::core::ffi::c_int);
                        }
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_SPACE => {
                        if re_space_char(c) != 0 {
                            re_add_state(pNext, x_0 + 1 as ::core::ffi::c_int);
                        }
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_NOTSPACE => {
                        if re_space_char(c) == 0 && c != 0 as ::core::ffi::c_int {
                            re_add_state(pNext, x_0 + 1 as ::core::ffi::c_int);
                        }
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_BOUNDARY => {
                        if re_word_char(c) != re_word_char(cPrev) {
                            re_add_state(pThis, x_0 + 1 as ::core::ffi::c_int);
                        }
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_ANYSTAR => {
                        re_add_state(pNext, x_0);
                        re_add_state(pThis, x_0 + 1 as ::core::ffi::c_int);
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_FORK => {
                        re_add_state(pThis, x_0 + *(*pRe).aArg.offset(x_0 as isize));
                        re_add_state(pThis, x_0 + 1 as ::core::ffi::c_int);
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_GOTO => {
                        re_add_state(pThis, x_0 + *(*pRe).aArg.offset(x_0 as isize));
                        c2rust_current_block = 2232869372362427478;
                    }
                    RE_OP_ACCEPT => {
                        rc = 1 as ::core::ffi::c_int;
                        c2rust_current_block = 10610501227418742877;
                        break 's_107;
                    }
                    RE_OP_CC_EXC => {
                        if c == 0 as ::core::ffi::c_int {
                            c2rust_current_block = 2232869372362427478;
                        } else {
                            c2rust_current_block = 3580086814630675314;
                        }
                    }
                    RE_OP_CC_INC => {
                        c2rust_current_block = 3580086814630675314;
                    }
                    _ => {
                        c2rust_current_block = 2232869372362427478;
                    }
                }
                match c2rust_current_block {
                    3580086814630675314 => {
                        let mut j: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        let mut n: ::core::ffi::c_int = *(*pRe)
                            .aArg
                            .offset(x_0 as isize);
                        let mut hit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        j = 1 as ::core::ffi::c_int;
                        while j > 0 as ::core::ffi::c_int && j < n {
                            if *(*pRe).aOp.offset((x_0 + j) as isize)
                                as ::core::ffi::c_int == RE_OP_CC_VALUE
                            {
                                if *(*pRe).aArg.offset((x_0 + j) as isize) == c {
                                    hit = 1 as ::core::ffi::c_int;
                                    j = -1 as ::core::ffi::c_int;
                                }
                            } else if *(*pRe).aArg.offset((x_0 + j) as isize) <= c
                                && *(*pRe)
                                    .aArg
                                    .offset((x_0 + j + 1 as ::core::ffi::c_int) as isize) >= c
                            {
                                hit = 1 as ::core::ffi::c_int;
                                j = -1 as ::core::ffi::c_int;
                            } else {
                                j += 1;
                            }
                            j += 1;
                        }
                        if *(*pRe).aOp.offset(x_0 as isize) as ::core::ffi::c_int
                            == RE_OP_CC_EXC
                        {
                            hit = (hit == 0) as ::core::ffi::c_int;
                        }
                        if hit != 0 {
                            re_add_state(pNext, x_0 + n);
                        }
                    }
                    _ => {}
                }
                i = i.wrapping_add(1);
            }
        }
        match c2rust_current_block {
            15855550149339537395 => {
                i = 0 as ::core::ffi::c_uint;
                while i < (*pNext).nState {
                    let mut x_1: ::core::ffi::c_int = *(*pNext).aState.offset(i as isize)
                        as ::core::ffi::c_int;
                    while *(*pRe).aOp.offset(x_1 as isize) as ::core::ffi::c_int
                        == RE_OP_GOTO
                    {
                        x_1 += *(*pRe).aArg.offset(x_1 as isize);
                    }
                    if *(*pRe).aOp.offset(x_1 as isize) as ::core::ffi::c_int
                        == RE_OP_ACCEPT
                    {
                        rc = 1 as ::core::ffi::c_int;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
            }
            _ => {}
        }
        sqlite3_free(pToFree as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn re_resize(
    mut p: *mut ReCompiled,
    mut N: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aOp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut aArg: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        if N > (*p).mxAlloc {
            (*p).zErr = b"REGEXP pattern too big\0".as_ptr()
                as *const ::core::ffi::c_char;
            return 1 as ::core::ffi::c_int;
        }
        aOp = sqlite3_realloc64(
            (*p).aOp as *mut ::core::ffi::c_void,
            (N as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if aOp.is_null() {
            (*p).zErr = b"out of memory\0".as_ptr() as *const ::core::ffi::c_char;
            return 1 as ::core::ffi::c_int;
        }
        (*p).aOp = aOp;
        aArg = sqlite3_realloc64(
            (*p).aArg as *mut ::core::ffi::c_void,
            (N as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as sqlite3_uint64,
        ) as *mut ::core::ffi::c_int;
        if aArg.is_null() {
            (*p).zErr = b"out of memory\0".as_ptr() as *const ::core::ffi::c_char;
            return 1 as ::core::ffi::c_int;
        }
        (*p).aArg = aArg;
        (*p).nAlloc = N;
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn re_insert(
    mut p: *mut ReCompiled,
    mut iBefore: ::core::ffi::c_int,
    mut op: ::core::ffi::c_int,
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if (*p).nAlloc <= (*p).nState
            && re_resize(p, (*p).nAlloc.wrapping_mul(2 as ::core::ffi::c_uint)) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        i = (*p).nState as ::core::ffi::c_int;
        while i > iBefore {
            *(*p).aOp.offset(i as isize) = *(*p)
                .aOp
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            *(*p).aArg.offset(i as isize) = *(*p)
                .aArg
                .offset((i - 1 as ::core::ffi::c_int) as isize);
            i -= 1;
        }
        (*p).nState = (*p).nState.wrapping_add(1);
        *(*p).aOp.offset(iBefore as isize) = op as ::core::ffi::c_char;
        *(*p).aArg.offset(iBefore as isize) = arg;
        return iBefore;
    }
}
unsafe extern "C" fn re_append(
    mut p: *mut ReCompiled,
    mut op: ::core::ffi::c_int,
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return re_insert(p, (*p).nState as ::core::ffi::c_int, op, arg);
    }
}
unsafe extern "C" fn re_copy(
    mut p: *mut ReCompiled,
    mut iStart: ::core::ffi::c_int,
    mut N: ::core::ffi::c_uint,
) {
    unsafe {
        if (*p).nState.wrapping_add(N) >= (*p).nAlloc
            && re_resize(
                p,
                (*p).nAlloc.wrapping_mul(2 as ::core::ffi::c_uint).wrapping_add(N),
            ) != 0
        {
            return;
        }
        memcpy(
            (*p).aOp.offset((*p).nState as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            (*p).aOp.offset(iStart as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            (N as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
        memcpy(
            (*p).aArg.offset((*p).nState as isize) as *mut ::core::ffi::c_int
                as *mut ::core::ffi::c_void,
            (*p).aArg.offset(iStart as isize) as *mut ::core::ffi::c_int
                as *const ::core::ffi::c_void,
            (N as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        (*p).nState = (*p).nState.wrapping_add(N);
    }
}
unsafe extern "C" fn re_hex(
    mut c: ::core::ffi::c_int,
    mut pV: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if c >= '0' as i32 && c <= '9' as i32 {
            c -= '0' as i32;
        } else if c >= 'a' as i32 && c <= 'f' as i32 {
            c -= 'a' as i32 - 10 as ::core::ffi::c_int;
        } else if c >= 'A' as i32 && c <= 'F' as i32 {
            c -= 'A' as i32 - 10 as ::core::ffi::c_int;
        } else {
            return 0 as ::core::ffi::c_int
        }
        *pV = *pV * 16 as ::core::ffi::c_int + (c & 0xff as ::core::ffi::c_int);
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn re_esc_char(mut p: *mut ReCompiled) -> ::core::ffi::c_uint {
    unsafe {
        static mut zEsc: [::core::ffi::c_char; 21] = unsafe {
            ::core::mem::transmute::<
                [u8; 21],
                [::core::ffi::c_char; 21],
            >(*b"afnrtv\\()*.+?[$^{|}]\0")
        };
        static mut zTrans: [::core::ffi::c_char; 7] = unsafe {
            ::core::mem::transmute::<
                [u8; 7],
                [::core::ffi::c_char; 7],
            >(*b"\x07\x0C\n\r\t\x0B\0")
        };
        let mut i: ::core::ffi::c_int = 0;
        let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut c: ::core::ffi::c_char = 0;
        if (*p).sIn.i >= (*p).sIn.mx {
            return 0 as ::core::ffi::c_uint;
        }
        c = *(*p).sIn.z.offset((*p).sIn.i as isize) as ::core::ffi::c_char;
        if c as ::core::ffi::c_int == 'u' as i32
            && ((*p).sIn.i + 4 as ::core::ffi::c_int) < (*p).sIn.mx
        {
            let mut zIn: *const ::core::ffi::c_uchar = (*p)
                .sIn
                .z
                .offset((*p).sIn.i as isize);
            if re_hex(
                *zIn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                &raw mut v,
            ) != 0
                && re_hex(
                    *zIn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                    &raw mut v,
                ) != 0
                && re_hex(
                    *zIn.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                    &raw mut v,
                ) != 0
                && re_hex(
                    *zIn.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                    &raw mut v,
                ) != 0
            {
                (*p).sIn.i += 5 as ::core::ffi::c_int;
                return v as ::core::ffi::c_uint;
            }
        }
        if c as ::core::ffi::c_int == 'x' as i32
            && ((*p).sIn.i + 2 as ::core::ffi::c_int) < (*p).sIn.mx
        {
            let mut zIn_0: *const ::core::ffi::c_uchar = (*p)
                .sIn
                .z
                .offset((*p).sIn.i as isize);
            if re_hex(
                *zIn_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                &raw mut v,
            ) != 0
                && re_hex(
                    *zIn_0.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int,
                    &raw mut v,
                ) != 0
            {
                (*p).sIn.i += 3 as ::core::ffi::c_int;
                return v as ::core::ffi::c_uint;
            }
        }
        i = 0 as ::core::ffi::c_int;
        while zEsc[i as usize] as ::core::ffi::c_int != 0
            && zEsc[i as usize] as ::core::ffi::c_int != c as ::core::ffi::c_int
        {
            i += 1;
        }
        if zEsc[i as usize] != 0 {
            if i < 6 as ::core::ffi::c_int {
                c = zTrans[i as usize];
            }
            (*p).sIn.i += 1;
        } else {
            (*p).zErr = b"unknown \\ escape\0".as_ptr() as *const ::core::ffi::c_char;
        }
        return c as ::core::ffi::c_uint;
    }
}
unsafe extern "C" fn rePeek(mut p: *mut ReCompiled) -> ::core::ffi::c_uchar {
    unsafe {
        return (if (*p).sIn.i < (*p).sIn.mx {
            *(*p).sIn.z.offset((*p).sIn.i as isize) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uchar;
    }
}
unsafe extern "C" fn re_subcompile_re(
    mut p: *mut ReCompiled,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut iStart: ::core::ffi::c_int = 0;
        let mut iEnd: ::core::ffi::c_int = 0;
        let mut iGoto: ::core::ffi::c_int = 0;
        iStart = (*p).nState as ::core::ffi::c_int;
        zErr = re_subcompile_string(p);
        if !zErr.is_null() {
            return zErr;
        }
        while rePeek(p) as ::core::ffi::c_int == '|' as i32 {
            iEnd = (*p).nState as ::core::ffi::c_int;
            re_insert(p, iStart, RE_OP_FORK, iEnd + 2 as ::core::ffi::c_int - iStart);
            iGoto = re_append(p, RE_OP_GOTO, 0 as ::core::ffi::c_int);
            (*p).sIn.i += 1;
            zErr = re_subcompile_string(p);
            if !zErr.is_null() {
                return zErr;
            }
            *(*p).aArg.offset(iGoto as isize) = (*p)
                .nState
                .wrapping_sub(iGoto as ::core::ffi::c_uint) as ::core::ffi::c_int;
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn re_subcompile_string(
    mut p: *mut ReCompiled,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut iPrev: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iStart: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_uint = 0;
        let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        loop {
            c = (*p).xNextChar.expect("non-null function pointer")(&raw mut (*p).sIn);
            if !(c != 0 as ::core::ffi::c_uint) {
                break;
            }
            iStart = (*p).nState as ::core::ffi::c_int;
            match c {
                124 | 41 => {
                    (*p).sIn.i -= 1;
                    return ::core::ptr::null::<::core::ffi::c_char>();
                }
                40 => {
                    zErr = re_subcompile_re(p);
                    if !zErr.is_null() {
                        return zErr;
                    }
                    if rePeek(p) as ::core::ffi::c_int != ')' as i32 {
                        return b"unmatched '('\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    (*p).sIn.i += 1;
                }
                46 => {
                    if rePeek(p) as ::core::ffi::c_int == '*' as i32 {
                        re_append(p, RE_OP_ANYSTAR, 0 as ::core::ffi::c_int);
                        (*p).sIn.i += 1;
                    } else {
                        re_append(p, RE_OP_ANY, 0 as ::core::ffi::c_int);
                    }
                }
                42 => {
                    if iPrev < 0 as ::core::ffi::c_int {
                        return b"'*' without operand\0".as_ptr()
                            as *const ::core::ffi::c_char;
                    }
                    re_insert(
                        p,
                        iPrev,
                        RE_OP_GOTO,
                        (*p)
                            .nState
                            .wrapping_sub(iPrev as ::core::ffi::c_uint)
                            .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                    );
                    re_append(
                        p,
                        RE_OP_FORK,
                        (iPrev as ::core::ffi::c_uint)
                            .wrapping_sub((*p).nState)
                            .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                    );
                }
                43 => {
                    if iPrev < 0 as ::core::ffi::c_int {
                        return b"'+' without operand\0".as_ptr()
                            as *const ::core::ffi::c_char;
                    }
                    re_append(
                        p,
                        RE_OP_FORK,
                        (iPrev as ::core::ffi::c_uint).wrapping_sub((*p).nState)
                            as ::core::ffi::c_int,
                    );
                }
                63 => {
                    if iPrev < 0 as ::core::ffi::c_int {
                        return b"'?' without operand\0".as_ptr()
                            as *const ::core::ffi::c_char;
                    }
                    re_insert(
                        p,
                        iPrev,
                        RE_OP_FORK,
                        (*p)
                            .nState
                            .wrapping_sub(iPrev as ::core::ffi::c_uint)
                            .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                    );
                }
                36 => {
                    re_append(p, RE_OP_MATCH, RE_EOF);
                }
                94 => {
                    re_append(p, RE_OP_ATSTART, 0 as ::core::ffi::c_int);
                }
                123 => {
                    let mut m: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                    let mut n: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                    let mut sz: ::core::ffi::c_uint = 0;
                    let mut j: ::core::ffi::c_uint = 0;
                    if iPrev < 0 as ::core::ffi::c_int {
                        return b"'{m,n}' without operand\0".as_ptr()
                            as *const ::core::ffi::c_char;
                    }
                    loop {
                        c = rePeek(p) as ::core::ffi::c_uint;
                        if !(c >= '0' as i32 as ::core::ffi::c_uint
                            && c <= '9' as i32 as ::core::ffi::c_uint)
                        {
                            break;
                        }
                        m = m
                            .wrapping_mul(10 as ::core::ffi::c_uint)
                            .wrapping_add(c)
                            .wrapping_sub('0' as i32 as ::core::ffi::c_uint);
                        if m.wrapping_mul(2 as ::core::ffi::c_uint) > (*p).mxAlloc {
                            return b"REGEXP pattern too big\0".as_ptr()
                                as *const ::core::ffi::c_char;
                        }
                        (*p).sIn.i += 1;
                    }
                    n = m;
                    if c == ',' as i32 as ::core::ffi::c_uint {
                        (*p).sIn.i += 1;
                        n = 0 as ::core::ffi::c_uint;
                        loop {
                            c = rePeek(p) as ::core::ffi::c_uint;
                            if !(c >= '0' as i32 as ::core::ffi::c_uint
                                && c <= '9' as i32 as ::core::ffi::c_uint)
                            {
                                break;
                            }
                            n = n
                                .wrapping_mul(10 as ::core::ffi::c_uint)
                                .wrapping_add(c)
                                .wrapping_sub('0' as i32 as ::core::ffi::c_uint);
                            if n.wrapping_mul(2 as ::core::ffi::c_uint) > (*p).mxAlloc {
                                return b"REGEXP pattern too big\0".as_ptr()
                                    as *const ::core::ffi::c_char;
                            }
                            (*p).sIn.i += 1;
                        }
                    }
                    if c != '}' as i32 as ::core::ffi::c_uint {
                        return b"unmatched '{'\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    if n < m {
                        return b"n less than m in '{m,n}'\0".as_ptr()
                            as *const ::core::ffi::c_char;
                    }
                    (*p).sIn.i += 1;
                    sz = (*p).nState.wrapping_sub(iPrev as ::core::ffi::c_uint);
                    if m == 0 as ::core::ffi::c_uint {
                        if n == 0 as ::core::ffi::c_uint {
                            return b"both m and n are zero in '{m,n}'\0".as_ptr()
                                as *const ::core::ffi::c_char;
                        }
                        re_insert(
                            p,
                            iPrev,
                            RE_OP_FORK,
                            sz.wrapping_add(1 as ::core::ffi::c_uint)
                                as ::core::ffi::c_int,
                        );
                        iPrev += 1;
                        n = n.wrapping_sub(1);
                    } else {
                        j = 1 as ::core::ffi::c_uint;
                        while j < m {
                            re_copy(p, iPrev, sz);
                            j = j.wrapping_add(1);
                        }
                    }
                    j = m;
                    while j < n {
                        re_append(
                            p,
                            RE_OP_FORK,
                            sz.wrapping_add(1 as ::core::ffi::c_uint)
                                as ::core::ffi::c_int,
                        );
                        re_copy(p, iPrev, sz);
                        j = j.wrapping_add(1);
                    }
                    if n == 0 as ::core::ffi::c_uint && m > 0 as ::core::ffi::c_uint {
                        re_append(p, RE_OP_FORK, -(sz as ::core::ffi::c_int));
                    }
                }
                91 => {
                    let mut iFirst: ::core::ffi::c_uint = (*p).nState;
                    if rePeek(p) as ::core::ffi::c_int == '^' as i32 {
                        re_append(p, RE_OP_CC_EXC, 0 as ::core::ffi::c_int);
                        (*p).sIn.i += 1;
                    } else {
                        re_append(p, RE_OP_CC_INC, 0 as ::core::ffi::c_int);
                    }
                    loop {
                        c = (*p)
                            .xNextChar
                            .expect("non-null function pointer")(&raw mut (*p).sIn);
                        if !(c != 0 as ::core::ffi::c_uint) {
                            break;
                        }
                        if c == '[' as i32 as ::core::ffi::c_uint
                            && rePeek(p) as ::core::ffi::c_int == ':' as i32
                        {
                            return b"POSIX character classes not supported\0".as_ptr()
                                as *const ::core::ffi::c_char;
                        }
                        if c == '\\' as i32 as ::core::ffi::c_uint {
                            c = re_esc_char(p);
                        }
                        if rePeek(p) as ::core::ffi::c_int == '-' as i32 {
                            re_append(p, RE_OP_CC_RANGE, c as ::core::ffi::c_int);
                            (*p).sIn.i += 1;
                            c = (*p)
                                .xNextChar
                                .expect("non-null function pointer")(&raw mut (*p).sIn);
                            if c == '\\' as i32 as ::core::ffi::c_uint {
                                c = re_esc_char(p);
                            }
                            re_append(p, RE_OP_CC_RANGE, c as ::core::ffi::c_int);
                        } else {
                            re_append(p, RE_OP_CC_VALUE, c as ::core::ffi::c_int);
                        }
                        if !(rePeek(p) as ::core::ffi::c_int == ']' as i32) {
                            continue;
                        }
                        (*p).sIn.i += 1;
                        break;
                    }
                    if c == 0 as ::core::ffi::c_uint {
                        return b"unclosed '['\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    if (*p).nState > iFirst {
                        *(*p).aArg.offset(iFirst as isize) = (*p)
                            .nState
                            .wrapping_sub(iFirst) as ::core::ffi::c_int;
                    }
                }
                92 => {
                    let mut specialOp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    match rePeek(p) as ::core::ffi::c_int {
                        98 => {
                            specialOp = RE_OP_BOUNDARY;
                        }
                        100 => {
                            specialOp = RE_OP_DIGIT;
                        }
                        68 => {
                            specialOp = RE_OP_NOTDIGIT;
                        }
                        115 => {
                            specialOp = RE_OP_SPACE;
                        }
                        83 => {
                            specialOp = RE_OP_NOTSPACE;
                        }
                        119 => {
                            specialOp = RE_OP_WORD;
                        }
                        87 => {
                            specialOp = RE_OP_NOTWORD;
                        }
                        _ => {}
                    }
                    if specialOp != 0 {
                        (*p).sIn.i += 1;
                        re_append(p, specialOp, 0 as ::core::ffi::c_int);
                    } else {
                        c = re_esc_char(p);
                        re_append(p, RE_OP_MATCH, c as ::core::ffi::c_int);
                    }
                }
                _ => {
                    re_append(p, RE_OP_MATCH, c as ::core::ffi::c_int);
                }
            }
            iPrev = iStart;
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn sqlite3re_free(mut pRe: *mut ReCompiled) {
    unsafe {
        if !pRe.is_null() {
            sqlite3_free((*pRe).aOp as *mut ::core::ffi::c_void);
            sqlite3_free((*pRe).aArg as *mut ::core::ffi::c_void);
            sqlite3_free(pRe as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn re_free_voidptr(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        sqlite3re_free(p as *mut ReCompiled);
    }
}
unsafe extern "C" fn sqlite3re_compile(
    mut ppRe: *mut *mut ReCompiled,
    mut zIn: *const ::core::ffi::c_char,
    mut mxRe: ::core::ffi::c_int,
    mut noCase: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut pRe: *mut ReCompiled = ::core::ptr::null_mut::<ReCompiled>();
        let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        *ppRe = ::core::ptr::null_mut::<ReCompiled>();
        pRe = sqlite3_malloc(::core::mem::size_of::<ReCompiled>() as ::core::ffi::c_int)
            as *mut ReCompiled;
        if pRe.is_null() {
            return b"out of memory\0".as_ptr() as *const ::core::ffi::c_char;
        }
        memset(
            pRe as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ReCompiled>() as size_t,
        );
        (*pRe).xNextChar = (if noCase != 0 {
            Some(
                re_next_char_nocase
                    as unsafe extern "C" fn(*mut ReInput) -> ::core::ffi::c_uint,
            )
        } else {
            Some(
                re_next_char as unsafe extern "C" fn(*mut ReInput) -> ::core::ffi::c_uint,
            )
        }) as Option<unsafe extern "C" fn(*mut ReInput) -> ::core::ffi::c_uint>;
        (*pRe).mxAlloc = mxRe as ::core::ffi::c_uint;
        if re_resize(pRe, 30 as ::core::ffi::c_uint) != 0 {
            zErr = (*pRe).zErr;
            sqlite3re_free(pRe);
            return zErr;
        }
        if *zIn.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '^' as i32
        {
            zIn = zIn.offset(1);
        } else {
            re_append(pRe, RE_OP_ANYSTAR, 0 as ::core::ffi::c_int);
        }
        (*pRe).sIn.z = zIn as *mut ::core::ffi::c_uchar;
        (*pRe).sIn.i = 0 as ::core::ffi::c_int;
        (*pRe).sIn.mx = strlen(zIn) as ::core::ffi::c_int;
        zErr = re_subcompile_re(pRe);
        if !zErr.is_null() {
            sqlite3re_free(pRe);
            return zErr;
        }
        if (*pRe).sIn.i >= (*pRe).sIn.mx {
            re_append(pRe, RE_OP_ACCEPT, 0 as ::core::ffi::c_int);
            *ppRe = pRe;
        } else {
            sqlite3re_free(pRe);
            return b"unrecognized character\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if *(*pRe).aOp.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == RE_OP_ANYSTAR && noCase == 0
        {
            j = 0 as ::core::ffi::c_int;
            i = 1 as ::core::ffi::c_int;
            while j
                < ::core::mem::size_of::<[::core::ffi::c_uchar; 12]>()
                    as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                && *(*pRe).aOp.offset(i as isize) as ::core::ffi::c_int == RE_OP_MATCH
            {
                let mut x: ::core::ffi::c_uint = *(*pRe).aArg.offset(i as isize)
                    as ::core::ffi::c_uint;
                if x <= 0x7f as ::core::ffi::c_uint {
                    let c2rust_fresh3 = j;
                    j = j + 1;
                    (*pRe).zInit[c2rust_fresh3 as usize] = x as ::core::ffi::c_uchar;
                } else if x <= 0x7ff as ::core::ffi::c_uint {
                    let c2rust_fresh4 = j;
                    j = j + 1;
                    (*pRe).zInit[c2rust_fresh4 as usize] = (0xc0 as ::core::ffi::c_uint
                        | x >> 6 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    let c2rust_fresh5 = j;
                    j = j + 1;
                    (*pRe).zInit[c2rust_fresh5 as usize] = (0x80 as ::core::ffi::c_uint
                        | x & 0x3f as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
                } else {
                    if !(x <= 0xffff as ::core::ffi::c_uint) {
                        break;
                    }
                    let c2rust_fresh6 = j;
                    j = j + 1;
                    (*pRe).zInit[c2rust_fresh6 as usize] = (0xe0 as ::core::ffi::c_uint
                        | x >> 12 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    let c2rust_fresh7 = j;
                    j = j + 1;
                    (*pRe).zInit[c2rust_fresh7 as usize] = (0x80 as ::core::ffi::c_uint
                        | x >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_uint)
                        as ::core::ffi::c_uchar;
                    let c2rust_fresh8 = j;
                    j = j + 1;
                    (*pRe).zInit[c2rust_fresh8 as usize] = (0x80 as ::core::ffi::c_uint
                        | x & 0x3f as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
                }
                i += 1;
            }
            if j > 0 as ::core::ffi::c_int
                && (*pRe).zInit[(j - 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                j -= 1;
            }
            (*pRe).nInit = j;
        }
        return (*pRe).zErr;
    }
}
unsafe extern "C" fn re_maxlen(mut context: *mut sqlite3_context) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        return 75 as ::core::ffi::c_int
            + sqlite3_limit(
                db,
                SQLITE_LIMIT_LIKE_PATTERN_LENGTH,
                -1 as ::core::ffi::c_int,
            ) / 2 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn re_sql_func(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pRe: *mut ReCompiled = ::core::ptr::null_mut::<ReCompiled>();
        let mut zPattern: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zStr: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut setAux: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pRe = sqlite3_get_auxdata(context, 0 as ::core::ffi::c_int) as *mut ReCompiled;
        if pRe.is_null() {
            zPattern = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
            if zPattern.is_null() {
                return;
            }
            zErr = sqlite3re_compile(
                &raw mut pRe,
                zPattern,
                re_maxlen(context),
                !sqlite3_user_data(context).is_null() as ::core::ffi::c_int,
            );
            if !zErr.is_null() {
                sqlite3re_free(pRe);
                sqlite3_result_error(context, zErr, -1 as ::core::ffi::c_int);
                return;
            }
            if pRe.is_null() {
                sqlite3_result_error_nomem(context);
                return;
            }
            setAux = 1 as ::core::ffi::c_int;
        }
        zStr = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize));
        if !zStr.is_null() {
            sqlite3_result_int(
                context,
                sqlite3re_match(pRe, zStr, -1 as ::core::ffi::c_int),
            );
        }
        if setAux != 0 {
            sqlite3_set_auxdata(
                context,
                0 as ::core::ffi::c_int,
                pRe as *mut ::core::ffi::c_void,
                Some(
                    re_free_voidptr
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_regexp_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        rc = sqlite3_create_function(
            db,
            b"regexp\0".as_ptr() as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
            SQLITE_UTF8 | SQLITE_INNOCUOUS | SQLITE_DETERMINISTIC,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                re_sql_func
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
                b"regexpi\0".as_ptr() as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
                SQLITE_UTF8 | SQLITE_INNOCUOUS | SQLITE_DETERMINISTIC,
                db as *mut ::core::ffi::c_void,
                Some(
                    re_sql_func
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
        return rc;
    }
}
