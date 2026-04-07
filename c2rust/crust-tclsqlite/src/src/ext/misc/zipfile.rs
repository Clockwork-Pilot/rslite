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
    pub type internal_state;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_vmprintf(
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
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_aggregate_context(
        _: *mut sqlite3_context,
        nBytes: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
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
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
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
    fn sqlite3_overload_function(
        _: *mut sqlite3,
        zFuncName: *const ::core::ffi::c_char,
        nArg: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
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
    fn sqlite3_vtab_nochange(_: *mut sqlite3_context) -> ::core::ffi::c_int;
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
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn deflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn deflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    fn inflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn inflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    fn deflateBound(strm: z_streamp, sourceLen: uLong) -> uLong;
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn deflateInit2_(
        strm: z_streamp,
        level: ::core::ffi::c_int,
        method: ::core::ffi::c_int,
        windowBits: ::core::ffi::c_int,
        memLevel: ::core::ffi::c_int,
        strategy: ::core::ffi::c_int,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: ::core::ffi::c_int,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut ::core::ffi::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut ::core::ffi::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: ::core::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
pub type i64_0 = sqlite3_int64;
pub type u8_0 = ::core::ffi::c_uchar;
pub type u32_0 = ::core::ffi::c_uint;
pub type u16_0 = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZipfileEOCD {
    pub iDisk: u16_0,
    pub iFirstDisk: u16_0,
    pub nEntry: u16_0,
    pub nEntryTotal: u16_0,
    pub nSize: u32_0,
    pub iOffset: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZipfileCDS {
    pub iVersionMadeBy: u16_0,
    pub iVersionExtract: u16_0,
    pub flags: u16_0,
    pub iCompression: u16_0,
    pub mTime: u16_0,
    pub mDate: u16_0,
    pub crc32: u32_0,
    pub szCompressed: u32_0,
    pub szUncompressed: u32_0,
    pub nFile: u16_0,
    pub nExtra: u16_0,
    pub nComment: u16_0,
    pub iDiskStart: u16_0,
    pub iInternalAttr: u16_0,
    pub iExternalAttr: u32_0,
    pub iOffset: u32_0,
    pub zFile: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZipfileLFH {
    pub iVersionExtract: u16_0,
    pub flags: u16_0,
    pub iCompression: u16_0,
    pub mTime: u16_0,
    pub mDate: u16_0,
    pub crc32: u32_0,
    pub szCompressed: u32_0,
    pub szUncompressed: u32_0,
    pub nFile: u16_0,
    pub nExtra: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZipfileEntry {
    pub cds: ZipfileCDS,
    pub mUnixTime: u32_0,
    pub aExtra: *mut u8_0,
    pub iDataOff: i64_0,
    pub aData: *mut u8_0,
    pub pNext: *mut ZipfileEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZipfileCsr {
    pub base: sqlite3_vtab_cursor,
    pub iId: i64_0,
    pub bEof: u8_0,
    pub bNoop: u8_0,
    pub pFile: *mut FILE,
    pub iNextOff: i64_0,
    pub eocd: ZipfileEOCD,
    pub pFreeEntry: *mut ZipfileEntry,
    pub pCurrent: *mut ZipfileEntry,
    pub pCsrNext: *mut ZipfileCsr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZipfileTab {
    pub base: sqlite3_vtab,
    pub zFile: *mut ::core::ffi::c_char,
    pub db: *mut sqlite3,
    pub aBuffer: *mut u8_0,
    pub pCsrList: *mut ZipfileCsr,
    pub iNextCsrid: i64_0,
    pub pFirstEntry: *mut ZipfileEntry,
    pub pLastEntry: *mut ZipfileEntry,
    pub pWriteFd: *mut FILE,
    pub szCurrent: i64_0,
    pub szOrig: i64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZipfileBuffer {
    pub a: *mut u8_0,
    pub n: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZipfileCtx {
    pub nEntry: ::core::ffi::c_int,
    pub body: ZipfileBuffer,
    pub cds: ZipfileBuffer,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_IGNORE: ::core::ffi::c_int = 2;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_VTAB_DIRECTONLY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_REPLACE: ::core::ffi::c_int = 5;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ZLIB_VERSION: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"1.2.13\0")
};
pub const Z_NO_FLUSH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_FINISH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const S_IFDIR: ::core::ffi::c_int = 0o40000 as ::core::ffi::c_int;
pub const S_IFREG: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const S_IFLNK: ::core::ffi::c_int = 0o120000 as ::core::ffi::c_int;
static mut ZIPFILE_SCHEMA: [::core::ffi::c_char; 91] = unsafe {
    ::core::mem::transmute::<
        [u8; 91],
        [::core::ffi::c_char; 91],
    >(
        *b"CREATE TABLE y(name PRIMARY KEY,mode,mtime,sz,rawdata,data,method,z HIDDEN) WITHOUT ROWID;\0",
    )
};
pub const ZIPFILE_F_COLUMN_IDX: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const ZIPFILE_MX_NAME: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
pub const ZIPFILE_BUFFER_SIZE: ::core::ffi::c_int = 200 as ::core::ffi::c_int
    * 1024 as ::core::ffi::c_int;
pub const ZIPFILE_EXTRA_TIMESTAMP: ::core::ffi::c_int = 21589;
pub const ZIPFILE_NEWENTRY_MADEBY: ::core::ffi::c_int = ((3 as ::core::ffi::c_int)
    << 8 as ::core::ffi::c_int) + 30 as ::core::ffi::c_int;
pub const ZIPFILE_NEWENTRY_REQUIRED: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const ZIPFILE_NEWENTRY_FLAGS: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const ZIPFILE_SIGNATURE_CDS: ::core::ffi::c_int = 0x2014b50 as ::core::ffi::c_int;
pub const ZIPFILE_SIGNATURE_LFH: ::core::ffi::c_int = 0x4034b50 as ::core::ffi::c_int;
pub const ZIPFILE_LFH_FIXED_SZ: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const ZIPFILE_EOCD_FIXED_SZ: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ZIPFILE_CDS_FIXED_SZ: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
unsafe extern "C" fn zipfileCtxErrorMsg(
    mut ctx: *mut sqlite3_context,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut zMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ap = c2rust_args.clone();
        zMsg = sqlite3_vmprintf(zFmt, ap);
        sqlite3_result_error(ctx, zMsg, -1 as ::core::ffi::c_int);
        sqlite3_free(zMsg as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn zipfileDequote(mut zIn: *mut ::core::ffi::c_char) {
    unsafe {
        let mut q: ::core::ffi::c_char = *zIn.offset(0 as ::core::ffi::c_int as isize);
        if q as ::core::ffi::c_int == '"' as i32
            || q as ::core::ffi::c_int == '\'' as i32
            || q as ::core::ffi::c_int == '`' as i32
            || q as ::core::ffi::c_int == '[' as i32
        {
            let mut iIn: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if q as ::core::ffi::c_int == '[' as i32 {
                q = ']' as i32 as ::core::ffi::c_char;
            }
            while *zIn.offset(iIn as isize) != 0 {
                let c2rust_fresh0 = iIn;
                iIn = iIn + 1;
                let mut c: ::core::ffi::c_char = *zIn.offset(c2rust_fresh0 as isize);
                if c as ::core::ffi::c_int == q as ::core::ffi::c_int
                    && {
                        let c2rust_fresh1 = iIn;
                        iIn = iIn + 1;
                        *zIn.offset(c2rust_fresh1 as isize) as ::core::ffi::c_int
                            != q as ::core::ffi::c_int
                    }
                {
                    break;
                }
                let c2rust_fresh2 = iOut;
                iOut = iOut + 1;
                *zIn.offset(c2rust_fresh2 as isize) = c;
            }
            *zIn.offset(iOut as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
    }
}
unsafe extern "C" fn zipfileConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nByte: ::core::ffi::c_int = (::core::mem::size_of::<ZipfileTab>()
            as usize)
            .wrapping_add(ZIPFILE_BUFFER_SIZE as usize) as ::core::ffi::c_int;
        let mut nFile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zFile: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pNew: *mut ZipfileTab = ::core::ptr::null_mut::<ZipfileTab>();
        let mut rc: ::core::ffi::c_int = 0;
        if 0 as ::core::ffi::c_int
            != sqlite3_stricmp(
                *argv.offset(2 as ::core::ffi::c_int as isize),
                b"zipfile\0".as_ptr() as *const ::core::ffi::c_char,
            ) && argc < 4 as ::core::ffi::c_int || argc > 4 as ::core::ffi::c_int
        {
            *pzErr = sqlite3_mprintf(
                b"zipfile constructor requires one argument\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return SQLITE_ERROR;
        }
        if argc > 3 as ::core::ffi::c_int {
            zFile = *argv.offset(3 as ::core::ffi::c_int as isize);
            nFile = strlen(zFile) as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        }
        rc = sqlite3_declare_vtab(
            db,
            &raw const ZIPFILE_SCHEMA as *const ::core::ffi::c_char,
        );
        if rc == SQLITE_OK {
            pNew = sqlite3_malloc64(
                (nByte as ::core::ffi::c_longlong + nFile as ::core::ffi::c_longlong)
                    as sqlite3_uint64,
            ) as *mut ZipfileTab;
            if pNew.is_null() {
                return SQLITE_NOMEM;
            }
            memset(
                pNew as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (nByte + nFile) as size_t,
            );
            (*pNew).db = db;
            (*pNew).aBuffer = pNew.offset(1 as ::core::ffi::c_int as isize)
                as *mut ZipfileTab as *mut u8_0;
            if !zFile.is_null() {
                (*pNew).zFile = (*pNew).aBuffer.offset(ZIPFILE_BUFFER_SIZE as isize)
                    as *mut u8_0 as *mut ::core::ffi::c_char;
                memcpy(
                    (*pNew).zFile as *mut ::core::ffi::c_void,
                    zFile as *const ::core::ffi::c_void,
                    nFile as size_t,
                );
                zipfileDequote((*pNew).zFile);
            }
        }
        sqlite3_vtab_config(db, SQLITE_VTAB_DIRECTONLY);
        *ppVtab = pNew as *mut sqlite3_vtab;
        return rc;
    }
}
unsafe extern "C" fn zipfileEntryFree(mut p: *mut ZipfileEntry) {
    unsafe {
        if !p.is_null() {
            sqlite3_free((*p).cds.zFile as *mut ::core::ffi::c_void);
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn zipfileCleanupTransaction(mut pTab: *mut ZipfileTab) {
    unsafe {
        let mut pEntry: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
        let mut pNext: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
        if !(*pTab).pWriteFd.is_null() {
            fclose((*pTab).pWriteFd);
            (*pTab).pWriteFd = ::core::ptr::null_mut::<FILE>();
        }
        pEntry = (*pTab).pFirstEntry;
        while !pEntry.is_null() {
            pNext = (*pEntry).pNext;
            zipfileEntryFree(pEntry);
            pEntry = pNext;
        }
        (*pTab).pFirstEntry = ::core::ptr::null_mut::<ZipfileEntry>();
        (*pTab).pLastEntry = ::core::ptr::null_mut::<ZipfileEntry>();
        (*pTab).szCurrent = 0 as i64_0;
        (*pTab).szOrig = 0 as i64_0;
    }
}
unsafe extern "C" fn zipfileDisconnect(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        zipfileCleanupTransaction(pVtab as *mut ZipfileTab);
        sqlite3_free(pVtab as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn zipfileOpen(
    mut p: *mut sqlite3_vtab,
    mut ppCsr: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut ZipfileTab = p as *mut ZipfileTab;
        let mut pCsr: *mut ZipfileCsr = ::core::ptr::null_mut::<ZipfileCsr>();
        pCsr = sqlite3_malloc(::core::mem::size_of::<ZipfileCsr>() as ::core::ffi::c_int)
            as *mut ZipfileCsr;
        *ppCsr = pCsr as *mut sqlite3_vtab_cursor;
        if pCsr.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pCsr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ZipfileCsr>() as size_t,
        );
        (*pTab).iNextCsrid += 1;
        (*pCsr).iId = (*pTab).iNextCsrid;
        (*pCsr).pCsrNext = (*pTab).pCsrList;
        (*pTab).pCsrList = pCsr;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn zipfileResetCursor(mut pCsr: *mut ZipfileCsr) {
    unsafe {
        let mut p: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
        let mut pNext: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
        (*pCsr).bEof = 0 as u8_0;
        if !(*pCsr).pFile.is_null() {
            fclose((*pCsr).pFile);
            (*pCsr).pFile = ::core::ptr::null_mut::<FILE>();
            zipfileEntryFree((*pCsr).pCurrent);
            (*pCsr).pCurrent = ::core::ptr::null_mut::<ZipfileEntry>();
        }
        p = (*pCsr).pFreeEntry;
        while !p.is_null() {
            pNext = (*p).pNext;
            zipfileEntryFree(p);
            p = pNext;
        }
    }
}
unsafe extern "C" fn zipfileClose(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut ZipfileCsr = cur as *mut ZipfileCsr;
        let mut pTab: *mut ZipfileTab = (*pCsr).base.pVtab as *mut ZipfileTab;
        let mut pp: *mut *mut ZipfileCsr = ::core::ptr::null_mut::<*mut ZipfileCsr>();
        zipfileResetCursor(pCsr);
        pp = &raw mut (*pTab).pCsrList;
        while *pp != pCsr {
            pp = &raw mut (**pp).pCsrNext;
        }
        *pp = (*pCsr).pCsrNext;
        sqlite3_free(pCsr as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn zipfileTableErr(
    mut pTab: *mut ZipfileTab,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut ap = c2rust_args.clone();
        sqlite3_free((*pTab).base.zErrMsg as *mut ::core::ffi::c_void);
        (*pTab).base.zErrMsg = sqlite3_vmprintf(zFmt, ap);
    }
}
unsafe extern "C" fn zipfileCursorErr(
    mut pCsr: *mut ZipfileCsr,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut ap = c2rust_args.clone();
        sqlite3_free((*(*pCsr).base.pVtab).zErrMsg as *mut ::core::ffi::c_void);
        (*(*pCsr).base.pVtab).zErrMsg = sqlite3_vmprintf(zFmt, ap);
    }
}
unsafe extern "C" fn zipfileReadData(
    mut pFile: *mut FILE,
    mut aRead: *mut u8_0,
    mut nRead: ::core::ffi::c_int,
    mut iOff: i64_0,
    mut pzErrmsg: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: size_t = 0;
        fseek(pFile, iOff as ::core::ffi::c_long, SEEK_SET);
        n = fread(aRead as *mut ::core::ffi::c_void, 1 as size_t, nRead as size_t, pFile)
            as size_t;
        if n as ::core::ffi::c_int != nRead {
            *pzErrmsg = sqlite3_mprintf(
                b"error in fread()\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return SQLITE_ERROR;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn zipfileAppendData(
    mut pTab: *mut ZipfileTab,
    mut aWrite: *const u8_0,
    mut nWrite: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if nWrite > 0 as ::core::ffi::c_int {
            let mut n: size_t = nWrite as size_t;
            fseek((*pTab).pWriteFd, (*pTab).szCurrent as ::core::ffi::c_long, SEEK_SET);
            n = fwrite(
                aWrite as *const ::core::ffi::c_void,
                1 as size_t,
                nWrite as size_t,
                (*pTab).pWriteFd,
            ) as size_t;
            if n as ::core::ffi::c_int != nWrite {
                (*pTab).base.zErrMsg = sqlite3_mprintf(
                    b"error in fwrite()\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return SQLITE_ERROR;
            }
            (*pTab).szCurrent += nWrite as ::core::ffi::c_longlong;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn zipfileGetU16(mut aBuf: *const u8_0) -> u16_0 {
    unsafe {
        return (((*aBuf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
            + *aBuf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as u16_0;
    }
}
unsafe extern "C" fn zipfileGetU32(mut aBuf: *const u8_0) -> u32_0 {
    unsafe {
        if aBuf.is_null() {
            return 0 as u32_0;
        }
        return ((*aBuf.offset(3 as ::core::ffi::c_int as isize) as u32_0)
            << 24 as ::core::ffi::c_int)
            .wrapping_add(
                (*aBuf.offset(2 as ::core::ffi::c_int as isize) as u32_0)
                    << 16 as ::core::ffi::c_int,
            )
            .wrapping_add(
                (*aBuf.offset(1 as ::core::ffi::c_int as isize) as u32_0)
                    << 8 as ::core::ffi::c_int,
            )
            .wrapping_add(
                (*aBuf.offset(0 as ::core::ffi::c_int as isize) as u32_0)
                    << 0 as ::core::ffi::c_int,
            );
    }
}
unsafe extern "C" fn zipfilePutU16(mut aBuf: *mut u8_0, mut val: u16_0) {
    unsafe {
        *aBuf.offset(0 as ::core::ffi::c_int as isize) = (val as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int) as u8_0;
        *aBuf.offset(1 as ::core::ffi::c_int as isize) = (val as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u8_0;
    }
}
unsafe extern "C" fn zipfilePutU32(mut aBuf: *mut u8_0, mut val: u32_0) {
    unsafe {
        *aBuf.offset(0 as ::core::ffi::c_int as isize) = (val as ::core::ffi::c_uint
            & 0xff as ::core::ffi::c_uint) as u8_0;
        *aBuf.offset(1 as ::core::ffi::c_int as isize) = (val as ::core::ffi::c_uint
            >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as u8_0;
        *aBuf.offset(2 as ::core::ffi::c_int as isize) = (val as ::core::ffi::c_uint
            >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as u8_0;
        *aBuf.offset(3 as ::core::ffi::c_int as isize) = (val as ::core::ffi::c_uint
            >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as u8_0;
    }
}
pub const ZIPFILE_CDS_NFILE_OFF: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const ZIPFILE_CDS_SZCOMPRESSED_OFF: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
unsafe extern "C" fn zipfileReadCDS(
    mut aBuf: *mut u8_0,
    mut pCDS: *mut ZipfileCDS,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aRead: *mut u8_0 = aBuf;
        aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
        let mut sig: u32_0 = zipfileGetU32(
            aRead.offset(-(4 as ::core::ffi::c_int as isize)),
        );
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if sig != ZIPFILE_SIGNATURE_CDS as ::core::ffi::c_uint {
            rc = SQLITE_ERROR;
        } else {
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).iVersionMadeBy = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).iVersionExtract = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).flags = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).iCompression = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).mTime = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).mDate = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
            (*pCDS).crc32 = zipfileGetU32(
                aRead.offset(-(4 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
            (*pCDS).szCompressed = zipfileGetU32(
                aRead.offset(-(4 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
            (*pCDS).szUncompressed = zipfileGetU32(
                aRead.offset(-(4 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).nFile = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).nExtra = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).nComment = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).iDiskStart = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pCDS).iInternalAttr = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
            (*pCDS).iExternalAttr = zipfileGetU32(
                aRead.offset(-(4 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
            (*pCDS).iOffset = zipfileGetU32(
                aRead.offset(-(4 as ::core::ffi::c_int as isize)),
            );
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileReadLFH(
    mut aBuffer: *mut u8_0,
    mut pLFH: *mut ZipfileLFH,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aRead: *mut u8_0 = aBuffer;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
        let mut sig: u32_0 = zipfileGetU32(
            aRead.offset(-(4 as ::core::ffi::c_int as isize)),
        );
        if sig != ZIPFILE_SIGNATURE_LFH as ::core::ffi::c_uint {
            rc = SQLITE_ERROR;
        } else {
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pLFH).iVersionExtract = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pLFH).flags = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pLFH).iCompression = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pLFH).mTime = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pLFH).mDate = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
            (*pLFH).crc32 = zipfileGetU32(
                aRead.offset(-(4 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
            (*pLFH).szCompressed = zipfileGetU32(
                aRead.offset(-(4 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
            (*pLFH).szUncompressed = zipfileGetU32(
                aRead.offset(-(4 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pLFH).nFile = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pLFH).nExtra = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            if (*pLFH).nFile as ::core::ffi::c_int > ZIPFILE_MX_NAME {
                rc = SQLITE_ERROR;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileScanExtra(
    mut aExtra: *mut u8_0,
    mut nExtra: ::core::ffi::c_int,
    mut pmTime: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p: *mut u8_0 = aExtra;
        let mut pEnd: *mut u8_0 = aExtra.offset(nExtra as isize) as *mut u8_0;
        while p < pEnd {
            p = p.offset(2 as ::core::ffi::c_int as isize);
            let mut id: u16_0 = zipfileGetU16(
                p.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            p = p.offset(2 as ::core::ffi::c_int as isize);
            let mut nByte: u16_0 = zipfileGetU16(
                p.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            match id as ::core::ffi::c_int {
                ZIPFILE_EXTRA_TIMESTAMP => {
                    let mut b: u8_0 = *p.offset(0 as ::core::ffi::c_int as isize);
                    if b as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
                        *pmTime = zipfileGetU32(
                            p.offset(1 as ::core::ffi::c_int as isize) as *mut u8_0,
                        );
                        ret = 1 as ::core::ffi::c_int;
                    }
                }
                _ => {}
            }
            p = p.offset(nByte as ::core::ffi::c_int as isize);
        }
        return ret;
    }
}
unsafe extern "C" fn zipfileMtime(mut pCDS: *mut ZipfileCDS) -> u32_0 {
    unsafe {
        let mut Y: ::core::ffi::c_int = 0;
        let mut M: ::core::ffi::c_int = 0;
        let mut D: ::core::ffi::c_int = 0;
        let mut X1: ::core::ffi::c_int = 0;
        let mut X2: ::core::ffi::c_int = 0;
        let mut A: ::core::ffi::c_int = 0;
        let mut B: ::core::ffi::c_int = 0;
        let mut sec: ::core::ffi::c_int = 0;
        let mut min: ::core::ffi::c_int = 0;
        let mut hr: ::core::ffi::c_int = 0;
        let mut JDsec: i64_0 = 0;
        Y = 1980 as ::core::ffi::c_int
            + ((*pCDS).mDate as ::core::ffi::c_int >> 9 as ::core::ffi::c_int
                & 0x7f as ::core::ffi::c_int);
        M = (*pCDS).mDate as ::core::ffi::c_int >> 5 as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int;
        D = (*pCDS).mDate as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int;
        sec = ((*pCDS).mTime as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
            * 2 as ::core::ffi::c_int;
        min = (*pCDS).mTime as ::core::ffi::c_int >> 5 as ::core::ffi::c_int
            & 0x3f as ::core::ffi::c_int;
        hr = (*pCDS).mTime as ::core::ffi::c_int >> 11 as ::core::ffi::c_int
            & 0x1f as ::core::ffi::c_int;
        if M <= 2 as ::core::ffi::c_int {
            Y -= 1;
            M += 12 as ::core::ffi::c_int;
        }
        X1 = 36525 as ::core::ffi::c_int * (Y + 4716 as ::core::ffi::c_int)
            / 100 as ::core::ffi::c_int;
        X2 = 306001 as ::core::ffi::c_int * (M + 1 as ::core::ffi::c_int)
            / 10000 as ::core::ffi::c_int;
        A = Y / 100 as ::core::ffi::c_int;
        B = 2 as ::core::ffi::c_int - A + A / 4 as ::core::ffi::c_int;
        JDsec = ((((X1 + X2 + D + B) as ::core::ffi::c_double - 1524.5f64)
            * 86400 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_longlong
            + (hr * 3600 as ::core::ffi::c_int) as ::core::ffi::c_longlong
            + (min * 60 as ::core::ffi::c_int) as ::core::ffi::c_longlong
            + sec as ::core::ffi::c_longlong) as i64_0;
        return (JDsec
            - 24405875 as ::core::ffi::c_int as i64_0
                * 8640 as ::core::ffi::c_int as i64_0) as u32_0;
    }
}
unsafe extern "C" fn zipfileMtimeToDos(mut pCds: *mut ZipfileCDS, mut mUnixTime: u32_0) {
    unsafe {
        let mut JD: i64_0 = 2440588 as ::core::ffi::c_int as i64_0
            + (mUnixTime as ::core::ffi::c_uint)
                .wrapping_div(
                    (24 as ::core::ffi::c_int * 60 as ::core::ffi::c_int
                        * 60 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as i64_0;
        let mut A: ::core::ffi::c_int = 0;
        let mut B: ::core::ffi::c_int = 0;
        let mut C: ::core::ffi::c_int = 0;
        let mut D: ::core::ffi::c_int = 0;
        let mut E: ::core::ffi::c_int = 0;
        let mut yr: ::core::ffi::c_int = 0;
        let mut mon: ::core::ffi::c_int = 0;
        let mut day: ::core::ffi::c_int = 0;
        let mut hr: ::core::ffi::c_int = 0;
        let mut min: ::core::ffi::c_int = 0;
        let mut sec: ::core::ffi::c_int = 0;
        A = ((JD as ::core::ffi::c_double - 1867216.25f64) / 36524.25f64)
            as ::core::ffi::c_int;
        A = (JD as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong
            + A as ::core::ffi::c_longlong
            - (A / 4 as ::core::ffi::c_int) as ::core::ffi::c_longlong)
            as ::core::ffi::c_int;
        B = A + 1524 as ::core::ffi::c_int;
        C = ((B as ::core::ffi::c_double - 122.1f64) / 365.25f64) as ::core::ffi::c_int;
        D = 36525 as ::core::ffi::c_int * (C & 32767 as ::core::ffi::c_int)
            / 100 as ::core::ffi::c_int;
        E = ((B - D) as ::core::ffi::c_double / 30.6001f64) as ::core::ffi::c_int;
        day = B - D - (30.6001f64 * E as ::core::ffi::c_double) as ::core::ffi::c_int;
        mon = if E < 14 as ::core::ffi::c_int {
            E - 1 as ::core::ffi::c_int
        } else {
            E - 13 as ::core::ffi::c_int
        };
        yr = if mon > 2 as ::core::ffi::c_int {
            C - 4716 as ::core::ffi::c_int
        } else {
            C - 4715 as ::core::ffi::c_int
        };
        hr = (mUnixTime as ::core::ffi::c_uint)
            .wrapping_rem(
                (24 as ::core::ffi::c_int * 60 as ::core::ffi::c_int
                    * 60 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            )
            .wrapping_div(
                (60 as ::core::ffi::c_int * 60 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ) as ::core::ffi::c_int;
        min = (mUnixTime as ::core::ffi::c_uint)
            .wrapping_rem(
                (60 as ::core::ffi::c_int * 60 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            )
            .wrapping_div(60 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        sec = (mUnixTime as ::core::ffi::c_uint).wrapping_rem(60 as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        if yr >= 1980 as ::core::ffi::c_int {
            (*pCds).mDate = (day + (mon << 5 as ::core::ffi::c_int)
                + ((yr - 1980 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int))
                as u16_0;
            (*pCds).mTime = (sec / 2 as ::core::ffi::c_int
                + (min << 5 as ::core::ffi::c_int) + (hr << 11 as ::core::ffi::c_int))
                as u16_0;
        } else {
            (*pCds).mTime = 0 as u16_0;
            (*pCds).mDate = (*pCds).mTime;
        };
    }
}
unsafe extern "C" fn zipfileCorrupt(
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        *pzErr = sqlite3_mprintf(
            b"zip archive is corrupt\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return SQLITE_CORRUPT;
    }
}
unsafe extern "C" fn zipfileGetEntry(
    mut pTab: *mut ZipfileTab,
    mut aBlob: *const u8_0,
    mut nBlob: ::core::ffi::c_int,
    mut pFile: *mut FILE,
    mut iOff: i64_0,
    mut ppEntry: *mut *mut ZipfileEntry,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aRead: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pzErr: *mut *mut ::core::ffi::c_char = &raw mut (*pTab).base.zErrMsg;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if aBlob.is_null() {
            aRead = (*pTab).aBuffer;
            rc = zipfileReadData(pFile, aRead, ZIPFILE_CDS_FIXED_SZ, iOff, pzErr);
        } else {
            if iOff as ::core::ffi::c_longlong
                + ZIPFILE_CDS_FIXED_SZ as ::core::ffi::c_longlong
                > nBlob as ::core::ffi::c_longlong
            {
                return zipfileCorrupt(pzErr);
            }
            aRead = aBlob.offset(iOff as isize) as *const u8_0 as *mut u8_0;
        }
        if rc == SQLITE_OK {
            let mut nAlloc: sqlite3_int64 = 0;
            let mut pNew: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
            let mut nFile: ::core::ffi::c_int = zipfileGetU16(
                aRead.offset(ZIPFILE_CDS_NFILE_OFF as isize) as *mut u8_0,
            ) as ::core::ffi::c_int;
            let mut nExtra: ::core::ffi::c_int = zipfileGetU16(
                aRead.offset((ZIPFILE_CDS_NFILE_OFF + 2 as ::core::ffi::c_int) as isize)
                    as *mut u8_0,
            ) as ::core::ffi::c_int;
            nExtra
                += zipfileGetU16(
                    aRead
                        .offset(
                            (ZIPFILE_CDS_NFILE_OFF + 4 as ::core::ffi::c_int) as isize,
                        ) as *mut u8_0,
                ) as ::core::ffi::c_int;
            nAlloc = (::core::mem::size_of::<ZipfileEntry>() as usize)
                .wrapping_add(nExtra as usize) as sqlite3_int64;
            if !aBlob.is_null() {
                nAlloc
                    += zipfileGetU32(
                        aRead.offset(ZIPFILE_CDS_SZCOMPRESSED_OFF as isize) as *mut u8_0,
                    ) as ::core::ffi::c_longlong;
            }
            pNew = sqlite3_malloc64(nAlloc as sqlite3_uint64) as *mut ZipfileEntry;
            if pNew.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                memset(
                    pNew as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<ZipfileEntry>() as size_t,
                );
                rc = zipfileReadCDS(aRead, &raw mut (*pNew).cds);
                if rc != SQLITE_OK {
                    *pzErr = sqlite3_mprintf(
                        b"failed to read CDS at offset %lld\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        iOff,
                    );
                } else if aBlob.is_null() {
                    rc = zipfileReadData(
                        pFile,
                        aRead,
                        nExtra + nFile,
                        iOff + ZIPFILE_CDS_FIXED_SZ as i64_0,
                        pzErr,
                    );
                } else {
                    aRead = aBlob
                        .offset(
                            (iOff as ::core::ffi::c_longlong
                                + ZIPFILE_CDS_FIXED_SZ as ::core::ffi::c_longlong) as isize,
                        ) as *const u8_0 as *mut u8_0;
                    if iOff as ::core::ffi::c_longlong
                        + ZIPFILE_CDS_FIXED_SZ as ::core::ffi::c_longlong
                        + nFile as ::core::ffi::c_longlong
                        + nExtra as ::core::ffi::c_longlong
                        > nBlob as ::core::ffi::c_longlong
                    {
                        rc = zipfileCorrupt(pzErr);
                    }
                }
            }
            if rc == SQLITE_OK {
                let mut pt: *mut u32_0 = &raw mut (*pNew).mUnixTime;
                (*pNew).cds.zFile = sqlite3_mprintf(
                    b"%.*s\0".as_ptr() as *const ::core::ffi::c_char,
                    nFile,
                    aRead,
                );
                (*pNew).aExtra = pNew.offset(1 as ::core::ffi::c_int as isize)
                    as *mut ZipfileEntry as *mut u8_0;
                memcpy(
                    (*pNew).aExtra as *mut ::core::ffi::c_void,
                    aRead.offset(nFile as isize) as *mut u8_0
                        as *const ::core::ffi::c_void,
                    nExtra as size_t,
                );
                if (*pNew).cds.zFile.is_null() {
                    rc = SQLITE_NOMEM;
                } else if 0 as ::core::ffi::c_int
                    == zipfileScanExtra(
                        aRead.offset(nFile as isize) as *mut u8_0,
                        (*pNew).cds.nExtra as ::core::ffi::c_int,
                        pt,
                    )
                {
                    (*pNew).mUnixTime = zipfileMtime(&raw mut (*pNew).cds);
                }
            }
            if rc == SQLITE_OK {
                static mut szFix: ::core::ffi::c_int = ZIPFILE_LFH_FIXED_SZ;
                let mut lfh: ZipfileLFH = ZipfileLFH {
                    iVersionExtract: 0,
                    flags: 0,
                    iCompression: 0,
                    mTime: 0,
                    mDate: 0,
                    crc32: 0,
                    szCompressed: 0,
                    szUncompressed: 0,
                    nFile: 0,
                    nExtra: 0,
                };
                if !pFile.is_null() {
                    rc = zipfileReadData(
                        pFile,
                        aRead,
                        szFix,
                        (*pNew).cds.iOffset as i64_0,
                        pzErr,
                    );
                } else {
                    aRead = aBlob.offset((*pNew).cds.iOffset as isize) as *const u8_0
                        as *mut u8_0;
                    if ((*pNew).cds.iOffset as ::core::ffi::c_uint)
                        .wrapping_add(ZIPFILE_LFH_FIXED_SZ as ::core::ffi::c_uint)
                        > nBlob as ::core::ffi::c_uint
                    {
                        rc = zipfileCorrupt(pzErr);
                    }
                }
                if rc == SQLITE_OK {
                    rc = zipfileReadLFH(aRead, &raw mut lfh);
                }
                if rc == SQLITE_OK {
                    (*pNew).iDataOff = ((*pNew).cds.iOffset as ::core::ffi::c_uint)
                        .wrapping_add(ZIPFILE_LFH_FIXED_SZ as ::core::ffi::c_uint)
                        as i64_0;
                    (*pNew).iDataOff
                        += (lfh.nFile as ::core::ffi::c_int
                            + lfh.nExtra as ::core::ffi::c_int)
                            as ::core::ffi::c_longlong;
                    if !aBlob.is_null() && (*pNew).cds.szCompressed != 0 {
                        if (*pNew).iDataOff as ::core::ffi::c_longlong
                            + (*pNew).cds.szCompressed as ::core::ffi::c_longlong
                            > nBlob as ::core::ffi::c_longlong
                        {
                            rc = zipfileCorrupt(pzErr);
                        } else {
                            (*pNew).aData = (*pNew).aExtra.offset(nExtra as isize)
                                as *mut u8_0;
                            memcpy(
                                (*pNew).aData as *mut ::core::ffi::c_void,
                                aBlob.offset((*pNew).iDataOff as isize) as *const u8_0
                                    as *const ::core::ffi::c_void,
                                (*pNew).cds.szCompressed as size_t,
                            );
                        }
                    }
                } else {
                    *pzErr = sqlite3_mprintf(
                        b"failed to read LFH at offset %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*pNew).cds.iOffset as ::core::ffi::c_int,
                    );
                }
            }
            if rc != SQLITE_OK {
                zipfileEntryFree(pNew);
            } else {
                *ppEntry = pNew;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileNext(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut ZipfileCsr = cur as *mut ZipfileCsr;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !(*pCsr).pFile.is_null() {
            let mut iEof: i64_0 = (*pCsr).eocd.iOffset.wrapping_add((*pCsr).eocd.nSize)
                as i64_0;
            zipfileEntryFree((*pCsr).pCurrent);
            (*pCsr).pCurrent = ::core::ptr::null_mut::<ZipfileEntry>();
            if (*pCsr).iNextOff >= iEof {
                (*pCsr).bEof = 1 as u8_0;
            } else {
                let mut p: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
                let mut pTab: *mut ZipfileTab = (*cur).pVtab as *mut ZipfileTab;
                rc = zipfileGetEntry(
                    pTab,
                    ::core::ptr::null::<u8_0>(),
                    0 as ::core::ffi::c_int,
                    (*pCsr).pFile,
                    (*pCsr).iNextOff,
                    &raw mut p,
                );
                if rc == SQLITE_OK {
                    (*pCsr).iNextOff += ZIPFILE_CDS_FIXED_SZ as ::core::ffi::c_longlong;
                    (*pCsr).iNextOff
                        += ((*p).cds.nExtra as ::core::ffi::c_int
                            + (*p).cds.nFile as ::core::ffi::c_int
                            + (*p).cds.nComment as ::core::ffi::c_int)
                            as ::core::ffi::c_longlong;
                }
                (*pCsr).pCurrent = p;
            }
        } else {
            if (*pCsr).bNoop == 0 {
                (*pCsr).pCurrent = (*(*pCsr).pCurrent).pNext;
            }
            if (*pCsr).pCurrent.is_null() {
                (*pCsr).bEof = 1 as u8_0;
            }
        }
        (*pCsr).bNoop = 0 as u8_0;
        return rc;
    }
}
unsafe extern "C" fn zipfileFree(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        sqlite3_free(p);
    }
}
unsafe extern "C" fn zipfileInflate(
    mut pCtx: *mut sqlite3_context,
    mut aIn: *const u8_0,
    mut nIn: ::core::ffi::c_int,
    mut nOut: ::core::ffi::c_int,
) {
    unsafe {
        let mut aRes: *mut u8_0 = sqlite3_malloc(nOut) as *mut u8_0;
        if aRes.is_null() {
            sqlite3_result_error_nomem(pCtx);
        } else {
            let mut err: ::core::ffi::c_int = 0;
            let mut str: z_stream = z_stream {
                next_in: ::core::ptr::null_mut::<Bytef>(),
                avail_in: 0,
                total_in: 0,
                next_out: ::core::ptr::null_mut::<Bytef>(),
                avail_out: 0,
                total_out: 0,
                msg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                state: ::core::ptr::null_mut::<internal_state>(),
                zalloc: None,
                zfree: None,
                opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                data_type: 0,
                adler: 0,
                reserved: 0,
            };
            memset(
                &raw mut str as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<z_stream>() as size_t,
            );
            str.next_in = aIn as *mut Byte as *mut Bytef;
            str.avail_in = nIn as uInt;
            str.next_out = aRes as *mut Byte as *mut Bytef;
            str.avail_out = nOut as uInt;
            err = inflateInit2_(
                &raw mut str,
                -15 as ::core::ffi::c_int,
                ZLIB_VERSION.as_ptr(),
                ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
            );
            if err != Z_OK {
                zipfileCtxErrorMsg(
                    pCtx,
                    b"inflateInit2() failed (%d)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    err,
                );
            } else {
                err = inflate(&raw mut str, Z_NO_FLUSH);
                if err != Z_STREAM_END {
                    zipfileCtxErrorMsg(
                        pCtx,
                        b"inflate() failed (%d)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        err,
                    );
                } else {
                    sqlite3_result_blob(
                        pCtx,
                        aRes as *const ::core::ffi::c_void,
                        nOut,
                        Some(
                            zipfileFree
                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                        ),
                    );
                    aRes = ::core::ptr::null_mut::<u8_0>();
                }
            }
            sqlite3_free(aRes as *mut ::core::ffi::c_void);
            inflateEnd(&raw mut str);
        };
    }
}
unsafe extern "C" fn zipfileDeflate(
    mut aIn: *const u8_0,
    mut nIn: ::core::ffi::c_int,
    mut ppOut: *mut *mut u8_0,
    mut pnOut: *mut ::core::ffi::c_int,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nAlloc: sqlite3_int64 = 0;
        let mut str: z_stream = z_stream {
            next_in: ::core::ptr::null_mut::<Bytef>(),
            avail_in: 0,
            total_in: 0,
            next_out: ::core::ptr::null_mut::<Bytef>(),
            avail_out: 0,
            total_out: 0,
            msg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            state: ::core::ptr::null_mut::<internal_state>(),
            zalloc: None,
            zfree: None,
            opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        };
        let mut aOut: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        memset(
            &raw mut str as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<z_stream>() as size_t,
        );
        str.next_in = aIn as *mut Bytef;
        str.avail_in = nIn as uInt;
        deflateInit2_(
            &raw mut str,
            9 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            -15 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
        );
        nAlloc = deflateBound(&raw mut str, nIn as uLong) as sqlite3_int64;
        aOut = sqlite3_malloc64(nAlloc as sqlite3_uint64) as *mut u8_0;
        if aOut.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            let mut res: ::core::ffi::c_int = 0;
            str.next_out = aOut as *mut Bytef;
            str.avail_out = nAlloc as uInt;
            res = deflate(&raw mut str, Z_FINISH);
            if res == Z_STREAM_END {
                *ppOut = aOut;
                *pnOut = str.total_out as ::core::ffi::c_int;
            } else {
                sqlite3_free(aOut as *mut ::core::ffi::c_void);
                *pzErr = sqlite3_mprintf(
                    b"zipfile: deflate() error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                rc = SQLITE_ERROR;
            }
            deflateEnd(&raw mut str);
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut ZipfileCsr = cur as *mut ZipfileCsr;
        let mut pCDS: *mut ZipfileCDS = &raw mut (*(*pCsr).pCurrent).cds;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut c2rust_current_block_37: u64;
        match i {
            0 => {
                sqlite3_result_text(
                    ctx,
                    (*pCDS).zFile,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                c2rust_current_block_37 = 18435049525520518667;
            }
            1 => {
                sqlite3_result_int(
                    ctx,
                    ((*pCDS).iExternalAttr >> 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_int,
                );
                c2rust_current_block_37 = 18435049525520518667;
            }
            2 => {
                sqlite3_result_int64(
                    ctx,
                    (*(*pCsr).pCurrent).mUnixTime as sqlite3_int64,
                );
                c2rust_current_block_37 = 18435049525520518667;
            }
            3 => {
                if sqlite3_vtab_nochange(ctx) == 0 as ::core::ffi::c_int {
                    sqlite3_result_int64(ctx, (*pCDS).szUncompressed as sqlite3_int64);
                }
                c2rust_current_block_37 = 18435049525520518667;
            }
            4 => {
                if sqlite3_vtab_nochange(ctx) != 0 {
                    c2rust_current_block_37 = 18435049525520518667;
                } else {
                    c2rust_current_block_37 = 13536709405535804910;
                }
            }
            5 => {
                c2rust_current_block_37 = 13536709405535804910;
            }
            6 => {
                sqlite3_result_int(ctx, (*pCDS).iCompression as ::core::ffi::c_int);
                c2rust_current_block_37 = 18435049525520518667;
            }
            _ => {
                sqlite3_result_int64(ctx, (*pCsr).iId as sqlite3_int64);
                c2rust_current_block_37 = 18435049525520518667;
            }
        }
        match c2rust_current_block_37 {
            13536709405535804910 => {
                if i == 4 as ::core::ffi::c_int
                    || (*pCDS).iCompression as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    || (*pCDS).iCompression as ::core::ffi::c_int
                        == 8 as ::core::ffi::c_int
                {
                    let mut sz: ::core::ffi::c_int = (*pCDS).szCompressed
                        as ::core::ffi::c_int;
                    let mut szFinal: ::core::ffi::c_int = (*pCDS).szUncompressed
                        as ::core::ffi::c_int;
                    if szFinal > 0 as ::core::ffi::c_int {
                        let mut aBuf: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                        let mut aFree: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                        if !(*(*pCsr).pCurrent).aData.is_null() {
                            aBuf = (*(*pCsr).pCurrent).aData;
                        } else {
                            aFree = sqlite3_malloc64(sz as sqlite3_uint64) as *mut u8_0;
                            aBuf = aFree;
                            if aBuf.is_null() {
                                rc = SQLITE_NOMEM;
                            } else {
                                let mut pFile: *mut FILE = (*pCsr).pFile;
                                if pFile.is_null() {
                                    pFile = (*((*pCsr).base.pVtab as *mut ZipfileTab)).pWriteFd;
                                }
                                rc = zipfileReadData(
                                    pFile,
                                    aBuf,
                                    sz,
                                    (*(*pCsr).pCurrent).iDataOff,
                                    &raw mut (*(*pCsr).base.pVtab).zErrMsg,
                                );
                            }
                        }
                        if rc == SQLITE_OK {
                            if i == 5 as ::core::ffi::c_int
                                && (*pCDS).iCompression as ::core::ffi::c_int != 0
                            {
                                zipfileInflate(ctx, aBuf, sz, szFinal);
                            } else {
                                sqlite3_result_blob(
                                    ctx,
                                    aBuf as *const ::core::ffi::c_void,
                                    sz,
                                    ::core::mem::transmute::<
                                        ::libc::intptr_t,
                                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                                );
                            }
                        }
                        sqlite3_free(aFree as *mut ::core::ffi::c_void);
                    } else {
                        let mut mode: u32_0 = (*pCDS).iExternalAttr
                            >> 16 as ::core::ffi::c_int;
                        if mode as ::core::ffi::c_uint & S_IFDIR as ::core::ffi::c_uint
                            == 0
                            && (*pCDS).nFile as ::core::ffi::c_int
                                >= 1 as ::core::ffi::c_int
                            && *(*pCDS)
                                .zFile
                                .offset(
                                    ((*pCDS).nFile as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int != '/' as i32
                        {
                            sqlite3_result_blob(
                                ctx,
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                SQLITE_STATIC,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileEof(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut ZipfileCsr = cur as *mut ZipfileCsr;
        return (*pCsr).bEof as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn zipfileReadEOCD(
    mut pTab: *mut ZipfileTab,
    mut aBlob: *const u8_0,
    mut nBlob: ::core::ffi::c_int,
    mut pFile: *mut FILE,
    mut pEOCD: *mut ZipfileEOCD,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aRead: *mut u8_0 = (*pTab).aBuffer;
        let mut nRead: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        memset(
            pEOCD as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ZipfileEOCD>() as size_t,
        );
        if aBlob.is_null() {
            let mut iOff: i64_0 = 0;
            let mut szFile: i64_0 = 0;
            fseek(pFile, 0 as ::core::ffi::c_long, SEEK_END);
            szFile = ftell(pFile) as i64_0;
            if szFile == 0 as ::core::ffi::c_longlong {
                return SQLITE_OK;
            }
            nRead = (if szFile
                < (200 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int)
                    as ::core::ffi::c_longlong
            {
                szFile as ::core::ffi::c_longlong
            } else {
                (200 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int)
                    as ::core::ffi::c_longlong
            }) as ::core::ffi::c_int;
            iOff = (szFile as ::core::ffi::c_longlong - nRead as ::core::ffi::c_longlong)
                as i64_0;
            rc = zipfileReadData(
                pFile,
                aRead,
                nRead,
                iOff,
                &raw mut (*pTab).base.zErrMsg,
            );
        } else {
            nRead = if nBlob < 200 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int {
                nBlob
            } else {
                200 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int
            };
            aRead = aBlob.offset((nBlob - nRead) as isize) as *const u8_0 as *mut u8_0;
        }
        if rc == SQLITE_OK {
            let mut i: ::core::ffi::c_int = 0;
            i = nRead - 20 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                if *aRead.offset(i as isize) as ::core::ffi::c_int
                    == 0x50 as ::core::ffi::c_int
                    && *aRead.offset((i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 0x4b as ::core::ffi::c_int
                    && *aRead.offset((i + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 0x5 as ::core::ffi::c_int
                    && *aRead.offset((i + 3 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == 0x6 as ::core::ffi::c_int
                {
                    break;
                }
                i -= 1;
            }
            if i < 0 as ::core::ffi::c_int {
                (*pTab).base.zErrMsg = sqlite3_mprintf(
                    b"cannot find end of central directory record\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return SQLITE_ERROR;
            }
            aRead = aRead.offset((i + 4 as ::core::ffi::c_int) as isize);
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pEOCD).iDisk = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pEOCD).iFirstDisk = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pEOCD).nEntry = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(2 as ::core::ffi::c_int as isize);
            (*pEOCD).nEntryTotal = zipfileGetU16(
                aRead.offset(-(2 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
            (*pEOCD).nSize = zipfileGetU32(
                aRead.offset(-(4 as ::core::ffi::c_int as isize)),
            );
            aRead = aRead.offset(4 as ::core::ffi::c_int as isize);
            (*pEOCD).iOffset = zipfileGetU32(
                aRead.offset(-(4 as ::core::ffi::c_int as isize)),
            );
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileAddEntry(
    mut pTab: *mut ZipfileTab,
    mut pBefore: *mut ZipfileEntry,
    mut pNew: *mut ZipfileEntry,
) {
    unsafe {
        if pBefore.is_null() {
            if (*pTab).pFirstEntry.is_null() {
                (*pTab).pLastEntry = pNew;
                (*pTab).pFirstEntry = (*pTab).pLastEntry;
            } else {
                (*(*pTab).pLastEntry).pNext = pNew;
                (*pTab).pLastEntry = pNew;
            }
        } else {
            let mut pp: *mut *mut ZipfileEntry = ::core::ptr::null_mut::<
                *mut ZipfileEntry,
            >();
            pp = &raw mut (*pTab).pFirstEntry;
            while *pp != pBefore {
                pp = &raw mut (**pp).pNext;
            }
            (*pNew).pNext = pBefore;
            *pp = pNew;
        };
    }
}
unsafe extern "C" fn zipfileLoadDirectory(
    mut pTab: *mut ZipfileTab,
    mut aBlob: *const u8_0,
    mut nBlob: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eocd: ZipfileEOCD = ZipfileEOCD {
            iDisk: 0,
            iFirstDisk: 0,
            nEntry: 0,
            nEntryTotal: 0,
            nSize: 0,
            iOffset: 0,
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut iOff: i64_0 = 0;
        rc = zipfileReadEOCD(pTab, aBlob, nBlob, (*pTab).pWriteFd, &raw mut eocd);
        iOff = eocd.iOffset as i64_0;
        i = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && i < eocd.nEntry as ::core::ffi::c_int {
            let mut pNew: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
            rc = zipfileGetEntry(
                pTab,
                aBlob,
                nBlob,
                (*pTab).pWriteFd,
                iOff,
                &raw mut pNew,
            );
            if rc == SQLITE_OK {
                zipfileAddEntry(pTab, ::core::ptr::null_mut::<ZipfileEntry>(), pNew);
                iOff += ZIPFILE_CDS_FIXED_SZ as ::core::ffi::c_longlong;
                iOff
                    += ((*pNew).cds.nExtra as ::core::ffi::c_int
                        + (*pNew).cds.nFile as ::core::ffi::c_int
                        + (*pNew).cds.nComment as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong;
            }
            i += 1;
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileFilter(
    mut cur: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut ZipfileTab = (*cur).pVtab as *mut ZipfileTab;
        let mut pCsr: *mut ZipfileCsr = cur as *mut ZipfileCsr;
        let mut zFile: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut bInMemory: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        zipfileResetCursor(pCsr);
        if !(*pTab).zFile.is_null() {
            zFile = (*pTab).zFile;
        } else if idxNum == 0 as ::core::ffi::c_int {
            zipfileCursorErr(
                pCsr,
                b"zipfile() function requires an argument\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return SQLITE_ERROR;
        } else if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_BLOB
        {
            static mut aEmptyBlob: u8_0 = 0 as u8_0;
            let mut aBlob: *const u8_0 = sqlite3_value_blob(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            ) as *const u8_0;
            let mut nBlob: ::core::ffi::c_int = sqlite3_value_bytes(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            if aBlob.is_null() {
                aBlob = &raw const aEmptyBlob;
                nBlob = 0 as ::core::ffi::c_int;
            }
            rc = zipfileLoadDirectory(pTab, aBlob, nBlob);
            (*pCsr).pFreeEntry = (*pTab).pFirstEntry;
            (*pTab).pLastEntry = ::core::ptr::null_mut::<ZipfileEntry>();
            (*pTab).pFirstEntry = (*pTab).pLastEntry;
            if rc != SQLITE_OK {
                return rc;
            }
            bInMemory = 1 as ::core::ffi::c_int;
        } else {
            zFile = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
        }
        if (*pTab).pWriteFd.is_null() && 0 as ::core::ffi::c_int == bInMemory {
            (*pCsr).pFile = if !zFile.is_null() {
                fopen(zFile, b"rb\0".as_ptr() as *const ::core::ffi::c_char)
            } else {
                ::core::ptr::null_mut::<FILE>()
            };
            if (*pCsr).pFile.is_null() {
                zipfileCursorErr(
                    pCsr,
                    b"cannot open file: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    zFile,
                );
                rc = SQLITE_ERROR;
            } else {
                rc = zipfileReadEOCD(
                    pTab,
                    ::core::ptr::null::<u8_0>(),
                    0 as ::core::ffi::c_int,
                    (*pCsr).pFile,
                    &raw mut (*pCsr).eocd,
                );
                if rc == SQLITE_OK {
                    if (*pCsr).eocd.nEntry as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        (*pCsr).bEof = 1 as u8_0;
                    } else {
                        (*pCsr).iNextOff = (*pCsr).eocd.iOffset as i64_0;
                        rc = zipfileNext(cur);
                    }
                }
            }
        } else {
            (*pCsr).bNoop = 1 as u8_0;
            (*pCsr).pCurrent = if !(*pCsr).pFreeEntry.is_null() {
                (*pCsr).pFreeEntry
            } else {
                (*pTab).pFirstEntry
            };
            rc = zipfileNext(cur);
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut unusable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdxInfo).nConstraint {
            let mut pCons: *const sqlite3_index_constraint = (*pIdxInfo)
                .aConstraint
                .offset(i as isize) as *mut sqlite3_index_constraint;
            if !((*pCons).iColumn != ZIPFILE_F_COLUMN_IDX) {
                if (*pCons).usable as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    unusable = 1 as ::core::ffi::c_int;
                } else if (*pCons).op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_EQ
                {
                    idx = i;
                }
            }
            i += 1;
        }
        (*pIdxInfo).estimatedCost = 1000.0f64;
        if idx >= 0 as ::core::ffi::c_int {
            (*(*pIdxInfo).aConstraintUsage.offset(idx as isize)).argvIndex = 1
                as ::core::ffi::c_int;
            (*(*pIdxInfo).aConstraintUsage.offset(idx as isize)).omit = 1
                as ::core::ffi::c_uchar;
            (*pIdxInfo).idxNum = 1 as ::core::ffi::c_int;
        } else if unusable != 0 {
            return SQLITE_CONSTRAINT
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn zipfileNewEntry(
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ZipfileEntry {
    unsafe {
        let mut pNew: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
        pNew = sqlite3_malloc(
            ::core::mem::size_of::<ZipfileEntry>() as ::core::ffi::c_int,
        ) as *mut ZipfileEntry;
        if !pNew.is_null() {
            memset(
                pNew as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ZipfileEntry>() as size_t,
            );
            (*pNew).cds.zFile = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                zPath,
            );
            if (*pNew).cds.zFile.is_null() {
                sqlite3_free(pNew as *mut ::core::ffi::c_void);
                pNew = ::core::ptr::null_mut::<ZipfileEntry>();
            }
        }
        return pNew;
    }
}
unsafe extern "C" fn zipfileSerializeLFH(
    mut pEntry: *mut ZipfileEntry,
    mut aBuf: *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCds: *mut ZipfileCDS = &raw mut (*pEntry).cds;
        let mut a: *mut u8_0 = aBuf;
        (*pCds).nExtra = 9 as u16_0;
        zipfilePutU32(a, 0x4034b50 as ::core::ffi::c_int as u32_0);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCds).iVersionExtract);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCds).flags);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCds).iCompression);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCds).mTime);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCds).mDate);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU32(a, (*pCds).crc32);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU32(a, (*pCds).szCompressed);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU32(a, (*pCds).szUncompressed);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCds).nFile);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCds).nExtra);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        memcpy(
            a as *mut ::core::ffi::c_void,
            (*pCds).zFile as *const ::core::ffi::c_void,
            (*pCds).nFile as ::core::ffi::c_int as size_t,
        );
        a = a.offset((*pCds).nFile as ::core::ffi::c_int as isize);
        zipfilePutU16(a, 0x5455 as u16_0);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, 5 as u16_0);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        let c2rust_fresh3 = a;
        a = a.offset(1);
        *c2rust_fresh3 = 0x1 as u8_0;
        zipfilePutU32(a, (*pEntry).mUnixTime);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        return a.offset_from(aBuf) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn zipfileAppendEntry(
    mut pTab: *mut ZipfileTab,
    mut pEntry: *mut ZipfileEntry,
    mut pData: *const u8_0,
    mut nData: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aBuf: *mut u8_0 = (*pTab).aBuffer;
        let mut nBuf: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        nBuf = zipfileSerializeLFH(pEntry, aBuf);
        rc = zipfileAppendData(pTab, aBuf, nBuf);
        if rc == SQLITE_OK {
            (*pEntry).iDataOff = (*pTab).szCurrent;
            rc = zipfileAppendData(pTab, pData, nData);
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileGetMode(
    mut pVal: *mut sqlite3_value,
    mut bIsDir: ::core::ffi::c_int,
    mut pMode: *mut u32_0,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut z: *const ::core::ffi::c_char = sqlite3_value_text(pVal)
            as *const ::core::ffi::c_char;
        let mut mode: u32_0 = 0 as u32_0;
        if z.is_null() {
            mode = (if bIsDir != 0 {
                S_IFDIR + 0o755 as ::core::ffi::c_int
            } else {
                S_IFREG + 0o644 as ::core::ffi::c_int
            }) as u32_0;
        } else if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >= '0' as i32
            && *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                <= '9' as i32
        {
            mode = sqlite3_value_int(pVal) as ::core::ffi::c_uint as u32_0;
        } else {
            let zTemplate: [::core::ffi::c_char; 11] = ::core::mem::transmute::<
                [u8; 11],
                [::core::ffi::c_char; 11],
            >(*b"-rwxrwxrwx\0");
            let mut i: ::core::ffi::c_int = 0;
            if strlen(z) != 10 as size_t {
                c2rust_current_block = 1990426801238179875;
            } else {
                match *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
                    45 => {
                        c2rust_current_block = 18014855578945748386;
                        match c2rust_current_block {
                            1239982780470697516 => {
                                mode |= S_IFLNK as ::core::ffi::c_uint;
                            }
                            8889991735039304700 => {
                                mode |= S_IFDIR as ::core::ffi::c_uint;
                            }
                            _ => {
                                mode |= S_IFREG as ::core::ffi::c_uint;
                            }
                        }
                        i = 1 as ::core::ffi::c_int;
                        loop {
                            if !(i < 10 as ::core::ffi::c_int) {
                                c2rust_current_block = 6057473163062296781;
                                break;
                            }
                            if *z.offset(i as isize) as ::core::ffi::c_int
                                == zTemplate[i as usize] as ::core::ffi::c_int
                            {
                                mode
                                    |= ((1 as ::core::ffi::c_int)
                                        << 9 as ::core::ffi::c_int - i) as ::core::ffi::c_uint;
                            } else if *z.offset(i as isize) as ::core::ffi::c_int
                                != '-' as i32
                            {
                                c2rust_current_block = 1990426801238179875;
                                break;
                            }
                            i += 1;
                        }
                    }
                    100 => {
                        c2rust_current_block = 8889991735039304700;
                        match c2rust_current_block {
                            1239982780470697516 => {
                                mode |= S_IFLNK as ::core::ffi::c_uint;
                            }
                            8889991735039304700 => {
                                mode |= S_IFDIR as ::core::ffi::c_uint;
                            }
                            _ => {
                                mode |= S_IFREG as ::core::ffi::c_uint;
                            }
                        }
                        i = 1 as ::core::ffi::c_int;
                        loop {
                            if !(i < 10 as ::core::ffi::c_int) {
                                c2rust_current_block = 6057473163062296781;
                                break;
                            }
                            if *z.offset(i as isize) as ::core::ffi::c_int
                                == zTemplate[i as usize] as ::core::ffi::c_int
                            {
                                mode
                                    |= ((1 as ::core::ffi::c_int)
                                        << 9 as ::core::ffi::c_int - i) as ::core::ffi::c_uint;
                            } else if *z.offset(i as isize) as ::core::ffi::c_int
                                != '-' as i32
                            {
                                c2rust_current_block = 1990426801238179875;
                                break;
                            }
                            i += 1;
                        }
                    }
                    108 => {
                        c2rust_current_block = 1239982780470697516;
                        match c2rust_current_block {
                            1239982780470697516 => {
                                mode |= S_IFLNK as ::core::ffi::c_uint;
                            }
                            8889991735039304700 => {
                                mode |= S_IFDIR as ::core::ffi::c_uint;
                            }
                            _ => {
                                mode |= S_IFREG as ::core::ffi::c_uint;
                            }
                        }
                        i = 1 as ::core::ffi::c_int;
                        loop {
                            if !(i < 10 as ::core::ffi::c_int) {
                                c2rust_current_block = 6057473163062296781;
                                break;
                            }
                            if *z.offset(i as isize) as ::core::ffi::c_int
                                == zTemplate[i as usize] as ::core::ffi::c_int
                            {
                                mode
                                    |= ((1 as ::core::ffi::c_int)
                                        << 9 as ::core::ffi::c_int - i) as ::core::ffi::c_uint;
                            } else if *z.offset(i as isize) as ::core::ffi::c_int
                                != '-' as i32
                            {
                                c2rust_current_block = 1990426801238179875;
                                break;
                            }
                            i += 1;
                        }
                    }
                    _ => {
                        c2rust_current_block = 1990426801238179875;
                    }
                }
            }
            match c2rust_current_block {
                6057473163062296781 => {}
                _ => {
                    *pzErr = sqlite3_mprintf(
                        b"zipfile: parse error in mode: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        z,
                    );
                    return SQLITE_ERROR;
                }
            }
        }
        if (mode as ::core::ffi::c_uint & S_IFDIR as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int == bIsDir
        {
            *pzErr = sqlite3_mprintf(
                b"zipfile: mode does not match data\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return SQLITE_CONSTRAINT;
        }
        *pMode = mode;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn zipfileComparePath(
    mut zA: *const ::core::ffi::c_char,
    mut zB: *const ::core::ffi::c_char,
    mut nB: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nA: ::core::ffi::c_int = strlen(zA) as ::core::ffi::c_int;
        if nA > 0 as ::core::ffi::c_int
            && *zA.offset((nA - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == '/' as i32
        {
            nA -= 1;
        }
        if nB > 0 as ::core::ffi::c_int
            && *zB.offset((nB - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == '/' as i32
        {
            nB -= 1;
        }
        if nA == nB
            && memcmp(
                zA as *const ::core::ffi::c_void,
                zB as *const ::core::ffi::c_void,
                nA as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn zipfileBegin(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut ZipfileTab = pVtab as *mut ZipfileTab;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pTab).zFile.is_null()
            || *(*pTab).zFile.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*pTab).base.zErrMsg = sqlite3_mprintf(
                b"zipfile: missing filename\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return SQLITE_ERROR;
        }
        (*pTab).pWriteFd = fopen(
            (*pTab).zFile,
            b"ab+\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if (*pTab).pWriteFd.is_null() {
            (*pTab).base.zErrMsg = sqlite3_mprintf(
                b"zipfile: failed to open file %s for writing\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pTab).zFile,
            );
            rc = SQLITE_ERROR;
        } else {
            fseek((*pTab).pWriteFd, 0 as ::core::ffi::c_long, SEEK_END);
            (*pTab).szOrig = ftell((*pTab).pWriteFd) as i64_0;
            (*pTab).szCurrent = (*pTab).szOrig;
            rc = zipfileLoadDirectory(
                pTab,
                ::core::ptr::null::<u8_0>(),
                0 as ::core::ffi::c_int,
            );
        }
        if rc != SQLITE_OK {
            zipfileCleanupTransaction(pTab);
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileTime() -> u32_0 {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = sqlite3_vfs_find(
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        let mut ret: u32_0 = 0;
        if pVfs.is_null() {
            return 0 as u32_0;
        }
        if (*pVfs).iVersion >= 2 as ::core::ffi::c_int
            && (*pVfs).xCurrentTimeInt64.is_some()
        {
            let mut ms: i64_0 = 0;
            (*pVfs)
                .xCurrentTimeInt64
                .expect("non-null function pointer")(pVfs, &raw mut ms);
            ret = (ms as ::core::ffi::c_longlong / 1000 as ::core::ffi::c_longlong
                - 24405875 as ::core::ffi::c_int as ::core::ffi::c_longlong
                    * 8640 as ::core::ffi::c_longlong) as u32_0;
        } else {
            let mut day: ::core::ffi::c_double = 0.;
            (*pVfs).xCurrentTime.expect("non-null function pointer")(pVfs, &raw mut day);
            ret = ((day - 2440587.5f64)
                * 86400 as ::core::ffi::c_int as ::core::ffi::c_double) as u32_0;
        }
        return ret;
    }
}
unsafe extern "C" fn zipfileGetTime(mut pVal: *mut sqlite3_value) -> u32_0 {
    unsafe {
        if pVal.is_null() || sqlite3_value_type(pVal) == SQLITE_NULL {
            return zipfileTime();
        }
        return sqlite3_value_int64(pVal) as u32_0;
    }
}
unsafe extern "C" fn zipfileRemoveEntryFromList(
    mut pTab: *mut ZipfileTab,
    mut pOld: *mut ZipfileEntry,
) {
    unsafe {
        if !pOld.is_null() {
            if (*pTab).pFirstEntry == pOld {
                (*pTab).pFirstEntry = (*pOld).pNext;
                if (*pTab).pLastEntry == pOld {
                    (*pTab).pLastEntry = ::core::ptr::null_mut::<ZipfileEntry>();
                }
            } else {
                let mut p: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
                p = (*pTab).pFirstEntry;
                while !p.is_null() {
                    if (*p).pNext == pOld {
                        (*p).pNext = (*pOld).pNext;
                        if (*pTab).pLastEntry == pOld {
                            (*pTab).pLastEntry = p;
                        }
                        break;
                    } else {
                        p = (*p).pNext;
                    }
                }
            }
            zipfileEntryFree(pOld);
        }
    }
}
unsafe extern "C" fn zipfileUpdate(
    mut pVtab: *mut sqlite3_vtab,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pTab: *mut ZipfileTab = pVtab as *mut ZipfileTab;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pNew: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
        let mut mode: u32_0 = 0 as u32_0;
        let mut mTime: u32_0 = 0 as u32_0;
        let mut sz: i64_0 = 0 as i64_0;
        let mut zPath: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nPath: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pData: *const u8_0 = ::core::ptr::null::<u8_0>();
        let mut nData: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iMethod: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pFree: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut zFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pOld: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
        let mut pOld2: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
        let mut bUpdate: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bIsDir: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iCrc32: u32_0 = 0 as u32_0;
        if (*pTab).pWriteFd.is_null() {
            rc = zipfileBegin(pVtab);
            if rc != SQLITE_OK {
                return rc;
            }
        }
        if sqlite3_value_type(*apVal.offset(0 as ::core::ffi::c_int as isize))
            != SQLITE_NULL
        {
            let mut zDelete: *const ::core::ffi::c_char = sqlite3_value_text(
                *apVal.offset(0 as ::core::ffi::c_int as isize),
            ) as *const ::core::ffi::c_char;
            let mut nDelete: ::core::ffi::c_int = strlen(zDelete) as ::core::ffi::c_int;
            if nVal > 1 as ::core::ffi::c_int {
                let mut zUpdate: *const ::core::ffi::c_char = sqlite3_value_text(
                    *apVal.offset(1 as ::core::ffi::c_int as isize),
                ) as *const ::core::ffi::c_char;
                if !zUpdate.is_null()
                    && zipfileComparePath(zUpdate, zDelete, nDelete)
                        != 0 as ::core::ffi::c_int
                {
                    bUpdate = 1 as ::core::ffi::c_int;
                }
            }
            pOld = (*pTab).pFirstEntry;
            while !(zipfileComparePath((*pOld).cds.zFile, zDelete, nDelete)
                == 0 as ::core::ffi::c_int)
            {
                pOld = (*pOld).pNext;
            }
        }
        if nVal > 1 as ::core::ffi::c_int {
            if sqlite3_value_type(*apVal.offset(5 as ::core::ffi::c_int as isize))
                != SQLITE_NULL
            {
                zipfileTableErr(
                    pTab,
                    b"sz must be NULL\0".as_ptr() as *const ::core::ffi::c_char,
                );
                rc = SQLITE_CONSTRAINT;
            }
            if sqlite3_value_type(*apVal.offset(6 as ::core::ffi::c_int as isize))
                != SQLITE_NULL
            {
                zipfileTableErr(
                    pTab,
                    b"rawdata must be NULL\0".as_ptr() as *const ::core::ffi::c_char,
                );
                rc = SQLITE_CONSTRAINT;
            }
            if rc == SQLITE_OK {
                if sqlite3_value_type(*apVal.offset(7 as ::core::ffi::c_int as isize))
                    == SQLITE_NULL
                {
                    bIsDir = 1 as ::core::ffi::c_int;
                } else {
                    let mut aIn: *const u8_0 = sqlite3_value_blob(
                        *apVal.offset(7 as ::core::ffi::c_int as isize),
                    ) as *const u8_0;
                    let mut nIn: ::core::ffi::c_int = sqlite3_value_bytes(
                        *apVal.offset(7 as ::core::ffi::c_int as isize),
                    );
                    let mut bAuto: ::core::ffi::c_int = (sqlite3_value_type(
                        *apVal.offset(8 as ::core::ffi::c_int as isize),
                    ) == SQLITE_NULL) as ::core::ffi::c_int;
                    iMethod = sqlite3_value_int(
                        *apVal.offset(8 as ::core::ffi::c_int as isize),
                    );
                    sz = nIn as i64_0;
                    pData = aIn;
                    nData = nIn;
                    if iMethod != 0 as ::core::ffi::c_int
                        && iMethod != 8 as ::core::ffi::c_int
                    {
                        zipfileTableErr(
                            pTab,
                            b"unknown compression method: %d\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            iMethod,
                        );
                        rc = SQLITE_CONSTRAINT;
                    } else {
                        if bAuto != 0 || iMethod != 0 {
                            let mut nCmp: ::core::ffi::c_int = 0;
                            rc = zipfileDeflate(
                                aIn,
                                nIn,
                                &raw mut pFree,
                                &raw mut nCmp,
                                &raw mut (*pTab).base.zErrMsg,
                            );
                            if rc == SQLITE_OK {
                                if iMethod != 0 || nCmp < nIn {
                                    iMethod = 8 as ::core::ffi::c_int;
                                    pData = pFree;
                                    nData = nCmp;
                                }
                            }
                        }
                        iCrc32 = crc32(0 as uLong, aIn as *const Bytef, nIn as uInt)
                            as u32_0;
                    }
                }
            }
            if rc == SQLITE_OK {
                rc = zipfileGetMode(
                    *apVal.offset(3 as ::core::ffi::c_int as isize),
                    bIsDir,
                    &raw mut mode,
                    &raw mut (*pTab).base.zErrMsg,
                );
            }
            if rc == SQLITE_OK {
                zPath = sqlite3_value_text(
                    *apVal.offset(2 as ::core::ffi::c_int as isize),
                ) as *const ::core::ffi::c_char;
                if zPath.is_null() {
                    zPath = b"\0".as_ptr() as *const ::core::ffi::c_char;
                }
                nPath = strlen(zPath) as ::core::ffi::c_int;
                if nPath > ZIPFILE_MX_NAME {
                    zipfileTableErr(
                        pTab,
                        b"filename too long; max: %d bytes\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ZIPFILE_MX_NAME,
                    );
                    rc = SQLITE_CONSTRAINT;
                }
                mTime = zipfileGetTime(*apVal.offset(4 as ::core::ffi::c_int as isize));
            }
            if rc == SQLITE_OK && bIsDir != 0 {
                if nPath <= 0 as ::core::ffi::c_int
                    || *zPath.offset((nPath - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int != '/' as i32
                {
                    zFree = sqlite3_mprintf(
                        b"%s/\0".as_ptr() as *const ::core::ffi::c_char,
                        zPath,
                    );
                    zPath = zFree as *const ::core::ffi::c_char;
                    if zFree.is_null() {
                        rc = SQLITE_NOMEM;
                        nPath = 0 as ::core::ffi::c_int;
                    } else {
                        nPath = strlen(zPath) as ::core::ffi::c_int;
                    }
                }
            }
            if (pOld.is_null() || bUpdate != 0) && rc == SQLITE_OK {
                let mut p: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
                p = (*pTab).pFirstEntry;
                loop {
                    if p.is_null() {
                        c2rust_current_block = 14648606000749551097;
                        break;
                    }
                    if zipfileComparePath((*p).cds.zFile, zPath, nPath)
                        == 0 as ::core::ffi::c_int
                    {
                        match sqlite3_vtab_on_conflict((*pTab).db) {
                            SQLITE_IGNORE => {
                                c2rust_current_block = 2313289325184626146;
                                break;
                            }
                            SQLITE_REPLACE => {
                                pOld2 = p;
                                c2rust_current_block = 14648606000749551097;
                                break;
                            }
                            _ => {
                                zipfileTableErr(
                                    pTab,
                                    b"duplicate name: \"%s\"\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    zPath,
                                );
                                rc = SQLITE_CONSTRAINT;
                                c2rust_current_block = 14648606000749551097;
                                break;
                            }
                        }
                    } else {
                        p = (*p).pNext;
                    }
                }
            } else {
                c2rust_current_block = 14648606000749551097;
            }
            match c2rust_current_block {
                2313289325184626146 => {}
                _ => {
                    if rc == SQLITE_OK {
                        pNew = zipfileNewEntry(zPath);
                        if pNew.is_null() {
                            rc = SQLITE_NOMEM;
                        } else {
                            (*pNew).cds.iVersionMadeBy = ZIPFILE_NEWENTRY_MADEBY
                                as u16_0;
                            (*pNew).cds.iVersionExtract = ZIPFILE_NEWENTRY_REQUIRED
                                as u16_0;
                            (*pNew).cds.flags = ZIPFILE_NEWENTRY_FLAGS as u16_0;
                            (*pNew).cds.iCompression = iMethod as u16_0;
                            zipfileMtimeToDos(&raw mut (*pNew).cds, mTime);
                            (*pNew).cds.crc32 = iCrc32;
                            (*pNew).cds.szCompressed = nData as u32_0;
                            (*pNew).cds.szUncompressed = sz as u32_0;
                            (*pNew).cds.iExternalAttr = mode << 16 as ::core::ffi::c_int;
                            (*pNew).cds.iOffset = (*pTab).szCurrent as u32_0;
                            (*pNew).cds.nFile = nPath as u16_0;
                            (*pNew).mUnixTime = mTime;
                            rc = zipfileAppendEntry(pTab, pNew, pData, nData);
                            zipfileAddEntry(pTab, pOld, pNew);
                        }
                    }
                    c2rust_current_block = 14865402277128115059;
                }
            }
        } else {
            c2rust_current_block = 14865402277128115059;
        }
        match c2rust_current_block {
            14865402277128115059 => {
                if rc == SQLITE_OK && (!pOld.is_null() || !pOld2.is_null()) {
                    let mut pCsr: *mut ZipfileCsr = ::core::ptr::null_mut::<
                        ZipfileCsr,
                    >();
                    pCsr = (*pTab).pCsrList;
                    while !pCsr.is_null() {
                        if !(*pCsr).pCurrent.is_null()
                            && ((*pCsr).pCurrent == pOld || (*pCsr).pCurrent == pOld2)
                        {
                            (*pCsr).pCurrent = (*(*pCsr).pCurrent).pNext;
                            (*pCsr).bNoop = 1 as u8_0;
                        }
                        pCsr = (*pCsr).pCsrNext;
                    }
                    zipfileRemoveEntryFromList(pTab, pOld);
                    zipfileRemoveEntryFromList(pTab, pOld2);
                }
            }
            _ => {}
        }
        sqlite3_free(pFree as *mut ::core::ffi::c_void);
        sqlite3_free(zFree as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn zipfileSerializeEOCD(
    mut p: *mut ZipfileEOCD,
    mut aBuf: *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: *mut u8_0 = aBuf;
        zipfilePutU32(a, 0x6054b50 as ::core::ffi::c_int as u32_0);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*p).iDisk);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*p).iFirstDisk);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*p).nEntry);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*p).nEntryTotal);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU32(a, (*p).nSize);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU32(a, (*p).iOffset);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, 0 as u16_0);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        return a.offset_from(aBuf) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn zipfileAppendEOCD(
    mut pTab: *mut ZipfileTab,
    mut p: *mut ZipfileEOCD,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nBuf: ::core::ffi::c_int = zipfileSerializeEOCD(p, (*pTab).aBuffer);
        return zipfileAppendData(pTab, (*pTab).aBuffer, nBuf);
    }
}
unsafe extern "C" fn zipfileSerializeCDS(
    mut pEntry: *mut ZipfileEntry,
    mut aBuf: *mut u8_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: *mut u8_0 = aBuf;
        let mut pCDS: *mut ZipfileCDS = &raw mut (*pEntry).cds;
        if (*pEntry).aExtra.is_null() {
            (*pCDS).nExtra = 9 as u16_0;
        }
        zipfilePutU32(a, 0x2014b50 as ::core::ffi::c_int as u32_0);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).iVersionMadeBy);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).iVersionExtract);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).flags);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).iCompression);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).mTime);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).mDate);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU32(a, (*pCDS).crc32);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU32(a, (*pCDS).szCompressed);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU32(a, (*pCDS).szUncompressed);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).nFile);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).nExtra);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).nComment);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).iDiskStart);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU16(a, (*pCDS).iInternalAttr);
        a = a.offset(2 as ::core::ffi::c_int as isize);
        zipfilePutU32(a, (*pCDS).iExternalAttr);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        zipfilePutU32(a, (*pCDS).iOffset);
        a = a.offset(4 as ::core::ffi::c_int as isize);
        memcpy(
            a as *mut ::core::ffi::c_void,
            (*pCDS).zFile as *const ::core::ffi::c_void,
            (*pCDS).nFile as size_t,
        );
        a = a.offset((*pCDS).nFile as ::core::ffi::c_int as isize);
        if !(*pEntry).aExtra.is_null() {
            let mut n: ::core::ffi::c_int = (*pCDS).nExtra as ::core::ffi::c_int
                + (*pCDS).nComment as ::core::ffi::c_int;
            memcpy(
                a as *mut ::core::ffi::c_void,
                (*pEntry).aExtra as *const ::core::ffi::c_void,
                n as size_t,
            );
            a = a.offset(n as isize);
        } else {
            zipfilePutU16(a, 0x5455 as u16_0);
            a = a.offset(2 as ::core::ffi::c_int as isize);
            zipfilePutU16(a, 5 as u16_0);
            a = a.offset(2 as ::core::ffi::c_int as isize);
            let c2rust_fresh4 = a;
            a = a.offset(1);
            *c2rust_fresh4 = 0x1 as u8_0;
            zipfilePutU32(a, (*pEntry).mUnixTime);
            a = a.offset(4 as ::core::ffi::c_int as isize);
        }
        return a.offset_from(aBuf) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn zipfileCommit(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut ZipfileTab = pVtab as *mut ZipfileTab;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !(*pTab).pWriteFd.is_null() {
            let mut iOffset: i64_0 = (*pTab).szCurrent;
            let mut p: *mut ZipfileEntry = ::core::ptr::null_mut::<ZipfileEntry>();
            let mut eocd: ZipfileEOCD = ZipfileEOCD {
                iDisk: 0,
                iFirstDisk: 0,
                nEntry: 0,
                nEntryTotal: 0,
                nSize: 0,
                iOffset: 0,
            };
            let mut nEntry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            p = (*pTab).pFirstEntry;
            while rc == SQLITE_OK && !p.is_null() {
                let mut n: ::core::ffi::c_int = zipfileSerializeCDS(p, (*pTab).aBuffer);
                rc = zipfileAppendData(pTab, (*pTab).aBuffer, n);
                nEntry += 1;
                p = (*p).pNext;
            }
            eocd.iDisk = 0 as u16_0;
            eocd.iFirstDisk = 0 as u16_0;
            eocd.nEntry = nEntry as u16_0;
            eocd.nEntryTotal = nEntry as u16_0;
            eocd.nSize = ((*pTab).szCurrent - iOffset) as u32_0;
            eocd.iOffset = iOffset as u32_0;
            rc = zipfileAppendEOCD(pTab, &raw mut eocd);
            zipfileCleanupTransaction(pTab);
        }
        return rc;
    }
}
unsafe extern "C" fn zipfileRollback(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        return zipfileCommit(pVtab);
    }
}
unsafe extern "C" fn zipfileFindCursor(
    mut pTab: *mut ZipfileTab,
    mut iId: i64_0,
) -> *mut ZipfileCsr {
    unsafe {
        let mut pCsr: *mut ZipfileCsr = ::core::ptr::null_mut::<ZipfileCsr>();
        pCsr = (*pTab).pCsrList;
        while !pCsr.is_null() {
            if iId == (*pCsr).iId {
                break;
            }
            pCsr = (*pCsr).pCsrNext;
        }
        return pCsr;
    }
}
unsafe extern "C" fn zipfileFunctionCds(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pCsr: *mut ZipfileCsr = ::core::ptr::null_mut::<ZipfileCsr>();
        let mut pTab: *mut ZipfileTab = sqlite3_user_data(context) as *mut ZipfileTab;
        pCsr = zipfileFindCursor(
            pTab,
            sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize)) as i64_0,
        );
        if !pCsr.is_null() {
            let mut p: *mut ZipfileCDS = &raw mut (*(*pCsr).pCurrent).cds;
            let mut zRes: *mut ::core::ffi::c_char = sqlite3_mprintf(
                b"{\"version-made-by\" : %u, \"version-to-extract\" : %u, \"flags\" : %u, \"compression\" : %u, \"time\" : %u, \"date\" : %u, \"crc32\" : %u, \"compressed-size\" : %u, \"uncompressed-size\" : %u, \"file-name-length\" : %u, \"extra-field-length\" : %u, \"file-comment-length\" : %u, \"disk-number-start\" : %u, \"internal-attr\" : %u, \"external-attr\" : %u, \"offset\" : %u }\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*p).iVersionMadeBy as u32_0,
                (*p).iVersionExtract as u32_0,
                (*p).flags as u32_0,
                (*p).iCompression as u32_0,
                (*p).mTime as u32_0,
                (*p).mDate as u32_0,
                (*p).crc32,
                (*p).szCompressed,
                (*p).szUncompressed,
                (*p).nFile as u32_0,
                (*p).nExtra as u32_0,
                (*p).nComment as u32_0,
                (*p).iDiskStart as u32_0,
                (*p).iInternalAttr as u32_0,
                (*p).iExternalAttr,
                (*p).iOffset,
            );
            if zRes.is_null() {
                sqlite3_result_error_nomem(context);
            } else {
                sqlite3_result_text(
                    context,
                    zRes,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                sqlite3_free(zRes as *mut ::core::ffi::c_void);
            }
        }
    }
}
unsafe extern "C" fn zipfileFindFunction(
    mut pVtab: *mut sqlite3_vtab,
    mut nArg: ::core::ffi::c_int,
    mut zName: *const ::core::ffi::c_char,
    mut pxFunc: *mut Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut ppArg: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_stricmp(
            b"zipfile_cds\0".as_ptr() as *const ::core::ffi::c_char,
            zName,
        ) == 0 as ::core::ffi::c_int
        {
            *pxFunc = Some(
                zipfileFunctionCds
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
                >;
            *ppArg = pVtab as *mut ::core::ffi::c_void;
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn zipfileBufferGrow(
    mut pBuf: *mut ZipfileBuffer,
    mut nByte: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pBuf).n + nByte > (*pBuf).nAlloc {
            let mut aNew: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
            let mut nNew: sqlite3_int64 = (if (*pBuf).n != 0 {
                (*pBuf).n * 2 as ::core::ffi::c_int
            } else {
                512 as ::core::ffi::c_int
            }) as sqlite3_int64;
            let mut nReq: ::core::ffi::c_int = (*pBuf).n + nByte;
            while nNew < nReq as ::core::ffi::c_longlong {
                nNew = (nNew as ::core::ffi::c_longlong * 2 as ::core::ffi::c_longlong)
                    as sqlite3_int64;
            }
            aNew = sqlite3_realloc64(
                (*pBuf).a as *mut ::core::ffi::c_void,
                nNew as sqlite3_uint64,
            ) as *mut u8_0;
            if aNew.is_null() {
                return SQLITE_NOMEM;
            }
            (*pBuf).a = aNew;
            (*pBuf).nAlloc = nNew as ::core::ffi::c_int;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn zipfileStep(
    mut pCtx: *mut sqlite3_context,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut p: *mut ZipfileCtx = ::core::ptr::null_mut::<ZipfileCtx>();
        let mut e: ZipfileEntry = ZipfileEntry {
            cds: ZipfileCDS {
                iVersionMadeBy: 0,
                iVersionExtract: 0,
                flags: 0,
                iCompression: 0,
                mTime: 0,
                mDate: 0,
                crc32: 0,
                szCompressed: 0,
                szUncompressed: 0,
                nFile: 0,
                nExtra: 0,
                nComment: 0,
                iDiskStart: 0,
                iInternalAttr: 0,
                iExternalAttr: 0,
                iOffset: 0,
                zFile: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            mUnixTime: 0,
            aExtra: ::core::ptr::null_mut::<u8_0>(),
            iDataOff: 0,
            aData: ::core::ptr::null_mut::<u8_0>(),
            pNext: ::core::ptr::null_mut::<ZipfileEntry>(),
        };
        let mut pName: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut pMode: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut pMtime: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut pData: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut pMethod: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut bIsDir: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mode: u32_0 = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut iMethod: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut aData: *const u8_0 = ::core::ptr::null::<u8_0>();
        let mut nData: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut szUncompressed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aFree: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut iCrc32: u32_0 = 0 as u32_0;
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nName: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nByte: ::core::ffi::c_int = 0;
        memset(
            &raw mut e as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ZipfileEntry>() as size_t,
        );
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<ZipfileCtx>() as ::core::ffi::c_int,
        ) as *mut ZipfileCtx;
        if p.is_null() {
            return;
        }
        if nVal != 2 as ::core::ffi::c_int && nVal != 4 as ::core::ffi::c_int
            && nVal != 5 as ::core::ffi::c_int
        {
            zErr = sqlite3_mprintf(
                b"wrong number of arguments to function zipfile()\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            rc = SQLITE_ERROR;
        } else {
            pName = *apVal.offset(0 as ::core::ffi::c_int as isize);
            if nVal == 2 as ::core::ffi::c_int {
                pData = *apVal.offset(1 as ::core::ffi::c_int as isize);
            } else {
                pMode = *apVal.offset(1 as ::core::ffi::c_int as isize);
                pMtime = *apVal.offset(2 as ::core::ffi::c_int as isize);
                pData = *apVal.offset(3 as ::core::ffi::c_int as isize);
                if nVal == 5 as ::core::ffi::c_int {
                    pMethod = *apVal.offset(4 as ::core::ffi::c_int as isize);
                }
            }
            zName = sqlite3_value_text(pName) as *mut ::core::ffi::c_char;
            nName = sqlite3_value_bytes(pName);
            if zName.is_null() {
                zErr = sqlite3_mprintf(
                    b"first argument to zipfile() must be non-NULL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                rc = SQLITE_ERROR;
            } else if nName > ZIPFILE_MX_NAME {
                zErr = sqlite3_mprintf(
                    b"filename argument to zipfile() too big; max: %d bytes\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ZIPFILE_MX_NAME,
                );
                rc = SQLITE_ERROR;
            } else {
                if !pMethod.is_null() && SQLITE_NULL != sqlite3_value_type(pMethod) {
                    iMethod = sqlite3_value_int64(pMethod) as ::core::ffi::c_int;
                    if iMethod != 0 as ::core::ffi::c_int
                        && iMethod != 8 as ::core::ffi::c_int
                    {
                        zErr = sqlite3_mprintf(
                            b"illegal method value: %d\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            iMethod,
                        );
                        rc = SQLITE_ERROR;
                        c2rust_current_block = 9161728879806770639;
                    } else {
                        c2rust_current_block = 18377268871191777778;
                    }
                } else {
                    c2rust_current_block = 18377268871191777778;
                }
                match c2rust_current_block {
                    9161728879806770639 => {}
                    _ => {
                        if sqlite3_value_type(pData) == SQLITE_NULL {
                            bIsDir = 1 as ::core::ffi::c_int;
                            iMethod = 0 as ::core::ffi::c_int;
                            c2rust_current_block = 6174974146017752131;
                        } else {
                            aData = sqlite3_value_blob(pData) as *const u8_0;
                            nData = sqlite3_value_bytes(pData);
                            szUncompressed = nData;
                            iCrc32 = crc32(
                                0 as uLong,
                                aData as *const Bytef,
                                nData as uInt,
                            ) as u32_0;
                            if iMethod < 0 as ::core::ffi::c_int
                                || iMethod == 8 as ::core::ffi::c_int
                            {
                                let mut nOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                rc = zipfileDeflate(
                                    aData,
                                    nData,
                                    &raw mut aFree,
                                    &raw mut nOut,
                                    &raw mut zErr,
                                );
                                if rc != SQLITE_OK {
                                    c2rust_current_block = 9161728879806770639;
                                } else {
                                    if iMethod == 8 as ::core::ffi::c_int || nOut < nData {
                                        aData = aFree;
                                        nData = nOut;
                                        iMethod = 8 as ::core::ffi::c_int;
                                    } else {
                                        iMethod = 0 as ::core::ffi::c_int;
                                    }
                                    c2rust_current_block = 6174974146017752131;
                                }
                            } else {
                                c2rust_current_block = 6174974146017752131;
                            }
                        }
                        match c2rust_current_block {
                            9161728879806770639 => {}
                            _ => {
                                rc = zipfileGetMode(
                                    pMode,
                                    bIsDir,
                                    &raw mut mode,
                                    &raw mut zErr,
                                );
                                if !(rc != 0) {
                                    e.mUnixTime = zipfileGetTime(pMtime);
                                    if bIsDir == 0 as ::core::ffi::c_int {
                                        if nName > 0 as ::core::ffi::c_int
                                            && *zName.offset((nName - 1 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_int == '/' as i32
                                        {
                                            zErr = sqlite3_mprintf(
                                                b"non-directory name must not end with /\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            rc = SQLITE_ERROR;
                                            c2rust_current_block = 9161728879806770639;
                                        } else {
                                            c2rust_current_block = 16203797167131938757;
                                        }
                                    } else if nName == 0 as ::core::ffi::c_int
                                        || *zName.offset((nName - 1 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int != '/' as i32
                                    {
                                        zFree = sqlite3_mprintf(
                                            b"%s/\0".as_ptr() as *const ::core::ffi::c_char,
                                            zName,
                                        );
                                        zName = zFree;
                                        if zName.is_null() {
                                            rc = SQLITE_NOMEM;
                                            c2rust_current_block = 9161728879806770639;
                                        } else {
                                            nName = strlen(zName) as ::core::ffi::c_int;
                                            c2rust_current_block = 16203797167131938757;
                                        }
                                    } else {
                                        while nName > 1 as ::core::ffi::c_int
                                            && *zName.offset((nName - 2 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_int == '/' as i32
                                        {
                                            nName -= 1;
                                        }
                                        c2rust_current_block = 16203797167131938757;
                                    }
                                    match c2rust_current_block {
                                        9161728879806770639 => {}
                                        _ => {
                                            e.cds.iVersionMadeBy = ZIPFILE_NEWENTRY_MADEBY as u16_0;
                                            e.cds.iVersionExtract = ZIPFILE_NEWENTRY_REQUIRED as u16_0;
                                            e.cds.flags = ZIPFILE_NEWENTRY_FLAGS as u16_0;
                                            e.cds.iCompression = iMethod as u16_0;
                                            zipfileMtimeToDos(&raw mut e.cds, e.mUnixTime);
                                            e.cds.crc32 = iCrc32;
                                            e.cds.szCompressed = nData as u32_0;
                                            e.cds.szUncompressed = szUncompressed as u32_0;
                                            e.cds.iExternalAttr = mode << 16 as ::core::ffi::c_int;
                                            e.cds.iOffset = (*p).body.n as u32_0;
                                            e.cds.nFile = nName as u16_0;
                                            e.cds.zFile = zName;
                                            nByte = ZIPFILE_LFH_FIXED_SZ
                                                + e.cds.nFile as ::core::ffi::c_int
                                                + 9 as ::core::ffi::c_int;
                                            rc = zipfileBufferGrow(&raw mut (*p).body, nByte);
                                            if !(rc != 0) {
                                                (*p).body.n
                                                    += zipfileSerializeLFH(
                                                        &raw mut e,
                                                        (*p).body.a.offset((*p).body.n as isize) as *mut u8_0,
                                                    );
                                                if nData > 0 as ::core::ffi::c_int {
                                                    rc = zipfileBufferGrow(&raw mut (*p).body, nData);
                                                    if rc != 0 {
                                                        c2rust_current_block = 9161728879806770639;
                                                    } else {
                                                        memcpy(
                                                            (*p).body.a.offset((*p).body.n as isize) as *mut u8_0
                                                                as *mut ::core::ffi::c_void,
                                                            aData as *const ::core::ffi::c_void,
                                                            nData as size_t,
                                                        );
                                                        (*p).body.n += nData;
                                                        c2rust_current_block = 17239133558811367971;
                                                    }
                                                } else {
                                                    c2rust_current_block = 17239133558811367971;
                                                }
                                                match c2rust_current_block {
                                                    9161728879806770639 => {}
                                                    _ => {
                                                        nByte = ZIPFILE_CDS_FIXED_SZ
                                                            + e.cds.nFile as ::core::ffi::c_int
                                                            + 9 as ::core::ffi::c_int;
                                                        rc = zipfileBufferGrow(&raw mut (*p).cds, nByte);
                                                        if !(rc != 0) {
                                                            (*p).cds.n
                                                                += zipfileSerializeCDS(
                                                                    &raw mut e,
                                                                    (*p).cds.a.offset((*p).cds.n as isize) as *mut u8_0,
                                                                );
                                                            (*p).nEntry += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        sqlite3_free(aFree as *mut ::core::ffi::c_void);
        sqlite3_free(zFree as *mut ::core::ffi::c_void);
        if rc != 0 {
            if !zErr.is_null() {
                sqlite3_result_error(pCtx, zErr, -1 as ::core::ffi::c_int);
            } else {
                sqlite3_result_error_code(pCtx, rc);
            }
        }
        sqlite3_free(zErr as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn zipfileFinal(mut pCtx: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut ZipfileCtx = ::core::ptr::null_mut::<ZipfileCtx>();
        let mut eocd: ZipfileEOCD = ZipfileEOCD {
            iDisk: 0,
            iFirstDisk: 0,
            nEntry: 0,
            nEntryTotal: 0,
            nSize: 0,
            iOffset: 0,
        };
        let mut nZip: sqlite3_int64 = 0;
        let mut aZip: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        p = sqlite3_aggregate_context(
            pCtx,
            ::core::mem::size_of::<ZipfileCtx>() as ::core::ffi::c_int,
        ) as *mut ZipfileCtx;
        if p.is_null() {
            return;
        }
        if (*p).nEntry > 0 as ::core::ffi::c_int {
            memset(
                &raw mut eocd as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ZipfileEOCD>() as size_t,
            );
            eocd.nEntry = (*p).nEntry as u16_0;
            eocd.nEntryTotal = (*p).nEntry as u16_0;
            eocd.nSize = (*p).cds.n as u32_0;
            eocd.iOffset = (*p).body.n as u32_0;
            nZip = ((*p).body.n + (*p).cds.n + ZIPFILE_EOCD_FIXED_SZ) as sqlite3_int64;
            aZip = sqlite3_malloc64(nZip as sqlite3_uint64) as *mut u8_0;
            if aZip.is_null() {
                sqlite3_result_error_nomem(pCtx);
            } else {
                memcpy(
                    aZip as *mut ::core::ffi::c_void,
                    (*p).body.a as *const ::core::ffi::c_void,
                    (*p).body.n as size_t,
                );
                memcpy(
                    aZip.offset((*p).body.n as isize) as *mut u8_0
                        as *mut ::core::ffi::c_void,
                    (*p).cds.a as *const ::core::ffi::c_void,
                    (*p).cds.n as size_t,
                );
                zipfileSerializeEOCD(
                    &raw mut eocd,
                    aZip.offset(((*p).body.n + (*p).cds.n) as isize) as *mut u8_0,
                );
                sqlite3_result_blob(
                    pCtx,
                    aZip as *const ::core::ffi::c_void,
                    nZip as ::core::ffi::c_int,
                    Some(
                        zipfileFree
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
            }
        }
        sqlite3_free((*p).body.a as *mut ::core::ffi::c_void);
        sqlite3_free((*p).cds.a as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn zipfileRegister(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        static mut zipfileModule: sqlite3_module = unsafe {
            sqlite3_module {
                iVersion: 1 as ::core::ffi::c_int,
                xCreate: Some(
                    zipfileConnect
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
                    zipfileConnect
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
                    zipfileBestIndex
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut sqlite3_index_info,
                        ) -> ::core::ffi::c_int,
                ),
                xDisconnect: Some(
                    zipfileDisconnect
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xDestroy: Some(
                    zipfileDisconnect
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xOpen: Some(
                    zipfileOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xClose: Some(
                    zipfileClose
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xFilter: Some(
                    zipfileFilter
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> ::core::ffi::c_int,
                ),
                xNext: Some(
                    zipfileNext
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xEof: Some(
                    zipfileEof
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xColumn: Some(
                    zipfileColumn
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xRowid: None,
                xUpdate: Some(
                    zipfileUpdate
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                            *mut sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xBegin: Some(
                    zipfileBegin
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xSync: None,
                xCommit: Some(
                    zipfileCommit
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xRollback: Some(
                    zipfileRollback
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xFindFunction: Some(
                    zipfileFindFunction
                        as unsafe extern "C" fn(
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
                ),
                xRename: None,
                xSavepoint: None,
                xRelease: None,
                xRollbackTo: None,
                xShadowName: None,
                xIntegrity: None,
            }
        };
        let mut rc: ::core::ffi::c_int = sqlite3_create_module(
            db,
            b"zipfile\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut zipfileModule,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        if rc == SQLITE_OK {
            rc = sqlite3_overload_function(
                db,
                b"zipfile_cds\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3_create_function(
                db,
                b"zipfile\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
                Some(
                    zipfileStep
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                Some(zipfileFinal as unsafe extern "C" fn(*mut sqlite3_context) -> ()),
            );
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_zipfile_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        return zipfileRegister(db);
    }
}
