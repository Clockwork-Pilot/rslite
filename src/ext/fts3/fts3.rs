extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_blob;
    pub type MatchinfoBuffer;
    pub type Fts3DeferredToken;
    pub type Fts3SegReader;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_libversion_number() -> ::core::ffi::c_int;
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
    fn sqlite3_set_last_insert_rowid(_: *mut sqlite3, _: sqlite3_int64);
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
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_errstr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare(
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
    fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_value(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_name(
        _: *mut sqlite3_stmt,
        N: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_data_count(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_blob(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_column_int(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> sqlite3_int64;
    fn sqlite3_column_value(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> *mut sqlite3_value;
    fn sqlite3_column_bytes(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_pointer(
        _: *mut sqlite3_value,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_numeric_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
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
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
    fn sqlite3_result_pointer(
        _: *mut sqlite3_context,
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
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
    fn sqlite3_create_module_v2(
        db: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
        p: *const sqlite3_module,
        pClientData: *mut ::core::ffi::c_void,
        xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
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
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vtab_config(_: *mut sqlite3, op: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3Fts3HashInit(
        pNew: *mut Fts3Hash,
        keyClass: ::core::ffi::c_char,
        copyKey: ::core::ffi::c_char,
    );
    fn sqlite3Fts3HashInsert(
        _: *mut Fts3Hash,
        pKey: *const ::core::ffi::c_void,
        nKey: ::core::ffi::c_int,
        pData: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3Fts3HashClear(_: *mut Fts3Hash);
    fn sqlite3Fts3UpdateMethod(
        _: *mut sqlite3_vtab,
        _: ::core::ffi::c_int,
        _: *mut *mut sqlite3_value,
        _: *mut sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3PendingTermsFlush(_: *mut Fts3Table) -> ::core::ffi::c_int;
    fn sqlite3Fts3PendingTermsClear(_: *mut Fts3Table);
    fn sqlite3Fts3Optimize(_: *mut Fts3Table) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegReaderNew(
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
        _: sqlite3_int64,
        _: sqlite3_int64,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut *mut Fts3SegReader,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegReaderPending(
        _: *mut Fts3Table,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut *mut Fts3SegReader,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegReaderFree(_: *mut Fts3SegReader);
    fn sqlite3Fts3AllSegdirs(
        _: *mut Fts3Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut *mut sqlite3_stmt,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3ReadBlock(
        _: *mut Fts3Table,
        _: sqlite3_int64,
        _: *mut *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SelectDoctotal(
        _: *mut Fts3Table,
        _: *mut *mut sqlite3_stmt,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3FreeDeferredTokens(_: *mut Fts3Cursor);
    fn sqlite3Fts3DeferToken(
        _: *mut Fts3Cursor,
        _: *mut Fts3PhraseToken,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3CacheDeferredDoclists(_: *mut Fts3Cursor) -> ::core::ffi::c_int;
    fn sqlite3Fts3FreeDeferredDoclists(_: *mut Fts3Cursor);
    fn sqlite3Fts3DeferredTokenList(
        _: *mut Fts3DeferredToken,
        _: *mut *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegmentsClose(_: *mut Fts3Table);
    fn sqlite3Fts3MaxLevel(_: *mut Fts3Table, _: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegReaderStart(
        _: *mut Fts3Table,
        _: *mut Fts3MultiSegReader,
        _: *mut Fts3SegFilter,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegReaderStep(
        _: *mut Fts3Table,
        _: *mut Fts3MultiSegReader,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegReaderFinish(_: *mut Fts3MultiSegReader);
    fn sqlite3Fts3Incrmerge(
        _: *mut Fts3Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3NextToken(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3Fts3InitHashTable(
        _: *mut sqlite3,
        _: *mut Fts3Hash,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3InitTokenizer(
        pHash: *mut Fts3Hash,
        _: *const ::core::ffi::c_char,
        _: *mut *mut sqlite3_tokenizer,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3IsIdChar(_: ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3Fts3Offsets(_: *mut sqlite3_context, _: *mut Fts3Cursor);
    fn sqlite3Fts3Snippet(
        _: *mut sqlite3_context,
        _: *mut Fts3Cursor,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3Fts3Matchinfo(
        _: *mut sqlite3_context,
        _: *mut Fts3Cursor,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3Fts3MIBufferFree(p: *mut MatchinfoBuffer);
    fn sqlite3Fts3ExprParse(
        _: *mut sqlite3_tokenizer,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut *mut Fts3Expr,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3ExprFree(_: *mut Fts3Expr);
    fn sqlite3Fts3ExprInitTestInterface(db: *mut sqlite3, _: *mut Fts3Hash) -> ::core::ffi::c_int;
    fn sqlite3Fts3MallocZero(nByte: i64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3Fts3InitAux(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3Fts3MsrIncrStart(
        _: *mut Fts3Table,
        _: *mut Fts3MultiSegReader,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3MsrIncrNext(
        _: *mut Fts3Table,
        _: *mut Fts3MultiSegReader,
        _: *mut sqlite3_int64,
        _: *mut *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3MsrOvfl(
        _: *mut Fts3Cursor,
        _: *mut Fts3MultiSegReader,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3MsrIncrRestart(pCsr: *mut Fts3MultiSegReader) -> ::core::ffi::c_int;
    fn sqlite3Fts3InitTok(
        _: *mut sqlite3,
        _: *mut Fts3Hash,
        xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3ExprIterate(
        _: *mut Fts3Expr,
        x: Option<
            unsafe extern "C" fn(
                *mut Fts3Expr,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3IntegrityCheck(
        p: *mut Fts3Table,
        pbOk: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SimpleTokenizerModule(ppModule: *mut *const sqlite3_tokenizer_module);
    fn sqlite3Fts3PorterTokenizerModule(ppModule: *mut *const sqlite3_tokenizer_module);
    fn sqlite3Fts3UnicodeTokenizer(ppModule: *mut *const sqlite3_tokenizer_module);
}

#[cfg(feature = "test")]
extern "C" {
    fn sqlite3Fts3InitTerm(db: *mut sqlite3) -> ::core::ffi::c_int;
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
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
        unsafe extern "C" fn(*mut sqlite3_vtab, *mut sqlite3_index_info) -> ::core::ffi::c_int,
    >,
    pub xDisconnect: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut *mut sqlite3_vtab_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xFilter: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xEof: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xColumn: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor, *mut sqlite3_int64) -> ::core::ffi::c_int,
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
        unsafe extern "C" fn(*mut sqlite3_vtab, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub xSavepoint:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xRelease:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xRollbackTo:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xShadowName: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
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
pub struct sqlite3_tokenizer_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_tokenizer) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_tokenizer_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_tokenizer_cursor) -> ::core::ffi::c_int>,
    pub xNext: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer_cursor,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xLanguageid: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer_cursor,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer_cursor {
    pub pTokenizer: *mut sqlite3_tokenizer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer {
    pub pModule: *const sqlite3_tokenizer_module,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Hash {
    pub keyClass: ::core::ffi::c_char,
    pub copyKey: ::core::ffi::c_char,
    pub count: ::core::ffi::c_int,
    pub first: *mut Fts3HashElem,
    pub htsize: ::core::ffi::c_int,
    pub ht: *mut _fts3ht,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fts3ht {
    pub count: ::core::ffi::c_int,
    pub chain: *mut Fts3HashElem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3HashElem {
    pub next: *mut Fts3HashElem,
    pub prev: *mut Fts3HashElem,
    pub data: *mut ::core::ffi::c_void,
    pub pKey: *mut ::core::ffi::c_void,
    pub nKey: ::core::ffi::c_int,
}
pub type u8_0 = ::core::ffi::c_uchar;
pub type i16_0 = ::core::ffi::c_short;
pub type u32_0 = ::core::ffi::c_uint;
pub type u64_0 = sqlite3_uint64;
pub type i64_0 = sqlite3_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Table {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub zDb: *const ::core::ffi::c_char,
    pub zName: *const ::core::ffi::c_char,
    pub nColumn: ::core::ffi::c_int,
    pub azColumn: *mut *mut ::core::ffi::c_char,
    pub abNotindexed: *mut u8_0,
    pub pTokenizer: *mut sqlite3_tokenizer,
    pub zContentTbl: *mut ::core::ffi::c_char,
    pub zLanguageid: *mut ::core::ffi::c_char,
    pub nAutoincrmerge: ::core::ffi::c_int,
    pub nLeafAdd: u32_0,
    pub bLock: ::core::ffi::c_int,
    pub aStmt: [*mut sqlite3_stmt; 40],
    pub pSeekStmt: *mut sqlite3_stmt,
    pub zReadExprlist: *mut ::core::ffi::c_char,
    pub zWriteExprlist: *mut ::core::ffi::c_char,
    pub nNodeSize: ::core::ffi::c_int,
    pub bFts4: u8_0,
    pub bHasStat: u8_0,
    pub bHasDocsize: u8_0,
    pub bDescIdx: u8_0,
    pub bIgnoreSavepoint: u8_0,
    pub nPgsz: ::core::ffi::c_int,
    pub zSegmentsTbl: *mut ::core::ffi::c_char,
    pub pSegments: *mut sqlite3_blob,
    pub iSavepoint: ::core::ffi::c_int,
    pub nIndex: ::core::ffi::c_int,
    pub aIndex: *mut Fts3Index,
    pub nMaxPendingData: ::core::ffi::c_int,
    pub nPendingData: ::core::ffi::c_int,
    pub iPrevDocid: sqlite_int64,
    pub iPrevLangid: ::core::ffi::c_int,
    pub bPrevDelete: ::core::ffi::c_int,
    pub bNoIncrDoclist: ::core::ffi::c_int,
    pub nMergeCount: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Index {
    pub nPrefix: ::core::ffi::c_int,
    pub hPending: Fts3Hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Cursor {
    pub base: sqlite3_vtab_cursor,
    pub eSearch: i16_0,
    pub isEof: u8_0,
    pub isRequireSeek: u8_0,
    pub bSeekStmt: u8_0,
    pub pStmt: *mut sqlite3_stmt,
    pub pExpr: *mut Fts3Expr,
    pub iLangid: ::core::ffi::c_int,
    pub nPhrase: ::core::ffi::c_int,
    pub pDeferred: *mut Fts3DeferredToken,
    pub iPrevId: sqlite3_int64,
    pub pNextId: *mut ::core::ffi::c_char,
    pub aDoclist: *mut ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
    pub bDesc: u8_0,
    pub eEvalmode: ::core::ffi::c_int,
    pub nRowAvg: ::core::ffi::c_int,
    pub nDoc: sqlite3_int64,
    pub iMinDocid: i64_0,
    pub iMaxDocid: i64_0,
    pub isMatchinfoNeeded: ::core::ffi::c_int,
    pub pMIBuffer: *mut MatchinfoBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Expr {
    pub eType: ::core::ffi::c_int,
    pub nNear: ::core::ffi::c_int,
    pub pParent: *mut Fts3Expr,
    pub pLeft: *mut Fts3Expr,
    pub pRight: *mut Fts3Expr,
    pub pPhrase: *mut Fts3Phrase,
    pub iDocid: sqlite3_int64,
    pub bEof: u8_0,
    pub bStart: u8_0,
    pub bDeferred: u8_0,
    pub iPhrase: ::core::ffi::c_int,
    pub aMI: *mut u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Phrase {
    pub doclist: Fts3Doclist,
    pub bIncr: ::core::ffi::c_int,
    pub iDoclistToken: ::core::ffi::c_int,
    pub pOrPoslist: *mut ::core::ffi::c_char,
    pub iOrDocid: i64_0,
    pub nToken: ::core::ffi::c_int,
    pub iColumn: ::core::ffi::c_int,
    pub aToken: [Fts3PhraseToken; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3PhraseToken {
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub isPrefix: ::core::ffi::c_int,
    pub bFirst: ::core::ffi::c_int,
    pub pDeferred: *mut Fts3DeferredToken,
    pub pSegcsr: *mut Fts3MultiSegReader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3MultiSegReader {
    pub apSegment: *mut *mut Fts3SegReader,
    pub nSegment: ::core::ffi::c_int,
    pub nAdvance: ::core::ffi::c_int,
    pub pFilter: *mut Fts3SegFilter,
    pub aBuffer: *mut ::core::ffi::c_char,
    pub nBuffer: i64_0,
    pub iColFilter: ::core::ffi::c_int,
    pub bRestart: ::core::ffi::c_int,
    pub nCost: ::core::ffi::c_int,
    pub bLookup: ::core::ffi::c_int,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTerm: ::core::ffi::c_int,
    pub aDoclist: *mut ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3SegFilter {
    pub zTerm: *const ::core::ffi::c_char,
    pub nTerm: ::core::ffi::c_int,
    pub iCol: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Doclist {
    pub aAll: *mut ::core::ffi::c_char,
    pub nAll: ::core::ffi::c_int,
    pub pNextDocid: *mut ::core::ffi::c_char,
    pub iDocid: sqlite3_int64,
    pub bFreeList: ::core::ffi::c_int,
    pub pList: *mut ::core::ffi::c_char,
    pub nList: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenDoclist {
    pub bIgnore: ::core::ffi::c_int,
    pub iDocid: sqlite3_int64,
    pub pList: *mut ::core::ffi::c_char,
    pub nList: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TermSelect {
    pub aaOutput: [*mut ::core::ffi::c_char; 16],
    pub anOutput: [::core::ffi::c_int; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3HashWrapper {
    pub hash: Fts3Hash,
    pub nRef: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Overloaded {
    pub zName: *const ::core::ffi::c_char,
    pub xFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3TokenAndCost {
    pub pPhrase: *mut Fts3Phrase,
    pub iToken: ::core::ffi::c_int,
    pub pToken: *mut Fts3PhraseToken,
    pub pRoot: *mut Fts3Expr,
    pub nOvfl: ::core::ffi::c_int,
    pub iCol: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts4Option {
    pub zOpt: *const ::core::ffi::c_char,
    pub nOpt: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_AUTH: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT_VTAB: ::core::ffi::c_int =
    SQLITE_CORRUPT | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_PREPARE_PERSISTENT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_SCAN_UNIQUE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_GT: ::core::ffi::c_int = 4;
pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8;
pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16;
pub const SQLITE_INDEX_CONSTRAINT_GE: ::core::ffi::c_int = 32;
pub const SQLITE_INDEX_CONSTRAINT_MATCH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SQLITE_VTAB_CONSTRAINT_SUPPORT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_VTAB_INNOCUOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FTS3_HASH_STRING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FTS3_MERGE_COUNT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const FTS3_MAX_PENDING_DATA: ::core::ffi::c_int =
    1 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
pub const FTS3_VARINT_MAX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FTS3_BUFFER_PADDING: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const POS_COLUMN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POS_END: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LARGEST_INT64: i64_0 =
    0xffffffff as i64_0 | (0x7fffffff as ::core::ffi::c_int as i64_0) << 32 as ::core::ffi::c_int;
pub const SMALLEST_INT64: i64_0 = -(1 as ::core::ffi::c_int) as i64_0 - LARGEST_INT64;
pub const FTS_CORRUPT_VTAB: ::core::ffi::c_int = SQLITE_CORRUPT_VTAB;
pub const FTS3_FULLSCAN_SEARCH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FTS3_DOCID_SEARCH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FTS3_FULLTEXT_SEARCH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FTS3_HAVE_LANGID: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const FTS3_HAVE_DOCID_GE: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const FTS3_HAVE_DOCID_LE: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const FTSQUERY_NEAR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FTSQUERY_NOT: ::core::ffi::c_int = 2;
pub const FTSQUERY_AND: ::core::ffi::c_int = 3;
pub const FTSQUERY_OR: ::core::ffi::c_int = 4;
pub const FTSQUERY_PHRASE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FTS3_SEGCURSOR_PENDING: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FTS3_SEGCURSOR_ALL: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const FTS3_SEGMENT_REQUIRE_POS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_IGNORE_EMPTY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_COLUMN_FILTER: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_PREFIX: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_FIRST: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3PutVarint(
    mut p: *mut ::core::ffi::c_char,
    mut v: sqlite_int64,
) -> ::core::ffi::c_int {
    let mut q: *mut ::core::ffi::c_uchar = p as *mut ::core::ffi::c_uchar;
    let mut vu: sqlite_uint64 = v as sqlite_uint64;
    loop {
        let fresh11 = q;
        q = q.offset(1);
        *fresh11 = (vu & 0x7f as sqlite_uint64 | 0x80 as sqlite_uint64) as ::core::ffi::c_uchar;
        vu >>= 7 as ::core::ffi::c_int;
        if !(vu != 0 as sqlite_uint64) {
            break;
        }
    }
    let ref mut fresh12 = *q.offset(-(1 as ::core::ffi::c_int) as isize);
    *fresh12 =
        (*fresh12 as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    return q.offset_from(p as *mut ::core::ffi::c_uchar) as ::core::ffi::c_long
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3GetVarintU(
    mut pBuf: *const ::core::ffi::c_char,
    mut v: *mut sqlite_uint64,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_uchar = pBuf as *const ::core::ffi::c_uchar;
    let mut pStart: *const ::core::ffi::c_uchar = p;
    let mut a: u32_0 = 0;
    let mut b: u64_0 = 0;
    let mut shift: ::core::ffi::c_int = 0;
    let fresh6 = p;
    p = p.offset(1);
    a = *fresh6 as u32_0;
    if a & 0x80 as u32_0 == 0 as u32_0 {
        *v = a as sqlite_uint64;
        return 1 as ::core::ffi::c_int;
    }
    let fresh7 = p;
    p = p.offset(1);
    a = a & 0x7f as u32_0 | ((*fresh7 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as u32_0;
    if a & 0x4000 as u32_0 == 0 as u32_0 {
        *v = a as sqlite_uint64;
        return 2 as ::core::ffi::c_int;
    }
    let fresh8 = p;
    p = p.offset(1);
    a = a & 0x3fff as u32_0
        | ((*fresh8 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as u32_0;
    if a & 0x200000 as ::core::ffi::c_int as u32_0 == 0 as u32_0 {
        *v = a as sqlite_uint64;
        return 3 as ::core::ffi::c_int;
    }
    let fresh9 = p;
    p = p.offset(1);
    a = a & 0x1fffff as ::core::ffi::c_int as u32_0
        | ((*fresh9 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as u32_0;
    if a & 0x10000000 as ::core::ffi::c_int as u32_0 == 0 as u32_0 {
        *v = a as sqlite_uint64;
        return 4 as ::core::ffi::c_int;
    }
    b = (a & 0xfffffff as ::core::ffi::c_int as u32_0) as u64_0;
    shift = 28 as ::core::ffi::c_int;
    while shift <= 63 as ::core::ffi::c_int {
        let fresh10 = p;
        p = p.offset(1);
        let mut c: u64_0 = *fresh10 as u64_0;
        b = b.wrapping_add((c & 0x7f as u64_0) << shift);
        if c & 0x80 as u64_0 == 0 as u64_0 {
            break;
        }
        shift += 7 as ::core::ffi::c_int;
    }
    *v = b as sqlite_uint64;
    return p.offset_from(pStart) as ::core::ffi::c_long as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3GetVarint(
    mut pBuf: *const ::core::ffi::c_char,
    mut v: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    return sqlite3Fts3GetVarintU(pBuf, v as *mut sqlite_uint64);
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3GetVarintBounded(
    mut pBuf: *const ::core::ffi::c_char,
    mut pEnd: *const ::core::ffi::c_char,
    mut v: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_uchar = pBuf as *const ::core::ffi::c_uchar;
    let mut pStart: *const ::core::ffi::c_uchar = p;
    let mut pX: *const ::core::ffi::c_uchar = pEnd as *const ::core::ffi::c_uchar;
    let mut b: u64_0 = 0 as u64_0;
    let mut shift: ::core::ffi::c_int = 0;
    shift = 0 as ::core::ffi::c_int;
    while shift <= 63 as ::core::ffi::c_int {
        let mut c: u64_0 = (if p < pX {
            *p as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as u64_0;
        p = p.offset(1);
        b = b.wrapping_add((c & 0x7f as u64_0) << shift);
        if c & 0x80 as u64_0 == 0 as u64_0 {
            break;
        }
        shift += 7 as ::core::ffi::c_int;
    }
    *v = b as sqlite_int64;
    return p.offset_from(pStart) as ::core::ffi::c_long as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3GetVarint32(
    mut p: *const ::core::ffi::c_char,
    mut pi: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ptr: *const ::core::ffi::c_uchar = p as *const ::core::ffi::c_uchar;
    let mut a: u32_0 = 0;
    let fresh2 = ptr;
    ptr = ptr.offset(1);
    a = *fresh2 as u32_0;
    let fresh3 = ptr;
    ptr = ptr.offset(1);
    a = a & 0x7f as u32_0 | ((*fresh3 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as u32_0;
    if a & 0x4000 as u32_0 == 0 as u32_0 {
        *pi = a as ::core::ffi::c_int;
        return 2 as ::core::ffi::c_int;
    }
    let fresh4 = ptr;
    ptr = ptr.offset(1);
    a = a & 0x3fff as u32_0
        | ((*fresh4 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as u32_0;
    if a & 0x200000 as ::core::ffi::c_int as u32_0 == 0 as u32_0 {
        *pi = a as ::core::ffi::c_int;
        return 3 as ::core::ffi::c_int;
    }
    let fresh5 = ptr;
    ptr = ptr.offset(1);
    a = a & 0x1fffff as ::core::ffi::c_int as u32_0
        | ((*fresh5 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as u32_0;
    if a & 0x10000000 as ::core::ffi::c_int as u32_0 == 0 as u32_0 {
        *pi = a as ::core::ffi::c_int;
        return 4 as ::core::ffi::c_int;
    }
    a = a & 0xfffffff as ::core::ffi::c_int as u32_0;
    *pi = (a
        | ((*ptr as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as u32_0)
            << 28 as ::core::ffi::c_int) as ::core::ffi::c_int;
    return 5 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3VarintLen(mut v: sqlite3_uint64) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        i += 1;
        v >>= 7 as ::core::ffi::c_int;
        if !(v != 0 as sqlite3_uint64) {
            break;
        }
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3Dequote(mut z: *mut ::core::ffi::c_char) {
    let mut quote: ::core::ffi::c_char = 0;
    quote = *z.offset(0 as ::core::ffi::c_int as isize);
    if quote as ::core::ffi::c_int == '[' as i32
        || quote as ::core::ffi::c_int == '\'' as i32
        || quote as ::core::ffi::c_int == '"' as i32
        || quote as ::core::ffi::c_int == '`' as i32
    {
        let mut iIn: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if quote as ::core::ffi::c_int == '[' as i32 {
            quote = ']' as i32 as ::core::ffi::c_char;
        }
        while *z.offset(iIn as isize) != 0 {
            if *z.offset(iIn as isize) as ::core::ffi::c_int == quote as ::core::ffi::c_int {
                if *z.offset((iIn + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    != quote as ::core::ffi::c_int
                {
                    break;
                }
                let fresh13 = iOut;
                iOut = iOut + 1;
                *z.offset(fresh13 as isize) = quote;
                iIn += 2 as ::core::ffi::c_int;
            } else {
                let fresh14 = iIn;
                iIn = iIn + 1;
                let fresh15 = iOut;
                iOut = iOut + 1;
                *z.offset(fresh15 as isize) = *z.offset(fresh14 as isize);
            }
        }
        *z.offset(iOut as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn fts3GetDeltaVarint(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut pVal: *mut sqlite3_int64,
) {
    let mut iVal: sqlite3_int64 = 0;
    *pp = (*pp).offset(sqlite3Fts3GetVarint(*pp, &raw mut iVal) as isize);
    *pVal += iVal;
}
unsafe extern "C" fn fts3GetReverseVarint(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut pStart: *mut ::core::ffi::c_char,
    mut pVal: *mut sqlite3_int64,
) {
    let mut iVal: sqlite3_int64 = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = (*pp).offset(-(2 as ::core::ffi::c_int as isize));
    while p >= pStart && *p as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        p = p.offset(-1);
    }
    p = p.offset(1);
    *pp = p;
    sqlite3Fts3GetVarint(p, &raw mut iVal);
    *pVal = iVal;
}
unsafe extern "C" fn fts3DisconnectMethod(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut i: ::core::ffi::c_int = 0;
    sqlite3_finalize((*p).pSeekStmt);
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[*mut sqlite3_stmt; 40]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut sqlite3_stmt>() as usize)
            as ::core::ffi::c_int
    {
        sqlite3_finalize((*p).aStmt[i as usize]);
        i += 1;
    }
    sqlite3_free((*p).zSegmentsTbl as *mut ::core::ffi::c_void);
    sqlite3_free((*p).zReadExprlist as *mut ::core::ffi::c_void);
    sqlite3_free((*p).zWriteExprlist as *mut ::core::ffi::c_void);
    sqlite3_free((*p).zContentTbl as *mut ::core::ffi::c_void);
    sqlite3_free((*p).zLanguageid as *mut ::core::ffi::c_void);
    (*(*(*p).pTokenizer).pModule)
        .xDestroy
        .expect("non-null function pointer")((*p).pTokenizer);
    sqlite3_free(p as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3ErrMsg(
    mut pzErr: *mut *mut ::core::ffi::c_char,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    sqlite3_free(*pzErr as *mut ::core::ffi::c_void);
    ap = args.clone();
    *pzErr = sqlite3_vmprintf(zFormat, ap.as_va_list());
}
unsafe extern "C" fn fts3DbExec(
    mut pRc: *mut ::core::ffi::c_int,
    mut db: *mut sqlite3,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *pRc != 0 {
        return;
    }
    ap = args.clone();
    zSql = sqlite3_vmprintf(zFormat, ap.as_va_list());
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
unsafe extern "C" fn fts3DestroyMethod(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut zDb: *const ::core::ffi::c_char = (*p).zDb;
    let mut db: *mut sqlite3 = (*p).db;
    fts3DbExec(
        &raw mut rc,
        db,
        b"DROP TABLE IF EXISTS %Q.'%q_segments';DROP TABLE IF EXISTS %Q.'%q_segdir';DROP TABLE IF EXISTS %Q.'%q_docsize';DROP TABLE IF EXISTS %Q.'%q_stat';%s DROP TABLE IF EXISTS %Q.'%q_content';\0"
            as *const u8 as *const ::core::ffi::c_char,
        zDb,
        (*p).zName,
        zDb,
        (*p).zName,
        zDb,
        (*p).zName,
        zDb,
        (*p).zName,
        if !(*p).zContentTbl.is_null() {
            b"--\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        zDb,
        (*p).zName,
    );
    return if rc == SQLITE_OK {
        fts3DisconnectMethod(pVtab)
    } else {
        rc
    };
}
unsafe extern "C" fn fts3DeclareVtab(mut pRc: *mut ::core::ffi::c_int, mut p: *mut Fts3Table) {
    if *pRc == SQLITE_OK {
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut zCols: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut zLanguageid: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        zLanguageid = if !(*p).zLanguageid.is_null() {
            (*p).zLanguageid as *const ::core::ffi::c_char
        } else {
            b"__langid\0" as *const u8 as *const ::core::ffi::c_char
        };
        sqlite3_vtab_config(
            (*p).db,
            SQLITE_VTAB_CONSTRAINT_SUPPORT,
            1 as ::core::ffi::c_int,
        );
        sqlite3_vtab_config((*p).db, SQLITE_VTAB_INNOCUOUS);
        zCols = sqlite3_mprintf(
            b"%Q, \0" as *const u8 as *const ::core::ffi::c_char,
            *(*p).azColumn.offset(0 as ::core::ffi::c_int as isize),
        );
        i = 1 as ::core::ffi::c_int;
        while !zCols.is_null() && i < (*p).nColumn {
            zCols = sqlite3_mprintf(
                b"%z%Q, \0" as *const u8 as *const ::core::ffi::c_char,
                zCols,
                *(*p).azColumn.offset(i as isize),
            );
            i += 1;
        }
        zSql = sqlite3_mprintf(
            b"CREATE TABLE x(%s %Q HIDDEN, docid HIDDEN, %Q HIDDEN)\0" as *const u8
                as *const ::core::ffi::c_char,
            zCols,
            (*p).zName,
            zLanguageid,
        );
        if zCols.is_null() || zSql.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            rc = sqlite3_declare_vtab((*p).db, zSql);
        }
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        sqlite3_free(zCols as *mut ::core::ffi::c_void);
        *pRc = rc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3CreateStatTable(
    mut pRc: *mut ::core::ffi::c_int,
    mut p: *mut Fts3Table,
) {
    fts3DbExec(
        pRc,
        (*p).db,
        b"CREATE TABLE IF NOT EXISTS %Q.'%q_stat'(id INTEGER PRIMARY KEY, value BLOB);\0"
            as *const u8 as *const ::core::ffi::c_char,
        (*p).zDb,
        (*p).zName,
    );
    if *pRc == SQLITE_OK {
        (*p).bHasStat = 1 as u8_0;
    }
}
unsafe extern "C" fn fts3CreateTables(mut p: *mut Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut i: ::core::ffi::c_int = 0;
    let mut db: *mut sqlite3 = (*p).db;
    if (*p).zContentTbl.is_null() {
        let mut zLanguageid: *const ::core::ffi::c_char = (*p).zLanguageid;
        let mut zContentCols: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        zContentCols = sqlite3_mprintf(
            b"docid INTEGER PRIMARY KEY\0" as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while !zContentCols.is_null() && i < (*p).nColumn {
            let mut z: *mut ::core::ffi::c_char = *(*p).azColumn.offset(i as isize);
            zContentCols = sqlite3_mprintf(
                b"%z, 'c%d%q'\0" as *const u8 as *const ::core::ffi::c_char,
                zContentCols,
                i,
                z,
            );
            i += 1;
        }
        if !zLanguageid.is_null() && !zContentCols.is_null() {
            zContentCols = sqlite3_mprintf(
                b"%z, langid\0" as *const u8 as *const ::core::ffi::c_char,
                zContentCols,
                zLanguageid,
            );
        }
        if zContentCols.is_null() {
            rc = SQLITE_NOMEM;
        }
        fts3DbExec(
            &raw mut rc,
            db,
            b"CREATE TABLE %Q.'%q_content'(%s)\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).zDb,
            (*p).zName,
            zContentCols,
        );
        sqlite3_free(zContentCols as *mut ::core::ffi::c_void);
    }
    fts3DbExec(
        &raw mut rc,
        db,
        b"CREATE TABLE %Q.'%q_segments'(blockid INTEGER PRIMARY KEY, block BLOB);\0" as *const u8
            as *const ::core::ffi::c_char,
        (*p).zDb,
        (*p).zName,
    );
    fts3DbExec(
        &raw mut rc,
        db,
        b"CREATE TABLE %Q.'%q_segdir'(level INTEGER,idx INTEGER,start_block INTEGER,leaves_end_block INTEGER,end_block INTEGER,root BLOB,PRIMARY KEY(level, idx));\0"
            as *const u8 as *const ::core::ffi::c_char,
        (*p).zDb,
        (*p).zName,
    );
    if (*p).bHasDocsize != 0 {
        fts3DbExec(
            &raw mut rc,
            db,
            b"CREATE TABLE %Q.'%q_docsize'(docid INTEGER PRIMARY KEY, size BLOB);\0" as *const u8
                as *const ::core::ffi::c_char,
            (*p).zDb,
            (*p).zName,
        );
    }
    if (*p).bHasStat != 0 {
        sqlite3Fts3CreateStatTable(&raw mut rc, p);
    }
    return rc;
}
unsafe extern "C" fn fts3DatabasePageSize(mut pRc: *mut ::core::ffi::c_int, mut p: *mut Fts3Table) {
    if *pRc == SQLITE_OK {
        let mut rc: ::core::ffi::c_int = 0;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        zSql = sqlite3_mprintf(
            b"PRAGMA %Q.page_size\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).zDb,
        );
        if zSql.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            rc = sqlite3_prepare(
                (*p).db,
                zSql,
                -(1 as ::core::ffi::c_int),
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc == SQLITE_OK {
                sqlite3_step(pStmt);
                (*p).nPgsz = sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
                rc = sqlite3_finalize(pStmt);
            } else if rc == SQLITE_AUTH {
                (*p).nPgsz = 1024 as ::core::ffi::c_int;
                rc = SQLITE_OK;
            }
        }
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        *pRc = rc;
    }
}
unsafe extern "C" fn fts3IsSpecialColumn(
    mut z: *const ::core::ffi::c_char,
    mut pnKey: *mut ::core::ffi::c_int,
    mut pzValue: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut zValue: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zCsr: *const ::core::ffi::c_char = z;
    while *zCsr as ::core::ffi::c_int != '=' as i32 {
        if *zCsr as ::core::ffi::c_int == '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        zCsr = zCsr.offset(1);
    }
    *pnKey = zCsr.offset_from(z) as ::core::ffi::c_long as ::core::ffi::c_int;
    zValue = sqlite3_mprintf(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        zCsr.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
    );
    if !zValue.is_null() {
        sqlite3Fts3Dequote(zValue);
    }
    *pzValue = zValue;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3Appendf(
    mut pRc: *mut ::core::ffi::c_int,
    mut pz: *mut *mut ::core::ffi::c_char,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    if *pRc == SQLITE_OK {
        let mut ap: ::core::ffi::VaListImpl;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ap = args.clone();
        z = sqlite3_vmprintf(zFormat, ap.as_va_list());
        if !z.is_null() && !(*pz).is_null() {
            let mut z2: *mut ::core::ffi::c_char =
                sqlite3_mprintf(b"%s%s\0" as *const u8 as *const ::core::ffi::c_char, *pz, z);
            sqlite3_free(z as *mut ::core::ffi::c_void);
            z = z2;
        }
        if z.is_null() {
            *pRc = SQLITE_NOMEM;
        }
        sqlite3_free(*pz as *mut ::core::ffi::c_void);
        *pz = z;
    }
}
unsafe extern "C" fn fts3QuoteId(
    mut zInput: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut nRet: sqlite3_int64 = 0;
    let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    nRet = (2 as ::core::ffi::c_int
        + strlen(zInput) as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as sqlite3_int64;
    zRet = sqlite3_malloc64(nRet as sqlite3_uint64) as *mut ::core::ffi::c_char;
    if !zRet.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut z: *mut ::core::ffi::c_char = zRet;
        let fresh50 = z;
        z = z.offset(1);
        *fresh50 = '"' as i32 as ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while *zInput.offset(i as isize) != 0 {
            if *zInput.offset(i as isize) as ::core::ffi::c_int == '"' as i32 {
                let fresh51 = z;
                z = z.offset(1);
                *fresh51 = '"' as i32 as ::core::ffi::c_char;
            }
            let fresh52 = z;
            z = z.offset(1);
            *fresh52 = *zInput.offset(i as isize);
            i += 1;
        }
        let fresh53 = z;
        z = z.offset(1);
        *fresh53 = '"' as i32 as ::core::ffi::c_char;
        let fresh54 = z;
        z = z.offset(1);
        *fresh54 = '\0' as i32 as ::core::ffi::c_char;
    }
    return zRet;
}
unsafe extern "C" fn fts3ReadExprList(
    mut p: *mut Fts3Table,
    mut zFunc: *const ::core::ffi::c_char,
    mut pRc: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zFunction: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if (*p).zContentTbl.is_null() {
        if zFunc.is_null() {
            zFunction =
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else {
            zFunction = fts3QuoteId(zFunc);
            zFree = zFunction;
        }
        fts3Appendf(
            pRc,
            &raw mut zRet,
            b"docid\0" as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nColumn {
            fts3Appendf(
                pRc,
                &raw mut zRet,
                b",%s(x.'c%d%q')\0" as *const u8 as *const ::core::ffi::c_char,
                zFunction,
                i,
                *(*p).azColumn.offset(i as isize),
            );
            i += 1;
        }
        if !(*p).zLanguageid.is_null() {
            fts3Appendf(
                pRc,
                &raw mut zRet,
                b", x.%Q\0" as *const u8 as *const ::core::ffi::c_char,
                b"langid\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        sqlite3_free(zFree as *mut ::core::ffi::c_void);
    } else {
        fts3Appendf(
            pRc,
            &raw mut zRet,
            b"rowid\0" as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nColumn {
            fts3Appendf(
                pRc,
                &raw mut zRet,
                b", x.'%q'\0" as *const u8 as *const ::core::ffi::c_char,
                *(*p).azColumn.offset(i as isize),
            );
            i += 1;
        }
        if !(*p).zLanguageid.is_null() {
            fts3Appendf(
                pRc,
                &raw mut zRet,
                b", x.%Q\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).zLanguageid,
            );
        }
    }
    fts3Appendf(
        pRc,
        &raw mut zRet,
        b" FROM '%q'.'%q%s' AS x\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).zDb,
        if !(*p).zContentTbl.is_null() {
            (*p).zContentTbl as *const ::core::ffi::c_char
        } else {
            (*p).zName
        },
        if !(*p).zContentTbl.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"_content\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    return zRet;
}
unsafe extern "C" fn fts3WriteExprList(
    mut p: *mut Fts3Table,
    mut zFunc: *const ::core::ffi::c_char,
    mut pRc: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zFunction: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if zFunc.is_null() {
        zFunction = b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else {
        zFunction = fts3QuoteId(zFunc);
        zFree = zFunction;
    }
    fts3Appendf(
        pRc,
        &raw mut zRet,
        b"?\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nColumn {
        fts3Appendf(
            pRc,
            &raw mut zRet,
            b",%s(?)\0" as *const u8 as *const ::core::ffi::c_char,
            zFunction,
        );
        i += 1;
    }
    if !(*p).zLanguageid.is_null() {
        fts3Appendf(
            pRc,
            &raw mut zRet,
            b", ?\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    sqlite3_free(zFree as *mut ::core::ffi::c_void);
    return zRet;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3ReadInt(
    mut z: *const ::core::ffi::c_char,
    mut pnOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut iVal: u64_0 = 0 as u64_0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while *z.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
        && *z.offset(i as isize) as ::core::ffi::c_int <= '9' as i32
    {
        iVal = iVal
            .wrapping_mul(10 as u64_0)
            .wrapping_add((*z.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as u64_0);
        if iVal > 0x7fffffff as u64_0 {
            return -(1 as ::core::ffi::c_int);
        }
        i += 1;
    }
    *pnOut = iVal as ::core::ffi::c_int;
    return i;
}
unsafe extern "C" fn fts3GobbleInt(
    mut pp: *mut *const ::core::ffi::c_char,
    mut pnOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let MAX_NPREFIX: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
    let mut nInt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nByte: ::core::ffi::c_int = 0;
    nByte = sqlite3Fts3ReadInt(*pp, &raw mut nInt);
    if nInt > MAX_NPREFIX {
        nInt = 0 as ::core::ffi::c_int;
    }
    if nByte == 0 as ::core::ffi::c_int {
        return SQLITE_ERROR;
    }
    *pnOut = nInt;
    *pp = (*pp).offset(nByte as isize);
    return SQLITE_OK;
}
unsafe extern "C" fn fts3PrefixParameter(
    mut zParam: *const ::core::ffi::c_char,
    mut pnIndex: *mut ::core::ffi::c_int,
    mut apIndex: *mut *mut Fts3Index,
) -> ::core::ffi::c_int {
    let mut aIndex: *mut Fts3Index = ::core::ptr::null_mut::<Fts3Index>();
    let mut nIndex: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if !zParam.is_null()
        && *zParam.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        nIndex += 1;
        p = zParam;
        while *p != 0 {
            if *p as ::core::ffi::c_int == ',' as i32 {
                nIndex += 1;
            }
            p = p.offset(1);
        }
    }
    aIndex = sqlite3_malloc64(
        (::core::mem::size_of::<Fts3Index>() as usize).wrapping_mul(nIndex as usize)
            as sqlite3_uint64,
    ) as *mut Fts3Index;
    *apIndex = aIndex;
    if aIndex.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        aIndex as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<Fts3Index>() as size_t).wrapping_mul(nIndex as size_t),
    );
    if !zParam.is_null() {
        let mut p_0: *const ::core::ffi::c_char = zParam;
        let mut i: ::core::ffi::c_int = 0;
        i = 1 as ::core::ffi::c_int;
        while i < nIndex {
            let mut nPrefix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if fts3GobbleInt(&raw mut p_0, &raw mut nPrefix) != 0 {
                return SQLITE_ERROR;
            }
            if nPrefix == 0 as ::core::ffi::c_int {
                nIndex -= 1;
                i -= 1;
            } else {
                (*aIndex.offset(i as isize)).nPrefix = nPrefix;
            }
            p_0 = p_0.offset(1);
            i += 1;
        }
    }
    *pnIndex = nIndex;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3ContentColumns(
    mut db: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
    mut zTbl: *const ::core::ffi::c_char,
    mut pazCol: *mut *mut *const ::core::ffi::c_char,
    mut pnCol: *mut ::core::ffi::c_int,
    mut pnStr: *mut ::core::ffi::c_int,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    zSql = sqlite3_mprintf(
        b"SELECT * FROM %Q.%Q\0" as *const u8 as *const ::core::ffi::c_char,
        zDb,
        zTbl,
    );
    if zSql.is_null() {
        rc = SQLITE_NOMEM;
    } else {
        rc = sqlite3_prepare(
            db,
            zSql,
            -(1 as ::core::ffi::c_int),
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            sqlite3Fts3ErrMsg(
                pzErr,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
        }
    }
    sqlite3_free(zSql as *mut ::core::ffi::c_void);
    if rc == SQLITE_OK {
        let mut azCol: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        let mut nStr: sqlite3_int64 = 0 as sqlite3_int64;
        let mut nCol: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        nCol = sqlite3_column_count(pStmt);
        i = 0 as ::core::ffi::c_int;
        while i < nCol {
            let mut zCol: *const ::core::ffi::c_char = sqlite3_column_name(pStmt, i);
            nStr = (nStr as ::core::ffi::c_ulonglong)
                .wrapping_add(strlen(zCol).wrapping_add(1 as size_t) as ::core::ffi::c_ulonglong)
                as sqlite3_int64 as sqlite3_int64;
            i += 1;
        }
        azCol = sqlite3_malloc64(
            ((::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                .wrapping_mul(nCol as usize) as sqlite3_uint64)
                .wrapping_add(nStr as sqlite3_uint64),
        ) as *mut *const ::core::ffi::c_char;
        if azCol.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            let mut p: *mut ::core::ffi::c_char = azCol.offset(nCol as isize)
                as *mut *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            i = 0 as ::core::ffi::c_int;
            while i < nCol {
                let mut zCol_0: *const ::core::ffi::c_char = sqlite3_column_name(pStmt, i);
                let mut n: ::core::ffi::c_int =
                    strlen(zCol_0) as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                memcpy(
                    p as *mut ::core::ffi::c_void,
                    zCol_0 as *const ::core::ffi::c_void,
                    n as size_t,
                );
                let ref mut fresh55 = *azCol.offset(i as isize);
                *fresh55 = p;
                p = p.offset(n as isize);
                i += 1;
            }
        }
        sqlite3_finalize(pStmt);
        *pnCol = nCol;
        *pnStr = nStr as ::core::ffi::c_int;
        *pazCol = azCol;
    }
    return rc;
}
unsafe extern "C" fn fts3InitVtab(
    mut isCreate: ::core::ffi::c_int,
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVTab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pHash: *mut Fts3Hash = &raw mut (*(pAux as *mut Fts3HashWrapper)).hash;
    let mut p: *mut Fts3Table = ::core::ptr::null_mut::<Fts3Table>();
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut i: ::core::ffi::c_int = 0;
    let mut nByte: sqlite3_int64 = 0;
    let mut iCol: ::core::ffi::c_int = 0;
    let mut nString: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut zCsr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nDb: ::core::ffi::c_int = 0;
    let mut nName: ::core::ffi::c_int = 0;
    let mut isFts4: ::core::ffi::c_int = (*(*argv.offset(0 as ::core::ffi::c_int as isize))
        .offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '4' as i32) as ::core::ffi::c_int;
    let mut aCol: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut pTokenizer: *mut sqlite3_tokenizer = ::core::ptr::null_mut::<sqlite3_tokenizer>();
    let mut nIndex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut aIndex: *mut Fts3Index = ::core::ptr::null_mut::<Fts3Index>();
    let mut bNoDocsize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bDescIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut zPrefix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zCompress: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zUncompress: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zContent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zLanguageid: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut azNotindexed: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut nNotindexed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    nDb = strlen(*argv.offset(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int;
    nName = strlen(*argv.offset(2 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int;
    nByte = (::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        .wrapping_mul((argc - 2 as ::core::ffi::c_int) as usize) as sqlite3_int64;
    aCol = sqlite3_malloc64(nByte as sqlite3_uint64) as *mut *const ::core::ffi::c_char;
    if !aCol.is_null() {
        memset(
            aCol as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nByte as size_t,
        );
        azNotindexed = sqlite3_malloc64(nByte as sqlite3_uint64) as *mut *mut ::core::ffi::c_char;
    }
    if !azNotindexed.is_null() {
        memset(
            azNotindexed as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nByte as size_t,
        );
    }
    if aCol.is_null() || azNotindexed.is_null() {
        rc = SQLITE_NOMEM;
    } else {
        i = 3 as ::core::ffi::c_int;
        while rc == SQLITE_OK && i < argc {
            let mut z: *const ::core::ffi::c_char = *argv.offset(i as isize);
            let mut nKey: ::core::ffi::c_int = 0;
            let mut zVal: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if pTokenizer.is_null()
                && strlen(z) > 8 as size_t
                && 0 as ::core::ffi::c_int
                    == sqlite3_strnicmp(
                        z,
                        b"tokenize\0" as *const u8 as *const ::core::ffi::c_char,
                        8 as ::core::ffi::c_int,
                    )
                && 0 as ::core::ffi::c_int
                    == sqlite3Fts3IsIdChar(*z.offset(8 as ::core::ffi::c_int as isize))
            {
                rc = sqlite3Fts3InitTokenizer(
                    pHash,
                    z.offset(9 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
                    &raw mut pTokenizer,
                    pzErr,
                );
            } else if isFts4 != 0 && fts3IsSpecialColumn(z, &raw mut nKey, &raw mut zVal) != 0 {
                let mut aFts4Opt: [Fts4Option; 8] = [
                    Fts4Option {
                        zOpt: b"matchinfo\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 9 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"prefix\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 6 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"compress\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 8 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"uncompress\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 10 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"order\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 5 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"content\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 7 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"languageid\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 10 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"notindexed\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 10 as ::core::ffi::c_int,
                    },
                ];
                let mut iOpt: ::core::ffi::c_int = 0;
                if zVal.is_null() {
                    rc = SQLITE_NOMEM;
                } else {
                    iOpt = 0 as ::core::ffi::c_int;
                    while iOpt
                        < (::core::mem::size_of::<[Fts4Option; 8]>() as usize)
                            .wrapping_div(::core::mem::size_of::<Fts4Option>() as usize)
                            as ::core::ffi::c_int
                    {
                        let mut pOp: *mut Fts4Option = (&raw mut aFts4Opt as *mut Fts4Option)
                            .offset(iOpt as isize)
                            as *mut Fts4Option;
                        if nKey == (*pOp).nOpt && sqlite3_strnicmp(z, (*pOp).zOpt, (*pOp).nOpt) == 0
                        {
                            break;
                        }
                        iOpt += 1;
                    }
                    match iOpt {
                        0 => {
                            if strlen(zVal) != 4 as size_t
                                || sqlite3_strnicmp(
                                    zVal,
                                    b"fts3\0" as *const u8 as *const ::core::ffi::c_char,
                                    4 as ::core::ffi::c_int,
                                ) != 0
                            {
                                sqlite3Fts3ErrMsg(
                                    pzErr,
                                    b"unrecognized matchinfo: %s\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    zVal,
                                );
                                rc = SQLITE_ERROR;
                            }
                            bNoDocsize = 1 as ::core::ffi::c_int;
                        }
                        1 => {
                            sqlite3_free(zPrefix as *mut ::core::ffi::c_void);
                            zPrefix = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        2 => {
                            sqlite3_free(zCompress as *mut ::core::ffi::c_void);
                            zCompress = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        3 => {
                            sqlite3_free(zUncompress as *mut ::core::ffi::c_void);
                            zUncompress = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        4 => {
                            if (strlen(zVal) != 3 as size_t
                                || sqlite3_strnicmp(
                                    zVal,
                                    b"asc\0" as *const u8 as *const ::core::ffi::c_char,
                                    3 as ::core::ffi::c_int,
                                ) != 0)
                                && (strlen(zVal) != 4 as size_t
                                    || sqlite3_strnicmp(
                                        zVal,
                                        b"desc\0" as *const u8 as *const ::core::ffi::c_char,
                                        4 as ::core::ffi::c_int,
                                    ) != 0)
                            {
                                sqlite3Fts3ErrMsg(
                                    pzErr,
                                    b"unrecognized order: %s\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    zVal,
                                );
                                rc = SQLITE_ERROR;
                            }
                            bDescIdx = (*zVal.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 'd' as i32
                                || *zVal.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 'D' as i32)
                                as ::core::ffi::c_int;
                        }
                        5 => {
                            sqlite3_free(zContent as *mut ::core::ffi::c_void);
                            zContent = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        6 => {
                            sqlite3_free(zLanguageid as *mut ::core::ffi::c_void);
                            zLanguageid = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        7 => {
                            let fresh42 = nNotindexed;
                            nNotindexed = nNotindexed + 1;
                            let ref mut fresh43 = *azNotindexed.offset(fresh42 as isize);
                            *fresh43 = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        _ => {
                            sqlite3Fts3ErrMsg(
                                pzErr,
                                b"unrecognized parameter: %s\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                z,
                            );
                            rc = SQLITE_ERROR;
                        }
                    }
                    sqlite3_free(zVal as *mut ::core::ffi::c_void);
                }
            } else {
                nString += strlen(z).wrapping_add(1 as size_t) as ::core::ffi::c_int;
                let fresh44 = nCol;
                nCol = nCol + 1;
                let ref mut fresh45 = *aCol.offset(fresh44 as isize);
                *fresh45 = z;
            }
            i += 1;
        }
        if rc == SQLITE_OK && !zContent.is_null() {
            sqlite3_free(zCompress as *mut ::core::ffi::c_void);
            sqlite3_free(zUncompress as *mut ::core::ffi::c_void);
            zCompress = ::core::ptr::null_mut::<::core::ffi::c_char>();
            zUncompress = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if nCol == 0 as ::core::ffi::c_int {
                sqlite3_free(aCol as *mut ::core::ffi::c_void);
                aCol = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
                rc = fts3ContentColumns(
                    db,
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                    zContent,
                    &raw mut aCol,
                    &raw mut nCol,
                    &raw mut nString,
                    pzErr,
                );
                if rc == SQLITE_OK && !zLanguageid.is_null() {
                    let mut j: ::core::ffi::c_int = 0;
                    j = 0 as ::core::ffi::c_int;
                    while j < nCol {
                        if sqlite3_stricmp(zLanguageid, *aCol.offset(j as isize))
                            == 0 as ::core::ffi::c_int
                        {
                            let mut k: ::core::ffi::c_int = 0;
                            k = j;
                            while k < nCol {
                                let ref mut fresh46 = *aCol.offset(k as isize);
                                *fresh46 = *aCol.offset((k + 1 as ::core::ffi::c_int) as isize);
                                k += 1;
                            }
                            nCol -= 1;
                            break;
                        } else {
                            j += 1;
                        }
                    }
                }
            }
        }
        if !(rc != SQLITE_OK) {
            if nCol == 0 as ::core::ffi::c_int {
                let ref mut fresh47 = *aCol.offset(0 as ::core::ffi::c_int as isize);
                *fresh47 = b"content\0" as *const u8 as *const ::core::ffi::c_char;
                nString = 8 as ::core::ffi::c_int;
                nCol = 1 as ::core::ffi::c_int;
            }
            if pTokenizer.is_null() {
                rc = sqlite3Fts3InitTokenizer(
                    pHash,
                    b"simple\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut pTokenizer,
                    pzErr,
                );
                if rc != SQLITE_OK {
                    current_block = 4597764963221154413;
                } else {
                    current_block = 18425699056680496821;
                }
            } else {
                current_block = 18425699056680496821;
            }
            match current_block {
                4597764963221154413 => {}
                _ => {
                    rc = fts3PrefixParameter(zPrefix, &raw mut nIndex, &raw mut aIndex);
                    if rc == SQLITE_ERROR {
                        sqlite3Fts3ErrMsg(
                            pzErr,
                            b"error parsing prefix parameter: %s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            zPrefix,
                        );
                    }
                    if !(rc != SQLITE_OK) {
                        nByte = (::core::mem::size_of::<Fts3Table>() as usize)
                            .wrapping_add((nCol as usize).wrapping_mul(::core::mem::size_of::<
                                *mut ::core::ffi::c_char,
                            >(
                            )
                                as usize))
                            .wrapping_add((nIndex as usize).wrapping_mul(::core::mem::size_of::<
                                Fts3Index,
                            >(
                            )
                                as usize))
                            .wrapping_add(
                                (nCol as usize)
                                    .wrapping_mul(::core::mem::size_of::<u8_0>() as usize),
                            )
                            .wrapping_add(nName as usize)
                            .wrapping_add(nDb as usize)
                            .wrapping_add(nString as usize)
                            as sqlite3_int64;
                        p = sqlite3_malloc64(nByte as sqlite3_uint64) as *mut Fts3Table;
                        if p.is_null() {
                            rc = SQLITE_NOMEM;
                        } else {
                            memset(
                                p as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                nByte as size_t,
                            );
                            (*p).db = db;
                            (*p).nColumn = nCol;
                            (*p).nPendingData = 0 as ::core::ffi::c_int;
                            (*p).azColumn = p.offset(1 as ::core::ffi::c_int as isize)
                                as *mut Fts3Table
                                as *mut *mut ::core::ffi::c_char;
                            (*p).pTokenizer = pTokenizer;
                            (*p).nMaxPendingData = FTS3_MAX_PENDING_DATA;
                            (*p).bHasDocsize =
                                (isFts4 != 0 && bNoDocsize == 0 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int as u8_0;
                            (*p).bHasStat = isFts4 as u8_0;
                            (*p).bFts4 = isFts4 as u8_0;
                            (*p).bDescIdx = bDescIdx as u8_0;
                            (*p).nAutoincrmerge = 0xff as ::core::ffi::c_int;
                            (*p).zContentTbl = zContent;
                            (*p).zLanguageid = zLanguageid;
                            zContent = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            zLanguageid = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            (*p).aIndex = (*p).azColumn.offset(nCol as isize)
                                as *mut *mut ::core::ffi::c_char
                                as *mut Fts3Index
                                as *mut Fts3Index;
                            memcpy(
                                (*p).aIndex as *mut ::core::ffi::c_void,
                                aIndex as *const ::core::ffi::c_void,
                                (::core::mem::size_of::<Fts3Index>() as size_t)
                                    .wrapping_mul(nIndex as size_t),
                            );
                            (*p).nIndex = nIndex;
                            i = 0 as ::core::ffi::c_int;
                            while i < nIndex {
                                sqlite3Fts3HashInit(
                                    &raw mut (*(*p).aIndex.offset(i as isize)).hPending,
                                    FTS3_HASH_STRING as ::core::ffi::c_char,
                                    1 as ::core::ffi::c_char,
                                );
                                i += 1;
                            }
                            (*p).abNotindexed =
                                (*p).aIndex.offset(nIndex as isize) as *mut Fts3Index as *mut u8_0;
                            zCsr = (*p).abNotindexed.offset(nCol as isize) as *mut u8_0
                                as *mut ::core::ffi::c_char;
                            (*p).zName = zCsr;
                            memcpy(
                                zCsr as *mut ::core::ffi::c_void,
                                *argv.offset(2 as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                nName as size_t,
                            );
                            zCsr = zCsr.offset(nName as isize);
                            (*p).zDb = zCsr;
                            memcpy(
                                zCsr as *mut ::core::ffi::c_void,
                                *argv.offset(1 as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                nDb as size_t,
                            );
                            zCsr = zCsr.offset(nDb as isize);
                            iCol = 0 as ::core::ffi::c_int;
                            while iCol < nCol {
                                let mut z_0: *mut ::core::ffi::c_char =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                z_0 = sqlite3Fts3NextToken(*aCol.offset(iCol as isize), &raw mut n)
                                    as *mut ::core::ffi::c_char;
                                if n > 0 as ::core::ffi::c_int {
                                    memcpy(
                                        zCsr as *mut ::core::ffi::c_void,
                                        z_0 as *const ::core::ffi::c_void,
                                        n as size_t,
                                    );
                                }
                                *zCsr.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
                                sqlite3Fts3Dequote(zCsr);
                                let ref mut fresh48 = *(*p).azColumn.offset(iCol as isize);
                                *fresh48 = zCsr;
                                zCsr = zCsr.offset((n + 1 as ::core::ffi::c_int) as isize);
                                iCol += 1;
                            }
                            iCol = 0 as ::core::ffi::c_int;
                            while iCol < nCol {
                                let mut n_0: ::core::ffi::c_int =
                                    strlen(*(*p).azColumn.offset(iCol as isize))
                                        as ::core::ffi::c_int;
                                i = 0 as ::core::ffi::c_int;
                                while i < nNotindexed {
                                    let mut zNot: *mut ::core::ffi::c_char =
                                        *azNotindexed.offset(i as isize);
                                    if !zNot.is_null()
                                        && n_0 == strlen(zNot) as ::core::ffi::c_int
                                        && 0 as ::core::ffi::c_int
                                            == sqlite3_strnicmp(
                                                *(*p).azColumn.offset(iCol as isize),
                                                zNot,
                                                n_0,
                                            )
                                    {
                                        *(*p).abNotindexed.offset(iCol as isize) = 1 as u8_0;
                                        sqlite3_free(zNot as *mut ::core::ffi::c_void);
                                        let ref mut fresh49 = *azNotindexed.offset(i as isize);
                                        *fresh49 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                    }
                                    i += 1;
                                }
                                iCol += 1;
                            }
                            i = 0 as ::core::ffi::c_int;
                            while i < nNotindexed {
                                if !(*azNotindexed.offset(i as isize)).is_null() {
                                    sqlite3Fts3ErrMsg(
                                        pzErr,
                                        b"no such column: %s\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        *azNotindexed.offset(i as isize),
                                    );
                                    rc = SQLITE_ERROR;
                                }
                                i += 1;
                            }
                            if rc == SQLITE_OK
                                && (zCompress == ::core::ptr::null_mut::<::core::ffi::c_char>())
                                    as ::core::ffi::c_int
                                    != (zUncompress
                                        == ::core::ptr::null_mut::<::core::ffi::c_char>())
                                        as ::core::ffi::c_int
                            {
                                let mut zMiss: *const ::core::ffi::c_char = if zCompress.is_null() {
                                    b"compress\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"uncompress\0" as *const u8 as *const ::core::ffi::c_char
                                };
                                rc = SQLITE_ERROR;
                                sqlite3Fts3ErrMsg(
                                    pzErr,
                                    b"missing %s parameter in fts4 constructor\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    zMiss,
                                );
                            }
                            (*p).zReadExprlist = fts3ReadExprList(p, zUncompress, &raw mut rc);
                            (*p).zWriteExprlist = fts3WriteExprList(p, zCompress, &raw mut rc);
                            if !(rc != SQLITE_OK) {
                                if isCreate != 0 {
                                    rc = fts3CreateTables(p);
                                }
                                if isFts4 == 0 && isCreate == 0 {
                                    (*p).bHasStat = 2 as u8_0;
                                }
                                fts3DatabasePageSize(&raw mut rc, p);
                                (*p).nNodeSize = (*p).nPgsz - 35 as ::core::ffi::c_int;
                                (*p).nMergeCount = FTS3_MERGE_COUNT;
                                fts3DeclareVtab(&raw mut rc, p);
                            }
                        }
                    }
                }
            }
        }
    }
    sqlite3_free(zPrefix as *mut ::core::ffi::c_void);
    sqlite3_free(aIndex as *mut ::core::ffi::c_void);
    sqlite3_free(zCompress as *mut ::core::ffi::c_void);
    sqlite3_free(zUncompress as *mut ::core::ffi::c_void);
    sqlite3_free(zContent as *mut ::core::ffi::c_void);
    sqlite3_free(zLanguageid as *mut ::core::ffi::c_void);
    i = 0 as ::core::ffi::c_int;
    while i < nNotindexed {
        sqlite3_free(*azNotindexed.offset(i as isize) as *mut ::core::ffi::c_void);
        i += 1;
    }
    sqlite3_free(aCol as *mut ::core::ffi::c_void);
    sqlite3_free(azNotindexed as *mut ::core::ffi::c_void);
    if rc != SQLITE_OK {
        if !p.is_null() {
            fts3DisconnectMethod(p as *mut sqlite3_vtab);
        } else if !pTokenizer.is_null() {
            (*(*pTokenizer).pModule)
                .xDestroy
                .expect("non-null function pointer")(pTokenizer);
        }
    } else {
        *ppVTab = &raw mut (*p).base;
    }
    return rc;
}
unsafe extern "C" fn fts3ConnectMethod(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return fts3InitVtab(0 as ::core::ffi::c_int, db, pAux, argc, argv, ppVtab, pzErr);
}
unsafe extern "C" fn fts3CreateMethod(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return fts3InitVtab(1 as ::core::ffi::c_int, db, pAux, argc, argv, ppVtab, pzErr);
}
unsafe extern "C" fn fts3SetEstimatedRows(mut pIdxInfo: *mut sqlite3_index_info, mut nRow: i64_0) {
    if sqlite3_libversion_number() >= 3008002 as ::core::ffi::c_int {
        (*pIdxInfo).estimatedRows = nRow as sqlite3_int64;
    }
}
unsafe extern "C" fn fts3SetUniqueFlag(mut pIdxInfo: *mut sqlite3_index_info) {
    if sqlite3_libversion_number() >= 3008012 as ::core::ffi::c_int {
        (*pIdxInfo).idxFlags |= SQLITE_INDEX_SCAN_UNIQUE;
    }
}
unsafe extern "C" fn fts3BestIndexMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut pInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    let mut p: *mut Fts3Table = pVTab as *mut Fts3Table;
    let mut i: ::core::ffi::c_int = 0;
    let mut iCons: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iLangidCons: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iDocidGe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iDocidLe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iIdx: ::core::ffi::c_int = 0;
    if (*p).bLock != 0 {
        return SQLITE_ERROR;
    }
    (*pInfo).idxNum = FTS3_FULLSCAN_SEARCH;
    (*pInfo).estimatedCost = 5000000 as ::core::ffi::c_int as ::core::ffi::c_double;
    i = 0 as ::core::ffi::c_int;
    while i < (*pInfo).nConstraint {
        let mut bDocid: ::core::ffi::c_int = 0;
        let mut pCons: *mut sqlite3_index_constraint =
            (*pInfo).aConstraint.offset(i as isize) as *mut sqlite3_index_constraint;
        if (*pCons).usable as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if (*pCons).op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_MATCH {
                (*pInfo).idxNum = FTS3_FULLSCAN_SEARCH;
                (*pInfo).estimatedCost = 1e50f64;
                fts3SetEstimatedRows(
                    pInfo,
                    (1 as ::core::ffi::c_int as i64_0) << 50 as ::core::ffi::c_int,
                );
                return SQLITE_OK;
            }
        } else {
            bDocid = ((*pCons).iColumn < 0 as ::core::ffi::c_int
                || (*pCons).iColumn == (*p).nColumn + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if iCons < 0 as ::core::ffi::c_int
                && (*pCons).op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_EQ
                && bDocid != 0
            {
                (*pInfo).idxNum = FTS3_DOCID_SEARCH;
                (*pInfo).estimatedCost = 1.0f64;
                iCons = i;
            }
            if (*pCons).op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_MATCH
                && (*pCons).iColumn >= 0 as ::core::ffi::c_int
                && (*pCons).iColumn <= (*p).nColumn
            {
                (*pInfo).idxNum = FTS3_FULLTEXT_SEARCH + (*pCons).iColumn;
                (*pInfo).estimatedCost = 2.0f64;
                iCons = i;
            }
            if (*pCons).op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_EQ
                && (*pCons).iColumn == (*p).nColumn + 2 as ::core::ffi::c_int
            {
                iLangidCons = i;
            }
            if bDocid != 0 {
                match (*pCons).op as ::core::ffi::c_int {
                    SQLITE_INDEX_CONSTRAINT_GE | SQLITE_INDEX_CONSTRAINT_GT => {
                        iDocidGe = i;
                    }
                    SQLITE_INDEX_CONSTRAINT_LE | SQLITE_INDEX_CONSTRAINT_LT => {
                        iDocidLe = i;
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    if (*pInfo).idxNum == FTS3_DOCID_SEARCH {
        fts3SetUniqueFlag(pInfo);
    }
    iIdx = 1 as ::core::ffi::c_int;
    if iCons >= 0 as ::core::ffi::c_int {
        let fresh38 = iIdx;
        iIdx = iIdx + 1;
        (*(*pInfo).aConstraintUsage.offset(iCons as isize)).argvIndex = fresh38;
        (*(*pInfo).aConstraintUsage.offset(iCons as isize)).omit = 1 as ::core::ffi::c_uchar;
    }
    if iLangidCons >= 0 as ::core::ffi::c_int {
        (*pInfo).idxNum |= FTS3_HAVE_LANGID;
        let fresh39 = iIdx;
        iIdx = iIdx + 1;
        (*(*pInfo).aConstraintUsage.offset(iLangidCons as isize)).argvIndex = fresh39;
    }
    if iDocidGe >= 0 as ::core::ffi::c_int {
        (*pInfo).idxNum |= FTS3_HAVE_DOCID_GE;
        let fresh40 = iIdx;
        iIdx = iIdx + 1;
        (*(*pInfo).aConstraintUsage.offset(iDocidGe as isize)).argvIndex = fresh40;
    }
    if iDocidLe >= 0 as ::core::ffi::c_int {
        (*pInfo).idxNum |= FTS3_HAVE_DOCID_LE;
        let fresh41 = iIdx;
        iIdx = iIdx + 1;
        (*(*pInfo).aConstraintUsage.offset(iDocidLe as isize)).argvIndex = fresh41;
    }
    if (*pInfo).nOrderBy == 1 as ::core::ffi::c_int {
        let mut pOrder: *mut sqlite3_index_orderby =
            (*pInfo).aOrderBy.offset(0 as ::core::ffi::c_int as isize)
                as *mut sqlite3_index_orderby;
        if (*pOrder).iColumn < 0 as ::core::ffi::c_int
            || (*pOrder).iColumn == (*p).nColumn + 1 as ::core::ffi::c_int
        {
            if (*pOrder).desc != 0 {
                (*pInfo).idxStr = b"DESC\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else {
                (*pInfo).idxStr =
                    b"ASC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            (*pInfo).orderByConsumed = 1 as ::core::ffi::c_int;
        }
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3OpenMethod(
    mut _pVTab: *mut sqlite3_vtab,
    mut ppCsr: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut sqlite3_vtab_cursor = ::core::ptr::null_mut::<sqlite3_vtab_cursor>();
    pCsr = sqlite3_malloc(::core::mem::size_of::<Fts3Cursor>() as ::core::ffi::c_int)
        as *mut sqlite3_vtab_cursor;
    *ppCsr = pCsr;
    if pCsr.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3Cursor>() as size_t,
    );
    return SQLITE_OK;
}
unsafe extern "C" fn fts3CursorFinalizeStmt(mut pCsr: *mut Fts3Cursor) {
    if (*pCsr).bSeekStmt != 0 {
        let mut p: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
        if (*p).pSeekStmt.is_null() {
            (*p).pSeekStmt = (*pCsr).pStmt;
            sqlite3_reset((*pCsr).pStmt);
            (*pCsr).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
        (*pCsr).bSeekStmt = 0 as u8_0;
    }
    sqlite3_finalize((*pCsr).pStmt);
}
unsafe extern "C" fn fts3ClearCursor(mut pCsr: *mut Fts3Cursor) {
    fts3CursorFinalizeStmt(pCsr);
    sqlite3Fts3FreeDeferredTokens(pCsr);
    sqlite3_free((*pCsr).aDoclist as *mut ::core::ffi::c_void);
    sqlite3Fts3MIBufferFree((*pCsr).pMIBuffer);
    sqlite3Fts3ExprFree((*pCsr).pExpr);
    memset(
        (&raw mut (*pCsr).base).offset(1 as ::core::ffi::c_int as isize) as *mut sqlite3_vtab_cursor
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<Fts3Cursor>() as size_t)
            .wrapping_sub(::core::mem::size_of::<sqlite3_vtab_cursor>() as size_t),
    );
}
unsafe extern "C" fn fts3CloseMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    fts3ClearCursor(pCsr);
    sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn fts3CursorSeekStmt(mut pCsr: *mut Fts3Cursor) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*pCsr).pStmt.is_null() {
        let mut p: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*p).pSeekStmt.is_null() {
            (*pCsr).pStmt = (*p).pSeekStmt;
            (*p).pSeekStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        } else {
            zSql = sqlite3_mprintf(
                b"SELECT %s WHERE rowid = ?\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).zReadExprlist,
            );
            if zSql.is_null() {
                return SQLITE_NOMEM;
            }
            (*p).bLock += 1;
            rc = sqlite3_prepare_v3(
                (*p).db,
                zSql,
                -(1 as ::core::ffi::c_int),
                SQLITE_PREPARE_PERSISTENT as ::core::ffi::c_uint,
                &raw mut (*pCsr).pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            (*p).bLock -= 1;
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
        }
        if rc == SQLITE_OK {
            (*pCsr).bSeekStmt = 1 as u8_0;
        }
    }
    return rc;
}
unsafe extern "C" fn fts3CursorSeek(
    mut pContext: *mut sqlite3_context,
    mut pCsr: *mut Fts3Cursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*pCsr).isRequireSeek != 0 {
        rc = fts3CursorSeekStmt(pCsr);
        if rc == SQLITE_OK {
            let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
            (*pTab).bLock += 1;
            sqlite3_bind_int64((*pCsr).pStmt, 1 as ::core::ffi::c_int, (*pCsr).iPrevId);
            (*pCsr).isRequireSeek = 0 as u8_0;
            if SQLITE_ROW == sqlite3_step((*pCsr).pStmt) {
                (*pTab).bLock -= 1;
                return SQLITE_OK;
            } else {
                (*pTab).bLock -= 1;
                rc = sqlite3_reset((*pCsr).pStmt);
                if rc == SQLITE_OK
                    && (*((*pCsr).base.pVtab as *mut Fts3Table))
                        .zContentTbl
                        .is_null()
                {
                    rc = FTS_CORRUPT_VTAB;
                    (*pCsr).isEof = 1 as u8_0;
                }
            }
        }
    }
    if rc != SQLITE_OK && !pContext.is_null() {
        sqlite3_result_error_code(pContext, rc);
    }
    return rc;
}
unsafe extern "C" fn fts3ScanInteriorNode(
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut zNode: *const ::core::ffi::c_char,
    mut nNode: ::core::ffi::c_int,
    mut piFirst: *mut sqlite3_int64,
    mut piLast: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut zCsr: *const ::core::ffi::c_char = zNode;
    let mut zEnd: *const ::core::ffi::c_char =
        zCsr.offset(nNode as isize) as *const ::core::ffi::c_char;
    let mut zBuffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nAlloc: i64_0 = 0 as i64_0;
    let mut isFirstTerm: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut iChild: u64_0 = 0;
    let mut nBuffer: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    zCsr = zCsr.offset(sqlite3Fts3GetVarintU(zCsr, &raw mut iChild) as isize);
    zCsr = zCsr.offset(sqlite3Fts3GetVarintU(zCsr, &raw mut iChild) as isize);
    if zCsr > zEnd {
        return FTS_CORRUPT_VTAB;
    }
    loop {
        if !(zCsr < zEnd && (!piFirst.is_null() || !piLast.is_null())) {
            current_block = 18377268871191777778;
            break;
        }
        let mut cmp: ::core::ffi::c_int = 0;
        let mut nSuffix: ::core::ffi::c_int = 0;
        let mut nPrefix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if isFirstTerm == 0 {
            zCsr = zCsr.offset(
                (if *(zCsr as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    sqlite3Fts3GetVarint32(zCsr, &raw mut nPrefix)
                } else {
                    nPrefix = *(zCsr as *mut u8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
            if nPrefix > nBuffer {
                rc = FTS_CORRUPT_VTAB;
                current_block = 1916983269954443888;
                break;
            }
        }
        isFirstTerm = 0 as ::core::ffi::c_int;
        zCsr = zCsr.offset(
            (if *(zCsr as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                sqlite3Fts3GetVarint32(zCsr, &raw mut nSuffix)
            } else {
                nSuffix = *(zCsr as *mut u8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
        if nPrefix as ::core::ffi::c_long > zCsr.offset_from(zNode) as ::core::ffi::c_long
            || nSuffix as ::core::ffi::c_long > zEnd.offset_from(zCsr) as ::core::ffi::c_long
            || nSuffix == 0 as ::core::ffi::c_int
        {
            rc = FTS_CORRUPT_VTAB;
            current_block = 1916983269954443888;
            break;
        } else {
            if nPrefix as i64_0 + nSuffix as i64_0 > nAlloc {
                let mut zNew: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                nAlloc = (nPrefix as i64_0 + nSuffix as i64_0) * 2 as i64_0;
                zNew = sqlite3_realloc64(
                    zBuffer as *mut ::core::ffi::c_void,
                    nAlloc as sqlite3_uint64,
                ) as *mut ::core::ffi::c_char;
                if zNew.is_null() {
                    rc = SQLITE_NOMEM;
                    current_block = 1916983269954443888;
                    break;
                } else {
                    zBuffer = zNew;
                }
            }
            memcpy(
                zBuffer.offset(nPrefix as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                zCsr as *const ::core::ffi::c_void,
                nSuffix as size_t,
            );
            nBuffer = nPrefix + nSuffix;
            zCsr = zCsr.offset(nSuffix as isize);
            cmp = memcmp(
                zTerm as *const ::core::ffi::c_void,
                zBuffer as *const ::core::ffi::c_void,
                (if nBuffer > nTerm { nTerm } else { nBuffer }) as size_t,
            );
            if !piFirst.is_null()
                && (cmp < 0 as ::core::ffi::c_int
                    || cmp == 0 as ::core::ffi::c_int && nBuffer > nTerm)
            {
                *piFirst = iChild as i64_0 as sqlite3_int64;
                piFirst = ::core::ptr::null_mut::<sqlite3_int64>();
            }
            if !piLast.is_null() && cmp < 0 as ::core::ffi::c_int {
                *piLast = iChild as i64_0 as sqlite3_int64;
                piLast = ::core::ptr::null_mut::<sqlite3_int64>();
            }
            iChild = iChild.wrapping_add(1);
        }
    }
    match current_block {
        18377268871191777778 => {
            if !piFirst.is_null() {
                *piFirst = iChild as i64_0 as sqlite3_int64;
            }
            if !piLast.is_null() {
                *piLast = iChild as i64_0 as sqlite3_int64;
            }
        }
        _ => {}
    }
    sqlite3_free(zBuffer as *mut ::core::ffi::c_void);
    return rc;
}
unsafe extern "C" fn fts3SelectLeaf(
    mut p: *mut Fts3Table,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut zNode: *const ::core::ffi::c_char,
    mut nNode: ::core::ffi::c_int,
    mut piLeaf: *mut sqlite3_int64,
    mut piLeaf2: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut iHeight: ::core::ffi::c_int = 0;
    if *(zNode as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        sqlite3Fts3GetVarint32(zNode, &raw mut iHeight);
    } else {
        iHeight = *(zNode as *mut u8_0) as ::core::ffi::c_int;
    };
    rc = fts3ScanInteriorNode(zTerm, nTerm, zNode, nNode, piLeaf, piLeaf2);
    if rc == SQLITE_OK && iHeight > 1 as ::core::ffi::c_int {
        let mut zBlob: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nBlob: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !piLeaf.is_null() && !piLeaf2.is_null() && *piLeaf != *piLeaf2 {
            rc = sqlite3Fts3ReadBlock(
                p,
                *piLeaf,
                &raw mut zBlob,
                &raw mut nBlob,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if rc == SQLITE_OK {
                rc = fts3SelectLeaf(
                    p,
                    zTerm,
                    nTerm,
                    zBlob,
                    nBlob,
                    piLeaf,
                    ::core::ptr::null_mut::<sqlite3_int64>(),
                );
            }
            sqlite3_free(zBlob as *mut ::core::ffi::c_void);
            piLeaf = ::core::ptr::null_mut::<sqlite3_int64>();
            zBlob = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if rc == SQLITE_OK {
            rc = sqlite3Fts3ReadBlock(
                p,
                if !piLeaf.is_null() { *piLeaf } else { *piLeaf2 },
                &raw mut zBlob,
                &raw mut nBlob,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        }
        if rc == SQLITE_OK {
            let mut iNewHeight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if *(zBlob as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                sqlite3Fts3GetVarint32(zBlob, &raw mut iNewHeight);
            } else {
                iNewHeight = *(zBlob as *mut u8_0) as ::core::ffi::c_int;
            };
            if iNewHeight >= iHeight {
                rc = FTS_CORRUPT_VTAB;
            } else {
                rc = fts3SelectLeaf(p, zTerm, nTerm, zBlob, nBlob, piLeaf, piLeaf2);
            }
        }
        sqlite3_free(zBlob as *mut ::core::ffi::c_void);
    }
    return rc;
}
unsafe extern "C" fn fts3PutDeltaVarint(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut piPrev: *mut sqlite3_int64,
    mut iVal: sqlite3_int64,
) {
    *pp = (*pp).offset(sqlite3Fts3PutVarint(*pp, iVal as sqlite_int64 - *piPrev) as isize);
    *piPrev = iVal;
}
unsafe extern "C" fn fts3PoslistCopy(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut ppPoslist: *mut *mut ::core::ffi::c_char,
) {
    let mut pEnd: *mut ::core::ffi::c_char = *ppPoslist;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    while *pEnd as ::core::ffi::c_int | c as ::core::ffi::c_int != 0 {
        let fresh19 = pEnd;
        pEnd = pEnd.offset(1);
        c = (*fresh19 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
    }
    pEnd = pEnd.offset(1);
    if !pp.is_null() {
        let mut n: ::core::ffi::c_int =
            pEnd.offset_from(*ppPoslist) as ::core::ffi::c_long as ::core::ffi::c_int;
        let mut p: *mut ::core::ffi::c_char = *pp;
        memcpy(
            p as *mut ::core::ffi::c_void,
            *ppPoslist as *const ::core::ffi::c_void,
            n as size_t,
        );
        p = p.offset(n as isize);
        *pp = p;
    }
    *ppPoslist = pEnd;
}
unsafe extern "C" fn fts3ColumnlistCopy(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut ppPoslist: *mut *mut ::core::ffi::c_char,
) {
    let mut pEnd: *mut ::core::ffi::c_char = *ppPoslist;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    while 0xfe as ::core::ffi::c_int & (*pEnd as ::core::ffi::c_int | c as ::core::ffi::c_int) != 0
    {
        let fresh22 = pEnd;
        pEnd = pEnd.offset(1);
        c = (*fresh22 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
    }
    if !pp.is_null() {
        let mut n: ::core::ffi::c_int =
            pEnd.offset_from(*ppPoslist) as ::core::ffi::c_long as ::core::ffi::c_int;
        let mut p: *mut ::core::ffi::c_char = *pp;
        memcpy(
            p as *mut ::core::ffi::c_void,
            *ppPoslist as *const ::core::ffi::c_void,
            n as size_t,
        );
        p = p.offset(n as isize);
        *pp = p;
    }
    *ppPoslist = pEnd;
}
pub const POSITION_LIST_END: i64_0 = LARGEST_INT64;
unsafe extern "C" fn fts3ReadNextPos(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut pi: *mut sqlite3_int64,
) {
    if **pp as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int != 0 {
        let mut iVal: ::core::ffi::c_int = 0;
        *pp = (*pp).offset(
            (if *(*pp as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                sqlite3Fts3GetVarint32(*pp, &raw mut iVal)
            } else {
                iVal = *(*pp as *mut u8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
        *pi += iVal as sqlite3_int64;
        *pi -= 2 as sqlite3_int64;
    } else {
        *pi = POSITION_LIST_END as sqlite3_int64;
    };
}
unsafe extern "C" fn fts3PutColNumber(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if iCol != 0 {
        let mut p: *mut ::core::ffi::c_char = *pp;
        n = 1 as ::core::ffi::c_int
            + sqlite3Fts3PutVarint(
                p.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
                iCol as sqlite_int64,
            );
        *p = 0x1 as ::core::ffi::c_char;
        *pp = p.offset(n as isize) as *mut ::core::ffi::c_char;
    }
    return n;
}
unsafe extern "C" fn fts3PoslistMerge(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut pp1: *mut *mut ::core::ffi::c_char,
    mut pp2: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = *pp;
    let mut p1: *mut ::core::ffi::c_char = *pp1;
    let mut p2: *mut ::core::ffi::c_char = *pp2;
    while *p1 as ::core::ffi::c_int != 0 || *p2 as ::core::ffi::c_int != 0 {
        let mut iCol1: ::core::ffi::c_int = 0;
        let mut iCol2: ::core::ffi::c_int = 0;
        if *p1 as ::core::ffi::c_int == POS_COLUMN {
            if *(p1.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *mut u8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(
                    p1.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
                    &raw mut iCol1,
                );
            } else {
                iCol1 = *(p1.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                    as *mut u8_0) as ::core::ffi::c_int;
            };
            if iCol1 == 0 as ::core::ffi::c_int {
                return FTS_CORRUPT_VTAB;
            }
        } else if *p1 as ::core::ffi::c_int == POS_END {
            iCol1 = 0x7fffffff as ::core::ffi::c_int;
        } else {
            iCol1 = 0 as ::core::ffi::c_int;
        }
        if *p2 as ::core::ffi::c_int == POS_COLUMN {
            if *(p2.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *mut u8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(
                    p2.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
                    &raw mut iCol2,
                );
            } else {
                iCol2 = *(p2.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                    as *mut u8_0) as ::core::ffi::c_int;
            };
            if iCol2 == 0 as ::core::ffi::c_int {
                return FTS_CORRUPT_VTAB;
            }
        } else if *p2 as ::core::ffi::c_int == POS_END {
            iCol2 = 0x7fffffff as ::core::ffi::c_int;
        } else {
            iCol2 = 0 as ::core::ffi::c_int;
        }
        if iCol1 == iCol2 {
            let mut i1: sqlite3_int64 = 0 as sqlite3_int64;
            let mut i2: sqlite3_int64 = 0 as sqlite3_int64;
            let mut iPrev: sqlite3_int64 = 0 as sqlite3_int64;
            let mut n: ::core::ffi::c_int = fts3PutColNumber(&raw mut p, iCol1);
            p1 = p1.offset(n as isize);
            p2 = p2.offset(n as isize);
            fts3GetDeltaVarint(&raw mut p1, &raw mut i1);
            fts3GetDeltaVarint(&raw mut p2, &raw mut i2);
            if i1 < 2 as sqlite3_int64 || i2 < 2 as sqlite3_int64 {
                break;
            }
            loop {
                fts3PutDeltaVarint(&raw mut p, &raw mut iPrev, if i1 < i2 { i1 } else { i2 });
                iPrev -= 2 as sqlite3_int64;
                if i1 == i2 {
                    fts3ReadNextPos(&raw mut p1, &raw mut i1);
                    fts3ReadNextPos(&raw mut p2, &raw mut i2);
                } else if i1 < i2 {
                    fts3ReadNextPos(&raw mut p1, &raw mut i1);
                } else {
                    fts3ReadNextPos(&raw mut p2, &raw mut i2);
                }
                if !(i1 != POSITION_LIST_END || i2 != POSITION_LIST_END) {
                    break;
                }
            }
        } else if iCol1 < iCol2 {
            p1 = p1.offset(fts3PutColNumber(&raw mut p, iCol1) as isize);
            fts3ColumnlistCopy(&raw mut p, &raw mut p1);
        } else {
            p2 = p2.offset(fts3PutColNumber(&raw mut p, iCol2) as isize);
            fts3ColumnlistCopy(&raw mut p, &raw mut p2);
        }
    }
    let fresh23 = p;
    p = p.offset(1);
    *fresh23 = POS_END as ::core::ffi::c_char;
    *pp = p;
    *pp1 = p1.offset(1 as ::core::ffi::c_int as isize);
    *pp2 = p2.offset(1 as ::core::ffi::c_int as isize);
    return SQLITE_OK;
}
unsafe extern "C" fn fts3PoslistPhraseMerge(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut nToken: ::core::ffi::c_int,
    mut isSaveLeft: ::core::ffi::c_int,
    mut isExact: ::core::ffi::c_int,
    mut pp1: *mut *mut ::core::ffi::c_char,
    mut pp2: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = *pp;
    let mut p1: *mut ::core::ffi::c_char = *pp1;
    let mut p2: *mut ::core::ffi::c_char = *pp2;
    let mut iCol1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iCol2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *p1 as ::core::ffi::c_int == POS_COLUMN {
        p1 = p1.offset(1);
        p1 = p1.offset(
            (if *(p1 as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                sqlite3Fts3GetVarint32(p1, &raw mut iCol1)
            } else {
                iCol1 = *(p1 as *mut u8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
        if iCol1 == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *p2 as ::core::ffi::c_int == POS_COLUMN {
        p2 = p2.offset(1);
        p2 = p2.offset(
            (if *(p2 as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                sqlite3Fts3GetVarint32(p2, &raw mut iCol2)
            } else {
                iCol2 = *(p2 as *mut u8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
        if iCol2 == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    loop {
        if iCol1 == iCol2 {
            let mut pSave: *mut ::core::ffi::c_char = p;
            let mut iPrev: sqlite3_int64 = 0 as sqlite3_int64;
            let mut iPos1: sqlite3_int64 = 0 as sqlite3_int64;
            let mut iPos2: sqlite3_int64 = 0 as sqlite3_int64;
            if iCol1 != 0 {
                let fresh20 = p;
                p = p.offset(1);
                *fresh20 = POS_COLUMN as ::core::ffi::c_char;
                p = p.offset(sqlite3Fts3PutVarint(p, iCol1 as sqlite_int64) as isize);
            }
            fts3GetDeltaVarint(&raw mut p1, &raw mut iPos1);
            iPos1 -= 2 as sqlite3_int64;
            fts3GetDeltaVarint(&raw mut p2, &raw mut iPos2);
            iPos2 -= 2 as sqlite3_int64;
            if iPos1 < 0 as sqlite3_int64 || iPos2 < 0 as sqlite3_int64 {
                break;
            }
            loop {
                if iPos2 == iPos1 + nToken as sqlite3_int64
                    || isExact == 0 as ::core::ffi::c_int
                        && iPos2 > iPos1
                        && iPos2 <= iPos1 + nToken as sqlite3_int64
                {
                    let mut iSave: sqlite3_int64 = 0;
                    iSave = if isSaveLeft != 0 { iPos1 } else { iPos2 };
                    fts3PutDeltaVarint(&raw mut p, &raw mut iPrev, iSave + 2 as sqlite3_int64);
                    iPrev -= 2 as sqlite3_int64;
                    pSave = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                if isSaveLeft == 0 && iPos2 <= iPos1 + nToken as sqlite3_int64 || iPos2 <= iPos1 {
                    if *p2 as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                    fts3GetDeltaVarint(&raw mut p2, &raw mut iPos2);
                    iPos2 -= 2 as sqlite3_int64;
                } else {
                    if *p1 as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                    fts3GetDeltaVarint(&raw mut p1, &raw mut iPos1);
                    iPos1 -= 2 as sqlite3_int64;
                }
            }
            if !pSave.is_null() {
                p = pSave;
            }
            fts3ColumnlistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p1,
            );
            fts3ColumnlistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p2,
            );
            if 0 as ::core::ffi::c_int == *p1 as ::core::ffi::c_int
                || 0 as ::core::ffi::c_int == *p2 as ::core::ffi::c_int
            {
                break;
            }
            p1 = p1.offset(1);
            p1 = p1.offset(
                (if *(p1 as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    sqlite3Fts3GetVarint32(p1, &raw mut iCol1)
                } else {
                    iCol1 = *(p1 as *mut u8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
            p2 = p2.offset(1);
            p2 = p2.offset(
                (if *(p2 as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    sqlite3Fts3GetVarint32(p2, &raw mut iCol2)
                } else {
                    iCol2 = *(p2 as *mut u8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
        } else if iCol1 < iCol2 {
            fts3ColumnlistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p1,
            );
            if 0 as ::core::ffi::c_int == *p1 as ::core::ffi::c_int {
                break;
            }
            p1 = p1.offset(1);
            p1 = p1.offset(
                (if *(p1 as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    sqlite3Fts3GetVarint32(p1, &raw mut iCol1)
                } else {
                    iCol1 = *(p1 as *mut u8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
        } else {
            fts3ColumnlistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p2,
            );
            if 0 as ::core::ffi::c_int == *p2 as ::core::ffi::c_int {
                break;
            }
            p2 = p2.offset(1);
            p2 = p2.offset(
                (if *(p2 as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    sqlite3Fts3GetVarint32(p2, &raw mut iCol2)
                } else {
                    iCol2 = *(p2 as *mut u8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
        }
    }
    fts3PoslistCopy(
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut p2,
    );
    fts3PoslistCopy(
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut p1,
    );
    *pp1 = p1;
    *pp2 = p2;
    if *pp == p {
        return 0 as ::core::ffi::c_int;
    }
    let fresh21 = p;
    p = p.offset(1);
    *fresh21 = 0 as ::core::ffi::c_char;
    *pp = p;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3PoslistNearMerge(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut aTmp: *mut ::core::ffi::c_char,
    mut nRight: ::core::ffi::c_int,
    mut nLeft: ::core::ffi::c_int,
    mut pp1: *mut *mut ::core::ffi::c_char,
    mut pp2: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p1: *mut ::core::ffi::c_char = *pp1;
    let mut p2: *mut ::core::ffi::c_char = *pp2;
    let mut pTmp1: *mut ::core::ffi::c_char = aTmp;
    let mut pTmp2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aTmp2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    fts3PoslistPhraseMerge(
        &raw mut pTmp1,
        nRight,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        pp1,
        pp2,
    );
    pTmp2 = pTmp1;
    aTmp2 = pTmp2;
    *pp1 = p1;
    *pp2 = p2;
    fts3PoslistPhraseMerge(
        &raw mut pTmp2,
        nLeft,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        pp2,
        pp1,
    );
    if pTmp1 != aTmp && pTmp2 != aTmp2 {
        fts3PoslistMerge(pp, &raw mut aTmp, &raw mut aTmp2);
    } else if pTmp1 != aTmp {
        fts3PoslistCopy(pp, &raw mut aTmp);
    } else if pTmp2 != aTmp2 {
        fts3PoslistCopy(pp, &raw mut aTmp2);
    } else {
        res = 0 as ::core::ffi::c_int;
    }
    return res;
}
unsafe extern "C" fn fts3GetDeltaVarint3(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut pEnd: *mut ::core::ffi::c_char,
    mut bDescIdx: ::core::ffi::c_int,
    mut pVal: *mut sqlite3_int64,
) {
    if *pp >= pEnd {
        *pp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        let mut iVal: u64_0 = 0;
        *pp = (*pp).offset(sqlite3Fts3GetVarintU(*pp, &raw mut iVal) as isize);
        if bDescIdx != 0 {
            *pVal = (*pVal as u64_0).wrapping_sub(iVal) as i64_0 as sqlite3_int64;
        } else {
            *pVal = (*pVal as u64_0).wrapping_add(iVal) as i64_0 as sqlite3_int64;
        }
    };
}
unsafe extern "C" fn fts3PutDeltaVarint3(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut bDescIdx: ::core::ffi::c_int,
    mut piPrev: *mut sqlite3_int64,
    mut pbFirst: *mut ::core::ffi::c_int,
    mut iVal: sqlite3_int64,
) {
    let mut iWrite: sqlite3_uint64 = 0;
    if bDescIdx == 0 as ::core::ffi::c_int || *pbFirst == 0 as ::core::ffi::c_int {
        iWrite = (iVal as u64_0).wrapping_sub(*piPrev as u64_0) as sqlite3_uint64;
    } else {
        iWrite = (*piPrev as u64_0).wrapping_sub(iVal as u64_0) as sqlite3_uint64;
    }
    *pp = (*pp).offset(sqlite3Fts3PutVarint(*pp, iWrite as sqlite_int64) as isize);
    *piPrev = iVal;
    *pbFirst = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3DoclistOrMerge(
    mut bDescDoclist: ::core::ffi::c_int,
    mut a1: *mut ::core::ffi::c_char,
    mut n1: ::core::ffi::c_int,
    mut a2: *mut ::core::ffi::c_char,
    mut n2: ::core::ffi::c_int,
    mut paOut: *mut *mut ::core::ffi::c_char,
    mut pnOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut i1: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i2: sqlite3_int64 = 0 as sqlite3_int64;
    let mut iPrev: sqlite3_int64 = 0 as sqlite3_int64;
    let mut pEnd1: *mut ::core::ffi::c_char = a1.offset(n1 as isize) as *mut ::core::ffi::c_char;
    let mut pEnd2: *mut ::core::ffi::c_char = a2.offset(n2 as isize) as *mut ::core::ffi::c_char;
    let mut p1: *mut ::core::ffi::c_char = a1;
    let mut p2: *mut ::core::ffi::c_char = a2;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bFirstOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *paOut = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *pnOut = 0 as ::core::ffi::c_int;
    aOut = sqlite3_malloc64(
        (n1 as i64_0 + n2 as i64_0 + FTS3_VARINT_MAX as i64_0 - 1 as i64_0
            + FTS3_BUFFER_PADDING as i64_0) as sqlite3_uint64,
    ) as *mut ::core::ffi::c_char;
    if aOut.is_null() {
        return SQLITE_NOMEM;
    }
    p = aOut;
    fts3GetDeltaVarint3(&raw mut p1, pEnd1, 0 as ::core::ffi::c_int, &raw mut i1);
    fts3GetDeltaVarint3(&raw mut p2, pEnd2, 0 as ::core::ffi::c_int, &raw mut i2);
    while !p1.is_null() || !p2.is_null() {
        let mut iDiff: sqlite3_int64 = ((if bDescDoclist != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        }) * (if i1 > i2 {
            1 as ::core::ffi::c_int
        } else {
            if i1 == i2 {
                0 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            }
        })) as sqlite3_int64;
        if !p2.is_null() && !p1.is_null() && iDiff == 0 as sqlite3_int64 {
            fts3PutDeltaVarint3(
                &raw mut p,
                bDescDoclist,
                &raw mut iPrev,
                &raw mut bFirstOut,
                i1,
            );
            rc = fts3PoslistMerge(&raw mut p, &raw mut p1, &raw mut p2);
            if rc != 0 {
                break;
            }
            fts3GetDeltaVarint3(&raw mut p1, pEnd1, bDescDoclist, &raw mut i1);
            fts3GetDeltaVarint3(&raw mut p2, pEnd2, bDescDoclist, &raw mut i2);
        } else if p2.is_null() || !p1.is_null() && iDiff < 0 as sqlite3_int64 {
            fts3PutDeltaVarint3(
                &raw mut p,
                bDescDoclist,
                &raw mut iPrev,
                &raw mut bFirstOut,
                i1,
            );
            fts3PoslistCopy(&raw mut p, &raw mut p1);
            fts3GetDeltaVarint3(&raw mut p1, pEnd1, bDescDoclist, &raw mut i1);
        } else {
            fts3PutDeltaVarint3(
                &raw mut p,
                bDescDoclist,
                &raw mut iPrev,
                &raw mut bFirstOut,
                i2,
            );
            fts3PoslistCopy(&raw mut p, &raw mut p2);
            fts3GetDeltaVarint3(&raw mut p2, pEnd2, bDescDoclist, &raw mut i2);
        }
    }
    if rc != SQLITE_OK {
        sqlite3_free(aOut as *mut ::core::ffi::c_void);
        aOut = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = aOut;
    } else {
        memset(
            aOut.offset(p.offset_from(aOut) as ::core::ffi::c_long as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            FTS3_BUFFER_PADDING as size_t,
        );
    }
    *paOut = aOut;
    *pnOut = p.offset_from(aOut) as ::core::ffi::c_long as ::core::ffi::c_int;
    return rc;
}
unsafe extern "C" fn fts3DoclistPhraseMerge(
    mut bDescDoclist: ::core::ffi::c_int,
    mut nDist: ::core::ffi::c_int,
    mut aLeft: *mut ::core::ffi::c_char,
    mut nLeft: ::core::ffi::c_int,
    mut paRight: *mut *mut ::core::ffi::c_char,
    mut pnRight: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i1: sqlite3_int64 = 0 as sqlite3_int64;
    let mut i2: sqlite3_int64 = 0 as sqlite3_int64;
    let mut iPrev: sqlite3_int64 = 0 as sqlite3_int64;
    let mut aRight: *mut ::core::ffi::c_char = *paRight;
    let mut pEnd1: *mut ::core::ffi::c_char =
        aLeft.offset(nLeft as isize) as *mut ::core::ffi::c_char;
    let mut pEnd2: *mut ::core::ffi::c_char =
        aRight.offset(*pnRight as isize) as *mut ::core::ffi::c_char;
    let mut p1: *mut ::core::ffi::c_char = aLeft;
    let mut p2: *mut ::core::ffi::c_char = aRight;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bFirstOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut aOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if bDescDoclist != 0 {
        aOut = sqlite3_malloc64(
            (*pnRight as sqlite3_int64 + FTS3_VARINT_MAX as sqlite3_int64) as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if aOut.is_null() {
            return SQLITE_NOMEM;
        }
    } else {
        aOut = aRight;
    }
    p = aOut;
    fts3GetDeltaVarint3(&raw mut p1, pEnd1, 0 as ::core::ffi::c_int, &raw mut i1);
    fts3GetDeltaVarint3(&raw mut p2, pEnd2, 0 as ::core::ffi::c_int, &raw mut i2);
    while !p1.is_null() && !p2.is_null() {
        let mut iDiff: sqlite3_int64 = ((if bDescDoclist != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        }) * (if i1 > i2 {
            1 as ::core::ffi::c_int
        } else {
            if i1 == i2 {
                0 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            }
        })) as sqlite3_int64;
        if iDiff == 0 as sqlite3_int64 {
            let mut pSave: *mut ::core::ffi::c_char = p;
            let mut iPrevSave: sqlite3_int64 = iPrev;
            let mut bFirstOutSave: ::core::ffi::c_int = bFirstOut;
            fts3PutDeltaVarint3(
                &raw mut p,
                bDescDoclist,
                &raw mut iPrev,
                &raw mut bFirstOut,
                i1,
            );
            if 0 as ::core::ffi::c_int
                == fts3PoslistPhraseMerge(
                    &raw mut p,
                    nDist,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    &raw mut p1,
                    &raw mut p2,
                )
            {
                p = pSave;
                iPrev = iPrevSave;
                bFirstOut = bFirstOutSave;
            }
            fts3GetDeltaVarint3(&raw mut p1, pEnd1, bDescDoclist, &raw mut i1);
            fts3GetDeltaVarint3(&raw mut p2, pEnd2, bDescDoclist, &raw mut i2);
        } else if iDiff < 0 as sqlite3_int64 {
            fts3PoslistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p1,
            );
            fts3GetDeltaVarint3(&raw mut p1, pEnd1, bDescDoclist, &raw mut i1);
        } else {
            fts3PoslistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p2,
            );
            fts3GetDeltaVarint3(&raw mut p2, pEnd2, bDescDoclist, &raw mut i2);
        }
    }
    *pnRight = p.offset_from(aOut) as ::core::ffi::c_long as ::core::ffi::c_int;
    if bDescDoclist != 0 {
        sqlite3_free(aRight as *mut ::core::ffi::c_void);
        *paRight = aOut;
    }
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3FirstFilter(
    mut iDelta: sqlite3_int64,
    mut pList: *mut ::core::ffi::c_char,
    mut nList: ::core::ffi::c_int,
    mut pOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut nOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bWritten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char = pList;
    let mut pEnd: *mut ::core::ffi::c_char =
        pList.offset(nList as isize) as *mut ::core::ffi::c_char;
    if *p as ::core::ffi::c_int != 0x1 as ::core::ffi::c_int {
        if *p as ::core::ffi::c_int == 0x2 as ::core::ffi::c_int {
            nOut += sqlite3Fts3PutVarint(
                pOut.offset(nOut as isize) as *mut ::core::ffi::c_char,
                iDelta as sqlite_int64,
            );
            let fresh27 = nOut;
            nOut = nOut + 1;
            *pOut.offset(fresh27 as isize) = 0x2 as ::core::ffi::c_char;
            bWritten = 1 as ::core::ffi::c_int;
        }
        fts3ColumnlistCopy(
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            &raw mut p,
        );
    }
    while p < pEnd {
        let mut iCol: sqlite3_int64 = 0;
        p = p.offset(1);
        p = p.offset(sqlite3Fts3GetVarint(p, &raw mut iCol) as isize);
        if *p as ::core::ffi::c_int == 0x2 as ::core::ffi::c_int {
            if bWritten == 0 as ::core::ffi::c_int {
                nOut += sqlite3Fts3PutVarint(
                    pOut.offset(nOut as isize) as *mut ::core::ffi::c_char,
                    iDelta as sqlite_int64,
                );
                bWritten = 1 as ::core::ffi::c_int;
            }
            let fresh28 = nOut;
            nOut = nOut + 1;
            *pOut.offset(fresh28 as isize) = 0x1 as ::core::ffi::c_char;
            nOut += sqlite3Fts3PutVarint(
                pOut.offset(nOut as isize) as *mut ::core::ffi::c_char,
                iCol as sqlite_int64,
            );
            let fresh29 = nOut;
            nOut = nOut + 1;
            *pOut.offset(fresh29 as isize) = 0x2 as ::core::ffi::c_char;
        }
        fts3ColumnlistCopy(
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            &raw mut p,
        );
    }
    if bWritten != 0 {
        let fresh30 = nOut;
        nOut = nOut + 1;
        *pOut.offset(fresh30 as isize) = 0 as ::core::ffi::c_char;
    }
    return nOut;
}
unsafe extern "C" fn fts3TermSelectFinishMerge(
    mut p: *mut Fts3Table,
    mut pTS: *mut TermSelect,
) -> ::core::ffi::c_int {
    let mut aOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as ::core::ffi::c_int
    {
        if !(*pTS).aaOutput[i as usize].is_null() {
            if aOut.is_null() {
                aOut = (*pTS).aaOutput[i as usize];
                nOut = (*pTS).anOutput[i as usize];
                (*pTS).aaOutput[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
            } else {
                let mut nNew: ::core::ffi::c_int = 0;
                let mut aNew: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut rc: ::core::ffi::c_int = fts3DoclistOrMerge(
                    (*p).bDescIdx as ::core::ffi::c_int,
                    (*pTS).aaOutput[i as usize],
                    (*pTS).anOutput[i as usize],
                    aOut,
                    nOut,
                    &raw mut aNew,
                    &raw mut nNew,
                );
                if rc != SQLITE_OK {
                    sqlite3_free(aOut as *mut ::core::ffi::c_void);
                    return rc;
                }
                sqlite3_free((*pTS).aaOutput[i as usize] as *mut ::core::ffi::c_void);
                sqlite3_free(aOut as *mut ::core::ffi::c_void);
                (*pTS).aaOutput[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
                aOut = aNew;
                nOut = nNew;
            }
        }
        i += 1;
    }
    (*pTS).aaOutput[0 as ::core::ffi::c_int as usize] = aOut;
    (*pTS).anOutput[0 as ::core::ffi::c_int as usize] = nOut;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3TermSelectMerge(
    mut p: *mut Fts3Table,
    mut pTS: *mut TermSelect,
    mut aDoclist: *mut ::core::ffi::c_char,
    mut nDoclist: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*pTS).aaOutput[0 as ::core::ffi::c_int as usize].is_null() {
        (*pTS).aaOutput[0 as ::core::ffi::c_int as usize] = sqlite3_malloc64(
            (nDoclist as i64_0 + FTS3_VARINT_MAX as i64_0 + 1 as i64_0) as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        (*pTS).anOutput[0 as ::core::ffi::c_int as usize] = nDoclist;
        if !(*pTS).aaOutput[0 as ::core::ffi::c_int as usize].is_null() {
            memcpy(
                (*pTS).aaOutput[0 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_void,
                aDoclist as *const ::core::ffi::c_void,
                nDoclist as size_t,
            );
            memset(
                (*(&raw mut (*pTS).aaOutput as *mut *mut ::core::ffi::c_char)
                    .offset(0 as ::core::ffi::c_int as isize))
                .offset(nDoclist as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                FTS3_VARINT_MAX as size_t,
            );
        } else {
            return SQLITE_NOMEM;
        }
    } else {
        let mut aMerge: *mut ::core::ffi::c_char = aDoclist;
        let mut nMerge: ::core::ffi::c_int = nDoclist;
        let mut iOut: ::core::ffi::c_int = 0;
        iOut = 0 as ::core::ffi::c_int;
        while iOut
            < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 16]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                as ::core::ffi::c_int
        {
            if (*pTS).aaOutput[iOut as usize].is_null() {
                (*pTS).aaOutput[iOut as usize] = aMerge;
                (*pTS).anOutput[iOut as usize] = nMerge;
                break;
            } else {
                let mut aNew: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut nNew: ::core::ffi::c_int = 0;
                let mut rc: ::core::ffi::c_int = fts3DoclistOrMerge(
                    (*p).bDescIdx as ::core::ffi::c_int,
                    aMerge,
                    nMerge,
                    (*pTS).aaOutput[iOut as usize],
                    (*pTS).anOutput[iOut as usize],
                    &raw mut aNew,
                    &raw mut nNew,
                );
                if rc != SQLITE_OK {
                    if aMerge != aDoclist {
                        sqlite3_free(aMerge as *mut ::core::ffi::c_void);
                    }
                    return rc;
                }
                if aMerge != aDoclist {
                    sqlite3_free(aMerge as *mut ::core::ffi::c_void);
                }
                sqlite3_free((*pTS).aaOutput[iOut as usize] as *mut ::core::ffi::c_void);
                (*pTS).aaOutput[iOut as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
                aMerge = aNew;
                nMerge = nNew;
                if iOut + 1 as ::core::ffi::c_int
                    == (::core::mem::size_of::<[*mut ::core::ffi::c_char; 16]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as ::core::ffi::c_int
                {
                    (*pTS).aaOutput[iOut as usize] = aMerge;
                    (*pTS).anOutput[iOut as usize] = nMerge;
                }
                iOut += 1;
            }
        }
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3SegReaderCursorAppend(
    mut pCsr: *mut Fts3MultiSegReader,
    mut pNew: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    if (*pCsr).nSegment % 16 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut apNew: *mut *mut Fts3SegReader = ::core::ptr::null_mut::<*mut Fts3SegReader>();
        let mut nByte: sqlite3_int64 = (((*pCsr).nSegment + 16 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<*mut Fts3SegReader>() as usize)
            as sqlite3_int64;
        apNew = sqlite3_realloc64(
            (*pCsr).apSegment as *mut ::core::ffi::c_void,
            nByte as sqlite3_uint64,
        ) as *mut *mut Fts3SegReader;
        if apNew.is_null() {
            sqlite3Fts3SegReaderFree(pNew);
            return SQLITE_NOMEM;
        }
        (*pCsr).apSegment = apNew;
    }
    let fresh0 = (*pCsr).nSegment;
    (*pCsr).nSegment = (*pCsr).nSegment + 1;
    let ref mut fresh1 = *(*pCsr).apSegment.offset(fresh0 as isize);
    *fresh1 = pNew;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3SegReaderCursor(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut isPrefix: ::core::ffi::c_int,
    mut isScan: ::core::ffi::c_int,
    mut pCsr: *mut Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc2: ::core::ffi::c_int = 0;
    if iLevel < 0 as ::core::ffi::c_int && !(*p).aIndex.is_null() && (*p).iPrevLangid == iLangid {
        let mut pSeg: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
        rc = sqlite3Fts3SegReaderPending(
            p,
            iIndex,
            zTerm,
            nTerm,
            (isPrefix != 0 || isScan != 0) as ::core::ffi::c_int,
            &raw mut pSeg,
        );
        if rc == SQLITE_OK && !pSeg.is_null() {
            rc = fts3SegReaderCursorAppend(pCsr, pSeg);
        }
    }
    if iLevel != FTS3_SEGCURSOR_PENDING {
        if rc == SQLITE_OK {
            rc = sqlite3Fts3AllSegdirs(p, iLangid, iIndex, iLevel, &raw mut pStmt);
        }
        while rc == SQLITE_OK && {
            rc = sqlite3_step(pStmt);
            SQLITE_ROW == rc
        } {
            let mut pSeg_0: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
            let mut iStartBlock: sqlite3_int64 =
                sqlite3_column_int64(pStmt, 1 as ::core::ffi::c_int);
            let mut iLeavesEndBlock: sqlite3_int64 =
                sqlite3_column_int64(pStmt, 2 as ::core::ffi::c_int);
            let mut iEndBlock: sqlite3_int64 = sqlite3_column_int64(pStmt, 3 as ::core::ffi::c_int);
            let mut nRoot: ::core::ffi::c_int =
                sqlite3_column_bytes(pStmt, 4 as ::core::ffi::c_int);
            let mut zRoot: *const ::core::ffi::c_char =
                sqlite3_column_blob(pStmt, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if iStartBlock != 0 && !zTerm.is_null() && !zRoot.is_null() {
                let mut pi: *mut sqlite3_int64 = if isPrefix != 0 {
                    &raw mut iLeavesEndBlock
                } else {
                    ::core::ptr::null_mut::<sqlite3_int64>()
                };
                rc = fts3SelectLeaf(p, zTerm, nTerm, zRoot, nRoot, &raw mut iStartBlock, pi);
                if rc != SQLITE_OK {
                    break;
                }
                if isPrefix == 0 as ::core::ffi::c_int && isScan == 0 as ::core::ffi::c_int {
                    iLeavesEndBlock = iStartBlock;
                }
            }
            rc = sqlite3Fts3SegReaderNew(
                (*pCsr).nSegment + 1 as ::core::ffi::c_int,
                (isPrefix == 0 as ::core::ffi::c_int && isScan == 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int,
                iStartBlock,
                iLeavesEndBlock,
                iEndBlock,
                zRoot,
                nRoot,
                &raw mut pSeg_0,
            );
            if rc != SQLITE_OK {
                break;
            }
            rc = fts3SegReaderCursorAppend(pCsr, pSeg_0);
        }
    }
    rc2 = sqlite3_reset(pStmt);
    if rc == SQLITE_DONE {
        rc = rc2;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SegReaderCursor(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut isPrefix: ::core::ffi::c_int,
    mut isScan: ::core::ffi::c_int,
    mut pCsr: *mut Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3MultiSegReader>() as size_t,
    );
    return fts3SegReaderCursor(
        p, iLangid, iIndex, iLevel, zTerm, nTerm, isPrefix, isScan, pCsr,
    );
}
unsafe extern "C" fn fts3SegReaderCursorAddZero(
    mut p: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut pCsr: *mut Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    return fts3SegReaderCursor(
        p,
        iLangid,
        0 as ::core::ffi::c_int,
        FTS3_SEGCURSOR_ALL,
        zTerm,
        nTerm,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        pCsr,
    );
}
unsafe extern "C" fn fts3TermSegReaderCursor(
    mut pCsr: *mut Fts3Cursor,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut isPrefix: ::core::ffi::c_int,
    mut ppSegcsr: *mut *mut Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut pSegcsr: *mut Fts3MultiSegReader = ::core::ptr::null_mut::<Fts3MultiSegReader>();
    let mut rc: ::core::ffi::c_int = SQLITE_NOMEM;
    pSegcsr = sqlite3_malloc(::core::mem::size_of::<Fts3MultiSegReader>() as ::core::ffi::c_int)
        as *mut Fts3MultiSegReader;
    if !pSegcsr.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut bFound: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
        if isPrefix != 0 {
            i = 1 as ::core::ffi::c_int;
            while bFound == 0 as ::core::ffi::c_int && i < (*p).nIndex {
                if (*(*p).aIndex.offset(i as isize)).nPrefix == nTerm {
                    bFound = 1 as ::core::ffi::c_int;
                    rc = sqlite3Fts3SegReaderCursor(
                        p,
                        (*pCsr).iLangid,
                        i,
                        FTS3_SEGCURSOR_ALL,
                        zTerm,
                        nTerm,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        pSegcsr,
                    );
                    (*pSegcsr).bLookup = 1 as ::core::ffi::c_int;
                }
                i += 1;
            }
            i = 1 as ::core::ffi::c_int;
            while bFound == 0 as ::core::ffi::c_int && i < (*p).nIndex {
                if (*(*p).aIndex.offset(i as isize)).nPrefix == nTerm + 1 as ::core::ffi::c_int {
                    bFound = 1 as ::core::ffi::c_int;
                    rc = sqlite3Fts3SegReaderCursor(
                        p,
                        (*pCsr).iLangid,
                        i,
                        FTS3_SEGCURSOR_ALL,
                        zTerm,
                        nTerm,
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        pSegcsr,
                    );
                    if rc == SQLITE_OK {
                        rc = fts3SegReaderCursorAddZero(p, (*pCsr).iLangid, zTerm, nTerm, pSegcsr);
                    }
                }
                i += 1;
            }
        }
        if bFound == 0 as ::core::ffi::c_int {
            rc = sqlite3Fts3SegReaderCursor(
                p,
                (*pCsr).iLangid,
                0 as ::core::ffi::c_int,
                FTS3_SEGCURSOR_ALL,
                zTerm,
                nTerm,
                isPrefix,
                0 as ::core::ffi::c_int,
                pSegcsr,
            );
            (*pSegcsr).bLookup = (isPrefix == 0) as ::core::ffi::c_int;
        }
    }
    *ppSegcsr = pSegcsr;
    return rc;
}
unsafe extern "C" fn fts3SegReaderCursorFree(mut pSegcsr: *mut Fts3MultiSegReader) {
    sqlite3Fts3SegReaderFinish(pSegcsr);
    sqlite3_free(pSegcsr as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn fts3TermSelect(
    mut p: *mut Fts3Table,
    mut pTok: *mut Fts3PhraseToken,
    mut iColumn: ::core::ffi::c_int,
    mut pnOut: *mut ::core::ffi::c_int,
    mut ppOut: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pSegcsr: *mut Fts3MultiSegReader = ::core::ptr::null_mut::<Fts3MultiSegReader>();
    let mut tsc: TermSelect = TermSelect {
        aaOutput: [::core::ptr::null_mut::<::core::ffi::c_char>(); 16],
        anOutput: [0; 16],
    };
    let mut filter: Fts3SegFilter = Fts3SegFilter {
        zTerm: ::core::ptr::null::<::core::ffi::c_char>(),
        nTerm: 0,
        iCol: 0,
        flags: 0,
    };
    pSegcsr = (*pTok).pSegcsr;
    memset(
        &raw mut tsc as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<TermSelect>() as size_t,
    );
    filter.flags = FTS3_SEGMENT_IGNORE_EMPTY
        | FTS3_SEGMENT_REQUIRE_POS
        | (if (*pTok).isPrefix != 0 {
            FTS3_SEGMENT_PREFIX
        } else {
            0 as ::core::ffi::c_int
        })
        | (if (*pTok).bFirst != 0 {
            FTS3_SEGMENT_FIRST
        } else {
            0 as ::core::ffi::c_int
        })
        | (if iColumn < (*p).nColumn {
            FTS3_SEGMENT_COLUMN_FILTER
        } else {
            0 as ::core::ffi::c_int
        });
    filter.iCol = iColumn;
    filter.zTerm = (*pTok).z;
    filter.nTerm = (*pTok).n;
    rc = sqlite3Fts3SegReaderStart(p, pSegcsr, &raw mut filter);
    while SQLITE_OK == rc && {
        rc = sqlite3Fts3SegReaderStep(p, pSegcsr);
        SQLITE_ROW == rc
    } {
        rc = fts3TermSelectMerge(p, &raw mut tsc, (*pSegcsr).aDoclist, (*pSegcsr).nDoclist);
    }
    if rc == SQLITE_OK {
        rc = fts3TermSelectFinishMerge(p, &raw mut tsc);
    }
    if rc == SQLITE_OK {
        *ppOut = tsc.aaOutput[0 as ::core::ffi::c_int as usize];
        *pnOut = tsc.anOutput[0 as ::core::ffi::c_int as usize];
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 16]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                as ::core::ffi::c_int
        {
            sqlite3_free(tsc.aaOutput[i as usize] as *mut ::core::ffi::c_void);
            i += 1;
        }
    }
    fts3SegReaderCursorFree(pSegcsr);
    (*pTok).pSegcsr = ::core::ptr::null_mut::<Fts3MultiSegReader>();
    return rc;
}
unsafe extern "C" fn fts3DoclistCountDocids(
    mut aList: *mut ::core::ffi::c_char,
    mut nList: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nDoc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !aList.is_null() {
        let mut aEnd: *mut ::core::ffi::c_char =
            aList.offset(nList as isize) as *mut ::core::ffi::c_char;
        let mut p: *mut ::core::ffi::c_char = aList;
        while p < aEnd {
            nDoc += 1;
            loop {
                let fresh36 = p;
                p = p.offset(1);
                if !(*fresh36 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0) {
                    break;
                }
            }
            fts3PoslistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p,
            );
        }
    }
    return nDoc;
}
unsafe extern "C" fn fts3NextMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    if (*pCsr).eSearch as ::core::ffi::c_int == FTS3_DOCID_SEARCH
        || (*pCsr).eSearch as ::core::ffi::c_int == FTS3_FULLSCAN_SEARCH
    {
        let mut pTab: *mut Fts3Table = (*pCursor).pVtab as *mut Fts3Table;
        (*pTab).bLock += 1;
        if SQLITE_ROW != sqlite3_step((*pCsr).pStmt) {
            (*pCsr).isEof = 1 as u8_0;
            rc = sqlite3_reset((*pCsr).pStmt);
        } else {
            (*pCsr).iPrevId = sqlite3_column_int64((*pCsr).pStmt, 0 as ::core::ffi::c_int);
            rc = SQLITE_OK;
        }
        (*pTab).bLock -= 1;
    } else {
        rc = fts3EvalNext(pCursor as *mut Fts3Cursor);
    }
    return rc;
}
unsafe extern "C" fn fts3DocidRange(
    mut pVal: *mut sqlite3_value,
    mut iDefault: i64_0,
) -> sqlite3_int64 {
    if !pVal.is_null() {
        let mut eType: ::core::ffi::c_int = sqlite3_value_numeric_type(pVal);
        if eType == SQLITE_INTEGER {
            return sqlite3_value_int64(pVal);
        }
    }
    return iDefault as sqlite3_int64;
}
unsafe extern "C" fn fts3FilterMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut _nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut eSearch: ::core::ffi::c_int = 0;
    let mut p: *mut Fts3Table = (*pCursor).pVtab as *mut Fts3Table;
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    let mut pCons: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
    let mut pLangid: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
    let mut pDocidGe: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
    let mut pDocidLe: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
    let mut iIdx: ::core::ffi::c_int = 0;
    if (*p).bLock != 0 {
        return SQLITE_ERROR;
    }
    eSearch = idxNum & 0xffff as ::core::ffi::c_int;
    iIdx = 0 as ::core::ffi::c_int;
    if eSearch != FTS3_FULLSCAN_SEARCH {
        let fresh32 = iIdx;
        iIdx = iIdx + 1;
        pCons = *apVal.offset(fresh32 as isize);
    }
    if idxNum & FTS3_HAVE_LANGID != 0 {
        let fresh33 = iIdx;
        iIdx = iIdx + 1;
        pLangid = *apVal.offset(fresh33 as isize);
    }
    if idxNum & FTS3_HAVE_DOCID_GE != 0 {
        let fresh34 = iIdx;
        iIdx = iIdx + 1;
        pDocidGe = *apVal.offset(fresh34 as isize);
    }
    if idxNum & FTS3_HAVE_DOCID_LE != 0 {
        let fresh35 = iIdx;
        iIdx = iIdx + 1;
        pDocidLe = *apVal.offset(fresh35 as isize);
    }
    fts3ClearCursor(pCsr);
    (*pCsr).iMinDocid = fts3DocidRange(pDocidGe, SMALLEST_INT64) as i64_0;
    (*pCsr).iMaxDocid = fts3DocidRange(pDocidLe, LARGEST_INT64) as i64_0;
    if !idxStr.is_null() {
        (*pCsr).bDesc = (*idxStr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32) as ::core::ffi::c_int as u8_0;
    } else {
        (*pCsr).bDesc = (*p).bDescIdx;
    }
    (*pCsr).eSearch = eSearch as i16_0;
    if eSearch != FTS3_DOCID_SEARCH && eSearch != FTS3_FULLSCAN_SEARCH {
        let mut iCol: ::core::ffi::c_int = eSearch - FTS3_FULLTEXT_SEARCH;
        let mut zQuery: *const ::core::ffi::c_char =
            sqlite3_value_text(pCons) as *const ::core::ffi::c_char;
        if zQuery.is_null() && sqlite3_value_type(pCons) != SQLITE_NULL {
            return SQLITE_NOMEM;
        }
        (*pCsr).iLangid = 0 as ::core::ffi::c_int;
        if !pLangid.is_null() {
            (*pCsr).iLangid = sqlite3_value_int(pLangid);
        }
        rc = sqlite3Fts3ExprParse(
            (*p).pTokenizer,
            (*pCsr).iLangid,
            (*p).azColumn,
            (*p).bFts4 as ::core::ffi::c_int,
            (*p).nColumn,
            iCol,
            zQuery,
            -(1 as ::core::ffi::c_int),
            &raw mut (*pCsr).pExpr,
            &raw mut (*p).base.zErrMsg,
        );
        if rc != SQLITE_OK {
            return rc;
        }
        rc = fts3EvalStart(pCsr);
        sqlite3Fts3SegmentsClose(p);
        if rc != SQLITE_OK {
            return rc;
        }
        (*pCsr).pNextId = (*pCsr).aDoclist;
        (*pCsr).iPrevId = 0 as sqlite3_int64;
    }
    if eSearch == FTS3_FULLSCAN_SEARCH {
        if !pDocidGe.is_null() || !pDocidLe.is_null() {
            zSql = sqlite3_mprintf(
                b"SELECT %s WHERE rowid BETWEEN %lld AND %lld ORDER BY rowid %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*p).zReadExprlist,
                (*pCsr).iMinDocid,
                (*pCsr).iMaxDocid,
                if (*pCsr).bDesc as ::core::ffi::c_int != 0 {
                    b"DESC\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"ASC\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
        } else {
            zSql = sqlite3_mprintf(
                b"SELECT %s ORDER BY rowid %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).zReadExprlist,
                if (*pCsr).bDesc as ::core::ffi::c_int != 0 {
                    b"DESC\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"ASC\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
        }
        if !zSql.is_null() {
            (*p).bLock += 1;
            rc = sqlite3_prepare_v3(
                (*p).db,
                zSql,
                -(1 as ::core::ffi::c_int),
                SQLITE_PREPARE_PERSISTENT as ::core::ffi::c_uint,
                &raw mut (*pCsr).pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            (*p).bLock -= 1;
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
        } else {
            rc = SQLITE_NOMEM;
        }
    } else if eSearch == FTS3_DOCID_SEARCH {
        rc = fts3CursorSeekStmt(pCsr);
        if rc == SQLITE_OK {
            rc = sqlite3_bind_value((*pCsr).pStmt, 1 as ::core::ffi::c_int, pCons);
        }
    }
    if rc != SQLITE_OK {
        return rc;
    }
    return fts3NextMethod(pCursor);
}
unsafe extern "C" fn fts3EofMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    if (*pCsr).isEof != 0 {
        fts3ClearCursor(pCsr);
        (*pCsr).isEof = 1 as u8_0;
    }
    return (*pCsr).isEof as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3RowidMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    *pRowid = (*pCsr).iPrevId as sqlite_int64;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3ColumnMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pCtx: *mut sqlite3_context,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    let mut p: *mut Fts3Table = (*pCursor).pVtab as *mut Fts3Table;
    let mut current_block_10: u64;
    match iCol - (*p).nColumn {
        0 => {
            sqlite3_result_pointer(
                pCtx,
                pCsr as *mut ::core::ffi::c_void,
                b"fts3cursor\0" as *const u8 as *const ::core::ffi::c_char,
                None,
            );
            current_block_10 = 7976072742316086414;
        }
        1 => {
            sqlite3_result_int64(pCtx, (*pCsr).iPrevId);
            current_block_10 = 7976072742316086414;
        }
        2 => {
            if !(*pCsr).pExpr.is_null() {
                sqlite3_result_int64(pCtx, (*pCsr).iLangid as sqlite3_int64);
                current_block_10 = 7976072742316086414;
            } else if (*p).zLanguageid.is_null() {
                sqlite3_result_int(pCtx, 0 as ::core::ffi::c_int);
                current_block_10 = 7976072742316086414;
            } else {
                iCol = (*p).nColumn;
                current_block_10 = 9242256417102454865;
            }
        }
        _ => {
            current_block_10 = 9242256417102454865;
        }
    }
    match current_block_10 {
        9242256417102454865 => {
            rc = fts3CursorSeek(::core::ptr::null_mut::<sqlite3_context>(), pCsr);
            if rc == SQLITE_OK && sqlite3_data_count((*pCsr).pStmt) - 1 as ::core::ffi::c_int > iCol
            {
                sqlite3_result_value(
                    pCtx,
                    sqlite3_column_value((*pCsr).pStmt, iCol + 1 as ::core::ffi::c_int),
                );
            }
        }
        _ => {}
    }
    return rc;
}
unsafe extern "C" fn fts3UpdateMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut nArg: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    return sqlite3Fts3UpdateMethod(pVtab, nArg, apVal, pRowid as *mut sqlite3_int64);
}
unsafe extern "C" fn fts3SyncMethod(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    let nMinMerge: u32_0 = 64 as u32_0;
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = 0;
    let mut iLastRowid: i64_0 = sqlite3_last_insert_rowid((*p).db) as i64_0;
    rc = sqlite3Fts3PendingTermsFlush(p);
    if rc == SQLITE_OK
        && (*p).nLeafAdd > nMinMerge.wrapping_div(16 as u32_0)
        && (*p).nAutoincrmerge != 0
        && (*p).nAutoincrmerge != 0xff as ::core::ffi::c_int
    {
        let mut mxLevel: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut A: ::core::ffi::c_int = 0;
        rc = sqlite3Fts3MaxLevel(p, &raw mut mxLevel);
        A = (*p).nLeafAdd.wrapping_mul(mxLevel as u32_0) as ::core::ffi::c_int;
        A += A / 2 as ::core::ffi::c_int;
        if A > nMinMerge as ::core::ffi::c_int {
            rc = sqlite3Fts3Incrmerge(p, A, (*p).nAutoincrmerge);
        }
    }
    sqlite3Fts3SegmentsClose(p);
    sqlite3_set_last_insert_rowid((*p).db, iLastRowid as sqlite3_int64);
    return rc;
}
unsafe extern "C" fn fts3SetHasStat(mut p: *mut Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*p).bHasStat as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        let mut zTbl: *mut ::core::ffi::c_char = sqlite3_mprintf(
            b"%s_stat\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).zName,
        );
        if !zTbl.is_null() {
            let mut res: ::core::ffi::c_int = sqlite3_table_column_metadata(
                (*p).db,
                (*p).zDb,
                zTbl,
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            sqlite3_free(zTbl as *mut ::core::ffi::c_void);
            (*p).bHasStat = (res == SQLITE_OK) as ::core::ffi::c_int as u8_0;
        } else {
            rc = SQLITE_NOMEM;
        }
    }
    return rc;
}
unsafe extern "C" fn fts3BeginMethod(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = 0;
    (*p).nLeafAdd = 0 as u32_0;
    rc = fts3SetHasStat(p);
    return rc;
}
unsafe extern "C" fn fts3CommitMethod(mut _pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    return SQLITE_OK;
}
unsafe extern "C" fn fts3RollbackMethod(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    sqlite3Fts3PendingTermsClear(p);
    return SQLITE_OK;
}
unsafe extern "C" fn fts3ReversePoslist(
    mut pStart: *mut ::core::ffi::c_char,
    mut ppPoslist: *mut *mut ::core::ffi::c_char,
) {
    let mut p: *mut ::core::ffi::c_char =
        (*ppPoslist).offset(-(2 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    while p > pStart && {
        let fresh16 = p;
        p = p.offset(-1);
        c = *fresh16;
        c as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    } {}
    while p > pStart
        && *p as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int | c as ::core::ffi::c_int != 0
    {
        let fresh17 = p;
        p = p.offset(-1);
        c = *fresh17;
    }
    if p > pStart
        || c as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && *ppPoslist > p.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
    {
        p = p.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
    }
    loop {
        let fresh18 = p;
        p = p.offset(1);
        if !(*fresh18 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0) {
            break;
        }
    }
    *ppPoslist = p;
}
unsafe extern "C" fn fts3FunctionArg(
    mut pContext: *mut sqlite3_context,
    mut zFunc: *const ::core::ffi::c_char,
    mut pVal: *mut sqlite3_value,
    mut ppCsr: *mut *mut Fts3Cursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    *ppCsr = sqlite3_value_pointer(
        pVal,
        b"fts3cursor\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut Fts3Cursor;
    if !(*ppCsr).is_null() {
        rc = SQLITE_OK;
    } else {
        let mut zErr: *mut ::core::ffi::c_char = sqlite3_mprintf(
            b"illegal first argument to %s\0" as *const u8 as *const ::core::ffi::c_char,
            zFunc,
        );
        sqlite3_result_error(pContext, zErr, -(1 as ::core::ffi::c_int));
        sqlite3_free(zErr as *mut ::core::ffi::c_void);
        rc = SQLITE_ERROR;
    }
    return rc;
}
unsafe extern "C" fn fts3SnippetFunc(
    mut pContext: *mut sqlite3_context,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    let mut pCsr: *mut Fts3Cursor = ::core::ptr::null_mut::<Fts3Cursor>();
    let mut zStart: *const ::core::ffi::c_char =
        b"<b>\0" as *const u8 as *const ::core::ffi::c_char;
    let mut zEnd: *const ::core::ffi::c_char = b"</b>\0" as *const u8 as *const ::core::ffi::c_char;
    let mut zEllipsis: *const ::core::ffi::c_char =
        b"<b>...</b>\0" as *const u8 as *const ::core::ffi::c_char;
    let mut iCol: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut nToken: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    if nVal > 6 as ::core::ffi::c_int {
        sqlite3_result_error(
            pContext,
            b"wrong number of arguments to function snippet()\0" as *const u8
                as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
        );
        return;
    }
    if fts3FunctionArg(
        pContext,
        b"snippet\0" as *const u8 as *const ::core::ffi::c_char,
        *apVal.offset(0 as ::core::ffi::c_int as isize),
        &raw mut pCsr,
    ) != 0
    {
        return;
    }
    let mut current_block_11: u64;
    match nVal {
        6 => {
            nToken = sqlite3_value_int(*apVal.offset(5 as ::core::ffi::c_int as isize));
            current_block_11 = 10414022648205370886;
        }
        5 => {
            current_block_11 = 10414022648205370886;
        }
        4 => {
            current_block_11 = 11896608253015855976;
        }
        3 => {
            current_block_11 = 5542415300624813314;
        }
        2 => {
            current_block_11 = 6920105402610931133;
        }
        _ => {
            current_block_11 = 11050875288958768710;
        }
    }
    match current_block_11 {
        10414022648205370886 => {
            iCol = sqlite3_value_int(*apVal.offset(4 as ::core::ffi::c_int as isize));
            current_block_11 = 11896608253015855976;
        }
        _ => {}
    }
    match current_block_11 {
        11896608253015855976 => {
            zEllipsis = sqlite3_value_text(*apVal.offset(3 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
            current_block_11 = 5542415300624813314;
        }
        _ => {}
    }
    match current_block_11 {
        5542415300624813314 => {
            zEnd = sqlite3_value_text(*apVal.offset(2 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
            current_block_11 = 6920105402610931133;
        }
        _ => {}
    }
    match current_block_11 {
        6920105402610931133 => {
            zStart = sqlite3_value_text(*apVal.offset(1 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    if zEllipsis.is_null() || zEnd.is_null() || zStart.is_null() {
        sqlite3_result_error_nomem(pContext);
    } else if nToken == 0 as ::core::ffi::c_int {
        sqlite3_result_text(
            pContext,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
            SQLITE_STATIC,
        );
    } else if SQLITE_OK == fts3CursorSeek(pContext, pCsr) {
        sqlite3Fts3Snippet(pContext, pCsr, zStart, zEnd, zEllipsis, iCol, nToken);
    }
}
unsafe extern "C" fn fts3OffsetsFunc(
    mut pContext: *mut sqlite3_context,
    mut _nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    let mut pCsr: *mut Fts3Cursor = ::core::ptr::null_mut::<Fts3Cursor>();
    if fts3FunctionArg(
        pContext,
        b"offsets\0" as *const u8 as *const ::core::ffi::c_char,
        *apVal.offset(0 as ::core::ffi::c_int as isize),
        &raw mut pCsr,
    ) != 0
    {
        return;
    }
    if SQLITE_OK == fts3CursorSeek(pContext, pCsr) {
        sqlite3Fts3Offsets(pContext, pCsr);
    }
}
unsafe extern "C" fn fts3OptimizeFunc(
    mut pContext: *mut sqlite3_context,
    mut _nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    let mut rc: ::core::ffi::c_int = 0;
    let mut p: *mut Fts3Table = ::core::ptr::null_mut::<Fts3Table>();
    let mut pCursor: *mut Fts3Cursor = ::core::ptr::null_mut::<Fts3Cursor>();
    if fts3FunctionArg(
        pContext,
        b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
        *apVal.offset(0 as ::core::ffi::c_int as isize),
        &raw mut pCursor,
    ) != 0
    {
        return;
    }
    p = (*pCursor).base.pVtab as *mut Fts3Table;
    rc = sqlite3Fts3Optimize(p);
    match rc {
        SQLITE_OK => {
            sqlite3_result_text(
                pContext,
                b"Index optimized\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int),
                SQLITE_STATIC,
            );
        }
        SQLITE_DONE => {
            sqlite3_result_text(
                pContext,
                b"Index already optimal\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int),
                SQLITE_STATIC,
            );
        }
        _ => {
            sqlite3_result_error_code(pContext, rc);
        }
    };
}
unsafe extern "C" fn fts3MatchinfoFunc(
    mut pContext: *mut sqlite3_context,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    let mut pCsr: *mut Fts3Cursor = ::core::ptr::null_mut::<Fts3Cursor>();
    if SQLITE_OK
        == fts3FunctionArg(
            pContext,
            b"matchinfo\0" as *const u8 as *const ::core::ffi::c_char,
            *apVal.offset(0 as ::core::ffi::c_int as isize),
            &raw mut pCsr,
        )
    {
        let mut zArg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if nVal > 1 as ::core::ffi::c_int {
            zArg = sqlite3_value_text(*apVal.offset(1 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
        }
        sqlite3Fts3Matchinfo(pContext, pCsr, zArg);
    }
}
unsafe extern "C" fn fts3FindFunctionMethod(
    mut _pVtab: *mut sqlite3_vtab,
    mut _nArg: ::core::ffi::c_int,
    mut zName: *const ::core::ffi::c_char,
    mut pxFunc: *mut Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut _ppArg: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut aOverload: [Overloaded; 4] = [
        Overloaded {
            zName: b"snippet\0" as *const u8 as *const ::core::ffi::c_char,
            xFunc: Some(
                fts3SnippetFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
        },
        Overloaded {
            zName: b"offsets\0" as *const u8 as *const ::core::ffi::c_char,
            xFunc: Some(
                fts3OffsetsFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
        },
        Overloaded {
            zName: b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
            xFunc: Some(
                fts3OptimizeFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
        },
        Overloaded {
            zName: b"matchinfo\0" as *const u8 as *const ::core::ffi::c_char,
            xFunc: Some(
                fts3MatchinfoFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
        },
    ];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[Overloaded; 4]>() as usize)
            .wrapping_div(::core::mem::size_of::<Overloaded>() as usize)
            as ::core::ffi::c_int
    {
        if strcmp(zName, aOverload[i as usize].zName) == 0 as ::core::ffi::c_int {
            *pxFunc = aOverload[i as usize].xFunc;
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3RenameMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut db: *mut sqlite3 = (*p).db;
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SetHasStat(p);
    if rc == SQLITE_OK {
        rc = sqlite3Fts3PendingTermsFlush(p);
    }
    (*p).bIgnoreSavepoint = 1 as u8_0;
    if (*p).zContentTbl.is_null() {
        fts3DbExec(
            &raw mut rc,
            db,
            b"ALTER TABLE %Q.'%q_content'  RENAME TO '%q_content';\0" as *const u8
                as *const ::core::ffi::c_char,
            (*p).zDb,
            (*p).zName,
            zName,
        );
    }
    if (*p).bHasDocsize != 0 {
        fts3DbExec(
            &raw mut rc,
            db,
            b"ALTER TABLE %Q.'%q_docsize'  RENAME TO '%q_docsize';\0" as *const u8
                as *const ::core::ffi::c_char,
            (*p).zDb,
            (*p).zName,
            zName,
        );
    }
    if (*p).bHasStat != 0 {
        fts3DbExec(
            &raw mut rc,
            db,
            b"ALTER TABLE %Q.'%q_stat'  RENAME TO '%q_stat';\0" as *const u8
                as *const ::core::ffi::c_char,
            (*p).zDb,
            (*p).zName,
            zName,
        );
    }
    fts3DbExec(
        &raw mut rc,
        db,
        b"ALTER TABLE %Q.'%q_segments' RENAME TO '%q_segments';\0" as *const u8
            as *const ::core::ffi::c_char,
        (*p).zDb,
        (*p).zName,
        zName,
    );
    fts3DbExec(
        &raw mut rc,
        db,
        b"ALTER TABLE %Q.'%q_segdir'   RENAME TO '%q_segdir';\0" as *const u8
            as *const ::core::ffi::c_char,
        (*p).zDb,
        (*p).zName,
        zName,
    );
    (*p).bIgnoreSavepoint = 0 as u8_0;
    return rc;
}
unsafe extern "C" fn fts3SavepointMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pTab: *mut Fts3Table = pVtab as *mut Fts3Table;
    if (*pTab).bIgnoreSavepoint as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if (*(*pTab).aIndex.offset(0 as ::core::ffi::c_int as isize))
            .hPending
            .count
            > 0 as ::core::ffi::c_int
        {
            let mut zSql: *mut ::core::ffi::c_char = sqlite3_mprintf(
                b"INSERT INTO %Q.%Q(%Q) VALUES('flush')\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*pTab).zDb,
                (*pTab).zName,
                (*pTab).zName,
            );
            if !zSql.is_null() {
                (*pTab).bIgnoreSavepoint = 1 as u8_0;
                rc = sqlite3_exec(
                    (*pTab).db,
                    zSql,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                (*pTab).bIgnoreSavepoint = 0 as u8_0;
                sqlite3_free(zSql as *mut ::core::ffi::c_void);
            } else {
                rc = SQLITE_NOMEM;
            }
        }
        if rc == SQLITE_OK {
            (*pTab).iSavepoint = iSavepoint + 1 as ::core::ffi::c_int;
        }
    }
    return rc;
}
unsafe extern "C" fn fts3ReleaseMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3Table = pVtab as *mut Fts3Table;
    (*pTab).iSavepoint = iSavepoint;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3RollbackToMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3Table = pVtab as *mut Fts3Table;
    if iSavepoint + 1 as ::core::ffi::c_int <= (*pTab).iSavepoint {
        sqlite3Fts3PendingTermsClear(pTab);
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3ShadowName(mut zName: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    static mut azName: [*const ::core::ffi::c_char; 5] = [
        b"content\0" as *const u8 as *const ::core::ffi::c_char,
        b"docsize\0" as *const u8 as *const ::core::ffi::c_char,
        b"segdir\0" as *const u8 as *const ::core::ffi::c_char,
        b"segments\0" as *const u8 as *const ::core::ffi::c_char,
        b"stat\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if sqlite3_stricmp(zName, azName[i as usize]) == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3IntegrityMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut zSchema: *const ::core::ffi::c_char,
    mut zTabname: *const ::core::ffi::c_char,
    mut _isQuick: ::core::ffi::c_int,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut bOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = sqlite3Fts3IntegrityCheck(p, &raw mut bOk);
    if rc == SQLITE_ERROR || rc & 0xff as ::core::ffi::c_int == SQLITE_CORRUPT {
        *pzErr = sqlite3_mprintf(
            b"unable to validate the inverted index for FTS%d table %s.%s: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            if (*p).bFts4 as ::core::ffi::c_int != 0 {
                4 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            },
            zSchema,
            zTabname,
            sqlite3_errstr(rc),
        );
        if !(*pzErr).is_null() {
            rc = SQLITE_OK;
        }
    } else if rc == SQLITE_OK && bOk == 0 as ::core::ffi::c_int {
        *pzErr = sqlite3_mprintf(
            b"malformed inverted index for FTS%d table %s.%s\0" as *const u8
                as *const ::core::ffi::c_char,
            if (*p).bFts4 as ::core::ffi::c_int != 0 {
                4 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            },
            zSchema,
            zTabname,
        );
        if (*pzErr).is_null() {
            rc = SQLITE_NOMEM;
        }
    }
    sqlite3Fts3SegmentsClose(p);
    return rc;
}
static mut fts3Module: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 4 as ::core::ffi::c_int,
        xCreate: Some(
            fts3CreateMethod
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
            fts3ConnectMethod
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
            fts3BestIndexMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            fts3DisconnectMethod as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            fts3DestroyMethod as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            fts3OpenMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            fts3CloseMethod as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            fts3FilterMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            fts3NextMethod as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            fts3EofMethod as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            fts3ColumnMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            fts3RowidMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: Some(
            fts3UpdateMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xBegin: Some(
            fts3BeginMethod as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            fts3SyncMethod as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xCommit: Some(
            fts3CommitMethod as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xRollback: Some(
            fts3RollbackMethod as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xFindFunction: Some(
            fts3FindFunctionMethod
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
        xRename: Some(
            fts3RenameMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSavepoint: Some(
            fts3SavepointMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRelease: Some(
            fts3ReleaseMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRollbackTo: Some(
            fts3RollbackToMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShadowName: Some(
            fts3ShadowName
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
        xIntegrity: Some(
            fts3IntegrityMethod
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn hashDestroy(mut p: *mut ::core::ffi::c_void) {
    let mut pHash: *mut Fts3HashWrapper = p as *mut Fts3HashWrapper;
    (*pHash).nRef -= 1;
    if (*pHash).nRef <= 0 as ::core::ffi::c_int {
        sqlite3Fts3HashClear(&raw mut (*pHash).hash);
        sqlite3_free(pHash as *mut ::core::ffi::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3Init(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pHash: *mut Fts3HashWrapper = ::core::ptr::null_mut::<Fts3HashWrapper>();
    let mut pSimple: *const sqlite3_tokenizer_module =
        ::core::ptr::null::<sqlite3_tokenizer_module>();
    let mut pPorter: *const sqlite3_tokenizer_module =
        ::core::ptr::null::<sqlite3_tokenizer_module>();
    let mut pUnicode: *const sqlite3_tokenizer_module =
        ::core::ptr::null::<sqlite3_tokenizer_module>();
    sqlite3Fts3UnicodeTokenizer(&raw mut pUnicode);
    #[cfg(feature = "test")]
    {
        rc = sqlite3Fts3InitTerm(db);
        if rc != SQLITE_OK {
            return rc;
        }
    }
    rc = sqlite3Fts3InitAux(db);
    if rc != SQLITE_OK {
        return rc;
    }
    sqlite3Fts3SimpleTokenizerModule(&raw mut pSimple);
    sqlite3Fts3PorterTokenizerModule(&raw mut pPorter);
    pHash = sqlite3_malloc(::core::mem::size_of::<Fts3HashWrapper>() as ::core::ffi::c_int)
        as *mut Fts3HashWrapper;
    if pHash.is_null() {
        rc = SQLITE_NOMEM;
    } else {
        sqlite3Fts3HashInit(
            &raw mut (*pHash).hash,
            FTS3_HASH_STRING as ::core::ffi::c_char,
            1 as ::core::ffi::c_char,
        );
        (*pHash).nRef = 0 as ::core::ffi::c_int;
    }
    if rc == SQLITE_OK {
        if !sqlite3Fts3HashInsert(
            &raw mut (*pHash).hash,
            b"simple\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            7 as ::core::ffi::c_int,
            pSimple as *mut ::core::ffi::c_void,
        )
        .is_null()
            || !sqlite3Fts3HashInsert(
                &raw mut (*pHash).hash,
                b"porter\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                7 as ::core::ffi::c_int,
                pPorter as *mut ::core::ffi::c_void,
            )
            .is_null()
            || !sqlite3Fts3HashInsert(
                &raw mut (*pHash).hash,
                b"unicode61\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                10 as ::core::ffi::c_int,
                pUnicode as *mut ::core::ffi::c_void,
            )
            .is_null()
        {
            rc = SQLITE_NOMEM;
        }
    }
    if rc == SQLITE_OK {
        rc = sqlite3Fts3ExprInitTestInterface(db, &raw mut (*pHash).hash);
    }
    if SQLITE_OK == rc
        && {
            rc = sqlite3Fts3InitHashTable(
                db,
                &raw mut (*pHash).hash,
                b"fts3_tokenizer\0" as *const u8 as *const ::core::ffi::c_char,
            );
            SQLITE_OK == rc
        }
        && {
            rc = sqlite3_overload_function(
                db,
                b"snippet\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int),
            );
            SQLITE_OK == rc
        }
        && {
            rc = sqlite3_overload_function(
                db,
                b"offsets\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            SQLITE_OK == rc
        }
        && {
            rc = sqlite3_overload_function(
                db,
                b"matchinfo\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            SQLITE_OK == rc
        }
        && {
            rc = sqlite3_overload_function(
                db,
                b"matchinfo\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            );
            SQLITE_OK == rc
        }
        && {
            rc = sqlite3_overload_function(
                db,
                b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            SQLITE_OK == rc
        }
    {
        (*pHash).nRef += 1;
        rc = sqlite3_create_module_v2(
            db,
            b"fts3\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const fts3Module,
            pHash as *mut ::core::ffi::c_void,
            Some(hashDestroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        if rc == SQLITE_OK {
            (*pHash).nRef += 1;
            rc = sqlite3_create_module_v2(
                db,
                b"fts4\0" as *const u8 as *const ::core::ffi::c_char,
                &raw const fts3Module,
                pHash as *mut ::core::ffi::c_void,
                Some(hashDestroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
        }
        if rc == SQLITE_OK {
            (*pHash).nRef += 1;
            rc = sqlite3Fts3InitTok(
                db,
                pHash as *mut ::core::ffi::c_void as *mut Fts3Hash,
                Some(hashDestroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
        }
        return rc;
    }
    if !pHash.is_null() {
        sqlite3Fts3HashClear(&raw mut (*pHash).hash);
        sqlite3_free(pHash as *mut ::core::ffi::c_void);
    }
    return rc;
}
unsafe extern "C" fn fts3EvalAllocateReaders(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
    mut pnToken: *mut ::core::ffi::c_int,
    mut pnOr: *mut ::core::ffi::c_int,
    mut pRc: *mut ::core::ffi::c_int,
) {
    if !pExpr.is_null() && SQLITE_OK == *pRc {
        if (*pExpr).eType == FTSQUERY_PHRASE {
            let mut i: ::core::ffi::c_int = 0;
            let mut nToken: ::core::ffi::c_int = (*(*pExpr).pPhrase).nToken;
            *pnToken += nToken;
            i = 0 as ::core::ffi::c_int;
            while i < nToken {
                let mut pToken: *mut Fts3PhraseToken = (&raw mut (*(*pExpr).pPhrase).aToken
                    as *mut Fts3PhraseToken)
                    .offset(i as isize)
                    as *mut Fts3PhraseToken;
                let mut rc: ::core::ffi::c_int = fts3TermSegReaderCursor(
                    pCsr,
                    (*pToken).z,
                    (*pToken).n,
                    (*pToken).isPrefix,
                    &raw mut (*pToken).pSegcsr,
                );
                if rc != SQLITE_OK {
                    *pRc = rc;
                    return;
                }
                i += 1;
            }
            (*(*pExpr).pPhrase).iDoclistToken = -(1 as ::core::ffi::c_int);
        } else {
            *pnOr += ((*pExpr).eType == FTSQUERY_OR) as ::core::ffi::c_int;
            fts3EvalAllocateReaders(pCsr, (*pExpr).pLeft, pnToken, pnOr, pRc);
            fts3EvalAllocateReaders(pCsr, (*pExpr).pRight, pnToken, pnOr, pRc);
        }
    }
}
unsafe extern "C" fn fts3EvalPhraseMergeToken(
    mut pTab: *mut Fts3Table,
    mut p: *mut Fts3Phrase,
    mut iToken: ::core::ffi::c_int,
    mut pList: *mut ::core::ffi::c_char,
    mut nList: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if pList.is_null() {
        sqlite3_free((*p).doclist.aAll as *mut ::core::ffi::c_void);
        (*p).doclist.aAll = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*p).doclist.nAll = 0 as ::core::ffi::c_int;
    } else if (*p).iDoclistToken < 0 as ::core::ffi::c_int {
        (*p).doclist.aAll = pList;
        (*p).doclist.nAll = nList;
    } else if (*p).doclist.aAll.is_null() {
        sqlite3_free(pList as *mut ::core::ffi::c_void);
    } else {
        let mut pLeft: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pRight: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nLeft: ::core::ffi::c_int = 0;
        let mut nRight: ::core::ffi::c_int = 0;
        let mut nDiff: ::core::ffi::c_int = 0;
        if (*p).iDoclistToken < iToken {
            pLeft = (*p).doclist.aAll;
            nLeft = (*p).doclist.nAll;
            pRight = pList;
            nRight = nList;
            nDiff = iToken - (*p).iDoclistToken;
        } else {
            pRight = (*p).doclist.aAll;
            nRight = (*p).doclist.nAll;
            pLeft = pList;
            nLeft = nList;
            nDiff = (*p).iDoclistToken - iToken;
        }
        rc = fts3DoclistPhraseMerge(
            (*pTab).bDescIdx as ::core::ffi::c_int,
            nDiff,
            pLeft,
            nLeft,
            &raw mut pRight,
            &raw mut nRight,
        );
        sqlite3_free(pLeft as *mut ::core::ffi::c_void);
        (*p).doclist.aAll = pRight;
        (*p).doclist.nAll = nRight;
    }
    if iToken > (*p).iDoclistToken {
        (*p).iDoclistToken = iToken;
    }
    return rc;
}
unsafe extern "C" fn fts3EvalPhraseLoad(
    mut pCsr: *mut Fts3Cursor,
    mut p: *mut Fts3Phrase,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut iToken: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    iToken = 0 as ::core::ffi::c_int;
    while rc == SQLITE_OK && iToken < (*p).nToken {
        let mut pToken: *mut Fts3PhraseToken = (&raw mut (*p).aToken as *mut Fts3PhraseToken)
            .offset(iToken as isize)
            as *mut Fts3PhraseToken;
        if !(*pToken).pSegcsr.is_null() {
            let mut nThis: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pThis: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            rc = fts3TermSelect(pTab, pToken, (*p).iColumn, &raw mut nThis, &raw mut pThis);
            if rc == SQLITE_OK {
                rc = fts3EvalPhraseMergeToken(pTab, p, iToken, pThis, nThis);
            }
        }
        iToken += 1;
    }
    return rc;
}
unsafe extern "C" fn fts3EvalDeferredPhrase(
    mut pCsr: *mut Fts3Cursor,
    mut pPhrase: *mut Fts3Phrase,
) -> ::core::ffi::c_int {
    let mut iToken: ::core::ffi::c_int = 0;
    let mut aPoslist: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nPoslist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iPrev: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut aFree: *mut ::core::ffi::c_char = if (*pPhrase).doclist.bFreeList != 0 {
        (*pPhrase).doclist.pList
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    iToken = 0 as ::core::ffi::c_int;
    while iToken < (*pPhrase).nToken {
        let mut pToken: *mut Fts3PhraseToken = (&raw mut (*pPhrase).aToken as *mut Fts3PhraseToken)
            .offset(iToken as isize)
            as *mut Fts3PhraseToken;
        let mut pDeferred: *mut Fts3DeferredToken = (*pToken).pDeferred;
        if !pDeferred.is_null() {
            let mut pList: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut nList: ::core::ffi::c_int = 0;
            let mut rc: ::core::ffi::c_int =
                sqlite3Fts3DeferredTokenList(pDeferred, &raw mut pList, &raw mut nList);
            if rc != SQLITE_OK {
                return rc;
            }
            if pList.is_null() {
                sqlite3_free(aPoslist as *mut ::core::ffi::c_void);
                sqlite3_free(aFree as *mut ::core::ffi::c_void);
                (*pPhrase).doclist.pList = ::core::ptr::null_mut::<::core::ffi::c_char>();
                (*pPhrase).doclist.nList = 0 as ::core::ffi::c_int;
                return SQLITE_OK;
            } else if aPoslist.is_null() {
                aPoslist = pList;
                nPoslist = nList;
            } else {
                let mut aOut: *mut ::core::ffi::c_char = pList;
                let mut p1: *mut ::core::ffi::c_char = aPoslist;
                let mut p2: *mut ::core::ffi::c_char = aOut;
                fts3PoslistPhraseMerge(
                    &raw mut aOut,
                    iToken - iPrev,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    &raw mut p1,
                    &raw mut p2,
                );
                sqlite3_free(aPoslist as *mut ::core::ffi::c_void);
                aPoslist = pList;
                nPoslist = aOut.offset_from(aPoslist) as ::core::ffi::c_long as ::core::ffi::c_int;
                if nPoslist == 0 as ::core::ffi::c_int {
                    sqlite3_free(aPoslist as *mut ::core::ffi::c_void);
                    sqlite3_free(aFree as *mut ::core::ffi::c_void);
                    (*pPhrase).doclist.pList = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    (*pPhrase).doclist.nList = 0 as ::core::ffi::c_int;
                    return SQLITE_OK;
                }
            }
            iPrev = iToken;
        }
        iToken += 1;
    }
    if iPrev >= 0 as ::core::ffi::c_int {
        let mut nMaxUndeferred: ::core::ffi::c_int = (*pPhrase).iDoclistToken;
        if nMaxUndeferred < 0 as ::core::ffi::c_int {
            (*pPhrase).doclist.pList = aPoslist;
            (*pPhrase).doclist.nList = nPoslist;
            (*pPhrase).doclist.iDocid = (*pCsr).iPrevId;
            (*pPhrase).doclist.bFreeList = 1 as ::core::ffi::c_int;
        } else {
            let mut nDistance: ::core::ffi::c_int = 0;
            let mut p1_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut p2_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut aOut_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            if nMaxUndeferred > iPrev {
                p1_0 = aPoslist;
                p2_0 = (*pPhrase).doclist.pList;
                nDistance = nMaxUndeferred - iPrev;
            } else {
                p1_0 = (*pPhrase).doclist.pList;
                p2_0 = aPoslist;
                nDistance = iPrev - nMaxUndeferred;
            }
            aOut_0 = sqlite3Fts3MallocZero(nPoslist as i64_0 + FTS3_BUFFER_PADDING as i64_0)
                as *mut ::core::ffi::c_char;
            if aOut_0.is_null() {
                sqlite3_free(aPoslist as *mut ::core::ffi::c_void);
                return SQLITE_NOMEM;
            }
            (*pPhrase).doclist.pList = aOut_0;
            if fts3PoslistPhraseMerge(
                &raw mut aOut_0,
                nDistance,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                &raw mut p1_0,
                &raw mut p2_0,
            ) != 0
            {
                (*pPhrase).doclist.bFreeList = 1 as ::core::ffi::c_int;
                (*pPhrase).doclist.nList = aOut_0.offset_from((*pPhrase).doclist.pList)
                    as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            } else {
                sqlite3_free(aOut_0 as *mut ::core::ffi::c_void);
                (*pPhrase).doclist.pList = ::core::ptr::null_mut::<::core::ffi::c_char>();
                (*pPhrase).doclist.nList = 0 as ::core::ffi::c_int;
            }
            sqlite3_free(aPoslist as *mut ::core::ffi::c_void);
        }
    }
    if (*pPhrase).doclist.pList != aFree {
        sqlite3_free(aFree as *mut ::core::ffi::c_void);
    }
    return SQLITE_OK;
}
pub const MAX_INCR_PHRASE_TOKENS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn fts3EvalPhraseStart(
    mut pCsr: *mut Fts3Cursor,
    mut bOptOk: ::core::ffi::c_int,
    mut p: *mut Fts3Phrase,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut i: ::core::ffi::c_int = 0;
    let mut bHaveIncr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bIncrOk: ::core::ffi::c_int = (bOptOk != 0
        && (*pCsr).bDesc as ::core::ffi::c_int == (*pTab).bDescIdx as ::core::ffi::c_int
        && (*p).nToken <= MAX_INCR_PHRASE_TOKENS
        && (*p).nToken > 0 as ::core::ffi::c_int
        && (*pTab).bNoIncrDoclist == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while bIncrOk == 1 as ::core::ffi::c_int && i < (*p).nToken {
        let mut pToken: *mut Fts3PhraseToken = (&raw mut (*p).aToken as *mut Fts3PhraseToken)
            .offset(i as isize)
            as *mut Fts3PhraseToken;
        if (*pToken).bFirst != 0
            || !(*pToken).pSegcsr.is_null() && (*(*pToken).pSegcsr).bLookup == 0
        {
            bIncrOk = 0 as ::core::ffi::c_int;
        }
        if !(*pToken).pSegcsr.is_null() {
            bHaveIncr = 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if bIncrOk != 0 && bHaveIncr != 0 {
        let mut iCol: ::core::ffi::c_int = if (*p).iColumn >= (*pTab).nColumn {
            -(1 as ::core::ffi::c_int)
        } else {
            (*p).iColumn
        };
        i = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && i < (*p).nToken {
            let mut pToken_0: *mut Fts3PhraseToken = (&raw mut (*p).aToken as *mut Fts3PhraseToken)
                .offset(i as isize)
                as *mut Fts3PhraseToken;
            let mut pSegcsr: *mut Fts3MultiSegReader = (*pToken_0).pSegcsr;
            if !pSegcsr.is_null() {
                rc = sqlite3Fts3MsrIncrStart(pTab, pSegcsr, iCol, (*pToken_0).z, (*pToken_0).n);
            }
            i += 1;
        }
        (*p).bIncr = 1 as ::core::ffi::c_int;
    } else {
        rc = fts3EvalPhraseLoad(pCsr, p);
        (*p).bIncr = 0 as ::core::ffi::c_int;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3DoclistPrev(
    mut bDescIdx: ::core::ffi::c_int,
    mut aDoclist: *mut ::core::ffi::c_char,
    mut nDoclist: ::core::ffi::c_int,
    mut ppIter: *mut *mut ::core::ffi::c_char,
    mut piDocid: *mut sqlite3_int64,
    mut pnList: *mut ::core::ffi::c_int,
    mut pbEof: *mut u8_0,
) {
    let mut p: *mut ::core::ffi::c_char = *ppIter;
    if p.is_null() {
        let mut iDocid: sqlite3_int64 = 0 as sqlite3_int64;
        let mut pNext: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pDocid: *mut ::core::ffi::c_char = aDoclist;
        let mut pEnd: *mut ::core::ffi::c_char =
            aDoclist.offset(nDoclist as isize) as *mut ::core::ffi::c_char;
        let mut iMul: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while pDocid < pEnd {
            let mut iDelta: sqlite3_int64 = 0;
            pDocid = pDocid.offset(sqlite3Fts3GetVarint(pDocid, &raw mut iDelta) as isize);
            iDocid += iMul as sqlite3_int64 * iDelta;
            pNext = pDocid;
            fts3PoslistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut pDocid,
            );
            while pDocid < pEnd && *pDocid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                pDocid = pDocid.offset(1);
            }
            iMul = if bDescIdx != 0 {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
        }
        *pnList = pEnd.offset_from(pNext) as ::core::ffi::c_long as ::core::ffi::c_int;
        *ppIter = pNext;
        *piDocid = iDocid;
    } else {
        let mut iMul_0: ::core::ffi::c_int = if bDescIdx != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        };
        let mut iDelta_0: sqlite3_int64 = 0;
        fts3GetReverseVarint(&raw mut p, aDoclist, &raw mut iDelta_0);
        *piDocid -= iMul_0 as sqlite3_int64 * iDelta_0;
        if p == aDoclist {
            *pbEof = 1 as u8_0;
        } else {
            let mut pSave: *mut ::core::ffi::c_char = p;
            fts3ReversePoslist(aDoclist, &raw mut p);
            *pnList = pSave.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int;
        }
        *ppIter = p;
    };
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3DoclistNext(
    mut bDescIdx: ::core::ffi::c_int,
    mut aDoclist: *mut ::core::ffi::c_char,
    mut nDoclist: ::core::ffi::c_int,
    mut ppIter: *mut *mut ::core::ffi::c_char,
    mut piDocid: *mut sqlite3_int64,
    mut pbEof: *mut u8_0,
) {
    let mut p: *mut ::core::ffi::c_char = *ppIter;
    if p.is_null() {
        p = aDoclist;
        p = p.offset(sqlite3Fts3GetVarint(p, piDocid as *mut sqlite_int64) as isize);
    } else {
        fts3PoslistCopy(
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            &raw mut p,
        );
        while p < aDoclist.offset(nDoclist as isize) as *mut ::core::ffi::c_char
            && *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if p >= aDoclist.offset(nDoclist as isize) as *mut ::core::ffi::c_char {
            *pbEof = 1 as u8_0;
        } else {
            let mut iVar: sqlite3_int64 = 0;
            p = p.offset(sqlite3Fts3GetVarint(p, &raw mut iVar) as isize);
            *piDocid += (if bDescIdx != 0 {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            }) as sqlite3_int64
                * iVar;
        }
    }
    *ppIter = p;
}
unsafe extern "C" fn fts3EvalDlPhraseNext(
    mut pTab: *mut Fts3Table,
    mut pDL: *mut Fts3Doclist,
    mut pbEof: *mut u8_0,
) {
    let mut pIter: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pEnd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*pDL).pNextDocid.is_null() {
        pIter = (*pDL).pNextDocid;
    } else {
        pIter = (*pDL).aAll;
    }
    if pIter.is_null() || {
        pEnd = (*pDL).aAll.offset((*pDL).nAll as isize);
        pIter >= pEnd
    } {
        *pbEof = 1 as u8_0;
    } else {
        let mut iDelta: sqlite3_int64 = 0;
        pIter = pIter.offset(sqlite3Fts3GetVarint(pIter, &raw mut iDelta) as isize);
        if (*pTab).bDescIdx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*pDL).pNextDocid.is_null()
        {
            (*pDL).iDocid += iDelta;
        } else {
            (*pDL).iDocid -= iDelta;
        }
        (*pDL).pList = pIter;
        fts3PoslistCopy(
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            &raw mut pIter,
        );
        (*pDL).nList = pIter.offset_from((*pDL).pList) as ::core::ffi::c_long as ::core::ffi::c_int;
        while pIter < pEnd && *pIter as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            pIter = pIter.offset(1);
        }
        (*pDL).pNextDocid = pIter;
        *pbEof = 0 as u8_0;
    };
}
unsafe extern "C" fn incrPhraseTokenNext(
    mut pTab: *mut Fts3Table,
    mut pPhrase: *mut Fts3Phrase,
    mut iToken: ::core::ffi::c_int,
    mut p: *mut TokenDoclist,
    mut pbEof: *mut u8_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*pPhrase).iDoclistToken == iToken {
        fts3EvalDlPhraseNext(pTab, &raw mut (*pPhrase).doclist, pbEof);
        (*p).pList = (*pPhrase).doclist.pList;
        (*p).nList = (*pPhrase).doclist.nList;
        (*p).iDocid = (*pPhrase).doclist.iDocid;
    } else {
        let mut pToken: *mut Fts3PhraseToken = (&raw mut (*pPhrase).aToken as *mut Fts3PhraseToken)
            .offset(iToken as isize)
            as *mut Fts3PhraseToken;
        if !(*pToken).pSegcsr.is_null() {
            rc = sqlite3Fts3MsrIncrNext(
                pTab,
                (*pToken).pSegcsr,
                &raw mut (*p).iDocid,
                &raw mut (*p).pList,
                &raw mut (*p).nList,
            );
            if (*p).pList.is_null() {
                *pbEof = 1 as u8_0;
            }
        } else {
            (*p).bIgnore = 1 as ::core::ffi::c_int;
        }
    }
    return rc;
}
unsafe extern "C" fn fts3EvalIncrPhraseNext(
    mut pCsr: *mut Fts3Cursor,
    mut p: *mut Fts3Phrase,
    mut pbEof: *mut u8_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pDL: *mut Fts3Doclist = &raw mut (*p).doclist;
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut bEof: u8_0 = 0 as u8_0;
    if (*p).nToken == 1 as ::core::ffi::c_int {
        rc = sqlite3Fts3MsrIncrNext(
            pTab,
            (*(&raw mut (*p).aToken as *mut Fts3PhraseToken)
                .offset(0 as ::core::ffi::c_int as isize))
            .pSegcsr,
            &raw mut (*pDL).iDocid,
            &raw mut (*pDL).pList,
            &raw mut (*pDL).nList,
        );
        if (*pDL).pList.is_null() {
            bEof = 1 as u8_0;
        }
    } else {
        let mut bDescDoclist: ::core::ffi::c_int = (*pCsr).bDesc as ::core::ffi::c_int;
        let mut a: [TokenDoclist; 4] = [TokenDoclist {
            bIgnore: 0,
            iDocid: 0,
            pList: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nList: 0,
        }; 4];
        memset(
            &raw mut a as *mut TokenDoclist as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[TokenDoclist; 4]>() as size_t,
        );
        while bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut bMaxSet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iMax: sqlite3_int64 = 0 as sqlite3_int64;
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while rc == SQLITE_OK
                && i < (*p).nToken
                && bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                rc = incrPhraseTokenNext(
                    pTab,
                    p,
                    i,
                    (&raw mut a as *mut TokenDoclist).offset(i as isize) as *mut TokenDoclist,
                    &raw mut bEof,
                );
                if a[i as usize].bIgnore == 0 as ::core::ffi::c_int
                    && (bMaxSet == 0 as ::core::ffi::c_int
                        || (if bDescDoclist != 0 {
                            -(1 as ::core::ffi::c_int)
                        } else {
                            1 as ::core::ffi::c_int
                        }) * (if iMax > a[i as usize].iDocid {
                            1 as ::core::ffi::c_int
                        } else {
                            if iMax == a[i as usize].iDocid {
                                0 as ::core::ffi::c_int
                            } else {
                                -(1 as ::core::ffi::c_int)
                            }
                        }) < 0 as ::core::ffi::c_int)
                {
                    iMax = a[i as usize].iDocid;
                    bMaxSet = 1 as ::core::ffi::c_int;
                }
                i += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nToken {
                while rc == SQLITE_OK
                    && bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && a[i as usize].bIgnore == 0 as ::core::ffi::c_int
                    && (if bDescDoclist != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) * (if a[i as usize].iDocid > iMax {
                        1 as ::core::ffi::c_int
                    } else {
                        if a[i as usize].iDocid == iMax {
                            0 as ::core::ffi::c_int
                        } else {
                            -(1 as ::core::ffi::c_int)
                        }
                    }) < 0 as ::core::ffi::c_int
                {
                    rc = incrPhraseTokenNext(
                        pTab,
                        p,
                        i,
                        (&raw mut a as *mut TokenDoclist).offset(i as isize) as *mut TokenDoclist,
                        &raw mut bEof,
                    );
                    if (if bDescDoclist != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) * (if a[i as usize].iDocid > iMax {
                        1 as ::core::ffi::c_int
                    } else {
                        if a[i as usize].iDocid == iMax {
                            0 as ::core::ffi::c_int
                        } else {
                            -(1 as ::core::ffi::c_int)
                        }
                    }) > 0 as ::core::ffi::c_int
                    {
                        iMax = a[i as usize].iDocid;
                        i = 0 as ::core::ffi::c_int;
                    }
                }
                i += 1;
            }
            if !(bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                continue;
            }
            let mut nList: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nByte: ::core::ffi::c_int =
                a[((*p).nToken - 1 as ::core::ffi::c_int) as usize].nList;
            let mut aDoclist: *mut ::core::ffi::c_char =
                sqlite3_malloc64((nByte as i64_0 + FTS3_BUFFER_PADDING as i64_0) as sqlite3_uint64)
                    as *mut ::core::ffi::c_char;
            if aDoclist.is_null() {
                return SQLITE_NOMEM;
            }
            memcpy(
                aDoclist as *mut ::core::ffi::c_void,
                a[((*p).nToken - 1 as ::core::ffi::c_int) as usize].pList
                    as *const ::core::ffi::c_void,
                (nByte + 1 as ::core::ffi::c_int) as size_t,
            );
            memset(
                aDoclist.offset(nByte as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                FTS3_BUFFER_PADDING as size_t,
            );
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nToken - 1 as ::core::ffi::c_int {
                if a[i as usize].bIgnore == 0 as ::core::ffi::c_int {
                    let mut pL: *mut ::core::ffi::c_char = a[i as usize].pList;
                    let mut pR: *mut ::core::ffi::c_char = aDoclist;
                    let mut pOut: *mut ::core::ffi::c_char = aDoclist;
                    let mut nDist: ::core::ffi::c_int = (*p).nToken - 1 as ::core::ffi::c_int - i;
                    let mut res: ::core::ffi::c_int = fts3PoslistPhraseMerge(
                        &raw mut pOut,
                        nDist,
                        0 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        &raw mut pL,
                        &raw mut pR,
                    );
                    if res == 0 as ::core::ffi::c_int {
                        break;
                    }
                    nList = pOut.offset_from(aDoclist) as ::core::ffi::c_long as ::core::ffi::c_int;
                }
                i += 1;
            }
            if i == (*p).nToken - 1 as ::core::ffi::c_int {
                (*pDL).iDocid = iMax;
                (*pDL).pList = aDoclist;
                (*pDL).nList = nList;
                (*pDL).bFreeList = 1 as ::core::ffi::c_int;
                break;
            } else {
                sqlite3_free(aDoclist as *mut ::core::ffi::c_void);
            }
        }
    }
    *pbEof = bEof;
    return rc;
}
unsafe extern "C" fn fts3EvalPhraseNext(
    mut pCsr: *mut Fts3Cursor,
    mut p: *mut Fts3Phrase,
    mut pbEof: *mut u8_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pDL: *mut Fts3Doclist = &raw mut (*p).doclist;
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    if (*p).bIncr != 0 {
        rc = fts3EvalIncrPhraseNext(pCsr, p, pbEof);
    } else if (*pCsr).bDesc as ::core::ffi::c_int != (*pTab).bDescIdx as ::core::ffi::c_int
        && (*pDL).nAll != 0
    {
        sqlite3Fts3DoclistPrev(
            (*pTab).bDescIdx as ::core::ffi::c_int,
            (*pDL).aAll,
            (*pDL).nAll,
            &raw mut (*pDL).pNextDocid,
            &raw mut (*pDL).iDocid,
            &raw mut (*pDL).nList,
            pbEof,
        );
        (*pDL).pList = (*pDL).pNextDocid;
    } else {
        fts3EvalDlPhraseNext(pTab, pDL, pbEof);
    }
    return rc;
}
unsafe extern "C" fn fts3EvalStartReaders(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
    mut pRc: *mut ::core::ffi::c_int,
) {
    if !pExpr.is_null() && SQLITE_OK == *pRc {
        if (*pExpr).eType == FTSQUERY_PHRASE {
            let mut nToken: ::core::ffi::c_int = (*(*pExpr).pPhrase).nToken;
            if nToken != 0 {
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while i < nToken {
                    if (*(&raw mut (*(*pExpr).pPhrase).aToken as *mut Fts3PhraseToken)
                        .offset(i as isize))
                    .pDeferred
                    .is_null()
                    {
                        break;
                    }
                    i += 1;
                }
                (*pExpr).bDeferred = (i == nToken) as ::core::ffi::c_int as u8_0;
            }
            *pRc = fts3EvalPhraseStart(pCsr, 1 as ::core::ffi::c_int, (*pExpr).pPhrase);
        } else {
            fts3EvalStartReaders(pCsr, (*pExpr).pLeft, pRc);
            fts3EvalStartReaders(pCsr, (*pExpr).pRight, pRc);
            (*pExpr).bDeferred = ((*(*pExpr).pLeft).bDeferred as ::core::ffi::c_int != 0
                && (*(*pExpr).pRight).bDeferred as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int as u8_0;
        }
    }
}
unsafe extern "C" fn fts3EvalTokenCosts(
    mut pCsr: *mut Fts3Cursor,
    mut pRoot: *mut Fts3Expr,
    mut pExpr: *mut Fts3Expr,
    mut ppTC: *mut *mut Fts3TokenAndCost,
    mut ppOr: *mut *mut *mut Fts3Expr,
    mut pRc: *mut ::core::ffi::c_int,
) {
    if *pRc == SQLITE_OK {
        if (*pExpr).eType == FTSQUERY_PHRASE {
            let mut pPhrase: *mut Fts3Phrase = (*pExpr).pPhrase;
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while *pRc == SQLITE_OK && i < (*pPhrase).nToken {
                let fresh37 = *ppTC;
                *ppTC = (*ppTC).offset(1);
                let mut pTC: *mut Fts3TokenAndCost = fresh37;
                (*pTC).pPhrase = pPhrase;
                (*pTC).iToken = i;
                (*pTC).pRoot = pRoot;
                (*pTC).pToken = (&raw mut (*pPhrase).aToken as *mut Fts3PhraseToken)
                    .offset(i as isize) as *mut Fts3PhraseToken;
                (*pTC).iCol = (*pPhrase).iColumn;
                *pRc = sqlite3Fts3MsrOvfl(pCsr, (*(*pTC).pToken).pSegcsr, &raw mut (*pTC).nOvfl);
                i += 1;
            }
        } else if (*pExpr).eType != FTSQUERY_NOT {
            if (*pExpr).eType == FTSQUERY_OR {
                pRoot = (*pExpr).pLeft;
                **ppOr = pRoot;
                *ppOr = (*ppOr).offset(1);
            }
            fts3EvalTokenCosts(pCsr, pRoot, (*pExpr).pLeft, ppTC, ppOr, pRc);
            if (*pExpr).eType == FTSQUERY_OR {
                pRoot = (*pExpr).pRight;
                **ppOr = pRoot;
                *ppOr = (*ppOr).offset(1);
            }
            fts3EvalTokenCosts(pCsr, pRoot, (*pExpr).pRight, ppTC, ppOr, pRc);
        }
    }
}
unsafe extern "C" fn fts3EvalAverageDocsize(
    mut pCsr: *mut Fts3Cursor,
    mut pnPage: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*pCsr).nRowAvg == 0 as ::core::ffi::c_int {
        let mut p: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut nDoc: sqlite3_int64 = 0 as sqlite3_int64;
        let mut nByte: sqlite3_int64 = 0 as sqlite3_int64;
        let mut pEnd: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut a: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        rc = sqlite3Fts3SelectDoctotal(p, &raw mut pStmt);
        if rc != SQLITE_OK {
            return rc;
        }
        a = sqlite3_column_blob(pStmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !a.is_null() {
            pEnd = a.offset((sqlite3_column_bytes
                as unsafe extern "C" fn(
                    *mut sqlite3_stmt,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int)(pStmt, 0 as ::core::ffi::c_int)
                as isize) as *const ::core::ffi::c_char;
            a = a.offset(sqlite3Fts3GetVarintBounded(a, pEnd, &raw mut nDoc) as isize);
            while a < pEnd {
                a = a.offset(sqlite3Fts3GetVarintBounded(a, pEnd, &raw mut nByte) as isize);
            }
        }
        if nDoc == 0 as sqlite3_int64 || nByte == 0 as sqlite3_int64 {
            sqlite3_reset(pStmt);
            return FTS_CORRUPT_VTAB;
        }
        (*pCsr).nDoc = nDoc;
        (*pCsr).nRowAvg = ((nByte / nDoc + (*p).nPgsz as sqlite3_int64)
            / (*p).nPgsz as sqlite3_int64) as ::core::ffi::c_int;
        rc = sqlite3_reset(pStmt);
    }
    *pnPage = (*pCsr).nRowAvg;
    return rc;
}
unsafe extern "C" fn fts3EvalSelectDeferred(
    mut pCsr: *mut Fts3Cursor,
    mut pRoot: *mut Fts3Expr,
    mut aTC: *mut Fts3TokenAndCost,
    mut nTC: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut nDocSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut ii: ::core::ffi::c_int = 0;
    let mut nOvfl: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nMinEst: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nLoad4: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if !(*pTab).zContentTbl.is_null() {
        return SQLITE_OK;
    }
    ii = 0 as ::core::ffi::c_int;
    while ii < nTC {
        if (*aTC.offset(ii as isize)).pRoot == pRoot {
            nOvfl += (*aTC.offset(ii as isize)).nOvfl;
            nToken += 1;
        }
        ii += 1;
    }
    if nOvfl == 0 as ::core::ffi::c_int || nToken < 2 as ::core::ffi::c_int {
        return SQLITE_OK;
    }
    rc = fts3EvalAverageDocsize(pCsr, &raw mut nDocSize);
    ii = 0 as ::core::ffi::c_int;
    while ii < nToken && rc == SQLITE_OK {
        let mut iTC: ::core::ffi::c_int = 0;
        let mut pTC: *mut Fts3TokenAndCost = ::core::ptr::null_mut::<Fts3TokenAndCost>();
        iTC = 0 as ::core::ffi::c_int;
        while iTC < nTC {
            if !(*aTC.offset(iTC as isize)).pToken.is_null()
                && (*aTC.offset(iTC as isize)).pRoot == pRoot
                && (pTC.is_null() || (*aTC.offset(iTC as isize)).nOvfl < (*pTC).nOvfl)
            {
                pTC = aTC.offset(iTC as isize) as *mut Fts3TokenAndCost;
            }
            iTC += 1;
        }
        if ii != 0
            && (*pTC).nOvfl
                >= (nMinEst + nLoad4 / 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / (nLoad4 / 4 as ::core::ffi::c_int)
                    * nDocSize
        {
            let mut pToken: *mut Fts3PhraseToken = (*pTC).pToken;
            rc = sqlite3Fts3DeferToken(pCsr, pToken, (*pTC).iCol);
            fts3SegReaderCursorFree((*pToken).pSegcsr);
            (*pToken).pSegcsr = ::core::ptr::null_mut::<Fts3MultiSegReader>();
        } else {
            if ii < 12 as ::core::ffi::c_int {
                nLoad4 = nLoad4 * 4 as ::core::ffi::c_int;
            }
            if ii == 0 as ::core::ffi::c_int
                || (*(*pTC).pPhrase).nToken > 1 as ::core::ffi::c_int
                    && ii != nToken - 1 as ::core::ffi::c_int
            {
                let mut pToken_0: *mut Fts3PhraseToken = (*pTC).pToken;
                let mut nList: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut pList: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                rc = fts3TermSelect(pTab, pToken_0, (*pTC).iCol, &raw mut nList, &raw mut pList);
                if rc == SQLITE_OK {
                    rc =
                        fts3EvalPhraseMergeToken(pTab, (*pTC).pPhrase, (*pTC).iToken, pList, nList);
                }
                if rc == SQLITE_OK {
                    let mut nCount: ::core::ffi::c_int = 0;
                    nCount = fts3DoclistCountDocids(
                        (*(*pTC).pPhrase).doclist.aAll,
                        (*(*pTC).pPhrase).doclist.nAll,
                    );
                    if ii == 0 as ::core::ffi::c_int || nCount < nMinEst {
                        nMinEst = nCount;
                    }
                }
            }
        }
        (*pTC).pToken = ::core::ptr::null_mut::<Fts3PhraseToken>();
        ii += 1;
    }
    return rc;
}
unsafe extern "C" fn fts3EvalStart(mut pCsr: *mut Fts3Cursor) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nOr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    fts3EvalAllocateReaders(
        pCsr,
        (*pCsr).pExpr,
        &raw mut nToken,
        &raw mut nOr,
        &raw mut rc,
    );
    if rc == SQLITE_OK
        && nToken > 1 as ::core::ffi::c_int
        && (*pTab).bFts4 as ::core::ffi::c_int != 0
    {
        let mut aTC: *mut Fts3TokenAndCost = ::core::ptr::null_mut::<Fts3TokenAndCost>();
        aTC = sqlite3_malloc64(
            (::core::mem::size_of::<Fts3TokenAndCost>() as usize)
                .wrapping_mul(nToken as usize)
                .wrapping_add(
                    (::core::mem::size_of::<*mut Fts3Expr>() as usize)
                        .wrapping_mul(nOr as usize)
                        .wrapping_mul(2 as usize),
                ) as sqlite3_uint64,
        ) as *mut Fts3TokenAndCost;
        if aTC.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            let mut apOr: *mut *mut Fts3Expr =
                aTC.offset(nToken as isize) as *mut Fts3TokenAndCost as *mut *mut Fts3Expr;
            let mut ii: ::core::ffi::c_int = 0;
            let mut pTC: *mut Fts3TokenAndCost = aTC;
            let mut ppOr: *mut *mut Fts3Expr = apOr;
            fts3EvalTokenCosts(
                pCsr,
                ::core::ptr::null_mut::<Fts3Expr>(),
                (*pCsr).pExpr,
                &raw mut pTC,
                &raw mut ppOr,
                &raw mut rc,
            );
            nToken = pTC.offset_from(aTC) as ::core::ffi::c_long as ::core::ffi::c_int;
            nOr = ppOr.offset_from(apOr) as ::core::ffi::c_long as ::core::ffi::c_int;
            if rc == SQLITE_OK {
                rc = fts3EvalSelectDeferred(pCsr, ::core::ptr::null_mut::<Fts3Expr>(), aTC, nToken);
                ii = 0 as ::core::ffi::c_int;
                while rc == SQLITE_OK && ii < nOr {
                    rc = fts3EvalSelectDeferred(pCsr, *apOr.offset(ii as isize), aTC, nToken);
                    ii += 1;
                }
            }
            sqlite3_free(aTC as *mut ::core::ffi::c_void);
        }
    }
    fts3EvalStartReaders(pCsr, (*pCsr).pExpr, &raw mut rc);
    return rc;
}
unsafe extern "C" fn fts3EvalInvalidatePoslist(mut pPhrase: *mut Fts3Phrase) {
    if (*pPhrase).doclist.bFreeList != 0 {
        sqlite3_free((*pPhrase).doclist.pList as *mut ::core::ffi::c_void);
    }
    (*pPhrase).doclist.pList = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pPhrase).doclist.nList = 0 as ::core::ffi::c_int;
    (*pPhrase).doclist.bFreeList = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3EvalNearTrim(
    mut nNear: ::core::ffi::c_int,
    mut aTmp: *mut ::core::ffi::c_char,
    mut paPoslist: *mut *mut ::core::ffi::c_char,
    mut pnToken: *mut ::core::ffi::c_int,
    mut pPhrase: *mut Fts3Phrase,
) -> ::core::ffi::c_int {
    let mut nParam1: ::core::ffi::c_int = nNear + (*pPhrase).nToken;
    let mut nParam2: ::core::ffi::c_int = nNear + *pnToken;
    let mut nNew: ::core::ffi::c_int = 0;
    let mut p2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: ::core::ffi::c_int = 0;
    pOut = (*pPhrase).doclist.pList;
    p2 = pOut;
    res = fts3PoslistNearMerge(
        &raw mut pOut,
        aTmp,
        nParam1,
        nParam2,
        paPoslist,
        &raw mut p2,
    );
    if res != 0 {
        nNew = pOut.offset_from((*pPhrase).doclist.pList) as ::core::ffi::c_long
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        if nNew >= 0 as ::core::ffi::c_int && nNew <= (*pPhrase).doclist.nList {
            memset(
                (*pPhrase).doclist.pList.offset(nNew as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*pPhrase).doclist.nList - nNew) as size_t,
            );
            (*pPhrase).doclist.nList = nNew;
        }
        *paPoslist = (*pPhrase).doclist.pList;
        *pnToken = (*pPhrase).nToken;
    }
    return res;
}
unsafe extern "C" fn fts3EvalNextRow(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
    mut pRc: *mut ::core::ffi::c_int,
) {
    if *pRc == SQLITE_OK && (*pExpr).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut bDescDoclist: ::core::ffi::c_int = (*pCsr).bDesc as ::core::ffi::c_int;
        (*pExpr).bStart = 1 as u8_0;
        match (*pExpr).eType {
            FTSQUERY_NEAR | FTSQUERY_AND => {
                let mut pLeft: *mut Fts3Expr = (*pExpr).pLeft;
                let mut pRight: *mut Fts3Expr = (*pExpr).pRight;
                if (*pLeft).bDeferred != 0 {
                    fts3EvalNextRow(pCsr, pRight, pRc);
                    (*pExpr).iDocid = (*pRight).iDocid;
                    (*pExpr).bEof = (*pRight).bEof;
                } else if (*pRight).bDeferred != 0 {
                    fts3EvalNextRow(pCsr, pLeft, pRc);
                    (*pExpr).iDocid = (*pLeft).iDocid;
                    (*pExpr).bEof = (*pLeft).bEof;
                } else {
                    fts3EvalNextRow(pCsr, pLeft, pRc);
                    fts3EvalNextRow(pCsr, pRight, pRc);
                    while (*pLeft).bEof == 0 && (*pRight).bEof == 0 && *pRc == SQLITE_OK {
                        let mut iDiff: sqlite3_int64 = ((if bDescDoclist != 0 {
                            -(1 as ::core::ffi::c_int)
                        } else {
                            1 as ::core::ffi::c_int
                        }) * (if (*pLeft).iDocid > (*pRight).iDocid
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            if (*pLeft).iDocid == (*pRight).iDocid {
                                0 as ::core::ffi::c_int
                            } else {
                                -(1 as ::core::ffi::c_int)
                            }
                        })) as sqlite3_int64;
                        if iDiff == 0 as sqlite3_int64 {
                            break;
                        }
                        if iDiff < 0 as sqlite3_int64 {
                            fts3EvalNextRow(pCsr, pLeft, pRc);
                        } else {
                            fts3EvalNextRow(pCsr, pRight, pRc);
                        }
                    }
                    (*pExpr).iDocid = (*pLeft).iDocid;
                    (*pExpr).bEof = ((*pLeft).bEof as ::core::ffi::c_int != 0
                        || (*pRight).bEof as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int as u8_0;
                    if (*pExpr).eType == FTSQUERY_NEAR && (*pExpr).bEof as ::core::ffi::c_int != 0 {
                        if !(*(*pRight).pPhrase).doclist.aAll.is_null() {
                            let mut pDl: *mut Fts3Doclist = &raw mut (*(*pRight).pPhrase).doclist;
                            while *pRc == SQLITE_OK
                                && (*pRight).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            {
                                memset(
                                    (*pDl).pList as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    (*pDl).nList as size_t,
                                );
                                fts3EvalNextRow(pCsr, pRight, pRc);
                            }
                        }
                        if !(*pLeft).pPhrase.is_null()
                            && !(*(*pLeft).pPhrase).doclist.aAll.is_null()
                        {
                            let mut pDl_0: *mut Fts3Doclist = &raw mut (*(*pLeft).pPhrase).doclist;
                            while *pRc == SQLITE_OK
                                && (*pLeft).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            {
                                memset(
                                    (*pDl_0).pList as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    (*pDl_0).nList as size_t,
                                );
                                fts3EvalNextRow(pCsr, pLeft, pRc);
                            }
                        }
                        (*pLeft).bEof = 1 as u8_0;
                        (*pRight).bEof = (*pLeft).bEof;
                    }
                }
            }
            FTSQUERY_OR => {
                let mut pLeft_0: *mut Fts3Expr = (*pExpr).pLeft;
                let mut pRight_0: *mut Fts3Expr = (*pExpr).pRight;
                let mut iCmp: sqlite3_int64 = ((if bDescDoclist != 0 {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                }) * (if (*pLeft_0).iDocid > (*pRight_0).iDocid {
                    1 as ::core::ffi::c_int
                } else {
                    if (*pLeft_0).iDocid == (*pRight_0).iDocid {
                        0 as ::core::ffi::c_int
                    } else {
                        -(1 as ::core::ffi::c_int)
                    }
                })) as sqlite3_int64;
                if (*pRight_0).bEof as ::core::ffi::c_int != 0
                    || (*pLeft_0).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && iCmp < 0 as sqlite3_int64
                {
                    fts3EvalNextRow(pCsr, pLeft_0, pRc);
                } else if (*pLeft_0).bEof as ::core::ffi::c_int != 0 || iCmp > 0 as sqlite3_int64 {
                    fts3EvalNextRow(pCsr, pRight_0, pRc);
                } else {
                    fts3EvalNextRow(pCsr, pLeft_0, pRc);
                    fts3EvalNextRow(pCsr, pRight_0, pRc);
                }
                (*pExpr).bEof = ((*pLeft_0).bEof as ::core::ffi::c_int != 0
                    && (*pRight_0).bEof as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int as u8_0;
                iCmp = ((if bDescDoclist != 0 {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                }) * (if (*pLeft_0).iDocid > (*pRight_0).iDocid {
                    1 as ::core::ffi::c_int
                } else {
                    if (*pLeft_0).iDocid == (*pRight_0).iDocid {
                        0 as ::core::ffi::c_int
                    } else {
                        -(1 as ::core::ffi::c_int)
                    }
                })) as sqlite3_int64;
                if (*pRight_0).bEof as ::core::ffi::c_int != 0
                    || (*pLeft_0).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && iCmp < 0 as sqlite3_int64
                {
                    (*pExpr).iDocid = (*pLeft_0).iDocid;
                } else {
                    (*pExpr).iDocid = (*pRight_0).iDocid;
                }
            }
            FTSQUERY_NOT => {
                let mut pLeft_1: *mut Fts3Expr = (*pExpr).pLeft;
                let mut pRight_1: *mut Fts3Expr = (*pExpr).pRight;
                if (*pRight_1).bStart as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    fts3EvalNextRow(pCsr, pRight_1, pRc);
                }
                fts3EvalNextRow(pCsr, pLeft_1, pRc);
                if (*pLeft_1).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    while *pRc == 0
                        && (*pRight_1).bEof == 0
                        && (if bDescDoclist != 0 {
                            -(1 as ::core::ffi::c_int)
                        } else {
                            1 as ::core::ffi::c_int
                        }) * (if (*pLeft_1).iDocid > (*pRight_1).iDocid {
                            1 as ::core::ffi::c_int
                        } else {
                            if (*pLeft_1).iDocid == (*pRight_1).iDocid {
                                0 as ::core::ffi::c_int
                            } else {
                                -(1 as ::core::ffi::c_int)
                            }
                        }) > 0 as ::core::ffi::c_int
                    {
                        fts3EvalNextRow(pCsr, pRight_1, pRc);
                    }
                }
                (*pExpr).iDocid = (*pLeft_1).iDocid;
                (*pExpr).bEof = (*pLeft_1).bEof;
            }
            _ => {
                let mut pPhrase: *mut Fts3Phrase = (*pExpr).pPhrase;
                fts3EvalInvalidatePoslist(pPhrase);
                *pRc = fts3EvalPhraseNext(pCsr, pPhrase, &raw mut (*pExpr).bEof);
                (*pExpr).iDocid = (*pPhrase).doclist.iDocid;
            }
        }
    }
}
unsafe extern "C" fn fts3EvalNearTest(
    mut pExpr: *mut Fts3Expr,
    mut pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if *pRc == SQLITE_OK
        && (*pExpr).eType == FTSQUERY_NEAR
        && ((*pExpr).pParent.is_null() || (*(*pExpr).pParent).eType != FTSQUERY_NEAR)
    {
        let mut p: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
        let mut nTmp: sqlite3_int64 = 0 as sqlite3_int64;
        let mut aTmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = pExpr;
        while !(*p).pLeft.is_null() {
            nTmp += (*(*(*p).pRight).pPhrase).doclist.nList as sqlite3_int64;
            p = (*p).pLeft;
        }
        nTmp += (*(*p).pPhrase).doclist.nList as sqlite3_int64;
        aTmp = sqlite3_malloc64(
            (nTmp * 2 as sqlite3_int64 + FTS3_VARINT_MAX as sqlite3_int64) as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if aTmp.is_null() {
            *pRc = SQLITE_NOMEM;
            res = 0 as ::core::ffi::c_int;
        } else {
            let mut aPoslist: *mut ::core::ffi::c_char = (*(*p).pPhrase).doclist.pList;
            let mut nToken: ::core::ffi::c_int = (*(*p).pPhrase).nToken;
            p = (*p).pParent;
            while res != 0 && !p.is_null() && (*p).eType == FTSQUERY_NEAR {
                let mut pPhrase: *mut Fts3Phrase = (*(*p).pRight).pPhrase;
                let mut nNear: ::core::ffi::c_int = (*p).nNear;
                res = fts3EvalNearTrim(nNear, aTmp, &raw mut aPoslist, &raw mut nToken, pPhrase);
                p = (*p).pParent;
            }
            aPoslist = (*(*(*pExpr).pRight).pPhrase).doclist.pList;
            nToken = (*(*(*pExpr).pRight).pPhrase).nToken;
            p = (*pExpr).pLeft;
            while !p.is_null() && res != 0 {
                let mut nNear_0: ::core::ffi::c_int = 0;
                let mut pPhrase_0: *mut Fts3Phrase = ::core::ptr::null_mut::<Fts3Phrase>();
                nNear_0 = (*(*p).pParent).nNear;
                pPhrase_0 = if (*p).eType == FTSQUERY_NEAR {
                    (*(*p).pRight).pPhrase
                } else {
                    (*p).pPhrase
                };
                res =
                    fts3EvalNearTrim(nNear_0, aTmp, &raw mut aPoslist, &raw mut nToken, pPhrase_0);
                p = (*p).pLeft;
            }
        }
        sqlite3_free(aTmp as *mut ::core::ffi::c_void);
    }
    return res;
}
unsafe extern "C" fn fts3EvalTestExpr(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
    mut pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bHit: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if *pRc == SQLITE_OK {
        match (*pExpr).eType {
            FTSQUERY_NEAR | FTSQUERY_AND => {
                bHit = (fts3EvalTestExpr(pCsr, (*pExpr).pLeft, pRc) != 0
                    && fts3EvalTestExpr(pCsr, (*pExpr).pRight, pRc) != 0
                    && fts3EvalNearTest(pExpr, pRc) != 0)
                    as ::core::ffi::c_int;
                if bHit == 0 as ::core::ffi::c_int
                    && (*pExpr).eType == FTSQUERY_NEAR
                    && ((*pExpr).pParent.is_null() || (*(*pExpr).pParent).eType != FTSQUERY_NEAR)
                {
                    let mut p: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
                    p = pExpr;
                    while (*p).pPhrase.is_null() {
                        if (*(*p).pRight).iDocid == (*pCsr).iPrevId {
                            fts3EvalInvalidatePoslist((*(*p).pRight).pPhrase);
                        }
                        p = (*p).pLeft;
                    }
                    if (*p).iDocid == (*pCsr).iPrevId {
                        fts3EvalInvalidatePoslist((*p).pPhrase);
                    }
                }
            }
            FTSQUERY_OR => {
                let mut bHit1: ::core::ffi::c_int = fts3EvalTestExpr(pCsr, (*pExpr).pLeft, pRc);
                let mut bHit2: ::core::ffi::c_int = fts3EvalTestExpr(pCsr, (*pExpr).pRight, pRc);
                bHit = (bHit1 != 0 || bHit2 != 0) as ::core::ffi::c_int;
            }
            FTSQUERY_NOT => {
                bHit = (fts3EvalTestExpr(pCsr, (*pExpr).pLeft, pRc) != 0
                    && fts3EvalTestExpr(pCsr, (*pExpr).pRight, pRc) == 0)
                    as ::core::ffi::c_int;
            }
            _ => {
                if !(*pCsr).pDeferred.is_null()
                    && ((*pExpr).bDeferred as ::core::ffi::c_int != 0
                        || (*pExpr).iDocid == (*pCsr).iPrevId
                            && !(*(*pExpr).pPhrase).doclist.pList.is_null())
                {
                    let mut pPhrase: *mut Fts3Phrase = (*pExpr).pPhrase;
                    if (*pExpr).bDeferred != 0 {
                        fts3EvalInvalidatePoslist(pPhrase);
                    }
                    *pRc = fts3EvalDeferredPhrase(pCsr, pPhrase);
                    bHit = ((*pPhrase).doclist.pList
                        != ::core::ptr::null_mut::<::core::ffi::c_char>())
                        as ::core::ffi::c_int;
                    (*pExpr).iDocid = (*pCsr).iPrevId;
                } else {
                    bHit = ((*pExpr).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && (*pExpr).iDocid == (*pCsr).iPrevId
                        && (*(*pExpr).pPhrase).doclist.nList > 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                }
            }
        }
    }
    return bHit;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3EvalTestDeferred(
    mut pCsr: *mut Fts3Cursor,
    mut pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = *pRc;
    let mut bMiss: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if rc == SQLITE_OK {
        if !(*pCsr).pDeferred.is_null() {
            rc = fts3CursorSeek(::core::ptr::null_mut::<sqlite3_context>(), pCsr);
            if rc == SQLITE_OK {
                rc = sqlite3Fts3CacheDeferredDoclists(pCsr);
            }
        }
        bMiss = (0 as ::core::ffi::c_int == fts3EvalTestExpr(pCsr, (*pCsr).pExpr, &raw mut rc))
            as ::core::ffi::c_int;
        sqlite3Fts3FreeDeferredDoclists(pCsr);
        *pRc = rc;
    }
    return (rc == SQLITE_OK && bMiss != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3EvalNext(mut pCsr: *mut Fts3Cursor) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pExpr: *mut Fts3Expr = (*pCsr).pExpr;
    if pExpr.is_null() {
        (*pCsr).isEof = 1 as u8_0;
    } else {
        loop {
            if (*pCsr).isRequireSeek as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                sqlite3_reset((*pCsr).pStmt);
            }
            fts3EvalNextRow(pCsr, pExpr, &raw mut rc);
            (*pCsr).isEof = (*pExpr).bEof;
            (*pCsr).isRequireSeek = 1 as u8_0;
            (*pCsr).isMatchinfoNeeded = 1 as ::core::ffi::c_int;
            (*pCsr).iPrevId = (*pExpr).iDocid;
            if !((*pCsr).isEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && sqlite3Fts3EvalTestDeferred(pCsr, &raw mut rc) != 0)
            {
                break;
            }
        }
    }
    if rc == SQLITE_OK
        && ((*pCsr).bDesc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*pCsr).iPrevId > (*pCsr).iMaxDocid
            || (*pCsr).bDesc as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && (*pCsr).iPrevId < (*pCsr).iMinDocid)
    {
        (*pCsr).isEof = 1 as u8_0;
    }
    return rc;
}
unsafe extern "C" fn fts3EvalRestart(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
    mut pRc: *mut ::core::ffi::c_int,
) {
    if !pExpr.is_null() && *pRc == SQLITE_OK {
        let mut pPhrase: *mut Fts3Phrase = (*pExpr).pPhrase;
        if !pPhrase.is_null() {
            fts3EvalInvalidatePoslist(pPhrase);
            if (*pPhrase).bIncr != 0 {
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while i < (*pPhrase).nToken {
                    let mut pToken: *mut Fts3PhraseToken =
                        (&raw mut (*pPhrase).aToken as *mut Fts3PhraseToken).offset(i as isize)
                            as *mut Fts3PhraseToken;
                    if !(*pToken).pSegcsr.is_null() {
                        sqlite3Fts3MsrIncrRestart((*pToken).pSegcsr);
                    }
                    i += 1;
                }
                *pRc = fts3EvalPhraseStart(pCsr, 0 as ::core::ffi::c_int, pPhrase);
            }
            (*pPhrase).doclist.pNextDocid = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*pPhrase).doclist.iDocid = 0 as sqlite3_int64;
            (*pPhrase).pOrPoslist = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        (*pExpr).iDocid = 0 as sqlite3_int64;
        (*pExpr).bEof = 0 as u8_0;
        (*pExpr).bStart = 0 as u8_0;
        fts3EvalRestart(pCsr, (*pExpr).pLeft, pRc);
        fts3EvalRestart(pCsr, (*pExpr).pRight, pRc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3MsrCancel(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*pExpr).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut iDocid: i64_0 = (*pExpr).iDocid as i64_0;
        fts3EvalRestart(pCsr, pExpr, &raw mut rc);
        while rc == SQLITE_OK && (*pExpr).iDocid != iDocid {
            fts3EvalNextRow(pCsr, pExpr, &raw mut rc);
            if (*pExpr).bEof != 0 {
                rc = FTS_CORRUPT_VTAB;
            }
        }
    }
    return rc;
}
unsafe extern "C" fn fts3EvalUpdateCounts(mut pExpr: *mut Fts3Expr, mut nCol: ::core::ffi::c_int) {
    if !pExpr.is_null() {
        let mut pPhrase: *mut Fts3Phrase = (*pExpr).pPhrase;
        if !pPhrase.is_null() && !(*pPhrase).doclist.pList.is_null() {
            let mut iCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut p: *mut ::core::ffi::c_char = (*pPhrase).doclist.pList;
            loop {
                let mut c: u8_0 = 0 as u8_0;
                let mut iCnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while 0xfe as ::core::ffi::c_int
                    & (*p as ::core::ffi::c_int | c as ::core::ffi::c_int)
                    != 0
                {
                    if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        iCnt += 1;
                    }
                    let fresh24 = p;
                    p = p.offset(1);
                    c = (*fresh24 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int) as u8_0;
                }
                let ref mut fresh25 = *(*pExpr)
                    .aMI
                    .offset((iCol * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
                *fresh25 = (*fresh25).wrapping_add(iCnt as u32_0);
                let ref mut fresh26 = *(*pExpr)
                    .aMI
                    .offset((iCol * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize);
                *fresh26 = (*fresh26)
                    .wrapping_add((iCnt > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as u32_0);
                if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
                p = p.offset(1);
                p = p.offset(
                    (if *(p as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                        sqlite3Fts3GetVarint32(p, &raw mut iCol)
                    } else {
                        iCol = *(p as *mut u8_0) as ::core::ffi::c_int;
                        1 as ::core::ffi::c_int
                    }) as isize,
                );
                if !(iCol < nCol) {
                    break;
                }
            }
        }
        fts3EvalUpdateCounts((*pExpr).pLeft, nCol);
        fts3EvalUpdateCounts((*pExpr).pRight, nCol);
    }
}
unsafe extern "C" fn fts3AllocateMSI(
    mut pExpr: *mut Fts3Expr,
    mut _iPhrase: ::core::ffi::c_int,
    mut pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3Table = pCtx as *mut Fts3Table;
    if (*pExpr).aMI.is_null() {
        (*pExpr).aMI = sqlite3_malloc64(
            (((*pTab).nColumn * 3 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<u32_0>() as usize)
                as sqlite3_uint64,
        ) as *mut u32_0;
        if (*pExpr).aMI.is_null() {
            return SQLITE_NOMEM;
        }
    }
    memset(
        (*pExpr).aMI as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (((*pTab).nColumn * 3 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<u32_0>() as size_t),
    );
    return SQLITE_OK;
}
unsafe extern "C" fn fts3EvalGatherStats(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*pExpr).aMI.is_null() {
        let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
        let mut pRoot: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
        let mut iPrevId: sqlite3_int64 = (*pCsr).iPrevId;
        let mut iDocid: sqlite3_int64 = 0;
        let mut bEof: u8_0 = 0;
        pRoot = pExpr;
        while !(*pRoot).pParent.is_null()
            && ((*(*pRoot).pParent).eType == FTSQUERY_NEAR
                || (*pRoot).bDeferred as ::core::ffi::c_int != 0)
        {
            pRoot = (*pRoot).pParent;
        }
        iDocid = (*pRoot).iDocid;
        bEof = (*pRoot).bEof;
        rc = sqlite3Fts3ExprIterate(
            pRoot,
            Some(
                fts3AllocateMSI
                    as unsafe extern "C" fn(
                        *mut Fts3Expr,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            pTab as *mut ::core::ffi::c_void,
        );
        if rc != SQLITE_OK {
            return rc;
        }
        fts3EvalRestart(pCsr, pRoot, &raw mut rc);
        while (*pCsr).isEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int && rc == SQLITE_OK {
            loop {
                if (*pCsr).isRequireSeek as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    sqlite3_reset((*pCsr).pStmt);
                }
                fts3EvalNextRow(pCsr, pRoot, &raw mut rc);
                (*pCsr).isEof = (*pRoot).bEof;
                (*pCsr).isRequireSeek = 1 as u8_0;
                (*pCsr).isMatchinfoNeeded = 1 as ::core::ffi::c_int;
                (*pCsr).iPrevId = (*pRoot).iDocid;
                if !((*pCsr).isEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*pRoot).eType == FTSQUERY_NEAR
                    && sqlite3Fts3EvalTestDeferred(pCsr, &raw mut rc) != 0)
                {
                    break;
                }
            }
            if rc == SQLITE_OK && (*pCsr).isEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                fts3EvalUpdateCounts(pRoot, (*pTab).nColumn);
            }
        }
        (*pCsr).isEof = 0 as u8_0;
        (*pCsr).iPrevId = iPrevId;
        if bEof != 0 {
            (*pRoot).bEof = bEof;
        } else {
            fts3EvalRestart(pCsr, pRoot, &raw mut rc);
            loop {
                fts3EvalNextRow(pCsr, pRoot, &raw mut rc);
                if (*pRoot).bEof != 0 {
                    rc = FTS_CORRUPT_VTAB;
                }
                if !((*pRoot).iDocid != iDocid && rc == SQLITE_OK) {
                    break;
                }
            }
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3EvalPhraseStats(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
    mut aiOut: *mut u32_0,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut iCol: ::core::ffi::c_int = 0;
    if (*pExpr).bDeferred as ::core::ffi::c_int != 0 && (*(*pExpr).pParent).eType != FTSQUERY_NEAR {
        iCol = 0 as ::core::ffi::c_int;
        while iCol < (*pTab).nColumn {
            *aiOut.offset((iCol * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (*pCsr).nDoc as u32_0;
            *aiOut.offset((iCol * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                (*pCsr).nDoc as u32_0;
            iCol += 1;
        }
    } else {
        rc = fts3EvalGatherStats(pCsr, pExpr);
        if rc == SQLITE_OK {
            iCol = 0 as ::core::ffi::c_int;
            while iCol < (*pTab).nColumn {
                *aiOut
                    .offset((iCol * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                    *(*pExpr).aMI.offset(
                        (iCol * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                    );
                *aiOut
                    .offset((iCol * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                    *(*pExpr).aMI.offset(
                        (iCol * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                    );
                iCol += 1;
            }
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3EvalPhrasePoslist(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
    mut iCol: ::core::ffi::c_int,
    mut ppOut: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pPhrase: *mut Fts3Phrase = (*pExpr).pPhrase;
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut pIter: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iThis: ::core::ffi::c_int = 0;
    let mut iDocid: sqlite3_int64 = 0;
    *ppOut = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*pPhrase).iColumn < (*pTab).nColumn && (*pPhrase).iColumn != iCol {
        return SQLITE_OK;
    }
    iDocid = (*pExpr).iDocid;
    pIter = (*pPhrase).doclist.pList;
    if iDocid != (*pCsr).iPrevId || (*pExpr).bEof as ::core::ffi::c_int != 0 {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut bDescDoclist: ::core::ffi::c_int = (*pTab).bDescIdx as ::core::ffi::c_int;
        let mut bOr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bTreeEof: u8_0 = 0 as u8_0;
        let mut p: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
        let mut pNear: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
        let mut pRun: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
        let mut bMatch: ::core::ffi::c_int = 0;
        pNear = pExpr;
        p = (*pExpr).pParent;
        while !p.is_null() {
            if (*p).eType == FTSQUERY_OR {
                bOr = 1 as ::core::ffi::c_int;
            }
            if (*p).eType == FTSQUERY_NEAR {
                pNear = p;
            }
            if (*p).bEof != 0 {
                bTreeEof = 1 as u8_0;
            }
            p = (*p).pParent;
        }
        if bOr == 0 as ::core::ffi::c_int {
            return SQLITE_OK;
        }
        pRun = pNear;
        while (*pRun).bDeferred != 0 {
            pRun = (*pRun).pParent;
        }
        if (*pPhrase).bIncr != 0 {
            let mut bEofSave: ::core::ffi::c_int = (*pRun).bEof as ::core::ffi::c_int;
            fts3EvalRestart(pCsr, pRun, &raw mut rc);
            while rc == SQLITE_OK && (*pRun).bEof == 0 {
                fts3EvalNextRow(pCsr, pRun, &raw mut rc);
                if bEofSave == 0 as ::core::ffi::c_int && (*pRun).iDocid == iDocid {
                    break;
                }
            }
            if rc == SQLITE_OK && (*pRun).bEof as ::core::ffi::c_int != bEofSave {
                rc = FTS_CORRUPT_VTAB;
            }
        }
        if bTreeEof != 0 {
            while rc == SQLITE_OK && (*pRun).bEof == 0 {
                fts3EvalNextRow(pCsr, pRun, &raw mut rc);
            }
        }
        if rc != SQLITE_OK {
            return rc;
        }
        bMatch = 1 as ::core::ffi::c_int;
        p = pNear;
        while !p.is_null() {
            let mut bEof: u8_0 = 0 as u8_0;
            let mut pTest: *mut Fts3Expr = p;
            let mut pPh: *mut Fts3Phrase = ::core::ptr::null_mut::<Fts3Phrase>();
            if (*pTest).eType == FTSQUERY_NEAR {
                pTest = (*pTest).pRight;
            }
            pPh = (*pTest).pPhrase;
            pIter = (*pPh).pOrPoslist;
            iDocid = (*pPh).iOrDocid as sqlite3_int64;
            if (*pCsr).bDesc as ::core::ffi::c_int == bDescDoclist {
                bEof = ((*pPh).doclist.nAll == 0
                    || pIter >= (*pPh).doclist.aAll.offset((*pPh).doclist.nAll as isize))
                    as ::core::ffi::c_int as u8_0;
                while (pIter.is_null()
                    || (if bDescDoclist != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) * (if iDocid > (*pCsr).iPrevId {
                        1 as ::core::ffi::c_int
                    } else {
                        if iDocid == (*pCsr).iPrevId {
                            0 as ::core::ffi::c_int
                        } else {
                            -(1 as ::core::ffi::c_int)
                        }
                    }) < 0 as ::core::ffi::c_int)
                    && bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    sqlite3Fts3DoclistNext(
                        bDescDoclist,
                        (*pPh).doclist.aAll,
                        (*pPh).doclist.nAll,
                        &raw mut pIter,
                        &raw mut iDocid,
                        &raw mut bEof,
                    );
                }
            } else {
                bEof = ((*pPh).doclist.nAll == 0
                    || !pIter.is_null() && pIter <= (*pPh).doclist.aAll)
                    as ::core::ffi::c_int as u8_0;
                while (pIter.is_null()
                    || (if bDescDoclist != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) * (if iDocid > (*pCsr).iPrevId {
                        1 as ::core::ffi::c_int
                    } else {
                        if iDocid == (*pCsr).iPrevId {
                            0 as ::core::ffi::c_int
                        } else {
                            -(1 as ::core::ffi::c_int)
                        }
                    }) > 0 as ::core::ffi::c_int)
                    && bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    let mut dummy: ::core::ffi::c_int = 0;
                    sqlite3Fts3DoclistPrev(
                        bDescDoclist,
                        (*pPh).doclist.aAll,
                        (*pPh).doclist.nAll,
                        &raw mut pIter,
                        &raw mut iDocid,
                        &raw mut dummy,
                        &raw mut bEof,
                    );
                }
            }
            (*pPh).pOrPoslist = pIter;
            (*pPh).iOrDocid = iDocid as i64_0;
            if bEof as ::core::ffi::c_int != 0 || iDocid != (*pCsr).iPrevId {
                bMatch = 0 as ::core::ffi::c_int;
            }
            p = (*p).pLeft;
        }
        if bMatch != 0 {
            pIter = (*pPhrase).pOrPoslist;
        } else {
            pIter = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    if pIter.is_null() {
        return SQLITE_OK;
    }
    if *pIter as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int {
        pIter = pIter.offset(1);
        pIter = pIter.offset(
            (if *(pIter as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                sqlite3Fts3GetVarint32(pIter, &raw mut iThis)
            } else {
                iThis = *(pIter as *mut u8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
    } else {
        iThis = 0 as ::core::ffi::c_int;
    }
    while iThis < iCol {
        fts3ColumnlistCopy(
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            &raw mut pIter,
        );
        if *pIter as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return SQLITE_OK;
        }
        pIter = pIter.offset(1);
        pIter = pIter.offset(
            (if *(pIter as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                sqlite3Fts3GetVarint32(pIter, &raw mut iThis)
            } else {
                iThis = *(pIter as *mut u8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
    }
    if *pIter as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        pIter = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *ppOut = if iCol == iThis {
        pIter
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3EvalPhraseCleanup(mut pPhrase: *mut Fts3Phrase) {
    if !pPhrase.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        sqlite3_free((*pPhrase).doclist.aAll as *mut ::core::ffi::c_void);
        fts3EvalInvalidatePoslist(pPhrase);
        memset(
            &raw mut (*pPhrase).doclist as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Fts3Doclist>() as size_t,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*pPhrase).nToken {
            fts3SegReaderCursorFree(
                (*(&raw mut (*pPhrase).aToken as *mut Fts3PhraseToken).offset(i as isize)).pSegcsr,
            );
            let ref mut fresh31 =
                (*(&raw mut (*pPhrase).aToken as *mut Fts3PhraseToken).offset(i as isize)).pSegcsr;
            *fresh31 = ::core::ptr::null_mut::<Fts3MultiSegReader>();
            i += 1;
        }
    }
}
