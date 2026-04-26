pub type SqliteExtCollationFn = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    ::core::ffi::c_int,
    *const ::core::ffi::c_void,
    ::core::ffi::c_int,
    *const ::core::ffi::c_void,
) -> ::core::ffi::c_int;

pub type SqliteExtFuncFn = unsafe extern "C" fn(
    *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    ::core::ffi::c_int,
    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ();

pub type SqliteExtFinalFn =
    unsafe extern "C" fn(*mut crate::src::headers::vdbeInt_h::sqlite3_context) -> ();

pub type SqliteExtDestructorFn = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ();

pub type SqliteCreateFunctionFn = unsafe extern "C" fn(
    *mut crate::src::headers::sqliteInt_h::sqlite3,
    *const ::core::ffi::c_char,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_void,
    Option<SqliteExtFuncFn>,
    Option<SqliteExtFuncFn>,
    Option<SqliteExtFinalFn>,
) -> ::core::ffi::c_int;

pub type SqliteCreateFunction16Fn = unsafe extern "C" fn(
    *mut crate::src::headers::sqliteInt_h::sqlite3,
    *const ::core::ffi::c_void,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_void,
    Option<SqliteExtFuncFn>,
    Option<SqliteExtFuncFn>,
    Option<SqliteExtFinalFn>,
) -> ::core::ffi::c_int;

pub type SqliteAuthorizerCallbackFn = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    ::core::ffi::c_int,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;

pub type SqliteUpdateHookCallbackFn = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    ::core::ffi::c_int,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    crate::src::headers::sqlite3_h::SqliteInt64,
) -> ();

pub type SqliteCreateFunctionV2Fn = unsafe extern "C" fn(
    *mut crate::src::headers::sqliteInt_h::sqlite3,
    *const ::core::ffi::c_char,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_void,
    Option<SqliteExtFuncFn>,
    Option<SqliteExtFuncFn>,
    Option<SqliteExtFinalFn>,
    Option<SqliteExtDestructorFn>,
) -> ::core::ffi::c_int;

