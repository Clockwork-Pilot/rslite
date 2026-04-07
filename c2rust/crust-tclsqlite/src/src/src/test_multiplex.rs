unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_str;
    pub type sqlite3_blob;
    pub type sqlite3_backup;
    pub type Tcl_Command_;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_uri_boolean(
        z: sqlite3_filename,
        zParam: *const ::core::ffi::c_char,
        bDefault: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_uri_int64(
        _: sqlite3_filename,
        _: *const ::core::ffi::c_char,
        _: sqlite3_int64,
    ) -> sqlite3_int64;
    fn sqlite3_create_filename(
        zDatabase: *const ::core::ffi::c_char,
        zJournal: *const ::core::ffi::c_char,
        zWal: *const ::core::ffi::c_char,
        nParam: ::core::ffi::c_int,
        azParam: *mut *const ::core::ffi::c_char,
    ) -> sqlite3_filename;
    fn sqlite3_free_filename(_: sqlite3_filename);
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
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_auto_extension(
        xEntryPoint: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        op: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_log(
        iErrCode: ::core::ffi::c_int,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn Tcl_GetBooleanFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_GetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        infoPtr: *mut Tcl_CmdInfo,
    ) -> ::core::ffi::c_int;
    fn Tcl_SetResult(
        interp: *mut Tcl_Interp,
        result: *mut ::core::ffi::c_char,
        freeProc: Option<Tcl_FreeProc>,
    );
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_GetIndexFromObjStruct(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        tablePtr: *const ::core::ffi::c_void,
        offset: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        indexPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub pOrigVfs: *mut sqlite3_vfs,
    pub sThisVfs: sqlite3_vfs,
    pub sIoMethodsV1: sqlite3_io_methods,
    pub sIoMethodsV2: sqlite3_io_methods,
    pub isInitialized: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multiplexGroup {
    pub aReal: *mut multiplexReal,
    pub nReal: ::core::ffi::c_int,
    pub zName: *mut ::core::ffi::c_char,
    pub nName: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub szChunk: ::core::ffi::c_uint,
    pub bEnabled: ::core::ffi::c_uchar,
    pub bTruncate: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multiplexReal {
    pub p: *mut sqlite3_file,
    pub z: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multiplexConn {
    pub base: sqlite3_file,
    pub pGroup: *mut multiplexGroup,
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
    pub internalRep: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2Rust_Unnamed_2,
    pub ptrAndLongRep: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
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
pub type Tcl_CmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;
pub type Tcl_FreeProc = unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ();
pub type Tcl_NamespaceDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Namespace {
    pub name: *mut ::core::ffi::c_char,
    pub fullName: *mut ::core::ffi::c_char,
    pub clientData: ClientData,
    pub deleteProc: Option<Tcl_NamespaceDeleteProc>,
    pub parentPtr: *mut Tcl_Namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_CmdInfo {
    pub isNativeObjectProc: ::core::ffi::c_int,
    pub objProc: Option<Tcl_ObjCmdProc>,
    pub objClientData: ClientData,
    pub proc: Option<Tcl_CmdProc>,
    pub clientData: ClientData,
    pub deleteProc: Option<Tcl_CmdDeleteProc>,
    pub deleteData: ClientData,
    pub namespacePtr: *mut Tcl_Namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubCommand {
    pub zName: *const ::core::ffi::c_char,
    pub op: ::core::ffi::c_int,
    pub argtype: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_CANTOPEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_IOERR_READ: ::core::ffi::c_int = SQLITE_IOERR
    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_WRITE: ::core::ffi::c_int = SQLITE_IOERR
    | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_TRUNCATE: ::core::ffi::c_int = SQLITE_IOERR
    | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_FSTAT: ::core::ffi::c_int = SQLITE_IOERR
    | (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_UNLOCK: ::core::ffi::c_int = SQLITE_IOERR
    | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int = SQLITE_IOERR
    | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_CHECKRESERVEDLOCK: ::core::ffi::c_int = SQLITE_IOERR
    | (14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_JOURNAL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SUPER_JOURNAL: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_WAL: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_SIZE_HINT: ::core::ffi::c_int = 5;
pub const SQLITE_FCNTL_CHUNK_SIZE: ::core::ffi::c_int = 6;
pub const SQLITE_FCNTL_VFSNAME: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_PRAGMA: ::core::ffi::c_int = 14;
pub const SQLITE_ACCESS_EXISTS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ANY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const MAX_PAGE_SIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const DEFAULT_SECTOR_SIZE: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const MX_CHUNK_NUMBER: ::core::ffi::c_int = 299 as ::core::ffi::c_int;
pub const SQLITE_MULTIPLEX_VFS_NAME: [::core::ffi::c_char; 10] = unsafe {
    ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"multiplex\0")
};
pub const SQLITE_MULTIPLEX_CHUNK_SIZE: ::core::ffi::c_int = 2147418112
    as ::core::ffi::c_int;
static mut gMultiplex: C2Rust_Unnamed = C2Rust_Unnamed {
    pOrigVfs: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
    sThisVfs: sqlite3_vfs {
        iVersion: 0,
        szOsFile: 0,
        mxPathname: 0,
        pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
        zName: ::core::ptr::null::<::core::ffi::c_char>(),
        pAppData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        xOpen: None,
        xDelete: None,
        xAccess: None,
        xFullPathname: None,
        xDlOpen: None,
        xDlError: None,
        xDlSym: None,
        xDlClose: None,
        xRandomness: None,
        xSleep: None,
        xCurrentTime: None,
        xGetLastError: None,
        xCurrentTimeInt64: None,
        xSetSystemCall: None,
        xGetSystemCall: None,
        xNextSystemCall: None,
    },
    sIoMethodsV1: sqlite3_io_methods {
        iVersion: 0,
        xClose: None,
        xRead: None,
        xWrite: None,
        xTruncate: None,
        xSync: None,
        xFileSize: None,
        xLock: None,
        xUnlock: None,
        xCheckReservedLock: None,
        xFileControl: None,
        xSectorSize: None,
        xDeviceCharacteristics: None,
        xShmMap: None,
        xShmLock: None,
        xShmBarrier: None,
        xShmUnmap: None,
        xFetch: None,
        xUnfetch: None,
    },
    sIoMethodsV2: sqlite3_io_methods {
        iVersion: 0,
        xClose: None,
        xRead: None,
        xWrite: None,
        xTruncate: None,
        xSync: None,
        xFileSize: None,
        xLock: None,
        xUnlock: None,
        xCheckReservedLock: None,
        xFileControl: None,
        xSectorSize: None,
        xDeviceCharacteristics: None,
        xShmMap: None,
        xShmLock: None,
        xShmBarrier: None,
        xShmUnmap: None,
        xFetch: None,
        xUnfetch: None,
    },
    isInitialized: 0,
};
unsafe extern "C" fn multiplexStrlen30(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut z2: *const ::core::ffi::c_char = z;
        if z.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        while *z2 != 0 {
            z2 = z2.offset(1);
        }
        return 0x3fffffff as ::core::ffi::c_int
            & z2.offset_from(z) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn multiplexFilename(
    mut zBase: *const ::core::ffi::c_char,
    mut nBase: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut iChunk: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut n: ::core::ffi::c_int = nBase;
        memcpy(
            zOut as *mut ::core::ffi::c_void,
            zBase as *const ::core::ffi::c_void,
            (n + 1 as ::core::ffi::c_int) as size_t,
        );
        if iChunk != 0 as ::core::ffi::c_int && iChunk <= MX_CHUNK_NUMBER {
            sqlite3_snprintf(
                4 as ::core::ffi::c_int,
                zOut.offset(n as isize) as *mut ::core::ffi::c_char,
                b"%03d\0".as_ptr() as *const ::core::ffi::c_char,
                iChunk,
            );
            n += 3 as ::core::ffi::c_int;
        }
        *zOut.offset((n + 1 as ::core::ffi::c_int) as isize) = '\0' as i32
            as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn multiplexSubFilename(
    mut pGroup: *mut multiplexGroup,
    mut iChunk: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if iChunk >= (*pGroup).nReal {
            let mut p: *mut multiplexReal = ::core::ptr::null_mut::<multiplexReal>();
            p = sqlite3_realloc64(
                (*pGroup).aReal as *mut ::core::ffi::c_void,
                ((iChunk + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<multiplexReal>() as usize)
                    as sqlite3_uint64,
            ) as *mut multiplexReal;
            if p.is_null() {
                return SQLITE_NOMEM;
            }
            memset(
                p.offset((*pGroup).nReal as isize) as *mut multiplexReal
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<multiplexReal>() as size_t)
                    .wrapping_mul(
                        (iChunk + 1 as ::core::ffi::c_int - (*pGroup).nReal) as size_t,
                    ),
            );
            (*pGroup).aReal = p as *mut multiplexReal;
            (*pGroup).nReal = iChunk + 1 as ::core::ffi::c_int;
        }
        if !(*pGroup).zName.is_null()
            && (*(*pGroup).aReal.offset(iChunk as isize)).z.is_null()
        {
            let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut n: ::core::ffi::c_int = (*pGroup).nName;
            z = sqlite3_malloc64((n + 5 as ::core::ffi::c_int) as sqlite3_uint64)
                as *mut ::core::ffi::c_char;
            if z.is_null() {
                return SQLITE_NOMEM;
            }
            multiplexFilename(
                (*pGroup).zName,
                (*pGroup).nName,
                (*pGroup).flags,
                iChunk,
                z,
            );
            let ref mut c2rust_fresh2 = (*(*pGroup).aReal.offset(iChunk as isize)).z;
            *c2rust_fresh2 = sqlite3_create_filename(
                z,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ) as *mut ::core::ffi::c_char;
            sqlite3_free(z as *mut ::core::ffi::c_void);
            if (*(*pGroup).aReal.offset(iChunk as isize)).z.is_null() {
                return SQLITE_NOMEM;
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn multiplexSubOpen(
    mut pGroup: *mut multiplexGroup,
    mut iChunk: ::core::ffi::c_int,
    mut rc: *mut ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
    mut createFlag: ::core::ffi::c_int,
) -> *mut sqlite3_file {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut pOrigVfs: *mut sqlite3_vfs = gMultiplex.pOrigVfs;
        *rc = multiplexSubFilename(pGroup, iChunk);
        if *rc == SQLITE_OK
            && {
                pSubOpen = (*(*pGroup).aReal.offset(iChunk as isize)).p;
                pSubOpen.is_null()
            }
        {
            let mut flags: ::core::ffi::c_int = 0;
            let mut bExists: ::core::ffi::c_int = 0;
            flags = (*pGroup).flags;
            if createFlag != 0 {
                flags |= SQLITE_OPEN_CREATE;
            } else if !(iChunk == 0 as ::core::ffi::c_int) {
                if (*(*pGroup).aReal.offset(iChunk as isize)).z.is_null() {
                    return ::core::ptr::null_mut::<sqlite3_file>()
                } else {
                    *rc = (*pOrigVfs)
                        .xAccess
                        .expect(
                            "non-null function pointer",
                        )(
                        pOrigVfs,
                        (*(*pGroup).aReal.offset(iChunk as isize)).z,
                        SQLITE_ACCESS_EXISTS,
                        &raw mut bExists,
                    );
                    if *rc != 0 || bExists == 0 {
                        if *rc != 0 {
                            sqlite3_log(
                                *rc,
                                b"multiplexor.xAccess failure on %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*(*pGroup).aReal.offset(iChunk as isize)).z,
                            );
                        }
                        return ::core::ptr::null_mut::<sqlite3_file>();
                    }
                    flags &= !SQLITE_OPEN_CREATE;
                }
            }
            pSubOpen = sqlite3_malloc64((*pOrigVfs).szOsFile as sqlite3_uint64)
                as *mut sqlite3_file;
            if pSubOpen.is_null() {
                *rc = SQLITE_IOERR_NOMEM;
                return ::core::ptr::null_mut::<sqlite3_file>();
            }
            let ref mut c2rust_fresh0 = (*(*pGroup).aReal.offset(iChunk as isize)).p;
            *c2rust_fresh0 = pSubOpen;
            *rc = (*pOrigVfs)
                .xOpen
                .expect(
                    "non-null function pointer",
                )(
                pOrigVfs,
                (*(*pGroup).aReal.offset(iChunk as isize)).z as sqlite3_filename,
                pSubOpen,
                flags,
                pOutFlags,
            );
            if *rc != SQLITE_OK {
                sqlite3_log(
                    *rc,
                    b"multiplexor.xOpen failure on %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(*pGroup).aReal.offset(iChunk as isize)).z,
                );
                sqlite3_free(pSubOpen as *mut ::core::ffi::c_void);
                let ref mut c2rust_fresh1 = (*(*pGroup).aReal.offset(iChunk as isize)).p;
                *c2rust_fresh1 = ::core::ptr::null_mut::<sqlite3_file>();
                return ::core::ptr::null_mut::<sqlite3_file>();
            }
        }
        return pSubOpen;
    }
}
unsafe extern "C" fn multiplexSubSize(
    mut pGroup: *mut multiplexGroup,
    mut iChunk: ::core::ffi::c_int,
    mut rc: *mut ::core::ffi::c_int,
) -> sqlite3_int64 {
    unsafe {
        let mut pSub: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut sz: sqlite3_int64 = 0 as sqlite3_int64;
        if *rc != 0 {
            return 0 as sqlite3_int64;
        }
        pSub = multiplexSubOpen(
            pGroup,
            iChunk,
            rc,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if pSub.is_null() {
            return 0 as sqlite3_int64;
        }
        *rc = (*(*pSub).pMethods)
            .xFileSize
            .expect("non-null function pointer")(pSub, &raw mut sz);
        return sz;
    }
}
unsafe extern "C" fn multiplexControlFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut op: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iVal: ::core::ffi::c_int = 0;
        if db.is_null() || argc != 2 as ::core::ffi::c_int {
            rc = SQLITE_ERROR;
        } else {
            op = sqlite3_value_int(*argv.offset(0 as ::core::ffi::c_int as isize));
            iVal = sqlite3_value_int(*argv.offset(1 as ::core::ffi::c_int as isize));
            match op {
                1 => {
                    op = MULTIPLEX_CTRL_ENABLE;
                }
                2 => {
                    op = MULTIPLEX_CTRL_SET_CHUNK_SIZE;
                }
                3 => {
                    op = MULTIPLEX_CTRL_SET_MAX_CHUNKS;
                }
                _ => {
                    rc = SQLITE_NOTFOUND;
                }
            }
        }
        if rc == SQLITE_OK {
            rc = sqlite3_file_control(
                db,
                ::core::ptr::null::<::core::ffi::c_char>(),
                op,
                &raw mut iVal as *mut ::core::ffi::c_void,
            );
        }
        sqlite3_result_error_code(context, rc);
    }
}
unsafe extern "C" fn multiplexFuncInit(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3_create_function(
            db,
            b"multiplex_control\0".as_ptr() as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
            SQLITE_ANY,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                multiplexControlFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
        return rc;
    }
}
unsafe extern "C" fn multiplexSubClose(
    mut pGroup: *mut multiplexGroup,
    mut iChunk: ::core::ffi::c_int,
    mut pOrigVfs: *mut sqlite3_vfs,
) {
    unsafe {
        let mut pSubOpen: *mut sqlite3_file = (*(*pGroup).aReal.offset(iChunk as isize))
            .p;
        if !pSubOpen.is_null() {
            (*(*pSubOpen).pMethods).xClose.expect("non-null function pointer")(pSubOpen);
            if !pOrigVfs.is_null()
                && !(*(*pGroup).aReal.offset(iChunk as isize)).z.is_null()
            {
                (*pOrigVfs)
                    .xDelete
                    .expect(
                        "non-null function pointer",
                    )(
                    pOrigVfs,
                    (*(*pGroup).aReal.offset(iChunk as isize)).z,
                    0 as ::core::ffi::c_int,
                );
            }
            sqlite3_free(
                (*(*pGroup).aReal.offset(iChunk as isize)).p as *mut ::core::ffi::c_void,
            );
        }
        sqlite3_free_filename(
            (*(*pGroup).aReal.offset(iChunk as isize)).z as sqlite3_filename,
        );
        memset(
            (*pGroup).aReal.offset(iChunk as isize) as *mut multiplexReal
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<multiplexReal>() as size_t,
        );
    }
}
unsafe extern "C" fn multiplexFreeComponents(mut pGroup: *mut multiplexGroup) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pGroup).nReal {
            multiplexSubClose(pGroup, i, ::core::ptr::null_mut::<sqlite3_vfs>());
            i += 1;
        }
        sqlite3_free((*pGroup).aReal as *mut ::core::ffi::c_void);
        (*pGroup).aReal = ::core::ptr::null_mut::<multiplexReal>();
        (*pGroup).nReal = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn multiplexOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pConn: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pMultiplexOpen: *mut multiplexConn = ::core::ptr::null_mut::<
            multiplexConn,
        >();
        let mut pGroup: *mut multiplexGroup = ::core::ptr::null_mut::<multiplexGroup>();
        let mut pSubOpen: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut pOrigVfs: *mut sqlite3_vfs = gMultiplex.pOrigVfs;
        let mut nName: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zToFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        memset(
            pConn as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (*pVfs).szOsFile as size_t,
        );
        pMultiplexOpen = pConn as *mut multiplexConn;
        if rc == SQLITE_OK {
            nName = if !zName.is_null() {
                multiplexStrlen30(zName)
            } else {
                0 as ::core::ffi::c_int
            };
            sz = (::core::mem::size_of::<multiplexGroup>() as usize)
                .wrapping_add(nName as usize)
                .wrapping_add(1 as usize) as ::core::ffi::c_int;
            pGroup = sqlite3_malloc64(sz as sqlite3_uint64) as *mut multiplexGroup;
            if pGroup.is_null() {
                rc = SQLITE_NOMEM;
            }
        }
        if rc == SQLITE_OK {
            let mut zUri: *const ::core::ffi::c_char = if flags & SQLITE_OPEN_URI != 0 {
                zName
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            };
            memset(
                pGroup as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                sz as size_t,
            );
            (*pMultiplexOpen).pGroup = pGroup;
            (*pGroup).bEnabled = -1 as ::core::ffi::c_int as ::core::ffi::c_uchar;
            (*pGroup).bTruncate = sqlite3_uri_boolean(
                zUri as sqlite3_filename,
                b"truncate\0".as_ptr() as *const ::core::ffi::c_char,
                (flags & SQLITE_OPEN_MAIN_DB == 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int,
            ) as ::core::ffi::c_uchar;
            (*pGroup).szChunk = sqlite3_uri_int64(
                zUri as sqlite3_filename,
                b"chunksize\0".as_ptr() as *const ::core::ffi::c_char,
                SQLITE_MULTIPLEX_CHUNK_SIZE as sqlite3_int64,
            ) as ::core::ffi::c_int as ::core::ffi::c_uint;
            (*pGroup).szChunk = (*pGroup)
                .szChunk
                .wrapping_add(0xffff as ::core::ffi::c_uint)
                & !(0xffff as ::core::ffi::c_int) as ::core::ffi::c_uint;
            if !zName.is_null() {
                let mut p: *mut ::core::ffi::c_char = pGroup
                    .offset(1 as ::core::ffi::c_int as isize) as *mut multiplexGroup
                    as *mut ::core::ffi::c_char;
                (*pGroup).zName = p;
                memcpy(
                    (*pGroup).zName as *mut ::core::ffi::c_void,
                    zName as *const ::core::ffi::c_void,
                    (nName + 1 as ::core::ffi::c_int) as size_t,
                );
                (*pGroup).nName = nName;
            }
            if (*pGroup).bEnabled != 0 {
                unsafe extern "C" {
                    static mut sqlite3PendingByte: ::core::ffi::c_int;
                }
                while (sqlite3PendingByte as ::core::ffi::c_uint)
                    .wrapping_rem((*pGroup).szChunk)
                    >= (*pGroup)
                        .szChunk
                        .wrapping_sub(65536 as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    (*pGroup).szChunk = (*pGroup)
                        .szChunk
                        .wrapping_add(
                            65536 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        );
                }
            }
            (*pGroup).flags = flags & !SQLITE_OPEN_URI;
            rc = multiplexSubFilename(pGroup, 1 as ::core::ffi::c_int);
            if rc == SQLITE_OK {
                pSubOpen = multiplexSubOpen(
                    pGroup,
                    0 as ::core::ffi::c_int,
                    &raw mut rc,
                    pOutFlags,
                    0 as ::core::ffi::c_int,
                );
                if pSubOpen.is_null() && rc == SQLITE_OK {
                    rc = SQLITE_CANTOPEN;
                }
            }
            if rc == SQLITE_OK {
                let mut sz64: sqlite3_int64 = 0;
                rc = (*(*pSubOpen).pMethods)
                    .xFileSize
                    .expect("non-null function pointer")(pSubOpen, &raw mut sz64);
                if rc == SQLITE_OK && !zName.is_null() {
                    let mut bExists: ::core::ffi::c_int = 0;
                    if flags & SQLITE_OPEN_SUPER_JOURNAL != 0 {
                        (*pGroup).bEnabled = 0 as ::core::ffi::c_uchar;
                    } else if sz64 == 0 as ::core::ffi::c_longlong {
                        if flags & SQLITE_OPEN_MAIN_JOURNAL != 0 {
                            let mut iChunk: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                            loop {
                                rc = (*pOrigVfs)
                                    .xAccess
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    pOrigVfs,
                                    (*(*pGroup).aReal.offset(iChunk as isize)).z,
                                    SQLITE_ACCESS_EXISTS,
                                    &raw mut bExists,
                                );
                                if rc == SQLITE_OK && bExists != 0 {
                                    rc = (*pOrigVfs)
                                        .xDelete
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        pOrigVfs,
                                        (*(*pGroup).aReal.offset(iChunk as isize)).z,
                                        0 as ::core::ffi::c_int,
                                    );
                                    if rc == SQLITE_OK {
                                        iChunk += 1;
                                        rc = multiplexSubFilename(pGroup, iChunk);
                                    }
                                }
                                if !(rc == SQLITE_OK && bExists != 0) {
                                    break;
                                }
                            }
                        }
                    } else {
                        rc = (*pOrigVfs)
                            .xAccess
                            .expect(
                                "non-null function pointer",
                            )(
                            pOrigVfs,
                            (*(*pGroup).aReal.offset(1 as ::core::ffi::c_int as isize))
                                .z,
                            SQLITE_ACCESS_EXISTS,
                            &raw mut bExists,
                        );
                        bExists = (multiplexSubSize(
                            pGroup,
                            1 as ::core::ffi::c_int,
                            &raw mut rc,
                        ) > 0 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
                        if rc == SQLITE_OK && bExists != 0
                            && sz64
                                == sz64 as ::core::ffi::c_longlong
                                    & 0xffff0000 as ::core::ffi::c_longlong
                            && sz64 > 0 as ::core::ffi::c_longlong
                            && sz64 != (*pGroup).szChunk as ::core::ffi::c_longlong
                        {
                            (*pGroup).szChunk = sz64 as ::core::ffi::c_int
                                as ::core::ffi::c_uint;
                        } else if rc == SQLITE_OK && bExists == 0
                            && sz64 > (*pGroup).szChunk as ::core::ffi::c_longlong
                        {
                            (*pGroup).bEnabled = 0 as ::core::ffi::c_uchar;
                        }
                    }
                }
            }
            if rc == SQLITE_OK {
                if (*(*pSubOpen).pMethods).iVersion == 1 as ::core::ffi::c_int {
                    (*pConn).pMethods = &raw mut gMultiplex.sIoMethodsV1;
                } else {
                    (*pConn).pMethods = &raw mut gMultiplex.sIoMethodsV2;
                }
            } else {
                multiplexFreeComponents(pGroup);
                sqlite3_free(pGroup as *mut ::core::ffi::c_void);
            }
        }
        sqlite3_free(zToFree as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn multiplexDelete(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut syncDir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pOrigVfs: *mut sqlite3_vfs = gMultiplex.pOrigVfs;
        rc = (*pOrigVfs)
            .xDelete
            .expect("non-null function pointer")(pOrigVfs, zName, syncDir);
        if rc == SQLITE_OK {
            let mut nName: ::core::ffi::c_int = strlen(zName) as ::core::ffi::c_int;
            let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            z = sqlite3_malloc64((nName + 5 as ::core::ffi::c_int) as sqlite3_uint64)
                as *mut ::core::ffi::c_char;
            if z.is_null() {
                rc = SQLITE_IOERR_NOMEM;
            } else {
                let mut iChunk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut bExists: ::core::ffi::c_int = 0;
                loop {
                    iChunk += 1;
                    multiplexFilename(zName, nName, SQLITE_OPEN_MAIN_JOURNAL, iChunk, z);
                    rc = (*pOrigVfs)
                        .xAccess
                        .expect(
                            "non-null function pointer",
                        )(pOrigVfs, z, SQLITE_ACCESS_EXISTS, &raw mut bExists);
                    if !(rc == SQLITE_OK && bExists != 0) {
                        break;
                    }
                }
                while rc == SQLITE_OK && iChunk > 1 as ::core::ffi::c_int {
                    iChunk -= 1;
                    multiplexFilename(zName, nName, SQLITE_OPEN_MAIN_JOURNAL, iChunk, z);
                    rc = (*pOrigVfs)
                        .xDelete
                        .expect("non-null function pointer")(pOrigVfs, z, syncDir);
                }
                if rc == SQLITE_OK {
                    iChunk = 0 as ::core::ffi::c_int;
                    loop {
                        iChunk += 1;
                        multiplexFilename(zName, nName, SQLITE_OPEN_WAL, iChunk, z);
                        rc = (*pOrigVfs)
                            .xAccess
                            .expect(
                                "non-null function pointer",
                            )(pOrigVfs, z, SQLITE_ACCESS_EXISTS, &raw mut bExists);
                        if !(rc == SQLITE_OK && bExists != 0) {
                            break;
                        }
                    }
                    while rc == SQLITE_OK && iChunk > 1 as ::core::ffi::c_int {
                        iChunk -= 1;
                        multiplexFilename(zName, nName, SQLITE_OPEN_WAL, iChunk, z);
                        rc = (*pOrigVfs)
                            .xDelete
                            .expect("non-null function pointer")(pOrigVfs, z, syncDir);
                    }
                }
            }
            sqlite3_free(z as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn multiplexAccess(
    mut a: *mut sqlite3_vfs,
    mut b: *const ::core::ffi::c_char,
    mut c: ::core::ffi::c_int,
    mut d: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*gMultiplex.pOrigVfs)
            .xAccess
            .expect("non-null function pointer")(gMultiplex.pOrigVfs, b, c, d);
    }
}
unsafe extern "C" fn multiplexFullPathname(
    mut a: *mut sqlite3_vfs,
    mut b: *const ::core::ffi::c_char,
    mut c: ::core::ffi::c_int,
    mut d: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*gMultiplex.pOrigVfs)
            .xFullPathname
            .expect("non-null function pointer")(gMultiplex.pOrigVfs, b, c, d);
    }
}
unsafe extern "C" fn multiplexDlOpen(
    mut a: *mut sqlite3_vfs,
    mut b: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*gMultiplex.pOrigVfs)
            .xDlOpen
            .expect("non-null function pointer")(gMultiplex.pOrigVfs, b);
    }
}
unsafe extern "C" fn multiplexDlError(
    mut a: *mut sqlite3_vfs,
    mut b: ::core::ffi::c_int,
    mut c: *mut ::core::ffi::c_char,
) {
    unsafe {
        (*gMultiplex.pOrigVfs)
            .xDlError
            .expect("non-null function pointer")(gMultiplex.pOrigVfs, b, c);
    }
}
unsafe extern "C" fn multiplexDlSym(
    mut a: *mut sqlite3_vfs,
    mut b: *mut ::core::ffi::c_void,
    mut c: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        return (*gMultiplex.pOrigVfs)
            .xDlSym
            .expect("non-null function pointer")(gMultiplex.pOrigVfs, b, c);
    }
}
unsafe extern "C" fn multiplexDlClose(
    mut a: *mut sqlite3_vfs,
    mut b: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*gMultiplex.pOrigVfs)
            .xDlClose
            .expect("non-null function pointer")(gMultiplex.pOrigVfs, b);
    }
}
unsafe extern "C" fn multiplexRandomness(
    mut a: *mut sqlite3_vfs,
    mut b: ::core::ffi::c_int,
    mut c: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return (*gMultiplex.pOrigVfs)
            .xRandomness
            .expect("non-null function pointer")(gMultiplex.pOrigVfs, b, c);
    }
}
unsafe extern "C" fn multiplexSleep(
    mut a: *mut sqlite3_vfs,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return (*gMultiplex.pOrigVfs)
            .xSleep
            .expect("non-null function pointer")(gMultiplex.pOrigVfs, b);
    }
}
unsafe extern "C" fn multiplexCurrentTime(
    mut a: *mut sqlite3_vfs,
    mut b: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        return (*gMultiplex.pOrigVfs)
            .xCurrentTime
            .expect("non-null function pointer")(gMultiplex.pOrigVfs, b);
    }
}
unsafe extern "C" fn multiplexGetLastError(
    mut a: *mut sqlite3_vfs,
    mut b: ::core::ffi::c_int,
    mut c: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if (*gMultiplex.pOrigVfs).xGetLastError.is_some() {
            return (*gMultiplex.pOrigVfs)
                .xGetLastError
                .expect("non-null function pointer")(gMultiplex.pOrigVfs, b, c)
        } else {
            return 0 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn multiplexCurrentTimeInt64(
    mut a: *mut sqlite3_vfs,
    mut b: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        return (*gMultiplex.pOrigVfs)
            .xCurrentTimeInt64
            .expect("non-null function pointer")(gMultiplex.pOrigVfs, b);
    }
}
unsafe extern "C" fn multiplexClose(mut pConn: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut pGroup: *mut multiplexGroup = (*p).pGroup;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        multiplexFreeComponents(pGroup);
        sqlite3_free(pGroup as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn multiplexRead(
    mut pConn: *mut sqlite3_file,
    mut pBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut pGroup: *mut multiplexGroup = (*p).pGroup;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pGroup).bEnabled == 0 {
            let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
                pGroup,
                0 as ::core::ffi::c_int,
                &raw mut rc,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                0 as ::core::ffi::c_int,
            );
            if pSubOpen.is_null() {
                rc = SQLITE_IOERR_READ;
            } else {
                rc = (*(*pSubOpen).pMethods)
                    .xRead
                    .expect("non-null function pointer")(pSubOpen, pBuf, iAmt, iOfst);
            }
        } else {
            while iAmt > 0 as ::core::ffi::c_int {
                let mut i: ::core::ffi::c_int = (iOfst as ::core::ffi::c_longlong
                    / (*pGroup).szChunk as ::core::ffi::c_longlong)
                    as ::core::ffi::c_int;
                let mut pSubOpen_0: *mut sqlite3_file = ::core::ptr::null_mut::<
                    sqlite3_file,
                >();
                pSubOpen_0 = multiplexSubOpen(
                    pGroup,
                    i,
                    &raw mut rc,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    1 as ::core::ffi::c_int,
                );
                if !pSubOpen_0.is_null() {
                    let mut extra: ::core::ffi::c_int = (((iOfst
                        as ::core::ffi::c_longlong
                        % (*pGroup).szChunk as ::core::ffi::c_longlong)
                        as ::core::ffi::c_int + iAmt) as ::core::ffi::c_uint)
                        .wrapping_sub((*pGroup).szChunk) as ::core::ffi::c_int;
                    if extra < 0 as ::core::ffi::c_int {
                        extra = 0 as ::core::ffi::c_int;
                    }
                    iAmt -= extra;
                    rc = (*(*pSubOpen_0).pMethods)
                        .xRead
                        .expect(
                            "non-null function pointer",
                        )(
                        pSubOpen_0,
                        pBuf,
                        iAmt,
                        iOfst % (*pGroup).szChunk as sqlite3_int64,
                    );
                    if rc != SQLITE_OK {
                        break;
                    }
                    pBuf = (pBuf as *mut ::core::ffi::c_char).offset(iAmt as isize)
                        as *mut ::core::ffi::c_void;
                    iOfst += iAmt as ::core::ffi::c_longlong;
                    iAmt = extra;
                } else {
                    rc = SQLITE_IOERR_READ;
                    break;
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn multiplexWrite(
    mut pConn: *mut sqlite3_file,
    mut pBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut pGroup: *mut multiplexGroup = (*p).pGroup;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pGroup).bEnabled == 0 {
            let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
                pGroup,
                0 as ::core::ffi::c_int,
                &raw mut rc,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                0 as ::core::ffi::c_int,
            );
            if pSubOpen.is_null() {
                rc = SQLITE_IOERR_WRITE;
            } else {
                rc = (*(*pSubOpen).pMethods)
                    .xWrite
                    .expect("non-null function pointer")(pSubOpen, pBuf, iAmt, iOfst);
            }
        } else {
            while rc == SQLITE_OK && iAmt > 0 as ::core::ffi::c_int {
                let mut i: ::core::ffi::c_int = (iOfst as ::core::ffi::c_longlong
                    / (*pGroup).szChunk as ::core::ffi::c_longlong)
                    as ::core::ffi::c_int;
                let mut pSubOpen_0: *mut sqlite3_file = multiplexSubOpen(
                    pGroup,
                    i,
                    &raw mut rc,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    1 as ::core::ffi::c_int,
                );
                if !pSubOpen_0.is_null() {
                    let mut extra: ::core::ffi::c_int = (((iOfst
                        as ::core::ffi::c_longlong
                        % (*pGroup).szChunk as ::core::ffi::c_longlong)
                        as ::core::ffi::c_int + iAmt) as ::core::ffi::c_uint)
                        .wrapping_sub((*pGroup).szChunk) as ::core::ffi::c_int;
                    if extra < 0 as ::core::ffi::c_int {
                        extra = 0 as ::core::ffi::c_int;
                    }
                    iAmt -= extra;
                    rc = (*(*pSubOpen_0).pMethods)
                        .xWrite
                        .expect(
                            "non-null function pointer",
                        )(
                        pSubOpen_0,
                        pBuf,
                        iAmt,
                        iOfst % (*pGroup).szChunk as sqlite3_int64,
                    );
                    pBuf = (pBuf as *mut ::core::ffi::c_char).offset(iAmt as isize)
                        as *const ::core::ffi::c_void;
                    iOfst += iAmt as ::core::ffi::c_longlong;
                    iAmt = extra;
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn multiplexTruncate(
    mut pConn: *mut sqlite3_file,
    mut size: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut pGroup: *mut multiplexGroup = (*p).pGroup;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pGroup).bEnabled == 0 {
            let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
                pGroup,
                0 as ::core::ffi::c_int,
                &raw mut rc,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                0 as ::core::ffi::c_int,
            );
            if pSubOpen.is_null() {
                rc = SQLITE_IOERR_TRUNCATE;
            } else {
                rc = (*(*pSubOpen).pMethods)
                    .xTruncate
                    .expect("non-null function pointer")(pSubOpen, size);
            }
        } else {
            let mut i: ::core::ffi::c_int = 0;
            let mut iBaseGroup: ::core::ffi::c_int = (size as ::core::ffi::c_longlong
                / (*pGroup).szChunk as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            let mut pSubOpen_0: *mut sqlite3_file = ::core::ptr::null_mut::<
                sqlite3_file,
            >();
            let mut pOrigVfs: *mut sqlite3_vfs = gMultiplex.pOrigVfs;
            i = (*pGroup).nReal - 1 as ::core::ffi::c_int;
            while i > iBaseGroup && rc == SQLITE_OK {
                if (*pGroup).bTruncate != 0 {
                    multiplexSubClose(pGroup, i, pOrigVfs);
                } else {
                    pSubOpen_0 = multiplexSubOpen(
                        pGroup,
                        i,
                        &raw mut rc,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        0 as ::core::ffi::c_int,
                    );
                    if !pSubOpen_0.is_null() {
                        rc = (*(*pSubOpen_0).pMethods)
                            .xTruncate
                            .expect(
                                "non-null function pointer",
                            )(pSubOpen_0, 0 as sqlite3_int64);
                    }
                }
                i -= 1;
            }
            if rc == SQLITE_OK {
                pSubOpen_0 = multiplexSubOpen(
                    pGroup,
                    iBaseGroup,
                    &raw mut rc,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    0 as ::core::ffi::c_int,
                );
                if !pSubOpen_0.is_null() {
                    rc = (*(*pSubOpen_0).pMethods)
                        .xTruncate
                        .expect(
                            "non-null function pointer",
                        )(pSubOpen_0, size % (*pGroup).szChunk as sqlite3_int64);
                }
            }
            if rc != 0 {
                rc = SQLITE_IOERR_TRUNCATE;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn multiplexSync(
    mut pConn: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut pGroup: *mut multiplexGroup = (*p).pGroup;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pGroup).nReal {
            let mut pSubOpen: *mut sqlite3_file = (*(*pGroup).aReal.offset(i as isize))
                .p;
            if !pSubOpen.is_null() {
                let mut rc2: ::core::ffi::c_int = (*(*pSubOpen).pMethods)
                    .xSync
                    .expect("non-null function pointer")(pSubOpen, flags);
                if rc2 != SQLITE_OK {
                    rc = rc2;
                }
            }
            i += 1;
        }
        return rc;
    }
}
unsafe extern "C" fn multiplexFileSize(
    mut pConn: *mut sqlite3_file,
    mut pSize: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut pGroup: *mut multiplexGroup = (*p).pGroup;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut i: ::core::ffi::c_int = 0;
        if (*pGroup).bEnabled == 0 {
            let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
                pGroup,
                0 as ::core::ffi::c_int,
                &raw mut rc,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                0 as ::core::ffi::c_int,
            );
            if pSubOpen.is_null() {
                rc = SQLITE_IOERR_FSTAT;
            } else {
                rc = (*(*pSubOpen).pMethods)
                    .xFileSize
                    .expect("non-null function pointer")(pSubOpen, pSize);
            }
        } else {
            *pSize = 0 as sqlite3_int64;
            i = 0 as ::core::ffi::c_int;
            while rc == SQLITE_OK {
                let mut sz: sqlite3_int64 = multiplexSubSize(pGroup, i, &raw mut rc);
                if sz == 0 as ::core::ffi::c_longlong {
                    break;
                }
                *pSize = i as sqlite3_int64 * (*pGroup).szChunk as sqlite3_int64 + sz;
                i += 1;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn multiplexLock(
    mut pConn: *mut sqlite3_file,
    mut lock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
            (*p).pGroup,
            0 as ::core::ffi::c_int,
            &raw mut rc,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if !pSubOpen.is_null() {
            return (*(*pSubOpen).pMethods)
                .xLock
                .expect("non-null function pointer")(pSubOpen, lock);
        }
        return SQLITE_BUSY;
    }
}
unsafe extern "C" fn multiplexUnlock(
    mut pConn: *mut sqlite3_file,
    mut lock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
            (*p).pGroup,
            0 as ::core::ffi::c_int,
            &raw mut rc,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if !pSubOpen.is_null() {
            return (*(*pSubOpen).pMethods)
                .xUnlock
                .expect("non-null function pointer")(pSubOpen, lock);
        }
        return SQLITE_IOERR_UNLOCK;
    }
}
unsafe extern "C" fn multiplexCheckReservedLock(
    mut pConn: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
            (*p).pGroup,
            0 as ::core::ffi::c_int,
            &raw mut rc,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if !pSubOpen.is_null() {
            return (*(*pSubOpen).pMethods)
                .xCheckReservedLock
                .expect("non-null function pointer")(pSubOpen, pResOut);
        }
        return SQLITE_IOERR_CHECKRESERVEDLOCK;
    }
}
unsafe extern "C" fn multiplexFileControl(
    mut pConn: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut pGroup: *mut multiplexGroup = (*p).pGroup;
        let mut rc: ::core::ffi::c_int = SQLITE_ERROR;
        let mut pSubOpen: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        if gMultiplex.isInitialized == 0 {
            return SQLITE_MISUSE;
        }
        let mut c2rust_current_block_46: u64;
        match op {
            MULTIPLEX_CTRL_ENABLE => {
                if !pArg.is_null() {
                    let mut bEnabled: ::core::ffi::c_int = *(pArg
                        as *mut ::core::ffi::c_int);
                    (*pGroup).bEnabled = bEnabled as ::core::ffi::c_uchar;
                    rc = SQLITE_OK;
                }
                c2rust_current_block_46 = 2516253395664191498;
            }
            MULTIPLEX_CTRL_SET_CHUNK_SIZE => {
                if !pArg.is_null() {
                    let mut szChunk: ::core::ffi::c_uint = *(pArg
                        as *mut ::core::ffi::c_uint);
                    if szChunk < 1 as ::core::ffi::c_uint {
                        rc = SQLITE_MISUSE;
                    } else {
                        szChunk = szChunk
                            .wrapping_add(
                                (MAX_PAGE_SIZE - 1 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint,
                            );
                        szChunk
                            &= !(MAX_PAGE_SIZE - 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint;
                        (*pGroup).szChunk = szChunk;
                        rc = SQLITE_OK;
                    }
                }
                c2rust_current_block_46 = 2516253395664191498;
            }
            MULTIPLEX_CTRL_SET_MAX_CHUNKS => {
                rc = SQLITE_OK;
                c2rust_current_block_46 = 2516253395664191498;
            }
            SQLITE_FCNTL_SIZE_HINT | SQLITE_FCNTL_CHUNK_SIZE => {
                rc = SQLITE_OK;
                c2rust_current_block_46 = 2516253395664191498;
            }
            SQLITE_FCNTL_PRAGMA => {
                let mut aFcntl: *mut *mut ::core::ffi::c_char = pArg
                    as *mut *mut ::core::ffi::c_char;
                if !(*aFcntl.offset(1 as ::core::ffi::c_int as isize)).is_null()
                    && sqlite3_strnicmp(
                        *aFcntl.offset(1 as ::core::ffi::c_int as isize),
                        b"multiplex_\0".as_ptr() as *const ::core::ffi::c_char,
                        10 as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int
                {
                    let mut sz: sqlite3_int64 = 0 as sqlite3_int64;
                    multiplexFileSize(pConn, &raw mut sz);
                    if sqlite3_stricmp(
                        *aFcntl.offset(1 as ::core::ffi::c_int as isize),
                        b"multiplex_truncate\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        if !(*aFcntl.offset(2 as ::core::ffi::c_int as isize)).is_null()
                            && *(*aFcntl.offset(2 as ::core::ffi::c_int as isize))
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int != 0
                        {
                            if sqlite3_stricmp(
                                *aFcntl.offset(2 as ::core::ffi::c_int as isize),
                                b"on\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                                || sqlite3_stricmp(
                                    *aFcntl.offset(2 as ::core::ffi::c_int as isize),
                                    b"1\0".as_ptr() as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                            {
                                (*pGroup).bTruncate = 1 as ::core::ffi::c_uchar;
                            } else if sqlite3_stricmp(
                                *aFcntl.offset(2 as ::core::ffi::c_int as isize),
                                b"off\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                                || sqlite3_stricmp(
                                    *aFcntl.offset(2 as ::core::ffi::c_int as isize),
                                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                            {
                                (*pGroup).bTruncate = 0 as ::core::ffi::c_uchar;
                            }
                        }
                        let ref mut c2rust_fresh3 = *aFcntl
                            .offset(0 as ::core::ffi::c_int as isize);
                        *c2rust_fresh3 = sqlite3_mprintf(
                            if (*pGroup).bTruncate as ::core::ffi::c_int != 0 {
                                b"on\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"off\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        );
                        rc = SQLITE_OK;
                        c2rust_current_block_46 = 2516253395664191498;
                    } else if sqlite3_stricmp(
                        *aFcntl.offset(1 as ::core::ffi::c_int as isize),
                        b"multiplex_enabled\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let ref mut c2rust_fresh4 = *aFcntl
                            .offset(0 as ::core::ffi::c_int as isize);
                        *c2rust_fresh4 = sqlite3_mprintf(
                            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                            ((*pGroup).bEnabled as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        );
                        rc = SQLITE_OK;
                        c2rust_current_block_46 = 2516253395664191498;
                    } else if sqlite3_stricmp(
                        *aFcntl.offset(1 as ::core::ffi::c_int as isize),
                        b"multiplex_chunksize\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                        && (*pGroup).bEnabled as ::core::ffi::c_int != 0
                    {
                        let ref mut c2rust_fresh5 = *aFcntl
                            .offset(0 as ::core::ffi::c_int as isize);
                        *c2rust_fresh5 = sqlite3_mprintf(
                            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                            (*pGroup).szChunk,
                        );
                        rc = SQLITE_OK;
                        c2rust_current_block_46 = 2516253395664191498;
                    } else if sqlite3_stricmp(
                        *aFcntl.offset(1 as ::core::ffi::c_int as isize),
                        b"multiplex_filecount\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut ii: ::core::ffi::c_int = 0;
                        ii = 0 as ::core::ffi::c_int;
                        while ii < (*pGroup).nReal {
                            if !(*(*pGroup).aReal.offset(ii as isize)).p.is_null() {
                                n += 1;
                            }
                            ii += 1;
                        }
                        let ref mut c2rust_fresh6 = *aFcntl
                            .offset(0 as ::core::ffi::c_int as isize);
                        *c2rust_fresh6 = sqlite3_mprintf(
                            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                            n,
                        );
                        rc = SQLITE_OK;
                        c2rust_current_block_46 = 2516253395664191498;
                    } else {
                        c2rust_current_block_46 = 10001882110310123864;
                    }
                } else {
                    c2rust_current_block_46 = 10001882110310123864;
                }
            }
            _ => {
                c2rust_current_block_46 = 10001882110310123864;
            }
        }
        match c2rust_current_block_46 {
            10001882110310123864 => {
                pSubOpen = multiplexSubOpen(
                    pGroup,
                    0 as ::core::ffi::c_int,
                    &raw mut rc,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    0 as ::core::ffi::c_int,
                );
                if !pSubOpen.is_null() {
                    rc = (*(*pSubOpen).pMethods)
                        .xFileControl
                        .expect("non-null function pointer")(pSubOpen, op, pArg);
                    if op == SQLITE_FCNTL_VFSNAME && rc == SQLITE_OK {
                        let ref mut c2rust_fresh7 = *(pArg
                            as *mut *mut ::core::ffi::c_char);
                        *c2rust_fresh7 = sqlite3_mprintf(
                            b"multiplex/%z\0".as_ptr() as *const ::core::ffi::c_char,
                            *(pArg as *mut *mut ::core::ffi::c_char),
                        );
                    }
                }
            }
            _ => {}
        }
        return rc;
    }
}
unsafe extern "C" fn multiplexSectorSize(
    mut pConn: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
            (*p).pGroup,
            0 as ::core::ffi::c_int,
            &raw mut rc,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if !pSubOpen.is_null() && (*(*pSubOpen).pMethods).xSectorSize.is_some() {
            return (*(*pSubOpen).pMethods)
                .xSectorSize
                .expect("non-null function pointer")(pSubOpen);
        }
        return DEFAULT_SECTOR_SIZE;
    }
}
unsafe extern "C" fn multiplexDeviceCharacteristics(
    mut pConn: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
            (*p).pGroup,
            0 as ::core::ffi::c_int,
            &raw mut rc,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if !pSubOpen.is_null() {
            return (*(*pSubOpen).pMethods)
                .xDeviceCharacteristics
                .expect("non-null function pointer")(pSubOpen);
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn multiplexShmMap(
    mut pConn: *mut sqlite3_file,
    mut iRegion: ::core::ffi::c_int,
    mut szRegion: ::core::ffi::c_int,
    mut bExtend: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
            (*p).pGroup,
            0 as ::core::ffi::c_int,
            &raw mut rc,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if !pSubOpen.is_null() {
            return (*(*pSubOpen).pMethods)
                .xShmMap
                .expect(
                    "non-null function pointer",
                )(pSubOpen, iRegion, szRegion, bExtend, pp);
        }
        return SQLITE_IOERR;
    }
}
unsafe extern "C" fn multiplexShmLock(
    mut pConn: *mut sqlite3_file,
    mut ofst: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
            (*p).pGroup,
            0 as ::core::ffi::c_int,
            &raw mut rc,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if !pSubOpen.is_null() {
            return (*(*pSubOpen).pMethods)
                .xShmLock
                .expect("non-null function pointer")(pSubOpen, ofst, n, flags);
        }
        return SQLITE_BUSY;
    }
}
unsafe extern "C" fn multiplexShmBarrier(mut pConn: *mut sqlite3_file) {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
            (*p).pGroup,
            0 as ::core::ffi::c_int,
            &raw mut rc,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if !pSubOpen.is_null() {
            (*(*pSubOpen).pMethods)
                .xShmBarrier
                .expect("non-null function pointer")(pSubOpen);
        }
    }
}
unsafe extern "C" fn multiplexShmUnmap(
    mut pConn: *mut sqlite3_file,
    mut deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut multiplexConn = pConn as *mut multiplexConn;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSubOpen: *mut sqlite3_file = multiplexSubOpen(
            (*p).pGroup,
            0 as ::core::ffi::c_int,
            &raw mut rc,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            0 as ::core::ffi::c_int,
        );
        if !pSubOpen.is_null() {
            return (*(*pSubOpen).pMethods)
                .xShmUnmap
                .expect("non-null function pointer")(pSubOpen, deleteFlag);
        }
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_multiplex_initialize(
    mut zOrigVfsName: *const ::core::ffi::c_char,
    mut makeDefault: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pOrigVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        if gMultiplex.isInitialized != 0 {
            return SQLITE_MISUSE;
        }
        pOrigVfs = sqlite3_vfs_find(zOrigVfsName);
        if pOrigVfs.is_null() {
            return SQLITE_ERROR;
        }
        gMultiplex.isInitialized = 1 as ::core::ffi::c_int;
        gMultiplex.pOrigVfs = pOrigVfs;
        gMultiplex.sThisVfs = *pOrigVfs;
        gMultiplex.sThisVfs.szOsFile = (gMultiplex.sThisVfs.szOsFile
            as ::core::ffi::c_ulong)
            .wrapping_add(
                ::core::mem::size_of::<multiplexConn>() as usize as ::core::ffi::c_ulong,
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        gMultiplex.sThisVfs.zName = SQLITE_MULTIPLEX_VFS_NAME.as_ptr();
        gMultiplex.sThisVfs.xOpen = Some(
            multiplexOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    sqlite3_filename,
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sThisVfs.xDelete = Some(
            multiplexDelete
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sThisVfs.xAccess = Some(
            multiplexAccess
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sThisVfs.xFullPathname = Some(
            multiplexFullPathname
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sThisVfs.xDlOpen = Some(
            multiplexDlOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_void,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *const ::core::ffi::c_char,
                ) -> *mut ::core::ffi::c_void,
            >;
        gMultiplex.sThisVfs.xDlError = Some(
            multiplexDlError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> (),
            >;
        gMultiplex.sThisVfs.xDlSym = Some(
            multiplexDlSym
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> Option<unsafe extern "C" fn() -> ()>,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> Option<unsafe extern "C" fn() -> ()>,
            >;
        gMultiplex.sThisVfs.xDlClose = Some(
            multiplexDlClose
                as unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
        )
            as Option<
                unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
            >;
        gMultiplex.sThisVfs.xRandomness = Some(
            multiplexRandomness
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sThisVfs.xSleep = Some(
            multiplexSleep
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sThisVfs.xCurrentTime = Some(
            multiplexCurrentTime
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut ::core::ffi::c_double,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sThisVfs.xGetLastError = Some(
            multiplexGetLastError
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sThisVfs.xCurrentTimeInt64 = Some(
            multiplexCurrentTimeInt64
                as unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_vfs,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV1.iVersion = 1 as ::core::ffi::c_int;
        gMultiplex.sIoMethodsV1.xClose = Some(
            multiplexClose
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>;
        gMultiplex.sIoMethodsV1.xRead = Some(
            multiplexRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV1.xWrite = Some(
            multiplexWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV1.xTruncate = Some(
            multiplexTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV1.xSync = Some(
            multiplexSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV1.xFileSize = Some(
            multiplexFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV1.xLock = Some(
            multiplexLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV1.xUnlock = Some(
            multiplexUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV1.xCheckReservedLock = Some(
            multiplexCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV1.xFileControl = Some(
            multiplexFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV1.xSectorSize = Some(
            multiplexSectorSize
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>;
        gMultiplex.sIoMethodsV1.xDeviceCharacteristics = Some(
            multiplexDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>;
        gMultiplex.sIoMethodsV2 = gMultiplex.sIoMethodsV1;
        gMultiplex.sIoMethodsV2.iVersion = 2 as ::core::ffi::c_int;
        gMultiplex.sIoMethodsV2.xShmMap = Some(
            multiplexShmMap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV2.xShmLock = Some(
            multiplexShmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        gMultiplex.sIoMethodsV2.xShmBarrier = Some(
            multiplexShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
        ) as Option<unsafe extern "C" fn(*mut sqlite3_file) -> ()>;
        gMultiplex.sIoMethodsV2.xShmUnmap = Some(
            multiplexShmUnmap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        sqlite3_vfs_register(&raw mut gMultiplex.sThisVfs, makeDefault);
        sqlite3_auto_extension(
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
                    multiplexFuncInit
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_multiplex_shutdown(
    mut eForce: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if gMultiplex.isInitialized == 0 as ::core::ffi::c_int {
            return SQLITE_MISUSE;
        }
        gMultiplex.isInitialized = 0 as ::core::ffi::c_int;
        sqlite3_vfs_unregister(&raw mut gMultiplex.sThisVfs);
        memset(
            &raw mut gMultiplex as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<C2Rust_Unnamed>() as size_t,
        );
        return rc;
    }
}
unsafe extern "C" fn test_multiplex_initialize(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut makeDefault: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"NAME MAKEDEFAULT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zName = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        if Tcl_GetBooleanFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut makeDefault,
        ) != 0
        {
            return TCL_ERROR;
        }
        if *zName.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
        {
            zName = ::core::ptr::null::<::core::ffi::c_char>();
        }
        rc = sqlite3_multiplex_initialize(zName, makeDefault);
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_multiplex_shutdown(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if objc == 2 as ::core::ffi::c_int
            && strcmp(
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                b"-force\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            objc = 3 as ::core::ffi::c_int;
        }
        if objc != 1 as ::core::ffi::c_int && objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?-force?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = sqlite3_multiplex_shutdown(
            (objc == 2 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_multiplex_control(
    mut cd: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut idx: ::core::ffi::c_int = 0;
        let mut cmdInfo: Tcl_CmdInfo = Tcl_CmdInfo {
            isNativeObjectProc: 0,
            objProc: None,
            objClientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            proc: None,
            clientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            deleteProc: None,
            deleteData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            namespacePtr: ::core::ptr::null_mut::<Tcl_Namespace>(),
        };
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut iValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pArg: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut aSub: [SubCommand; 4] = [
            SubCommand {
                zName: b"enable\0".as_ptr() as *const ::core::ffi::c_char,
                op: MULTIPLEX_CTRL_ENABLE,
                argtype: 1 as ::core::ffi::c_int,
            },
            SubCommand {
                zName: b"chunk_size\0".as_ptr() as *const ::core::ffi::c_char,
                op: MULTIPLEX_CTRL_SET_CHUNK_SIZE,
                argtype: 1 as ::core::ffi::c_int,
            },
            SubCommand {
                zName: b"max_chunks\0".as_ptr() as *const ::core::ffi::c_char,
                op: MULTIPLEX_CTRL_SET_MAX_CHUNKS,
                argtype: 1 as ::core::ffi::c_int,
            },
            SubCommand {
                zName: ::core::ptr::null::<::core::ffi::c_char>(),
                op: 0 as ::core::ffi::c_int,
                argtype: 0 as ::core::ffi::c_int,
            },
        ];
        if objc != 5 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HANDLE DBNAME SUB-COMMAND INT-VALUE\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if 0 as ::core::ffi::c_int
            == Tcl_GetCommandInfo(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut cmdInfo,
            )
        {
            Tcl_AppendResult(
                interp,
                b"expected database handle, got \"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                NULL_0,
            );
            Tcl_AppendResult(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                b"\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL_0,
            );
            return TCL_ERROR;
        } else {
            db = *(cmdInfo.objClientData as *mut *mut sqlite3);
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut aSub as *mut SubCommand as *const ::core::ffi::c_void,
            ::core::mem::size_of::<SubCommand>() as ::core::ffi::c_int,
            b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut idx,
        );
        if rc != TCL_OK {
            return rc;
        }
        match aSub[idx as usize].argtype {
            1 => {
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(4 as ::core::ffi::c_int as isize),
                    &raw mut iValue,
                ) != 0
                {
                    return TCL_ERROR;
                }
                pArg = &raw mut iValue as *mut ::core::ffi::c_void;
            }
            _ => {
                Tcl_WrongNumArgs(
                    interp,
                    4 as ::core::ffi::c_int,
                    objv,
                    b"SUB-COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return TCL_ERROR;
            }
        }
        rc = sqlite3_file_control(
            db,
            Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
            aSub[idx as usize].op,
            pArg,
        );
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return if rc == SQLITE_OK { TCL_OK } else { TCL_ERROR };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitemultiplex_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aCmd: [C2Rust_Unnamed_3; 3] = unsafe {
            [
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_multiplex_initialize\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_multiplex_initialize
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_multiplex_shutdown\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_multiplex_shutdown
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_3 {
                    zName: b"sqlite3_multiplex_control\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_multiplex_control
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
            ]
        };
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_3; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_3>() as usize)
        {
            Tcl_CreateObjCommand(
                interp,
                aCmd[i as usize].zName,
                aCmd[i as usize].xProc,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            i += 1;
        }
        return TCL_OK;
    }
}
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_STATIC: Option<Tcl_FreeProc> = None;
pub const MULTIPLEX_CTRL_ENABLE: ::core::ffi::c_int = 214014 as ::core::ffi::c_int;
pub const MULTIPLEX_CTRL_SET_CHUNK_SIZE: ::core::ffi::c_int = 214015
    as ::core::ffi::c_int;
pub const MULTIPLEX_CTRL_SET_MAX_CHUNKS: ::core::ffi::c_int = 214016
    as ::core::ffi::c_int;
