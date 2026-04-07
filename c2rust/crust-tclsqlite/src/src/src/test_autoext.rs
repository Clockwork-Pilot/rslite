unsafe extern "C" {
    pub type Tcl_Command_;
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_str;
    pub type sqlite3_blob;
    pub type sqlite3_backup;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
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
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
    fn sqlite3_auto_extension(
        xEntryPoint: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_cancel_auto_extension(
        xEntryPoint: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_reset_auto_extension();
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type ClientData = *mut ::core::ffi::c_void;
pub type Tcl_WideInt = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut ::core::ffi::c_char,
    pub freeProcDontUse: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ()>,
    pub errorLineDontUse: ::core::ffi::c_int,
}
pub type Tcl_Command = *mut Tcl_Command_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Obj {
    pub refCount: ::core::ffi::c_int,
    pub bytes: *mut ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub typePtr: *const Tcl_ObjType,
    pub internalRep: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2Rust_Unnamed_1,
    pub ptrAndLongRep: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub ptr1: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_ObjType {
    pub name: *const ::core::ffi::c_char,
    pub freeIntRepProc: Option<Tcl_FreeInternalRepProc>,
    pub dupIntRepProc: Option<Tcl_DupInternalRepProc>,
    pub updateStringProc: Option<Tcl_UpdateStringProc>,
    pub setFromAnyProc: Option<Tcl_SetFromAnyProc>,
}
pub type Tcl_SetFromAnyProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
) -> ::core::ffi::c_int;
pub type Tcl_UpdateStringProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_DupInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> ();
pub type Tcl_FreeInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_CmdDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
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
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ANY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
unsafe extern "C" fn sqrFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut r: ::core::ffi::c_double = sqlite3_value_double(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        sqlite3_result_double(context, r * r);
    }
}
unsafe extern "C" fn sqr_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_create_function(
            db,
            b"sqr\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            SQLITE_ANY,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                sqrFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn cubeFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut r: ::core::ffi::c_double = sqlite3_value_double(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        sqlite3_result_double(context, r * r * r);
    }
}
unsafe extern "C" fn cube_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_create_function(
            db,
            b"cube\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            SQLITE_ANY,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                cubeFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn broken_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        zErr = sqlite3_mprintf(
            b"broken autoext!\0".as_ptr() as *const ::core::ffi::c_char,
        );
        *pzErrMsg = zErr;
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn autoExtSqrObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_auto_extension(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut *mut ::core::ffi::c_char,
                        *const sqlite3_api_routines,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    sqr_init
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return SQLITE_OK;
    }
}
unsafe extern "C" fn cancelAutoExtSqrObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_cancel_auto_extension(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut *mut ::core::ffi::c_char,
                        *const sqlite3_api_routines,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    sqr_init
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return SQLITE_OK;
    }
}
unsafe extern "C" fn autoExtCubeObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_auto_extension(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut *mut ::core::ffi::c_char,
                        *const sqlite3_api_routines,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    cube_init
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return SQLITE_OK;
    }
}
unsafe extern "C" fn cancelAutoExtCubeObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_cancel_auto_extension(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut *mut ::core::ffi::c_char,
                        *const sqlite3_api_routines,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    cube_init
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return SQLITE_OK;
    }
}
unsafe extern "C" fn autoExtBrokenObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_auto_extension(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut *mut ::core::ffi::c_char,
                        *const sqlite3_api_routines,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    broken_init
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return SQLITE_OK;
    }
}
unsafe extern "C" fn cancelAutoExtBrokenObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_cancel_auto_extension(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut *mut ::core::ffi::c_char,
                        *const sqlite3_api_routines,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    broken_init
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return SQLITE_OK;
    }
}
unsafe extern "C" fn resetAutoExtObjCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_reset_auto_extension();
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest_autoext_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_auto_extension_sqr\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                autoExtSqrObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_auto_extension_cube\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                autoExtCubeObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_auto_extension_broken\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                autoExtBrokenObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_cancel_auto_extension_sqr\0".as_ptr()
                as *const ::core::ffi::c_char,
            Some(
                cancelAutoExtSqrObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_cancel_auto_extension_cube\0".as_ptr()
                as *const ::core::ffi::c_char,
            Some(
                cancelAutoExtCubeObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_cancel_auto_extension_broken\0".as_ptr()
                as *const ::core::ffi::c_char,
            Some(
                cancelAutoExtBrokenObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_reset_auto_extension\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                resetAutoExtObjCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        return TCL_OK;
    }
}