pub type SqliteTraceV2CallbackFn = unsafe extern "C" fn(
    ::core::ffi::c_uint,
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;

pub type SqliteCreateWindowFunctionFn = unsafe extern "C" fn(
    *mut crate::src::headers::sqliteInt_h::sqlite3,
    *const ::core::ffi::c_char,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_void,
    Option<SqliteExtFuncFn>,
    Option<SqliteExtFinalFn>,
    Option<SqliteExtFinalFn>,
    Option<SqliteExtFuncFn>,
    Option<SqliteExtDestructorFn>,
) -> ::core::ffi::c_int;

pub type SqliteAutovacuumPagesCallbackFn = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *const ::core::ffi::c_char,
    ::core::ffi::c_uint,
    ::core::ffi::c_uint,
    ::core::ffi::c_uint,
) -> ::core::ffi::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_api_routines {
    pub aggregate_context: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub aggregate_count: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_blob: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<SqliteExtDestructorFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_double: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            ::core::ffi::c_double,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_int: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_int64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            crate::src::headers::sqlite3_h::SqliteInt64,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_null: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_parameter_count: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_parameter_index: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_parameter_name: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub bind_text: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            Option<SqliteExtDestructorFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_text16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<SqliteExtDestructorFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_value: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            *const crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub busy_handler: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
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
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub changes: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub close: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub collation_needed: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut crate::src::headers::sqliteInt_h::sqlite3,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                ) -> (),
            >,
        ) -> ::core::ffi::c_int,
    >,
    pub collation_needed16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut crate::src::headers::sqliteInt_h::sqlite3,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> (),
            >,
        ) -> ::core::ffi::c_int,
    >,
    pub column_blob: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_bytes: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub column_bytes16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub column_count: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub column_database_name: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub column_database_name16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_decltype: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub column_decltype16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_double: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_double,
    >,
    pub column_int: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub column_int64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> crate::src::headers::sqlite3_h::SqliteInt64,
    >,
    pub column_name: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub column_name16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_origin_name: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub column_origin_name16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_table_name: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub column_table_name16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_text: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_uchar,
    >,
    pub column_text16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_void,
    >,
    pub column_type: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub column_value: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    >,
    pub commit_hook: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub complete: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub complete16: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub create_collation: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<SqliteExtCollationFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub create_collation16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<SqliteExtCollationFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub create_function: Option<SqliteCreateFunctionFn>,
    pub create_function16: Option<SqliteCreateFunction16Fn>,
    pub create_module: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            *const crate::src::headers::sqlite3_h::sqlite3_module,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub data_count: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub db_handle: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> *mut crate::src::headers::sqliteInt_h::sqlite3,
    >,
    pub declare_vtab: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub enable_shared_cache: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub errcode: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub errmsg: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> *const ::core::ffi::c_char,
    >,
    pub errmsg16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> *const ::core::ffi::c_void,
    >,
    pub exec: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            crate::src::headers::sqlite3_h::Sqlite3Callback,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub expired: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub finalize: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub free: Option<SqliteExtDestructorFn>,
    pub free_table: Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_char) -> ()>,
    pub get_autocommit: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub get_auxdata: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub get_table: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            *mut *mut *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub global_recover: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub interruptx:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ()>,
    pub last_insert_rowid: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> crate::src::headers::sqlite3_h::SqliteInt64,
    >,
    pub libversion: Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
    pub libversion_number: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub malloc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> *mut ::core::ffi::c_void>,
    pub mprintf:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char>,
    pub open: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *mut *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> ::core::ffi::c_int,
    >,
    pub open16: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *mut *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            *mut *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub profile: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    crate::src::headers::sqlite3_h::SqliteUint64,
                ) -> (),
            >,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub progress_handler: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
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
    pub reset: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub result_blob: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<SqliteExtDestructorFn>,
        ) -> (),
    >,
    pub result_double: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub result_error: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub result_error16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub result_int: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub result_int64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            crate::src::headers::sqlite3_h::SqliteInt64,
        ) -> (),
    >,
    pub result_null:
        Option<SqliteExtFinalFn>,
    pub result_text: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            Option<SqliteExtDestructorFn>,
        ) -> (),
    >,
    pub result_text16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<SqliteExtDestructorFn>,
        ) -> (),
    >,
    pub result_text16be: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<SqliteExtDestructorFn>,
        ) -> (),
    >,
    pub result_text16le: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            Option<SqliteExtDestructorFn>,
        ) -> (),
    >,
    pub result_value: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> (),
    >,
    pub rollback_hook: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            Option<SqliteExtDestructorFn>,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub set_authorizer: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            Option<SqliteAuthorizerCallbackFn>,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub set_auxdata: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<SqliteExtDestructorFn>,
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
    pub step: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub table_column_metadata: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
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
    pub total_changes: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub trace: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> ()>,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub transfer_bindings: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub update_hook: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            Option<SqliteUpdateHookCallbackFn>,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub user_data: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub value_blob: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> *const ::core::ffi::c_void,
    >,
    pub value_bytes: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub value_bytes16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub value_double: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_double,
    >,
    pub value_int: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub value_int64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> crate::src::headers::sqlite3_h::SqliteInt64,
    >,
    pub value_numeric_type: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub value_text: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> *const ::core::ffi::c_uchar,
    >,
    pub value_text16: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> *const ::core::ffi::c_void,
    >,
    pub value_text16be: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> *const ::core::ffi::c_void,
    >,
    pub value_text16le: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> *const ::core::ffi::c_void,
    >,
    pub value_type: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub vmprintf: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void, /* VaList — opaque, handled in C */
        ) -> *mut ::core::ffi::c_char,
    >,
    pub overload_function: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare_v2: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare16_v2: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            *mut *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub clear_bindings: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub create_module_v2: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            *const crate::src::headers::sqlite3_h::sqlite3_module,
            *mut ::core::ffi::c_void,
            Option<SqliteExtDestructorFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_zeroblob: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub blob_bytes: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Blob,
        ) -> ::core::ffi::c_int,
    >,
    pub blob_close: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Blob,
        ) -> ::core::ffi::c_int,
    >,
    pub blob_open: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            crate::src::headers::sqlite3_h::Sqlite3Int64,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::sqlite3_h::Sqlite3Blob,
        ) -> ::core::ffi::c_int,
    >,
    pub blob_read: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Blob,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub blob_write: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Blob,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub create_collation_v2: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<SqliteExtCollationFn>,
            Option<SqliteExtDestructorFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub file_control: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub memory_highwater: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> crate::src::headers::sqlite3_h::Sqlite3Int64,
    >,
    pub memory_used:
        Option<unsafe extern "C" fn() -> crate::src::headers::sqlite3_h::Sqlite3Int64>,
    pub mutex_alloc: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut crate::src::src::mutex_unix::sqlite3_mutex,
    >,
    pub mutex_enter:
        Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
    pub mutex_free:
        Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
    pub mutex_leave:
        Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
    pub mutex_try: Option<
        unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int,
    >,
    pub open_v2: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *mut *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub release_memory: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub result_error_nomem:
        Option<SqliteExtFinalFn>,
    pub result_error_toobig:
        Option<SqliteExtFinalFn>,
    pub sleep: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub soft_heap_limit: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    pub vfs_find: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
        ) -> *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    >,
    pub vfs_register: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub vfs_unregister: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
        ) -> ::core::ffi::c_int,
    >,
    pub xthreadsafe: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub result_zeroblob: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub result_error_code: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub test_control: Option<unsafe extern "C" fn(::core::ffi::c_int, ...) -> ::core::ffi::c_int>,
    pub randomness:
        Option<unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void) -> ()>,
    pub context_db_handle: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        ) -> *mut crate::src::headers::sqliteInt_h::sqlite3,
    >,
    pub extended_result_codes: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub limit: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub next_stmt: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    >,
    pub sql: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> *const ::core::ffi::c_char,
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
        unsafe extern "C" fn(*mut crate::src::src::backup::sqlite3_backup) -> ::core::ffi::c_int,
    >,
    pub backup_init: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
        ) -> *mut crate::src::src::backup::sqlite3_backup,
    >,
    pub backup_pagecount: Option<
        unsafe extern "C" fn(*mut crate::src::src::backup::sqlite3_backup) -> ::core::ffi::c_int,
    >,
    pub backup_remaining: Option<
        unsafe extern "C" fn(*mut crate::src::src::backup::sqlite3_backup) -> ::core::ffi::c_int,
    >,
    pub backup_step: Option<
        unsafe extern "C" fn(
            *mut crate::src::src::backup::sqlite3_backup,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub compileoption_get:
        Option<unsafe extern "C" fn(::core::ffi::c_int) -> *const ::core::ffi::c_char>,
    pub compileoption_used:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub create_function_v2: Option<SqliteCreateFunctionV2Fn>,
    pub db_config: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int,
    >,
    pub db_mutex: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> *mut crate::src::src::mutex_unix::sqlite3_mutex,
    >,
    pub db_status: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub extended_errcode: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub log:
        Option<unsafe extern "C" fn(::core::ffi::c_int, *const ::core::ffi::c_char, ...) -> ()>,
    pub soft_heap_limit64: Option<
        unsafe extern "C" fn(
            crate::src::headers::sqlite3_h::Sqlite3Int64,
        ) -> crate::src::headers::sqlite3_h::Sqlite3Int64,
    >,
    pub sourceid: Option<unsafe extern "C" fn() -> *const ::core::ffi::c_char>,
    pub stmt_status: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
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
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_void, ::core::ffi::c_int) -> ()>,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub wal_autocheckpoint: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub wal_checkpoint: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub wal_hook: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut crate::src::headers::sqliteInt_h::sqlite3,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub blob_reopen: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Blob,
            crate::src::headers::sqlite3_h::Sqlite3Int64,
        ) -> ::core::ffi::c_int,
    >,
    pub vtab_config: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int,
    >,
    pub vtab_on_conflict: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub close_v2: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub db_filename: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >,
    pub db_readonly: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub db_release_memory: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub errstr: Option<unsafe extern "C" fn(::core::ffi::c_int) -> *const ::core::ffi::c_char>,
    pub stmt_busy: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub stmt_readonly: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
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
            crate::src::headers::sqlite3_h::Sqlite3Int64,
        ) -> crate::src::headers::sqlite3_h::Sqlite3Int64,
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
            *mut ::core::ffi::c_void, /* VaList — opaque, handled in C */
        ) -> *mut ::core::ffi::c_char,
    >,
    pub wal_checkpoint_v2: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub auto_extension:
        Option<unsafe extern "C" fn(Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int>,
    pub bind_blob64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            crate::src::headers::sqlite3_h::Sqlite3Uint64,
            Option<SqliteExtDestructorFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_text64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            crate::src::headers::sqlite3_h::Sqlite3Uint64,
            Option<SqliteExtDestructorFn>,
            ::core::ffi::c_uchar,
        ) -> ::core::ffi::c_int,
    >,
    pub cancel_auto_extension:
        Option<unsafe extern "C" fn(Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int>,
    pub load_extension: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub malloc64: Option<
        unsafe extern "C" fn(
            crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub msize: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
        ) -> crate::src::headers::sqlite3_h::Sqlite3Uint64,
    >,
    pub realloc64: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub reset_auto_extension: Option<unsafe extern "C" fn() -> ()>,
    pub result_blob64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *const ::core::ffi::c_void,
            crate::src::headers::sqlite3_h::Sqlite3Uint64,
            Option<SqliteExtDestructorFn>,
        ) -> (),
    >,
    pub result_text64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *const ::core::ffi::c_char,
            crate::src::headers::sqlite3_h::Sqlite3Uint64,
            Option<SqliteExtDestructorFn>,
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
        unsafe extern "C" fn(
            *const crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    >,
    pub value_free:
        Option<unsafe extern "C" fn(*mut crate::src::headers::vdbeInt_h::sqlite3_value) -> ()>,
    pub result_zeroblob64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_zeroblob64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) -> ::core::ffi::c_int,
    >,
    pub value_subtype: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_uint,
    >,
    pub result_subtype: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_uint,
        ) -> (),
    >,
    pub status64: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
            *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
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
    pub db_cacheflush: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub system_errno: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub trace_v2: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_uint,
            Option<SqliteTraceV2CallbackFn>,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub expanded_sql: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub set_last_insert_rowid: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            crate::src::headers::sqlite3_h::Sqlite3Int64,
        ) -> (),
    >,
    pub prepare_v3: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_uint,
            *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare16_v3: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_uint,
            *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            *mut *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub bind_pointer: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            Option<SqliteExtDestructorFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub result_pointer: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            Option<SqliteExtDestructorFn>,
        ) -> (),
    >,
    pub value_pointer: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub vtab_nochange: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        ) -> ::core::ffi::c_int,
    >,
    pub value_nochange: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub vtab_collation: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
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
        unsafe extern "C" fn(*const ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub str_new: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> *mut crate::src::headers::sqliteInt_h::sqlite3_str,
    >,
    pub str_finish: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub str_appendf: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3_str,
            *const ::core::ffi::c_char,
            ...
        ) -> (),
    >,
    pub str_vappendf: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3_str,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void, /* VaList — opaque, handled in C */
        ) -> (),
    >,
    pub str_append: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3_str,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub str_appendall: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3_str,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub str_appendchar: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3_str,
            ::core::ffi::c_int,
            ::core::ffi::c_char,
        ) -> (),
    >,
    pub str_reset:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3_str) -> ()>,
    pub str_errcode: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        ) -> ::core::ffi::c_int,
    >,
    pub str_length: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        ) -> ::core::ffi::c_int,
    >,
    pub str_value: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub create_window_function: Option<SqliteCreateWindowFunctionFn>,
    pub normalized_sql: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> *const ::core::ffi::c_char,
    >,
    pub stmt_isexplain: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        ) -> ::core::ffi::c_int,
    >,
    pub value_frombind: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub drop_modules: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub hard_heap_limit64: Option<
        unsafe extern "C" fn(
            crate::src::headers::sqlite3_h::Sqlite3Int64,
        ) -> crate::src::headers::sqlite3_h::Sqlite3Int64,
    >,
    pub uri_key: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub filename_database:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *const ::core::ffi::c_char>,
    pub filename_journal:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *const ::core::ffi::c_char>,
    pub filename_wal:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *const ::core::ffi::c_char>,
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
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
        ) -> *mut crate::src::headers::sqlite3_h::sqlite3_file,
    >,
    pub txn_state: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub changes64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> crate::src::headers::sqlite3_h::Sqlite3Int64,
    >,
    pub total_changes64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> crate::src::headers::sqlite3_h::Sqlite3Int64,
    >,
    pub autovacuum_pages: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            Option<SqliteAutovacuumPagesCallbackFn>,
            *mut ::core::ffi::c_void,
            Option<SqliteExtDestructorFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub error_offset: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub vtab_rhs_value: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub vtab_distinct: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
        ) -> ::core::ffi::c_int,
    >,
    pub vtab_in: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub vtab_in_first: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub vtab_in_next: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub deserialize: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_uchar,
            crate::src::headers::sqlite3_h::Sqlite3Int64,
            crate::src::headers::sqlite3_h::Sqlite3Int64,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
    pub serialize: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
            ::core::ffi::c_uint,
        ) -> *mut ::core::ffi::c_uchar,
    >,
    pub db_name: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char,
    >,
    pub value_encoding: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub is_interrupted: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int,
    >,
    pub stmt_explain: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub get_clientdata: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub set_clientdata: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
            Option<SqliteExtDestructorFn>,
        ) -> ::core::ffi::c_int,
    >,
    pub setlk_timeout: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub set_errmsg: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub db_status64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
            *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
pub type Sqlite3LoadextEntry = Option<
    unsafe extern "C" fn(
        *mut crate::src::headers::sqliteInt_h::sqlite3,
        *mut *mut ::core::ffi::c_char,
        *const crate::src::headers::sqlite3ext_h::sqlite3_api_routines,
    ) -> ::core::ffi::c_int,
>;
