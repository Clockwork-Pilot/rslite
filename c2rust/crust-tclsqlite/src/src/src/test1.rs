use ::c2rust_bitfields;
use ::libc;
unsafe extern "C" {
    pub type VdbeSorter;
    pub type BtCursor;
    pub type Btree;
    pub type RenameToken;
    pub type TableLock;
    pub type VtabCtx;
    pub type sqlite3_mutex;
    pub type sqlite3_api_routines;
    pub type sqlite3_stmt;
    pub type sqlite3_blob;
    pub type sqlite3_pcache;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Pager;
    pub type Tcl_Channel_;
    pub type Tcl_Command_;
    fn sqlite3_libversion_number() -> ::core::ffi::c_int;
    fn sqlite3_threadsafe() -> ::core::ffi::c_int;
    fn sqlite3_close(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_close_v2(_: *mut sqlite3) -> ::core::ffi::c_int;
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
    fn sqlite3_config(_: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3_db_config(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn sqlite3_extended_result_codes(
        _: *mut sqlite3,
        onoff: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
    fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_interrupt(_: *mut sqlite3);
    fn sqlite3_is_interrupted(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_complete16(sql: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn sqlite3_busy_timeout(
        _: *mut sqlite3,
        ms: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_setlk_timeout(
        _: *mut sqlite3,
        ms: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_get_table(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        pazResult: *mut *mut *mut ::core::ffi::c_char,
        pnRow: *mut ::core::ffi::c_int,
        pnColumn: *mut ::core::ffi::c_int,
        pzErrmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_free_table(result: *mut *mut ::core::ffi::c_char);
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_open(
        filename: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
    ) -> ::core::ffi::c_int;
    fn sqlite3_open16(
        filename: *const ::core::ffi::c_void,
        ppDb: *mut *mut sqlite3,
    ) -> ::core::ffi::c_int;
    fn sqlite3_open_v2(
        filename: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
        flags: ::core::ffi::c_int,
        zVfs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_errcode(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_extended_errcode(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_errmsg16(_: *mut sqlite3) -> *const ::core::ffi::c_void;
    fn sqlite3_errstr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3_error_offset(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_set_errmsg(
        db: *mut sqlite3,
        errcode: ::core::ffi::c_int,
        zErrMsg: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_limit(
        _: *mut sqlite3,
        id: ::core::ffi::c_int,
        newVal: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3_prepare_v3(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        prepFlags: ::core::ffi::c_uint,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_prepare16(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_void,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_prepare16_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_void,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const ::core::ffi::c_char;
    fn sqlite3_expanded_sql(pStmt: *mut sqlite3_stmt) -> *mut ::core::ffi::c_char;
    fn sqlite3_stmt_readonly(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_stmt_isexplain(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_stmt_explain(
        pStmt: *mut sqlite3_stmt,
        eMode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_stmt_busy(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_bind_blob(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        n: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_double(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_null(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text16(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_value(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_pointer(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_zeroblob(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_zeroblob64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_uint64,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_parameter_count(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_bind_parameter_name(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_bind_parameter_index(
        _: *mut sqlite3_stmt,
        zName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_clear_bindings(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_name(
        _: *mut sqlite3_stmt,
        N: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_column_name16(
        _: *mut sqlite3_stmt,
        N: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_column_decltype(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_column_decltype16(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_data_count(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_blob(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_column_double(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
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
    fn sqlite3_column_text16(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_column_value(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *mut sqlite3_value;
    fn sqlite3_column_bytes(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_bytes16(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_type(
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
    fn sqlite3_create_function16(
        db: *mut sqlite3,
        zFunctionName: *const ::core::ffi::c_void,
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
    fn sqlite3_aggregate_count(_: *mut sqlite3_context) -> ::core::ffi::c_int;
    fn sqlite3_expired(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_transfer_bindings(
        _: *mut sqlite3_stmt,
        _: *mut sqlite3_stmt,
    ) -> ::core::ffi::c_int;
    fn sqlite3_global_recover() -> ::core::ffi::c_int;
    fn sqlite3_thread_cleanup();
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_text16(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_text16le(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_text16be(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_bytes16(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_aggregate_context(
        _: *mut sqlite3_context,
        nBytes: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error16(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_null(_: *mut sqlite3_context);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_text16(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_text16le(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_text16be(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
    fn sqlite3_result_pointer(
        _: *mut sqlite3_context,
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
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
    fn sqlite3_create_collation_v2(
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
        xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_create_collation16(
        _: *mut sqlite3,
        zName: *const ::core::ffi::c_void,
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
    fn sqlite3_collation_needed16(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut sqlite3,
                ::core::ffi::c_int,
                *const ::core::ffi::c_void,
            ) -> (),
        >,
    ) -> ::core::ffi::c_int;
    fn sqlite3_sleep(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut sqlite3_temp_directory: *mut ::core::ffi::c_char;
    static mut sqlite3_data_directory: *mut ::core::ffi::c_char;
    fn sqlite3_get_autocommit(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_db_handle(_: *mut sqlite3_stmt) -> *mut sqlite3;
    fn sqlite3_db_filename(
        db: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
    ) -> sqlite3_filename;
    fn sqlite3_db_readonly(
        db: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_txn_state(
        _: *mut sqlite3,
        zSchema: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_next_stmt(
        pDb: *mut sqlite3,
        pStmt: *mut sqlite3_stmt,
    ) -> *mut sqlite3_stmt;
    fn sqlite3_autovacuum_pages(
        db: *mut sqlite3,
        _: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ::core::ffi::c_uint,
                ::core::ffi::c_uint,
                ::core::ffi::c_uint,
            ) -> ::core::ffi::c_uint,
        >,
        _: *mut ::core::ffi::c_void,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_enable_shared_cache(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_db_release_memory(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_soft_heap_limit64(N: sqlite3_int64) -> sqlite3_int64;
    fn sqlite3_hard_heap_limit64(N: sqlite3_int64) -> sqlite3_int64;
    fn sqlite3_table_column_metadata(
        db: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        zTableName: *const ::core::ffi::c_char,
        zColumnName: *const ::core::ffi::c_char,
        pzDataType: *mut *const ::core::ffi::c_char,
        pzCollSeq: *mut *const ::core::ffi::c_char,
        pNotNull: *mut ::core::ffi::c_int,
        pPrimaryKey: *mut ::core::ffi::c_int,
        pAutoinc: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_load_extension(
        db: *mut sqlite3,
        zFile: *const ::core::ffi::c_char,
        zProc: *const ::core::ffi::c_char,
        pzErrMsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_enable_load_extension(
        db: *mut sqlite3,
        onoff: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
        p: *const sqlite3_module,
        pClientData: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_drop_modules(
        db: *mut sqlite3,
        azKeep: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_blob_reopen(_: *mut sqlite3_blob, _: sqlite3_int64) -> ::core::ffi::c_int;
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        op: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_test_control(op: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3_stmt_status(
        _: *mut sqlite3_stmt,
        op: ::core::ffi::c_int,
        resetFlg: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strglob(
        zGlob: *const ::core::ffi::c_char,
        zStr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_wal_autocheckpoint(
        db: *mut sqlite3,
        N: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_wal_checkpoint(
        db: *mut sqlite3,
        zDb: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_wal_checkpoint_v2(
        db: *mut sqlite3,
        zDb: *const ::core::ffi::c_char,
        eMode: ::core::ffi::c_int,
        pnLog: *mut ::core::ffi::c_int,
        pnCkpt: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_db_cacheflush(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_preupdate_old(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3_preupdate_new(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3_system_errno(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_carray_bind(
        pStmt: *mut sqlite3_stmt,
        i: ::core::ffi::c_int,
        aData: *mut ::core::ffi::c_void,
        nData: ::core::ffi::c_int,
        mFlags: ::core::ffi::c_int,
        xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn sqlite3PcacheStats(
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    );
    fn sqlite3PagerStats(_: *mut Pager) -> *mut ::core::ffi::c_int;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3_delete_database(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn sqlite3StrICmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Strlen30(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3Atoi64(
        _: *const ::core::ffi::c_char,
        _: *mut i64_0,
        _: ::core::ffi::c_int,
        _: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3ErrStr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3ValueText(_: *mut sqlite3_value, _: u8_0) -> *const ::core::ffi::c_void;
    fn sqlite3ValueSetStr(
        _: *mut sqlite3_value,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: u8_0,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3ValueFree(_: *mut sqlite3_value);
    fn sqlite3ValueNew(_: *mut sqlite3) -> *mut sqlite3_value;
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
    fn sqlite3DbstatRegister(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn Tcl_Alloc(size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char;
    fn Tcl_Free(ptr: *mut ::core::ffi::c_char);
    fn Tcl_DuplicateObj(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_GetBooleanFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetByteArrayFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_uchar;
    fn Tcl_GetDouble(
        interp: *mut Tcl_Interp,
        src: *const ::core::ffi::c_char,
        doublePtr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetDoubleFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        doublePtr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetInt(
        interp: *mut Tcl_Interp,
        src: *const ::core::ffi::c_char,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetStringFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_ListObjGetElements(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objcPtr: *mut ::core::ffi::c_int,
        objvPtr: *mut *mut *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewByteArrayObj(
        bytes: *const ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_NewDoubleObj(doubleValue: ::core::ffi::c_double) -> *mut Tcl_Obj;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_NewStringObj(
        bytes: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_AppendElement(interp: *mut Tcl_Interp, element: *const ::core::ffi::c_char);
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_BackgroundError(interp: *mut Tcl_Interp);
    fn Tcl_CreateCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_CmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_DStringAppend(
        dsPtr: *mut Tcl_DString,
        bytes: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_DStringAppendElement(
        dsPtr: *mut Tcl_DString,
        element: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_DStringFree(dsPtr: *mut Tcl_DString);
    fn Tcl_DStringInit(dsPtr: *mut Tcl_DString);
    fn Tcl_Eval(
        interp: *mut Tcl_Interp,
        script: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_Flush(chan: Tcl_Channel) -> ::core::ffi::c_int;
    fn Tcl_GetChannel(
        interp: *mut Tcl_Interp,
        chanName: *const ::core::ffi::c_char,
        modePtr: *mut ::core::ffi::c_int,
    ) -> Tcl_Channel;
    fn Tcl_GetChannelInstanceData(chan: Tcl_Channel) -> ClientData;
    fn Tcl_GetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        infoPtr: *mut Tcl_CmdInfo,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetObjResult(interp: *mut Tcl_Interp) -> *mut Tcl_Obj;
    fn Tcl_GetStringResult(interp: *mut Tcl_Interp) -> *const ::core::ffi::c_char;
    fn Tcl_LinkVar(
        interp: *mut Tcl_Interp,
        varName: *const ::core::ffi::c_char,
        addr: *mut ::core::ffi::c_char,
        type_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_ObjSetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        newValuePtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_ResetResult(interp: *mut Tcl_Interp);
    fn Tcl_SetResult(
        interp: *mut Tcl_Interp,
        result: *mut ::core::ffi::c_char,
        freeProc: Option<Tcl_FreeProc>,
    );
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_EvalObjEx(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetIndexFromObjStruct(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        tablePtr: *const ::core::ffi::c_void,
        offset: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        indexPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetVar2Ex(
        interp: *mut Tcl_Interp,
        part1: *const ::core::ffi::c_char,
        part2: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn Tcl_GetWideIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        widePtr: *mut Tcl_WideInt,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
    fn Tcl_Seek(
        chan: Tcl_Channel,
        offset: Tcl_WideInt,
        mode: ::core::ffi::c_int,
    ) -> Tcl_WideInt;
    fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3 {
    pub pVfs: *mut sqlite3_vfs,
    pub pVdbe: *mut Vdbe,
    pub pDfltColl: *mut CollSeq,
    pub mutex: *mut sqlite3_mutex,
    pub aDb: *mut Db,
    pub nDb: ::core::ffi::c_int,
    pub mDbFlags: u32_0,
    pub flags: u64_0,
    pub lastRowid: i64_0,
    pub szMmap: i64_0,
    pub nSchemaLock: u32_0,
    pub openFlags: ::core::ffi::c_uint,
    pub errCode: ::core::ffi::c_int,
    pub errByteOffset: ::core::ffi::c_int,
    pub errMask: ::core::ffi::c_int,
    pub iSysErrno: ::core::ffi::c_int,
    pub dbOptFlags: u32_0,
    pub enc: u8_0,
    pub autoCommit: u8_0,
    pub temp_store: u8_0,
    pub mallocFailed: u8_0,
    pub bBenignMalloc: u8_0,
    pub dfltLockMode: u8_0,
    pub nextAutovac: ::core::ffi::c_schar,
    pub suppressErr: u8_0,
    pub vtabOnConflict: u8_0,
    pub isTransactionSavepoint: u8_0,
    pub mTrace: u8_0,
    pub noSharedCache: u8_0,
    pub nSqlExec: u8_0,
    pub eOpenState: u8_0,
    pub nextPagesize: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub nTotalChange: i64_0,
    pub aLimit: [::core::ffi::c_int; 12],
    pub nMaxSorterMmap: ::core::ffi::c_int,
    pub init: sqlite3InitInfo,
    pub nVdbeActive: ::core::ffi::c_int,
    pub nVdbeRead: ::core::ffi::c_int,
    pub nVdbeWrite: ::core::ffi::c_int,
    pub nVdbeExec: ::core::ffi::c_int,
    pub nVDestroy: ::core::ffi::c_int,
    pub nExtension: ::core::ffi::c_int,
    pub aExtension: *mut *mut ::core::ffi::c_void,
    pub trace: C2Rust_Unnamed_25,
    pub pTraceArg: *mut ::core::ffi::c_void,
    pub xProfile: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            u64_0,
        ) -> (),
    >,
    pub pProfileArg: *mut ::core::ffi::c_void,
    pub pCommitArg: *mut ::core::ffi::c_void,
    pub xCommitCallback: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub pRollbackArg: *mut ::core::ffi::c_void,
    pub xRollbackCallback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pUpdateArg: *mut ::core::ffi::c_void,
    pub xUpdateCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            sqlite_int64,
        ) -> (),
    >,
    pub pAutovacPagesArg: *mut ::core::ffi::c_void,
    pub xAutovacDestr: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xAutovacPages: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            u32_0,
            u32_0,
            u32_0,
        ) -> ::core::ffi::c_uint,
    >,
    pub pParse: *mut Parse,
    pub pPreUpdateArg: *mut ::core::ffi::c_void,
    pub xPreUpdateCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            sqlite3_int64,
            sqlite3_int64,
        ) -> (),
    >,
    pub pPreUpdate: *mut PreUpdate,
    pub xWalCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pWalArg: *mut ::core::ffi::c_void,
    pub xCollNeeded: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub xCollNeeded16: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> (),
    >,
    pub pCollNeededArg: *mut ::core::ffi::c_void,
    pub pErr: *mut sqlite3_value,
    pub u1: C2Rust_Unnamed_22,
    pub lookaside: Lookaside,
    pub xAuth: sqlite3_xauth,
    pub pAuthArg: *mut ::core::ffi::c_void,
    pub xProgress: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub pProgressArg: *mut ::core::ffi::c_void,
    pub nProgressOps: ::core::ffi::c_uint,
    pub nVTrans: ::core::ffi::c_int,
    pub aModule: Hash,
    pub pVtabCtx: *mut VtabCtx,
    pub aVTrans: *mut *mut VTable,
    pub pDisconnect: *mut VTable,
    pub aFunc: Hash,
    pub aCollSeq: Hash,
    pub busyHandler: BusyHandler,
    pub aDbStatic: [Db; 2],
    pub pSavepoint: *mut Savepoint,
    pub nAnalysisLimit: ::core::ffi::c_int,
    pub busyTimeout: ::core::ffi::c_int,
    pub nSavepoint: ::core::ffi::c_int,
    pub nStatement: ::core::ffi::c_int,
    pub nDeferredCons: i64_0,
    pub nDeferredImmCons: i64_0,
    pub pnBytesFreed: *mut ::core::ffi::c_int,
    pub pDbData: *mut DbClientData,
    pub nSpill: u64_0,
}
pub type u64_0 = sqlite_uint64;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbClientData {
    pub pNext: *mut DbClientData,
    pub pData: *mut ::core::ffi::c_void,
    pub xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub zName: [::core::ffi::c_char; 0],
}
pub type i64_0 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Savepoint {
    pub zName: *mut ::core::ffi::c_char,
    pub nDeferredCons: i64_0,
    pub nDeferredImmCons: i64_0,
    pub pNext: *mut Savepoint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Db {
    pub zDbSName: *mut ::core::ffi::c_char,
    pub pBt: *mut Btree,
    pub safety_level: u8_0,
    pub bSyncSet: u8_0,
    pub pSchema: *mut Schema,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Schema {
    pub schema_cookie: ::core::ffi::c_int,
    pub iGeneration: ::core::ffi::c_int,
    pub tblHash: Hash,
    pub idxHash: Hash,
    pub trigHash: Hash,
    pub fkeyHash: Hash,
    pub pSeqTab: *mut Table,
    pub file_format: u8_0,
    pub enc: u8_0,
    pub schemaFlags: u16_0,
    pub cache_size: ::core::ffi::c_int,
}
pub type u16_0 = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = u16;
pub type u8_0 = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub zName: *mut ::core::ffi::c_char,
    pub aCol: *mut Column,
    pub pIndex: *mut Index,
    pub zColAff: *mut ::core::ffi::c_char,
    pub pCheck: *mut ExprList,
    pub tnum: Pgno,
    pub nTabRef: u32_0,
    pub tabFlags: u32_0,
    pub iPKey: i16_0,
    pub nCol: i16_0,
    pub nNVCol: i16_0,
    pub nRowLogEst: LogEst,
    pub szTabRow: LogEst,
    pub keyConf: u8_0,
    pub eTabType: u8_0,
    pub u: C2Rust_Unnamed_18,
    pub pTrigger: *mut Trigger,
    pub pSchema: *mut Schema,
    pub aHx: [u8_0; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Trigger {
    pub zName: *mut ::core::ffi::c_char,
    pub table: *mut ::core::ffi::c_char,
    pub op: u8_0,
    pub tr_tm: u8_0,
    pub bReturning: u8_0,
    pub pWhen: *mut Expr,
    pub pColumns: *mut IdList,
    pub pSchema: *mut Schema,
    pub pTabSchema: *mut Schema,
    pub step_list: *mut TriggerStep,
    pub pNext: *mut Trigger,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerStep {
    pub op: u8_0,
    pub orconf: u8_0,
    pub pTrig: *mut Trigger,
    pub pSelect: *mut Select,
    pub zTarget: *mut ::core::ffi::c_char,
    pub pFrom: *mut SrcList,
    pub pWhere: *mut Expr,
    pub pExprList: *mut ExprList,
    pub pIdList: *mut IdList,
    pub pUpsert: *mut Upsert,
    pub zSpan: *mut ::core::ffi::c_char,
    pub pNext: *mut TriggerStep,
    pub pLast: *mut TriggerStep,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upsert {
    pub pUpsertTarget: *mut ExprList,
    pub pUpsertTargetWhere: *mut Expr,
    pub pUpsertSet: *mut ExprList,
    pub pUpsertWhere: *mut Expr,
    pub pNextUpsert: *mut Upsert,
    pub isDoUpdate: u8_0,
    pub isDup: u8_0,
    pub pToFree: *mut ::core::ffi::c_void,
    pub pUpsertIdx: *mut Index,
    pub pUpsertSrc: *mut SrcList,
    pub regData: ::core::ffi::c_int,
    pub iDataCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SrcList {
    pub nSrc: ::core::ffi::c_int,
    pub nAlloc: u32_0,
    pub a: [SrcItem; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SrcItem {
    pub zName: *mut ::core::ffi::c_char,
    pub zAlias: *mut ::core::ffi::c_char,
    pub pSTab: *mut Table,
    pub fg: C2Rust_Unnamed_17,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: Bitmask,
    pub u1: C2Rust_Unnamed_16,
    pub u2: C2Rust_Unnamed_15,
    pub u3: C2Rust_Unnamed_14,
    pub u4: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub pSchema: *mut Schema,
    pub zDatabase: *mut ::core::ffi::c_char,
    pub pSubq: *mut Subquery,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Subquery {
    pub pSelect: *mut Select,
    pub addrFillSub: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
    pub regResult: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Select {
    pub op: u8_0,
    pub nSelectRow: LogEst,
    pub selFlags: u32_0,
    pub iLimit: ::core::ffi::c_int,
    pub iOffset: ::core::ffi::c_int,
    pub selId: u32_0,
    pub addrOpenEphm: [::core::ffi::c_int; 2],
    pub pEList: *mut ExprList,
    pub pSrc: *mut SrcList,
    pub pWhere: *mut Expr,
    pub pGroupBy: *mut ExprList,
    pub pHaving: *mut Expr,
    pub pOrderBy: *mut ExprList,
    pub pPrior: *mut Select,
    pub pNext: *mut Select,
    pub pLimit: *mut Expr,
    pub pWith: *mut With,
    pub pWin: *mut Window,
    pub pWinDefn: *mut Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Window {
    pub zName: *mut ::core::ffi::c_char,
    pub zBase: *mut ::core::ffi::c_char,
    pub pPartition: *mut ExprList,
    pub pOrderBy: *mut ExprList,
    pub eFrmType: u8_0,
    pub eStart: u8_0,
    pub eEnd: u8_0,
    pub bImplicitFrame: u8_0,
    pub eExclude: u8_0,
    pub pStart: *mut Expr,
    pub pEnd: *mut Expr,
    pub ppThis: *mut *mut Window,
    pub pNextWin: *mut Window,
    pub pFilter: *mut Expr,
    pub pWFunc: *mut FuncDef,
    pub iEphCsr: ::core::ffi::c_int,
    pub regAccum: ::core::ffi::c_int,
    pub regResult: ::core::ffi::c_int,
    pub csrApp: ::core::ffi::c_int,
    pub regApp: ::core::ffi::c_int,
    pub regPart: ::core::ffi::c_int,
    pub pOwner: *mut Expr,
    pub nBufferCol: ::core::ffi::c_int,
    pub iArgCol: ::core::ffi::c_int,
    pub regOne: ::core::ffi::c_int,
    pub regStartRowid: ::core::ffi::c_int,
    pub regEndRowid: ::core::ffi::c_int,
    pub bExprArgs: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr {
    pub op: u8_0,
    pub affExpr: ::core::ffi::c_char,
    pub op2: u8_0,
    pub flags: u32_0,
    pub u: C2Rust_Unnamed_13,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: C2Rust_Unnamed_12,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_0,
    pub w: C2Rust_Unnamed_11,
    pub pAggInfo: *mut AggInfo,
    pub y: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub pTab: *mut Table,
    pub pWin: *mut Window,
    pub nReg: ::core::ffi::c_int,
    pub sub: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub iAddr: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo {
    pub directMode: u8_0,
    pub useSortingIdx: u8_0,
    pub nSortingColumn: u32_0,
    pub sortingIdx: ::core::ffi::c_int,
    pub sortingIdxPTab: ::core::ffi::c_int,
    pub iFirstReg: ::core::ffi::c_int,
    pub pGroupBy: *mut ExprList,
    pub aCol: *mut AggInfo_col,
    pub nColumn: ::core::ffi::c_int,
    pub nAccumulator: ::core::ffi::c_int,
    pub aFunc: *mut AggInfo_func,
    pub nFunc: ::core::ffi::c_int,
    pub selId: u32_0,
}
pub type u32_0 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo_func {
    pub pFExpr: *mut Expr,
    pub pFunc: *mut FuncDef,
    pub iDistinct: ::core::ffi::c_int,
    pub iDistAddr: ::core::ffi::c_int,
    pub iOBTab: ::core::ffi::c_int,
    pub bOBPayload: u8_0,
    pub bOBUnique: u8_0,
    pub bUseSubtype: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncDef {
    pub nArg: i16_0,
    pub funcFlags: u32_0,
    pub pUserData: *mut ::core::ffi::c_void,
    pub pNext: *mut FuncDef,
    pub xSFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    pub xFinalize: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    pub xValue: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    pub xInverse: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    pub zName: *const ::core::ffi::c_char,
    pub u: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub pHash: *mut FuncDef,
    pub pDestructor: *mut FuncDestructor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncDestructor {
    pub nRef: ::core::ffi::c_int,
    pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pUserData: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_value {
    pub u: MemValue,
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub flags: u16_0,
    pub enc: u8_0,
    pub eSubtype: u8_0,
    pub db: *mut sqlite3,
    pub szMalloc: ::core::ffi::c_int,
    pub uTemp: u32_0,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union MemValue {
    pub r: ::core::ffi::c_double,
    pub i: i64_0,
    pub nZero: ::core::ffi::c_int,
    pub zPType: *const ::core::ffi::c_char,
    pub pDef: *mut FuncDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_context {
    pub pOut: *mut Mem,
    pub pFunc: *mut FuncDef,
    pub pMem: *mut Mem,
    pub pVdbe: *mut Vdbe,
    pub iOp: ::core::ffi::c_int,
    pub isError: ::core::ffi::c_int,
    pub enc: u8_0,
    pub skipFlag: u8_0,
    pub argc: u16_0,
    pub argv: [*mut sqlite3_value; 0],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Vdbe {
    pub db: *mut sqlite3,
    pub ppVPrev: *mut *mut Vdbe,
    pub pVNext: *mut Vdbe,
    pub pParse: *mut Parse,
    pub nVar: ynVar,
    pub nMem: ::core::ffi::c_int,
    pub nCursor: ::core::ffi::c_int,
    pub cacheCtr: u32_0,
    pub pc: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub iStatement: ::core::ffi::c_int,
    pub iCurrentTime: i64_0,
    pub nFkConstraint: i64_0,
    pub nStmtDefCons: i64_0,
    pub nStmtDefImmCons: i64_0,
    pub aMem: *mut Mem,
    pub apArg: *mut *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aVar: *mut Mem,
    pub aOp: *mut Op,
    pub nOp: ::core::ffi::c_int,
    pub nOpAlloc: ::core::ffi::c_int,
    pub aColName: *mut Mem,
    pub pResultRow: *mut Mem,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub pVList: *mut VList,
    pub startTime: i64_0,
    pub nResColumn: u16_0,
    pub nResAlloc: u16_0,
    pub errorAction: u8_0,
    pub minWriteFileFormat: u8_0,
    pub prepFlags: u8_0,
    pub eVdbeState: u8_0,
    #[bitfield(name = "expired", ty = "bft", bits = "0..=1")]
    #[bitfield(name = "explain", ty = "bft", bits = "2..=3")]
    #[bitfield(name = "changeCntOn", ty = "bft", bits = "4..=4")]
    #[bitfield(name = "usesStmtJournal", ty = "bft", bits = "5..=5")]
    #[bitfield(name = "readOnly", ty = "bft", bits = "6..=6")]
    #[bitfield(name = "bIsReader", ty = "bft", bits = "7..=7")]
    #[bitfield(name = "haveEqpOps", ty = "bft", bits = "8..=8")]
    pub expired_explain_changeCntOn_usesStmtJournal_readOnly_bIsReader_haveEqpOps: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub btreeMask: yDbMask,
    pub lockMask: yDbMask,
    pub aCounter: [u32_0; 9],
    pub zSql: *mut ::core::ffi::c_char,
    pub pFree: *mut ::core::ffi::c_void,
    pub pFrame: *mut VdbeFrame,
    pub pDelFrame: *mut VdbeFrame,
    pub nFrame: ::core::ffi::c_int,
    pub expmask: u32_0,
    pub pProgram: *mut SubProgram,
    pub pAuxData: *mut AuxData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuxData {
    pub iAuxOp: ::core::ffi::c_int,
    pub iAuxArg: ::core::ffi::c_int,
    pub pAux: *mut ::core::ffi::c_void,
    pub xDeleteAux: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pNextAux: *mut AuxData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubProgram {
    pub aOp: *mut VdbeOp,
    pub nOp: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub nCsr: ::core::ffi::c_int,
    pub aOnce: *mut u8_0,
    pub token: *mut ::core::ffi::c_void,
    pub pNext: *mut SubProgram,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeOp {
    pub opcode: u8_0,
    pub p4type: ::core::ffi::c_schar,
    pub p5: u16_0,
    pub p1: ::core::ffi::c_int,
    pub p2: ::core::ffi::c_int,
    pub p3: ::core::ffi::c_int,
    pub p4: p4union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union p4union {
    pub i: ::core::ffi::c_int,
    pub p: *mut ::core::ffi::c_void,
    pub z: *mut ::core::ffi::c_char,
    pub pI64: *mut i64_0,
    pub pReal: *mut ::core::ffi::c_double,
    pub pFunc: *mut FuncDef,
    pub pCtx: *mut sqlite3_context,
    pub pColl: *mut CollSeq,
    pub pMem: *mut Mem,
    pub pVtab: *mut VTable,
    pub pKeyInfo: *mut KeyInfo,
    pub ai: *mut u32_0,
    pub pProgram: *mut SubProgram,
    pub pTab: *mut Table,
    pub pSubrtnSig: *mut SubrtnSig,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubrtnSig {
    pub selId: ::core::ffi::c_int,
    pub bComplete: u8_0,
    pub zAff: *mut ::core::ffi::c_char,
    pub iTable: ::core::ffi::c_int,
    pub iAddr: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyInfo {
    pub nRef: u32_0,
    pub enc: u8_0,
    pub nKeyField: u16_0,
    pub nAllField: u16_0,
    pub db: *mut sqlite3,
    pub aSortFlags: *mut u8_0,
    pub aColl: [*mut CollSeq; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollSeq {
    pub zName: *mut ::core::ffi::c_char,
    pub enc: u8_0,
    pub pUser: *mut ::core::ffi::c_void,
    pub xCmp: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VTable {
    pub db: *mut sqlite3,
    pub pMod: *mut Module,
    pub pVtab: *mut sqlite3_vtab,
    pub nRef: ::core::ffi::c_int,
    pub bConstraint: u8_0,
    pub bAllSchemas: u8_0,
    pub eVtabRisk: u8_0,
    pub iSavepoint: ::core::ffi::c_int,
    pub pNext: *mut VTable,
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
pub type sqlite3_int64 = sqlite_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
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
pub type sqlite3_uint64 = sqlite_uint64;
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
pub struct Module {
    pub pModule: *const sqlite3_module,
    pub zName: *const ::core::ffi::c_char,
    pub nRefModule: ::core::ffi::c_int,
    pub pAux: *mut ::core::ffi::c_void,
    pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pEpoTab: *mut Table,
}
pub type Mem = sqlite3_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeFrame {
    pub v: *mut Vdbe,
    pub pParent: *mut VdbeFrame,
    pub aOp: *mut Op,
    pub aMem: *mut Mem,
    pub apCsr: *mut *mut VdbeCursor,
    pub aOnce: *mut u8_0,
    pub token: *mut ::core::ffi::c_void,
    pub lastRowid: i64_0,
    pub pAuxData: *mut AuxData,
    pub nCursor: ::core::ffi::c_int,
    pub pc: ::core::ffi::c_int,
    pub nOp: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub nChildMem: ::core::ffi::c_int,
    pub nChildCsr: ::core::ffi::c_int,
    pub nChange: i64_0,
    pub nDbChange: i64_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VdbeCursor {
    pub eCurType: u8_0,
    pub iDb: i8_0,
    pub nullRow: u8_0,
    pub deferredMoveto: u8_0,
    pub isTable: u8_0,
    #[bitfield(name = "isEphemeral", ty = "Bool", bits = "0..=0")]
    #[bitfield(name = "useRandomRowid", ty = "Bool", bits = "1..=1")]
    #[bitfield(name = "isOrdered", ty = "Bool", bits = "2..=2")]
    #[bitfield(name = "noReuse", ty = "Bool", bits = "3..=3")]
    #[bitfield(name = "colCache", ty = "Bool", bits = "4..=4")]
    pub isEphemeral_useRandomRowid_isOrdered_noReuse_colCache: [u8; 1],
    pub seekHit: u16_0,
    pub ub: C2Rust_Unnamed_4,
    pub seqCount: i64_0,
    pub cacheStatus: u32_0,
    pub seekResult: ::core::ffi::c_int,
    pub pAltCursor: *mut VdbeCursor,
    pub uc: C2Rust_Unnamed_3,
    pub pKeyInfo: *mut KeyInfo,
    pub iHdrOffset: u32_0,
    pub pgnoRoot: Pgno,
    pub nField: i16_0,
    pub nHdrParsed: u16_0,
    pub movetoTarget: i64_0,
    pub aOffset: *mut u32_0,
    pub aRow: *const u8_0,
    pub payloadSize: u32_0,
    pub szRow: u32_0,
    pub pCache: *mut VdbeTxtBlbCache,
    pub aType: [u32_0; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VdbeTxtBlbCache {
    pub pCValue: *mut ::core::ffi::c_char,
    pub iOffset: i64_0,
    pub iCol: ::core::ffi::c_int,
    pub cacheStatus: u32_0,
    pub colCacheCtr: u32_0,
}
pub type i16_0 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = i16;
pub type Pgno = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub pCursor: *mut BtCursor,
    pub pVCur: *mut sqlite3_vtab_cursor,
    pub pSorter: *mut VdbeSorter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub pBtx: *mut Btree,
    pub aAltMap: *mut u32_0,
}
pub type Bool = ::core::ffi::c_uint;
pub type i8_0 = int8_t;
pub type int8_t = __int8_t;
pub type __int8_t = i8;
pub type Op = VdbeOp;
pub type yDbMask = ::core::ffi::c_uint;
pub type bft = ::core::ffi::c_uint;
pub type VList = ::core::ffi::c_int;
pub type ynVar = i16_0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Parse {
    pub db: *mut sqlite3,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub pVdbe: *mut Vdbe,
    pub rc: ::core::ffi::c_int,
    pub nQueryLoop: LogEst,
    pub nested: u8_0,
    pub nTempReg: u8_0,
    pub isMultiWrite: u8_0,
    pub mayAbort: u8_0,
    pub hasCompound: u8_0,
    pub disableLookaside: u8_0,
    pub prepFlags: u8_0,
    pub withinRJSubrtn: u8_0,
    pub bHasExists: u8_0,
    pub mSubrtnSig: u8_0,
    pub eTriggerOp: u8_0,
    pub bReturning: u8_0,
    pub eOrconf: u8_0,
    pub disableTriggers: u8_0,
    #[bitfield(name = "colNamesSet", ty = "bft", bits = "0..=0")]
    #[bitfield(name = "bHasWith", ty = "bft", bits = "1..=1")]
    #[bitfield(name = "okConstFactor", ty = "bft", bits = "2..=2")]
    #[bitfield(name = "checkSchema", ty = "bft", bits = "3..=3")]
    pub colNamesSet_bHasWith_okConstFactor_checkSchema: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub nRangeReg: ::core::ffi::c_int,
    pub iRangeReg: ::core::ffi::c_int,
    pub nErr: ::core::ffi::c_int,
    pub nTab: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub szOpAlloc: ::core::ffi::c_int,
    pub iSelfTab: ::core::ffi::c_int,
    pub nLabel: ::core::ffi::c_int,
    pub nLabelAlloc: ::core::ffi::c_int,
    pub aLabel: *mut ::core::ffi::c_int,
    pub pConstExpr: *mut ExprList,
    pub pIdxEpr: *mut IndexedExpr,
    pub pIdxPartExpr: *mut IndexedExpr,
    pub writeMask: yDbMask,
    pub cookieMask: yDbMask,
    pub nMaxArg: ::core::ffi::c_int,
    pub nSelect: ::core::ffi::c_int,
    pub nProgressSteps: u32_0,
    pub nTableLock: ::core::ffi::c_int,
    pub aTableLock: *mut TableLock,
    pub pAinc: *mut AutoincInfo,
    pub pToplevel: *mut Parse,
    pub pTriggerTab: *mut Table,
    pub pTriggerPrg: *mut TriggerPrg,
    pub pCleanup: *mut ParseCleanup,
    pub aTempReg: [::core::ffi::c_int; 8],
    pub pOuterParse: *mut Parse,
    pub sNameToken: Token,
    pub oldmask: u32_0,
    pub newmask: u32_0,
    pub u1: C2Rust_Unnamed_8,
    pub sLastToken: Token,
    pub nVar: ynVar,
    pub iPkSortOrder: u8_0,
    pub explain: u8_0,
    pub eParseMode: u8_0,
    pub nVtabLock: ::core::ffi::c_int,
    pub nHeight: ::core::ffi::c_int,
    pub addrExplain: ::core::ffi::c_int,
    pub pVList: *mut VList,
    pub pReprepare: *mut Vdbe,
    pub zTail: *const ::core::ffi::c_char,
    pub pNewTable: *mut Table,
    pub pNewIndex: *mut Index,
    pub pNewTrigger: *mut Trigger,
    pub zAuthContext: *const ::core::ffi::c_char,
    pub sArg: Token,
    pub apVtabLock: *mut *mut Table,
    pub pWith: *mut With,
    pub pRename: *mut RenameToken,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct With {
    pub nCte: ::core::ffi::c_int,
    pub bView: ::core::ffi::c_int,
    pub pOuter: *mut With,
    pub a: [Cte; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cte {
    pub zName: *mut ::core::ffi::c_char,
    pub pCols: *mut ExprList,
    pub pSelect: *mut Select,
    pub zCteErr: *const ::core::ffi::c_char,
    pub pUse: *mut CteUse,
    pub eM10d: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CteUse {
    pub nUse: ::core::ffi::c_int,
    pub addrM9e: ::core::ffi::c_int,
    pub regRtn: ::core::ffi::c_int,
    pub iCur: ::core::ffi::c_int,
    pub nRowEst: LogEst,
    pub eM10d: u8_0,
}
pub type LogEst = int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprList {
    pub nExpr: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
    pub a: [ExprList_item; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprList_item {
    pub pExpr: *mut Expr,
    pub zEName: *mut ::core::ffi::c_char,
    pub fg: C2Rust_Unnamed_7,
    pub u: C2Rust_Unnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub x: C2Rust_Unnamed_6,
    pub iConstExprReg: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub iOrderByCol: u16_0,
    pub iAlias: u16_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub sortFlags: u8_0,
    #[bitfield(name = "eEName", ty = "::core::ffi::c_uint", bits = "0..=1")]
    #[bitfield(name = "done", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "reusable", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "bSorterRef", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "bNulls", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "bUsed", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "bUsingTerm", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "bNoExpand", ty = "::core::ffi::c_uint", bits = "8..=8")]
    pub eEName_done_reusable_bSorterRef_bNulls_bUsed_bUsingTerm_bNoExpand: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub z: *const ::core::ffi::c_char,
    pub n: ::core::ffi::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Index {
    pub zName: *mut ::core::ffi::c_char,
    pub aiColumn: *mut i16_0,
    pub aiRowLogEst: *mut LogEst,
    pub pTable: *mut Table,
    pub zColAff: *mut ::core::ffi::c_char,
    pub pNext: *mut Index,
    pub pSchema: *mut Schema,
    pub aSortOrder: *mut u8_0,
    pub azColl: *mut *const ::core::ffi::c_char,
    pub pPartIdxWhere: *mut Expr,
    pub aColExpr: *mut ExprList,
    pub tnum: Pgno,
    pub szIdxRow: LogEst,
    pub nKeyCol: u16_0,
    pub nColumn: u16_0,
    pub onError: u8_0,
    #[bitfield(name = "idxType", ty = "::core::ffi::c_uint", bits = "0..=1")]
    #[bitfield(name = "bUnordered", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "uniqNotNull", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "isResized", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "isCovering", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "noSkipScan", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "hasStat1", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "bNoQuery", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "bAscKeyBug", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "bHasVCol", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "bHasExpr", ty = "::core::ffi::c_uint", bits = "11..=11")]
    pub idxType_bUnordered_uniqNotNull_isResized_isCovering_noSkipScan_hasStat1_bNoQuery_bAscKeyBug_bHasVCol_bHasExpr: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub colNotIdxed: Bitmask,
}
pub type Bitmask = u64_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub cr: C2Rust_Unnamed_10,
    pub d: C2Rust_Unnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub pReturning: *mut Returning,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Returning {
    pub pParse: *mut Parse,
    pub pReturnEL: *mut ExprList,
    pub retTrig: Trigger,
    pub retTStep: TriggerStep,
    pub iRetCur: ::core::ffi::c_int,
    pub nRetCol: ::core::ffi::c_int,
    pub iRetReg: ::core::ffi::c_int,
    pub zName: [::core::ffi::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub addrCrTab: ::core::ffi::c_int,
    pub regRowid: ::core::ffi::c_int,
    pub regRoot: ::core::ffi::c_int,
    pub constraintName: Token,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseCleanup {
    pub pNext: *mut ParseCleanup,
    pub pPtr: *mut ::core::ffi::c_void,
    pub xCleanup: Option<
        unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerPrg {
    pub pTrigger: *mut Trigger,
    pub pNext: *mut TriggerPrg,
    pub pProgram: *mut SubProgram,
    pub orconf: ::core::ffi::c_int,
    pub aColmask: [u32_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AutoincInfo {
    pub pNext: *mut AutoincInfo,
    pub pTab: *mut Table,
    pub iDb: ::core::ffi::c_int,
    pub regCtr: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexedExpr {
    pub pExpr: *mut Expr,
    pub iDataCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int,
    pub iIdxCol: ::core::ffi::c_int,
    pub bMaybeNullRow: u8_0,
    pub aff: u8_0,
    pub pIENext: *mut IndexedExpr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo_col {
    pub pTab: *mut Table,
    pub pCExpr: *mut Expr,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ::core::ffi::c_int,
    pub iSorterColumn: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_13 {
    pub zToken: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_14 {
    pub pOn: *mut Expr,
    pub pUsing: *mut IdList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdList {
    pub nId: ::core::ffi::c_int,
    pub a: [IdList_item; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdList_item {
    pub zName: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_15 {
    pub pIBIndex: *mut Index,
    pub pCteUse: *mut CteUse,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub jointype: u8_0,
    #[bitfield(name = "notIndexed", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "isIndexedBy", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "isSubquery", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "isTabFunc", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "isCorrelated", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "isMaterialized", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "viaCoroutine", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "isRecursive", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "fromDDL", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "isCte", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "notCte", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "isUsing", ty = "::core::ffi::c_uint", bits = "11..=11")]
    #[bitfield(name = "isOn", ty = "::core::ffi::c_uint", bits = "12..=12")]
    #[bitfield(name = "isSynthUsing", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "isNestedFrom", ty = "::core::ffi::c_uint", bits = "14..=14")]
    #[bitfield(name = "rowidUsed", ty = "::core::ffi::c_uint", bits = "15..=15")]
    #[bitfield(name = "fixedSchema", ty = "::core::ffi::c_uint", bits = "16..=16")]
    #[bitfield(name = "hadSchema", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(name = "fromExists", ty = "::core::ffi::c_uint", bits = "18..=18")]
    pub notIndexed_isIndexedBy_isSubquery_isTabFunc_isCorrelated_isMaterialized_viaCoroutine_isRecursive_fromDDL_isCte_notCte_isUsing_isOn_isSynthUsing_isNestedFrom_rowidUsed_fixedSchema_hadSchema_fromExists: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_18 {
    pub tab: C2Rust_Unnamed_21,
    pub view: C2Rust_Unnamed_20,
    pub vtab: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub nArg: ::core::ffi::c_int,
    pub azArg: *mut *mut ::core::ffi::c_char,
    pub p: *mut VTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub pSelect: *mut Select,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub addColOffset: ::core::ffi::c_int,
    pub pFKey: *mut FKey,
    pub pDfltList: *mut ExprList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FKey {
    pub pFrom: *mut Table,
    pub pNextFrom: *mut FKey,
    pub zTo: *mut ::core::ffi::c_char,
    pub pNextTo: *mut FKey,
    pub pPrevTo: *mut FKey,
    pub nCol: ::core::ffi::c_int,
    pub isDeferred: u8_0,
    pub aAction: [u8_0; 2],
    pub apTrigger: [*mut Trigger; 2],
    pub aCol: [sColMap; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sColMap {
    pub iFrom: ::core::ffi::c_int,
    pub zCol: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Column {
    pub zCnName: *mut ::core::ffi::c_char,
    #[bitfield(name = "notNull", ty = "::core::ffi::c_uint", bits = "0..=3")]
    #[bitfield(name = "eCType", ty = "::core::ffi::c_uint", bits = "4..=7")]
    pub notNull_eCType: [u8; 1],
    pub affinity: ::core::ffi::c_char,
    pub szEst: u8_0,
    pub hName: u8_0,
    pub iDflt: u16_0,
    pub colFlags: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub htsize: ::core::ffi::c_uint,
    pub count: ::core::ffi::c_uint,
    pub first: *mut HashElem,
    pub ht: *mut _ht,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ht {
    pub count: ::core::ffi::c_uint,
    pub chain: *mut HashElem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashElem {
    pub next: *mut HashElem,
    pub prev: *mut HashElem,
    pub data: *mut ::core::ffi::c_void,
    pub pKey: *const ::core::ffi::c_char,
    pub h: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BusyHandler {
    pub xBusyHandler: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pBusyArg: *mut ::core::ffi::c_void,
    pub nBusy: ::core::ffi::c_int,
}
pub type sqlite3_xauth = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lookaside {
    pub bDisable: u32_0,
    pub sz: u16_0,
    pub szTrue: u16_0,
    pub bMalloced: u8_0,
    pub nSlot: u32_0,
    pub anStat: [u32_0; 3],
    pub pInit: *mut LookasideSlot,
    pub pFree: *mut LookasideSlot,
    pub pSmallInit: *mut LookasideSlot,
    pub pSmallFree: *mut LookasideSlot,
    pub pMiddle: *mut ::core::ffi::c_void,
    pub pStart: *mut ::core::ffi::c_void,
    pub pEnd: *mut ::core::ffi::c_void,
    pub pTrueEnd: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookasideSlot {
    pub pNext: *mut LookasideSlot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_22 {
    pub isInterrupted: ::core::ffi::c_int,
    pub notUsed1: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreUpdate {
    pub v: *mut Vdbe,
    pub pCsr: *mut VdbeCursor,
    pub op: ::core::ffi::c_int,
    pub aRecord: *mut u8_0,
    pub pKeyinfo: *mut KeyInfo,
    pub pUnpacked: *mut UnpackedRecord,
    pub pNewUnpacked: *mut UnpackedRecord,
    pub iNewReg: ::core::ffi::c_int,
    pub iBlobWrite: ::core::ffi::c_int,
    pub iKey1: i64_0,
    pub iKey2: i64_0,
    pub oldipk: Mem,
    pub aNew: *mut Mem,
    pub pTab: *mut Table,
    pub pPk: *mut Index,
    pub apDflt: *mut *mut sqlite3_value,
    pub uKey: C2Rust_Unnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub keyinfoSpace: [u8_0; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnpackedRecord {
    pub pKeyInfo: *mut KeyInfo,
    pub aMem: *mut Mem,
    pub u: C2Rust_Unnamed_24,
    pub n: ::core::ffi::c_int,
    pub nField: u16_0,
    pub default_rc: i8_0,
    pub errCode: u8_0,
    pub r1: i8_0,
    pub r2: i8_0,
    pub eqSeen: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_24 {
    pub z: *mut ::core::ffi::c_char,
    pub i: i64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_25 {
    pub xLegacy: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> (),
    >,
    pub xV2: Option<
        unsafe extern "C" fn(
            u32_0,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct sqlite3InitInfo {
    pub newTnum: Pgno,
    pub iDb: u8_0,
    pub busy: u8_0,
    #[bitfield(name = "orphanTrigger", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "imposterTable", ty = "::core::ffi::c_uint", bits = "1..=2")]
    #[bitfield(name = "reopenMemdb", ty = "::core::ffi::c_uint", bits = "3..=3")]
    pub orphanTrigger_imposterTable_reopenMemdb: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub azInit: *mut *const ::core::ffi::c_char,
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
pub type sqlite3_filename = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_mem_methods {
    pub xMalloc: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut ::core::ffi::c_void,
    >,
    pub xFree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xRealloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub xSize: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub xRoundup: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xInit: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub xShutdown: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pAppData: *mut ::core::ffi::c_void,
}
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_mutex_methods {
    pub xMutexInit: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub xMutexEnd: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub xMutexAlloc: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut sqlite3_mutex,
    >,
    pub xMutexFree: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub xMutexEnter: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub xMutexTry: Option<
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
    >,
    pub xMutexLeave: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub xMutexHeld: Option<
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
    >,
    pub xMutexNotheld: Option<
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_pcache_methods2 {
    pub iVersion: ::core::ffi::c_int,
    pub pArg: *mut ::core::ffi::c_void,
    pub xInit: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub xShutdown: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xCreate: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut sqlite3_pcache,
    >,
    pub xCachesize: Option<
        unsafe extern "C" fn(*mut sqlite3_pcache, ::core::ffi::c_int) -> (),
    >,
    pub xPagecount: Option<
        unsafe extern "C" fn(*mut sqlite3_pcache) -> ::core::ffi::c_int,
    >,
    pub xFetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_pcache,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
        ) -> *mut sqlite3_pcache_page,
    >,
    pub xUnpin: Option<
        unsafe extern "C" fn(
            *mut sqlite3_pcache,
            *mut sqlite3_pcache_page,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub xRekey: Option<
        unsafe extern "C" fn(
            *mut sqlite3_pcache,
            *mut sqlite3_pcache_page,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
        ) -> (),
    >,
    pub xTruncate: Option<
        unsafe extern "C" fn(*mut sqlite3_pcache, ::core::ffi::c_uint) -> (),
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
    pub xShrink: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type size_t = usize;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type C2Rust_Unnamed_26 = ::core::ffi::c_uint;
pub const _ISalnum: C2Rust_Unnamed_26 = 8;
pub const _ISpunct: C2Rust_Unnamed_26 = 4;
pub const _IScntrl: C2Rust_Unnamed_26 = 2;
pub const _ISblank: C2Rust_Unnamed_26 = 1;
pub const _ISgraph: C2Rust_Unnamed_26 = 32768;
pub const _ISprint: C2Rust_Unnamed_26 = 16384;
pub const _ISspace: C2Rust_Unnamed_26 = 8192;
pub const _ISxdigit: C2Rust_Unnamed_26 = 4096;
pub const _ISdigit: C2Rust_Unnamed_26 = 2048;
pub const _ISalpha: C2Rust_Unnamed_26 = 1024;
pub const _ISlower: C2Rust_Unnamed_26 = 512;
pub const _ISupper: C2Rust_Unnamed_26 = 256;
pub type uptr = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sqlite3Config {
    pub bMemstat: ::core::ffi::c_int,
    pub bCoreMutex: u8_0,
    pub bFullMutex: u8_0,
    pub bOpenUri: u8_0,
    pub bUseCis: u8_0,
    pub bSmallMalloc: u8_0,
    pub bExtraSchemaChecks: u8_0,
    pub mxStrlen: ::core::ffi::c_int,
    pub neverCorrupt: ::core::ffi::c_int,
    pub szLookaside: ::core::ffi::c_int,
    pub nLookaside: ::core::ffi::c_int,
    pub nStmtSpill: ::core::ffi::c_int,
    pub m: sqlite3_mem_methods,
    pub mutex: sqlite3_mutex_methods,
    pub pcache2: sqlite3_pcache_methods2,
    pub pHeap: *mut ::core::ffi::c_void,
    pub nHeap: ::core::ffi::c_int,
    pub mnReq: ::core::ffi::c_int,
    pub mxReq: ::core::ffi::c_int,
    pub szMmap: sqlite3_int64,
    pub mxMmap: sqlite3_int64,
    pub pPage: *mut ::core::ffi::c_void,
    pub szPage: ::core::ffi::c_int,
    pub nPage: ::core::ffi::c_int,
    pub mxParserStack: ::core::ffi::c_int,
    pub sharedCacheEnabled: ::core::ffi::c_int,
    pub szPma: u32_0,
    pub isInit: ::core::ffi::c_int,
    pub inProgress: ::core::ffi::c_int,
    pub isMutexInit: ::core::ffi::c_int,
    pub isMallocInit: ::core::ffi::c_int,
    pub isPCacheInit: ::core::ffi::c_int,
    pub nRefInitMutex: ::core::ffi::c_int,
    pub pInitMutex: *mut sqlite3_mutex,
    pub xLog: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub pLogArg: *mut ::core::ffi::c_void,
    pub mxMemdbSize: sqlite3_int64,
    pub xTestCallback: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub bLocaltimeFault: ::core::ffi::c_int,
    pub xAltLocaltime: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub iOnceResetThreshold: ::core::ffi::c_int,
    pub szSorterRef: u32_0,
    pub iPrngSeed: ::core::ffi::c_uint,
}
pub type C2Rust_Unnamed_27 = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2Rust_Unnamed_27 = 250;
pub const _SC_MINSIGSTKSZ: C2Rust_Unnamed_27 = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2Rust_Unnamed_27 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2Rust_Unnamed_27 = 247;
pub const _SC_XOPEN_STREAMS: C2Rust_Unnamed_27 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2Rust_Unnamed_27 = 245;
pub const _SC_TRACE_SYS_MAX: C2Rust_Unnamed_27 = 244;
pub const _SC_TRACE_NAME_MAX: C2Rust_Unnamed_27 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2Rust_Unnamed_27 = 242;
pub const _SC_SS_REPL_MAX: C2Rust_Unnamed_27 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2Rust_Unnamed_27 = 240;
pub const _SC_V7_LP64_OFF64: C2Rust_Unnamed_27 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2Rust_Unnamed_27 = 238;
pub const _SC_V7_ILP32_OFF32: C2Rust_Unnamed_27 = 237;
pub const _SC_RAW_SOCKETS: C2Rust_Unnamed_27 = 236;
pub const _SC_IPV6: C2Rust_Unnamed_27 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2Rust_Unnamed_27 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2Rust_Unnamed_27 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2Rust_Unnamed_27 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2Rust_Unnamed_27 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2Rust_Unnamed_27 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2Rust_Unnamed_27 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2Rust_Unnamed_27 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2Rust_Unnamed_27 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2Rust_Unnamed_27 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2Rust_Unnamed_27 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2Rust_Unnamed_27 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2Rust_Unnamed_27 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2Rust_Unnamed_27 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2Rust_Unnamed_27 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2Rust_Unnamed_27 = 185;
pub const _SC_TRACE_LOG: C2Rust_Unnamed_27 = 184;
pub const _SC_TRACE_INHERIT: C2Rust_Unnamed_27 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2Rust_Unnamed_27 = 182;
pub const _SC_TRACE: C2Rust_Unnamed_27 = 181;
pub const _SC_HOST_NAME_MAX: C2Rust_Unnamed_27 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2Rust_Unnamed_27 = 179;
pub const _SC_V6_LP64_OFF64: C2Rust_Unnamed_27 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2Rust_Unnamed_27 = 177;
pub const _SC_V6_ILP32_OFF32: C2Rust_Unnamed_27 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2Rust_Unnamed_27 = 175;
pub const _SC_STREAMS: C2Rust_Unnamed_27 = 174;
pub const _SC_SYMLOOP_MAX: C2Rust_Unnamed_27 = 173;
pub const _SC_2_PBS_TRACK: C2Rust_Unnamed_27 = 172;
pub const _SC_2_PBS_MESSAGE: C2Rust_Unnamed_27 = 171;
pub const _SC_2_PBS_LOCATE: C2Rust_Unnamed_27 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2Rust_Unnamed_27 = 169;
pub const _SC_2_PBS: C2Rust_Unnamed_27 = 168;
pub const _SC_USER_GROUPS_R: C2Rust_Unnamed_27 = 167;
pub const _SC_USER_GROUPS: C2Rust_Unnamed_27 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2Rust_Unnamed_27 = 165;
pub const _SC_TIMEOUTS: C2Rust_Unnamed_27 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2Rust_Unnamed_27 = 163;
pub const _SC_SYSTEM_DATABASE: C2Rust_Unnamed_27 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2Rust_Unnamed_27 = 161;
pub const _SC_SPORADIC_SERVER: C2Rust_Unnamed_27 = 160;
pub const _SC_SPAWN: C2Rust_Unnamed_27 = 159;
pub const _SC_SIGNALS: C2Rust_Unnamed_27 = 158;
pub const _SC_SHELL: C2Rust_Unnamed_27 = 157;
pub const _SC_REGEX_VERSION: C2Rust_Unnamed_27 = 156;
pub const _SC_REGEXP: C2Rust_Unnamed_27 = 155;
pub const _SC_SPIN_LOCKS: C2Rust_Unnamed_27 = 154;
pub const _SC_READER_WRITER_LOCKS: C2Rust_Unnamed_27 = 153;
pub const _SC_NETWORKING: C2Rust_Unnamed_27 = 152;
pub const _SC_SINGLE_PROCESS: C2Rust_Unnamed_27 = 151;
pub const _SC_MULTI_PROCESS: C2Rust_Unnamed_27 = 150;
pub const _SC_MONOTONIC_CLOCK: C2Rust_Unnamed_27 = 149;
pub const _SC_FILE_SYSTEM: C2Rust_Unnamed_27 = 148;
pub const _SC_FILE_LOCKING: C2Rust_Unnamed_27 = 147;
pub const _SC_FILE_ATTRIBUTES: C2Rust_Unnamed_27 = 146;
pub const _SC_PIPE: C2Rust_Unnamed_27 = 145;
pub const _SC_FIFO: C2Rust_Unnamed_27 = 144;
pub const _SC_FD_MGMT: C2Rust_Unnamed_27 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2Rust_Unnamed_27 = 142;
pub const _SC_DEVICE_SPECIFIC: C2Rust_Unnamed_27 = 141;
pub const _SC_DEVICE_IO: C2Rust_Unnamed_27 = 140;
pub const _SC_THREAD_CPUTIME: C2Rust_Unnamed_27 = 139;
pub const _SC_CPUTIME: C2Rust_Unnamed_27 = 138;
pub const _SC_CLOCK_SELECTION: C2Rust_Unnamed_27 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2Rust_Unnamed_27 = 136;
pub const _SC_C_LANG_SUPPORT: C2Rust_Unnamed_27 = 135;
pub const _SC_BASE: C2Rust_Unnamed_27 = 134;
pub const _SC_BARRIERS: C2Rust_Unnamed_27 = 133;
pub const _SC_ADVISORY_INFO: C2Rust_Unnamed_27 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2Rust_Unnamed_27 = 131;
pub const _SC_XOPEN_REALTIME: C2Rust_Unnamed_27 = 130;
pub const _SC_XOPEN_LEGACY: C2Rust_Unnamed_27 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2Rust_Unnamed_27 = 128;
pub const _SC_XBS5_LP64_OFF64: C2Rust_Unnamed_27 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2Rust_Unnamed_27 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2Rust_Unnamed_27 = 125;
pub const _SC_NL_TEXTMAX: C2Rust_Unnamed_27 = 124;
pub const _SC_NL_SETMAX: C2Rust_Unnamed_27 = 123;
pub const _SC_NL_NMAX: C2Rust_Unnamed_27 = 122;
pub const _SC_NL_MSGMAX: C2Rust_Unnamed_27 = 121;
pub const _SC_NL_LANGMAX: C2Rust_Unnamed_27 = 120;
pub const _SC_NL_ARGMAX: C2Rust_Unnamed_27 = 119;
pub const _SC_USHRT_MAX: C2Rust_Unnamed_27 = 118;
pub const _SC_ULONG_MAX: C2Rust_Unnamed_27 = 117;
pub const _SC_UINT_MAX: C2Rust_Unnamed_27 = 116;
pub const _SC_UCHAR_MAX: C2Rust_Unnamed_27 = 115;
pub const _SC_SHRT_MIN: C2Rust_Unnamed_27 = 114;
pub const _SC_SHRT_MAX: C2Rust_Unnamed_27 = 113;
pub const _SC_SCHAR_MIN: C2Rust_Unnamed_27 = 112;
pub const _SC_SCHAR_MAX: C2Rust_Unnamed_27 = 111;
pub const _SC_SSIZE_MAX: C2Rust_Unnamed_27 = 110;
pub const _SC_NZERO: C2Rust_Unnamed_27 = 109;
pub const _SC_MB_LEN_MAX: C2Rust_Unnamed_27 = 108;
pub const _SC_WORD_BIT: C2Rust_Unnamed_27 = 107;
pub const _SC_LONG_BIT: C2Rust_Unnamed_27 = 106;
pub const _SC_INT_MIN: C2Rust_Unnamed_27 = 105;
pub const _SC_INT_MAX: C2Rust_Unnamed_27 = 104;
pub const _SC_CHAR_MIN: C2Rust_Unnamed_27 = 103;
pub const _SC_CHAR_MAX: C2Rust_Unnamed_27 = 102;
pub const _SC_CHAR_BIT: C2Rust_Unnamed_27 = 101;
pub const _SC_XOPEN_XPG4: C2Rust_Unnamed_27 = 100;
pub const _SC_XOPEN_XPG3: C2Rust_Unnamed_27 = 99;
pub const _SC_XOPEN_XPG2: C2Rust_Unnamed_27 = 98;
pub const _SC_2_UPE: C2Rust_Unnamed_27 = 97;
pub const _SC_2_C_VERSION: C2Rust_Unnamed_27 = 96;
pub const _SC_2_CHAR_TERM: C2Rust_Unnamed_27 = 95;
pub const _SC_XOPEN_SHM: C2Rust_Unnamed_27 = 94;
pub const _SC_XOPEN_ENH_I18N: C2Rust_Unnamed_27 = 93;
pub const _SC_XOPEN_CRYPT: C2Rust_Unnamed_27 = 92;
pub const _SC_XOPEN_UNIX: C2Rust_Unnamed_27 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2Rust_Unnamed_27 = 90;
pub const _SC_XOPEN_VERSION: C2Rust_Unnamed_27 = 89;
pub const _SC_PASS_MAX: C2Rust_Unnamed_27 = 88;
pub const _SC_ATEXIT_MAX: C2Rust_Unnamed_27 = 87;
pub const _SC_AVPHYS_PAGES: C2Rust_Unnamed_27 = 86;
pub const _SC_PHYS_PAGES: C2Rust_Unnamed_27 = 85;
pub const _SC_NPROCESSORS_ONLN: C2Rust_Unnamed_27 = 84;
pub const _SC_NPROCESSORS_CONF: C2Rust_Unnamed_27 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2Rust_Unnamed_27 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2Rust_Unnamed_27 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2Rust_Unnamed_27 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2Rust_Unnamed_27 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2Rust_Unnamed_27 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2Rust_Unnamed_27 = 77;
pub const _SC_THREAD_THREADS_MAX: C2Rust_Unnamed_27 = 76;
pub const _SC_THREAD_STACK_MIN: C2Rust_Unnamed_27 = 75;
pub const _SC_THREAD_KEYS_MAX: C2Rust_Unnamed_27 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2Rust_Unnamed_27 = 73;
pub const _SC_TTY_NAME_MAX: C2Rust_Unnamed_27 = 72;
pub const _SC_LOGIN_NAME_MAX: C2Rust_Unnamed_27 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2Rust_Unnamed_27 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2Rust_Unnamed_27 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2Rust_Unnamed_27 = 68;
pub const _SC_THREADS: C2Rust_Unnamed_27 = 67;
pub const _SC_T_IOV_MAX: C2Rust_Unnamed_27 = 66;
pub const _SC_PII_OSI_M: C2Rust_Unnamed_27 = 65;
pub const _SC_PII_OSI_CLTS: C2Rust_Unnamed_27 = 64;
pub const _SC_PII_OSI_COTS: C2Rust_Unnamed_27 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2Rust_Unnamed_27 = 62;
pub const _SC_PII_INTERNET_STREAM: C2Rust_Unnamed_27 = 61;
pub const _SC_IOV_MAX: C2Rust_Unnamed_27 = 60;
pub const _SC_UIO_MAXIOV: C2Rust_Unnamed_27 = 60;
pub const _SC_SELECT: C2Rust_Unnamed_27 = 59;
pub const _SC_POLL: C2Rust_Unnamed_27 = 58;
pub const _SC_PII_OSI: C2Rust_Unnamed_27 = 57;
pub const _SC_PII_INTERNET: C2Rust_Unnamed_27 = 56;
pub const _SC_PII_SOCKET: C2Rust_Unnamed_27 = 55;
pub const _SC_PII_XTI: C2Rust_Unnamed_27 = 54;
pub const _SC_PII: C2Rust_Unnamed_27 = 53;
pub const _SC_2_LOCALEDEF: C2Rust_Unnamed_27 = 52;
pub const _SC_2_SW_DEV: C2Rust_Unnamed_27 = 51;
pub const _SC_2_FORT_RUN: C2Rust_Unnamed_27 = 50;
pub const _SC_2_FORT_DEV: C2Rust_Unnamed_27 = 49;
pub const _SC_2_C_DEV: C2Rust_Unnamed_27 = 48;
pub const _SC_2_C_BIND: C2Rust_Unnamed_27 = 47;
pub const _SC_2_VERSION: C2Rust_Unnamed_27 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2Rust_Unnamed_27 = 45;
pub const _SC_RE_DUP_MAX: C2Rust_Unnamed_27 = 44;
pub const _SC_LINE_MAX: C2Rust_Unnamed_27 = 43;
pub const _SC_EXPR_NEST_MAX: C2Rust_Unnamed_27 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2Rust_Unnamed_27 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2Rust_Unnamed_27 = 40;
pub const _SC_BC_STRING_MAX: C2Rust_Unnamed_27 = 39;
pub const _SC_BC_SCALE_MAX: C2Rust_Unnamed_27 = 38;
pub const _SC_BC_DIM_MAX: C2Rust_Unnamed_27 = 37;
pub const _SC_BC_BASE_MAX: C2Rust_Unnamed_27 = 36;
pub const _SC_TIMER_MAX: C2Rust_Unnamed_27 = 35;
pub const _SC_SIGQUEUE_MAX: C2Rust_Unnamed_27 = 34;
pub const _SC_SEM_VALUE_MAX: C2Rust_Unnamed_27 = 33;
pub const _SC_SEM_NSEMS_MAX: C2Rust_Unnamed_27 = 32;
pub const _SC_RTSIG_MAX: C2Rust_Unnamed_27 = 31;
pub const _SC_PAGESIZE: C2Rust_Unnamed_27 = 30;
pub const _SC_VERSION: C2Rust_Unnamed_27 = 29;
pub const _SC_MQ_PRIO_MAX: C2Rust_Unnamed_27 = 28;
pub const _SC_MQ_OPEN_MAX: C2Rust_Unnamed_27 = 27;
pub const _SC_DELAYTIMER_MAX: C2Rust_Unnamed_27 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2Rust_Unnamed_27 = 25;
pub const _SC_AIO_MAX: C2Rust_Unnamed_27 = 24;
pub const _SC_AIO_LISTIO_MAX: C2Rust_Unnamed_27 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2Rust_Unnamed_27 = 22;
pub const _SC_SEMAPHORES: C2Rust_Unnamed_27 = 21;
pub const _SC_MESSAGE_PASSING: C2Rust_Unnamed_27 = 20;
pub const _SC_MEMORY_PROTECTION: C2Rust_Unnamed_27 = 19;
pub const _SC_MEMLOCK_RANGE: C2Rust_Unnamed_27 = 18;
pub const _SC_MEMLOCK: C2Rust_Unnamed_27 = 17;
pub const _SC_MAPPED_FILES: C2Rust_Unnamed_27 = 16;
pub const _SC_FSYNC: C2Rust_Unnamed_27 = 15;
pub const _SC_SYNCHRONIZED_IO: C2Rust_Unnamed_27 = 14;
pub const _SC_PRIORITIZED_IO: C2Rust_Unnamed_27 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2Rust_Unnamed_27 = 12;
pub const _SC_TIMERS: C2Rust_Unnamed_27 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2Rust_Unnamed_27 = 10;
pub const _SC_REALTIME_SIGNALS: C2Rust_Unnamed_27 = 9;
pub const _SC_SAVED_IDS: C2Rust_Unnamed_27 = 8;
pub const _SC_JOB_CONTROL: C2Rust_Unnamed_27 = 7;
pub const _SC_TZNAME_MAX: C2Rust_Unnamed_27 = 6;
pub const _SC_STREAM_MAX: C2Rust_Unnamed_27 = 5;
pub const _SC_OPEN_MAX: C2Rust_Unnamed_27 = 4;
pub const _SC_NGROUPS_MAX: C2Rust_Unnamed_27 = 3;
pub const _SC_CLK_TCK: C2Rust_Unnamed_27 = 2;
pub const _SC_CHILD_MAX: C2Rust_Unnamed_27 = 1;
pub const _SC_ARG_MAX: C2Rust_Unnamed_27 = 0;
pub type ClientData = *mut ::core::ffi::c_void;
pub type Tcl_WideInt = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut ::core::ffi::c_char,
    pub freeProcDontUse: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ()>,
    pub errorLineDontUse: ::core::ffi::c_int,
}
pub type Tcl_Channel = *mut Tcl_Channel_;
pub type Tcl_Command = *mut Tcl_Command_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Obj {
    pub refCount: ::core::ffi::c_int,
    pub bytes: *mut ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub typePtr: *const Tcl_ObjType,
    pub internalRep: C2Rust_Unnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_28 {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2Rust_Unnamed_30,
    pub ptrAndLongRep: C2Rust_Unnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_29 {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
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
pub struct Tcl_DString {
    pub string: *mut ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub spaceAvl: ::core::ffi::c_int,
    pub staticSpace: [::core::ffi::c_char; 200],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqliteDb {
    pub db: *mut sqlite3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dstr {
    pub nAlloc: ::core::ffi::c_int,
    pub nUsed: ::core::ffi::c_int,
    pub z: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1CountCtx {
    pub n: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestCollationX {
    pub interp: *mut Tcl_Interp,
    pub pCmp: *mut Tcl_Obj,
    pub pDel: *mut Tcl_Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreateFunctionV2 {
    pub interp: *mut Tcl_Interp,
    pub pFunc: *mut Tcl_Obj,
    pub pStep: *mut Tcl_Obj,
    pub pFinal: *mut Tcl_Obj,
    pub pDestroy: *mut Tcl_Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncTable {
    pub zEnc: *const ::core::ffi::c_char,
    pub enc: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_31 {
    pub zName: *const ::core::ffi::c_char,
    pub op: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_32 {
    pub zName: *const ::core::ffi::c_char,
    pub iUpper: ::core::ffi::c_uint,
    pub iLower: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpenFlag {
    pub zFlag: *const ::core::ffi::c_char,
    pub flag: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_33 {
    pub zName: *mut ::core::ffi::c_char,
    pub id: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LogCallback {
    pub pInterp: *mut Tcl_Interp,
    pub pObj: *mut Tcl_Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Verb {
    pub zName: *const ::core::ffi::c_char,
    pub i: ::core::ffi::c_int,
}
pub type __rusage_who = ::core::ffi::c_int;
pub const RUSAGE_THREAD: __rusage_who = 1;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2Rust_Unnamed_47,
    pub c2rust_unnamed_0: C2Rust_Unnamed_46,
    pub c2rust_unnamed_1: C2Rust_Unnamed_45,
    pub c2rust_unnamed_2: C2Rust_Unnamed_44,
    pub c2rust_unnamed_3: C2Rust_Unnamed_43,
    pub c2rust_unnamed_4: C2Rust_Unnamed_42,
    pub c2rust_unnamed_5: C2Rust_Unnamed_41,
    pub c2rust_unnamed_6: C2Rust_Unnamed_40,
    pub c2rust_unnamed_7: C2Rust_Unnamed_39,
    pub c2rust_unnamed_8: C2Rust_Unnamed_38,
    pub c2rust_unnamed_9: C2Rust_Unnamed_37,
    pub c2rust_unnamed_10: C2Rust_Unnamed_36,
    pub c2rust_unnamed_11: C2Rust_Unnamed_35,
    pub c2rust_unnamed_12: C2Rust_Unnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_34 {
    pub ru_nivcsw: ::core::ffi::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_35 {
    pub ru_nvcsw: ::core::ffi::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_36 {
    pub ru_nsignals: ::core::ffi::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_37 {
    pub ru_msgrcv: ::core::ffi::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_38 {
    pub ru_msgsnd: ::core::ffi::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_39 {
    pub ru_oublock: ::core::ffi::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_40 {
    pub ru_inblock: ::core::ffi::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_41 {
    pub ru_nswap: ::core::ffi::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_42 {
    pub ru_majflt: ::core::ffi::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_43 {
    pub ru_minflt: ::core::ffi::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_44 {
    pub ru_isrss: ::core::ffi::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_45 {
    pub ru_idrss: ::core::ffi::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_46 {
    pub ru_ixrss: ::core::ffi::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_47 {
    pub ru_maxrss: ::core::ffi::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub type __rusage_who_t = __rusage_who;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_48 {
    pub zOptName: *const ::core::ffi::c_char,
    pub mask: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_49 {
    pub zExtName: *const ::core::ffi::c_char,
    pub pInit: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut *mut ::core::ffi::c_char,
            *const sqlite3_api_routines,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqliteDb_0 {
    pub db: *mut sqlite3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_50 {
    pub zName: *const ::core::ffi::c_char,
    pub eVal: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AutovacPageData {
    pub interp: *mut Tcl_Interp,
    pub zScript: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_51 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
    pub clientData: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_52 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_CmdProc>,
}
static mut bitmask_size: ::core::ffi::c_int = 0;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_OK_LOAD_PERMANENTLY: ::core::ffi::c_int = SQLITE_OK
    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_DELETEONCLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_EXCLUSIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_OPEN_AUTOPROXY: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_OPEN_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TEMP_DB: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TRANSIENT_DB: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_JOURNAL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_OPEN_TEMP_JOURNAL: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SUBJOURNAL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SUPER_JOURNAL: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_NOMUTEX: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_FULLMUTEX: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_SHAREDCACHE: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_PRIVATECACHE: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_WAL: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_OPEN_EXRESCODE: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_BATCH_ATOMIC: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_LOCKSTATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_LAST_ERRNO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_SIZE_HINT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_CHUNK_SIZE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_FILE_POINTER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_PERSIST_WAL: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_VFSNAME: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 13
    as ::core::ffi::c_int;
pub const SQLITE_FCNTL_TEMPFILENAME: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_DATA_VERSION: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_RESERVE_BYTES: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_EXTERNAL_READER: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const SQLITE_LAST_ERRNO: ::core::ffi::c_int = SQLITE_FCNTL_LAST_ERRNO;
pub const SQLITE_CONFIG_LOG: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_SORTERREF_SIZE: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_MAINDBNAME: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_FKEY: ::core::ffi::c_int = 1002 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_TRIGGER: ::core::ffi::c_int = 1003
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER: ::core::ffi::c_int = 1004
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION: ::core::ffi::c_int = 1005
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE: ::core::ffi::c_int = 1006
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_QPSG: ::core::ffi::c_int = 1007 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_TRIGGER_EQP: ::core::ffi::c_int = 1008 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_RESET_DATABASE: ::core::ffi::c_int = 1009
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_DEFENSIVE: ::core::ffi::c_int = 1010 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_WRITABLE_SCHEMA: ::core::ffi::c_int = 1011
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_LEGACY_ALTER_TABLE: ::core::ffi::c_int = 1012
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_DQS_DML: ::core::ffi::c_int = 1013 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_DQS_DDL: ::core::ffi::c_int = 1014 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_LEGACY_FILE_FORMAT: ::core::ffi::c_int = 1016
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_TRUSTED_SCHEMA: ::core::ffi::c_int = 1017
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_STMT_SCANSTATUS: ::core::ffi::c_int = 1018
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_REVERSE_SCANORDER: ::core::ffi::c_int = 1019
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE: ::core::ffi::c_int = 1020
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE: ::core::ffi::c_int = 1021
    as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_COMMENTS: ::core::ffi::c_int = 1022
    as ::core::ffi::c_int;
pub const SQLITE_SETLK_BLOCK_ON_CONNECT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_SQL_LENGTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_COLUMN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_EXPR_DEPTH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_COMPOUND_SELECT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_VDBE_OP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_FUNCTION_ARG: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_ATTACHED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_LIKE_PATTERN_LENGTH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_VARIABLE_NUMBER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_TRIGGER_DEPTH: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_WORKER_THREADS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_UTF16BE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_UTF16: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_ANY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_UTF16_ALIGNED: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_TESTCTRL_PRNG_SAVE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_PRNG_RESTORE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_FK_NO_ACTION: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_OPTIMIZATIONS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_INTERNAL_FUNCTIONS: ::core::ffi::c_int = 17
    as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_LOCALTIME_FAULT: ::core::ffi::c_int = 18;
pub const SQLITE_TESTCTRL_NEVER_CORRUPT: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_SORTER_MMAP: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_IMPOSTER: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_RESULT_INTREAL: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_PRNG_SEED: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS: ::core::ffi::c_int = 29
    as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_TRACEFLAGS: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_FULLSCAN_STEP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_SORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_AUTOINDEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_VM_STEP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_REPREPARE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_RUN: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_MEMUSED: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_BIGENDIAN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_UTF16NATIVE: ::core::ffi::c_int = SQLITE_UTF16LE;
unsafe extern "C" fn testHexToInt(mut h: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        if h >= '0' as i32 && h <= '9' as i32 {
            return h - '0' as i32
        } else if h >= 'a' as i32 && h <= 'f' as i32 {
            return h - 'a' as i32 + 10 as ::core::ffi::c_int
        } else {
            return h - 'A' as i32 + 10 as ::core::ffi::c_int
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3TestTextToPtr(
    mut z: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut v: u64_0 = 0;
        let mut v2: u32_0 = 0;
        if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '0' as i32
            && *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'x' as i32
        {
            z = z.offset(2 as ::core::ffi::c_int as isize);
        }
        v = 0 as u64_0;
        while *z != 0 {
            v = ((v as ::core::ffi::c_ulonglong) << 4 as ::core::ffi::c_int)
                .wrapping_add(
                    testHexToInt(*z as ::core::ffi::c_int) as ::core::ffi::c_ulonglong,
                ) as u64_0;
            z = z.offset(1);
        }
        if ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
            == ::core::mem::size_of::<u64_0>() as usize
        {
            memcpy(
                &raw mut p as *mut ::core::ffi::c_void,
                &raw mut v as *const ::core::ffi::c_void,
                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
            );
        } else {
            v2 = v as u32_0;
            memcpy(
                &raw mut p as *mut ::core::ffi::c_void,
                &raw mut v2 as *const ::core::ffi::c_void,
                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
            );
        }
        return p;
    }
}
unsafe extern "C" fn get_sqlite_pointer(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut SqliteDb = ::core::ptr::null_mut::<SqliteDb>();
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
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SQLITE-CONNECTION\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetCommandInfo(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut cmdInfo,
        ) == 0
        {
            Tcl_AppendResult(
                interp,
                b"command not found: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        p = cmdInfo.objClientData as *mut SqliteDb;
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%p\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).db,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getDbPointer(
    mut interp: *mut Tcl_Interp,
    mut zA: *const ::core::ffi::c_char,
    mut ppDb: *mut *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut SqliteDb = ::core::ptr::null_mut::<SqliteDb>();
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
        if Tcl_GetCommandInfo(interp, zA, &raw mut cmdInfo) != 0 {
            p = cmdInfo.objClientData as *mut SqliteDb;
            *ppDb = (*p).db;
        } else {
            *ppDb = sqlite3TestTextToPtr(zA) as *mut sqlite3;
        }
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3TestErrCode(
    mut interp: *mut Tcl_Interp,
    mut db: *mut sqlite3,
    mut rc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if sqlite3_threadsafe() == 0 as ::core::ffi::c_int && rc != SQLITE_MISUSE
            && rc != SQLITE_OK && sqlite3_errcode(db) != rc
        {
            let mut zBuf: [::core::ffi::c_char; 200] = [0; 200];
            let mut r2: ::core::ffi::c_int = sqlite3_errcode(db);
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 200]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"error code %s (%d) does not match sqlite3_errcode %s (%d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3ErrName(rc),
                rc,
                sqlite3ErrName(r2),
                r2,
            );
            Tcl_ResetResult(interp);
            Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn getStmtPointer(
    mut interp: *mut Tcl_Interp,
    mut zArg: *const ::core::ffi::c_char,
    mut ppStmt: *mut *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        *ppStmt = sqlite3TestTextToPtr(zArg) as *mut sqlite3_stmt;
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3TestMakePointerStr(
    mut interp: *mut Tcl_Interp,
    mut zPtr: *mut ::core::ffi::c_char,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_snprintf(
            100 as ::core::ffi::c_int,
            zPtr,
            b"%p\0".as_ptr() as *const ::core::ffi::c_char,
            p,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn exec_printf_cb(
    mut pArg: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut name: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut str: *mut Tcl_DString = pArg as *mut Tcl_DString;
        let mut i: ::core::ffi::c_int = 0;
        if (*str).length == 0 as ::core::ffi::c_int {
            i = 0 as ::core::ffi::c_int;
            while i < argc {
                Tcl_DStringAppendElement(
                    str,
                    if !(*name.offset(i as isize)).is_null() {
                        *name.offset(i as isize) as *const ::core::ffi::c_char
                    } else {
                        b"NULL\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                i += 1;
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            Tcl_DStringAppendElement(
                str,
                if !(*argv.offset(i as isize)).is_null() {
                    *argv.offset(i as isize) as *const ::core::ffi::c_char
                } else {
                    b"NULL\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn test_io_trace(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
unsafe extern "C" fn clang_sanitize_address(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if res == 0 as ::core::ffi::c_int
            && !getenv(b"OMIT_MISUSE\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        {
            res = 1 as ::core::ffi::c_int;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(res));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_exec_printf(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut str: Tcl_DString = Tcl_DString {
            string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            length: 0,
            spaceAvl: 0,
            staticSpace: [0; 200],
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zBuf: [::core::ffi::c_char; 30] = [0; 30];
        if argc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB FORMAT STRING\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_DStringInit(&raw mut str);
        zSql = sqlite3_mprintf(
            *argv.offset(2 as ::core::ffi::c_int as isize),
            *argv.offset(3 as ::core::ffi::c_int as isize),
        );
        rc = sqlite3_exec(
            db,
            zSql,
            Some(
                exec_printf_cb
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *mut *mut ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            &raw mut str as *mut ::core::ffi::c_void,
            &raw mut zErr,
        );
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 30]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
        );
        Tcl_AppendElement(interp, &raw mut zBuf as *mut ::core::ffi::c_char);
        Tcl_AppendElement(interp, if rc == SQLITE_OK { str.string } else { zErr });
        Tcl_DStringFree(&raw mut str);
        if !zErr.is_null() {
            sqlite3_free(zErr as *mut ::core::ffi::c_void);
        }
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_exec_hex(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut str: Tcl_DString = Tcl_DString {
            string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            length: 0,
            spaceAvl: 0,
            staticSpace: [0; 200],
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zHex: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zSql: [::core::ffi::c_char; 501] = [0; 501];
        let mut zBuf: [::core::ffi::c_char; 30] = [0; 30];
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB HEX\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zHex = *argv.offset(2 as ::core::ffi::c_int as isize);
        j = 0 as ::core::ffi::c_int;
        i = j;
        while (i as usize)
            < (::core::mem::size_of::<[::core::ffi::c_char; 501]>() as usize)
                .wrapping_sub(1 as usize)
            && *zHex.offset(j as isize) as ::core::ffi::c_int != 0
        {
            if *zHex.offset(j as isize) as ::core::ffi::c_int == '%' as i32
                && *zHex.offset((j + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int != 0
                && *zHex.offset((j + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int != 0
            {
                zSql[i as usize] = ((testHexToInt(
                    *zHex.offset((j + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
                ) << 4 as ::core::ffi::c_int)
                    + testHexToInt(
                        *zHex.offset((j + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int,
                    )) as ::core::ffi::c_char;
                j += 2 as ::core::ffi::c_int;
            } else {
                zSql[i as usize] = *zHex.offset(j as isize);
            }
            i += 1;
            j += 1;
        }
        zSql[i as usize] = 0 as ::core::ffi::c_char;
        Tcl_DStringInit(&raw mut str);
        rc = sqlite3_exec(
            db,
            &raw mut zSql as *mut ::core::ffi::c_char,
            Some(
                exec_printf_cb
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *mut *mut ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            &raw mut str as *mut ::core::ffi::c_void,
            &raw mut zErr,
        );
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 30]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
        );
        Tcl_AppendElement(interp, &raw mut zBuf as *mut ::core::ffi::c_char);
        Tcl_AppendElement(interp, if rc == SQLITE_OK { str.string } else { zErr });
        Tcl_DStringFree(&raw mut str);
        if !zErr.is_null() {
            sqlite3_free(zErr as *mut ::core::ffi::c_void);
        }
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn db_enter(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_mutex_enter((*db).mutex);
        return TCL_OK;
    }
}
unsafe extern "C" fn db_leave(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_mutex_leave((*db).mutex);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_exec(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut str: Tcl_DString = Tcl_DString {
            string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            length: 0,
            spaceAvl: 0,
            staticSpace: [0; 200],
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 30] = [0; 30];
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB SQL\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_DStringInit(&raw mut str);
        zSql = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(2 as ::core::ffi::c_int as isize),
        );
        j = 0 as ::core::ffi::c_int;
        i = j;
        while *zSql.offset(i as isize) != 0 {
            if *zSql.offset(i as isize) as ::core::ffi::c_int == '%' as i32 {
                let c2rust_fresh0 = j;
                j = j + 1;
                *zSql.offset(c2rust_fresh0 as isize) = ((testHexToInt(
                    *zSql.offset((i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
                ) << 4 as ::core::ffi::c_int)
                    + testHexToInt(
                        *zSql.offset((i + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int,
                    )) as ::core::ffi::c_char;
                i += 3 as ::core::ffi::c_int;
            } else {
                let c2rust_fresh1 = i;
                i = i + 1;
                let c2rust_fresh2 = j;
                j = j + 1;
                *zSql.offset(c2rust_fresh2 as isize) = *zSql
                    .offset(c2rust_fresh1 as isize);
            }
        }
        *zSql.offset(j as isize) = 0 as ::core::ffi::c_char;
        rc = sqlite3_exec(
            db,
            zSql,
            Some(
                exec_printf_cb
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *mut *mut ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            &raw mut str as *mut ::core::ffi::c_void,
            &raw mut zErr,
        );
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 30]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
        );
        Tcl_AppendElement(interp, &raw mut zBuf as *mut ::core::ffi::c_char);
        Tcl_AppendElement(interp, if rc == SQLITE_OK { str.string } else { zErr });
        Tcl_DStringFree(&raw mut str);
        if !zErr.is_null() {
            sqlite3_free(zErr as *mut ::core::ffi::c_void);
        }
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_exec_nr(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB SQL\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_exec(
            db,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            &raw mut zErr,
        );
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_mprintf_z(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zResult: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        i = 2 as ::core::ffi::c_int;
        while i < argc && (i == 2 as ::core::ffi::c_int || !zResult.is_null()) {
            zResult = sqlite3_mprintf(
                b"%z%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                zResult,
                *argv.offset(1 as ::core::ffi::c_int as isize),
                *argv.offset(i as isize),
            );
            i += 1;
        }
        Tcl_AppendResult(interp, zResult, NULL);
        sqlite3_free(zResult as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_mprintf_n(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zStr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        zStr = sqlite3_mprintf(
            b"%s%n\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut n,
        );
        sqlite3_free(zStr as *mut ::core::ffi::c_void);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(n));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_snprintf_int(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zStr: [::core::ffi::c_char; 100] = [0; 100];
        let mut n: ::core::ffi::c_int = atoi(
            *argv.offset(1 as ::core::ffi::c_int as isize),
        );
        let mut zFormat: *const ::core::ffi::c_char = *argv
            .offset(2 as ::core::ffi::c_int as isize);
        let mut a1: ::core::ffi::c_int = atoi(
            *argv.offset(3 as ::core::ffi::c_int as isize),
        );
        if n as usize > ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as usize {
            n = ::core::mem::size_of::<[::core::ffi::c_char; 100]>()
                as ::core::ffi::c_int;
        }
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zStr as *mut ::core::ffi::c_char,
            b"abcdefghijklmnopqrstuvwxyz\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3_snprintf(n, &raw mut zStr as *mut ::core::ffi::c_char, zFormat, a1);
        Tcl_AppendResult(interp, &raw mut zStr as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_get_table_printf(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut str: Tcl_DString = Tcl_DString {
            string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            length: 0,
            spaceAvl: 0,
            staticSpace: [0; 200],
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nRow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aResult: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            *mut ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 30] = [0; 30];
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut resCount: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        if argc == 5 as ::core::ffi::c_int {
            if Tcl_GetInt(
                interp,
                *argv.offset(4 as ::core::ffi::c_int as isize),
                &raw mut resCount,
            ) != 0
            {
                return TCL_ERROR;
            }
        }
        if argc != 4 as ::core::ffi::c_int && argc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB FORMAT STRING ?COUNT?\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_DStringInit(&raw mut str);
        zSql = sqlite3_mprintf(
            *argv.offset(2 as ::core::ffi::c_int as isize),
            *argv.offset(3 as ::core::ffi::c_int as isize),
        );
        if argc == 5 as ::core::ffi::c_int {
            rc = sqlite3_get_table(
                db,
                zSql,
                &raw mut aResult,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                &raw mut zErr,
            );
        } else {
            rc = sqlite3_get_table(
                db,
                zSql,
                &raw mut aResult,
                &raw mut nRow,
                &raw mut nCol,
                &raw mut zErr,
            );
            resCount = (nRow + 1 as ::core::ffi::c_int) * nCol;
        }
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 30]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
        );
        Tcl_ResetResult(interp);
        Tcl_AppendElement(interp, &raw mut zBuf as *mut ::core::ffi::c_char);
        if rc == SQLITE_OK {
            if argc == 4 as ::core::ffi::c_int {
                sqlite3_snprintf(
                    ::core::mem::size_of::<[::core::ffi::c_char; 30]>()
                        as ::core::ffi::c_int,
                    &raw mut zBuf as *mut ::core::ffi::c_char,
                    b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                    nRow,
                );
                Tcl_AppendElement(interp, &raw mut zBuf as *mut ::core::ffi::c_char);
                sqlite3_snprintf(
                    ::core::mem::size_of::<[::core::ffi::c_char; 30]>()
                        as ::core::ffi::c_int,
                    &raw mut zBuf as *mut ::core::ffi::c_char,
                    b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                    nCol,
                );
                Tcl_AppendElement(interp, &raw mut zBuf as *mut ::core::ffi::c_char);
            }
            i = 0 as ::core::ffi::c_int;
            while i < resCount {
                Tcl_AppendElement(
                    interp,
                    if !(*aResult.offset(i as isize)).is_null() {
                        *aResult.offset(i as isize) as *const ::core::ffi::c_char
                    } else {
                        b"NULL\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                i += 1;
            }
        } else {
            Tcl_AppendElement(interp, zErr);
        }
        sqlite3_free_table(aResult);
        if !zErr.is_null() {
            sqlite3_free(zErr as *mut ::core::ffi::c_void);
        }
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_last_rowid(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zBuf: [::core::ffi::c_char; 30] = [0; 30];
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 30]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_last_insert_rowid(db),
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn test_key(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
unsafe extern "C" fn test_rekey(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite_test_close(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FILENAME\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_close(db);
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite_test_close_v2(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FILENAME\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_close_v2(db);
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn t1_ifnullFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            if SQLITE_NULL != sqlite3_value_type(*argv.offset(i as isize)) {
                let mut n: ::core::ffi::c_int = sqlite3_value_bytes(
                    *argv.offset(i as isize),
                );
                sqlite3_result_text(
                    context,
                    sqlite3_value_text(*argv.offset(i as isize))
                        as *mut ::core::ffi::c_char,
                    n,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                break;
            } else {
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn hex8Func(
    mut p: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut z: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 200] = [0; 200];
        z = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[::core::ffi::c_char; 200]>() as usize)
                .wrapping_div(2 as usize)
                .wrapping_sub(2 as usize)
            && *z.offset(i as isize) as ::core::ffi::c_int != 0
        {
            sqlite3_snprintf(
                (::core::mem::size_of::<[::core::ffi::c_char; 200]>() as usize)
                    .wrapping_sub((i * 2 as ::core::ffi::c_int) as usize)
                    as ::core::ffi::c_int,
                (&raw mut zBuf as *mut ::core::ffi::c_char)
                    .offset((i * 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *z.offset(i as isize) as ::core::ffi::c_int,
            );
            i += 1;
        }
        zBuf[(i * 2 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_char;
        sqlite3_result_text(
            p,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
unsafe extern "C" fn hex16Func(
    mut p: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut z: *const ::core::ffi::c_ushort = ::core::ptr::null::<
            ::core::ffi::c_ushort,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 400] = [0; 400];
        z = sqlite3_value_text16(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_ushort;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[::core::ffi::c_char; 400]>() as usize)
                .wrapping_div(4 as usize)
                .wrapping_sub(4 as usize)
            && *z.offset(i as isize) as ::core::ffi::c_int != 0
        {
            sqlite3_snprintf(
                (::core::mem::size_of::<[::core::ffi::c_char; 400]>() as usize)
                    .wrapping_sub((i * 4 as ::core::ffi::c_int) as usize)
                    as ::core::ffi::c_int,
                (&raw mut zBuf as *mut ::core::ffi::c_char)
                    .offset((i * 4 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char,
                b"%04x\0".as_ptr() as *const ::core::ffi::c_char,
                *z.offset(i as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
            );
            i += 1;
        }
        zBuf[(i * 4 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_char;
        sqlite3_result_text(
            p,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
unsafe extern "C" fn dstrAppend(
    mut p: *mut dstr,
    mut z: *const ::core::ffi::c_char,
    mut divider: ::core::ffi::c_int,
) {
    unsafe {
        let mut n: ::core::ffi::c_int = strlen(z) as ::core::ffi::c_int;
        if (*p).nUsed + n + 2 as ::core::ffi::c_int > (*p).nAlloc {
            let mut zNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            (*p).nAlloc = (*p).nAlloc * 2 as ::core::ffi::c_int + n
                + 200 as ::core::ffi::c_int;
            zNew = sqlite3_realloc((*p).z as *mut ::core::ffi::c_void, (*p).nAlloc)
                as *mut ::core::ffi::c_char;
            if zNew.is_null() {
                sqlite3_free((*p).z as *mut ::core::ffi::c_void);
                memset(
                    p as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<dstr>() as size_t,
                );
                return;
            }
            (*p).z = zNew;
        }
        if divider != 0 && (*p).nUsed > 0 as ::core::ffi::c_int {
            let c2rust_fresh3 = (*p).nUsed;
            (*p).nUsed = (*p).nUsed + 1;
            *(*p).z.offset(c2rust_fresh3 as isize) = divider as ::core::ffi::c_char;
        }
        memcpy(
            (*p).z.offset((*p).nUsed as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            z as *const ::core::ffi::c_void,
            (n + 1 as ::core::ffi::c_int) as size_t,
        );
        (*p).nUsed += n;
    }
}
unsafe extern "C" fn execFuncCallback(
    mut pData: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut NotUsed: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut dstr = pData as *mut dstr;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            if (*argv.offset(i as isize)).is_null() {
                dstrAppend(
                    p,
                    b"NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    ' ' as i32,
                );
            } else {
                dstrAppend(p, *argv.offset(i as isize), ' ' as i32);
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn sqlite3ExecFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut x: dstr = dstr {
            nAlloc: 0,
            nUsed: 0,
            z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        memset(
            &raw mut x as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<dstr>() as size_t,
        );
        sqlite3_exec(
            sqlite3_user_data(context) as *mut sqlite3,
            sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *mut ::core::ffi::c_char,
            Some(
                execFuncCallback
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *mut *mut ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            &raw mut x as *mut ::core::ffi::c_void,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        sqlite3_result_text(
            context,
            x.z,
            x.nUsed,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_free(x.z as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn tkt2213Function(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut nText: ::core::ffi::c_int = 0;
        let mut zText1: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zText2: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zText3: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        nText = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        zText1 = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        zText2 = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        zText3 = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        if zText1 != zText2 || zText2 != zText3 {
            sqlite3_result_error(
                context,
                b"tkt2213 is not fixed\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            );
        } else {
            let mut zCopy: *mut ::core::ffi::c_char = sqlite3_malloc(nText)
                as *mut ::core::ffi::c_char;
            memcpy(
                zCopy as *mut ::core::ffi::c_void,
                zText1 as *const ::core::ffi::c_void,
                nText as size_t,
            );
            sqlite3_result_text(
                context,
                zCopy,
                nText,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        };
    }
}
unsafe extern "C" fn ptrChngFunction(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p1: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut p2: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut zCmd: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if argc != 4 as ::core::ffi::c_int {
            return;
        }
        zCmd = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        if zCmd.is_null() {
            return;
        }
        if strcmp(zCmd, b"text\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            p1 = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_void;
        } else if strcmp(zCmd, b"text16\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            p1 = sqlite3_value_text16(*argv.offset(0 as ::core::ffi::c_int as isize));
        } else if strcmp(zCmd, b"blob\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            p1 = sqlite3_value_blob(*argv.offset(0 as ::core::ffi::c_int as isize));
        } else {
            return
        }
        zCmd = sqlite3_value_text(*argv.offset(2 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        if zCmd.is_null() {
            return;
        }
        if strcmp(zCmd, b"bytes\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        } else if strcmp(zCmd, b"bytes16\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            sqlite3_value_bytes16(*argv.offset(0 as ::core::ffi::c_int as isize));
        } else if strcmp(zCmd, b"noop\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {} else {
            return
        }
        zCmd = sqlite3_value_text(*argv.offset(3 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        if zCmd.is_null() {
            return;
        }
        if strcmp(zCmd, b"text\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            p2 = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_void;
        } else if strcmp(zCmd, b"text16\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            p2 = sqlite3_value_text16(*argv.offset(0 as ::core::ffi::c_int as isize));
        } else if strcmp(zCmd, b"blob\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            p2 = sqlite3_value_blob(*argv.offset(0 as ::core::ffi::c_int as isize));
        } else {
            return
        }
        sqlite3_result_int(context, (p1 != p2) as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn nondeterministicFunction(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        static mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let c2rust_fresh4 = cnt;
        cnt = cnt + 1;
        sqlite3_result_int(context, c2rust_fresh4);
    }
}
unsafe extern "C" fn intrealFunction(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut v: sqlite3_int64 = sqlite3_value_int64(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        sqlite3_result_int64(context, v);
        sqlite3_test_control(SQLITE_TESTCTRL_RESULT_INTREAL, context);
    }
}
unsafe extern "C" fn addTextTypeFunction(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        sqlite3_result_value(context, *argv.offset(0 as ::core::ffi::c_int as isize));
    }
}
unsafe extern "C" fn addIntTypeFunction(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize));
        sqlite3_result_value(context, *argv.offset(0 as ::core::ffi::c_int as isize));
    }
}
unsafe extern "C" fn addRealTypeFunction(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize));
        sqlite3_result_value(context, *argv.offset(0 as ::core::ffi::c_int as isize));
    }
}
unsafe extern "C" fn shellStrtod(
    mut pCtx: *mut sqlite3_context,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = sqlite3_value_text(
            *apVal.offset(0 as ::core::ffi::c_int as isize),
        ) as *mut ::core::ffi::c_char;
        if z.is_null() {
            return;
        }
        sqlite3_result_double(
            pCtx,
            strtod(z, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>()),
        );
    }
}
unsafe extern "C" fn shellDtostr(
    mut pCtx: *mut sqlite3_context,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut r: ::core::ffi::c_double = sqlite3_value_double(
            *apVal.offset(0 as ::core::ffi::c_int as isize),
        );
        let mut n: ::core::ffi::c_int = if nVal >= 2 as ::core::ffi::c_int {
            sqlite3_value_int(*apVal.offset(1 as ::core::ffi::c_int as isize))
        } else {
            26 as ::core::ffi::c_int
        };
        let mut z: [::core::ffi::c_char; 400] = [0; 400];
        if n < 1 as ::core::ffi::c_int {
            n = 1 as ::core::ffi::c_int;
        }
        if n > 350 as ::core::ffi::c_int {
            n = 350 as ::core::ffi::c_int;
        }
        sprintf(
            &raw mut z as *mut ::core::ffi::c_char,
            b"%#+.*e\0".as_ptr() as *const ::core::ffi::c_char,
            n,
            r,
        );
        sqlite3_result_text(
            pCtx,
            &raw mut z as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
unsafe extern "C" fn inttoptrFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut i64: sqlite3_int64 = 0;
        i64 = sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize));
        if ::core::mem::size_of::<sqlite3_int64>() as usize
            == ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
        {
            memcpy(
                &raw mut p as *mut ::core::ffi::c_void,
                &raw mut i64 as *const ::core::ffi::c_void,
                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
            );
        } else {
            let mut i32: ::core::ffi::c_int = (i64 as ::core::ffi::c_longlong
                & 0xffffffff as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            memcpy(
                &raw mut p as *mut ::core::ffi::c_void,
                &raw mut i32 as *const ::core::ffi::c_void,
                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
            );
        }
        sqlite3_result_pointer(
            context,
            p,
            b"carray\0".as_ptr() as *const ::core::ffi::c_char,
            None,
        );
    }
}
unsafe extern "C" fn test_create_function(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_create_function(
            db,
            b"x_coalesce\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                t1_ifnullFunc
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
                b"hex8\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8 | SQLITE_DETERMINISTIC,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    hex8Func
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
                b"hex16\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF16 | SQLITE_DETERMINISTIC,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    hex16Func
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
                b"tkt2213func\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_ANY,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    tkt2213Function
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
                b"pointer_change\0".as_ptr() as *const ::core::ffi::c_char,
                4 as ::core::ffi::c_int,
                SQLITE_ANY,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    ptrChngFunction
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
                b"counter1\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    nondeterministicFunction
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
                b"counter2\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_UTF8 | SQLITE_DETERMINISTIC,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    nondeterministicFunction
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
                b"intreal\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    intrealFunction
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
                b"add_text_type\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    addTextTypeFunction
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
                b"add_int_type\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    addIntTypeFunction
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
                b"add_real_type\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    addRealTypeFunction
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
                b"strtod\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    shellStrtod
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
                b"dtostr\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    shellDtostr
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
                b"dtostr\0".as_ptr() as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    shellDtostr
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
                b"inttoptr\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    inttoptrFunc
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
            let mut zUtf16: *const ::core::ffi::c_void = ::core::ptr::null::<
                ::core::ffi::c_void,
            >();
            let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
            sqlite3_mutex_enter((*db).mutex);
            pVal = sqlite3ValueNew(db);
            sqlite3ValueSetStr(
                pVal,
                -1 as ::core::ffi::c_int,
                b"x_sqlite_exec\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                SQLITE_UTF8 as u8_0,
                SQLITE_STATIC,
            );
            zUtf16 = sqlite3ValueText(pVal, SQLITE_UTF16NATIVE as u8_0);
            if (*db).mallocFailed != 0 {
                rc = SQLITE_NOMEM;
            } else {
                rc = sqlite3_create_function16(
                    db,
                    zUtf16,
                    1 as ::core::ffi::c_int,
                    SQLITE_UTF16,
                    db as *mut ::core::ffi::c_void,
                    Some(
                        sqlite3ExecFunc
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
            sqlite3ValueFree(pVal);
            sqlite3_mutex_leave((*db).mutex);
        }
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        Tcl_SetResult(interp, sqlite3ErrName(rc) as *mut ::core::ffi::c_char, None);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_drop_modules(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if argc < 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_drop_modules(
            db,
            if argc > 2 as ::core::ffi::c_int {
                argv.offset(2 as ::core::ffi::c_int as isize)
                    as *mut *const ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>()
            },
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn t1CountStep(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut t1CountCtx = ::core::ptr::null_mut::<t1CountCtx>();
        p = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<t1CountCtx>() as ::core::ffi::c_int,
        ) as *mut t1CountCtx;
        if (argc == 0 as ::core::ffi::c_int
            || SQLITE_NULL
                != sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize)))
            && !p.is_null()
        {
            (*p).n += 1;
        }
        if argc > 0 as ::core::ffi::c_int {
            let mut v: ::core::ffi::c_int = sqlite3_value_int(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            if v == 40 as ::core::ffi::c_int {
                sqlite3_result_error(
                    context,
                    b"value of 40 handed to x_count\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
            } else if v == 41 as ::core::ffi::c_int {
                let zUtf16ErrMsg: [::core::ffi::c_char; 9] = [
                    0 as ::core::ffi::c_int as ::core::ffi::c_char,
                    0x61 as ::core::ffi::c_int as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int as ::core::ffi::c_char,
                    0x62 as ::core::ffi::c_int as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int as ::core::ffi::c_char,
                    0x63 as ::core::ffi::c_int as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int as ::core::ffi::c_char,
                ];
                sqlite3_result_error16(
                    context,
                    (&raw const zUtf16ErrMsg as *const ::core::ffi::c_char)
                        .offset((1 as ::core::ffi::c_int - SQLITE_BIGENDIAN) as isize)
                        as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    -1 as ::core::ffi::c_int,
                );
            }
        }
    }
}
unsafe extern "C" fn t1CountFinalize(mut context: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut t1CountCtx = ::core::ptr::null_mut::<t1CountCtx>();
        p = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<t1CountCtx>() as ::core::ffi::c_int,
        ) as *mut t1CountCtx;
        if !p.is_null() {
            if (*p).n == 42 as ::core::ffi::c_int {
                sqlite3_result_error(
                    context,
                    b"x_count totals to 42\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
            } else {
                sqlite3_result_int(
                    context,
                    if !p.is_null() { (*p).n } else { 0 as ::core::ffi::c_int },
                );
            }
        }
    }
}
unsafe extern "C" fn legacyCountStep(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {}
unsafe extern "C" fn legacyCountFinalize(mut context: *mut sqlite3_context) {
    unsafe {
        sqlite3_result_int(context, sqlite3_aggregate_count(context));
    }
}
unsafe extern "C" fn test_create_aggregate(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FILENAME\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_create_function(
            db,
            b"x_count\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
            Some(
                t1CountStep
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            Some(t1CountFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> ()),
        );
        if rc == SQLITE_OK {
            rc = sqlite3_create_function(
                db,
                b"x_count\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
                Some(
                    t1CountStep
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                Some(t1CountFinalize as unsafe extern "C" fn(*mut sqlite3_context) -> ()),
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3_create_function(
                db,
                b"legacy_count\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                SQLITE_ANY,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
                Some(
                    legacyCountStep
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                Some(
                    legacyCountFinalize
                        as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                ),
            );
        }
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        Tcl_SetResult(interp, sqlite3ErrName(rc) as *mut ::core::ffi::c_char, None);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_printf(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" TEXT\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        printf(
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(1 as ::core::ffi::c_int as isize),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_mprintf_int(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: [::core::ffi::c_int; 3] = [0; 3];
        let mut i: ::core::ffi::c_int = 0;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FORMAT INT INT INT\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        i = 2 as ::core::ffi::c_int;
        while i < 5 as ::core::ffi::c_int {
            if Tcl_GetInt(
                interp,
                *argv.offset(i as isize),
                (&raw mut a as *mut ::core::ffi::c_int)
                    .offset((i - 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_int,
            ) != 0
            {
                return TCL_ERROR;
            }
            i += 1;
        }
        z = sqlite3_mprintf(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            a[0 as ::core::ffi::c_int as usize],
            a[1 as ::core::ffi::c_int as usize],
            a[2 as ::core::ffi::c_int as usize],
        );
        Tcl_AppendResult(interp, z, NULL);
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_mprintf_int64(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut a: [sqlite_int64; 3] = [0; 3];
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FORMAT INT INT INT\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        i = 2 as ::core::ffi::c_int;
        while i < 5 as ::core::ffi::c_int {
            if sqlite3Atoi64(
                *argv.offset(i as isize),
                (&raw mut a as *mut sqlite_int64)
                    .offset((i - 2 as ::core::ffi::c_int) as isize) as *mut i64_0,
                sqlite3Strlen30(*argv.offset(i as isize)),
                SQLITE_UTF8 as u8_0,
            ) != 0
            {
                Tcl_AppendResult(
                    interp,
                    b"argument is not a valid 64-bit integer\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    NULL,
                );
                return TCL_ERROR;
            }
            i += 1;
        }
        z = sqlite3_mprintf(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            a[0 as ::core::ffi::c_int as usize],
            a[1 as ::core::ffi::c_int as usize],
            a[2 as ::core::ffi::c_int as usize],
        );
        Tcl_AppendResult(interp, z, NULL);
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_mprintf_long(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut a: [::core::ffi::c_long; 3] = [0; 3];
        let mut b: [::core::ffi::c_int; 3] = [0; 3];
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FORMAT INT INT INT\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        i = 2 as ::core::ffi::c_int;
        while i < 5 as ::core::ffi::c_int {
            if Tcl_GetInt(
                interp,
                *argv.offset(i as isize),
                (&raw mut b as *mut ::core::ffi::c_int)
                    .offset((i - 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_int,
            ) != 0
            {
                return TCL_ERROR;
            }
            a[(i - 2 as ::core::ffi::c_int) as usize] = b[(i - 2 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_long;
            a[(i - 2 as ::core::ffi::c_int) as usize] = (a[(i - 2 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_ulonglong
                & ((1 as ::core::ffi::c_int as ::core::ffi::c_ulonglong)
                    << (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        .wrapping_mul(8 as usize))
                    .wrapping_sub(1 as ::core::ffi::c_ulonglong)) as ::core::ffi::c_long;
            i += 1;
        }
        z = sqlite3_mprintf(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            a[0 as ::core::ffi::c_int as usize],
            a[1 as ::core::ffi::c_int as usize],
            a[2 as ::core::ffi::c_int as usize],
        );
        Tcl_AppendResult(interp, z, NULL);
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_mprintf_str(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: [::core::ffi::c_int; 3] = [0; 3];
        let mut i: ::core::ffi::c_int = 0;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc < 4 as ::core::ffi::c_int || argc > 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FORMAT INT INT ?STRING?\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        i = 2 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            if Tcl_GetInt(
                interp,
                *argv.offset(i as isize),
                (&raw mut a as *mut ::core::ffi::c_int)
                    .offset((i - 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_int,
            ) != 0
            {
                return TCL_ERROR;
            }
            i += 1;
        }
        z = sqlite3_mprintf(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            a[0 as ::core::ffi::c_int as usize],
            a[1 as ::core::ffi::c_int as usize],
            if argc > 4 as ::core::ffi::c_int {
                *argv.offset(4 as ::core::ffi::c_int as isize)
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
        );
        Tcl_AppendResult(interp, z, NULL);
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_snprintf_str(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: [::core::ffi::c_int; 3] = [0; 3];
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc < 5 as ::core::ffi::c_int || argc > 6 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" INT FORMAT INT INT ?STRING?\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if Tcl_GetInt(interp, *argv.offset(1 as ::core::ffi::c_int as isize), &raw mut n)
            != 0
        {
            return TCL_ERROR;
        }
        if n < 0 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"N must be non-negative\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = 3 as ::core::ffi::c_int;
        while i < 5 as ::core::ffi::c_int {
            if Tcl_GetInt(
                interp,
                *argv.offset(i as isize),
                (&raw mut a as *mut ::core::ffi::c_int)
                    .offset((i - 3 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_int,
            ) != 0
            {
                return TCL_ERROR;
            }
            i += 1;
        }
        z = sqlite3_malloc(n + 1 as ::core::ffi::c_int) as *mut ::core::ffi::c_char;
        sqlite3_snprintf(
            n,
            z,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            a[0 as ::core::ffi::c_int as usize],
            a[1 as ::core::ffi::c_int as usize],
            if argc > 4 as ::core::ffi::c_int {
                *argv.offset(5 as ::core::ffi::c_int as isize)
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
        );
        Tcl_AppendResult(interp, z, NULL);
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_mprintf_double(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: [::core::ffi::c_int; 3] = [0; 3];
        let mut i: ::core::ffi::c_int = 0;
        let mut r: ::core::ffi::c_double = 0.;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FORMAT INT INT DOUBLE\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        i = 2 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            if Tcl_GetInt(
                interp,
                *argv.offset(i as isize),
                (&raw mut a as *mut ::core::ffi::c_int)
                    .offset((i - 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_int,
            ) != 0
            {
                return TCL_ERROR;
            }
            i += 1;
        }
        if Tcl_GetDouble(
            interp,
            *argv.offset(4 as ::core::ffi::c_int as isize),
            &raw mut r,
        ) != 0
        {
            return TCL_ERROR;
        }
        z = sqlite3_mprintf(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            a[0 as ::core::ffi::c_int as usize],
            a[1 as ::core::ffi::c_int as usize],
            r,
        );
        Tcl_AppendResult(interp, z, NULL);
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_mprintf_scaled(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut r: [::core::ffi::c_double; 2] = [0.; 2];
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FORMAT DOUBLE DOUBLE\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        i = 2 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            if Tcl_GetDouble(
                interp,
                *argv.offset(i as isize),
                (&raw mut r as *mut ::core::ffi::c_double)
                    .offset((i - 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
            ) != 0
            {
                return TCL_ERROR;
            }
            i += 1;
        }
        z = sqlite3_mprintf(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            r[0 as ::core::ffi::c_int as usize] * r[1 as ::core::ffi::c_int as usize],
        );
        Tcl_AppendResult(interp, z, NULL);
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_mprintf_stronly(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FORMAT STRING\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        z = sqlite3_mprintf(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            *argv.offset(2 as ::core::ffi::c_int as isize),
        );
        Tcl_AppendResult(interp, z, NULL);
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_mprintf_hexdouble(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut r: ::core::ffi::c_double = 0.;
        let mut x1: ::core::ffi::c_uint = 0;
        let mut x2: ::core::ffi::c_uint = 0;
        let mut d: sqlite_uint64 = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FORMAT STRING\"\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if sscanf(
            *argv.offset(2 as ::core::ffi::c_int as isize),
            b"%08x%08x\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut x2,
            &raw mut x1,
        ) != 2 as ::core::ffi::c_int
        {
            Tcl_AppendResult(
                interp,
                b"2nd argument should be 16-characters of hex\0".as_ptr()
                    as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        d = x2 as sqlite_uint64;
        d = ((d as ::core::ffi::c_ulonglong) << 32 as ::core::ffi::c_int)
            .wrapping_add(x1 as ::core::ffi::c_ulonglong) as sqlite_uint64;
        memcpy(
            &raw mut r as *mut ::core::ffi::c_void,
            &raw mut d as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        z = sqlite3_mprintf(*argv.offset(1 as ::core::ffi::c_int as isize), r);
        Tcl_AppendResult(interp, z, NULL);
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_enable_shared(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut enable: ::core::ffi::c_int = 0;
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc != 2 as ::core::ffi::c_int && objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?BOOLEAN?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        ret = sqlite3Config.sharedCacheEnabled;
        if objc == 2 as ::core::ffi::c_int {
            if Tcl_GetBooleanFromObj(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut enable,
            ) != 0
            {
                return TCL_ERROR;
            }
            rc = sqlite3_enable_shared_cache(enable);
            if rc != SQLITE_OK {
                Tcl_SetResult(
                    interp,
                    sqlite3ErrStr(rc) as *mut ::core::ffi::c_char,
                    TCL_STATIC,
                );
                return TCL_ERROR;
            }
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj((ret != 0 as ::core::ffi::c_int) as ::core::ffi::c_int),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_extended_result_codes(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut enable: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetBooleanFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut enable,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_extended_result_codes(db, enable);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_libversion_number(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_libversion_number()));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_table_column_metadata(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zTbl: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zCol: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut zDatatype: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zCollseq: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut notnull: ::core::ffi::c_int = 0;
        let mut primarykey: ::core::ffi::c_int = 0;
        let mut autoincrement: ::core::ffi::c_int = 0;
        if objc != 5 as ::core::ffi::c_int && objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB dbname tblname colname\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zDb = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        zTbl = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        zCol = if objc == 5 as ::core::ffi::c_int {
            Tcl_GetString(*objv.offset(4 as ::core::ffi::c_int as isize))
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        if strlen(zDb) == 0 as size_t {
            zDb = ::core::ptr::null::<::core::ffi::c_char>();
        }
        rc = sqlite3_table_column_metadata(
            db,
            zDb,
            zTbl,
            zCol,
            &raw mut zDatatype,
            &raw mut zCollseq,
            &raw mut notnull,
            &raw mut primarykey,
            &raw mut autoincrement,
        );
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3_errmsg(db), NULL);
            return TCL_ERROR;
        }
        pRet = Tcl_NewObj();
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pRet,
            Tcl_NewStringObj(zDatatype, -1 as ::core::ffi::c_int),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pRet,
            Tcl_NewStringObj(zCollseq, -1 as ::core::ffi::c_int),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pRet,
            Tcl_NewIntObj(notnull),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pRet,
            Tcl_NewIntObj(primarykey),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pRet,
            Tcl_NewIntObj(autoincrement),
        );
        Tcl_SetObjResult(interp, pRet);
        return TCL_OK;
    }
}
unsafe extern "C" fn blobHandleFromObj(
    mut interp: *mut Tcl_Interp,
    mut pObj: *mut Tcl_Obj,
    mut ppBlob: *mut *mut sqlite3_blob,
) -> ::core::ffi::c_int {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut n: ::core::ffi::c_int = 0;
        z = Tcl_GetStringFromObj(pObj, &raw mut n);
        if n == 0 as ::core::ffi::c_int {
            *ppBlob = ::core::ptr::null_mut::<sqlite3_blob>();
        } else {
            let mut notUsed: ::core::ffi::c_int = 0;
            let mut channel: Tcl_Channel = ::core::ptr::null_mut::<Tcl_Channel_>();
            let mut instanceData: ClientData = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            channel = Tcl_GetChannel(interp, z, &raw mut notUsed);
            if channel.is_null() {
                return TCL_ERROR;
            }
            Tcl_Flush(channel);
            Tcl_Seek(channel, 0 as Tcl_WideInt, SEEK_SET);
            instanceData = Tcl_GetChannelInstanceData(channel);
            *ppBlob = *(instanceData as *mut *mut sqlite3_blob);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_blob_reopen(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iRowid: Tcl_WideInt = 0;
        let mut pBlob: *mut sqlite3_blob = ::core::ptr::null_mut::<sqlite3_blob>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"CHANNEL ROWID\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if blobHandleFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut pBlob,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetWideIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut iRowid,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_blob_reopen(pBlob, iRowid as sqlite3_int64);
        if rc != SQLITE_OK {
            Tcl_SetResult(
                interp,
                sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        return if rc == SQLITE_OK { TCL_OK } else { TCL_ERROR };
    }
}
unsafe extern "C" fn testCreateCollationDel(mut pCtx: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut TestCollationX = pCtx as *mut TestCollationX;
        let mut rc: ::core::ffi::c_int = Tcl_EvalObjEx(
            (*p).interp,
            (*p).pDel,
            TCL_EVAL_DIRECT | TCL_EVAL_GLOBAL,
        );
        if rc != TCL_OK {
            Tcl_BackgroundError((*p).interp);
        }
        let mut _objPtr: *mut Tcl_Obj = (*p).pCmp;
        let c2rust_fresh5 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh5 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        let mut _objPtr_0: *mut Tcl_Obj = (*p).pDel;
        let c2rust_fresh6 = (*_objPtr_0).refCount;
        (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
        if c2rust_fresh6 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr_0);
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn testCreateCollationCmp(
    mut pCtx: *mut ::core::ffi::c_void,
    mut nLeft: ::core::ffi::c_int,
    mut zLeft: *const ::core::ffi::c_void,
    mut nRight: ::core::ffi::c_int,
    mut zRight: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut TestCollationX = pCtx as *mut TestCollationX;
        let mut pScript: *mut Tcl_Obj = Tcl_DuplicateObj((*p).pCmp);
        let mut iRes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        (*pScript).refCount += 1;
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pScript,
            Tcl_NewStringObj(zLeft as *mut ::core::ffi::c_char, nLeft),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pScript,
            Tcl_NewStringObj(zRight as *mut ::core::ffi::c_char, nRight),
        );
        if TCL_OK
            != Tcl_EvalObjEx((*p).interp, pScript, TCL_EVAL_DIRECT | TCL_EVAL_GLOBAL)
            || TCL_OK
                != Tcl_GetIntFromObj(
                    (*p).interp,
                    Tcl_GetObjResult((*p).interp),
                    &raw mut iRes,
                )
        {
            Tcl_BackgroundError((*p).interp);
        }
        let mut _objPtr: *mut Tcl_Obj = pScript;
        let c2rust_fresh7 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh7 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        return iRes;
    }
}
unsafe extern "C" fn test_create_collation_v2(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut TestCollationX = ::core::ptr::null_mut::<TestCollationX>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 5 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB-HANDLE NAME CMP-PROC DEL-PROC\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        p = sqlite3_malloc(
            ::core::mem::size_of::<TestCollationX>() as ::core::ffi::c_int,
        ) as *mut TestCollationX;
        (*p).pCmp = *objv.offset(3 as ::core::ffi::c_int as isize);
        (*p).pDel = *objv.offset(4 as ::core::ffi::c_int as isize);
        (*p).interp = interp;
        (*(*p).pCmp).refCount += 1;
        (*(*p).pDel).refCount += 1;
        rc = sqlite3_create_collation_v2(
            db,
            Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
            16 as ::core::ffi::c_int,
            p as *mut ::core::ffi::c_void,
            Some(
                testCreateCollationCmp
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                testCreateCollationDel
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
        if rc != SQLITE_MISUSE {
            Tcl_AppendResult(
                interp,
                b"sqlite3_create_collate_v2() failed to detect an invalid encoding\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        rc = sqlite3_create_collation_v2(
            db,
            Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
            SQLITE_UTF8,
            p as *mut ::core::ffi::c_void,
            Some(
                testCreateCollationCmp
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                testCreateCollationDel
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn cf2Func(
    mut ctx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut aArg: *mut *mut sqlite3_value,
) {}
unsafe extern "C" fn cf2Step(
    mut ctx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut aArg: *mut *mut sqlite3_value,
) {}
unsafe extern "C" fn cf2Final(mut ctx: *mut sqlite3_context) {}
unsafe extern "C" fn cf2Destroy(mut pUser: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut CreateFunctionV2 = pUser as *mut CreateFunctionV2;
        if !(*p).interp.is_null() && !(*p).pDestroy.is_null() {
            let mut rc: ::core::ffi::c_int = Tcl_EvalObjEx(
                (*p).interp,
                (*p).pDestroy,
                0 as ::core::ffi::c_int,
            );
            if rc != TCL_OK {
                Tcl_BackgroundError((*p).interp);
            }
        }
        if !(*p).pFunc.is_null() {
            let mut _objPtr: *mut Tcl_Obj = (*p).pFunc;
            let c2rust_fresh8 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh8 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
        }
        if !(*p).pStep.is_null() {
            let mut _objPtr_0: *mut Tcl_Obj = (*p).pStep;
            let c2rust_fresh9 = (*_objPtr_0).refCount;
            (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
            if c2rust_fresh9 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr_0);
            }
        }
        if !(*p).pFinal.is_null() {
            let mut _objPtr_1: *mut Tcl_Obj = (*p).pFinal;
            let c2rust_fresh10 = (*_objPtr_1).refCount;
            (*_objPtr_1).refCount = (*_objPtr_1).refCount - 1;
            if c2rust_fresh10 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr_1);
            }
        }
        if !(*p).pDestroy.is_null() {
            let mut _objPtr_2: *mut Tcl_Obj = (*p).pDestroy;
            let c2rust_fresh11 = (*_objPtr_2).refCount;
            (*_objPtr_2).refCount = (*_objPtr_2).refCount - 1;
            if c2rust_fresh11 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr_2);
            }
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn test_create_function_v2(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zFunc: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nArg: ::core::ffi::c_int = 0;
        let mut enc: ::core::ffi::c_int = 0;
        let mut p: *mut CreateFunctionV2 = ::core::ptr::null_mut::<CreateFunctionV2>();
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut aEnc: [EncTable; 7] = [
            EncTable {
                zEnc: b"utf8\0".as_ptr() as *const ::core::ffi::c_char,
                enc: SQLITE_UTF8,
            },
            EncTable {
                zEnc: b"utf16\0".as_ptr() as *const ::core::ffi::c_char,
                enc: SQLITE_UTF16,
            },
            EncTable {
                zEnc: b"utf16le\0".as_ptr() as *const ::core::ffi::c_char,
                enc: SQLITE_UTF16LE,
            },
            EncTable {
                zEnc: b"utf16be\0".as_ptr() as *const ::core::ffi::c_char,
                enc: SQLITE_UTF16BE,
            },
            EncTable {
                zEnc: b"any\0".as_ptr() as *const ::core::ffi::c_char,
                enc: SQLITE_ANY,
            },
            EncTable {
                zEnc: b"0\0".as_ptr() as *const ::core::ffi::c_char,
                enc: 0 as ::core::ffi::c_int,
            },
            EncTable {
                zEnc: ::core::ptr::null::<::core::ffi::c_char>(),
                enc: 0 as ::core::ffi::c_int,
            },
        ];
        if objc < 5 as ::core::ffi::c_int
            || objc % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB NAME NARG ENC SWITCHES...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zFunc = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut nArg,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(4 as ::core::ffi::c_int as isize),
            &raw mut aEnc as *mut EncTable as *const ::core::ffi::c_void,
            ::core::mem::size_of::<EncTable>() as ::core::ffi::c_int,
            b"encoding\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut enc,
        ) != 0
        {
            return TCL_ERROR;
        }
        enc = aEnc[enc as usize].enc;
        p = sqlite3_malloc(
            ::core::mem::size_of::<CreateFunctionV2>() as ::core::ffi::c_int,
        ) as *mut CreateFunctionV2;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<CreateFunctionV2>() as size_t,
        );
        (*p).interp = interp;
        i = 5 as ::core::ffi::c_int;
        while i < objc {
            let mut iSwitch: ::core::ffi::c_int = 0;
            let mut azSwitch: [*const ::core::ffi::c_char; 5] = [
                b"-func\0".as_ptr() as *const ::core::ffi::c_char,
                b"-step\0".as_ptr() as *const ::core::ffi::c_char,
                b"-final\0".as_ptr() as *const ::core::ffi::c_char,
                b"-destroy\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ];
            if Tcl_GetIndexFromObjStruct(
                interp,
                *objv.offset(i as isize),
                &raw mut azSwitch as *mut *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as ::core::ffi::c_int,
                b"switch\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                &raw mut iSwitch,
            ) != 0
            {
                sqlite3_free(p as *mut ::core::ffi::c_void);
                return TCL_ERROR;
            }
            match iSwitch {
                0 => {
                    (*p).pFunc = *objv.offset((i + 1 as ::core::ffi::c_int) as isize);
                }
                1 => {
                    (*p).pStep = *objv.offset((i + 1 as ::core::ffi::c_int) as isize);
                }
                2 => {
                    (*p).pFinal = *objv.offset((i + 1 as ::core::ffi::c_int) as isize);
                }
                3 => {
                    (*p).pDestroy = *objv.offset((i + 1 as ::core::ffi::c_int) as isize);
                }
                _ => {}
            }
            i += 2 as ::core::ffi::c_int;
        }
        if !(*p).pFunc.is_null() {
            (*p).pFunc = Tcl_DuplicateObj((*p).pFunc);
        }
        if !(*p).pStep.is_null() {
            (*p).pStep = Tcl_DuplicateObj((*p).pStep);
        }
        if !(*p).pFinal.is_null() {
            (*p).pFinal = Tcl_DuplicateObj((*p).pFinal);
        }
        if !(*p).pDestroy.is_null() {
            (*p).pDestroy = Tcl_DuplicateObj((*p).pDestroy);
        }
        if !(*p).pFunc.is_null() {
            (*(*p).pFunc).refCount += 1;
        }
        if !(*p).pStep.is_null() {
            (*(*p).pStep).refCount += 1;
        }
        if !(*p).pFinal.is_null() {
            (*(*p).pFinal).refCount += 1;
        }
        if !(*p).pDestroy.is_null() {
            (*(*p).pDestroy).refCount += 1;
        }
        rc = sqlite3_create_function_v2(
            db,
            zFunc,
            nArg,
            enc,
            p as *mut ::core::ffi::c_void,
            if !(*p).pFunc.is_null() {
                Some(
                    cf2Func
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                )
            } else {
                None
            },
            if !(*p).pStep.is_null() {
                Some(
                    cf2Step
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                )
            } else {
                None
            },
            if !(*p).pFinal.is_null() {
                Some(cf2Final as unsafe extern "C" fn(*mut sqlite3_context) -> ())
            } else {
                None
            },
            Some(cf2Destroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        if rc != SQLITE_OK {
            Tcl_ResetResult(interp);
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_load_extension(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
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
        let mut rc: ::core::ffi::c_int = 0;
        let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zFile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zProc: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc != 4 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB-HANDLE FILE ?PROC?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zDb = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        zFile = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if objc == 4 as ::core::ffi::c_int {
            zProc = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        }
        if Tcl_GetCommandInfo(interp, zDb, &raw mut cmdInfo) == 0 {
            Tcl_AppendResult(
                interp,
                b"command not found: \0".as_ptr() as *const ::core::ffi::c_char,
                zDb,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        db = (*(cmdInfo.objClientData as *mut SqliteDb)).db;
        rc = sqlite3_load_extension(db, zFile, zProc, &raw mut zErr);
        if rc != SQLITE_OK {
            Tcl_SetResult(
                interp,
                (if !zErr.is_null() {
                    zErr as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                }) as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            rc = TCL_ERROR;
        } else {
            rc = TCL_OK;
        }
        sqlite3_free(zErr as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn test_enable_load(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
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
        let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut onoff: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB-HANDLE ONOFF\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zDb = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        if Tcl_GetCommandInfo(interp, zDb, &raw mut cmdInfo) == 0 {
            Tcl_AppendResult(
                interp,
                b"command not found: \0".as_ptr() as *const ::core::ffi::c_char,
                zDb,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        db = (*(cmdInfo.objClientData as *mut SqliteDb)).db;
        if Tcl_GetBooleanFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut onoff,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_enable_load_extension(db, onoff);
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite_abort(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        exit(255 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn testFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        loop {
            if !(argc >= 2 as ::core::ffi::c_int) {
                c2rust_current_block = 4808432441040389987;
                break;
            }
            let mut zArg0: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            ) as *mut ::core::ffi::c_char;
            if zArg0.is_null() {
                c2rust_current_block = 16070514031714774181;
                break;
            }
            if 0 as ::core::ffi::c_int
                == sqlite3StrICmp(zArg0, b"int\0".as_ptr() as *const ::core::ffi::c_char)
            {
                sqlite3_result_int(
                    context,
                    sqlite3_value_int(*argv.offset(1 as ::core::ffi::c_int as isize)),
                );
            } else if sqlite3StrICmp(
                zArg0,
                b"int64\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                sqlite3_result_int64(
                    context,
                    sqlite3_value_int64(*argv.offset(1 as ::core::ffi::c_int as isize)),
                );
            } else if sqlite3StrICmp(
                zArg0,
                b"string\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                sqlite3_result_text(
                    context,
                    sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            } else if sqlite3StrICmp(
                zArg0,
                b"double\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                sqlite3_result_double(
                    context,
                    sqlite3_value_double(*argv.offset(1 as ::core::ffi::c_int as isize)),
                );
            } else if sqlite3StrICmp(
                zArg0,
                b"null\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                sqlite3_result_null(context);
            } else {
                if !(sqlite3StrICmp(
                    zArg0,
                    b"value\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
                {
                    c2rust_current_block = 16070514031714774181;
                    break;
                }
                sqlite3_result_value(
                    context,
                    *argv
                        .offset(
                            sqlite3_value_int(
                                *argv.offset(1 as ::core::ffi::c_int as isize),
                            ) as isize,
                        ),
                );
            }
            argc -= 2 as ::core::ffi::c_int;
            argv = argv.offset(2 as ::core::ffi::c_int as isize);
        }
        match c2rust_current_block {
            4808432441040389987 => return,
            _ => {
                sqlite3_result_error(
                    context,
                    b"first argument should be one of: int int64 string double null value\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
        };
    }
}
unsafe extern "C" fn test_register_func(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB FUNCTION-NAME\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_create_function(
            db,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            -1 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                testFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
        if rc != 0 as ::core::ffi::c_int {
            Tcl_AppendResult(interp, sqlite3ErrStr(rc), NULL);
            return TCL_ERROR;
        }
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_finalize(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" <STMT>\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if !pStmt.is_null() {
            db = sqlite3_db_handle(pStmt);
        }
        rc = sqlite3_finalize(pStmt);
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        if !db.is_null() && sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_stmt_status(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iValue: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut op: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut resetFlag: ::core::ffi::c_int = 0;
        let mut zOpName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        static mut aOp: [C2Rust_Unnamed_31; 7] = [
            C2Rust_Unnamed_31 {
                zName: b"SQLITE_STMTSTATUS_FULLSCAN_STEP\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STMTSTATUS_FULLSCAN_STEP,
            },
            C2Rust_Unnamed_31 {
                zName: b"SQLITE_STMTSTATUS_SORT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STMTSTATUS_SORT,
            },
            C2Rust_Unnamed_31 {
                zName: b"SQLITE_STMTSTATUS_AUTOINDEX\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STMTSTATUS_AUTOINDEX,
            },
            C2Rust_Unnamed_31 {
                zName: b"SQLITE_STMTSTATUS_VM_STEP\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STMTSTATUS_VM_STEP,
            },
            C2Rust_Unnamed_31 {
                zName: b"SQLITE_STMTSTATUS_REPREPARE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STMTSTATUS_REPREPARE,
            },
            C2Rust_Unnamed_31 {
                zName: b"SQLITE_STMTSTATUS_RUN\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_STMTSTATUS_RUN,
            },
            C2Rust_Unnamed_31 {
                zName: b"SQLITE_STMTSTATUS_MEMUSED\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STMTSTATUS_MEMUSED,
            },
        ];
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT PARAMETER RESETFLAG\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        zOpName = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[C2Rust_Unnamed_31; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_31>() as usize)
                as ::core::ffi::c_int
        {
            if strcmp(aOp[i as usize].zName, zOpName) == 0 as ::core::ffi::c_int {
                op = aOp[i as usize].op;
                break;
            } else {
                i += 1;
            }
        }
        if i
            >= (::core::mem::size_of::<[C2Rust_Unnamed_31; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_31>() as usize)
                as ::core::ffi::c_int
        {
            if Tcl_GetIntFromObj(
                interp,
                *objv.offset(2 as ::core::ffi::c_int as isize),
                &raw mut op,
            ) != 0
            {
                return TCL_ERROR;
            }
        }
        if Tcl_GetBooleanFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut resetFlag,
        ) != 0
        {
            return TCL_ERROR;
        }
        iValue = sqlite3_stmt_status(pStmt, op, resetFlag);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(iValue));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_config_sorterref(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iVal: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"NBYTE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut iVal,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_config(SQLITE_CONFIG_SORTERREF_SIZE, iVal);
        return TCL_OK;
    }
}
unsafe extern "C" fn vfsCurrentTimeInt64(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut t: i64_0 = 0;
        let mut pVfs: *mut sqlite3_vfs = sqlite3_vfs_find(
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        (*pVfs).xCurrentTimeInt64.expect("non-null function pointer")(pVfs, &raw mut t);
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(t as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_create_null_module(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB DBNAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zName = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        sqlite3_create_module(
            db,
            zName,
            ::core::ptr::null::<sqlite3_module>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_delete_database(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut zFile: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zFile = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        rc = sqlite3_delete_database(zFile);
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_atomic_batch_write(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zFile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pFd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut bRes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"PATH\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zFile = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        rc = sqlite3_open(zFile, &raw mut db);
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3_errmsg(db), NULL);
            sqlite3_close(db);
            return TCL_ERROR;
        }
        rc = sqlite3_file_control(
            db,
            b"main\0".as_ptr() as *const ::core::ffi::c_char,
            SQLITE_FCNTL_FILE_POINTER,
            &raw mut pFd as *mut ::core::ffi::c_void,
        );
        dc = (*(*pFd).pMethods)
            .xDeviceCharacteristics
            .expect("non-null function pointer")(pFd);
        if dc & SQLITE_IOCAP_BATCH_ATOMIC != 0 {
            bRes = 1 as ::core::ffi::c_int;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(bRes));
        sqlite3_close(db);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_next_stmt(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB STMT\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        pStmt = sqlite3_next_stmt(db, pStmt);
        if !pStmt.is_null() {
            if sqlite3TestMakePointerStr(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                pStmt as *mut ::core::ffi::c_void,
            ) != 0
            {
                return TCL_ERROR;
            }
            Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_stmt_readonly(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_stmt_readonly(pStmt);
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj((rc != 0 as ::core::ffi::c_int) as ::core::ffi::c_int),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_stmt_isexplain(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_stmt_isexplain(pStmt);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_stmt_explain(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut eMode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT INT\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut eMode,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_stmt_explain(pStmt, eMode);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_stmt_busy(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_stmt_busy(pStmt);
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj((rc != 0 as ::core::ffi::c_int) as ::core::ffi::c_int),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn uses_stmt_journal(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_stmt_readonly(pStmt);
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj(
                ((*(pStmt as *mut Vdbe)).usesStmtJournal() as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_reset(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" <STMT>\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_reset(pStmt);
        if !pStmt.is_null()
            && sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_expired(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" <STMT>\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj(
                (sqlite3_expired(pStmt) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_transfer_bind(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt1: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut pStmt2: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" FROM-STMT TO-STMT\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt1,
        ) != 0
        {
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
            &raw mut pStmt2,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj(sqlite3_transfer_bindings(pStmt1, pStmt2)),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_changes(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_changes(db)));
        return TCL_OK;
    }
}
static mut sqlite_static_bind_value: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
static mut sqlite_static_bind_nbyte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn test_bind(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut idx: ::core::ffi::c_int = 0;
        if argc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" VM IDX VALUE (null|static|normal)\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetInt(
            interp,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        if strcmp(
            *argv.offset(4 as ::core::ffi::c_int as isize),
            b"null\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            rc = sqlite3_bind_null(pStmt, idx);
        } else if strcmp(
            *argv.offset(4 as ::core::ffi::c_int as isize),
            b"static\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            rc = sqlite3_bind_text(
                pStmt,
                idx,
                sqlite_static_bind_value,
                -1 as ::core::ffi::c_int,
                None,
            );
        } else if strcmp(
            *argv.offset(4 as ::core::ffi::c_int as isize),
            b"static-nbytes\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            rc = sqlite3_bind_text(
                pStmt,
                idx,
                sqlite_static_bind_value,
                sqlite_static_bind_nbyte,
                None,
            );
        } else if strcmp(
            *argv.offset(4 as ::core::ffi::c_int as isize),
            b"normal\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            rc = sqlite3_bind_text(
                pStmt,
                idx,
                *argv.offset(3 as ::core::ffi::c_int as isize),
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else if strcmp(
            *argv.offset(4 as ::core::ffi::c_int as isize),
            b"blob10\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            rc = sqlite3_bind_text(
                pStmt,
                idx,
                b"abc\0xyz\0pq\0".as_ptr() as *const ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
        } else {
            Tcl_AppendResult(
                interp,
                b"4th argument should be \"null\" or \"static\" or \"normal\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0 {
            return TCL_ERROR;
        }
        if rc != 0 {
            let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"(%d) \0".as_ptr() as *const ::core::ffi::c_char,
                rc,
            );
            Tcl_AppendResult(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                sqlite3ErrStr(rc),
                NULL,
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
static mut pTestCollateInterp: *mut Tcl_Interp = ::core::ptr::null::<Tcl_Interp>()
    as *mut Tcl_Interp;
unsafe extern "C" fn test_collate_func(
    mut pCtx: *mut ::core::ffi::c_void,
    mut nA: ::core::ffi::c_int,
    mut zA: *const ::core::ffi::c_void,
    mut nB: ::core::ffi::c_int,
    mut zB: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: *mut Tcl_Interp = pTestCollateInterp;
        let mut encin: ::core::ffi::c_int = pCtx as intptr_t as ::core::ffi::c_int;
        let mut res: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut pX: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        pX = Tcl_NewStringObj(
            b"test_collate\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
        );
        (*pX).refCount += 1;
        match encin {
            SQLITE_UTF8 => {
                Tcl_ListObjAppendElement(
                    i,
                    pX,
                    Tcl_NewStringObj(
                        b"UTF-8\0".as_ptr() as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    ),
                );
            }
            SQLITE_UTF16LE => {
                Tcl_ListObjAppendElement(
                    i,
                    pX,
                    Tcl_NewStringObj(
                        b"UTF-16LE\0".as_ptr() as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    ),
                );
            }
            SQLITE_UTF16BE => {
                Tcl_ListObjAppendElement(
                    i,
                    pX,
                    Tcl_NewStringObj(
                        b"UTF-16BE\0".as_ptr() as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    ),
                );
            }
            _ => {}
        }
        sqlite3BeginBenignMalloc();
        pVal = sqlite3ValueNew(::core::ptr::null_mut::<sqlite3>());
        if !pVal.is_null() {
            sqlite3ValueSetStr(pVal, nA, zA, encin as u8_0, SQLITE_STATIC);
            n = sqlite3_value_bytes(pVal);
            Tcl_ListObjAppendElement(
                i,
                pX,
                Tcl_NewStringObj(sqlite3_value_text(pVal) as *mut ::core::ffi::c_char, n),
            );
            sqlite3ValueSetStr(pVal, nB, zB, encin as u8_0, SQLITE_STATIC);
            n = sqlite3_value_bytes(pVal);
            Tcl_ListObjAppendElement(
                i,
                pX,
                Tcl_NewStringObj(sqlite3_value_text(pVal) as *mut ::core::ffi::c_char, n),
            );
            sqlite3ValueFree(pVal);
        }
        sqlite3EndBenignMalloc();
        Tcl_EvalObjEx(i, pX, 0 as ::core::ffi::c_int);
        let mut _objPtr: *mut Tcl_Obj = pX;
        let c2rust_fresh12 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh12 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        Tcl_GetIntFromObj(i, Tcl_GetObjResult(i), &raw mut res);
        return res;
    }
}
unsafe extern "C" fn test_collate(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut val: ::core::ffi::c_int = 0;
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" <DB> <utf8> <utf16le> <utf16be>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        } else {
            pTestCollateInterp = interp;
            if getDbPointer(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut db,
            ) != 0
            {
                return TCL_ERROR;
            }
            if TCL_OK
                != Tcl_GetBooleanFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut val,
                )
            {
                return TCL_ERROR;
            }
            rc = sqlite3_create_collation(
                db,
                b"test_collate\0".as_ptr() as *const ::core::ffi::c_char,
                SQLITE_UTF8,
                SQLITE_UTF8 as *mut ::core::ffi::c_void,
                if val != 0 {
                    Some(
                        test_collate_func
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_void,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_void,
                            ) -> ::core::ffi::c_int,
                    )
                } else {
                    None
                },
            );
            if rc == SQLITE_OK {
                let mut zUtf16: *const ::core::ffi::c_void = ::core::ptr::null::<
                    ::core::ffi::c_void,
                >();
                if TCL_OK
                    != Tcl_GetBooleanFromObj(
                        interp,
                        *objv.offset(3 as ::core::ffi::c_int as isize),
                        &raw mut val,
                    )
                {
                    return TCL_ERROR;
                }
                rc = sqlite3_create_collation(
                    db,
                    b"test_collate\0".as_ptr() as *const ::core::ffi::c_char,
                    SQLITE_UTF16LE,
                    SQLITE_UTF16LE as *mut ::core::ffi::c_void,
                    if val != 0 {
                        Some(
                            test_collate_func
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *const ::core::ffi::c_void,
                                ) -> ::core::ffi::c_int,
                        )
                    } else {
                        None
                    },
                );
                if TCL_OK
                    != Tcl_GetBooleanFromObj(
                        interp,
                        *objv.offset(4 as ::core::ffi::c_int as isize),
                        &raw mut val,
                    )
                {
                    return TCL_ERROR;
                }
                sqlite3_mutex_enter((*db).mutex);
                pVal = sqlite3ValueNew(db);
                sqlite3ValueSetStr(
                    pVal,
                    -1 as ::core::ffi::c_int,
                    b"test_collate\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    SQLITE_UTF8 as u8_0,
                    SQLITE_STATIC,
                );
                zUtf16 = sqlite3ValueText(pVal, SQLITE_UTF16NATIVE as u8_0);
                if (*db).mallocFailed != 0 {
                    rc = SQLITE_NOMEM;
                } else {
                    rc = sqlite3_create_collation16(
                        db,
                        zUtf16,
                        SQLITE_UTF16BE,
                        SQLITE_UTF16BE as *mut ::core::ffi::c_void,
                        if val != 0 {
                            Some(
                                test_collate_func
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        ::core::ffi::c_int,
                                        *const ::core::ffi::c_void,
                                        ::core::ffi::c_int,
                                        *const ::core::ffi::c_void,
                                    ) -> ::core::ffi::c_int,
                            )
                        } else {
                            None
                        },
                    );
                }
                sqlite3ValueFree(pVal);
                sqlite3_mutex_leave((*db).mutex);
            }
            if sqlite3TestErrCode(interp, db, rc) != 0 {
                return TCL_ERROR;
            }
            if rc != SQLITE_OK {
                Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
                return TCL_ERROR;
            }
            return TCL_OK;
        };
    }
}
unsafe extern "C" fn test_utf16bin_collate_func(
    mut pCtx: *mut ::core::ffi::c_void,
    mut nA: ::core::ffi::c_int,
    mut zA: *const ::core::ffi::c_void,
    mut nB: ::core::ffi::c_int,
    mut zB: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nCmp: ::core::ffi::c_int = if nA > nB { nB } else { nA };
        let mut res: ::core::ffi::c_int = memcmp(zA, zB, nCmp as size_t);
        if res == 0 as ::core::ffi::c_int {
            res = nA - nB;
        }
        return res;
    }
}
unsafe extern "C" fn test_utf16bin_collate(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        } else {
            if getDbPointer(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut db,
            ) != 0
            {
                return TCL_ERROR;
            }
            rc = sqlite3_create_collation(
                db,
                b"utf16bin\0".as_ptr() as *const ::core::ffi::c_char,
                SQLITE_UTF16,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    test_utf16bin_collate_func
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
            if sqlite3TestErrCode(interp, db, rc) != 0 {
                return TCL_ERROR;
            }
            return TCL_OK;
        };
    }
}
static mut zNeededCollation: [::core::ffi::c_char; 200] = [0; 200];
static mut pzNeededCollation: *mut ::core::ffi::c_char = unsafe {
    &raw const zNeededCollation as *mut ::core::ffi::c_char
};
unsafe extern "C" fn test_collate_needed_cb(
    mut pCtx: *mut ::core::ffi::c_void,
    mut db: *mut sqlite3,
    mut eTextRep: ::core::ffi::c_int,
    mut pName: *const ::core::ffi::c_void,
) {
    unsafe {
        let mut enc: ::core::ffi::c_int = (*db).enc as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        z = pName as *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while *z as ::core::ffi::c_int != 0
            || *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            if *z != 0 {
                let c2rust_fresh13 = i;
                i = i + 1;
                zNeededCollation[c2rust_fresh13 as usize] = *z;
            }
            z = z.offset(1);
        }
        zNeededCollation[i as usize] = 0 as ::core::ffi::c_char;
        sqlite3_create_collation(
            db,
            b"test_collate\0".as_ptr() as *const ::core::ffi::c_char,
            (*db).enc as ::core::ffi::c_int,
            enc as intptr_t as *mut ::core::ffi::c_void,
            Some(
                test_collate_func
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
}
unsafe extern "C" fn test_collate_needed(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        } else {
            if getDbPointer(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut db,
            ) != 0
            {
                return TCL_ERROR;
            }
            rc = sqlite3_collation_needed16(
                db,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    test_collate_needed_cb
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut sqlite3,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                        ) -> (),
                ),
            );
            zNeededCollation[0 as ::core::ffi::c_int as usize] = 0
                as ::core::ffi::c_char;
            if sqlite3TestErrCode(interp, db, rc) != 0 {
                return TCL_ERROR;
            }
            return TCL_OK;
        };
    }
}
static mut unaligned_string_counter: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn alignmentCollFunc(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut nKey1: ::core::ffi::c_int,
    mut pKey1: *const ::core::ffi::c_void,
    mut nKey2: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        n = if nKey1 < nKey2 { nKey1 } else { nKey2 };
        if nKey1 > 0 as ::core::ffi::c_int
            && 1 as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int & pKey1 as intptr_t as ::core::ffi::c_int
        {
            unaligned_string_counter += 1;
        }
        if nKey2 > 0 as ::core::ffi::c_int
            && 1 as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int & pKey2 as intptr_t as ::core::ffi::c_int
        {
            unaligned_string_counter += 1;
        }
        rc = memcmp(pKey1, pKey2, n as size_t);
        if rc == 0 as ::core::ffi::c_int {
            rc = nKey1 - nKey2;
        }
        return rc;
    }
}
unsafe extern "C" fn add_alignment_test_collations(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc >= 2 as ::core::ffi::c_int {
            if getDbPointer(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut db,
            ) != 0
            {
                return TCL_ERROR;
            }
            sqlite3_create_collation(
                db,
                b"utf16_unaligned\0".as_ptr() as *const ::core::ffi::c_char,
                SQLITE_UTF16,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    alignmentCollFunc
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
            sqlite3_create_collation(
                db,
                b"utf16_aligned\0".as_ptr() as *const ::core::ffi::c_char,
                SQLITE_UTF16_ALIGNED,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    alignmentCollFunc
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
        return SQLITE_OK;
    }
}
unsafe extern "C" fn test_function_utf8(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut interp: *mut Tcl_Interp = ::core::ptr::null_mut::<Tcl_Interp>();
        let mut pX: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        interp = sqlite3_user_data(pCtx) as *mut Tcl_Interp;
        pX = Tcl_NewStringObj(
            b"test_function\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
        );
        (*pX).refCount += 1;
        Tcl_ListObjAppendElement(
            interp,
            pX,
            Tcl_NewStringObj(
                b"UTF-8\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(
            interp,
            pX,
            Tcl_NewStringObj(
                sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_EvalObjEx(interp, pX, 0 as ::core::ffi::c_int);
        let mut _objPtr: *mut Tcl_Obj = pX;
        let c2rust_fresh14 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh14 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        sqlite3_result_text(
            pCtx,
            Tcl_GetStringResult(interp),
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        pVal = sqlite3ValueNew(::core::ptr::null_mut::<sqlite3>());
        sqlite3ValueSetStr(
            pVal,
            -1 as ::core::ffi::c_int,
            Tcl_GetStringResult(interp) as *const ::core::ffi::c_void,
            SQLITE_UTF8 as u8_0,
            SQLITE_STATIC,
        );
        sqlite3_result_text16be(
            pCtx,
            sqlite3_value_text16be(pVal),
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3ValueFree(pVal);
    }
}
unsafe extern "C" fn test_function_utf16le(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut interp: *mut Tcl_Interp = ::core::ptr::null_mut::<Tcl_Interp>();
        let mut pX: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        interp = sqlite3_user_data(pCtx) as *mut Tcl_Interp;
        pX = Tcl_NewStringObj(
            b"test_function\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
        );
        (*pX).refCount += 1;
        Tcl_ListObjAppendElement(
            interp,
            pX,
            Tcl_NewStringObj(
                b"UTF-16LE\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(
            interp,
            pX,
            Tcl_NewStringObj(
                sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_EvalObjEx(interp, pX, 0 as ::core::ffi::c_int);
        let mut _objPtr: *mut Tcl_Obj = pX;
        let c2rust_fresh15 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh15 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        pVal = sqlite3ValueNew(::core::ptr::null_mut::<sqlite3>());
        sqlite3ValueSetStr(
            pVal,
            -1 as ::core::ffi::c_int,
            Tcl_GetStringResult(interp) as *const ::core::ffi::c_void,
            SQLITE_UTF8 as u8_0,
            SQLITE_STATIC,
        );
        sqlite3_result_text(
            pCtx,
            sqlite3_value_text(pVal) as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3ValueFree(pVal);
    }
}
unsafe extern "C" fn test_function_utf16be(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut interp: *mut Tcl_Interp = ::core::ptr::null_mut::<Tcl_Interp>();
        let mut pX: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        interp = sqlite3_user_data(pCtx) as *mut Tcl_Interp;
        pX = Tcl_NewStringObj(
            b"test_function\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
        );
        (*pX).refCount += 1;
        Tcl_ListObjAppendElement(
            interp,
            pX,
            Tcl_NewStringObj(
                b"UTF-16BE\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(
            interp,
            pX,
            Tcl_NewStringObj(
                sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                    as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_EvalObjEx(interp, pX, 0 as ::core::ffi::c_int);
        let mut _objPtr: *mut Tcl_Obj = pX;
        let c2rust_fresh16 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh16 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        pVal = sqlite3ValueNew(::core::ptr::null_mut::<sqlite3>());
        sqlite3ValueSetStr(
            pVal,
            -1 as ::core::ffi::c_int,
            Tcl_GetStringResult(interp) as *const ::core::ffi::c_void,
            SQLITE_UTF8 as u8_0,
            SQLITE_STATIC,
        );
        sqlite3_result_text16(
            pCtx,
            sqlite3_value_text16le(pVal),
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_result_text16be(
            pCtx,
            sqlite3_value_text16le(pVal),
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_result_text16le(
            pCtx,
            sqlite3_value_text16le(pVal),
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3ValueFree(pVal);
    }
}
unsafe extern "C" fn test_function(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut val: ::core::ffi::c_int = 0;
        if objc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" <DB> <utf8> <utf16le> <utf16be>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        } else {
            if getDbPointer(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut db,
            ) != 0
            {
                return TCL_ERROR;
            }
            if TCL_OK
                != Tcl_GetBooleanFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut val,
                )
            {
                return TCL_ERROR;
            }
            if val != 0 {
                sqlite3_create_function(
                    db,
                    b"test_function\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    SQLITE_UTF8,
                    interp as *mut ::core::ffi::c_void,
                    Some(
                        test_function_utf8
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
            if TCL_OK
                != Tcl_GetBooleanFromObj(
                    interp,
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut val,
                )
            {
                return TCL_ERROR;
            }
            if val != 0 {
                sqlite3_create_function(
                    db,
                    b"test_function\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    SQLITE_UTF16LE,
                    interp as *mut ::core::ffi::c_void,
                    Some(
                        test_function_utf16le
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
            if TCL_OK
                != Tcl_GetBooleanFromObj(
                    interp,
                    *objv.offset(4 as ::core::ffi::c_int as isize),
                    &raw mut val,
                )
            {
                return TCL_ERROR;
            }
            if val != 0 {
                sqlite3_create_function(
                    db,
                    b"test_function\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    SQLITE_UTF16BE,
                    interp as *mut ::core::ffi::c_void,
                    Some(
                        test_function_utf16be
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
            return TCL_OK;
        };
    }
}
unsafe extern "C" fn test_errstr(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zCode: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"<error code>\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        zCode = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        i = 0 as ::core::ffi::c_int;
        while i < 200 as ::core::ffi::c_int {
            if 0 as ::core::ffi::c_int == strcmp(sqlite3ErrName(i), zCode) {
                break;
            }
            i += 1;
        }
        Tcl_SetResult(interp, sqlite3ErrStr(i) as *mut ::core::ffi::c_char, None);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_breakpoint(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_zeroblob(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut idx: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT IDX N\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut n,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_bind_zeroblob(pStmt, idx, n);
        if sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0 {
            return TCL_ERROR;
        }
        if rc != SQLITE_OK {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_zeroblob64(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut idx: ::core::ffi::c_int = 0;
        let mut n: Tcl_WideInt = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT IDX N\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetWideIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut n,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_bind_zeroblob64(pStmt, idx, n as sqlite3_uint64);
        if sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0 {
            return TCL_ERROR;
        }
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_int(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut idx: ::core::ffi::c_int = 0;
        let mut value: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT N VALUE\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut value,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_bind_int(pStmt, idx, value);
        if sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0 {
            return TCL_ERROR;
        }
        if rc != SQLITE_OK {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_intarray_addr(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        static mut p: *mut ::core::ffi::c_int = ::core::ptr::null::<::core::ffi::c_int>()
            as *mut ::core::ffi::c_int;
        sqlite3_free(p as *mut ::core::ffi::c_void);
        p = ::core::ptr::null_mut::<::core::ffi::c_int>();
        if objc > 1 as ::core::ffi::c_int {
            p = sqlite3_malloc(
                (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    .wrapping_mul((objc - 1 as ::core::ffi::c_int) as usize)
                    as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_int;
            if p.is_null() {
                return TCL_ERROR;
            }
            i = 0 as ::core::ffi::c_int;
            while i < objc - 1 as ::core::ffi::c_int {
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset((1 as ::core::ffi::c_int + i) as isize),
                    p.offset(i as isize) as *mut ::core::ffi::c_int,
                ) != 0
                {
                    sqlite3_free(p as *mut ::core::ffi::c_void);
                    p = ::core::ptr::null_mut::<::core::ffi::c_int>();
                    return TCL_ERROR;
                }
                i += 1;
            }
        }
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(p as uptr as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_int64array_addr(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        static mut p: *mut sqlite3_int64 = ::core::ptr::null::<sqlite3_int64>()
            as *mut sqlite3_int64;
        sqlite3_free(p as *mut ::core::ffi::c_void);
        p = ::core::ptr::null_mut::<sqlite3_int64>();
        if objc > 1 as ::core::ffi::c_int {
            p = sqlite3_malloc(
                (::core::mem::size_of::<sqlite3_int64>() as usize)
                    .wrapping_mul((objc - 1 as ::core::ffi::c_int) as usize)
                    as ::core::ffi::c_int,
            ) as *mut sqlite3_int64;
            if p.is_null() {
                return TCL_ERROR;
            }
            i = 0 as ::core::ffi::c_int;
            while i < objc - 1 as ::core::ffi::c_int {
                let mut v: Tcl_WideInt = 0;
                if Tcl_GetWideIntFromObj(
                    interp,
                    *objv.offset((1 as ::core::ffi::c_int + i) as isize),
                    &raw mut v,
                ) != 0
                {
                    sqlite3_free(p as *mut ::core::ffi::c_void);
                    p = ::core::ptr::null_mut::<sqlite3_int64>();
                    return TCL_ERROR;
                }
                *p.offset(i as isize) = v as sqlite3_int64;
                i += 1;
            }
        }
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(p as uptr as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_doublearray_addr(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        static mut p: *mut ::core::ffi::c_double = ::core::ptr::null::<
            ::core::ffi::c_double,
        >() as *mut ::core::ffi::c_double;
        sqlite3_free(p as *mut ::core::ffi::c_void);
        p = ::core::ptr::null_mut::<::core::ffi::c_double>();
        if objc > 1 as ::core::ffi::c_int {
            p = sqlite3_malloc(
                (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                    .wrapping_mul((objc - 1 as ::core::ffi::c_int) as usize)
                    as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_double;
            if p.is_null() {
                return TCL_ERROR;
            }
            i = 0 as ::core::ffi::c_int;
            while i < objc - 1 as ::core::ffi::c_int {
                if Tcl_GetDoubleFromObj(
                    interp,
                    *objv.offset((1 as ::core::ffi::c_int + i) as isize),
                    p.offset(i as isize) as *mut ::core::ffi::c_double,
                ) != 0
                {
                    sqlite3_free(p as *mut ::core::ffi::c_void);
                    p = ::core::ptr::null_mut::<::core::ffi::c_double>();
                    return TCL_ERROR;
                }
                i += 1;
            }
        }
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(p as uptr as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_textarray_addr(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        static mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        static mut p: *mut *mut ::core::ffi::c_char = ::core::ptr::null::<
            *mut ::core::ffi::c_char,
        >() as *mut *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            sqlite3_free(*p.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
        p = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        if objc > 1 as ::core::ffi::c_int {
            p = sqlite3_malloc(
                (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                    .wrapping_mul((objc - 1 as ::core::ffi::c_int) as usize)
                    as ::core::ffi::c_int,
            ) as *mut *mut ::core::ffi::c_char;
            if p.is_null() {
                return TCL_ERROR;
            }
            i = 0 as ::core::ffi::c_int;
            while i < objc - 1 as ::core::ffi::c_int {
                let ref mut c2rust_fresh17 = *p.offset(i as isize);
                *c2rust_fresh17 = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    Tcl_GetString(*objv.offset((1 as ::core::ffi::c_int + i) as isize)),
                );
                i += 1;
            }
        }
        n = objc - 1 as ::core::ffi::c_int;
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(p as uptr as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_int64(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut idx: ::core::ffi::c_int = 0;
        let mut value: Tcl_WideInt = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT N VALUE\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetWideIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut value,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_bind_int64(pStmt, idx, value as sqlite3_int64);
        if sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0 {
            return TCL_ERROR;
        }
        if rc != SQLITE_OK {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_double(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut idx: ::core::ffi::c_int = 0;
        let mut value: ::core::ffi::c_double = 0 as ::core::ffi::c_int
            as ::core::ffi::c_double;
        let mut rc: ::core::ffi::c_int = 0;
        let mut zVal: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        static mut aSpecialFp: [C2Rust_Unnamed_32; 10] = [
            C2Rust_Unnamed_32 {
                zName: b"NaN\0".as_ptr() as *const ::core::ffi::c_char,
                iUpper: 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
                iLower: 0xffffffff as ::core::ffi::c_uint,
            },
            C2Rust_Unnamed_32 {
                zName: b"SNaN\0".as_ptr() as *const ::core::ffi::c_char,
                iUpper: 0x7ff7ffff as ::core::ffi::c_int as ::core::ffi::c_uint,
                iLower: 0xffffffff as ::core::ffi::c_uint,
            },
            C2Rust_Unnamed_32 {
                zName: b"-NaN\0".as_ptr() as *const ::core::ffi::c_char,
                iUpper: 0xffffffff as ::core::ffi::c_uint,
                iLower: 0xffffffff as ::core::ffi::c_uint,
            },
            C2Rust_Unnamed_32 {
                zName: b"-SNaN\0".as_ptr() as *const ::core::ffi::c_char,
                iUpper: 0xfff7ffff as ::core::ffi::c_uint,
                iLower: 0xffffffff as ::core::ffi::c_uint,
            },
            C2Rust_Unnamed_32 {
                zName: b"+Inf\0".as_ptr() as *const ::core::ffi::c_char,
                iUpper: 0x7ff00000 as ::core::ffi::c_int as ::core::ffi::c_uint,
                iLower: 0 as ::core::ffi::c_uint,
            },
            C2Rust_Unnamed_32 {
                zName: b"-Inf\0".as_ptr() as *const ::core::ffi::c_char,
                iUpper: 0xfff00000 as ::core::ffi::c_uint,
                iLower: 0 as ::core::ffi::c_uint,
            },
            C2Rust_Unnamed_32 {
                zName: b"Epsilon\0".as_ptr() as *const ::core::ffi::c_char,
                iUpper: 0 as ::core::ffi::c_uint,
                iLower: 0x1 as ::core::ffi::c_uint,
            },
            C2Rust_Unnamed_32 {
                zName: b"-Epsilon\0".as_ptr() as *const ::core::ffi::c_char,
                iUpper: 0x80000000 as ::core::ffi::c_uint,
                iLower: 0x1 as ::core::ffi::c_uint,
            },
            C2Rust_Unnamed_32 {
                zName: b"NaN0\0".as_ptr() as *const ::core::ffi::c_char,
                iUpper: 0x7ff80000 as ::core::ffi::c_int as ::core::ffi::c_uint,
                iLower: 0 as ::core::ffi::c_uint,
            },
            C2Rust_Unnamed_32 {
                zName: b"-NaN0\0".as_ptr() as *const ::core::ffi::c_char,
                iUpper: 0xfff80000 as ::core::ffi::c_uint,
                iLower: 0 as ::core::ffi::c_uint,
            },
        ];
        if objc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT N VALUE\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        zVal = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_32; 10]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_32>() as usize)
        {
            if strcmp(aSpecialFp[i as usize].zName, zVal) == 0 as ::core::ffi::c_int {
                let mut x: sqlite3_uint64 = 0;
                x = aSpecialFp[i as usize].iUpper as sqlite3_uint64;
                x <<= 32 as ::core::ffi::c_int;
                x |= aSpecialFp[i as usize].iLower as ::core::ffi::c_ulonglong;
                memcpy(
                    &raw mut value as *mut ::core::ffi::c_void,
                    &raw mut x as *const ::core::ffi::c_void,
                    8 as size_t,
                );
                break;
            } else {
                i += 1;
            }
        }
        if i as usize
            >= (::core::mem::size_of::<[C2Rust_Unnamed_32; 10]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_32>() as usize)
            && Tcl_GetDoubleFromObj(
                interp,
                *objv.offset(3 as ::core::ffi::c_int as isize),
                &raw mut value,
            ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_bind_double(pStmt, idx, value);
        if sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0 {
            return TCL_ERROR;
        }
        if rc != SQLITE_OK {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_null(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut idx: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT N\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_bind_null(pStmt, idx);
        if sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0 {
            return TCL_ERROR;
        }
        if rc != SQLITE_OK {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_text(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut idx: ::core::ffi::c_int = 0;
        let mut trueLength: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bytes: ::core::ffi::c_int = 0;
        let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut toFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT N VALUE BYTES\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        value = Tcl_GetByteArrayFromObj(
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut trueLength,
        ) as *mut ::core::ffi::c_char;
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(4 as ::core::ffi::c_int as isize),
            &raw mut bytes,
        ) != 0
        {
            return TCL_ERROR;
        }
        if bytes < 0 as ::core::ffi::c_int {
            toFree = malloc((trueLength + 1 as ::core::ffi::c_int) as size_t)
                as *mut ::core::ffi::c_char;
            if toFree.is_null() {
                Tcl_AppendResult(
                    interp,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
                return TCL_ERROR;
            }
            memcpy(
                toFree as *mut ::core::ffi::c_void,
                value as *const ::core::ffi::c_void,
                trueLength as size_t,
            );
            *toFree.offset(trueLength as isize) = 0 as ::core::ffi::c_char;
            value = toFree;
        }
        rc = sqlite3_bind_text(
            pStmt,
            idx,
            value,
            bytes,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        free(toFree as *mut ::core::ffi::c_void);
        if sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0 {
            return TCL_ERROR;
        }
        if rc != SQLITE_OK {
            Tcl_AppendResult(
                interp,
                sqlite3ErrName(rc),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_text16(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut idx: ::core::ffi::c_int = 0;
        let mut bytes: ::core::ffi::c_int = 0;
        let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut toFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut trueLength: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = if objc
            == 6 as ::core::ffi::c_int
        {
            SQLITE_STATIC
        } else {
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t)
        };
        let mut oStmt: *mut Tcl_Obj = *objv
            .offset((objc - 4 as ::core::ffi::c_int) as isize);
        let mut oN: *mut Tcl_Obj = *objv
            .offset((objc - 3 as ::core::ffi::c_int) as isize);
        let mut oString: *mut Tcl_Obj = *objv
            .offset((objc - 2 as ::core::ffi::c_int) as isize);
        let mut oBytes: *mut Tcl_Obj = *objv
            .offset((objc - 1 as ::core::ffi::c_int) as isize);
        if objc != 5 as ::core::ffi::c_int && objc != 6 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT N VALUE BYTES\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(interp, Tcl_GetString(oStmt), &raw mut pStmt) != 0 {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(interp, oN, &raw mut idx) != 0 {
            return TCL_ERROR;
        }
        value = Tcl_GetByteArrayFromObj(oString, &raw mut trueLength)
            as *mut ::core::ffi::c_char;
        if Tcl_GetIntFromObj(interp, oBytes, &raw mut bytes) != 0 {
            return TCL_ERROR;
        }
        if bytes < 0 as ::core::ffi::c_int
            && xDel
                == ::core::mem::transmute::<
                    ::libc::intptr_t,
                    sqlite3_destructor_type,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t)
        {
            toFree = malloc((trueLength + 3 as ::core::ffi::c_int) as size_t)
                as *mut ::core::ffi::c_char;
            if toFree.is_null() {
                Tcl_AppendResult(
                    interp,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
                return TCL_ERROR;
            }
            memcpy(
                toFree as *mut ::core::ffi::c_void,
                value as *const ::core::ffi::c_void,
                trueLength as size_t,
            );
            memset(
                toFree.offset(trueLength as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                3 as size_t,
            );
            value = toFree;
        }
        rc = sqlite3_bind_text16(
            pStmt,
            idx,
            value as *mut ::core::ffi::c_void,
            bytes,
            xDel,
        );
        free(toFree as *mut ::core::ffi::c_void);
        if sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0 {
            return TCL_ERROR;
        }
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_blob(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut len: ::core::ffi::c_int = 0;
        let mut idx: ::core::ffi::c_int = 0;
        let mut bytes: ::core::ffi::c_int = 0;
        let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut xDestructor: sqlite3_destructor_type = ::core::mem::transmute::<
            ::libc::intptr_t,
            sqlite3_destructor_type,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t);
        if objc != 5 as ::core::ffi::c_int && objc != 6 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" STMT N DATA BYTES\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if objc == 6 as ::core::ffi::c_int {
            xDestructor = SQLITE_STATIC;
            objv = objv.offset(1);
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        value = Tcl_GetByteArrayFromObj(
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut len,
        ) as *mut ::core::ffi::c_char;
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(4 as ::core::ffi::c_int as isize),
            &raw mut bytes,
        ) != 0
        {
            return TCL_ERROR;
        }
        if bytes > len {
            let mut zBuf: [::core::ffi::c_char; 200] = [0; 200];
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 200]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"cannot use %d blob bytes, have %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                bytes,
                len,
            );
            Tcl_AppendResult(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        rc = sqlite3_bind_blob(
            pStmt,
            idx,
            value as *const ::core::ffi::c_void,
            bytes,
            xDestructor as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        );
        if sqlite3TestErrCode(interp, sqlite3_db_handle(pStmt), rc) != 0 {
            return TCL_ERROR;
        }
        if rc != SQLITE_OK {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_value_from_preupdate(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut idx: ::core::ffi::c_int = 0;
        let mut bidx: ::core::ffi::c_int = 0;
        let mut z3: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        if objc != 5 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT N NEW|OLD IDX\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(4 as ::core::ffi::c_int as isize),
            &raw mut bidx,
        ) != 0
        {
            return TCL_ERROR;
        }
        z3 = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        db = sqlite3_db_handle(pStmt);
        if *z3.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'n' as i32
        {
            sqlite3_preupdate_new(db, bidx, &raw mut pVal);
        } else if *z3.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'o' as i32
        {
            sqlite3_preupdate_old(db, bidx, &raw mut pVal);
        } else {
            Tcl_AppendResult(
                interp,
                b"expected new or old, got: \0".as_ptr() as *const ::core::ffi::c_char,
                z3,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        sqlite3_bind_value(pStmt, idx, pVal);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_value_from_select(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut pStmt2: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut idx: ::core::ffi::c_int = 0;
        let mut zSql: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT N SELECT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut idx,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSql = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        db = sqlite3_db_handle(pStmt);
        rc = sqlite3_prepare_v2(
            db,
            zSql,
            -1 as ::core::ffi::c_int,
            &raw mut pStmt2,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            Tcl_AppendResult(
                interp,
                b"error in SQL: \0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        if sqlite3_step(pStmt2) == SQLITE_ROW {
            let mut pVal: *mut sqlite3_value = sqlite3_column_value(
                pStmt2,
                0 as ::core::ffi::c_int,
            );
            sqlite3_bind_value(pStmt, idx, pVal);
        }
        rc = sqlite3_finalize(pStmt2);
        if rc != SQLITE_OK {
            Tcl_AppendResult(
                interp,
                b"error runnning SQL: \0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn testCarrayAlloc(
    mut n: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pRet: *mut u8_0 = sqlite3_malloc(n + 16 as ::core::ffi::c_int)
            as *mut u8_0;
        if !pRet.is_null() {
            pRet = pRet.offset(16 as ::core::ffi::c_int as isize) as *mut u8_0;
        }
        return pRet as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn testCarrayFree(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        if !p.is_null() {
            let mut p2: *mut u8_0 = p as *mut u8_0;
            sqlite3_free(
                p2.offset(-16 as ::core::ffi::c_int as isize) as *mut u8_0
                    as *mut ::core::ffi::c_void,
            );
        }
    }
}
unsafe extern "C" fn delIntptr(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        Tcl_Free(p as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn bind_carray_intptr(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut iVar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aInt: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut nInt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ii: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if objc < 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut iVar,
        ) != 0
        {
            return TCL_ERROR;
        }
        nInt = objc - 3 as ::core::ffi::c_int;
        aInt = Tcl_Alloc(
            ((nInt + 1 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as ::core::ffi::c_uint,
        ) as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
        ii = 0 as ::core::ffi::c_int;
        while ii < nInt {
            if Tcl_GetIntFromObj(
                interp,
                *objv.offset((3 as ::core::ffi::c_int + ii) as isize),
                aInt.offset(ii as isize) as *mut ::core::ffi::c_int,
            ) != 0
            {
                Tcl_Free(aInt as *mut ::core::ffi::c_char);
                return TCL_ERROR;
            }
            ii += 1;
        }
        rc = sqlite3_bind_pointer(
            pStmt,
            iVar,
            aInt as *mut ::core::ffi::c_void,
            b"carray\0".as_ptr() as *const ::core::ffi::c_char,
            Some(delIntptr as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        Tcl_SetResult(interp, sqlite3ErrName(rc) as *mut ::core::ffi::c_char, None);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_carray_bind(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut eType: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mFlagsOverride: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nData: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut isTransient: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut isStatic: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut isMalloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut idx: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = Some(
            sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        );
        static mut aStaticData: *mut ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >() as *mut ::core::ffi::c_void;
        static mut nStaticData: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        static mut eStaticType: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !aStaticData.is_null() {
            if eStaticType == 3 as ::core::ffi::c_int {
                i = 0 as ::core::ffi::c_int;
                while i < nStaticData {
                    sqlite3_free(
                        *(aStaticData as *mut *mut ::core::ffi::c_char)
                            .offset(i as isize) as *mut ::core::ffi::c_void,
                    );
                    i += 1;
                }
            }
            if eStaticType == 4 as ::core::ffi::c_int {
                i = 0 as ::core::ffi::c_int;
                while i < nStaticData {
                    sqlite3_free(
                        (*(aStaticData as *mut iovec).offset(i as isize)).iov_base,
                    );
                    i += 1;
                }
            }
            sqlite3_free(aStaticData);
            aStaticData = ::core::ptr::null_mut::<::core::ffi::c_void>();
            nStaticData = 0 as ::core::ffi::c_int;
            eStaticType = 0 as ::core::ffi::c_int;
        }
        if objc == 1 as ::core::ffi::c_int {
            return TCL_OK;
        }
        i = 1 as ::core::ffi::c_int;
        while i < objc
            && *Tcl_GetString(*objv.offset(i as isize))
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32
        {
            let mut z: *const ::core::ffi::c_char = Tcl_GetString(
                *objv.offset(i as isize),
            );
            if strcmp(z, b"-transient\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                isTransient = 1 as ::core::ffi::c_int;
                xDel = ::core::mem::transmute::<
                    ::libc::intptr_t,
                    sqlite3_destructor_type,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t)
                    as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
            } else if strcmp(z, b"-static\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                isStatic = 1 as ::core::ffi::c_int;
                xDel = SQLITE_STATIC
                    as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
            } else if strcmp(z, b"-malloc\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                isMalloc = 1 as ::core::ffi::c_int;
                xDel = Some(
                    testCarrayFree
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
            } else if strcmp(z, b"-int32\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                eType = 0 as ::core::ffi::c_int;
            } else if strcmp(z, b"-int64\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                eType = 1 as ::core::ffi::c_int;
            } else if strcmp(z, b"-double\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                eType = 2 as ::core::ffi::c_int;
            } else if strcmp(z, b"-text\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                eType = 3 as ::core::ffi::c_int;
            } else if strcmp(z, b"-blob\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                eType = 4 as ::core::ffi::c_int;
            } else if i < objc - 1 as ::core::ffi::c_int
                && strcmp(z, b"-flags\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                i += 1;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(i as isize),
                    &raw mut mFlagsOverride,
                ) != 0
                {
                    return TCL_ERROR;
                }
            } else {
                if strcmp(z, b"--\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    break;
                }
                Tcl_AppendResult(
                    interp,
                    b"unknown option: \0".as_ptr() as *const ::core::ffi::c_char,
                    z,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_ERROR;
            }
            i += 1;
        }
        if eType == 3 as ::core::ffi::c_int && isStatic == 0 && isTransient == 0 {
            Tcl_AppendResult(
                interp,
                b"text data must be either -static or -transient\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        if eType == 4 as ::core::ffi::c_int && isStatic == 0 && isTransient == 0 {
            Tcl_AppendResult(
                interp,
                b"blob data must be either -static or -transient\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        if isStatic != 0 && isTransient != 0 {
            Tcl_AppendResult(
                interp,
                b"cannot be both -static and -transient\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        if objc - i < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"[OPTIONS] STMT IDX VALUE ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(i as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        i += 1;
        if Tcl_GetIntFromObj(interp, *objv.offset(i as isize), &raw mut idx) != 0 {
            return TCL_ERROR;
        }
        i += 1;
        nData = objc - i;
        match eType
            + 5 as ::core::ffi::c_int
                * (nData <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        {
            0 => {
                let mut a: *mut ::core::ffi::c_int = sqlite3_malloc(
                    (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        .wrapping_mul(nData as usize) as ::core::ffi::c_int,
                ) as *mut ::core::ffi::c_int;
                if a.is_null() {
                    rc = SQLITE_NOMEM;
                    c2rust_current_block = 15390890445113671489;
                } else {
                    j = 0 as ::core::ffi::c_int;
                    while j < nData {
                        let mut v: ::core::ffi::c_int = 0;
                        if Tcl_GetIntFromObj(
                            interp,
                            *objv.offset((i + j) as isize),
                            &raw mut v,
                        ) != 0
                        {
                            sqlite3_free(a as *mut ::core::ffi::c_void);
                            return TCL_ERROR;
                        }
                        *a.offset(j as isize) = v;
                        j += 1;
                    }
                    aData = a as *mut ::core::ffi::c_void;
                    c2rust_current_block = 10256747982273457880;
                }
            }
            1 => {
                let mut a_0: *mut sqlite3_int64 = sqlite3_malloc(
                    (::core::mem::size_of::<sqlite3_int64>() as usize)
                        .wrapping_mul(nData as usize) as ::core::ffi::c_int,
                ) as *mut sqlite3_int64;
                if a_0.is_null() {
                    rc = SQLITE_NOMEM;
                    c2rust_current_block = 15390890445113671489;
                } else {
                    j = 0 as ::core::ffi::c_int;
                    while j < nData {
                        let mut v_0: Tcl_WideInt = 0;
                        if Tcl_GetWideIntFromObj(
                            interp,
                            *objv.offset((i + j) as isize),
                            &raw mut v_0,
                        ) != 0
                        {
                            sqlite3_free(a_0 as *mut ::core::ffi::c_void);
                            return TCL_ERROR;
                        }
                        *a_0.offset(j as isize) = v_0 as sqlite3_int64;
                        j += 1;
                    }
                    aData = a_0 as *mut ::core::ffi::c_void;
                    c2rust_current_block = 10256747982273457880;
                }
            }
            2 => {
                let mut a_1: *mut ::core::ffi::c_double = sqlite3_malloc(
                    (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                        .wrapping_mul(nData as usize) as ::core::ffi::c_int,
                ) as *mut ::core::ffi::c_double;
                if a_1.is_null() {
                    rc = SQLITE_NOMEM;
                    c2rust_current_block = 15390890445113671489;
                } else {
                    j = 0 as ::core::ffi::c_int;
                    while j < nData {
                        let mut v_1: ::core::ffi::c_double = 0.;
                        if Tcl_GetDoubleFromObj(
                            interp,
                            *objv.offset((i + j) as isize),
                            &raw mut v_1,
                        ) != 0
                        {
                            sqlite3_free(a_1 as *mut ::core::ffi::c_void);
                            return TCL_ERROR;
                        }
                        *a_1.offset(j as isize) = v_1;
                        j += 1;
                    }
                    aData = a_1 as *mut ::core::ffi::c_void;
                    c2rust_current_block = 10256747982273457880;
                }
            }
            3 => {
                let mut a_2: *mut *mut ::core::ffi::c_char = sqlite3_malloc(
                    (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        .wrapping_mul(nData as usize) as ::core::ffi::c_int,
                ) as *mut *mut ::core::ffi::c_char;
                if a_2.is_null() {
                    rc = SQLITE_NOMEM;
                } else {
                    memset(
                        a_2 as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                            .wrapping_mul(nData as size_t),
                    );
                }
                j = 0 as ::core::ffi::c_int;
                while rc == SQLITE_OK && j < nData {
                    let mut v_2: *const ::core::ffi::c_char = Tcl_GetString(
                        *objv.offset((i + j) as isize),
                    );
                    if !v_2.is_null()
                        && strcmp(v_2, b"NULL\0".as_ptr() as *const ::core::ffi::c_char)
                            != 0
                    {
                        let ref mut c2rust_fresh18 = *a_2.offset(j as isize);
                        *c2rust_fresh18 = sqlite3_mprintf(
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            v_2,
                        );
                        if (*a_2.offset(j as isize)).is_null() {
                            rc = SQLITE_NOMEM;
                        }
                    }
                    j += 1;
                }
                aData = a_2 as *mut ::core::ffi::c_void;
                c2rust_current_block = 10256747982273457880;
            }
            4 => {
                let mut a_3: *mut iovec = sqlite3_malloc(
                    (::core::mem::size_of::<iovec>() as usize)
                        .wrapping_mul(nData as usize) as ::core::ffi::c_int,
                ) as *mut iovec;
                if a_3.is_null() {
                    rc = SQLITE_NOMEM;
                } else {
                    memset(
                        a_3 as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (::core::mem::size_of::<iovec>() as size_t)
                            .wrapping_mul(nData as size_t),
                    );
                }
                j = 0 as ::core::ffi::c_int;
                while rc == SQLITE_OK && j < nData {
                    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut v_3: *mut ::core::ffi::c_uchar = Tcl_GetByteArrayFromObj(
                        *objv.offset((i + i) as isize),
                        &raw mut n,
                    );
                    (*a_3.offset(j as isize)).iov_len = n as size_t;
                    let ref mut c2rust_fresh19 = (*a_3.offset(j as isize)).iov_base;
                    *c2rust_fresh19 = sqlite3_malloc64(n as sqlite3_uint64);
                    if (*a_3.offset(j as isize)).iov_base.is_null() {
                        (*a_3.offset(j as isize)).iov_len = 0 as size_t;
                        rc = SQLITE_NOMEM;
                    } else {
                        memcpy(
                            (*a_3.offset(j as isize)).iov_base,
                            v_3 as *const ::core::ffi::c_void,
                            n as size_t,
                        );
                    }
                    j += 1;
                }
                aData = a_3 as *mut ::core::ffi::c_void;
                c2rust_current_block = 10256747982273457880;
            }
            5 => {
                aData = b"\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_void;
                xDel = SQLITE_STATIC
                    as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
                isTransient = 0 as ::core::ffi::c_int;
                isStatic = 0 as ::core::ffi::c_int;
                c2rust_current_block = 10256747982273457880;
            }
            _ => {
                c2rust_current_block = 10256747982273457880;
            }
        }
        match c2rust_current_block {
            10256747982273457880 => {
                if rc == SQLITE_OK {
                    if isStatic != 0 {
                        aStaticData = aData;
                        nStaticData = nData;
                        eStaticType = eType;
                    } else if isMalloc != 0 {
                        let mut nByte: ::core::ffi::c_int = (if eType
                            == 0 as ::core::ffi::c_int
                        {
                            ::core::mem::size_of::<::core::ffi::c_int>() as usize
                        } else {
                            ::core::mem::size_of::<i64_0>() as usize
                        })
                            .wrapping_mul(nData as usize) as ::core::ffi::c_int;
                        let mut aByte: *mut ::core::ffi::c_void = testCarrayAlloc(nByte);
                        if aByte.is_null() {
                            sqlite3_free(aData);
                            rc = SQLITE_NOMEM;
                        } else {
                            memcpy(aByte, aData, nByte as size_t);
                            sqlite3_free(aData);
                            aData = aByte;
                            xDel = Some(
                                testCarrayFree
                                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                            )
                                as Option<
                                    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                                >;
                        }
                    }
                }
                if rc == SQLITE_OK {
                    if mFlagsOverride == 0 as ::core::ffi::c_int {
                        mFlagsOverride = eType;
                    }
                    rc = sqlite3_carray_bind(
                        pStmt,
                        idx,
                        aData,
                        nData,
                        mFlagsOverride,
                        xDel,
                    );
                }
                if isTransient != 0 {
                    if eType == 3 as ::core::ffi::c_int && !aData.is_null() {
                        i = 0 as ::core::ffi::c_int;
                        while i < nData {
                            sqlite3_free(
                                *(aData as *mut *mut ::core::ffi::c_char).offset(i as isize)
                                    as *mut ::core::ffi::c_void,
                            );
                            i += 1;
                        }
                    }
                    if eType == 4 as ::core::ffi::c_int && !aData.is_null() {
                        i = 0 as ::core::ffi::c_int;
                        while i < nData {
                            sqlite3_free(
                                (*(aData as *mut iovec).offset(i as isize)).iov_base,
                            );
                            i += 1;
                        }
                    }
                    sqlite3_free(aData);
                }
            }
            _ => {}
        }
        if rc != 0 {
            Tcl_AppendResult(
                interp,
                sqlite3_errstr(rc),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_parameter_count(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_bind_parameter_count(pStmt)));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_parameter_name(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut i: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT N\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut i,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj(
                sqlite3_bind_parameter_name(pStmt, i),
                -1 as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_bind_parameter_index(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT NAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj(
                sqlite3_bind_parameter_index(
                    pStmt,
                    Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                ),
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_clear_bindings(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_clear_bindings(pStmt)));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_sleep(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ms: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"MILLISECONDS\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut ms,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_sleep(ms)));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_ex_errcode(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_extended_errcode(db);
        Tcl_AppendResult(interp, sqlite3ErrName(rc) as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_errcode(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_errcode(db);
        Tcl_AppendResult(interp, sqlite3ErrName(rc) as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_errmsg(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zErr = sqlite3_errmsg(db);
        Tcl_SetObjResult(interp, Tcl_NewStringObj(zErr, -1 as ::core::ffi::c_int));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_error_offset(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut iByteOffset: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        iByteOffset = sqlite3_error_offset(db);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(iByteOffset));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_errmsg16(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zErr: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut bytes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zErr = sqlite3_errmsg16(db);
        if !zErr.is_null() {
            z = zErr as *const ::core::ffi::c_char;
            bytes = 0 as ::core::ffi::c_int;
            while *z.offset(bytes as isize) as ::core::ffi::c_int != 0
                || *z.offset((bytes + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int != 0
            {
                bytes += 2 as ::core::ffi::c_int;
            }
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewByteArrayObj(zErr as *const ::core::ffi::c_uchar, bytes),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_set_errmsg(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut iErr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB ERRCODE ERRMSG\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zDb = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        if *zDb.offset(0 as ::core::ffi::c_int as isize) != 0 {
            if getDbPointer(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut db,
            ) != 0
            {
                return TCL_ERROR;
            }
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut iErr,
        ) != 0
        {
            return TCL_ERROR;
        }
        zErr = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        rc = sqlite3_set_errmsg(
            db,
            iErr,
            if *zErr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            {
                zErr
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
        );
        Tcl_SetResult(interp, sqlite3ErrName(rc) as *mut ::core::ffi::c_char, None);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_prepare(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zSql: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut bytes: ::core::ffi::c_int = 0;
        let mut zTail: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 5 as ::core::ffi::c_int && objc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB sql bytes ?tailvar?\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSql = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut bytes,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_prepare(
            db,
            zSql,
            bytes,
            &raw mut pStmt,
            if objc >= 5 as ::core::ffi::c_int {
                &raw mut zTail
            } else {
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>()
            },
        );
        Tcl_ResetResult(interp);
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        if !zTail.is_null() && objc >= 5 as ::core::ffi::c_int {
            if bytes >= 0 as ::core::ffi::c_int {
                bytes = bytes
                    - zTail.offset_from(zSql) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
            }
            if (strlen(zTail) as ::core::ffi::c_int) < bytes {
                bytes = strlen(zTail) as ::core::ffi::c_int;
            }
            Tcl_ObjSetVar2(
                interp,
                *objv.offset(4 as ::core::ffi::c_int as isize),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                Tcl_NewStringObj(zTail, bytes),
                0 as ::core::ffi::c_int,
            );
        }
        if rc != SQLITE_OK {
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"(%d) \0".as_ptr() as *const ::core::ffi::c_char,
                rc,
            );
            Tcl_AppendResult(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                sqlite3_errmsg(db),
                NULL,
            );
            return TCL_ERROR;
        }
        if !pStmt.is_null() {
            if sqlite3TestMakePointerStr(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                pStmt as *mut ::core::ffi::c_void,
            ) != 0
            {
                return TCL_ERROR;
            }
            Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_prepare_v2(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zSql: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zCopy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut bytes: ::core::ffi::c_int = 0;
        let mut zTail: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pzTail: *mut *const ::core::ffi::c_char = ::core::ptr::null_mut::<
            *const ::core::ffi::c_char,
        >();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 5 as ::core::ffi::c_int && objc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB sql bytes tailvar\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSql = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut bytes,
        ) != 0
        {
            return TCL_ERROR;
        }
        if bytes >= 0 as ::core::ffi::c_int {
            zCopy = malloc(bytes as size_t) as *mut ::core::ffi::c_char;
            memcpy(
                zCopy as *mut ::core::ffi::c_void,
                zSql as *const ::core::ffi::c_void,
                bytes as size_t,
            );
        } else {
            let mut n: ::core::ffi::c_int = strlen(zSql) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int;
            zCopy = malloc(n as size_t) as *mut ::core::ffi::c_char;
            memcpy(
                zCopy as *mut ::core::ffi::c_void,
                zSql as *const ::core::ffi::c_void,
                n as size_t,
            );
        }
        pzTail = if objc >= 5 as ::core::ffi::c_int {
            &raw mut zTail
        } else {
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>()
        };
        rc = sqlite3_prepare_v2(db, zCopy, bytes, &raw mut pStmt, pzTail);
        if objc >= 5 as ::core::ffi::c_int {
            zTail = zSql.offset(zTail.offset_from(zCopy) as ::core::ffi::c_long as isize)
                as *const ::core::ffi::c_char;
        }
        free(zCopy as *mut ::core::ffi::c_void);
        Tcl_ResetResult(interp);
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        if rc == SQLITE_OK && objc >= 5 as ::core::ffi::c_int && !zTail.is_null() {
            if bytes >= 0 as ::core::ffi::c_int {
                bytes = bytes
                    - zTail.offset_from(zSql) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
            }
            Tcl_ObjSetVar2(
                interp,
                *objv.offset(4 as ::core::ffi::c_int as isize),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                Tcl_NewStringObj(zTail, bytes),
                0 as ::core::ffi::c_int,
            );
        }
        if rc != SQLITE_OK {
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"(%d) \0".as_ptr() as *const ::core::ffi::c_char,
                rc,
            );
            Tcl_AppendResult(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                sqlite3_errmsg(db),
                NULL,
            );
            return TCL_ERROR;
        }
        if !pStmt.is_null() {
            if sqlite3TestMakePointerStr(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                pStmt as *mut ::core::ffi::c_void,
            ) != 0
            {
                return TCL_ERROR;
            }
            Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_prepare_v3(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zSql: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zCopy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut bytes: ::core::ffi::c_int = 0;
        let mut flags: ::core::ffi::c_int = 0;
        let mut zTail: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pzTail: *mut *const ::core::ffi::c_char = ::core::ptr::null_mut::<
            *const ::core::ffi::c_char,
        >();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 6 as ::core::ffi::c_int && objc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB sql bytes flags tailvar\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSql = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut bytes,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(4 as ::core::ffi::c_int as isize),
            &raw mut flags,
        ) != 0
        {
            return TCL_ERROR;
        }
        if bytes >= 0 as ::core::ffi::c_int {
            zCopy = malloc(bytes as size_t) as *mut ::core::ffi::c_char;
            memcpy(
                zCopy as *mut ::core::ffi::c_void,
                zSql as *const ::core::ffi::c_void,
                bytes as size_t,
            );
        } else {
            let mut n: ::core::ffi::c_int = strlen(zSql) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int;
            zCopy = malloc(n as size_t) as *mut ::core::ffi::c_char;
            memcpy(
                zCopy as *mut ::core::ffi::c_void,
                zSql as *const ::core::ffi::c_void,
                n as size_t,
            );
        }
        pzTail = if objc >= 6 as ::core::ffi::c_int {
            &raw mut zTail
        } else {
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>()
        };
        rc = sqlite3_prepare_v3(
            db,
            zCopy,
            bytes,
            flags as ::core::ffi::c_uint,
            &raw mut pStmt,
            pzTail,
        );
        if !pzTail.is_null() {
            zTail = zSql.offset(zTail.offset_from(zCopy) as ::core::ffi::c_long as isize)
                as *const ::core::ffi::c_char;
        }
        free(zCopy as *mut ::core::ffi::c_void);
        Tcl_ResetResult(interp);
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        if rc == SQLITE_OK && !zTail.is_null() && objc >= 6 as ::core::ffi::c_int {
            if bytes >= 0 as ::core::ffi::c_int {
                bytes = bytes
                    - zTail.offset_from(zSql) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
            }
            Tcl_ObjSetVar2(
                interp,
                *objv.offset(5 as ::core::ffi::c_int as isize),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                Tcl_NewStringObj(zTail, bytes),
                0 as ::core::ffi::c_int,
            );
        }
        if rc != SQLITE_OK {
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"(%d) \0".as_ptr() as *const ::core::ffi::c_char,
                rc,
            );
            Tcl_AppendResult(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                sqlite3_errmsg(db),
                NULL,
            );
            return TCL_ERROR;
        }
        if !pStmt.is_null() {
            if sqlite3TestMakePointerStr(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                pStmt as *mut ::core::ffi::c_void,
            ) != 0
            {
                return TCL_ERROR;
            }
            Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_prepare_tkt3134(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        static mut zSql: [::core::ffi::c_char; 10] = unsafe {
            ::core::mem::transmute::<
                [u8; 10],
                [::core::ffi::c_char; 10],
            >(*b"\0SELECT 1\0")
        };
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB sql bytes tailvar\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_prepare_v2(
            db,
            (&raw const zSql as *const ::core::ffi::c_char)
                .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        if rc != SQLITE_OK {
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"(%d) \0".as_ptr() as *const ::core::ffi::c_char,
                rc,
            );
            Tcl_AppendResult(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                sqlite3_errmsg(db),
                NULL,
            );
            return TCL_ERROR;
        }
        if !pStmt.is_null() {
            if sqlite3TestMakePointerStr(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                pStmt as *mut ::core::ffi::c_void,
            ) != 0
            {
                return TCL_ERROR;
            }
            Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_prepare16(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zSql: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut zTail: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut pTail: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
        let mut rc: ::core::ffi::c_int = 0;
        let mut bytes: ::core::ffi::c_int = 0;
        let mut objlen: ::core::ffi::c_int = 0;
        if objc != 5 as ::core::ffi::c_int && objc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB sql bytes ?tailvar?\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSql = Tcl_GetByteArrayFromObj(
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut objlen,
        ) as *const ::core::ffi::c_void;
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut bytes,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_prepare16(
            db,
            zSql,
            bytes,
            &raw mut pStmt,
            if objc >= 5 as ::core::ffi::c_int {
                &raw mut zTail
            } else {
                ::core::ptr::null_mut::<*const ::core::ffi::c_void>()
            },
        );
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        if rc != 0 {
            return TCL_ERROR;
        }
        if objc >= 5 as ::core::ffi::c_int {
            if !zTail.is_null() {
                objlen = objlen
                    - (zTail as *mut u8_0).offset_from(zSql as *mut u8_0)
                        as ::core::ffi::c_long as ::core::ffi::c_int;
            } else {
                objlen = 0 as ::core::ffi::c_int;
            }
            pTail = Tcl_NewByteArrayObj(zTail as *mut u8_0, objlen);
            (*pTail).refCount += 1;
            Tcl_ObjSetVar2(
                interp,
                *objv.offset(4 as ::core::ffi::c_int as isize),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                pTail,
                0 as ::core::ffi::c_int,
            );
            let mut _objPtr: *mut Tcl_Obj = pTail;
            let c2rust_fresh20 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh20 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
        }
        if !pStmt.is_null() {
            if sqlite3TestMakePointerStr(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                pStmt as *mut ::core::ffi::c_void,
            ) != 0
            {
                return TCL_ERROR;
            }
        }
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_prepare16_v2(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zSql: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut zTail: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut pTail: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
        let mut rc: ::core::ffi::c_int = 0;
        let mut bytes: ::core::ffi::c_int = 0;
        let mut objlen: ::core::ffi::c_int = 0;
        if objc != 5 as ::core::ffi::c_int && objc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" DB sql bytes ?tailvar?\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSql = Tcl_GetByteArrayFromObj(
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut objlen,
        ) as *const ::core::ffi::c_void;
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut bytes,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_prepare16_v2(
            db,
            zSql,
            bytes,
            &raw mut pStmt,
            if objc >= 5 as ::core::ffi::c_int {
                &raw mut zTail
            } else {
                ::core::ptr::null_mut::<*const ::core::ffi::c_void>()
            },
        );
        if sqlite3TestErrCode(interp, db, rc) != 0 {
            return TCL_ERROR;
        }
        if rc != 0 {
            return TCL_ERROR;
        }
        if objc >= 5 as ::core::ffi::c_int {
            if !zTail.is_null() {
                objlen = objlen
                    - (zTail as *mut u8_0).offset_from(zSql as *mut u8_0)
                        as ::core::ffi::c_long as ::core::ffi::c_int;
            } else {
                objlen = 0 as ::core::ffi::c_int;
            }
            pTail = Tcl_NewByteArrayObj(zTail as *mut u8_0, objlen);
            (*pTail).refCount += 1;
            Tcl_ObjSetVar2(
                interp,
                *objv.offset(4 as ::core::ffi::c_int as isize),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                pTail,
                0 as ::core::ffi::c_int,
            );
            let mut _objPtr: *mut Tcl_Obj = pTail;
            let c2rust_fresh21 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh21 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
        }
        if !pStmt.is_null() {
            if sqlite3TestMakePointerStr(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                pStmt as *mut ::core::ffi::c_void,
            ) != 0
            {
                return TCL_ERROR;
            }
        }
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_open(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zFilename: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        if objc != 3 as ::core::ffi::c_int && objc != 2 as ::core::ffi::c_int
            && objc != 1 as ::core::ffi::c_int
        {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" filename options-list\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        zFilename = if objc > 1 as ::core::ffi::c_int {
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize))
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        sqlite3_open(zFilename, &raw mut db);
        if sqlite3TestMakePointerStr(
            interp,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            db as *mut ::core::ffi::c_void,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_open_v2(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zFilename: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zVfs: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        let mut nFlag: ::core::ffi::c_int = 0;
        let mut apFlag: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<*mut Tcl_Obj>();
        let mut i: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"FILENAME FLAGS VFS\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zFilename = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        zVfs = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        if *zVfs.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            zVfs = ::core::ptr::null::<::core::ffi::c_char>();
        }
        rc = Tcl_ListObjGetElements(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut nFlag,
            &raw mut apFlag,
        );
        if rc != TCL_OK {
            return rc;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nFlag {
            let mut iFlag: ::core::ffi::c_int = 0;
            let mut aFlag: [OpenFlag; 21] = [
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_READONLY\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_READONLY,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_READWRITE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_READWRITE,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_CREATE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_CREATE,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_DELETEONCLOSE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_DELETEONCLOSE,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_EXCLUSIVE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_EXCLUSIVE,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_AUTOPROXY\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_AUTOPROXY,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_MAIN_DB\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_MAIN_DB,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_TEMP_DB\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_TEMP_DB,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_TRANSIENT_DB\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_TRANSIENT_DB,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_MAIN_JOURNAL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_MAIN_JOURNAL,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_TEMP_JOURNAL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_TEMP_JOURNAL,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_SUBJOURNAL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_SUBJOURNAL,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_SUPER_JOURNAL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_SUPER_JOURNAL,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_NOMUTEX\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_NOMUTEX,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_FULLMUTEX\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_FULLMUTEX,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_SHAREDCACHE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_SHAREDCACHE,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_PRIVATECACHE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_PRIVATECACHE,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_WAL\0".as_ptr() as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_WAL,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_URI\0".as_ptr() as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_URI,
                },
                OpenFlag {
                    zFlag: b"SQLITE_OPEN_EXRESCODE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    flag: SQLITE_OPEN_EXRESCODE,
                },
                OpenFlag {
                    zFlag: ::core::ptr::null::<::core::ffi::c_char>(),
                    flag: 0 as ::core::ffi::c_int,
                },
            ];
            rc = Tcl_GetIndexFromObjStruct(
                interp,
                *apFlag.offset(i as isize),
                &raw mut aFlag as *mut OpenFlag as *const ::core::ffi::c_void,
                ::core::mem::size_of::<OpenFlag>() as ::core::ffi::c_int,
                b"flag\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                &raw mut iFlag,
            );
            if rc != TCL_OK {
                return rc;
            }
            flags |= aFlag[iFlag as usize].flag;
            i += 1;
        }
        rc = sqlite3_open_v2(zFilename, &raw mut db, flags, zVfs);
        if sqlite3TestMakePointerStr(
            interp,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            db as *mut ::core::ffi::c_void,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_open16(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zFilename: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" filename options-list\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        zFilename = Tcl_GetByteArrayFromObj(
            *objv.offset(1 as ::core::ffi::c_int as isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ) as *const ::core::ffi::c_void;
        sqlite3_open16(zFilename, &raw mut db);
        if sqlite3TestMakePointerStr(
            interp,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            db as *mut ::core::ffi::c_void,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_complete16(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"<utf-16 sql>\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zBuf = Tcl_GetByteArrayFromObj(
            *objv.offset(1 as ::core::ffi::c_int as isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ) as *mut ::core::ffi::c_char;
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj(sqlite3_complete16(zBuf as *const ::core::ffi::c_void)),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_normalize(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zNorm: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        unsafe extern "C" {
            #[link_name = "sqlite3_normalize"]
            fn sqlite3_normalize_0(
                _: *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_char;
        }
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SQL\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zSql = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        zNorm = sqlite3_normalize_0(zSql);
        if !zNorm.is_null() {
            Tcl_SetObjResult(interp, Tcl_NewStringObj(zNorm, -1 as ::core::ffi::c_int));
            sqlite3_free(zNorm as *mut ::core::ffi::c_void);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_step(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" STMT\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_step(pStmt);
        Tcl_SetResult(interp, sqlite3ErrName(rc) as *mut ::core::ffi::c_char, None);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_sql(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetResult(
            interp,
            sqlite3_sql(pStmt) as *mut ::core::ffi::c_char,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<Tcl_FreeProc>,
            >(1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_ex_sql(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        z = sqlite3_expanded_sql(pStmt);
        Tcl_SetResult(
            interp,
            z,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<Tcl_FreeProc>,
            >(1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_column_count(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" STMT column\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_column_count(pStmt)));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_column_type(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut col: ::core::ffi::c_int = 0;
        let mut tp: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" STMT column\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut col,
        ) != 0
        {
            return TCL_ERROR;
        }
        tp = sqlite3_column_type(pStmt, col);
        match tp {
            SQLITE_INTEGER => {
                Tcl_SetResult(
                    interp,
                    b"INTEGER\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    TCL_STATIC,
                );
            }
            SQLITE_NULL => {
                Tcl_SetResult(
                    interp,
                    b"NULL\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    TCL_STATIC,
                );
            }
            SQLITE_FLOAT => {
                Tcl_SetResult(
                    interp,
                    b"FLOAT\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    TCL_STATIC,
                );
            }
            SQLITE_TEXT => {
                Tcl_SetResult(
                    interp,
                    b"TEXT\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    TCL_STATIC,
                );
            }
            SQLITE_BLOB => {
                Tcl_SetResult(
                    interp,
                    b"BLOB\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    TCL_STATIC,
                );
            }
            _ => {}
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_column_int64(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut col: ::core::ffi::c_int = 0;
        let mut iVal: i64_0 = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" STMT column\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut col,
        ) != 0
        {
            return TCL_ERROR;
        }
        iVal = sqlite3_column_int64(pStmt, col) as i64_0;
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(iVal as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_column_blob(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut col: ::core::ffi::c_int = 0;
        let mut len: ::core::ffi::c_int = 0;
        let mut pBlob: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" STMT column\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut col,
        ) != 0
        {
            return TCL_ERROR;
        }
        len = sqlite3_column_bytes(pStmt, col);
        pBlob = sqlite3_column_blob(pStmt, col);
        Tcl_SetObjResult(
            interp,
            Tcl_NewByteArrayObj(pBlob as *const ::core::ffi::c_uchar, len),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_column_double(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut col: ::core::ffi::c_int = 0;
        let mut rVal: ::core::ffi::c_double = 0.;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" STMT column\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut col,
        ) != 0
        {
            return TCL_ERROR;
        }
        rVal = sqlite3_column_double(pStmt, col);
        Tcl_SetObjResult(interp, Tcl_NewDoubleObj(rVal));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_data_count(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" STMT column\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_data_count(pStmt)));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_stmt_utf8(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut col: ::core::ffi::c_int = 0;
        let mut xFunc: Option<
            unsafe extern "C" fn(
                *mut sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        > = None;
        let mut xFuncU: Option<
            unsafe extern "C" fn(
                *mut sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_uchar,
        > = None;
        let mut zRet: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        xFunc = ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_stmt,
                    ::core::ffi::c_int,
                ) -> *const ::core::ffi::c_char,
            >,
        >(clientData);
        xFuncU = ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_stmt,
                    ::core::ffi::c_int,
                ) -> *const ::core::ffi::c_char,
            >,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_stmt,
                    ::core::ffi::c_int,
                ) -> *const ::core::ffi::c_uchar,
            >,
        >(xFunc);
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" STMT column\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut col,
        ) != 0
        {
            return TCL_ERROR;
        }
        if xFunc
            == Some(
                sqlite3_column_name
                    as unsafe extern "C" fn(
                        *mut sqlite3_stmt,
                        ::core::ffi::c_int,
                    ) -> *const ::core::ffi::c_char,
            )
            || xFunc
                == Some(
                    sqlite3_column_decltype
                        as unsafe extern "C" fn(
                            *mut sqlite3_stmt,
                            ::core::ffi::c_int,
                        ) -> *const ::core::ffi::c_char,
                )
        {
            zRet = xFunc.expect("non-null function pointer")(pStmt, col);
        } else {
            zRet = xFuncU.expect("non-null function pointer")(pStmt, col)
                as *const ::core::ffi::c_char;
        }
        if !zRet.is_null() {
            Tcl_SetResult(interp, zRet as *mut ::core::ffi::c_char, None);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_global_recover(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = sqlite3_global_recover();
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_stmt_utf16(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut col: ::core::ffi::c_int = 0;
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut zName16: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut xFunc: Option<
            unsafe extern "C" fn(
                *mut sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_void,
        > = None;
        xFunc = ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_stmt,
                    ::core::ffi::c_int,
                ) -> *const ::core::ffi::c_void,
            >,
        >(clientData);
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" STMT column\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut col,
        ) != 0
        {
            return TCL_ERROR;
        }
        zName16 = xFunc.expect("non-null function pointer")(pStmt, col);
        if !zName16.is_null() {
            let mut n: ::core::ffi::c_int = 0;
            let mut z: *const ::core::ffi::c_char = zName16
                as *const ::core::ffi::c_char;
            n = 0 as ::core::ffi::c_int;
            while *z.offset(n as isize) as ::core::ffi::c_int != 0
                || *z.offset((n + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int != 0
            {
                n += 2 as ::core::ffi::c_int;
            }
            pRet = Tcl_NewByteArrayObj(
                zName16 as *const ::core::ffi::c_uchar,
                n + 2 as ::core::ffi::c_int,
            );
            Tcl_SetObjResult(interp, pRet);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_stmt_int(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut col: ::core::ffi::c_int = 0;
        let mut xFunc: Option<
            unsafe extern "C" fn(
                *mut sqlite3_stmt,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        > = None;
        xFunc = ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut sqlite3_stmt,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        >(clientData);
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                b" STMT column\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut col,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj(xFunc.expect("non-null function pointer")(pStmt, col)),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_interrupt(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_interrupt(db);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_is_interrupted(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_is_interrupted(db);
        Tcl_AppendResult(
            interp,
            if rc != 0 {
                b"1\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"0\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn delete_function(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB function-name\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_create_function(
            db,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            -1 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
            None,
            None,
        );
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn delete_collation(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB function-name\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_create_collation(
            db,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn get_autocommit(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zBuf: [::core::ffi::c_char; 30] = [0; 30];
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 30]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_get_autocommit(db),
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_busy_timeout(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut ms: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetInt(
            interp,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut ms,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_busy_timeout(db, ms);
        Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_setlk_timeout(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut ms: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut bBlockOnConnect: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if argc == 4 as ::core::ffi::c_int {
            let mut zArg: *const ::core::ffi::c_char = *argv
                .offset(1 as ::core::ffi::c_int as isize);
            let nArg: size_t = strlen(zArg) as size_t;
            if nArg >= 2 as size_t && nArg <= 15 as size_t
                && memcmp(
                    zArg as *const ::core::ffi::c_void,
                    b"-blockonconnect\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    nArg,
                ) == 0 as ::core::ffi::c_int
            {
                bBlockOnConnect = 1 as ::core::ffi::c_int;
            }
        }
        if argc != 3 as ::core::ffi::c_int + bBlockOnConnect {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ?-blockonconnect? DB MS\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            *argv.offset((argc - 2 as ::core::ffi::c_int) as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetInt(
            interp,
            *argv.offset((argc - 1 as ::core::ffi::c_int) as isize),
            &raw mut ms,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_setlk_timeout(
            db,
            ms,
            if bBlockOnConnect != 0 {
                SQLITE_SETLK_BLOCK_ON_CONNECT
            } else {
                0 as ::core::ffi::c_int
            },
        );
        Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn tcl_variable_type(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVar: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"VARIABLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pVar = Tcl_GetVar2Ex(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            ::core::ptr::null::<::core::ffi::c_char>(),
            TCL_LEAVE_ERR_MSG,
        );
        if pVar.is_null() {
            return TCL_ERROR;
        }
        if !(*pVar).typePtr.is_null() {
            Tcl_SetObjResult(
                interp,
                Tcl_NewStringObj((*(*pVar).typePtr).name, -1 as ::core::ffi::c_int),
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn fpnum_compare(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zA: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut zB: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut nDigit: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STRING1 STRING2\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zA = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_uchar;
        zB = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_uchar;
        j = 0 as ::core::ffi::c_int;
        i = j;
        loop {
            while *(*__ctype_b_loc())
                .offset(*zA.offset(i as isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
            {
                i += 1;
            }
            while *(*__ctype_b_loc())
                .offset(*zB.offset(j as isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
            {
                j += 1;
            }
            if *zA.offset(i as isize) as ::core::ffi::c_int
                != *zB.offset(j as isize) as ::core::ffi::c_int
            {
                break;
            }
            if *zA.offset(i as isize) as ::core::ffi::c_int == '-' as i32
                && *(*__ctype_b_loc())
                    .offset(
                        *zA.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int as isize,
                    ) as ::core::ffi::c_int
                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
            {
                i += 1;
                j += 1;
            }
            if *(*__ctype_b_loc())
                .offset(*zA.offset(i as isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int == 0
            {
                while *(*__ctype_b_loc())
                    .offset(*zA.offset(i as isize) as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int == 0
                    && *zA.offset(i as isize) as ::core::ffi::c_int != 0
                    && *zA.offset(i as isize) as ::core::ffi::c_int
                        == *zB.offset(j as isize) as ::core::ffi::c_int
                {
                    i += 1;
                    j += 1;
                }
                if *zA.offset(i as isize) as ::core::ffi::c_int
                    != *zB.offset(j as isize) as ::core::ffi::c_int
                {
                    break;
                }
                if !(*(*__ctype_b_loc())
                    .offset(*zA.offset(i as isize) as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0)
                {
                    break;
                }
            } else {
                nDigit = 0 as ::core::ffi::c_int;
                while *zA.offset(i as isize) as ::core::ffi::c_int
                    == *zB.offset(j as isize) as ::core::ffi::c_int
                    && *(*__ctype_b_loc())
                        .offset(*zA.offset(i as isize) as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int != 0
                {
                    i += 1;
                    j += 1;
                    nDigit += 1;
                }
                if *zA.offset(i as isize) as ::core::ffi::c_int
                    != *zB.offset(j as isize) as ::core::ffi::c_int
                {
                    break;
                }
                if *zA.offset(i as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if *zA.offset(i as isize) as ::core::ffi::c_int == '.' as i32
                    && *zB.offset(j as isize) as ::core::ffi::c_int == '.' as i32
                {
                    i += 1;
                    j += 1;
                    while *zA.offset(i as isize) as ::core::ffi::c_int
                        == *zB.offset(j as isize) as ::core::ffi::c_int
                        && *(*__ctype_b_loc())
                            .offset(
                                *zA.offset(i as isize) as ::core::ffi::c_int as isize,
                            ) as ::core::ffi::c_int
                            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                as ::core::ffi::c_int != 0
                    {
                        i += 1;
                        j += 1;
                        nDigit += 1;
                    }
                    if *zA.offset(i as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        while *zB.offset(j as isize) as ::core::ffi::c_int == '0' as i32
                            || *(*__ctype_b_loc())
                                .offset(
                                    *zB.offset(j as isize) as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                                && nDigit >= 15 as ::core::ffi::c_int
                        {
                            j += 1;
                            nDigit += 1;
                        }
                        break;
                    } else if *zB.offset(j as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        while *zA.offset(i as isize) as ::core::ffi::c_int == '0' as i32
                            || *(*__ctype_b_loc())
                                .offset(
                                    *zA.offset(i as isize) as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                                && nDigit >= 15 as ::core::ffi::c_int
                        {
                            i += 1;
                            nDigit += 1;
                        }
                        break;
                    } else {
                        if *(*__ctype_b_loc())
                            .offset(
                                *zA.offset(i as isize) as ::core::ffi::c_int as isize,
                            ) as ::core::ffi::c_int
                            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                as ::core::ffi::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(
                                    *zB.offset(j as isize) as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                        {
                            continue;
                        }
                        if *(*__ctype_b_loc())
                            .offset(
                                *zA.offset(i as isize) as ::core::ffi::c_int as isize,
                            ) as ::core::ffi::c_int
                            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                as ::core::ffi::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(
                                    *zB.offset(j as isize) as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                        {
                            if *zA.offset(i as isize) as ::core::ffi::c_int
                                == *zB.offset(j as isize) as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                && *(*__ctype_b_loc())
                                    .offset(
                                        *zA.offset((i + 1 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int as isize,
                                    ) as ::core::ffi::c_int
                                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                        as ::core::ffi::c_int == 0
                                && *(*__ctype_b_loc())
                                    .offset(
                                        *zB.offset((j + 1 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int as isize,
                                    ) as ::core::ffi::c_int
                                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                        as ::core::ffi::c_int != 0
                            {
                                j += 1;
                                while *zB.offset(j as isize) as ::core::ffi::c_int
                                    == '9' as i32
                                {
                                    j += 1;
                                    nDigit += 1;
                                }
                                if nDigit < 14 as ::core::ffi::c_int
                                    && (*(*__ctype_b_loc())
                                        .offset(
                                            *zB.offset(j as isize) as ::core::ffi::c_int as isize,
                                        ) as ::core::ffi::c_int
                                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                            as ::core::ffi::c_int == 0
                                        || (*zB.offset(j as isize) as ::core::ffi::c_int)
                                            < 5 as ::core::ffi::c_int)
                                {
                                    break;
                                }
                                while *(*__ctype_b_loc())
                                    .offset(
                                        *zB.offset(j as isize) as ::core::ffi::c_int as isize,
                                    ) as ::core::ffi::c_int
                                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                        as ::core::ffi::c_int != 0
                                {
                                    j += 1;
                                }
                                i += 1;
                            } else {
                                if !(*zB.offset(j as isize) as ::core::ffi::c_int
                                    == *zA.offset(i as isize) as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int
                                    && *(*__ctype_b_loc())
                                        .offset(
                                            *zB.offset((j + 1 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_int as isize,
                                        ) as ::core::ffi::c_int
                                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                            as ::core::ffi::c_int == 0
                                    && *(*__ctype_b_loc())
                                        .offset(
                                            *zA.offset((i + 1 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_int as isize,
                                        ) as ::core::ffi::c_int
                                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                            as ::core::ffi::c_int != 0)
                                {
                                    break;
                                }
                                i += 1;
                                while *zA.offset(i as isize) as ::core::ffi::c_int
                                    == '9' as i32
                                {
                                    i += 1;
                                    nDigit += 1;
                                }
                                if nDigit < 14 as ::core::ffi::c_int
                                    && (*(*__ctype_b_loc())
                                        .offset(
                                            *zA.offset(i as isize) as ::core::ffi::c_int as isize,
                                        ) as ::core::ffi::c_int
                                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                            as ::core::ffi::c_int == 0
                                        || (*zA.offset(i as isize) as ::core::ffi::c_int)
                                            < 5 as ::core::ffi::c_int)
                                {
                                    break;
                                }
                                while *(*__ctype_b_loc())
                                    .offset(
                                        *zA.offset(i as isize) as ::core::ffi::c_int as isize,
                                    ) as ::core::ffi::c_int
                                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                        as ::core::ffi::c_int != 0
                                {
                                    i += 1;
                                }
                                j += 1;
                            }
                        } else if *(*__ctype_b_loc())
                            .offset(
                                *zA.offset(i as isize) as ::core::ffi::c_int as isize,
                            ) as ::core::ffi::c_int
                            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                as ::core::ffi::c_int == 0
                            && *(*__ctype_b_loc())
                                .offset(
                                    *zB.offset(j as isize) as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                        {
                            while *zB.offset(j as isize) as ::core::ffi::c_int
                                == '0' as i32
                            {
                                j += 1;
                                nDigit += 1;
                            }
                            if nDigit < 15 as ::core::ffi::c_int {
                                break;
                            }
                            while *(*__ctype_b_loc())
                                .offset(
                                    *zB.offset(j as isize) as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                            {
                                j += 1;
                            }
                        } else {
                            if !(*(*__ctype_b_loc())
                                .offset(
                                    *zB.offset(j as isize) as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int == 0
                                && *(*__ctype_b_loc())
                                    .offset(
                                        *zA.offset(i as isize) as ::core::ffi::c_int as isize,
                                    ) as ::core::ffi::c_int
                                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                        as ::core::ffi::c_int != 0)
                            {
                                break;
                            }
                            while *zA.offset(i as isize) as ::core::ffi::c_int
                                == '0' as i32
                            {
                                i += 1;
                                nDigit += 1;
                            }
                            if nDigit < 15 as ::core::ffi::c_int {
                                break;
                            }
                            while *(*__ctype_b_loc())
                                .offset(
                                    *zA.offset(i as isize) as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                            {
                                i += 1;
                            }
                        }
                    }
                }
                if !(*zA.offset(i as isize) as ::core::ffi::c_int == 'e' as i32
                    && *zB.offset(j as isize) as ::core::ffi::c_int == 'e' as i32)
                {
                    continue;
                }
                i += 1;
                j += 1;
                if (*zA.offset(i as isize) as ::core::ffi::c_int == '+' as i32
                    || *zA.offset(i as isize) as ::core::ffi::c_int == '-' as i32)
                    && *zB.offset(j as isize) as ::core::ffi::c_int
                        == *zA.offset(i as isize) as ::core::ffi::c_int
                {
                    i += 1;
                    j += 1;
                }
                if *zA.offset(i as isize) as ::core::ffi::c_int
                    != *zB.offset(j as isize) as ::core::ffi::c_int
                {
                    if *zA.offset(i as isize) as ::core::ffi::c_int == '0' as i32
                        && *zA.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == *zB.offset(j as isize) as ::core::ffi::c_int
                    {
                        i += 1;
                    }
                    if *zB.offset(j as isize) as ::core::ffi::c_int == '0' as i32
                        && *zB.offset((j + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == *zA.offset(i as isize) as ::core::ffi::c_int
                    {
                        j += 1;
                    }
                }
                while *zA.offset(i as isize) as ::core::ffi::c_int
                    == *zB.offset(j as isize) as ::core::ffi::c_int
                    && *(*__ctype_b_loc())
                        .offset(*zA.offset(i as isize) as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int != 0
                {
                    i += 1;
                    j += 1;
                }
                if *zA.offset(i as isize) as ::core::ffi::c_int
                    != *zB.offset(j as isize) as ::core::ffi::c_int
                {
                    break;
                }
                if *zA.offset(i as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    break;
                }
            }
        }
        while *(*__ctype_b_loc())
            .offset(*zA.offset(i as isize) as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            i += 1;
        }
        while *(*__ctype_b_loc())
            .offset(*zB.offset(j as isize) as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            j += 1;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj(
                (*zA.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && *zB.offset(j as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_release_memory(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
unsafe extern "C" fn test_db_release_memory(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_db_release_memory(db);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_db_cacheflush(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_db_cacheflush(db);
        if rc != 0 {
            Tcl_SetResult(
                interp,
                sqlite3ErrStr(rc) as *mut ::core::ffi::c_char,
                TCL_STATIC,
            );
            return TCL_ERROR;
        }
        Tcl_ResetResult(interp);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_system_errno(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut iErrno: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        iErrno = sqlite3_system_errno(db);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(iErrno));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_db_filename(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zDbName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB DBNAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zDbName = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        Tcl_AppendResult(
            interp,
            sqlite3_db_filename(db, zDbName),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_db_readonly(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zDbName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB DBNAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zDbName = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_db_readonly(db, zDbName)));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_soft_heap_limit(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut amt: sqlite3_int64 = 0;
        let mut N: Tcl_WideInt = -1 as ::core::ffi::c_int as Tcl_WideInt;
        if objc != 1 as ::core::ffi::c_int && objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?N?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if objc == 2 as ::core::ffi::c_int {
            if Tcl_GetWideIntFromObj(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut N,
            ) != 0
            {
                return TCL_ERROR;
            }
        }
        amt = sqlite3_soft_heap_limit64(N as sqlite3_int64);
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(amt as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_hard_heap_limit(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut amt: sqlite3_int64 = 0;
        let mut N: Tcl_WideInt = -1 as ::core::ffi::c_int as Tcl_WideInt;
        if objc != 1 as ::core::ffi::c_int && objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?N?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if objc == 2 as ::core::ffi::c_int {
            if Tcl_GetWideIntFromObj(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut N,
            ) != 0
            {
                return TCL_ERROR;
            }
        }
        amt = sqlite3_hard_heap_limit64(N as sqlite3_int64);
        Tcl_SetObjResult(interp, Tcl_NewWideIntObj(amt as Tcl_WideInt));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_thread_cleanup(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_thread_cleanup();
        return TCL_OK;
    }
}
unsafe extern "C" fn test_pager_refcounts(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut i: ::core::ffi::c_int = 0;
        let mut v: ::core::ffi::c_int = 0;
        let mut a: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut pResult: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        pResult = Tcl_NewObj();
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nDb {
            if (*(*db).aDb.offset(i as isize)).pBt.is_null() {
                v = -1 as ::core::ffi::c_int;
            } else {
                sqlite3_mutex_enter((*db).mutex);
                a = sqlite3PagerStats(
                    sqlite3BtreePager((*(*db).aDb.offset(i as isize)).pBt) as *mut Pager,
                );
                v = *a.offset(0 as ::core::ffi::c_int as isize);
                sqlite3_mutex_leave((*db).mutex);
            }
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pResult,
                Tcl_NewIntObj(v),
            );
            i += 1;
        }
        Tcl_SetObjResult(interp, pResult);
        return TCL_OK;
    }
}
unsafe extern "C" fn working_64bit_int(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTestObj: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut working: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pTestObj = Tcl_NewWideIntObj(
            1000000 as Tcl_WideInt * 1234567890 as ::core::ffi::c_int as Tcl_WideInt,
        );
        working = (strcmp(
            Tcl_GetString(pTestObj),
            b"1234567890000000\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut _objPtr: *mut Tcl_Obj = pTestObj;
        let c2rust_fresh22 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh22 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj((working != 0 as ::core::ffi::c_int) as ::core::ffi::c_int),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn vfs_unlink_test(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut pMain: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut apVfs_0: [*mut sqlite3_vfs; 20] = [::core::ptr::null_mut::<
            sqlite3_vfs,
        >(); 20];
        let mut one: sqlite3_vfs = sqlite3_vfs {
            iVersion: 0,
            szOsFile: 0,
            mxPathname: 0,
            pNext: ::core::ptr::null_mut::<sqlite3_vfs>(),
            zName: ::core::ptr::null::<::core::ffi::c_char>(),
            pAppData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
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
        };
        let mut two: sqlite3_vfs = sqlite3_vfs {
            iVersion: 0,
            szOsFile: 0,
            mxPathname: 0,
            pNext: ::core::ptr::null_mut::<sqlite3_vfs>(),
            zName: ::core::ptr::null::<::core::ffi::c_char>(),
            pAppData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
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
        };
        sqlite3_vfs_unregister(::core::ptr::null_mut::<sqlite3_vfs>());
        one.zName = b"__one\0".as_ptr() as *const ::core::ffi::c_char;
        two.zName = b"__two\0".as_ptr() as *const ::core::ffi::c_char;
        pMain = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        sqlite3_vfs_register(&raw mut one, 0 as ::core::ffi::c_int);
        sqlite3_vfs_register(&raw mut two, 0 as ::core::ffi::c_int);
        sqlite3_vfs_register(&raw mut one, 1 as ::core::ffi::c_int);
        sqlite3_vfs_register(&raw mut two, 1 as ::core::ffi::c_int);
        if !pMain.is_null() {
            sqlite3_vfs_register(pMain, 1 as ::core::ffi::c_int);
        }
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[*mut sqlite3_vfs; 20]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut sqlite3_vfs>() as usize)
        {
            apVfs_0[i as usize] = sqlite3_vfs_find(
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            if !apVfs_0[i as usize].is_null() {
                sqlite3_vfs_unregister(apVfs_0[i as usize]);
            }
            i += 1;
        }
        sqlite3_vfs_register(pMain, 0 as ::core::ffi::c_int);
        sqlite3_vfs_unregister(pMain);
        i = (::core::mem::size_of::<[*mut sqlite3_vfs; 20]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut sqlite3_vfs>() as usize)
            .wrapping_sub(1 as usize) as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            if !apVfs_0[i as usize].is_null() {
                sqlite3_vfs_register(apVfs_0[i as usize], 1 as ::core::ffi::c_int);
            }
            i -= 1;
        }
        sqlite3_vfs_unregister(&raw mut one);
        sqlite3_vfs_unregister(&raw mut two);
        sqlite3_vfs_unregister(&raw mut one);
        sqlite3_vfs_unregister(&raw mut two);
        return TCL_OK;
    }
}
unsafe extern "C" fn vfs_initfail_test(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut one: sqlite3_vfs = sqlite3_vfs {
            iVersion: 0,
            szOsFile: 0,
            mxPathname: 0,
            pNext: ::core::ptr::null_mut::<sqlite3_vfs>(),
            zName: ::core::ptr::null::<::core::ffi::c_char>(),
            pAppData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
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
        };
        one.zName = b"__one\0".as_ptr() as *const ::core::ffi::c_char;
        if !sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>()).is_null() {
            return TCL_ERROR;
        }
        sqlite3_vfs_register(&raw mut one, 0 as ::core::ffi::c_int);
        if !sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>()).is_null() {
            return TCL_ERROR;
        }
        sqlite3_vfs_register(&raw mut one, 1 as ::core::ffi::c_int);
        if !sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>()).is_null() {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
static mut apVfs: [*mut sqlite3_vfs; 20] = [::core::ptr::null::<sqlite3_vfs>()
    as *mut sqlite3_vfs; 20];
static mut nVfs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn vfs_unregister_all(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[*mut sqlite3_vfs; 20]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut sqlite3_vfs>() as usize)
                as ::core::ffi::c_int
        {
            apVfs[i as usize] = sqlite3_vfs_find(
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            if apVfs[i as usize].is_null() {
                break;
            }
            sqlite3_vfs_unregister(apVfs[i as usize]);
            i += 1;
        }
        nVfs = i;
        return TCL_OK;
    }
}
unsafe extern "C" fn vfs_reregister_all(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = nVfs - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            sqlite3_vfs_register(apVfs[i as usize], 1 as ::core::ffi::c_int);
            i -= 1;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_test(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iArg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_file_control(
            db,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            &raw mut iArg as *mut ::core::ffi::c_void,
        );
        rc = sqlite3_file_control(
            db,
            b"notadatabase\0".as_ptr() as *const ::core::ffi::c_char,
            SQLITE_FCNTL_LOCKSTATE,
            &raw mut iArg as *mut ::core::ffi::c_void,
        );
        rc = sqlite3_file_control(
            db,
            b"main\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut iArg as *mut ::core::ffi::c_void,
        );
        rc = sqlite3_file_control(
            db,
            b"temp\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut iArg as *mut ::core::ffi::c_void,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_lasterrno_test(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iArg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_file_control(
            db,
            ::core::ptr::null::<::core::ffi::c_char>(),
            SQLITE_LAST_ERRNO,
            &raw mut iArg as *mut ::core::ffi::c_void,
        );
        if rc != 0 {
            Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
            return TCL_ERROR;
        }
        if iArg != 0 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"Unexpected non-zero errno: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    Tcl_NewIntObj(iArg),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" \0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_data_version(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iVers: ::core::ffi::c_uint = 0;
        let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        if objc != 3 as ::core::ffi::c_int && objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB [DBNAME]\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zDb = if objc == 3 as ::core::ffi::c_int {
            Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize))
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        rc = sqlite3_file_control(
            db,
            zDb,
            SQLITE_FCNTL_DATA_VERSION,
            &raw mut iVers as *mut ::core::ffi::c_void,
        );
        if rc != 0 {
            Tcl_SetResult(
                interp,
                sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                TCL_STATIC,
            );
            return TCL_ERROR;
        } else {
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                iVers,
            );
            Tcl_SetResult(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            return TCL_OK;
        };
    }
}
unsafe extern "C" fn file_control_chunksize_test(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nSize: ::core::ffi::c_int = 0;
        let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB DBNAME SIZE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
            || Tcl_GetIntFromObj(
                interp,
                *objv.offset(3 as ::core::ffi::c_int as isize),
                &raw mut nSize,
            ) != 0
        {
            return TCL_ERROR;
        }
        zDb = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if *zDb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
        {
            zDb = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        rc = sqlite3_file_control(
            db,
            zDb,
            SQLITE_FCNTL_CHUNK_SIZE,
            &raw mut nSize as *mut ::core::ffi::c_void,
        );
        if rc != 0 {
            Tcl_SetResult(
                interp,
                sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                TCL_STATIC,
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_sizehint_test(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nSize: Tcl_WideInt = 0;
        let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB DBNAME SIZE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
            || Tcl_GetWideIntFromObj(
                interp,
                *objv.offset(3 as ::core::ffi::c_int as isize),
                &raw mut nSize,
            ) != 0
        {
            return TCL_ERROR;
        }
        zDb = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if *zDb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
        {
            zDb = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        rc = sqlite3_file_control(
            db,
            zDb,
            SQLITE_FCNTL_SIZE_HINT,
            &raw mut nSize as *mut ::core::ffi::c_void,
        );
        if rc != 0 {
            Tcl_SetResult(
                interp,
                sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                TCL_STATIC,
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_lockproxy_test(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB PWD\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_persist_wal(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut bPersist: ::core::ffi::c_int = 0;
        let mut z: [::core::ffi::c_char; 100] = [0; 100];
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB FLAG\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut bPersist,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_file_control(
            db,
            ::core::ptr::null::<::core::ffi::c_char>(),
            SQLITE_FCNTL_PERSIST_WAL,
            &raw mut bPersist as *mut ::core::ffi::c_void,
        );
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut z as *mut ::core::ffi::c_char,
            b"%d %d\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
            bPersist,
        );
        Tcl_AppendResult(
            interp,
            &raw mut z as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_powersafe_overwrite(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut b: ::core::ffi::c_int = 0;
        let mut z: [::core::ffi::c_char; 100] = [0; 100];
        if objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB FLAG\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut b,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_file_control(
            db,
            ::core::ptr::null::<::core::ffi::c_char>(),
            SQLITE_FCNTL_POWERSAFE_OVERWRITE,
            &raw mut b as *mut ::core::ffi::c_void,
        );
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut z as *mut ::core::ffi::c_char,
            b"%d %d\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
            b,
        );
        Tcl_AppendResult(
            interp,
            &raw mut z as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_vfsname(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zDbName: *const ::core::ffi::c_char = b"main\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut zVfsName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB ?AUXDB?\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if objc == 3 as ::core::ffi::c_int {
            zDbName = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        }
        sqlite3_file_control(
            db,
            zDbName,
            SQLITE_FCNTL_VFSNAME,
            &raw mut zVfsName as *mut ::core::ffi::c_void,
        );
        Tcl_AppendResult(
            interp,
            zVfsName,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        sqlite3_free(zVfsName as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_reservebytes(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zDbName: *const ::core::ffi::c_char = b"main\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB N\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
            || Tcl_GetIntFromObj(
                interp,
                *objv.offset(2 as ::core::ffi::c_int as isize),
                &raw mut n,
            ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_file_control(
            db,
            zDbName,
            SQLITE_FCNTL_RESERVE_BYTES,
            &raw mut n as *mut ::core::ffi::c_void,
        );
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_tempfilename(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zDbName: *const ::core::ffi::c_char = b"main\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut zTName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB ?AUXDB?\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if objc == 3 as ::core::ffi::c_int {
            zDbName = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        }
        sqlite3_file_control(
            db,
            zDbName,
            SQLITE_FCNTL_TEMPFILENAME,
            &raw mut zTName as *mut ::core::ffi::c_void,
        );
        Tcl_AppendResult(interp, zTName, ::core::ptr::null_mut::<::core::ffi::c_char>());
        sqlite3_free(zTName as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn file_control_external_reader(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zName: *const ::core::ffi::c_char = b"main\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut iRes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB ?AUXDB?\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if objc == 3 as ::core::ffi::c_int {
            zName = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        }
        rc = sqlite3_file_control(
            db,
            zName,
            SQLITE_FCNTL_EXTERNAL_READER,
            &raw mut iRes as *mut ::core::ffi::c_void,
        );
        if rc != SQLITE_OK {
            Tcl_SetResult(
                interp,
                sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                TCL_STATIC,
            );
            return TCL_ERROR;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(iRes));
        return TCL_OK;
    }
}
unsafe extern "C" fn vfs_list(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut pRet: *mut Tcl_Obj = Tcl_NewObj();
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        while !pVfs.is_null() {
            Tcl_ListObjAppendElement(
                interp,
                pRet,
                Tcl_NewStringObj((*pVfs).zName, -1 as ::core::ffi::c_int),
            );
            pVfs = (*pVfs).pNext;
        }
        Tcl_SetObjResult(interp, pRet);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_limit(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        static mut aId: [C2Rust_Unnamed_33; 14] = [
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_LENGTH\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_LENGTH,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_SQL_LENGTH\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_SQL_LENGTH,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_COLUMN\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_COLUMN,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_EXPR_DEPTH\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_EXPR_DEPTH,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_COMPOUND_SELECT\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_COMPOUND_SELECT,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_VDBE_OP\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_VDBE_OP,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_FUNCTION_ARG\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_FUNCTION_ARG,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_ATTACHED\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_ATTACHED,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_LIKE_PATTERN_LENGTH\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_LIKE_PATTERN_LENGTH,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_VARIABLE_NUMBER\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_VARIABLE_NUMBER,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_TRIGGER_DEPTH\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_TRIGGER_DEPTH,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_WORKER_THREADS\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_WORKER_THREADS,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_TOOSMALL\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                id: -1 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_33 {
                zName: b"SQLITE_LIMIT_TOOBIG\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                id: SQLITE_LIMIT_WORKER_THREADS + 1 as ::core::ffi::c_int,
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let mut id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut val: ::core::ffi::c_int = 0;
        let mut zId: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if objc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringFromObj(
                    *objv.offset(0 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ),
                b" DB ID VALUE\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zId = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_33; 14]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_33>() as usize)
        {
            if strcmp(zId, aId[i as usize].zName) == 0 as ::core::ffi::c_int {
                id = aId[i as usize].id;
                break;
            } else {
                i += 1;
            }
        }
        if i as usize
            >= (::core::mem::size_of::<[C2Rust_Unnamed_33; 14]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_33>() as usize)
        {
            Tcl_AppendResult(
                interp,
                b"unknown limit type: \0".as_ptr() as *const ::core::ffi::c_char,
                zId,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut val,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_limit(db, id, val);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn save_prng_state(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_test_control(
            9999 as ::core::ffi::c_int,
        );
        rc = sqlite3_test_control(-1 as ::core::ffi::c_int);
        sqlite3_test_control(SQLITE_TESTCTRL_PRNG_SAVE);
        return TCL_OK;
    }
}
unsafe extern "C" fn restore_prng_state(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_test_control(SQLITE_TESTCTRL_PRNG_RESTORE);
        return TCL_OK;
    }
}
unsafe extern "C" fn reset_prng_state(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_randomness(
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn prng_seed(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SEED ?DB?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut i,
        ) != 0
        {
            return TCL_ERROR;
        }
        if objc == 3 as ::core::ffi::c_int
            && getDbPointer(
                interp,
                Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                &raw mut db,
            ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_test_control(SQLITE_TESTCTRL_PRNG_SEED, i, db);
        return TCL_OK;
    }
}
unsafe extern "C" fn extra_schema_checks(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetBooleanFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut i,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_test_control(SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS, i);
        return TCL_OK;
    }
}
unsafe extern "C" fn database_may_be_corrupt(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_test_control(SQLITE_TESTCTRL_NEVER_CORRUPT, 0 as ::core::ffi::c_int);
        return TCL_OK;
    }
}
unsafe extern "C" fn database_never_corrupt(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_test_control(SQLITE_TESTCTRL_NEVER_CORRUPT, 1 as ::core::ffi::c_int);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_pcache_stats(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nMin: ::core::ffi::c_int = 0;
        let mut nMax: ::core::ffi::c_int = 0;
        let mut nCurrent: ::core::ffi::c_int = 0;
        let mut nRecyclable: ::core::ffi::c_int = 0;
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        sqlite3PcacheStats(
            &raw mut nCurrent,
            &raw mut nMax,
            &raw mut nMin,
            &raw mut nRecyclable,
        );
        pRet = Tcl_NewObj();
        Tcl_ListObjAppendElement(
            interp,
            pRet,
            Tcl_NewStringObj(
                b"current\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(interp, pRet, Tcl_NewIntObj(nCurrent));
        Tcl_ListObjAppendElement(
            interp,
            pRet,
            Tcl_NewStringObj(
                b"max\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(interp, pRet, Tcl_NewIntObj(nMax));
        Tcl_ListObjAppendElement(
            interp,
            pRet,
            Tcl_NewStringObj(
                b"min\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(interp, pRet, Tcl_NewIntObj(nMin));
        Tcl_ListObjAppendElement(
            interp,
            pRet,
            Tcl_NewStringObj(
                b"recyclable\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(interp, pRet, Tcl_NewIntObj(nRecyclable));
        Tcl_SetObjResult(interp, pRet);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_wal_checkpoint(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int && objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB ?NAME?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if objc == 3 as ::core::ffi::c_int {
            zDb = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        }
        rc = sqlite3_wal_checkpoint(db, zDb);
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_wal_checkpoint_v2(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut eMode: ::core::ffi::c_int = 0;
        let mut nLog: ::core::ffi::c_int = -555 as ::core::ffi::c_int;
        let mut nCkpt: ::core::ffi::c_int = -555 as ::core::ffi::c_int;
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut aMode: [*const ::core::ffi::c_char; 5] = [
            b"passive\0".as_ptr() as *const ::core::ffi::c_char,
            b"full\0".as_ptr() as *const ::core::ffi::c_char,
            b"restart\0".as_ptr() as *const ::core::ffi::c_char,
            b"truncate\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        if objc != 3 as ::core::ffi::c_int && objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB MODE ?NAME?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if objc == 4 as ::core::ffi::c_int {
            zDb = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
            || TCL_OK
                != Tcl_GetIntFromObj(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut eMode,
                )
                && TCL_OK
                    != Tcl_GetIndexFromObjStruct(
                        interp,
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut aMode as *mut *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<*mut ::core::ffi::c_char>()
                            as ::core::ffi::c_int,
                        b"mode\0".as_ptr() as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                        &raw mut eMode,
                    )
        {
            return TCL_ERROR;
        }
        rc = sqlite3_wal_checkpoint_v2(db, zDb, eMode, &raw mut nLog, &raw mut nCkpt);
        if rc != SQLITE_OK && rc != SQLITE_BUSY {
            let mut zErrCode: *const ::core::ffi::c_char = sqlite3ErrName(rc);
            Tcl_ResetResult(interp);
            Tcl_AppendResult(
                interp,
                zErrCode,
                b" - \0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db) as *mut ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pRet = Tcl_NewObj();
        Tcl_ListObjAppendElement(
            interp,
            pRet,
            Tcl_NewIntObj(
                if rc == SQLITE_BUSY {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                },
            ),
        );
        Tcl_ListObjAppendElement(interp, pRet, Tcl_NewIntObj(nLog));
        Tcl_ListObjAppendElement(interp, pRet, Tcl_NewIntObj(nCkpt));
        Tcl_SetObjResult(interp, pRet);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_wal_autocheckpoint(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut iVal: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB VALUE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
            || Tcl_GetIntFromObj(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                *objv.offset(2 as ::core::ffi::c_int as isize),
                &raw mut iVal,
            ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_wal_autocheckpoint(db, iVal);
        Tcl_ResetResult(interp);
        if rc != SQLITE_OK {
            let mut zErrCode: *const ::core::ffi::c_char = sqlite3ErrName(rc);
            Tcl_SetObjResult(
                interp,
                Tcl_NewStringObj(zErrCode, -1 as ::core::ffi::c_int),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
static mut logcallback: LogCallback = LogCallback {
    pInterp: ::core::ptr::null::<Tcl_Interp>() as *mut Tcl_Interp,
    pObj: ::core::ptr::null::<Tcl_Obj>() as *mut Tcl_Obj,
};
unsafe extern "C" fn xLogcallback(
    mut unused: *mut ::core::ffi::c_void,
    mut err: ::core::ffi::c_int,
    mut zMsg: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut pNew: *mut Tcl_Obj = Tcl_DuplicateObj(logcallback.pObj);
        (*pNew).refCount += 1;
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pNew,
            Tcl_NewStringObj(sqlite3ErrName(err), -1 as ::core::ffi::c_int),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pNew,
            Tcl_NewStringObj(zMsg, -1 as ::core::ffi::c_int),
        );
        Tcl_EvalObjEx(logcallback.pInterp, pNew, TCL_EVAL_GLOBAL | TCL_EVAL_DIRECT);
        let mut _objPtr: *mut Tcl_Obj = pNew;
        let c2rust_fresh23 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh23 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
    }
}
unsafe extern "C" fn test_sqlite3_log(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        if objc > 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if !logcallback.pObj.is_null() {
            let mut _objPtr: *mut Tcl_Obj = logcallback.pObj;
            let c2rust_fresh24 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh24 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
            logcallback.pObj = ::core::ptr::null_mut::<Tcl_Obj>();
            logcallback.pInterp = ::core::ptr::null_mut::<Tcl_Interp>();
            sqlite3_config(
                SQLITE_CONFIG_LOG,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        if objc > 1 as ::core::ffi::c_int
            && *Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize))
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            logcallback.pObj = *objv.offset(1 as ::core::ffi::c_int as isize);
            (*logcallback.pObj).refCount += 1;
            logcallback.pInterp = interp;
            sqlite3_config(
                SQLITE_CONFIG_LOG,
                Some(
                    xLogcallback
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                        ) -> (),
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn runAsObjProc(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
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
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"COMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetCommandInfo(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut cmdInfo,
        ) == 0
        {
            Tcl_AppendResult(
                interp,
                b"command not found: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        if cmdInfo.objProc.is_none() {
            Tcl_AppendResult(
                interp,
                b"command has no objProc: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        return cmdInfo
            .objProc
            .expect(
                "non-null function pointer",
            )(
            cmdInfo.objClientData,
            interp,
            objc - 1 as ::core::ffi::c_int,
            objv.offset(1 as ::core::ffi::c_int as isize),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn printExplainQueryPlan(
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zSql: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zExplain: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pExplain: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        zSql = sqlite3_sql(pStmt);
        if zSql.is_null() {
            return SQLITE_ERROR;
        }
        zExplain = sqlite3_mprintf(
            b"EXPLAIN QUERY PLAN %s\0".as_ptr() as *const ::core::ffi::c_char,
            zSql,
        );
        if zExplain.is_null() {
            return SQLITE_NOMEM;
        }
        rc = sqlite3_prepare_v2(
            sqlite3_db_handle(pStmt),
            zExplain,
            -1 as ::core::ffi::c_int,
            &raw mut pExplain,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        sqlite3_free(zExplain as *mut ::core::ffi::c_void);
        if rc != SQLITE_OK {
            return rc;
        }
        while SQLITE_ROW == sqlite3_step(pExplain) {
            let mut iSelectid: ::core::ffi::c_int = sqlite3_column_int(
                pExplain,
                0 as ::core::ffi::c_int,
            );
            let mut iOrder: ::core::ffi::c_int = sqlite3_column_int(
                pExplain,
                1 as ::core::ffi::c_int,
            );
            let mut iFrom: ::core::ffi::c_int = sqlite3_column_int(
                pExplain,
                2 as ::core::ffi::c_int,
            );
            let mut zDetail: *const ::core::ffi::c_char = sqlite3_column_text(
                pExplain,
                3 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            printf(
                b"%d %d %d %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                iSelectid,
                iOrder,
                iFrom,
                zDetail,
            );
        }
        return sqlite3_finalize(pExplain);
    }
}
unsafe extern "C" fn test_print_eqp(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"STMT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getStmtPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pStmt,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = printExplainQueryPlan(pStmt);
        fflush(stdout);
        Tcl_SetResult(interp, sqlite3ErrName(rc) as *mut ::core::ffi::c_char, None);
        return TCL_OK;
    }
}
unsafe extern "C" fn testLocaltime(
    mut aliasT: *const ::core::ffi::c_void,
    mut aliasTM: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let t: time_t = *(aliasT as *const time_t);
        let mut pTm: *mut tm = aliasTM as *mut tm;
        let mut altT: time_t = 0;
        let mut iJD: sqlite3_int64 = 0;
        let mut Z: ::core::ffi::c_int = 0;
        let mut A: ::core::ffi::c_int = 0;
        let mut B: ::core::ffi::c_int = 0;
        let mut C: ::core::ffi::c_int = 0;
        let mut D: ::core::ffi::c_int = 0;
        let mut E: ::core::ffi::c_int = 0;
        let mut X1: ::core::ffi::c_int = 0;
        let mut S: ::core::ffi::c_int = 0;
        if t as ::core::ffi::c_long / 86400 as ::core::ffi::c_long
            & 1 as ::core::ffi::c_long != 0
        {
            altT = (t as ::core::ffi::c_long + 1800 as ::core::ffi::c_long) as time_t;
        } else {
            altT = (t as ::core::ffi::c_long - 1800 as ::core::ffi::c_long) as time_t;
        }
        iJD = (altT as ::core::ffi::c_long + 210866760000 as ::core::ffi::c_long)
            as sqlite3_int64;
        Z = ((iJD as ::core::ffi::c_longlong + 43200 as ::core::ffi::c_longlong)
            / 86400 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        A = ((Z as ::core::ffi::c_double - 1867216.25f64) / 36524.25f64)
            as ::core::ffi::c_int;
        A = Z + 1 as ::core::ffi::c_int + A - A / 4 as ::core::ffi::c_int;
        B = A + 1524 as ::core::ffi::c_int;
        C = ((B as ::core::ffi::c_double - 122.1f64) / 365.25f64) as ::core::ffi::c_int;
        D = 36525 as ::core::ffi::c_int * (C & 32767 as ::core::ffi::c_int)
            / 100 as ::core::ffi::c_int;
        E = ((B - D) as ::core::ffi::c_double / 30.6001f64) as ::core::ffi::c_int;
        X1 = (30.6001f64 * E as ::core::ffi::c_double) as ::core::ffi::c_int;
        (*pTm).tm_mday = B - D - X1;
        (*pTm).tm_mon = if E < 14 as ::core::ffi::c_int {
            E - 2 as ::core::ffi::c_int
        } else {
            E - 14 as ::core::ffi::c_int
        };
        (*pTm).tm_year = (if (*pTm).tm_mon > 1 as ::core::ffi::c_int {
            C - 4716 as ::core::ffi::c_int
        } else {
            C - 4715 as ::core::ffi::c_int
        }) - 1900 as ::core::ffi::c_int;
        S = ((iJD as ::core::ffi::c_longlong + 43200 as ::core::ffi::c_longlong)
            % 86400 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        (*pTm).tm_hour = S / 3600 as ::core::ffi::c_int;
        (*pTm).tm_min = S / 60 as ::core::ffi::c_int % 60 as ::core::ffi::c_int;
        (*pTm).tm_sec = S % 60 as ::core::ffi::c_int;
        return (t == 959609760 as ::core::ffi::c_long) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn strftime_cmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ts: Tcl_WideInt = 0;
        let mut t: time_t = 0;
        let mut pTm: *mut tm = ::core::ptr::null_mut::<tm>();
        let mut zFmt: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut n: size_t = 0;
        let mut zBuf: [::core::ffi::c_char; 1000] = [0; 1000];
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"FORMAT UNIXTIMESTAMP\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetWideIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut ts,
        ) != 0
        {
            return TCL_ERROR;
        }
        zFmt = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        t = ts as time_t;
        pTm = gmtime(&raw mut t);
        n = strftime(
            &raw mut zBuf as *mut ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 1000]>() as size_t)
                .wrapping_sub(1 as size_t),
            zFmt,
            pTm,
        );
        if n >= 0 as size_t
            && n < ::core::mem::size_of::<[::core::ffi::c_char; 1000]>() as usize
        {
            zBuf[n as usize] = 0 as ::core::ffi::c_char;
            Tcl_SetResult(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_treetrace(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut v: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        if objc >= 2 as ::core::ffi::c_int {
            if Tcl_GetIntFromObj(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut v as *mut ::core::ffi::c_int,
            ) == TCL_OK
            {
                sqlite3_test_control(
                    SQLITE_TESTCTRL_TRACEFLAGS,
                    1 as ::core::ffi::c_int,
                    &raw mut v,
                );
            }
        }
        sqlite3_test_control(
            SQLITE_TESTCTRL_TRACEFLAGS,
            0 as ::core::ffi::c_int,
            &raw mut v,
        );
        Tcl_SetObjResult(interp, Tcl_NewIntObj(v as ::core::ffi::c_int));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_test_control(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aVerb: [Verb; 6] = [
            Verb {
                zName: b"SQLITE_TESTCTRL_LOCALTIME_FAULT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i: SQLITE_TESTCTRL_LOCALTIME_FAULT,
            },
            Verb {
                zName: b"SQLITE_TESTCTRL_SORTER_MMAP\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i: SQLITE_TESTCTRL_SORTER_MMAP,
            },
            Verb {
                zName: b"SQLITE_TESTCTRL_IMPOSTER\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i: SQLITE_TESTCTRL_IMPOSTER,
            },
            Verb {
                zName: b"SQLITE_TESTCTRL_INTERNAL_FUNCTIONS\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i: SQLITE_TESTCTRL_INTERNAL_FUNCTIONS,
            },
            Verb {
                zName: b"SQLITE_TESTCTRL_FK_NO_ACTION\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i: SQLITE_TESTCTRL_FK_NO_ACTION,
            },
            Verb {
                zName: ::core::ptr::null::<::core::ffi::c_char>(),
                i: 0 as ::core::ffi::c_int,
            },
        ];
        let mut iVerb: ::core::ffi::c_int = 0;
        let mut iFlag: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"VERB ARGS...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aVerb as *mut Verb as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Verb>() as ::core::ffi::c_int,
            b"VERB\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iVerb,
        );
        if rc != TCL_OK {
            return rc;
        }
        iFlag = aVerb[iVerb as usize].i;
        match iFlag {
            SQLITE_TESTCTRL_INTERNAL_FUNCTIONS => {
                let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"DB\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if getDbPointer(
                    interp,
                    Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                    &raw mut db,
                ) != 0
                {
                    return TCL_ERROR;
                }
                sqlite3_test_control(SQLITE_TESTCTRL_INTERNAL_FUNCTIONS, db);
            }
            SQLITE_TESTCTRL_LOCALTIME_FAULT => {
                let mut val: ::core::ffi::c_int = 0;
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"0|1|2\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut val,
                ) != 0
                {
                    return TCL_ERROR;
                }
                sqlite3_test_control(
                    iFlag,
                    val,
                    Some(
                        testLocaltime
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_void,
                                *mut ::core::ffi::c_void,
                            ) -> ::core::ffi::c_int,
                    ),
                );
            }
            SQLITE_TESTCTRL_FK_NO_ACTION => {
                let mut val_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut db_0: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
                if objc != 4 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"DB BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if getDbPointer(
                    interp,
                    Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                    &raw mut db_0,
                ) != 0
                {
                    return TCL_ERROR;
                }
                if Tcl_GetBooleanFromObj(
                    interp,
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut val_0,
                ) != 0
                {
                    return TCL_ERROR;
                }
                sqlite3_test_control(SQLITE_TESTCTRL_FK_NO_ACTION, db_0, val_0);
            }
            SQLITE_TESTCTRL_SORTER_MMAP => {
                let mut val_1: ::core::ffi::c_int = 0;
                let mut db_1: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
                if objc != 4 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"DB LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if getDbPointer(
                    interp,
                    Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                    &raw mut db_1,
                ) != 0
                {
                    return TCL_ERROR;
                }
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut val_1,
                ) != 0
                {
                    return TCL_ERROR;
                }
                sqlite3_test_control(SQLITE_TESTCTRL_SORTER_MMAP, db_1, val_1);
            }
            SQLITE_TESTCTRL_IMPOSTER => {
                let mut onOff: ::core::ffi::c_int = 0;
                let mut tnum: ::core::ffi::c_int = 0;
                let mut zDbName: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut db_2: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
                if objc != 6 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"DB dbName onOff tnum\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if getDbPointer(
                    interp,
                    Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                    &raw mut db_2,
                ) != 0
                {
                    return TCL_ERROR;
                }
                zDbName = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(4 as ::core::ffi::c_int as isize),
                    &raw mut onOff,
                ) != 0
                {
                    return TCL_ERROR;
                }
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(5 as ::core::ffi::c_int as isize),
                    &raw mut tnum,
                ) != 0
                {
                    return TCL_ERROR;
                }
                sqlite3_test_control(
                    SQLITE_TESTCTRL_IMPOSTER,
                    db_2,
                    zDbName,
                    onOff,
                    tnum,
                );
            }
            _ => {}
        }
        Tcl_ResetResult(interp);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_getrusage(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut r: rusage = rusage {
            ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
            ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
            c2rust_unnamed: C2Rust_Unnamed_47 { ru_maxrss: 0 },
            c2rust_unnamed_0: C2Rust_Unnamed_46 { ru_ixrss: 0 },
            c2rust_unnamed_1: C2Rust_Unnamed_45 { ru_idrss: 0 },
            c2rust_unnamed_2: C2Rust_Unnamed_44 { ru_isrss: 0 },
            c2rust_unnamed_3: C2Rust_Unnamed_43 { ru_minflt: 0 },
            c2rust_unnamed_4: C2Rust_Unnamed_42 { ru_majflt: 0 },
            c2rust_unnamed_5: C2Rust_Unnamed_41 { ru_nswap: 0 },
            c2rust_unnamed_6: C2Rust_Unnamed_40 { ru_inblock: 0 },
            c2rust_unnamed_7: C2Rust_Unnamed_39 { ru_oublock: 0 },
            c2rust_unnamed_8: C2Rust_Unnamed_38 { ru_msgsnd: 0 },
            c2rust_unnamed_9: C2Rust_Unnamed_37 { ru_msgrcv: 0 },
            c2rust_unnamed_10: C2Rust_Unnamed_36 {
                ru_nsignals: 0,
            },
            c2rust_unnamed_11: C2Rust_Unnamed_35 { ru_nvcsw: 0 },
            c2rust_unnamed_12: C2Rust_Unnamed_34 { ru_nivcsw: 0 },
        };
        memset(
            &raw mut r as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<rusage>() as size_t,
        );
        getrusage(RUSAGE_SELF, &raw mut r);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            &raw mut buf as *mut ::core::ffi::c_char,
            b"ru_utime=%d.%06d ru_stime=%d.%06d ru_minflt=%d ru_majflt=%d\0".as_ptr()
                as *const ::core::ffi::c_char,
            r.ru_utime.tv_sec as ::core::ffi::c_int,
            r.ru_utime.tv_usec as ::core::ffi::c_int,
            r.ru_stime.tv_sec as ::core::ffi::c_int,
            r.ru_stime.tv_usec as ::core::ffi::c_int,
            r.c2rust_unnamed_3.ru_minflt as ::core::ffi::c_int,
            r.c2rust_unnamed_4.ru_majflt as ::core::ffi::c_int,
        );
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj(
                &raw mut buf as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn optimization_control(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zOpt: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut onoff: ::core::ffi::c_int = 0;
        let mut mask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        static mut aOpt: [C2Rust_Unnamed_48; 18] = [
            C2Rust_Unnamed_48 {
                zOptName: b"all\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_AllOpts as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"none\0".as_ptr() as *const ::core::ffi::c_char,
                mask: 0 as ::core::ffi::c_int,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"query-flattener\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_QueryFlattener,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"groupby-order\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_GroupByOrder,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"factor-constants\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_FactorOutConst,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"distinct-opt\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_DistinctOpt,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"cover-idx-scan\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_CoverIdxScan,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"order-by-idx-join\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_OrderByIdxJoin,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"order-by-subquery\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_OrderBySubq,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"transitive\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_Transitive,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"omit-noop-join\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_OmitNoopJoin,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"stat4\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_Stat4,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"skip-scan\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_SkipScan,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"push-down\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_PushDown,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"balanced-merge\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_BalancedMerge,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"propagate-const\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_PropagateConst,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"one-pass\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_OnePass,
            },
            C2Rust_Unnamed_48 {
                zOptName: b"exists-to-join\0".as_ptr() as *const ::core::ffi::c_char,
                mask: SQLITE_ExistsToJoin,
            },
        ];
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB OPT BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetBooleanFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut onoff,
        ) != 0
        {
            return TCL_ERROR;
        }
        zOpt = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_48; 18]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_48>() as usize)
        {
            if !strstr(zOpt, aOpt[i as usize].zOptName).is_null() {
                mask |= aOpt[i as usize].mask;
                cnt += 1;
            }
            i += 1;
        }
        if onoff != 0 {
            mask = !mask;
        }
        if cnt == 0 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"unknown optimization - should be one of:\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            i = 0 as ::core::ffi::c_int;
            while (i as usize)
                < (::core::mem::size_of::<[C2Rust_Unnamed_48; 18]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_48>() as usize)
            {
                Tcl_AppendResult(
                    interp,
                    b" \0".as_ptr() as *const ::core::ffi::c_char,
                    aOpt[i as usize].zOptName,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                i += 1;
            }
            return TCL_ERROR;
        }
        sqlite3_test_control(SQLITE_TESTCTRL_OPTIMIZATIONS, db, mask);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(mask));
        return TCL_OK;
    }
}
unsafe extern "C" fn tclLoadStaticExtensionCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            #[link_name = "sqlite3_amatch_init"]
            fn sqlite3_amatch_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_appendvfs_init"]
            fn sqlite3_appendvfs_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_basexx_init"]
            fn sqlite3_basexx_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_closure_init"]
            fn sqlite3_closure_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_csv_init"]
            fn sqlite3_csv_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_eval_init"]
            fn sqlite3_eval_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_explain_init"]
            fn sqlite3_explain_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_fileio_init"]
            fn sqlite3_fileio_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_decimal_init"]
            fn sqlite3_decimal_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_fuzzer_init"]
            fn sqlite3_fuzzer_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_ieee_init"]
            fn sqlite3_ieee_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_nextchar_init"]
            fn sqlite3_nextchar_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_prefixes_init"]
            fn sqlite3_prefixes_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_qpvtab_init"]
            fn sqlite3_qpvtab_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_randomjson_init"]
            fn sqlite3_randomjson_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_regexp_init"]
            fn sqlite3_regexp_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_remember_init"]
            fn sqlite3_remember_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_series_init"]
            fn sqlite3_series_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_spellfix_init"]
            fn sqlite3_spellfix_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_stmtrand_init"]
            fn sqlite3_stmtrand_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_totype_init"]
            fn sqlite3_totype_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_wholenumber_init"]
            fn sqlite3_wholenumber_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3_unionvtab_init"]
            fn sqlite3_unionvtab_init_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        static mut aExtension: [C2Rust_Unnamed_49; 23] = unsafe {
            [
                C2Rust_Unnamed_49 {
                    zExtName: b"amatch\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_amatch_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"appendvfs\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_appendvfs_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"basexx\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_basexx_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"closure\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_closure_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"csv\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_csv_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"decimal\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_decimal_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"eval\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_eval_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"explain\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_explain_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"fileio\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_fileio_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"fuzzer\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_fuzzer_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"ieee754\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_ieee_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"nextchar\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_nextchar_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"prefixes\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_prefixes_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"qpvtab\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_qpvtab_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"randomjson\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_randomjson_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"regexp\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_regexp_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"remember\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_remember_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"series\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_series_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"spellfix\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_spellfix_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"stmtrand\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_stmtrand_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"totype\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_totype_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"unionvtab\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_unionvtab_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_49 {
                    zExtName: b"wholenumber\0".as_ptr() as *const ::core::ffi::c_char,
                    pInit: Some(
                        sqlite3_wholenumber_init_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                },
            ]
        };
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut zErrMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc < 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB NAME ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        j = 2 as ::core::ffi::c_int;
        while j < objc {
            zName = Tcl_GetString(*objv.offset(j as isize));
            i = 0 as ::core::ffi::c_int;
            while i
                < (::core::mem::size_of::<[C2Rust_Unnamed_49; 23]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_49>() as usize)
                    as ::core::ffi::c_int
            {
                if strcmp(zName, aExtension[i as usize].zExtName)
                    == 0 as ::core::ffi::c_int
                {
                    break;
                }
                i += 1;
            }
            if i
                >= (::core::mem::size_of::<[C2Rust_Unnamed_49; 23]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_49>() as usize)
                    as ::core::ffi::c_int
            {
                Tcl_AppendResult(
                    interp,
                    b"no such extension: \0".as_ptr() as *const ::core::ffi::c_char,
                    zName,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_ERROR;
            }
            if aExtension[i as usize].pInit.is_some() {
                rc = aExtension[i as usize]
                    .pInit
                    .expect(
                        "non-null function pointer",
                    )(db, &raw mut zErrMsg, ::core::ptr::null::<sqlite3_api_routines>());
            } else {
                rc = SQLITE_OK;
            }
            if rc != SQLITE_OK && rc != SQLITE_OK_LOAD_PERMANENTLY || !zErrMsg.is_null()
            {
                Tcl_AppendResult(
                    interp,
                    b"initialization of \0".as_ptr() as *const ::core::ffi::c_char,
                    zName,
                    b" failed: \0".as_ptr() as *const ::core::ffi::c_char,
                    zErrMsg,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                sqlite3_free(zErrMsg as *mut ::core::ffi::c_void);
                return TCL_ERROR;
            }
            j += 1;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn sorter_test_fakeheap(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut bArg: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"BOOL\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetBooleanFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut bArg,
        ) != 0
        {
            return TCL_ERROR;
        }
        if bArg != 0 {
            if sqlite3Config.pHeap.is_null() {
                sqlite3Config.pHeap = -1 as ::core::ffi::c_int as intptr_t
                    as *mut ::core::ffi::c_void;
            }
        } else if sqlite3Config.pHeap
            == -1 as ::core::ffi::c_int as intptr_t as *mut ::core::ffi::c_void
        {
            sqlite3Config.pHeap = ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        Tcl_ResetResult(interp);
        return TCL_OK;
    }
}
unsafe extern "C" fn sorter_test_sort4_helper(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zSql1: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zSql2: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nStep: ::core::ffi::c_int = 0;
        let mut iStep: ::core::ffi::c_int = 0;
        let mut iCksum1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut iCksum2: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut rc: ::core::ffi::c_int = 0;
        let mut iB: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if objc != 5 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB SQL1 NSTEP SQL2\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSql1 = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut nStep,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSql2 = Tcl_GetString(*objv.offset(4 as ::core::ffi::c_int as isize));
        rc = sqlite3_prepare_v2(
            db,
            zSql1,
            -1 as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if !(rc != SQLITE_OK) {
            iB = sqlite3_column_count(pStmt) - 1 as ::core::ffi::c_int;
            iStep = 0 as ::core::ffi::c_int;
            while iStep < nStep && SQLITE_ROW == sqlite3_step(pStmt) {
                let mut a: ::core::ffi::c_int = sqlite3_column_int(
                    pStmt,
                    0 as ::core::ffi::c_int,
                );
                if a != sqlite3_column_int(pStmt, iB) {
                    Tcl_AppendResult(
                        interp,
                        b"data error: (a!=b)\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    );
                    return TCL_ERROR;
                }
                iCksum1 = iCksum1
                    .wrapping_add(
                        (iCksum1 << 3 as ::core::ffi::c_int)
                            .wrapping_add(a as ::core::ffi::c_uint),
                    );
                iStep += 1;
            }
            rc = sqlite3_finalize(pStmt);
            if !(rc != SQLITE_OK) {
                rc = sqlite3_prepare_v2(
                    db,
                    zSql2,
                    -1 as ::core::ffi::c_int,
                    &raw mut pStmt,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                if !(rc != SQLITE_OK) {
                    iStep = 0 as ::core::ffi::c_int;
                    while SQLITE_ROW == sqlite3_step(pStmt) {
                        let mut a_0: ::core::ffi::c_int = sqlite3_column_int(
                            pStmt,
                            0 as ::core::ffi::c_int,
                        );
                        iCksum2 = iCksum2
                            .wrapping_add(
                                (iCksum2 << 3 as ::core::ffi::c_int)
                                    .wrapping_add(a_0 as ::core::ffi::c_uint),
                            );
                        iStep += 1;
                    }
                    rc = sqlite3_finalize(pStmt);
                    if !(rc != SQLITE_OK) {
                        if iCksum1 != iCksum2 {
                            Tcl_AppendResult(
                                interp,
                                b"checksum mismatch\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            );
                            return TCL_ERROR;
                        }
                        return TCL_OK;
                    }
                }
            }
        }
        Tcl_AppendResult(
            interp,
            b"sql error: \0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return TCL_ERROR;
    }
}
unsafe extern "C" fn test_register_dbstat_vtab(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zDb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
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
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zDb = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        if Tcl_GetCommandInfo(interp, zDb, &raw mut cmdInfo) != 0 {
            let mut db: *mut sqlite3 = (*(cmdInfo.objClientData as *mut SqliteDb_0)).db;
            sqlite3DbstatRegister(db);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_sqlite3_db_config(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aSetting: [C2Rust_Unnamed_50; 20] = [
            C2Rust_Unnamed_50 {
                zName: b"FKEY\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_ENABLE_FKEY,
            },
            C2Rust_Unnamed_50 {
                zName: b"TRIGGER\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_ENABLE_TRIGGER,
            },
            C2Rust_Unnamed_50 {
                zName: b"FTS3_TOKENIZER\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER,
            },
            C2Rust_Unnamed_50 {
                zName: b"LOAD_EXTENSION\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION,
            },
            C2Rust_Unnamed_50 {
                zName: b"NO_CKPT_ON_CLOSE\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE,
            },
            C2Rust_Unnamed_50 {
                zName: b"QPSG\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_ENABLE_QPSG,
            },
            C2Rust_Unnamed_50 {
                zName: b"TRIGGER_EQP\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_TRIGGER_EQP,
            },
            C2Rust_Unnamed_50 {
                zName: b"RESET_DB\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_RESET_DATABASE,
            },
            C2Rust_Unnamed_50 {
                zName: b"DEFENSIVE\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_DEFENSIVE,
            },
            C2Rust_Unnamed_50 {
                zName: b"WRITABLE_SCHEMA\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_WRITABLE_SCHEMA,
            },
            C2Rust_Unnamed_50 {
                zName: b"LEGACY_ALTER_TABLE\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_LEGACY_ALTER_TABLE,
            },
            C2Rust_Unnamed_50 {
                zName: b"DQS_DML\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_DQS_DML,
            },
            C2Rust_Unnamed_50 {
                zName: b"DQS_DDL\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_DQS_DDL,
            },
            C2Rust_Unnamed_50 {
                zName: b"LEGACY_FILE_FORMAT\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_LEGACY_FILE_FORMAT,
            },
            C2Rust_Unnamed_50 {
                zName: b"TRUSTED_SCHEMA\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_TRUSTED_SCHEMA,
            },
            C2Rust_Unnamed_50 {
                zName: b"STMT_SCANSTATUS\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_STMT_SCANSTATUS,
            },
            C2Rust_Unnamed_50 {
                zName: b"REVERSE_SCANORDER\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_REVERSE_SCANORDER,
            },
            C2Rust_Unnamed_50 {
                zName: b"ATTACH_CREATE\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE,
            },
            C2Rust_Unnamed_50 {
                zName: b"ATTACH_WRITE\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE,
            },
            C2Rust_Unnamed_50 {
                zName: b"COMMENTS\0".as_ptr() as *const ::core::ffi::c_char,
                eVal: SQLITE_DBCONFIG_ENABLE_COMMENTS,
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zSetting: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 4 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB SETTING [VALUE]\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSetting = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if sqlite3_strglob(
            b"SQLITE_*\0".as_ptr() as *const ::core::ffi::c_char,
            zSetting,
        ) == 0 as ::core::ffi::c_int
        {
            zSetting = zSetting.offset(7 as ::core::ffi::c_int as isize);
        }
        if sqlite3_strglob(
            b"DBCONFIG_*\0".as_ptr() as *const ::core::ffi::c_char,
            zSetting,
        ) == 0 as ::core::ffi::c_int
        {
            zSetting = zSetting.offset(9 as ::core::ffi::c_int as isize);
        }
        if sqlite3_strglob(
            b"ENABLE_*\0".as_ptr() as *const ::core::ffi::c_char,
            zSetting,
        ) == 0 as ::core::ffi::c_int
        {
            zSetting = zSetting.offset(7 as ::core::ffi::c_int as isize);
        }
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[C2Rust_Unnamed_50; 20]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_50>() as usize)
                as ::core::ffi::c_int
        {
            if strcmp(zSetting, aSetting[i as usize].zName) == 0 as ::core::ffi::c_int {
                break;
            }
            i += 1;
        }
        if i
            >= (::core::mem::size_of::<[C2Rust_Unnamed_50; 20]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_50>() as usize)
                as ::core::ffi::c_int
        {
            Tcl_SetObjResult(
                interp,
                Tcl_NewStringObj(
                    b"unknown sqlite3_db_config setting\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
            return TCL_ERROR;
        }
        if objc == 4 as ::core::ffi::c_int {
            if Tcl_GetIntFromObj(
                interp,
                *objv.offset(3 as ::core::ffi::c_int as isize),
                &raw mut v,
            ) != 0
            {
                return TCL_ERROR;
            }
        } else {
            v = -1 as ::core::ffi::c_int;
        }
        sqlite3_db_config(db, aSetting[i as usize].eVal, v, &raw mut v);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(v));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_sqlite3_txn_state(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zSchema: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut iTxn: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB ?SCHEMA?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSchema = if objc == 3 as ::core::ffi::c_int {
            Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize))
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        iTxn = sqlite3_txn_state(db, zSchema);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(iTxn));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_dbconfig_maindbname_icecube(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        } else {
            if getDbPointer(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut db,
            ) != 0
            {
                return TCL_ERROR;
            }
            rc = sqlite3_db_config(
                db,
                SQLITE_DBCONFIG_MAINDBNAME,
                b"icecube\0".as_ptr() as *const ::core::ffi::c_char,
            );
            Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
            return TCL_OK;
        };
    }
}
unsafe extern "C" fn test_mmap_warm(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            #[link_name = "sqlite3_mmap_warm"]
            fn sqlite3_mmap_warm_0(
                db: *mut sqlite3,
                _: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
        }
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB ?DBNAME?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        } else {
            let mut rc: ::core::ffi::c_int = 0;
            let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
            let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            if getDbPointer(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut db,
            ) != 0
            {
                return TCL_ERROR;
            }
            if objc == 3 as ::core::ffi::c_int {
                zDb = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
            }
            rc = sqlite3_mmap_warm_0(db, zDb);
            Tcl_SetObjResult(
                interp,
                Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
            );
            return TCL_OK;
        };
    }
}
unsafe extern "C" fn test_write_db(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut iOff: Tcl_WideInt = 0 as Tcl_WideInt;
        let mut aData: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut nData: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pFile: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB OFFSET DATA\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetWideIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut iOff,
        ) != 0
        {
            return TCL_ERROR;
        }
        aData = Tcl_GetByteArrayFromObj(
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut nData,
        );
        sqlite3_file_control(
            db,
            b"main\0".as_ptr() as *const ::core::ffi::c_char,
            SQLITE_FCNTL_FILE_POINTER,
            &raw mut pFile as *mut ::core::ffi::c_void,
        );
        rc = (*(*pFile).pMethods)
            .xWrite
            .expect(
                "non-null function pointer",
            )(
            pFile,
            aData as *const ::core::ffi::c_void,
            nData & 0x7fffffff as ::core::ffi::c_int,
            iOff as sqlite3_int64,
        );
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<Tcl_FreeProc>,
            >(1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_register_cksumvfs(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        } else {
            unsafe extern "C" {
                #[link_name = "sqlite3_register_cksumvfs"]
                fn sqlite3_register_cksumvfs_0(
                    _: *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int;
            }
            let mut rc: ::core::ffi::c_int = sqlite3_register_cksumvfs_0(
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            Tcl_SetResult(
                interp,
                sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_unregister_cksumvfs(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        } else {
            unsafe extern "C" {
                #[link_name = "sqlite3_unregister_cksumvfs"]
                fn sqlite3_unregister_cksumvfs_0() -> ::core::ffi::c_int;
            }
            let mut rc: ::core::ffi::c_int = sqlite3_unregister_cksumvfs_0();
            Tcl_SetResult(
                interp,
                sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_decode_hexdb(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zIn: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut a: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut iNext: ::core::ffi::c_int = 0;
        let mut iOffset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut x: [::core::ffi::c_uint; 16] = [0; 16];
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"HEXDB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zIn = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        i = 0 as ::core::ffi::c_int;
        while *zIn.offset(i as isize) != 0 {
            iNext = i;
            while *zIn.offset(iNext as isize) as ::core::ffi::c_int != 0
                && *zIn.offset(iNext as isize) as ::core::ffi::c_int != '\n' as i32
            {
                iNext += 1;
            }
            if *zIn.offset(iNext as isize) as ::core::ffi::c_int == '\n' as i32 {
                iNext += 1;
            }
            while *zIn.offset(i as isize) as ::core::ffi::c_int == ' ' as i32
                || *zIn.offset(i as isize) as ::core::ffi::c_int == '\t' as i32
            {
                i += 1;
            }
            if a.is_null() {
                let mut pgsz: ::core::ffi::c_int = 0;
                rc = sscanf(
                    zIn.offset(i as isize),
                    b"| size %d pagesize %d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut n,
                    &raw mut pgsz,
                );
                if !(rc != 2 as ::core::ffi::c_int) {
                    if pgsz < 512 as ::core::ffi::c_int
                        || pgsz > 65536 as ::core::ffi::c_int
                        || pgsz & pgsz - 1 as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                    {
                        Tcl_AppendResult(
                            interp,
                            b"bad 'pagesize' field\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                        return TCL_ERROR;
                    }
                    n = n + pgsz - 1 as ::core::ffi::c_int
                        & !(pgsz - 1 as ::core::ffi::c_int);
                    if n < 512 as ::core::ffi::c_int {
                        Tcl_AppendResult(
                            interp,
                            b"bad 'size' field\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                        return TCL_ERROR;
                    }
                    a = malloc(n as size_t) as *mut ::core::ffi::c_uchar;
                    if a.is_null() {
                        Tcl_AppendResult(
                            interp,
                            b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                        return TCL_ERROR;
                    }
                    memset(
                        a as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        n as size_t,
                    );
                }
            } else {
                rc = sscanf(
                    zIn.offset(i as isize),
                    b"| page %d offset %d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut j,
                    &raw mut k,
                );
                if rc == 2 as ::core::ffi::c_int {
                    iOffset = k;
                } else {
                    rc = sscanf(
                        zIn.offset(i as isize),
                        b"| %d: %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x %x\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut j,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(3 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(4 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(5 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(6 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(7 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(9 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(10 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(11 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(12 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(13 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(14 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                        (&raw mut x as *mut ::core::ffi::c_uint)
                            .offset(15 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uint,
                    );
                    if rc == 17 as ::core::ffi::c_int {
                        k = iOffset + j;
                        if k + 16 as ::core::ffi::c_int <= n {
                            let mut ii: ::core::ffi::c_int = 0;
                            ii = 0 as ::core::ffi::c_int;
                            while ii < 16 as ::core::ffi::c_int {
                                *a.offset((k + ii) as isize) = (x[ii as usize]
                                    & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
                                ii += 1;
                            }
                        }
                    }
                }
            }
            i = iNext;
        }
        Tcl_SetObjResult(interp, Tcl_NewByteArrayObj(a, n));
        free(a as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_autovacuum_pages_callback(
    mut pClientData: *mut ::core::ffi::c_void,
    mut zSchema: *const ::core::ffi::c_char,
    mut nFilePages: ::core::ffi::c_uint,
    mut nFreePages: ::core::ffi::c_uint,
    mut nBytePerPage: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut pData: *mut AutovacPageData = pClientData as *mut AutovacPageData;
        let mut str: Tcl_DString = Tcl_DString {
            string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            length: 0,
            spaceAvl: 0,
            staticSpace: [0; 200],
        };
        let mut x: ::core::ffi::c_uint = 0;
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        Tcl_DStringInit(&raw mut str);
        Tcl_DStringAppend(&raw mut str, (*pData).zScript, -1 as ::core::ffi::c_int);
        Tcl_DStringAppendElement(&raw mut str, zSchema);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            nFilePages,
        );
        Tcl_DStringAppendElement(
            &raw mut str,
            &raw mut zBuf as *mut ::core::ffi::c_char,
        );
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            nFreePages,
        );
        Tcl_DStringAppendElement(
            &raw mut str,
            &raw mut zBuf as *mut ::core::ffi::c_char,
        );
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            nBytePerPage,
        );
        Tcl_DStringAppendElement(
            &raw mut str,
            &raw mut zBuf as *mut ::core::ffi::c_char,
        );
        Tcl_ResetResult((*pData).interp);
        Tcl_Eval((*pData).interp, str.string);
        Tcl_DStringFree(&raw mut str);
        x = nFreePages;
        Tcl_GetIntFromObj(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            Tcl_GetObjResult((*pData).interp),
            &raw mut x as *mut ::core::ffi::c_int,
        );
        return x;
    }
}
unsafe extern "C" fn test_autovacuum_pages(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pData: *mut AutovacPageData = ::core::ptr::null_mut::<AutovacPageData>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut zScript: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB ?SCRIPT?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zScript = if objc == 3 as ::core::ffi::c_int {
            Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize))
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        if !zScript.is_null() {
            let mut nScript: size_t = strlen(zScript);
            pData = sqlite3_malloc64(
                (::core::mem::size_of::<AutovacPageData>() as usize)
                    .wrapping_add(nScript as usize)
                    .wrapping_add(1 as usize) as sqlite3_uint64,
            ) as *mut AutovacPageData;
            if pData.is_null() {
                Tcl_AppendResult(
                    interp,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
                return TCL_ERROR;
            }
            (*pData).interp = interp;
            (*pData).zScript = pData.offset(1 as ::core::ffi::c_int as isize)
                as *mut AutovacPageData as *mut ::core::ffi::c_char;
            memcpy(
                (*pData).zScript as *mut ::core::ffi::c_void,
                zScript as *const ::core::ffi::c_void,
                nScript.wrapping_add(1 as size_t),
            );
            rc = sqlite3_autovacuum_pages(
                db,
                Some(
                    test_autovacuum_pages_callback
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                        ) -> ::core::ffi::c_uint,
                ),
                pData as *mut ::core::ffi::c_void,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        } else {
            rc = sqlite3_autovacuum_pages(
                db,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
        }
        if rc != 0 {
            let mut zBuf: [::core::ffi::c_char; 1000] = [0; 1000];
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 1000]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"sqlite3_autovacuum_pages() returns %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                rc,
            );
            Tcl_AppendResult(
                interp,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn guess_number_of_cores(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nCore: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
        nCore = sysconf(_SC_NPROCESSORS_ONLN as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        if nCore <= 0 as ::core::ffi::c_uint {
            nCore = 1 as ::core::ffi::c_uint;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(nCore as ::core::ffi::c_int));
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest1_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            static mut sqlite3_search_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_found_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_interrupt_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_open_file_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_sort_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_current_time: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_max_blobsize: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "sqlite3BtreeSharedCacheReport"]
            fn sqlite3BtreeSharedCacheReport_0(
                _: *mut ::core::ffi::c_void,
                _: *mut Tcl_Interp,
                _: ::core::ffi::c_int,
                _: *const *mut Tcl_Obj,
            ) -> ::core::ffi::c_int;
        }
        static mut aCmd: [C2Rust_Unnamed_52; 41] = unsafe {
            [
                C2Rust_Unnamed_52 {
                    zName: b"db_enter\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            db_enter
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"db_leave\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            db_leave
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_mprintf_int\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite3_mprintf_int
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_mprintf_int64\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite3_mprintf_int64
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_mprintf_long\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite3_mprintf_long
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_mprintf_str\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite3_mprintf_str
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_snprintf_str\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite3_snprintf_str
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_mprintf_stronly\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite3_mprintf_stronly
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_mprintf_double\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite3_mprintf_double
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_mprintf_scaled\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite3_mprintf_scaled
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_mprintf_hexdouble\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite3_mprintf_hexdouble
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_mprintf_z_test\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_mprintf_z
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_mprintf_n_test\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_mprintf_n
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_snprintf_int\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_snprintf_int
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_last_insert_rowid\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_last_rowid
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_exec_printf\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_exec_printf
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_exec_hex\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_exec_hex
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_exec\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_exec
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_exec_nr\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_exec_nr
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_get_table_printf\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_get_table_printf
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_close\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite_test_close
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_close_v2\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite_test_close_v2
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_create_function\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_create_function
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_create_aggregate\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_create_aggregate
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_drop_modules\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_drop_modules
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite_register_test_function\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_register_func
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite_abort\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            sqlite_abort
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite_bind\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_bind
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"breakpoint\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_breakpoint
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_key\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_key
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_rekey\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_rekey
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_interrupt\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_interrupt
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_is_interrupted\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_is_interrupted
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite_delete_function\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            delete_function
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite_delete_collation\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            delete_collation
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_get_autocommit\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            get_autocommit
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_busy_timeout\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_busy_timeout
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3_setlk_timeout\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_setlk_timeout
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"printf\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_printf
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"sqlite3IoTrace\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            test_io_trace
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_52 {
                    zName: b"clang_sanitize_address\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *mut ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            clang_sanitize_address
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
            ]
        };
        static mut aObjCmd: [C2Rust_Unnamed_51; 157] = unsafe {
            [
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_db_config\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_sqlite3_db_config
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_txn_state\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_sqlite3_txn_state
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"register_dbstat_vtab\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_register_dbstat_vtab
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_connection_pointer\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        get_sqlite_pointer
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"intarray_addr\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_intarray_addr
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"int64array_addr\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_int64array_addr
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"doublearray_addr\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_doublearray_addr
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"textarray_addr\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_textarray_addr
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_int\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_int
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_zeroblob\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_zeroblob
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_zeroblob64\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_zeroblob64
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_int64\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_int64
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_double\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_double
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_null\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_null
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_text\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_text
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_text16\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_text16
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_blob\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_blob
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_value_from_select\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_value_from_select
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_value_from_preupdate\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_value_from_preupdate
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_carray_bind\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_carray_bind
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"bind_carray_intptr\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        bind_carray_intptr
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_parameter_count\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_parameter_count
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_parameter_name\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_parameter_name
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_bind_parameter_index\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_bind_parameter_index
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_clear_bindings\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_clear_bindings
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_sleep\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_sleep
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_errcode\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_errcode
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_extended_errcode\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_ex_errcode
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_errmsg\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_errmsg
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_error_offset\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_error_offset
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_errmsg16\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_errmsg16
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_set_errmsg\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_set_errmsg
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_open\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_open
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_open16\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_open16
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_open_v2\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_open_v2
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_complete16\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_complete16
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_normalize\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_normalize
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_prepare\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_prepare
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_prepare16\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_prepare16
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_prepare_v2\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_prepare_v2
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_prepare_v3\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_prepare_v3
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_prepare_tkt3134\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_prepare_tkt3134
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_prepare16_v2\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_prepare16_v2
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_finalize\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_finalize
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_stmt_status\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_status
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_reset\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_reset
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_expired\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_expired
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_transfer_bindings\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_transfer_bind
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_changes\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_changes
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_step\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_step
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_sql\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_sql
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_expanded_sql\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_ex_sql
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_next_stmt\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_next_stmt
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_stmt_readonly\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_readonly
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_stmt_isexplain\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_isexplain
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_stmt_explain\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_explain
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_stmt_busy\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_busy
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"uses_stmt_journal\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        uses_stmt_journal
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_release_memory\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_release_memory
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_db_release_memory\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_db_release_memory
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_db_cacheflush\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_db_cacheflush
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_system_errno\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_system_errno
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_db_filename\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_db_filename
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_db_readonly\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_db_readonly
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_soft_heap_limit\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_soft_heap_limit
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_soft_heap_limit64\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_soft_heap_limit
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_hard_heap_limit64\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_hard_heap_limit
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_thread_cleanup\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_thread_cleanup
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_pager_refcounts\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_pager_refcounts
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_load_extension\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_load_extension
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_enable_load_extension\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_enable_load
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_extended_result_codes\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_extended_result_codes
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_limit\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_limit
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"dbconfig_maindbname_icecube\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_dbconfig_maindbname_icecube
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"save_prng_state\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        save_prng_state
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"restore_prng_state\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        restore_prng_state
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"reset_prng_state\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        reset_prng_state
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"prng_seed\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        prng_seed
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"extra_schema_checks\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        extra_schema_checks
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"database_never_corrupt\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        database_never_corrupt
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"database_may_be_corrupt\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        database_may_be_corrupt
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"optimization_control\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        optimization_control
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"tcl_objproc\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        runAsObjProc
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_count\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_column_count
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_data_count\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_data_count
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_type\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_column_type
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_blob\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_column_blob
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_double\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_column_double
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_int64\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_column_int64
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_text\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_utf8
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut sqlite3_stmt,
                                ::core::ffi::c_int,
                            ) -> *const ::core::ffi::c_uchar,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sqlite3_column_text
                                as unsafe extern "C" fn(
                                    *mut sqlite3_stmt,
                                    ::core::ffi::c_int,
                                ) -> *const ::core::ffi::c_uchar,
                        ),
                    ),
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_name\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_utf8
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut sqlite3_stmt,
                                ::core::ffi::c_int,
                            ) -> *const ::core::ffi::c_char,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sqlite3_column_name
                                as unsafe extern "C" fn(
                                    *mut sqlite3_stmt,
                                    ::core::ffi::c_int,
                                ) -> *const ::core::ffi::c_char,
                        ),
                    ),
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_int\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_int
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut sqlite3_stmt,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sqlite3_column_int
                                as unsafe extern "C" fn(
                                    *mut sqlite3_stmt,
                                    ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_bytes\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_int
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut sqlite3_stmt,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sqlite3_column_bytes
                                as unsafe extern "C" fn(
                                    *mut sqlite3_stmt,
                                    ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_decltype\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_utf8
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut sqlite3_stmt,
                                ::core::ffi::c_int,
                            ) -> *const ::core::ffi::c_char,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sqlite3_column_decltype
                                as unsafe extern "C" fn(
                                    *mut sqlite3_stmt,
                                    ::core::ffi::c_int,
                                ) -> *const ::core::ffi::c_char,
                        ),
                    ),
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_bytes16\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_int
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut sqlite3_stmt,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sqlite3_column_bytes16
                                as unsafe extern "C" fn(
                                    *mut sqlite3_stmt,
                                    ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_text16\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_utf16
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut sqlite3_stmt,
                                ::core::ffi::c_int,
                            ) -> *const ::core::ffi::c_void,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sqlite3_column_text16
                                as unsafe extern "C" fn(
                                    *mut sqlite3_stmt,
                                    ::core::ffi::c_int,
                                ) -> *const ::core::ffi::c_void,
                        ),
                    ),
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_name16\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_utf16
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut sqlite3_stmt,
                                ::core::ffi::c_int,
                            ) -> *const ::core::ffi::c_void,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sqlite3_column_name16
                                as unsafe extern "C" fn(
                                    *mut sqlite3_stmt,
                                    ::core::ffi::c_int,
                                ) -> *const ::core::ffi::c_void,
                        ),
                    ),
                },
                C2Rust_Unnamed_51 {
                    zName: b"add_alignment_test_collations\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        add_alignment_test_collations
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_column_decltype16\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_stmt_utf16
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut sqlite3_stmt,
                                ::core::ffi::c_int,
                            ) -> *const ::core::ffi::c_void,
                        >,
                        *mut ::core::ffi::c_void,
                    >(
                        Some(
                            sqlite3_column_decltype16
                                as unsafe extern "C" fn(
                                    *mut sqlite3_stmt,
                                    ::core::ffi::c_int,
                                ) -> *const ::core::ffi::c_void,
                        ),
                    ),
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_create_collation_v2\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_create_collation_v2
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_global_recover\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_global_recover
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"working_64bit_int\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        working_64bit_int
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"vfs_unlink_test\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        vfs_unlink_test
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"vfs_initfail_test\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        vfs_initfail_test
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"vfs_unregister_all\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        vfs_unregister_all
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"vfs_reregister_all\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        vfs_reregister_all
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_test\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_test
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_lasterrno_test\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_lasterrno_test
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_lockproxy_test\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_lockproxy_test
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_chunksize_test\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_chunksize_test
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_sizehint_test\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_sizehint_test
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_data_version\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_data_version
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_persist_wal\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_persist_wal
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_powersafe_overwrite\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_powersafe_overwrite
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_vfsname\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_vfsname
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_reservebytes\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_reservebytes
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_tempfilename\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_tempfilename
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"file_control_external_reader\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        file_control_external_reader
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_vfs_list\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        vfs_list
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_create_function_v2\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_create_function_v2
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"add_test_collate\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_collate
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"add_test_collate_needed\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_collate_needed
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"add_test_function\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_function
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"add_test_utf16bin_collate\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_utf16bin_collate
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_test_errstr\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_errstr
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"tcl_variable_type\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        tcl_variable_type
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"fpnum_compare\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        fpnum_compare
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_enable_shared_cache\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_enable_shared
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_shared_cache_report\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        sqlite3BtreeSharedCacheReport_0
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_libversion_number\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_libversion_number
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_table_column_metadata\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_table_column_metadata
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_blob_reopen\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_blob_reopen
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"pcache_stats\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_pcache_stats
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_wal_checkpoint\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_wal_checkpoint
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_wal_checkpoint_v2\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_wal_checkpoint_v2
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_wal_autocheckpoint\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_wal_autocheckpoint
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"test_sqlite3_log\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_sqlite3_log
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"print_explain_query_plan\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_print_eqp
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"strftime\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        strftime_cmd
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_test_control\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_test_control
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b".treetrace\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_treetrace
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"getrusage\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_getrusage
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"load_static_extension\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        tclLoadStaticExtensionCmd
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sorter_test_fakeheap\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        sorter_test_fakeheap
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sorter_test_sort4_helper\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        sorter_test_sort4_helper
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"vfs_current_time_int64\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        vfsCurrentTimeInt64
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_delete_database\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_delete_database
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"atomic_batch_write\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_atomic_batch_write
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_mmap_warm\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_mmap_warm
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_config_sorterref\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_config_sorterref
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_autovacuum_pages\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_autovacuum_pages
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"decode_hexdb\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_decode_hexdb
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"test_write_db\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_write_db
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_register_cksumvfs\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_register_cksumvfs
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"sqlite3_unregister_cksumvfs\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_unregister_cksumvfs
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"number_of_cores\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        guess_number_of_cores
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
                C2Rust_Unnamed_51 {
                    zName: b"create_null_module\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_create_null_module
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: ::core::ptr::null::<::core::ffi::c_void>()
                        as *mut ::core::ffi::c_void,
                },
            ]
        };
        let mut i: ::core::ffi::c_int = 0;
        unsafe extern "C" {
            static mut sqlite3_sync_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_fullsync_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_opentemp_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_like_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_xferopt_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_pager_readdb_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_pager_writedb_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_pager_writej_count: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_fts3_enable_parentheses: ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_52; 41]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_52>() as usize)
        {
            Tcl_CreateCommand(
                interp,
                aCmd[i as usize].zName,
                aCmd[i as usize].xProc,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_51; 157]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_51>() as usize)
        {
            Tcl_CreateObjCommand(
                interp,
                aObjCmd[i as usize].zName,
                aObjCmd[i as usize].xProc,
                aObjCmd[i as usize].clientData as ClientData,
                None,
            );
            i += 1;
        }
        Tcl_LinkVar(
            interp,
            b"sqlite_search_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_search_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_found_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_found_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_sort_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_sort_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite3_max_blobsize\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_max_blobsize as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_like_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_like_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_interrupt_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_interrupt_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_open_file_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_open_file_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_current_time\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_current_time as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite3_xferopt_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_xferopt_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite3_pager_readdb_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_pager_readdb_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite3_pager_writedb_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_pager_writedb_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite3_pager_writej_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_pager_writej_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"unaligned_string_counter\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut unaligned_string_counter as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_last_needed_collation\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut pzNeededCollation as *mut ::core::ffi::c_char,
            TCL_LINK_STRING | TCL_LINK_READ_ONLY,
        );
        static mut query_plan: *const ::core::ffi::c_char = b"*** OBSOLETE VARIABLE ***\0"
            .as_ptr() as *const ::core::ffi::c_char;
        Tcl_LinkVar(
            interp,
            b"sqlite_query_plan\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut query_plan as *mut ::core::ffi::c_char,
            TCL_LINK_STRING | TCL_LINK_READ_ONLY,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_opentemp_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_opentemp_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_static_bind_value\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite_static_bind_value as *mut ::core::ffi::c_char,
            TCL_LINK_STRING,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_static_bind_nbyte\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite_static_bind_nbyte as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_temp_directory\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_temp_directory as *mut ::core::ffi::c_char,
            TCL_LINK_STRING,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_data_directory\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_data_directory as *mut ::core::ffi::c_char,
            TCL_LINK_STRING,
        );
        Tcl_LinkVar(
            interp,
            b"bitmask_size\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut bitmask_size as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_sync_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_sync_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_fullsync_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_fullsync_count as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_fts3_enable_parentheses\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_fts3_enable_parentheses as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        return TCL_OK;
    }
}
pub const SQLITE_QueryFlattener: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_GroupByOrder: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_FactorOutConst: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_DistinctOpt: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_CoverIdxScan: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_OrderByIdxJoin: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_Transitive: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_OmitNoopJoin: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_Stat4: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_PushDown: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SQLITE_SkipScan: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const SQLITE_PropagateConst: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const SQLITE_BalancedMerge: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const SQLITE_OnePass: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const SQLITE_OrderBySubq: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SQLITE_ExistsToJoin: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const SQLITE_AllOpts: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_EVAL_GLOBAL: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const TCL_EVAL_DIRECT: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const TCL_STATIC: Option<Tcl_FreeProc> = None;
pub const TCL_LEAVE_ERR_MSG: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const TCL_LINK_INT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_LINK_STRING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TCL_LINK_READ_ONLY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        bitmask_size = (::core::mem::size_of::<Bitmask>() as usize)
            .wrapping_mul(8 as usize) as ::core::ffi::c_int
    }
}
#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".CRT$XIB"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
