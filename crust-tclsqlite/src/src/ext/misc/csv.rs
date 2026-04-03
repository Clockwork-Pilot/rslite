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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_vsnprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
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
    fn sqlite3_str_new(_: *mut sqlite3) -> *mut sqlite3_str;
    fn sqlite3_str_finish(_: *mut sqlite3_str) -> *mut ::core::ffi::c_char;
    fn sqlite3_str_appendf(
        _: *mut sqlite3_str,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
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
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type i64_0 = sqlite3_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsvReader {
    pub in_0: *mut FILE,
    pub z: *mut ::core::ffi::c_char,
    pub n: i64_0,
    pub nAlloc: i64_0,
    pub nLine: i64_0,
    pub bNotFirst: ::core::ffi::c_int,
    pub cTerm: ::core::ffi::c_int,
    pub iIn: size_t,
    pub nIn: size_t,
    pub zIn: *mut ::core::ffi::c_char,
    pub zErr: [::core::ffi::c_char; 200],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsvTable {
    pub base: sqlite3_vtab,
    pub zFilename: *mut ::core::ffi::c_char,
    pub zData: *mut ::core::ffi::c_char,
    pub iStart: ::core::ffi::c_long,
    pub nCol: ::core::ffi::c_int,
    pub tstFlags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsvCursor {
    pub base: sqlite3_vtab_cursor,
    pub rdr: CsvReader,
    pub azVal: *mut *mut ::core::ffi::c_char,
    pub aLen: *mut ::core::ffi::c_int,
    pub iRowid: sqlite3_int64,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LIKE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_GLOB: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const SQLITE_VTAB_DIRECTONLY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CSV_MXERR: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const CSV_INBUFSZ: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C" fn csv_reader_init(mut p: *mut CsvReader) {
    unsafe {
        (*p).in_0 = ::core::ptr::null_mut::<FILE>();
        (*p).z = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*p).n = 0 as i64_0;
        (*p).nAlloc = 0 as i64_0;
        (*p).nLine = 0 as i64_0;
        (*p).bNotFirst = 0 as ::core::ffi::c_int;
        (*p).nIn = 0 as size_t;
        (*p).zIn = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*p).zErr[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn csv_reader_reset(mut p: *mut CsvReader) {
    unsafe {
        if !(*p).in_0.is_null() {
            fclose((*p).in_0);
            sqlite3_free((*p).zIn as *mut ::core::ffi::c_void);
        }
        sqlite3_free((*p).z as *mut ::core::ffi::c_void);
        csv_reader_init(p);
    }
}
unsafe extern "C" fn csv_errmsg(
    mut p: *mut CsvReader,
    mut zFormat: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut ap = c2rust_args.clone();
        sqlite3_vsnprintf(
            CSV_MXERR,
            &raw mut (*p).zErr as *mut ::core::ffi::c_char,
            zFormat,
            ap,
        );
    }
}
unsafe extern "C" fn csv_reader_open(
    mut p: *mut CsvReader,
    mut zFilename: *const ::core::ffi::c_char,
    mut zData: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if !zFilename.is_null() {
            (*p).zIn = sqlite3_malloc(CSV_INBUFSZ) as *mut ::core::ffi::c_char;
            if (*p).zIn.is_null() {
                csv_errmsg(p, b"out of memory\0".as_ptr() as *const ::core::ffi::c_char);
                return 1 as ::core::ffi::c_int;
            }
            (*p).in_0 = fopen(zFilename, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
            if (*p).in_0.is_null() {
                sqlite3_free((*p).zIn as *mut ::core::ffi::c_void);
                csv_reader_reset(p);
                csv_errmsg(
                    p,
                    b"cannot open '%s' for reading\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zFilename,
                );
                return 1 as ::core::ffi::c_int;
            }
        } else {
            (*p).zIn = zData as *mut ::core::ffi::c_char;
            (*p).nIn = strlen(zData);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn csv_getc_refill(mut p: *mut CsvReader) -> ::core::ffi::c_int {
    unsafe {
        let mut got: size_t = 0;
        got = fread(
            (*p).zIn as *mut ::core::ffi::c_void,
            1 as size_t,
            CSV_INBUFSZ as size_t,
            (*p).in_0,
        ) as size_t;
        if got == 0 as size_t {
            return EOF;
        }
        (*p).nIn = got;
        (*p).iIn = 1 as size_t;
        return *(*p).zIn.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn csv_getc(mut p: *mut CsvReader) -> ::core::ffi::c_int {
    unsafe {
        if (*p).iIn >= (*p).nIn {
            if !(*p).in_0.is_null() {
                return csv_getc_refill(p);
            }
            return EOF;
        }
        let c2rust_fresh0 = (*p).iIn;
        (*p).iIn = (*p).iIn.wrapping_add(1);
        return *((*p).zIn as *mut ::core::ffi::c_uchar).offset(c2rust_fresh0 as isize)
            as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn csv_resize_and_append(
    mut p: *mut CsvReader,
    mut c: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nNew: i64_0 = (*p).nAlloc * 2 as i64_0 + 100 as i64_0;
        zNew = sqlite3_realloc64(
            (*p).z as *mut ::core::ffi::c_void,
            nNew as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if !zNew.is_null() {
            (*p).z = zNew;
            (*p).nAlloc = nNew;
            let c2rust_fresh1 = (*p).n;
            (*p).n = (*p).n + 1;
            *(*p).z.offset(c2rust_fresh1 as isize) = c;
            return 0 as ::core::ffi::c_int;
        } else {
            csv_errmsg(p, b"out of memory\0".as_ptr() as *const ::core::ffi::c_char);
            return 1 as ::core::ffi::c_int;
        };
    }
}
unsafe extern "C" fn csv_append(
    mut p: *mut CsvReader,
    mut c: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).n
            >= (*p).nAlloc as ::core::ffi::c_longlong - 1 as ::core::ffi::c_longlong
        {
            return csv_resize_and_append(p, c);
        }
        let c2rust_fresh2 = (*p).n;
        (*p).n = (*p).n + 1;
        *(*p).z.offset(c2rust_fresh2 as isize) = c;
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn csv_read_one_field(
    mut p: *mut CsvReader,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut c: ::core::ffi::c_int = 0;
        (*p).n = 0 as i64_0;
        c = csv_getc(p);
        if c == EOF {
            (*p).cTerm = EOF;
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if c == '"' as i32 {
            let mut pc: ::core::ffi::c_int = 0;
            let mut ppc: ::core::ffi::c_int = 0;
            let mut startLine: ::core::ffi::c_int = (*p).nLine as ::core::ffi::c_int;
            ppc = 0 as ::core::ffi::c_int;
            pc = ppc;
            loop {
                c = csv_getc(p);
                if c <= '"' as i32 || pc == '"' as i32 {
                    if c == '\n' as i32 {
                        (*p).nLine += 1;
                    }
                    if c == '"' as i32 {
                        if pc == '"' as i32 {
                            pc = 0 as ::core::ffi::c_int;
                            continue;
                        }
                    }
                    if c == ',' as i32 && pc == '"' as i32
                        || c == '\n' as i32 && pc == '"' as i32
                        || c == '\n' as i32 && pc == '\r' as i32 && ppc == '"' as i32
                        || c == EOF && pc == '"' as i32
                    {
                        loop {
                            (*p).n -= 1;
                            if !(*(*p).z.offset((*p).n as isize) as ::core::ffi::c_int
                                != '"' as i32)
                            {
                                break;
                            }
                        }
                        (*p).cTerm = c as ::core::ffi::c_char as ::core::ffi::c_int;
                        break;
                    } else if pc == '"' as i32 && c != '\r' as i32 {
                        csv_errmsg(
                            p,
                            b"line %d: unescaped %c character\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*p).nLine,
                            '"' as i32,
                        );
                        break;
                    } else if c == EOF {
                        csv_errmsg(
                            p,
                            b"line %d: unterminated %c-quoted field\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            startLine,
                            '"' as i32,
                        );
                        (*p).cTerm = c as ::core::ffi::c_char as ::core::ffi::c_int;
                        break;
                    }
                }
                if csv_append(p, c as ::core::ffi::c_char) != 0 {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                ppc = pc;
                pc = c;
            }
        } else {
            if c & 0xff as ::core::ffi::c_int == 0xef as ::core::ffi::c_int
                && (*p).bNotFirst == 0 as ::core::ffi::c_int
            {
                csv_append(p, c as ::core::ffi::c_char);
                c = csv_getc(p);
                if c & 0xff as ::core::ffi::c_int == 0xbb as ::core::ffi::c_int {
                    csv_append(p, c as ::core::ffi::c_char);
                    c = csv_getc(p);
                    if c & 0xff as ::core::ffi::c_int == 0xbf as ::core::ffi::c_int {
                        (*p).bNotFirst = 1 as ::core::ffi::c_int;
                        (*p).n = 0 as i64_0;
                        return csv_read_one_field(p);
                    }
                }
            }
            while c > ',' as i32 || c != EOF && c != ',' as i32 && c != '\n' as i32 {
                if csv_append(p, c as ::core::ffi::c_char) != 0 {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                c = csv_getc(p);
            }
            if c == '\n' as i32 {
                (*p).nLine += 1;
                if (*p).n > 0 as ::core::ffi::c_longlong
                    && *(*p)
                        .z
                        .offset(
                            ((*p).n as ::core::ffi::c_longlong
                                - 1 as ::core::ffi::c_longlong) as isize,
                        ) as ::core::ffi::c_int == '\r' as i32
                {
                    (*p).n -= 1;
                }
            }
            (*p).cTerm = c as ::core::ffi::c_char as ::core::ffi::c_int;
        }
        if !(*p).z.is_null() {
            *(*p).z.offset((*p).n as isize) = 0 as ::core::ffi::c_char;
        }
        (*p).bNotFirst = 1 as ::core::ffi::c_int;
        return (*p).z;
    }
}
pub const CSVTEST_FIDX: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
unsafe extern "C" fn csv_xfer_error(mut pTab: *mut CsvTable, mut pRdr: *mut CsvReader) {
    unsafe {
        sqlite3_free((*pTab).base.zErrMsg as *mut ::core::ffi::c_void);
        (*pTab).base.zErrMsg = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*pRdr).zErr as *mut ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn csvtabDisconnect(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut CsvTable = pVtab as *mut CsvTable;
        sqlite3_free((*p).zFilename as *mut ::core::ffi::c_void);
        sqlite3_free((*p).zData as *mut ::core::ffi::c_void);
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn csv_skip_whitespace(
    mut z: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        while *(*__ctype_b_loc())
            .offset(
                *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                    as ::core::ffi::c_int as isize,
            ) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            z = z.offset(1);
        }
        return z;
    }
}
unsafe extern "C" fn csv_trim_whitespace(mut z: *mut ::core::ffi::c_char) {
    unsafe {
        let mut n: size_t = strlen(z);
        while n > 0 as size_t
            && *(*__ctype_b_loc())
                .offset(
                    *z.offset(n as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
                        as isize,
                ) as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
        {
            n = n.wrapping_sub(1);
        }
        *z.offset(n as isize) = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn csv_dequote(mut z: *mut ::core::ffi::c_char) {
    unsafe {
        let mut j: ::core::ffi::c_int = 0;
        let mut cQuote: ::core::ffi::c_char = *z
            .offset(0 as ::core::ffi::c_int as isize);
        let mut i: size_t = 0;
        let mut n: size_t = 0;
        if cQuote as ::core::ffi::c_int != '\'' as i32
            && cQuote as ::core::ffi::c_int != '"' as i32
        {
            return;
        }
        n = strlen(z);
        if n < 2 as size_t
            || *z.offset(n.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                != *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            return;
        }
        i = 1 as size_t;
        j = 0 as ::core::ffi::c_int;
        while i < n.wrapping_sub(1 as size_t) {
            if *z.offset(i as isize) as ::core::ffi::c_int
                == cQuote as ::core::ffi::c_int
                && *z.offset(i.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                    == cQuote as ::core::ffi::c_int
            {
                i = i.wrapping_add(1);
            }
            let c2rust_fresh3 = j;
            j = j + 1;
            *z.offset(c2rust_fresh3 as isize) = *z.offset(i as isize);
            i = i.wrapping_add(1);
        }
        *z.offset(j as isize) = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn csv_parameter(
    mut zTag: *const ::core::ffi::c_char,
    mut nTag: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        z = csv_skip_whitespace(z);
        if strncmp(zTag, z, nTag as size_t) != 0 as ::core::ffi::c_int {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        z = csv_skip_whitespace(z.offset(nTag as isize));
        if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '=' as i32
        {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return csv_skip_whitespace(z.offset(1 as ::core::ffi::c_int as isize));
    }
}
unsafe extern "C" fn csv_string_parameter(
    mut p: *mut CsvReader,
    mut zParam: *const ::core::ffi::c_char,
    mut zArg: *const ::core::ffi::c_char,
    mut pzVal: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zValue: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        zValue = csv_parameter(zParam, strlen(zParam) as ::core::ffi::c_int, zArg);
        if zValue.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*p).zErr[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        if !(*pzVal).is_null() {
            csv_errmsg(
                p,
                b"more than one '%s' parameter\0".as_ptr() as *const ::core::ffi::c_char,
                zParam,
            );
            return 1 as ::core::ffi::c_int;
        }
        *pzVal = sqlite3_mprintf(b"%s\0".as_ptr() as *const ::core::ffi::c_char, zValue);
        if (*pzVal).is_null() {
            csv_errmsg(p, b"out of memory\0".as_ptr() as *const ::core::ffi::c_char);
            return 1 as ::core::ffi::c_int;
        }
        csv_trim_whitespace(*pzVal);
        csv_dequote(*pzVal);
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn csv_boolean(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_stricmp(b"yes\0".as_ptr() as *const ::core::ffi::c_char, z)
            == 0 as ::core::ffi::c_int
            || sqlite3_stricmp(b"on\0".as_ptr() as *const ::core::ffi::c_char, z)
                == 0 as ::core::ffi::c_int
            || sqlite3_stricmp(b"true\0".as_ptr() as *const ::core::ffi::c_char, z)
                == 0 as ::core::ffi::c_int
            || *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '1' as i32
                && *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        if sqlite3_stricmp(b"no\0".as_ptr() as *const ::core::ffi::c_char, z)
            == 0 as ::core::ffi::c_int
            || sqlite3_stricmp(b"off\0".as_ptr() as *const ::core::ffi::c_char, z)
                == 0 as ::core::ffi::c_int
            || sqlite3_stricmp(b"false\0".as_ptr() as *const ::core::ffi::c_char, z)
                == 0 as ::core::ffi::c_int
            || *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '0' as i32
                && *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn csv_boolean_parameter(
    mut zTag: *const ::core::ffi::c_char,
    mut nTag: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
    mut pValue: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut b: ::core::ffi::c_int = 0;
        z = csv_skip_whitespace(z);
        if strncmp(zTag, z, nTag as size_t) != 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        z = csv_skip_whitespace(z.offset(nTag as isize));
        if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            *pValue = 1 as ::core::ffi::c_int;
            return 1 as ::core::ffi::c_int;
        }
        if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '=' as i32
        {
            return 0 as ::core::ffi::c_int;
        }
        z = csv_skip_whitespace(z.offset(1 as ::core::ffi::c_int as isize));
        b = csv_boolean(z);
        if b >= 0 as ::core::ffi::c_int {
            *pValue = b;
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn csvtabConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pNew: *mut CsvTable = ::core::ptr::null_mut::<CsvTable>();
        let mut bHeader: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut tstFlags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut b: ::core::ffi::c_int = 0;
        let mut nCol: ::core::ffi::c_int = -99 as ::core::ffi::c_int;
        let mut sRdr: CsvReader = CsvReader {
            in_0: ::core::ptr::null_mut::<FILE>(),
            z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            n: 0,
            nAlloc: 0,
            nLine: 0,
            bNotFirst: 0,
            cTerm: 0,
            iIn: 0,
            nIn: 0,
            zIn: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            zErr: [0; 200],
        };
        static mut azParam: [*const ::core::ffi::c_char; 3] = [
            b"filename\0".as_ptr() as *const ::core::ffi::c_char,
            b"data\0".as_ptr() as *const ::core::ffi::c_char,
            b"schema\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut azPValue: [*mut ::core::ffi::c_char; 3] = [::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >(); 3];
        memset(
            &raw mut sRdr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<CsvReader>() as size_t,
        );
        memset(
            &raw mut azPValue as *mut *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[*mut ::core::ffi::c_char; 3]>() as size_t,
        );
        i = 3 as ::core::ffi::c_int;
        loop {
            if !(i < argc) {
                c2rust_current_block = 4761528863920922185;
                break;
            }
            let mut z: *const ::core::ffi::c_char = *argv.offset(i as isize);
            let mut zValue: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            j = 0 as ::core::ffi::c_int;
            while (j as usize)
                < (::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>() as usize)
                    .wrapping_div(
                        ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                    )
            {
                if csv_string_parameter(
                    &raw mut sRdr,
                    azParam[j as usize],
                    z,
                    (&raw mut azPValue as *mut *mut ::core::ffi::c_char)
                        .offset(j as isize) as *mut *mut ::core::ffi::c_char,
                ) != 0
                {
                    break;
                }
                j += 1;
            }
            if (j as usize)
                < (::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>() as usize)
                    .wrapping_div(
                        ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                    )
            {
                if sRdr.zErr[0 as ::core::ffi::c_int as usize] != 0 {
                    c2rust_current_block = 15749539317656514632;
                    break;
                }
            } else if csv_boolean_parameter(
                b"header\0".as_ptr() as *const ::core::ffi::c_char,
                6 as ::core::ffi::c_int,
                z,
                &raw mut b,
            ) != 0
            {
                if bHeader >= 0 as ::core::ffi::c_int {
                    csv_errmsg(
                        &raw mut sRdr,
                        b"more than one 'header' parameter\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    c2rust_current_block = 15749539317656514632;
                    break;
                } else {
                    bHeader = b;
                }
            } else {
                zValue = csv_parameter(
                    b"testflags\0".as_ptr() as *const ::core::ffi::c_char,
                    9 as ::core::ffi::c_int,
                    z,
                );
                if !zValue.is_null() {
                    tstFlags = atoi(zValue) as ::core::ffi::c_uint as ::core::ffi::c_int;
                } else {
                    zValue = csv_parameter(
                        b"columns\0".as_ptr() as *const ::core::ffi::c_char,
                        7 as ::core::ffi::c_int,
                        z,
                    );
                    if !zValue.is_null() {
                        if nCol > 0 as ::core::ffi::c_int {
                            csv_errmsg(
                                &raw mut sRdr,
                                b"more than one 'columns' parameter\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            c2rust_current_block = 15749539317656514632;
                            break;
                        } else {
                            nCol = atoi(zValue);
                            if nCol <= 0 as ::core::ffi::c_int {
                                csv_errmsg(
                                    &raw mut sRdr,
                                    b"column= value must be positive\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                c2rust_current_block = 15749539317656514632;
                                break;
                            }
                        }
                    } else {
                        csv_errmsg(
                            &raw mut sRdr,
                            b"bad parameter: '%s'\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            z,
                        );
                        c2rust_current_block = 15749539317656514632;
                        break;
                    }
                }
            }
            i += 1;
        }
        match c2rust_current_block {
            4761528863920922185 => {
                if azPValue[0 as ::core::ffi::c_int as usize].is_null()
                    as ::core::ffi::c_int
                    == azPValue[1 as ::core::ffi::c_int as usize].is_null()
                        as ::core::ffi::c_int
                {
                    csv_errmsg(
                        &raw mut sRdr,
                        b"must specify either filename= or data= but not both\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else if !((nCol <= 0 as ::core::ffi::c_int
                    || bHeader == 1 as ::core::ffi::c_int)
                    && csv_reader_open(
                        &raw mut sRdr,
                        azPValue[0 as ::core::ffi::c_int as usize],
                        azPValue[1 as ::core::ffi::c_int as usize],
                    ) != 0)
                {
                    pNew = sqlite3_malloc(
                        ::core::mem::size_of::<CsvTable>() as ::core::ffi::c_int,
                    ) as *mut CsvTable;
                    *ppVtab = pNew as *mut sqlite3_vtab;
                    if pNew.is_null() {
                        c2rust_current_block = 6943292423749570065;
                    } else {
                        memset(
                            pNew as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<CsvTable>() as size_t,
                        );
                        if azPValue[2 as ::core::ffi::c_int as usize].is_null() {
                            let mut pStr: *mut sqlite3_str = sqlite3_str_new(
                                ::core::ptr::null_mut::<sqlite3>(),
                            );
                            let mut zSep: *mut ::core::ffi::c_char = b"\0".as_ptr()
                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                            let mut iCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            sqlite3_str_appendf(
                                pStr,
                                b"CREATE TABLE x(\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            if nCol < 0 as ::core::ffi::c_int
                                && bHeader < 1 as ::core::ffi::c_int
                            {
                                nCol = 0 as ::core::ffi::c_int;
                                loop {
                                    csv_read_one_field(&raw mut sRdr);
                                    nCol += 1;
                                    if !(sRdr.cTerm == ',' as i32) {
                                        break;
                                    }
                                }
                            }
                            if nCol > 0 as ::core::ffi::c_int
                                && bHeader < 1 as ::core::ffi::c_int
                            {
                                iCol = 0 as ::core::ffi::c_int;
                                while iCol < nCol {
                                    sqlite3_str_appendf(
                                        pStr,
                                        b"%sc%d TEXT\0".as_ptr() as *const ::core::ffi::c_char,
                                        zSep,
                                        iCol,
                                    );
                                    zSep = b",\0".as_ptr() as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    iCol += 1;
                                }
                            } else {
                                loop {
                                    let mut z_0: *mut ::core::ffi::c_char = csv_read_one_field(
                                        &raw mut sRdr,
                                    );
                                    if nCol > 0 as ::core::ffi::c_int && iCol < nCol
                                        || nCol < 0 as ::core::ffi::c_int && bHeader != 0
                                    {
                                        sqlite3_str_appendf(
                                            pStr,
                                            b"%s\"%w\" TEXT\0".as_ptr() as *const ::core::ffi::c_char,
                                            zSep,
                                            z_0,
                                        );
                                        zSep = b",\0".as_ptr() as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char;
                                        iCol += 1;
                                    }
                                    if !(sRdr.cTerm == ',' as i32) {
                                        break;
                                    }
                                }
                                if nCol < 0 as ::core::ffi::c_int {
                                    nCol = iCol;
                                } else {
                                    while iCol < nCol {
                                        iCol += 1;
                                        sqlite3_str_appendf(
                                            pStr,
                                            b"%sc%d TEXT\0".as_ptr() as *const ::core::ffi::c_char,
                                            zSep,
                                            iCol,
                                        );
                                        zSep = b",\0".as_ptr() as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char;
                                    }
                                }
                            }
                            (*pNew).nCol = nCol;
                            sqlite3_str_appendf(
                                pStr,
                                b")\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            azPValue[2 as ::core::ffi::c_int as usize] = sqlite3_str_finish(
                                pStr,
                            );
                            if azPValue[2 as ::core::ffi::c_int as usize].is_null() {
                                c2rust_current_block = 6943292423749570065;
                            } else {
                                c2rust_current_block = 7385833325316299293;
                            }
                        } else {
                            if nCol < 0 as ::core::ffi::c_int {
                                loop {
                                    csv_read_one_field(&raw mut sRdr);
                                    (*pNew).nCol += 1;
                                    if !(sRdr.cTerm == ',' as i32) {
                                        break;
                                    }
                                }
                            } else {
                                (*pNew).nCol = nCol;
                            }
                            c2rust_current_block = 7385833325316299293;
                        }
                        match c2rust_current_block {
                            6943292423749570065 => {}
                            _ => {
                                (*pNew).zFilename = azPValue[0 as ::core::ffi::c_int
                                    as usize];
                                azPValue[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<
                                    ::core::ffi::c_char,
                                >();
                                (*pNew).zData = azPValue[1 as ::core::ffi::c_int as usize];
                                azPValue[1 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<
                                    ::core::ffi::c_char,
                                >();
                                (*pNew).tstFlags = tstFlags as ::core::ffi::c_uint;
                                if bHeader != 1 as ::core::ffi::c_int {
                                    (*pNew).iStart = 0 as ::core::ffi::c_long;
                                } else if !(*pNew).zData.is_null() {
                                    (*pNew).iStart = sRdr.iIn as ::core::ffi::c_int
                                        as ::core::ffi::c_long;
                                } else {
                                    (*pNew).iStart = (ftell(sRdr.in_0) as size_t)
                                        .wrapping_sub(sRdr.nIn)
                                        .wrapping_add(sRdr.iIn) as ::core::ffi::c_int
                                        as ::core::ffi::c_long;
                                }
                                csv_reader_reset(&raw mut sRdr);
                                rc = sqlite3_declare_vtab(
                                    db,
                                    azPValue[2 as ::core::ffi::c_int as usize],
                                );
                                if rc != 0 {
                                    csv_errmsg(
                                        &raw mut sRdr,
                                        b"bad schema: '%s' - %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        azPValue[2 as ::core::ffi::c_int as usize],
                                        sqlite3_errmsg(db),
                                    );
                                } else {
                                    i = 0 as ::core::ffi::c_int;
                                    while (i as usize)
                                        < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 3]>()
                                            as usize)
                                            .wrapping_div(
                                                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                                            )
                                    {
                                        sqlite3_free(
                                            azPValue[i as usize] as *mut ::core::ffi::c_void,
                                        );
                                        i += 1;
                                    }
                                    sqlite3_vtab_config(db, SQLITE_VTAB_DIRECTONLY);
                                    return SQLITE_OK;
                                }
                                c2rust_current_block = 15749539317656514632;
                            }
                        }
                    }
                    match c2rust_current_block {
                        15749539317656514632 => {}
                        _ => {
                            rc = SQLITE_NOMEM;
                            csv_errmsg(
                                &raw mut sRdr,
                                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        if !pNew.is_null() {
            csvtabDisconnect(&raw mut (*pNew).base);
        }
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 3]>() as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                )
        {
            sqlite3_free(azPValue[i as usize] as *mut ::core::ffi::c_void);
            i += 1;
        }
        if sRdr.zErr[0 as ::core::ffi::c_int as usize] != 0 {
            sqlite3_free(*pzErr as *mut ::core::ffi::c_void);
            *pzErr = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sRdr.zErr as *mut ::core::ffi::c_char,
            );
        }
        csv_reader_reset(&raw mut sRdr);
        if rc == SQLITE_OK {
            rc = SQLITE_ERROR;
        }
        return rc;
    }
}
unsafe extern "C" fn csvtabCursorRowReset(mut pCur: *mut CsvCursor) {
    unsafe {
        let mut pTab: *mut CsvTable = (*pCur).base.pVtab as *mut CsvTable;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pTab).nCol {
            sqlite3_free(*(*pCur).azVal.offset(i as isize) as *mut ::core::ffi::c_void);
            let ref mut c2rust_fresh4 = *(*pCur).azVal.offset(i as isize);
            *c2rust_fresh4 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            *(*pCur).aLen.offset(i as isize) = 0 as ::core::ffi::c_int;
            i += 1;
        }
    }
}
unsafe extern "C" fn csvtabCreate(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return csvtabConnect(db, pAux, argc, argv, ppVtab, pzErr);
    }
}
unsafe extern "C" fn csvtabClose(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut CsvCursor = cur as *mut CsvCursor;
        csvtabCursorRowReset(pCur);
        csv_reader_reset(&raw mut (*pCur).rdr);
        sqlite3_free(cur as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn csvtabOpen(
    mut p: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut CsvTable = p as *mut CsvTable;
        let mut pCur: *mut CsvCursor = ::core::ptr::null_mut::<CsvCursor>();
        let mut nByte: size_t = 0;
        nByte = (::core::mem::size_of::<CsvCursor>() as usize)
            .wrapping_add(
                (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                    .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    .wrapping_mul((*pTab).nCol as usize),
            ) as size_t;
        pCur = sqlite3_malloc64(nByte as sqlite3_uint64) as *mut CsvCursor;
        if pCur.is_null() {
            return SQLITE_NOMEM;
        }
        memset(pCur as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, nByte);
        (*pCur).azVal = pCur.offset(1 as ::core::ffi::c_int as isize) as *mut CsvCursor
            as *mut *mut ::core::ffi::c_char;
        (*pCur).aLen = (*pCur).azVal.offset((*pTab).nCol as isize)
            as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_int;
        *ppCursor = &raw mut (*pCur).base;
        if csv_reader_open(&raw mut (*pCur).rdr, (*pTab).zFilename, (*pTab).zData) != 0 {
            csv_xfer_error(pTab, &raw mut (*pCur).rdr);
            return SQLITE_ERROR;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn csvtabNext(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut CsvCursor = cur as *mut CsvCursor;
        let mut pTab: *mut CsvTable = (*cur).pVtab as *mut CsvTable;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        loop {
            z = csv_read_one_field(&raw mut (*pCur).rdr);
            if z.is_null() {
                break;
            }
            if i < (*pTab).nCol {
                if (*(*pCur).aLen.offset(i as isize) as ::core::ffi::c_longlong)
                    < (*pCur).rdr.n as ::core::ffi::c_longlong
                        + 1 as ::core::ffi::c_longlong
                {
                    let mut zNew: *mut ::core::ffi::c_char = sqlite3_realloc64(
                        *(*pCur).azVal.offset(i as isize) as *mut ::core::ffi::c_void,
                        ((*pCur).rdr.n as ::core::ffi::c_longlong
                            + 1 as ::core::ffi::c_longlong) as sqlite3_uint64,
                    ) as *mut ::core::ffi::c_char;
                    if zNew.is_null() {
                        csv_errmsg(
                            &raw mut (*pCur).rdr,
                            b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        csv_xfer_error(pTab, &raw mut (*pCur).rdr);
                        break;
                    } else {
                        let ref mut c2rust_fresh5 = *(*pCur).azVal.offset(i as isize);
                        *c2rust_fresh5 = zNew;
                        *(*pCur).aLen.offset(i as isize) = ((*pCur).rdr.n
                            as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                            as ::core::ffi::c_int;
                    }
                }
                memcpy(
                    *(*pCur).azVal.offset(i as isize) as *mut ::core::ffi::c_void,
                    z as *const ::core::ffi::c_void,
                    ((*pCur).rdr.n as ::core::ffi::c_longlong
                        + 1 as ::core::ffi::c_longlong) as size_t,
                );
                i += 1;
            }
            if !((*pCur).rdr.cTerm == ',' as i32) {
                break;
            }
        }
        if z.is_null() && i == 0 as ::core::ffi::c_int {
            (*pCur).iRowid = -1 as ::core::ffi::c_int as sqlite3_int64;
        } else {
            (*pCur).iRowid += 1;
            while i < (*pTab).nCol {
                sqlite3_free(
                    *(*pCur).azVal.offset(i as isize) as *mut ::core::ffi::c_void,
                );
                let ref mut c2rust_fresh6 = *(*pCur).azVal.offset(i as isize);
                *c2rust_fresh6 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                *(*pCur).aLen.offset(i as isize) = 0 as ::core::ffi::c_int;
                i += 1;
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn csvtabColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut CsvCursor = cur as *mut CsvCursor;
        let mut pTab: *mut CsvTable = (*cur).pVtab as *mut CsvTable;
        if i >= 0 as ::core::ffi::c_int && i < (*pTab).nCol
            && !(*(*pCur).azVal.offset(i as isize)).is_null()
        {
            sqlite3_result_text(
                ctx,
                *(*pCur).azVal.offset(i as isize),
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn csvtabRowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut CsvCursor = cur as *mut CsvCursor;
        *pRowid = (*pCur).iRowid as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn csvtabEof(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut CsvCursor = cur as *mut CsvCursor;
        return ((*pCur).iRowid < 0 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn csvtabFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut CsvCursor = pVtabCursor as *mut CsvCursor;
        let mut pTab: *mut CsvTable = (*pVtabCursor).pVtab as *mut CsvTable;
        (*pCur).iRowid = 0 as sqlite3_int64;
        if csv_append(&raw mut (*pCur).rdr, 0 as ::core::ffi::c_char) != 0 {
            return SQLITE_NOMEM;
        }
        if (*pCur).rdr.in_0.is_null() {
            (*pCur).rdr.iIn = (*pTab).iStart as size_t;
        } else {
            fseek((*pCur).rdr.in_0, (*pTab).iStart, SEEK_SET);
            (*pCur).rdr.iIn = 0 as size_t;
            (*pCur).rdr.nIn = 0 as size_t;
        }
        return csvtabNext(pVtabCursor);
    }
}
unsafe extern "C" fn csvtabBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        (*pIdxInfo).estimatedCost = 1000000 as ::core::ffi::c_int
            as ::core::ffi::c_double;
        if (*(tab as *mut CsvTable)).tstFlags & CSVTEST_FIDX as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            let mut i: ::core::ffi::c_int = 0;
            let mut nConst: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < (*pIdxInfo).nConstraint {
                let mut op: ::core::ffi::c_uchar = 0;
                if !((*(*pIdxInfo).aConstraint.offset(i as isize)).usable
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                {
                    op = (*(*pIdxInfo).aConstraint.offset(i as isize)).op;
                    if op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_EQ
                        || op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_LIKE
                        || op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_GLOB
                    {
                        (*pIdxInfo).estimatedCost = 10 as ::core::ffi::c_int
                            as ::core::ffi::c_double;
                        (*(*pIdxInfo).aConstraintUsage.offset(nConst as isize))
                            .argvIndex = nConst + 1 as ::core::ffi::c_int;
                        nConst += 1;
                    }
                }
                i += 1;
            }
        }
        return SQLITE_OK;
    }
}
static mut CsvModule: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: Some(
            csvtabCreate
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
            csvtabConnect
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
            csvtabBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            csvtabDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            csvtabDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            csvtabOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            csvtabClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            csvtabFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            csvtabNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            csvtabEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            csvtabColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            csvtabRowid
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
unsafe extern "C" fn csvtabUpdate(
    mut p: *mut sqlite3_vtab,
    mut n: ::core::ffi::c_int,
    mut v: *mut *mut sqlite3_value,
    mut x: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_READONLY;
    }
}
static mut CsvModuleFauxWrite: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: Some(
            csvtabCreate
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
            csvtabConnect
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
            csvtabBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            csvtabDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            csvtabDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            csvtabOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            csvtabClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            csvtabFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            csvtabNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            csvtabEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            csvtabColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            csvtabRowid
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: Some(
            csvtabUpdate
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                    *mut sqlite3_int64,
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
pub unsafe extern "C" fn sqlite3_csv_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3_create_module(
            db,
            b"csv\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut CsvModule,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        if rc == SQLITE_OK {
            rc = sqlite3_create_module(
                db,
                b"csv_wr\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut CsvModuleFauxWrite,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        return rc;
    }
}
