use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    fn sqlite3_close(_: *mut sqlite3) -> ::core::ffi::c_int;
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
    fn sqlite3_db_config(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_vmprintf(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_set_authorizer(
        _: *mut sqlite3,
        xAuth: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pUserData: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_open(
        filename: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
    ) -> ::core::ffi::c_int;
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const ::core::ffi::c_char;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    fn sqlite3_column_value(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *mut sqlite3_value;
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
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
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
    fn sqlite3_collation_needed(
        _: *mut sqlite3,
        _: *mut ::core::ffi::c_void,
        _: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut sqlite3,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
            ) -> (),
        >,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3_keyword_check(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vtab_collation(
        _: *mut sqlite3_index_info,
        _: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
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
pub struct sqlite3expert {
    pub iSample: ::core::ffi::c_int,
    pub db: *mut sqlite3,
    pub dbm: *mut sqlite3,
    pub dbv: *mut sqlite3,
    pub pTable: *mut IdxTable,
    pub pScan: *mut IdxScan,
    pub pWrite: *mut IdxWrite,
    pub pStatement: *mut IdxStatement,
    pub bRun: ::core::ffi::c_int,
    pub pzErrmsg: *mut *mut ::core::ffi::c_char,
    pub rc: ::core::ffi::c_int,
    pub hIdx: IdxHash,
    pub zCandidates: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxHash {
    pub pFirst: *mut IdxHashEntry,
    pub aHash: [*mut IdxHashEntry; 1023],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxHashEntry {
    pub zKey: *mut ::core::ffi::c_char,
    pub zVal: *mut ::core::ffi::c_char,
    pub zVal2: *mut ::core::ffi::c_char,
    pub pHashNext: *mut IdxHashEntry,
    pub pNext: *mut IdxHashEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxStatement {
    pub iId: ::core::ffi::c_int,
    pub zSql: *mut ::core::ffi::c_char,
    pub zIdx: *mut ::core::ffi::c_char,
    pub zEQP: *mut ::core::ffi::c_char,
    pub pNext: *mut IdxStatement,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxWrite {
    pub pTab: *mut IdxTable,
    pub eOp: ::core::ffi::c_int,
    pub pNext: *mut IdxWrite,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxTable {
    pub nCol: ::core::ffi::c_int,
    pub zName: *mut ::core::ffi::c_char,
    pub aCol: *mut IdxColumn,
    pub pNext: *mut IdxTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxColumn {
    pub zName: *mut ::core::ffi::c_char,
    pub zColl: *mut ::core::ffi::c_char,
    pub iPk: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxScan {
    pub pTab: *mut IdxTable,
    pub iDb: ::core::ffi::c_int,
    pub covering: i64_0,
    pub pOrder: *mut IdxConstraint,
    pub pEq: *mut IdxConstraint,
    pub pRange: *mut IdxConstraint,
    pub pNextScan: *mut IdxScan,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxConstraint {
    pub zColl: *mut ::core::ffi::c_char,
    pub bRange: ::core::ffi::c_int,
    pub iCol: ::core::ffi::c_int,
    pub bFlag: ::core::ffi::c_int,
    pub bDesc: ::core::ffi::c_int,
    pub pNext: *mut IdxConstraint,
    pub pLink: *mut IdxConstraint,
}
pub type i64_0 = sqlite3_int64;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExpertCsr {
    pub base: sqlite3_vtab_cursor,
    pub pData: *mut sqlite3_stmt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExpertVtab {
    pub base: sqlite3_vtab,
    pub pTab: *mut IdxTable,
    pub pExpert: *mut sqlite3expert,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxRemCtx {
    pub nSlot: ::core::ffi::c_int,
    pub aSlot: [IdxRemSlot; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxRemSlot {
    pub eType: ::core::ffi::c_int,
    pub iVal: i64_0,
    pub rVal: ::core::ffi::c_double,
    pub nByte: i64_0,
    pub n: i64_0,
    pub z: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxSampleCtx {
    pub iTarget: ::core::ffi::c_int,
    pub target: ::core::ffi::c_double,
    pub nRow: ::core::ffi::c_double,
    pub nRet: ::core::ffi::c_double,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_BUSY_TIMEOUT: ::core::ffi::c_int = SQLITE_BUSY
    | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_TRIGGER_EQP: ::core::ffi::c_int = 1008 as ::core::ffi::c_int;
pub const SQLITE_DELETE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_INSERT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_UPDATE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_UTF16BE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_DIRECTONLY: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_GT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_GE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const EXPERT_CONFIG_SAMPLE: ::core::ffi::c_int = 1;
pub const EXPERT_REPORT_SQL: ::core::ffi::c_int = 1;
pub const EXPERT_REPORT_INDEXES: ::core::ffi::c_int = 2;
pub const EXPERT_REPORT_PLAN: ::core::ffi::c_int = 3;
pub const EXPERT_REPORT_CANDIDATES: ::core::ffi::c_int = 4;
pub const UNIQUE_TABLE_NAME: [::core::ffi::c_char; 38] = unsafe {
    ::core::mem::transmute::<
        [u8; 38],
        [::core::ffi::c_char; 38],
    >(*b"t592690916721053953805701627921227776\0")
};
pub const IDX_HASH_SIZE: ::core::ffi::c_int = 1023 as ::core::ffi::c_int;
unsafe extern "C" fn idxMalloc(
    mut pRc: *mut ::core::ffi::c_int,
    mut nByte: i64_0,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pRet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        pRet = sqlite3_malloc64(nByte as sqlite3_uint64);
        if !pRet.is_null() {
            memset(pRet, 0 as ::core::ffi::c_int, nByte as size_t);
        } else {
            *pRc = SQLITE_NOMEM;
        }
        return pRet;
    }
}
unsafe extern "C" fn idxHashInit(mut pHash: *mut IdxHash) {
    unsafe {
        memset(
            pHash as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<IdxHash>() as size_t,
        );
    }
}
unsafe extern "C" fn idxHashClear(mut pHash: *mut IdxHash) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < IDX_HASH_SIZE {
            let mut pEntry: *mut IdxHashEntry = ::core::ptr::null_mut::<IdxHashEntry>();
            let mut pNext: *mut IdxHashEntry = ::core::ptr::null_mut::<IdxHashEntry>();
            pEntry = (*pHash).aHash[i as usize];
            while !pEntry.is_null() {
                pNext = (*pEntry).pHashNext;
                sqlite3_free((*pEntry).zVal2 as *mut ::core::ffi::c_void);
                sqlite3_free(pEntry as *mut ::core::ffi::c_void);
                pEntry = pNext;
            }
            i += 1;
        }
        memset(
            pHash as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<IdxHash>() as size_t,
        );
    }
}
unsafe extern "C" fn idxHashString(
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            ret = ret
                .wrapping_add(
                    (ret << 3 as ::core::ffi::c_int)
                        .wrapping_add(
                            *z.offset(i as isize) as ::core::ffi::c_uchar
                                as ::core::ffi::c_uint,
                        ),
                );
            i += 1;
        }
        return ret.wrapping_rem(IDX_HASH_SIZE as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn idxHashAdd(
    mut pRc: *mut ::core::ffi::c_int,
    mut pHash: *mut IdxHash,
    mut zKey: *const ::core::ffi::c_char,
    mut zVal: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nKey: ::core::ffi::c_int = strlen(zKey) as ::core::ffi::c_int;
        let mut iHash: ::core::ffi::c_int = idxHashString(zKey, nKey);
        let mut nVal: ::core::ffi::c_int = if !zVal.is_null() {
            strlen(zVal) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut pEntry: *mut IdxHashEntry = ::core::ptr::null_mut::<IdxHashEntry>();
        pEntry = (*pHash).aHash[iHash as usize];
        while !pEntry.is_null() {
            if strlen((*pEntry).zKey) as ::core::ffi::c_int == nKey
                && 0 as ::core::ffi::c_int
                    == memcmp(
                        (*pEntry).zKey as *const ::core::ffi::c_void,
                        zKey as *const ::core::ffi::c_void,
                        nKey as size_t,
                    )
            {
                return 1 as ::core::ffi::c_int;
            }
            pEntry = (*pEntry).pHashNext;
        }
        pEntry = idxMalloc(
            pRc,
            (::core::mem::size_of::<IdxHashEntry>() as ::core::ffi::c_ulonglong)
                .wrapping_add(nKey as i64_0 as ::core::ffi::c_ulonglong)
                .wrapping_add(1 as ::core::ffi::c_ulonglong)
                .wrapping_add(nVal as i64_0 as ::core::ffi::c_ulonglong)
                .wrapping_add(1 as ::core::ffi::c_ulonglong) as i64_0,
        ) as *mut IdxHashEntry;
        if !pEntry.is_null() {
            (*pEntry).zKey = pEntry.offset(1 as ::core::ffi::c_int as isize)
                as *mut IdxHashEntry as *mut ::core::ffi::c_char;
            memcpy(
                (*pEntry).zKey as *mut ::core::ffi::c_void,
                zKey as *const ::core::ffi::c_void,
                nKey as size_t,
            );
            if !zVal.is_null() {
                (*pEntry).zVal = (*pEntry)
                    .zKey
                    .offset((nKey + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char;
                memcpy(
                    (*pEntry).zVal as *mut ::core::ffi::c_void,
                    zVal as *const ::core::ffi::c_void,
                    nVal as size_t,
                );
            }
            (*pEntry).pHashNext = (*pHash).aHash[iHash as usize];
            (*pHash).aHash[iHash as usize] = pEntry;
            (*pEntry).pNext = (*pHash).pFirst;
            (*pHash).pFirst = pEntry;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn idxHashFind(
    mut pHash: *mut IdxHash,
    mut zKey: *const ::core::ffi::c_char,
    mut nKey: ::core::ffi::c_int,
) -> *mut IdxHashEntry {
    unsafe {
        let mut iHash: ::core::ffi::c_int = 0;
        let mut pEntry: *mut IdxHashEntry = ::core::ptr::null_mut::<IdxHashEntry>();
        if nKey < 0 as ::core::ffi::c_int {
            nKey = strlen(zKey) as ::core::ffi::c_int;
        }
        iHash = idxHashString(zKey, nKey);
        pEntry = (*pHash).aHash[iHash as usize];
        while !pEntry.is_null() {
            if strlen((*pEntry).zKey) as ::core::ffi::c_int == nKey
                && 0 as ::core::ffi::c_int
                    == memcmp(
                        (*pEntry).zKey as *const ::core::ffi::c_void,
                        zKey as *const ::core::ffi::c_void,
                        nKey as size_t,
                    )
            {
                return pEntry;
            }
            pEntry = (*pEntry).pHashNext;
        }
        return ::core::ptr::null_mut::<IdxHashEntry>();
    }
}
unsafe extern "C" fn idxHashSearch(
    mut pHash: *mut IdxHash,
    mut zKey: *const ::core::ffi::c_char,
    mut nKey: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut pEntry: *mut IdxHashEntry = idxHashFind(pHash, zKey, nKey);
        if !pEntry.is_null() {
            return (*pEntry).zVal;
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn idxNewConstraint(
    mut pRc: *mut ::core::ffi::c_int,
    mut zColl: *const ::core::ffi::c_char,
) -> *mut IdxConstraint {
    unsafe {
        let mut pNew: *mut IdxConstraint = ::core::ptr::null_mut::<IdxConstraint>();
        let mut nColl: ::core::ffi::c_int = strlen(zColl) as ::core::ffi::c_int;
        pNew = idxMalloc(
            pRc,
            (::core::mem::size_of::<IdxConstraint>() as usize)
                .wrapping_mul(nColl as usize)
                .wrapping_add(1 as usize) as i64_0,
        ) as *mut IdxConstraint;
        if !pNew.is_null() {
            (*pNew).zColl = pNew.offset(1 as ::core::ffi::c_int as isize)
                as *mut IdxConstraint as *mut ::core::ffi::c_char;
            memcpy(
                (*pNew).zColl as *mut ::core::ffi::c_void,
                zColl as *const ::core::ffi::c_void,
                (nColl + 1 as ::core::ffi::c_int) as size_t,
            );
        }
        return pNew;
    }
}
unsafe extern "C" fn idxDatabaseError(
    mut db: *mut sqlite3,
    mut pzErrmsg: *mut *mut ::core::ffi::c_char,
) {
    unsafe {
        *pzErrmsg = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
}
unsafe extern "C" fn idxPrepareStmt(
    mut db: *mut sqlite3,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzErrmsg: *mut *mut ::core::ffi::c_char,
    mut zSql: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
            db,
            zSql,
            -1 as ::core::ffi::c_int,
            ppStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            *ppStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            idxDatabaseError(db, pzErrmsg);
        }
        return rc;
    }
}
unsafe extern "C" fn idxPrintfPrepareStmt(
    mut db: *mut sqlite3,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzErrmsg: *mut *mut ::core::ffi::c_char,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ap = c2rust_args.clone();
        zSql = sqlite3_vmprintf(zFmt, ap);
        if zSql.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            rc = idxPrepareStmt(db, ppStmt, pzErrmsg, zSql);
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn expertDequote(
    mut zIn: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut n: i64_0 = strlen(zIn) as ::core::ffi::c_int as i64_0;
        let mut zRet: *mut ::core::ffi::c_char = sqlite3_malloc64(n as sqlite3_uint64)
            as *mut ::core::ffi::c_char;
        if !zRet.is_null() {
            let mut iOut: i64_0 = 0 as i64_0;
            let mut iIn: i64_0 = 0 as i64_0;
            iIn = 1 as i64_0;
            while iIn < n as ::core::ffi::c_longlong - 1 as ::core::ffi::c_longlong {
                if *zIn.offset(iIn as isize) as ::core::ffi::c_int == '\'' as i32 {
                    iIn += 1;
                }
                let c2rust_fresh0 = iOut;
                iOut = iOut + 1;
                *zRet.offset(c2rust_fresh0 as isize) = *zIn.offset(iIn as isize);
                iIn += 1;
            }
            *zRet.offset(iOut as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
        return zRet;
    }
}
unsafe extern "C" fn expertConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pExpert: *mut sqlite3expert = pAux as *mut sqlite3expert;
        let mut p: *mut ExpertVtab = ::core::ptr::null_mut::<ExpertVtab>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 4 as ::core::ffi::c_int {
            *pzErr = sqlite3_mprintf(
                b"internal error!\0".as_ptr() as *const ::core::ffi::c_char,
            );
            rc = SQLITE_ERROR;
        } else {
            let mut zCreateTable: *mut ::core::ffi::c_char = expertDequote(
                *argv.offset(3 as ::core::ffi::c_int as isize),
            );
            if !zCreateTable.is_null() {
                rc = sqlite3_declare_vtab(db, zCreateTable);
                if rc == SQLITE_OK {
                    p = idxMalloc(
                        &raw mut rc,
                        ::core::mem::size_of::<ExpertVtab>() as i64_0,
                    ) as *mut ExpertVtab;
                }
                if rc == SQLITE_OK {
                    (*p).pExpert = pExpert;
                    (*p).pTab = (*pExpert).pTable;
                }
                sqlite3_free(zCreateTable as *mut ::core::ffi::c_void);
            } else {
                rc = SQLITE_NOMEM;
            }
        }
        *ppVtab = p as *mut sqlite3_vtab;
        return rc;
    }
}
unsafe extern "C" fn expertDisconnect(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ExpertVtab = pVtab as *mut ExpertVtab;
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn expertBestIndex(
    mut pVtab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ExpertVtab = pVtab as *mut ExpertVtab;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pScan: *mut IdxScan = ::core::ptr::null_mut::<IdxScan>();
        let opmask: ::core::ffi::c_int = SQLITE_INDEX_CONSTRAINT_EQ
            | SQLITE_INDEX_CONSTRAINT_GT | SQLITE_INDEX_CONSTRAINT_LT
            | SQLITE_INDEX_CONSTRAINT_GE | SQLITE_INDEX_CONSTRAINT_LE;
        pScan = idxMalloc(&raw mut rc, ::core::mem::size_of::<IdxScan>() as i64_0)
            as *mut IdxScan;
        if !pScan.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            (*pScan).pTab = (*p).pTab;
            (*pScan).pNextScan = (*(*p).pExpert).pScan;
            (*(*p).pExpert).pScan = pScan;
            i = 0 as ::core::ffi::c_int;
            while i < (*pIdxInfo).nConstraint {
                let mut pCons: *mut sqlite3_index_constraint = (*pIdxInfo)
                    .aConstraint
                    .offset(i as isize) as *mut sqlite3_index_constraint;
                if (*pCons).usable as ::core::ffi::c_int != 0
                    && (*pCons).iColumn >= 0 as ::core::ffi::c_int
                    && (*(*(*p).pTab).aCol.offset((*pCons).iColumn as isize)).iPk
                        == 0 as ::core::ffi::c_int
                    && (*pCons).op as ::core::ffi::c_int & opmask != 0
                {
                    let mut pNew: *mut IdxConstraint = ::core::ptr::null_mut::<
                        IdxConstraint,
                    >();
                    let mut zColl: *const ::core::ffi::c_char = sqlite3_vtab_collation(
                        pIdxInfo,
                        i,
                    );
                    pNew = idxNewConstraint(&raw mut rc, zColl);
                    if !pNew.is_null() {
                        (*pNew).iCol = (*pCons).iColumn;
                        if (*pCons).op as ::core::ffi::c_int
                            == SQLITE_INDEX_CONSTRAINT_EQ
                        {
                            (*pNew).pNext = (*pScan).pEq;
                            (*pScan).pEq = pNew;
                        } else {
                            (*pNew).bRange = 1 as ::core::ffi::c_int;
                            (*pNew).pNext = (*pScan).pRange;
                            (*pScan).pRange = pNew;
                        }
                    }
                    n += 1;
                    (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).argvIndex = n;
                }
                i += 1;
            }
            i = (*pIdxInfo).nOrderBy - 1 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                let mut iCol: ::core::ffi::c_int = (*(*pIdxInfo)
                    .aOrderBy
                    .offset(i as isize))
                    .iColumn;
                if iCol >= 0 as ::core::ffi::c_int {
                    let mut pNew_0: *mut IdxConstraint = idxNewConstraint(
                        &raw mut rc,
                        (*(*(*p).pTab).aCol.offset(iCol as isize)).zColl,
                    );
                    if !pNew_0.is_null() {
                        (*pNew_0).iCol = iCol;
                        (*pNew_0).bDesc = (*(*pIdxInfo).aOrderBy.offset(i as isize)).desc
                            as ::core::ffi::c_int;
                        (*pNew_0).pNext = (*pScan).pOrder;
                        (*pNew_0).pLink = (*pScan).pOrder;
                        (*pScan).pOrder = pNew_0;
                        n += 1;
                    }
                }
                i -= 1;
            }
        }
        (*pIdxInfo).estimatedCost = 1000000.0f64
            / (n + 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        return rc;
    }
}
unsafe extern "C" fn expertUpdate(
    mut pVtab: *mut sqlite3_vtab,
    mut nData: ::core::ffi::c_int,
    mut azData: *mut *mut sqlite3_value,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn expertOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pCsr: *mut ExpertCsr = ::core::ptr::null_mut::<ExpertCsr>();
        pCsr = idxMalloc(&raw mut rc, ::core::mem::size_of::<ExpertCsr>() as i64_0)
            as *mut ExpertCsr;
        *ppCursor = pCsr as *mut sqlite3_vtab_cursor;
        return rc;
    }
}
unsafe extern "C" fn expertClose(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut ExpertCsr = cur as *mut ExpertCsr;
        sqlite3_finalize((*pCsr).pData);
        sqlite3_free(pCsr as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn expertEof(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut ExpertCsr = cur as *mut ExpertCsr;
        return (*pCsr).pData.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn expertNext(
    mut cur: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut ExpertCsr = cur as *mut ExpertCsr;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        rc = sqlite3_step((*pCsr).pData);
        if rc != SQLITE_ROW {
            rc = sqlite3_finalize((*pCsr).pData);
            (*pCsr).pData = ::core::ptr::null_mut::<sqlite3_stmt>();
        } else {
            rc = SQLITE_OK;
        }
        return rc;
    }
}
unsafe extern "C" fn expertRowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        *pRowid = 0 as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn expertColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut ExpertCsr = cur as *mut ExpertCsr;
        let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<sqlite3_value>();
        pVal = sqlite3_column_value((*pCsr).pData, i);
        if !pVal.is_null() {
            sqlite3_result_value(ctx, pVal);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn expertFilter(
    mut cur: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut ExpertCsr = cur as *mut ExpertCsr;
        let mut pVtab: *mut ExpertVtab = (*cur).pVtab as *mut ExpertVtab;
        let mut pExpert: *mut sqlite3expert = (*pVtab).pExpert;
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3_finalize((*pCsr).pData);
        (*pCsr).pData = ::core::ptr::null_mut::<sqlite3_stmt>();
        if rc == SQLITE_OK {
            rc = idxPrintfPrepareStmt(
                (*pExpert).db,
                &raw mut (*pCsr).pData,
                &raw mut (*pVtab).base.zErrMsg,
                b"SELECT * FROM main.%Q WHERE sqlite_expert_sample()\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*(*pVtab).pTab).zName,
            );
        }
        if rc == SQLITE_OK {
            rc = expertNext(cur);
        }
        return rc;
    }
}
unsafe extern "C" fn idxRegisterVtab(mut p: *mut sqlite3expert) -> ::core::ffi::c_int {
    unsafe {
        static mut expertModule: sqlite3_module = unsafe {
            sqlite3_module {
                iVersion: 2 as ::core::ffi::c_int,
                xCreate: Some(
                    expertConnect
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
                    expertConnect
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
                    expertBestIndex
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut sqlite3_index_info,
                        ) -> ::core::ffi::c_int,
                ),
                xDisconnect: Some(
                    expertDisconnect
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xDestroy: Some(
                    expertDisconnect
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xOpen: Some(
                    expertOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xClose: Some(
                    expertClose
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xFilter: Some(
                    expertFilter
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> ::core::ffi::c_int,
                ),
                xNext: Some(
                    expertNext
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xEof: Some(
                    expertEof
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xColumn: Some(
                    expertColumn
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xRowid: Some(
                    expertRowid
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            *mut sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xUpdate: Some(
                    expertUpdate
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                            *mut sqlite_int64,
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
        return sqlite3_create_module(
            (*p).dbv,
            b"expert\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut expertModule,
            p as *mut ::core::ffi::c_void,
        );
    }
}
unsafe extern "C" fn idxFinalize(
    mut pRc: *mut ::core::ffi::c_int,
    mut pStmt: *mut sqlite3_stmt,
) {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_finalize(pStmt);
        if *pRc == SQLITE_OK {
            *pRc = rc;
        }
    }
}
unsafe extern "C" fn idxGetTableInfo(
    mut db: *mut sqlite3,
    mut zTab: *const ::core::ffi::c_char,
    mut ppOut: *mut *mut IdxTable,
    mut pzErrmsg: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p1: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nTab: ::core::ffi::c_int = 0;
        let mut nByte: i64_0 = 0;
        let mut pNew: *mut IdxTable = ::core::ptr::null_mut::<IdxTable>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut rc2: ::core::ffi::c_int = 0;
        let mut pCsr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nPk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        *ppOut = ::core::ptr::null_mut::<IdxTable>();
        if zTab.is_null() {
            return SQLITE_ERROR;
        }
        nTab = strlen(zTab) as ::core::ffi::c_int;
        nByte = (::core::mem::size_of::<IdxTable>() as usize)
            .wrapping_add(nTab as usize)
            .wrapping_add(1 as usize) as i64_0;
        rc = idxPrintfPrepareStmt(
            db,
            &raw mut p1,
            pzErrmsg,
            b"PRAGMA table_xinfo=%Q\0".as_ptr() as *const ::core::ffi::c_char,
            zTab,
        );
        while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(p1) {
            let mut zCol: *const ::core::ffi::c_char = sqlite3_column_text(
                p1,
                1 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            let mut zColSeq: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            if zCol.is_null() {
                rc = SQLITE_ERROR;
                break;
            } else {
                nByte
                    += (1 as ::core::ffi::c_int + strlen(zCol) as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong;
                rc = sqlite3_table_column_metadata(
                    db,
                    b"main\0".as_ptr() as *const ::core::ffi::c_char,
                    zTab,
                    zCol,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    &raw mut zColSeq,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if zColSeq.is_null() {
                    zColSeq = b"binary\0".as_ptr() as *const ::core::ffi::c_char;
                }
                nByte
                    += (1 as ::core::ffi::c_int + strlen(zColSeq) as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong;
                nCol += 1;
                nPk
                    += (sqlite3_column_int(p1, 5 as ::core::ffi::c_int)
                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            }
        }
        rc2 = sqlite3_reset(p1);
        if rc == SQLITE_OK {
            rc = rc2;
        }
        nByte = (nByte as ::core::ffi::c_ulonglong)
            .wrapping_add(
                (::core::mem::size_of::<IdxColumn>() as usize)
                    .wrapping_mul(nCol as usize) as ::core::ffi::c_ulonglong,
            ) as i64_0 as i64_0;
        if rc == SQLITE_OK {
            pNew = idxMalloc(&raw mut rc, nByte) as *mut IdxTable;
        }
        if rc == SQLITE_OK {
            (*pNew).aCol = pNew.offset(1 as ::core::ffi::c_int as isize) as *mut IdxTable
                as *mut IdxColumn;
            (*pNew).nCol = nCol;
            pCsr = (*pNew).aCol.offset(nCol as isize) as *mut IdxColumn
                as *mut ::core::ffi::c_char;
        }
        nCol = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(p1) {
            let mut zCol_0: *const ::core::ffi::c_char = sqlite3_column_text(
                p1,
                1 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            let mut zColSeq_0: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut nCopy: ::core::ffi::c_int = 0;
            if zCol_0.is_null() {
                continue;
            }
            nCopy = strlen(zCol_0) as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            let ref mut c2rust_fresh1 = (*(*pNew).aCol.offset(nCol as isize)).zName;
            *c2rust_fresh1 = pCsr;
            (*(*pNew).aCol.offset(nCol as isize)).iPk = (sqlite3_column_int(
                p1,
                5 as ::core::ffi::c_int,
            ) == 1 as ::core::ffi::c_int && nPk == 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            memcpy(
                pCsr as *mut ::core::ffi::c_void,
                zCol_0 as *const ::core::ffi::c_void,
                nCopy as size_t,
            );
            pCsr = pCsr.offset(nCopy as isize);
            rc = sqlite3_table_column_metadata(
                db,
                b"main\0".as_ptr() as *const ::core::ffi::c_char,
                zTab,
                zCol_0,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                &raw mut zColSeq_0,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if rc == SQLITE_OK {
                if zColSeq_0.is_null() {
                    zColSeq_0 = b"binary\0".as_ptr() as *const ::core::ffi::c_char;
                }
                nCopy = strlen(zColSeq_0) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int;
                let ref mut c2rust_fresh2 = (*(*pNew).aCol.offset(nCol as isize)).zColl;
                *c2rust_fresh2 = pCsr;
                memcpy(
                    pCsr as *mut ::core::ffi::c_void,
                    zColSeq_0 as *const ::core::ffi::c_void,
                    nCopy as size_t,
                );
                pCsr = pCsr.offset(nCopy as isize);
            }
            nCol += 1;
        }
        idxFinalize(&raw mut rc, p1);
        if rc != SQLITE_OK {
            sqlite3_free(pNew as *mut ::core::ffi::c_void);
            pNew = ::core::ptr::null_mut::<IdxTable>();
        } else if !pNew.is_null() {
            (*pNew).zName = pCsr;
            if !(*pNew).zName.is_null() {
                memcpy(
                    (*pNew).zName as *mut ::core::ffi::c_void,
                    zTab as *const ::core::ffi::c_void,
                    (nTab + 1 as ::core::ffi::c_int) as size_t,
                );
            }
        }
        *ppOut = pNew;
        return rc;
    }
}
unsafe extern "C" fn idxAppendText(
    mut pRc: *mut ::core::ffi::c_int,
    mut zIn: *mut ::core::ffi::c_char,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zAppend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nIn: i64_0 = (if !zIn.is_null() {
            strlen(zIn) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as i64_0;
        let mut nAppend: i64_0 = 0 as i64_0;
        let mut ap = c2rust_args.clone();
        if *pRc == SQLITE_OK {
            zAppend = sqlite3_vmprintf(zFmt, ap);
            if !zAppend.is_null() {
                nAppend = strlen(zAppend) as ::core::ffi::c_int as i64_0;
                zRet = sqlite3_malloc64(
                    (nIn as ::core::ffi::c_longlong + nAppend as ::core::ffi::c_longlong
                        + 1 as ::core::ffi::c_longlong) as sqlite3_uint64,
                ) as *mut ::core::ffi::c_char;
            }
            if !zAppend.is_null() && !zRet.is_null() {
                if nIn != 0 {
                    memcpy(
                        zRet as *mut ::core::ffi::c_void,
                        zIn as *const ::core::ffi::c_void,
                        nIn as size_t,
                    );
                }
                memcpy(
                    zRet.offset(nIn as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    zAppend as *const ::core::ffi::c_void,
                    (nAppend as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                        as size_t,
                );
            } else {
                sqlite3_free(zRet as *mut ::core::ffi::c_void);
                zRet = ::core::ptr::null_mut::<::core::ffi::c_char>();
                *pRc = SQLITE_NOMEM;
            }
            sqlite3_free(zAppend as *mut ::core::ffi::c_void);
            sqlite3_free(zIn as *mut ::core::ffi::c_void);
        }
        return zRet;
    }
}
unsafe extern "C" fn idxIdentifierRequiresQuotes(
    mut zId: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut nId: ::core::ffi::c_int = strlen(zId) as ::core::ffi::c_int;
        if sqlite3_keyword_check(zId, nId) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while *zId.offset(i as isize) != 0 {
            if !(*zId.offset(i as isize) as ::core::ffi::c_int == '_' as i32)
                && !(*zId.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
                    && *zId.offset(i as isize) as ::core::ffi::c_int <= '9' as i32)
                && !(*zId.offset(i as isize) as ::core::ffi::c_int >= 'a' as i32
                    && *zId.offset(i as isize) as ::core::ffi::c_int <= 'z' as i32)
                && !(*zId.offset(i as isize) as ::core::ffi::c_int >= 'A' as i32
                    && *zId.offset(i as isize) as ::core::ffi::c_int <= 'Z' as i32)
            {
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn idxAppendColDefn(
    mut pRc: *mut ::core::ffi::c_int,
    mut zIn: *mut ::core::ffi::c_char,
    mut pTab: *mut IdxTable,
    mut pCons: *mut IdxConstraint,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zRet: *mut ::core::ffi::c_char = zIn;
        let mut p: *mut IdxColumn = (*pTab).aCol.offset((*pCons).iCol as isize)
            as *mut IdxColumn;
        if !zRet.is_null() {
            zRet = idxAppendText(
                pRc,
                zRet,
                b", \0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if idxIdentifierRequiresQuotes((*p).zName) != 0 {
            zRet = idxAppendText(
                pRc,
                zRet,
                b"%Q\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).zName,
            );
        } else {
            zRet = idxAppendText(
                pRc,
                zRet,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).zName,
            );
        }
        if sqlite3_stricmp((*p).zColl, (*pCons).zColl) != 0 {
            if idxIdentifierRequiresQuotes((*pCons).zColl) != 0 {
                zRet = idxAppendText(
                    pRc,
                    zRet,
                    b" COLLATE %Q\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pCons).zColl,
                );
            } else {
                zRet = idxAppendText(
                    pRc,
                    zRet,
                    b" COLLATE %s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pCons).zColl,
                );
            }
        }
        if (*pCons).bDesc != 0 {
            zRet = idxAppendText(
                pRc,
                zRet,
                b" DESC\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return zRet;
    }
}
unsafe extern "C" fn idxFindCompatible(
    mut pRc: *mut ::core::ffi::c_int,
    mut dbm: *mut sqlite3,
    mut pScan: *mut IdxScan,
    mut pEq: *mut IdxConstraint,
    mut pTail: *mut IdxConstraint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zTbl: *const ::core::ffi::c_char = (*(*pScan).pTab).zName;
        let mut pIdxList: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut pIter: *mut IdxConstraint = ::core::ptr::null_mut::<IdxConstraint>();
        let mut nEq: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0;
        pIter = pEq;
        while !pIter.is_null() {
            nEq += 1;
            pIter = (*pIter).pLink;
        }
        rc = idxPrintfPrepareStmt(
            dbm,
            &raw mut pIdxList,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            b"PRAGMA index_list=%Q\0".as_ptr() as *const ::core::ffi::c_char,
            zTbl,
        );
        while rc == SQLITE_OK && sqlite3_step(pIdxList) == SQLITE_ROW {
            let mut bMatch: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut pT: *mut IdxConstraint = pTail;
            let mut pInfo: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            let mut zIdx: *const ::core::ffi::c_char = sqlite3_column_text(
                pIdxList,
                1 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            if zIdx.is_null() {
                continue;
            }
            pIter = pEq;
            while !pIter.is_null() {
                (*pIter).bFlag = 0 as ::core::ffi::c_int;
                pIter = (*pIter).pLink;
            }
            rc = idxPrintfPrepareStmt(
                dbm,
                &raw mut pInfo,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                b"PRAGMA index_xInfo=%Q\0".as_ptr() as *const ::core::ffi::c_char,
                zIdx,
            );
            while rc == SQLITE_OK && sqlite3_step(pInfo) == SQLITE_ROW {
                let mut iIdx: ::core::ffi::c_int = sqlite3_column_int(
                    pInfo,
                    0 as ::core::ffi::c_int,
                );
                let mut iCol: ::core::ffi::c_int = sqlite3_column_int(
                    pInfo,
                    1 as ::core::ffi::c_int,
                );
                let mut zColl: *const ::core::ffi::c_char = sqlite3_column_text(
                    pInfo,
                    4 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                if iIdx < nEq {
                    pIter = pEq;
                    while !pIter.is_null() {
                        if !((*pIter).bFlag != 0) {
                            if !((*pIter).iCol != iCol) {
                                if !(sqlite3_stricmp((*pIter).zColl, zColl) != 0) {
                                    (*pIter).bFlag = 1 as ::core::ffi::c_int;
                                    break;
                                }
                            }
                        }
                        pIter = (*pIter).pLink;
                    }
                    if !pIter.is_null() {
                        continue;
                    }
                    bMatch = 0 as ::core::ffi::c_int;
                    break;
                } else {
                    if pT.is_null() {
                        continue;
                    }
                    if (*pT).iCol != iCol || sqlite3_stricmp((*pT).zColl, zColl) != 0 {
                        bMatch = 0 as ::core::ffi::c_int;
                        break;
                    } else {
                        pT = (*pT).pLink;
                    }
                }
            }
            idxFinalize(&raw mut rc, pInfo);
            if rc == SQLITE_OK && bMatch != 0 {
                sqlite3_finalize(pIdxList);
                return 1 as ::core::ffi::c_int;
            }
        }
        idxFinalize(&raw mut rc, pIdxList);
        *pRc = rc;
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn countNonzeros(
    mut pCount: *mut ::core::ffi::c_void,
    mut nc: ::core::ffi::c_int,
    mut azResults: *mut *mut ::core::ffi::c_char,
    mut azColumns: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if nc > 0 as ::core::ffi::c_int
            && (*(*azResults.offset(0 as ::core::ffi::c_int as isize))
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '0' as i32
                || *(*azResults.offset(0 as ::core::ffi::c_int as isize))
                    .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
        {
            *(pCount as *mut ::core::ffi::c_int) += 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn idxCreateFromCons(
    mut p: *mut sqlite3expert,
    mut pScan: *mut IdxScan,
    mut pEq: *mut IdxConstraint,
    mut pTail: *mut IdxConstraint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut dbm: *mut sqlite3 = (*p).dbm;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (!pEq.is_null() || !pTail.is_null())
            && 0 as ::core::ffi::c_int
                == idxFindCompatible(&raw mut rc, dbm, pScan, pEq, pTail)
        {
            let mut pTab: *mut IdxTable = (*pScan).pTab;
            let mut zCols: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut zIdx: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut pCons: *mut IdxConstraint = ::core::ptr::null_mut::<IdxConstraint>();
            let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            let mut zFmt: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            pCons = pEq;
            while !pCons.is_null() {
                zCols = idxAppendColDefn(&raw mut rc, zCols, pTab, pCons);
                pCons = (*pCons).pLink;
            }
            pCons = pTail;
            while !pCons.is_null() {
                zCols = idxAppendColDefn(&raw mut rc, zCols, pTab, pCons);
                pCons = (*pCons).pLink;
            }
            if rc == SQLITE_OK {
                let mut zTable: *const ::core::ffi::c_char = (*(*pScan).pTab).zName;
                let mut quoteTable: ::core::ffi::c_int = idxIdentifierRequiresQuotes(
                    zTable,
                );
                let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut collisions: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                loop {
                    let mut i: ::core::ffi::c_int = 0;
                    let mut zFind: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    i = 0 as ::core::ffi::c_int;
                    while *zCols.offset(i as isize) != 0 {
                        h = h
                            .wrapping_add(
                                (h << 3 as ::core::ffi::c_int)
                                    .wrapping_add(
                                        *zCols.offset(i as isize) as ::core::ffi::c_uint,
                                    ),
                            );
                        i += 1;
                    }
                    sqlite3_free(zName as *mut ::core::ffi::c_void);
                    zName = sqlite3_mprintf(
                        b"%s_idx_%08x\0".as_ptr() as *const ::core::ffi::c_char,
                        zTable,
                        h,
                    );
                    if zName.is_null() {
                        break;
                    }
                    zFmt = b"SELECT count(*) FROM sqlite_schema WHERE name=%Q AND type in ('index','table','view')\0"
                        .as_ptr() as *const ::core::ffi::c_char;
                    zFind = sqlite3_mprintf(zFmt, zName);
                    i = 0 as ::core::ffi::c_int;
                    rc = sqlite3_exec(
                        dbm,
                        zFind,
                        Some(
                            countNonzeros
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *mut *mut ::core::ffi::c_char,
                                    *mut *mut ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut i as *mut ::core::ffi::c_void,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    );
                    sqlite3_free(zFind as *mut ::core::ffi::c_void);
                    if i == 0 as ::core::ffi::c_int {
                        collisions = 0 as ::core::ffi::c_int;
                        break;
                    } else {
                        collisions += 1;
                        if !(collisions < 50 as ::core::ffi::c_int && !zName.is_null()) {
                            break;
                        }
                    }
                }
                if collisions != 0 {
                    rc = SQLITE_BUSY_TIMEOUT;
                } else if zName.is_null() {
                    rc = SQLITE_NOMEM;
                } else {
                    if quoteTable != 0 {
                        zFmt = b"CREATE INDEX \"%w\" ON \"%w\"(%s)\0".as_ptr()
                            as *const ::core::ffi::c_char;
                    } else {
                        zFmt = b"CREATE INDEX %s ON %s(%s)\0".as_ptr()
                            as *const ::core::ffi::c_char;
                    }
                    zIdx = sqlite3_mprintf(zFmt, zName, zTable, zCols);
                    if zIdx.is_null() {
                        rc = SQLITE_NOMEM;
                    } else {
                        rc = sqlite3_exec(
                            dbm,
                            zIdx,
                            None,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            (*p).pzErrmsg,
                        );
                        if rc != SQLITE_OK {
                            rc = SQLITE_BUSY_TIMEOUT;
                        } else {
                            idxHashAdd(&raw mut rc, &raw mut (*p).hIdx, zName, zIdx);
                        }
                    }
                    sqlite3_free(zName as *mut ::core::ffi::c_void);
                    sqlite3_free(zIdx as *mut ::core::ffi::c_void);
                }
            }
            sqlite3_free(zCols as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn idxFindConstraint(
    mut pList: *mut IdxConstraint,
    mut p: *mut IdxConstraint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCmp: *mut IdxConstraint = ::core::ptr::null_mut::<IdxConstraint>();
        pCmp = pList;
        while !pCmp.is_null() {
            if (*p).iCol == (*pCmp).iCol {
                return 1 as ::core::ffi::c_int;
            }
            pCmp = (*pCmp).pLink;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn idxCreateFromWhere(
    mut p: *mut sqlite3expert,
    mut pScan: *mut IdxScan,
    mut pTail: *mut IdxConstraint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p1: *mut IdxConstraint = ::core::ptr::null_mut::<IdxConstraint>();
        let mut pCon: *mut IdxConstraint = ::core::ptr::null_mut::<IdxConstraint>();
        let mut rc: ::core::ffi::c_int = 0;
        pCon = (*pScan).pEq;
        while !pCon.is_null() {
            if idxFindConstraint(p1, pCon) == 0 && idxFindConstraint(pTail, pCon) == 0 {
                (*pCon).pLink = p1;
                p1 = pCon;
            }
            pCon = (*pCon).pNext;
        }
        rc = idxCreateFromCons(p, pScan, p1, pTail);
        if pTail.is_null() {
            pCon = (*pScan).pRange;
            while rc == SQLITE_OK && !pCon.is_null() {
                if idxFindConstraint(p1, pCon) == 0
                    && idxFindConstraint(pTail, pCon) == 0
                {
                    rc = idxCreateFromCons(p, pScan, p1, pCon);
                }
                pCon = (*pCon).pNext;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn idxCreateCandidates(
    mut p: *mut sqlite3expert,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pIter: *mut IdxScan = ::core::ptr::null_mut::<IdxScan>();
        pIter = (*p).pScan;
        while !pIter.is_null() && rc == SQLITE_OK {
            rc = idxCreateFromWhere(p, pIter, ::core::ptr::null_mut::<IdxConstraint>());
            if rc == SQLITE_OK && !(*pIter).pOrder.is_null() {
                rc = idxCreateFromWhere(p, pIter, (*pIter).pOrder);
            }
            pIter = (*pIter).pNextScan;
        }
        return rc;
    }
}
unsafe extern "C" fn idxConstraintFree(mut pConstraint: *mut IdxConstraint) {
    unsafe {
        let mut pNext: *mut IdxConstraint = ::core::ptr::null_mut::<IdxConstraint>();
        let mut p: *mut IdxConstraint = ::core::ptr::null_mut::<IdxConstraint>();
        p = pConstraint;
        while !p.is_null() {
            pNext = (*p).pNext;
            sqlite3_free(p as *mut ::core::ffi::c_void);
            p = pNext;
        }
    }
}
unsafe extern "C" fn idxScanFree(mut pScan: *mut IdxScan, mut pLast: *mut IdxScan) {
    unsafe {
        let mut p: *mut IdxScan = ::core::ptr::null_mut::<IdxScan>();
        let mut pNext: *mut IdxScan = ::core::ptr::null_mut::<IdxScan>();
        p = pScan;
        while p != pLast {
            pNext = (*p).pNextScan;
            idxConstraintFree((*p).pOrder);
            idxConstraintFree((*p).pEq);
            idxConstraintFree((*p).pRange);
            sqlite3_free(p as *mut ::core::ffi::c_void);
            p = pNext;
        }
    }
}
unsafe extern "C" fn idxStatementFree(
    mut pStatement: *mut IdxStatement,
    mut pLast: *mut IdxStatement,
) {
    unsafe {
        let mut p: *mut IdxStatement = ::core::ptr::null_mut::<IdxStatement>();
        let mut pNext: *mut IdxStatement = ::core::ptr::null_mut::<IdxStatement>();
        p = pStatement;
        while p != pLast {
            pNext = (*p).pNext;
            sqlite3_free((*p).zEQP as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zIdx as *mut ::core::ffi::c_void);
            sqlite3_free(p as *mut ::core::ffi::c_void);
            p = pNext;
        }
    }
}
unsafe extern "C" fn idxTableFree(mut pTab: *mut IdxTable) {
    unsafe {
        let mut pIter: *mut IdxTable = ::core::ptr::null_mut::<IdxTable>();
        let mut pNext: *mut IdxTable = ::core::ptr::null_mut::<IdxTable>();
        pIter = pTab;
        while !pIter.is_null() {
            pNext = (*pIter).pNext;
            sqlite3_free(pIter as *mut ::core::ffi::c_void);
            pIter = pNext;
        }
    }
}
unsafe extern "C" fn idxWriteFree(mut pTab: *mut IdxWrite) {
    unsafe {
        let mut pIter: *mut IdxWrite = ::core::ptr::null_mut::<IdxWrite>();
        let mut pNext: *mut IdxWrite = ::core::ptr::null_mut::<IdxWrite>();
        pIter = pTab;
        while !pIter.is_null() {
            pNext = (*pIter).pNext;
            sqlite3_free(pIter as *mut ::core::ffi::c_void);
            pIter = pNext;
        }
    }
}
unsafe extern "C" fn idxFindIndexes(
    mut p: *mut sqlite3expert,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut IdxStatement = ::core::ptr::null_mut::<IdxStatement>();
        let mut dbm: *mut sqlite3 = (*p).dbm;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut hIdx: IdxHash = IdxHash {
            pFirst: ::core::ptr::null_mut::<IdxHashEntry>(),
            aHash: [::core::ptr::null_mut::<IdxHashEntry>(); 1023],
        };
        idxHashInit(&raw mut hIdx);
        pStmt = (*p).pStatement;
        's_13: while rc == SQLITE_OK && !pStmt.is_null() {
            let mut pEntry: *mut IdxHashEntry = ::core::ptr::null_mut::<IdxHashEntry>();
            let mut pExplain: *mut sqlite3_stmt = ::core::ptr::null_mut::<
                sqlite3_stmt,
            >();
            idxHashClear(&raw mut hIdx);
            rc = idxPrintfPrepareStmt(
                dbm,
                &raw mut pExplain,
                pzErr,
                b"EXPLAIN QUERY PLAN %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*pStmt).zSql,
            );
            while rc == SQLITE_OK && sqlite3_step(pExplain) == SQLITE_ROW {
                let mut zDetail: *const ::core::ffi::c_char = sqlite3_column_text(
                    pExplain,
                    3 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                let mut nDetail: ::core::ffi::c_int = 0;
                let mut i: ::core::ffi::c_int = 0;
                if zDetail.is_null() {
                    continue;
                }
                nDetail = strlen(zDetail) as ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while i < nDetail {
                    let mut zIdx: *const ::core::ffi::c_char = ::core::ptr::null::<
                        ::core::ffi::c_char,
                    >();
                    if (i + 13 as ::core::ffi::c_int) < nDetail
                        && memcmp(
                            zDetail.offset(i as isize) as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            b" USING INDEX \0".as_ptr() as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            13 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        zIdx = zDetail.offset((i + 13 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_char;
                    } else if (i + 22 as ::core::ffi::c_int) < nDetail
                        && memcmp(
                            zDetail.offset(i as isize) as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            b" USING COVERING INDEX \0".as_ptr()
                                as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                            22 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        zIdx = zDetail.offset((i + 22 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_char;
                    }
                    if !zIdx.is_null() {
                        let mut zSql: *const ::core::ffi::c_char = ::core::ptr::null::<
                            ::core::ffi::c_char,
                        >();
                        let mut nIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while *zIdx.offset(nIdx as isize) as ::core::ffi::c_int
                            != '\0' as i32
                            && (*zIdx.offset(nIdx as isize) as ::core::ffi::c_int
                                != ' ' as i32
                                || *zIdx.offset((nIdx + 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int != '(' as i32)
                        {
                            nIdx += 1;
                        }
                        zSql = idxHashSearch(&raw mut (*p).hIdx, zIdx, nIdx);
                        if zSql.is_null() {
                            break;
                        }
                        idxHashAdd(
                            &raw mut rc,
                            &raw mut hIdx,
                            zSql,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                        if rc != 0 {
                            break 's_13;
                        } else {
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                if *zDetail.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int != '-' as i32
                {
                    (*pStmt).zEQP = idxAppendText(
                        &raw mut rc,
                        (*pStmt).zEQP,
                        b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        zDetail,
                    );
                }
            }
            pEntry = hIdx.pFirst;
            while !pEntry.is_null() {
                (*pStmt).zIdx = idxAppendText(
                    &raw mut rc,
                    (*pStmt).zIdx,
                    b"%s;\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pEntry).zKey,
                );
                pEntry = (*pEntry).pNext;
            }
            idxFinalize(&raw mut rc, pExplain);
            pStmt = (*pStmt).pNext;
        }
        idxHashClear(&raw mut hIdx);
        return rc;
    }
}
unsafe extern "C" fn idxAuthCallback(
    mut pCtx: *mut ::core::ffi::c_void,
    mut eOp: ::core::ffi::c_int,
    mut z3: *const ::core::ffi::c_char,
    mut z4: *const ::core::ffi::c_char,
    mut zDb: *const ::core::ffi::c_char,
    mut zTrigger: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if eOp == SQLITE_INSERT || eOp == SQLITE_UPDATE || eOp == SQLITE_DELETE {
            if sqlite3_stricmp(zDb, b"main\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                let mut p: *mut sqlite3expert = pCtx as *mut sqlite3expert;
                let mut pTab: *mut IdxTable = ::core::ptr::null_mut::<IdxTable>();
                pTab = (*p).pTable;
                while !pTab.is_null() {
                    if 0 as ::core::ffi::c_int == sqlite3_stricmp(z3, (*pTab).zName) {
                        break;
                    }
                    pTab = (*pTab).pNext;
                }
                if !pTab.is_null() {
                    let mut pWrite: *mut IdxWrite = ::core::ptr::null_mut::<IdxWrite>();
                    pWrite = (*p).pWrite;
                    while !pWrite.is_null() {
                        if (*pWrite).pTab == pTab && (*pWrite).eOp == eOp {
                            break;
                        }
                        pWrite = (*pWrite).pNext;
                    }
                    if pWrite.is_null() {
                        pWrite = idxMalloc(
                            &raw mut rc,
                            ::core::mem::size_of::<IdxWrite>() as i64_0,
                        ) as *mut IdxWrite;
                        if rc == SQLITE_OK {
                            (*pWrite).pTab = pTab;
                            (*pWrite).eOp = eOp;
                            (*pWrite).pNext = (*p).pWrite;
                            (*p).pWrite = pWrite;
                        }
                    }
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn idxProcessOneTrigger(
    mut p: *mut sqlite3expert,
    mut pWrite: *mut IdxWrite,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        static mut zInt: *const ::core::ffi::c_char = UNIQUE_TABLE_NAME.as_ptr();
        static mut zDrop: *const ::core::ffi::c_char = b"DROP TABLE t592690916721053953805701627921227776\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut pTab: *mut IdxTable = (*pWrite).pTab;
        let mut zTab: *const ::core::ffi::c_char = (*pTab).zName;
        let mut zSql: *const ::core::ffi::c_char = b"SELECT 'CREATE TEMP' || substr(sql, 7) FROM sqlite_schema WHERE tbl_name = %Q AND type IN ('table', 'trigger') ORDER BY type;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut pSelect: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zWrite: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        rc = idxPrintfPrepareStmt((*p).db, &raw mut pSelect, pzErr, zSql, zTab, zTab);
        while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pSelect) {
            let mut zCreate: *const ::core::ffi::c_char = sqlite3_column_text(
                pSelect,
                0 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            if zCreate.is_null() {
                continue;
            }
            rc = sqlite3_exec(
                (*p).dbv,
                zCreate,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pzErr,
            );
        }
        idxFinalize(&raw mut rc, pSelect);
        if rc == SQLITE_OK {
            let mut z: *mut ::core::ffi::c_char = sqlite3_mprintf(
                b"ALTER TABLE temp.%Q RENAME TO %Q\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zTab,
                zInt,
            );
            if z.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                rc = sqlite3_exec(
                    (*p).dbv,
                    z,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    pzErr,
                );
                sqlite3_free(z as *mut ::core::ffi::c_void);
            }
        }
        match (*pWrite).eOp {
            SQLITE_INSERT => {
                let mut i: ::core::ffi::c_int = 0;
                zWrite = idxAppendText(
                    &raw mut rc,
                    zWrite,
                    b"INSERT INTO %Q VALUES(\0".as_ptr() as *const ::core::ffi::c_char,
                    zInt,
                );
                i = 0 as ::core::ffi::c_int;
                while i < (*pTab).nCol {
                    zWrite = idxAppendText(
                        &raw mut rc,
                        zWrite,
                        b"%s?\0".as_ptr() as *const ::core::ffi::c_char,
                        if i == 0 as ::core::ffi::c_int {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b", \0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    i += 1;
                }
                zWrite = idxAppendText(
                    &raw mut rc,
                    zWrite,
                    b")\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            SQLITE_UPDATE => {
                let mut i_0: ::core::ffi::c_int = 0;
                zWrite = idxAppendText(
                    &raw mut rc,
                    zWrite,
                    b"UPDATE %Q SET \0".as_ptr() as *const ::core::ffi::c_char,
                    zInt,
                );
                i_0 = 0 as ::core::ffi::c_int;
                while i_0 < (*pTab).nCol {
                    zWrite = idxAppendText(
                        &raw mut rc,
                        zWrite,
                        b"%s%Q=?\0".as_ptr() as *const ::core::ffi::c_char,
                        if i_0 == 0 as ::core::ffi::c_int {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b", \0".as_ptr() as *const ::core::ffi::c_char
                        },
                        (*(*pTab).aCol.offset(i_0 as isize)).zName,
                    );
                    i_0 += 1;
                }
            }
            _ => {
                if rc == SQLITE_OK {
                    zWrite = sqlite3_mprintf(
                        b"DELETE FROM %Q\0".as_ptr() as *const ::core::ffi::c_char,
                        zInt,
                    );
                    if zWrite.is_null() {
                        rc = SQLITE_NOMEM;
                    }
                }
            }
        }
        if rc == SQLITE_OK {
            let mut pX: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            rc = sqlite3_prepare_v2(
                (*p).dbv,
                zWrite,
                -1 as ::core::ffi::c_int,
                &raw mut pX,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            idxFinalize(&raw mut rc, pX);
            if rc != SQLITE_OK {
                idxDatabaseError((*p).dbv, pzErr);
            }
        }
        sqlite3_free(zWrite as *mut ::core::ffi::c_void);
        if rc == SQLITE_OK {
            rc = sqlite3_exec(
                (*p).dbv,
                zDrop,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pzErr,
            );
        }
        return rc;
    }
}
unsafe extern "C" fn idxProcessTriggers(
    mut p: *mut sqlite3expert,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pEnd: *mut IdxWrite = ::core::ptr::null_mut::<IdxWrite>();
        let mut pFirst: *mut IdxWrite = (*p).pWrite;
        while rc == SQLITE_OK && pFirst != pEnd {
            let mut pIter: *mut IdxWrite = ::core::ptr::null_mut::<IdxWrite>();
            pIter = pFirst;
            while rc == SQLITE_OK && pIter != pEnd {
                rc = idxProcessOneTrigger(p, pIter, pzErr);
                pIter = (*pIter).pNext;
            }
            pEnd = pFirst;
            pFirst = (*p).pWrite;
        }
        return rc;
    }
}
unsafe extern "C" fn expertDbContainsObject(
    mut db: *mut sqlite3,
    mut zTab: *const ::core::ffi::c_char,
    mut pbContains: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zSql: *const ::core::ffi::c_char = b"SELECT 1 FROM sqlite_schema WHERE name = ?\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut pSql: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = sqlite3_prepare_v2(
            db,
            zSql,
            -1 as ::core::ffi::c_int,
            &raw mut pSql,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_text(
                pSql,
                1 as ::core::ffi::c_int,
                zTab,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            if SQLITE_ROW == sqlite3_step(pSql) {
                ret = 1 as ::core::ffi::c_int;
            }
            rc = sqlite3_finalize(pSql);
        }
        *pbContains = ret;
        return rc;
    }
}
unsafe extern "C" fn expertSchemaSql(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        rc = sqlite3_exec(
            db,
            zSql,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            &raw mut zErr,
        );
        if rc != SQLITE_OK && !zErr.is_null() {
            let mut nErr: ::core::ffi::c_int = strlen(zErr) as ::core::ffi::c_int;
            if nErr >= 15 as ::core::ffi::c_int
                && memcmp(
                    zErr as *const ::core::ffi::c_void,
                    b"no such module:\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    15 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                sqlite3_free(zErr as *mut ::core::ffi::c_void);
                rc = SQLITE_OK;
                zErr = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
        }
        *pzErr = zErr;
        return rc;
    }
}
unsafe extern "C" fn idxCreateVtabSchema(
    mut p: *mut sqlite3expert,
    mut pzErrmsg: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = idxRegisterVtab(p);
        let mut pSchema: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = idxPrepareStmt(
            (*p).db,
            &raw mut pSchema,
            pzErrmsg,
            b"SELECT type, name, sql, 1,        substr(sql,1,14)=='create virtual' COLLATE nocase FROM sqlite_schema WHERE type IN ('table','view') AND       substr(name,1,7)!='sqlite_' COLLATE nocase  UNION ALL SELECT type, name, sql, 2, 0 FROM sqlite_schema WHERE type = 'trigger'  AND tbl_name IN(SELECT name FROM sqlite_schema WHERE type = 'view') ORDER BY 4, 5 DESC, 1\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pSchema) {
            let mut zType: *const ::core::ffi::c_char = sqlite3_column_text(
                pSchema,
                0 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            let mut zName: *const ::core::ffi::c_char = sqlite3_column_text(
                pSchema,
                1 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            let mut zSql: *const ::core::ffi::c_char = sqlite3_column_text(
                pSchema,
                2 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            let mut bVirtual: ::core::ffi::c_int = sqlite3_column_int(
                pSchema,
                4 as ::core::ffi::c_int,
            );
            let mut bExists: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if zType.is_null() || zName.is_null() {
                continue;
            }
            rc = expertDbContainsObject((*p).dbv, zName, &raw mut bExists);
            if rc != 0 || bExists != 0 {
                continue;
            }
            if *zType.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'v' as i32
                || *zType.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'r' as i32 || bVirtual != 0
            {
                if !zSql.is_null() {
                    rc = expertSchemaSql((*p).dbv, zSql, pzErrmsg);
                }
            } else {
                let mut pTab: *mut IdxTable = ::core::ptr::null_mut::<IdxTable>();
                rc = idxGetTableInfo((*p).db, zName, &raw mut pTab, pzErrmsg);
                if rc == SQLITE_OK && !pTab.is_null() {
                    let mut i: ::core::ffi::c_int = 0;
                    let mut zInner: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    let mut zOuter: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    (*pTab).pNext = (*p).pTable;
                    (*p).pTable = pTab;
                    zInner = idxAppendText(
                        &raw mut rc,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b"CREATE TABLE x(\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    i = 0 as ::core::ffi::c_int;
                    while i < (*pTab).nCol {
                        zInner = idxAppendText(
                            &raw mut rc,
                            zInner,
                            b"%s%Q COLLATE %s\0".as_ptr() as *const ::core::ffi::c_char,
                            if i == 0 as ::core::ffi::c_int {
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b", \0".as_ptr() as *const ::core::ffi::c_char
                            },
                            (*(*pTab).aCol.offset(i as isize)).zName,
                            (*(*pTab).aCol.offset(i as isize)).zColl,
                        );
                        i += 1;
                    }
                    zInner = idxAppendText(
                        &raw mut rc,
                        zInner,
                        b")\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    zOuter = idxAppendText(
                        &raw mut rc,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b"CREATE VIRTUAL TABLE %Q USING expert(%Q)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zName,
                        zInner,
                    );
                    if rc == SQLITE_OK {
                        rc = sqlite3_exec(
                            (*p).dbv,
                            zOuter,
                            None,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            pzErrmsg,
                        );
                    }
                    sqlite3_free(zInner as *mut ::core::ffi::c_void);
                    sqlite3_free(zOuter as *mut ::core::ffi::c_void);
                }
            }
        }
        idxFinalize(&raw mut rc, pSchema);
        return rc;
    }
}
unsafe extern "C" fn idxSampleFunc(
    mut pCtx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut IdxSampleCtx = sqlite3_user_data(pCtx) as *mut IdxSampleCtx;
        let mut bRet: ::core::ffi::c_int = 0;
        if (*p).nRow == 0.0f64 {
            bRet = 1 as ::core::ffi::c_int;
        } else {
            bRet = ((*p).nRet / (*p).nRow <= (*p).target) as ::core::ffi::c_int;
            if bRet == 0 as ::core::ffi::c_int {
                let mut rnd: ::core::ffi::c_ushort = 0;
                sqlite3_randomness(
                    2 as ::core::ffi::c_int,
                    &raw mut rnd as *mut ::core::ffi::c_void,
                );
                bRet = (rnd as ::core::ffi::c_int % 100 as ::core::ffi::c_int
                    <= (*p).iTarget) as ::core::ffi::c_int;
            }
        }
        sqlite3_result_int(pCtx, bRet);
        (*p).nRow += 1.0f64;
        (*p).nRet += bRet as ::core::ffi::c_double;
    }
}
unsafe extern "C" fn idxRemFunc(
    mut pCtx: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut IdxRemCtx = sqlite3_user_data(pCtx) as *mut IdxRemCtx;
        let mut pSlot: *mut IdxRemSlot = ::core::ptr::null_mut::<IdxRemSlot>();
        let mut iSlot: ::core::ffi::c_int = 0;
        iSlot = sqlite3_value_int(*argv.offset(0 as ::core::ffi::c_int as isize));
        pSlot = (&raw mut (*p).aSlot as *mut IdxRemSlot).offset(iSlot as isize)
            as *mut IdxRemSlot as *mut IdxRemSlot;
        match (*pSlot).eType {
            SQLITE_INTEGER => {
                sqlite3_result_int64(pCtx, (*pSlot).iVal as sqlite3_int64);
            }
            SQLITE_FLOAT => {
                sqlite3_result_double(pCtx, (*pSlot).rVal);
            }
            SQLITE_BLOB => {
                sqlite3_result_blob(
                    pCtx,
                    (*pSlot).z as *const ::core::ffi::c_void,
                    (*pSlot).n as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
            SQLITE_TEXT => {
                sqlite3_result_text(
                    pCtx,
                    (*pSlot).z,
                    (*pSlot).n as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
            SQLITE_NULL | _ => {}
        }
        (*pSlot).eType = sqlite3_value_type(
            *argv.offset(1 as ::core::ffi::c_int as isize),
        );
        match (*pSlot).eType {
            SQLITE_INTEGER => {
                (*pSlot).iVal = sqlite3_value_int64(
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                ) as i64_0;
            }
            SQLITE_FLOAT => {
                (*pSlot).rVal = sqlite3_value_double(
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                );
            }
            SQLITE_BLOB | SQLITE_TEXT => {
                let mut nByte: i64_0 = sqlite3_value_bytes(
                    *argv.offset(1 as ::core::ffi::c_int as isize),
                ) as i64_0;
                let mut pData: *const ::core::ffi::c_void = ::core::ptr::null::<
                    ::core::ffi::c_void,
                >();
                if nByte > (*pSlot).nByte {
                    let mut zNew: *mut ::core::ffi::c_char = sqlite3_realloc64(
                        (*pSlot).z as *mut ::core::ffi::c_void,
                        (nByte as ::core::ffi::c_longlong * 2 as ::core::ffi::c_longlong)
                            as sqlite3_uint64,
                    ) as *mut ::core::ffi::c_char;
                    if zNew.is_null() {
                        sqlite3_result_error_nomem(pCtx);
                        return;
                    }
                    (*pSlot).nByte = (nByte as ::core::ffi::c_longlong
                        * 2 as ::core::ffi::c_longlong) as i64_0;
                    (*pSlot).z = zNew;
                }
                (*pSlot).n = nByte;
                if (*pSlot).eType == SQLITE_BLOB {
                    pData = sqlite3_value_blob(
                        *argv.offset(1 as ::core::ffi::c_int as isize),
                    );
                    if !pData.is_null() {
                        memcpy(
                            (*pSlot).z as *mut ::core::ffi::c_void,
                            pData,
                            nByte as size_t,
                        );
                    }
                } else {
                    pData = sqlite3_value_text(
                        *argv.offset(1 as ::core::ffi::c_int as isize),
                    ) as *const ::core::ffi::c_void;
                    memcpy(
                        (*pSlot).z as *mut ::core::ffi::c_void,
                        pData,
                        nByte as size_t,
                    );
                }
            }
            SQLITE_NULL | _ => {}
        };
    }
}
unsafe extern "C" fn idxLargestIndex(
    mut db: *mut sqlite3,
    mut pnMax: *mut ::core::ffi::c_int,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zMax: *const ::core::ffi::c_char = b"SELECT max(i.seqno) FROM   sqlite_schema AS s,   pragma_index_list(s.name) AS l,   pragma_index_info(l.name) AS i WHERE s.type = 'table'\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut pMax: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        *pnMax = 0 as ::core::ffi::c_int;
        rc = idxPrepareStmt(db, &raw mut pMax, pzErr, zMax);
        if rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pMax) {
            *pnMax = sqlite3_column_int(pMax, 0 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int;
        }
        idxFinalize(&raw mut rc, pMax);
        return rc;
    }
}
unsafe extern "C" fn idxPopulateOneStat1(
    mut p: *mut sqlite3expert,
    mut pIndexXInfo: *mut sqlite3_stmt,
    mut pWriteStat: *mut sqlite3_stmt,
    mut zTab: *const ::core::ffi::c_char,
    mut zIdx: *const ::core::ffi::c_char,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zCols: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zOrder: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zQuery: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        let mut pQuery: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut aStat: *mut i64_0 = ::core::ptr::null_mut::<i64_0>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        sqlite3_bind_text(
            pIndexXInfo,
            1 as ::core::ffi::c_int,
            zIdx,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        while SQLITE_OK == rc && SQLITE_ROW == sqlite3_step(pIndexXInfo) {
            let mut zComma: *const ::core::ffi::c_char = if zCols.is_null() {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b", \0".as_ptr() as *const ::core::ffi::c_char
            };
            let mut zName: *const ::core::ffi::c_char = sqlite3_column_text(
                pIndexXInfo,
                0 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            let mut zColl: *const ::core::ffi::c_char = sqlite3_column_text(
                pIndexXInfo,
                1 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            if zName.is_null() {
                sqlite3_free(zCols as *mut ::core::ffi::c_void);
                sqlite3_free(zOrder as *mut ::core::ffi::c_void);
                return sqlite3_reset(pIndexXInfo);
            }
            zCols = idxAppendText(
                &raw mut rc,
                zCols,
                b"%sx.%Q IS sqlite_expert_rem(%d, x.%Q) COLLATE %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zComma,
                zName,
                nCol,
                zName,
                zColl,
            );
            nCol += 1;
            zOrder = idxAppendText(
                &raw mut rc,
                zOrder,
                b"%s%d\0".as_ptr() as *const ::core::ffi::c_char,
                zComma,
                nCol,
            );
        }
        sqlite3_reset(pIndexXInfo);
        if rc == SQLITE_OK {
            if (*p).iSample == 100 as ::core::ffi::c_int {
                zQuery = sqlite3_mprintf(
                    b"SELECT %s FROM %Q x ORDER BY %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    zCols,
                    zTab,
                    zOrder,
                );
            } else {
                zQuery = sqlite3_mprintf(
                    b"SELECT %s FROM temp.t592690916721053953805701627921227776 x ORDER BY %s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    zCols,
                    zOrder,
                );
            }
        }
        sqlite3_free(zCols as *mut ::core::ffi::c_void);
        sqlite3_free(zOrder as *mut ::core::ffi::c_void);
        if rc == SQLITE_OK {
            let mut dbrem: *mut sqlite3 = if (*p).iSample == 100 as ::core::ffi::c_int {
                (*p).db
            } else {
                (*p).dbv
            };
            rc = idxPrepareStmt(dbrem, &raw mut pQuery, pzErr, zQuery);
        }
        sqlite3_free(zQuery as *mut ::core::ffi::c_void);
        if rc == SQLITE_OK {
            aStat = idxMalloc(
                &raw mut rc,
                (::core::mem::size_of::<i64_0>() as usize)
                    .wrapping_mul((nCol + 1 as ::core::ffi::c_int) as usize) as i64_0,
            ) as *mut i64_0;
        }
        if rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pQuery) {
            let mut pEntry: *mut IdxHashEntry = ::core::ptr::null_mut::<IdxHashEntry>();
            let mut zStat: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            i = 0 as ::core::ffi::c_int;
            while i <= nCol {
                *aStat.offset(i as isize) = 1 as i64_0;
                i += 1;
            }
            while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pQuery) {
                let ref mut c2rust_fresh3 = *aStat
                    .offset(0 as ::core::ffi::c_int as isize);
                *c2rust_fresh3 += 1;
                i = 0 as ::core::ffi::c_int;
                while i < nCol {
                    if sqlite3_column_int(pQuery, i) == 0 as ::core::ffi::c_int {
                        break;
                    }
                    i += 1;
                }
                while i < nCol {
                    let ref mut c2rust_fresh4 = *aStat
                        .offset((i + 1 as ::core::ffi::c_int) as isize);
                    *c2rust_fresh4 += 1;
                    i += 1;
                }
            }
            if rc == SQLITE_OK {
                let mut s0: i64_0 = *aStat.offset(0 as ::core::ffi::c_int as isize);
                zStat = sqlite3_mprintf(
                    b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
                    s0,
                );
                if zStat.is_null() {
                    rc = SQLITE_NOMEM;
                }
                i = 1 as ::core::ffi::c_int;
                while rc == SQLITE_OK && i <= nCol {
                    zStat = idxAppendText(
                        &raw mut rc,
                        zStat,
                        b" %lld\0".as_ptr() as *const ::core::ffi::c_char,
                        (s0 + *aStat.offset(i as isize) / 2 as i64_0)
                            / *aStat.offset(i as isize),
                    );
                    i += 1;
                }
            }
            if rc == SQLITE_OK {
                sqlite3_bind_text(
                    pWriteStat,
                    1 as ::core::ffi::c_int,
                    zTab,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                sqlite3_bind_text(
                    pWriteStat,
                    2 as ::core::ffi::c_int,
                    zIdx,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                sqlite3_bind_text(
                    pWriteStat,
                    3 as ::core::ffi::c_int,
                    zStat,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                sqlite3_step(pWriteStat);
                rc = sqlite3_reset(pWriteStat);
            }
            pEntry = idxHashFind(
                &raw mut (*p).hIdx,
                zIdx,
                strlen(zIdx) as ::core::ffi::c_int,
            );
            if !pEntry.is_null() {
                (*pEntry).zVal2 = zStat;
            } else {
                sqlite3_free(zStat as *mut ::core::ffi::c_void);
            }
        }
        sqlite3_free(aStat as *mut ::core::ffi::c_void);
        idxFinalize(&raw mut rc, pQuery);
        return rc;
    }
}
unsafe extern "C" fn idxBuildSampleTable(
    mut p: *mut sqlite3expert,
    mut zTab: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        rc = sqlite3_exec(
            (*p).dbv,
            b"DROP TABLE IF EXISTS temp.t592690916721053953805701627921227776\0".as_ptr()
                as *const ::core::ffi::c_char,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            return rc;
        }
        zSql = sqlite3_mprintf(
            b"CREATE TABLE temp.t592690916721053953805701627921227776 AS SELECT * FROM %Q\0"
                .as_ptr() as *const ::core::ffi::c_char,
            zTab,
        );
        if zSql.is_null() {
            return SQLITE_NOMEM;
        }
        rc = sqlite3_exec(
            (*p).dbv,
            zSql,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn idxPopulateStat1(
    mut p: *mut sqlite3expert,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nMax: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pCtx: *mut IdxRemCtx = ::core::ptr::null_mut::<IdxRemCtx>();
        let mut samplectx: IdxSampleCtx = IdxSampleCtx {
            iTarget: 0,
            target: 0.,
            nRow: 0.,
            nRet: 0.,
        };
        let mut i: ::core::ffi::c_int = 0;
        let mut iPrev: i64_0 = -100000 as ::core::ffi::c_int as i64_0;
        let mut pAllIndex: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut pIndexXInfo: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut pWrite: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zAllIndex: *const ::core::ffi::c_char = b"SELECT s.rowid, s.name, l.name FROM   sqlite_schema AS s,   pragma_index_list(s.name) AS l WHERE s.type = 'table'\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut zIndexXInfo: *const ::core::ffi::c_char = b"SELECT name, coll FROM pragma_index_xinfo(?) WHERE key\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut zWrite: *const ::core::ffi::c_char = b"INSERT INTO sqlite_stat1 VALUES(?, ?, ?)\0"
            .as_ptr() as *const ::core::ffi::c_char;
        if (*p).iSample == 0 as ::core::ffi::c_int {
            return SQLITE_OK;
        }
        rc = idxLargestIndex((*p).dbm, &raw mut nMax, pzErr);
        if nMax <= 0 as ::core::ffi::c_int || rc != SQLITE_OK {
            return rc;
        }
        rc = sqlite3_exec(
            (*p).dbm,
            b"ANALYZE; PRAGMA writable_schema=1\0".as_ptr()
                as *const ::core::ffi::c_char,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            let mut nByte: i64_0 = (::core::mem::size_of::<IdxRemCtx>() as usize)
                .wrapping_add(
                    (::core::mem::size_of::<IdxRemSlot>() as usize)
                        .wrapping_mul(nMax as usize),
                ) as i64_0;
            pCtx = idxMalloc(&raw mut rc, nByte) as *mut IdxRemCtx;
        }
        if rc == SQLITE_OK {
            let mut dbrem: *mut sqlite3 = if (*p).iSample == 100 as ::core::ffi::c_int {
                (*p).db
            } else {
                (*p).dbv
            };
            rc = sqlite3_create_function(
                dbrem,
                b"sqlite_expert_rem\0".as_ptr() as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
                SQLITE_UTF8,
                pCtx as *mut ::core::ffi::c_void,
                Some(
                    idxRemFunc
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
                (*p).db,
                b"sqlite_expert_sample\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                SQLITE_UTF8,
                &raw mut samplectx as *mut ::core::ffi::c_void,
                Some(
                    idxSampleFunc
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
            (*pCtx).nSlot = (nMax as ::core::ffi::c_longlong
                + 1 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            rc = idxPrepareStmt((*p).dbm, &raw mut pAllIndex, pzErr, zAllIndex);
        }
        if rc == SQLITE_OK {
            rc = idxPrepareStmt((*p).dbm, &raw mut pIndexXInfo, pzErr, zIndexXInfo);
        }
        if rc == SQLITE_OK {
            rc = idxPrepareStmt((*p).dbm, &raw mut pWrite, pzErr, zWrite);
        }
        while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pAllIndex) {
            let mut iRowid: i64_0 = sqlite3_column_int64(
                pAllIndex,
                0 as ::core::ffi::c_int,
            ) as i64_0;
            let mut zTab: *const ::core::ffi::c_char = sqlite3_column_text(
                pAllIndex,
                1 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            let mut zIdx: *const ::core::ffi::c_char = sqlite3_column_text(
                pAllIndex,
                2 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            if zTab.is_null() || zIdx.is_null() {
                continue;
            }
            if (*p).iSample < 100 as ::core::ffi::c_int && iPrev != iRowid {
                samplectx.target = (*p).iSample as ::core::ffi::c_double / 100.0f64;
                samplectx.iTarget = (*p).iSample;
                samplectx.nRow = 0.0f64;
                samplectx.nRet = 0.0f64;
                rc = idxBuildSampleTable(p, zTab);
                if rc != SQLITE_OK {
                    break;
                }
            }
            rc = idxPopulateOneStat1(p, pIndexXInfo, pWrite, zTab, zIdx, pzErr);
            iPrev = iRowid;
        }
        if rc == SQLITE_OK && (*p).iSample < 100 as ::core::ffi::c_int {
            rc = sqlite3_exec(
                (*p).dbv,
                b"DROP TABLE IF EXISTS temp.t592690916721053953805701627921227776\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
        idxFinalize(&raw mut rc, pAllIndex);
        idxFinalize(&raw mut rc, pIndexXInfo);
        idxFinalize(&raw mut rc, pWrite);
        if !pCtx.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*pCtx).nSlot {
                sqlite3_free(
                    (*(&raw mut (*pCtx).aSlot as *mut IdxRemSlot).offset(i as isize)).z
                        as *mut ::core::ffi::c_void,
                );
                i += 1;
            }
            sqlite3_free(pCtx as *mut ::core::ffi::c_void);
        }
        if rc == SQLITE_OK {
            rc = sqlite3_exec(
                (*p).dbm,
                b"ANALYZE sqlite_schema\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
        sqlite3_create_function(
            (*p).db,
            b"sqlite_expert_rem\0".as_ptr() as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
            None,
            None,
        );
        sqlite3_create_function(
            (*p).db,
            b"sqlite_expert_sample\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
            None,
            None,
        );
        sqlite3_exec(
            (*p).db,
            b"DROP TABLE IF EXISTS temp.t592690916721053953805701627921227776\0".as_ptr()
                as *const ::core::ffi::c_char,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dummyCompare(
    mut up1: *mut ::core::ffi::c_void,
    mut up2: ::core::ffi::c_int,
    mut up3: *const ::core::ffi::c_void,
    mut up4: ::core::ffi::c_int,
    mut up5: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn useDummyCS(
    mut up1: *mut ::core::ffi::c_void,
    mut db: *mut sqlite3,
    mut etr: ::core::ffi::c_int,
    mut zName: *const ::core::ffi::c_char,
) {
    unsafe {
        sqlite3_create_collation_v2(
            db,
            zName,
            etr,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                dummyCompare
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            None,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dummyUDF(
    mut up1: *mut sqlite3_context,
    mut up2: ::core::ffi::c_int,
    mut up3: *mut *mut sqlite3_value,
) {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dummyUDFvalue(mut up1: *mut sqlite3_context) {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn registerUDFs(
    mut dbSrc: *mut sqlite3,
    mut dbDst: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
            dbSrc,
            b"SELECT name,type,enc,narg,flags FROM pragma_function_list() WHERE builtin==0\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            loop {
                rc = sqlite3_step(pStmt);
                if !(SQLITE_ROW == rc) {
                    break;
                }
                let mut nargs: ::core::ffi::c_int = sqlite3_column_int(
                    pStmt,
                    3 as ::core::ffi::c_int,
                );
                let mut flags: ::core::ffi::c_int = sqlite3_column_int(
                    pStmt,
                    4 as ::core::ffi::c_int,
                );
                let mut name: *const ::core::ffi::c_char = sqlite3_column_text(
                    pStmt,
                    0 as ::core::ffi::c_int,
                ) as *mut ::core::ffi::c_char;
                let mut type_0: *const ::core::ffi::c_char = sqlite3_column_text(
                    pStmt,
                    1 as ::core::ffi::c_int,
                ) as *mut ::core::ffi::c_char;
                let mut enc: *const ::core::ffi::c_char = sqlite3_column_text(
                    pStmt,
                    2 as ::core::ffi::c_int,
                ) as *mut ::core::ffi::c_char;
                if name.is_null() || type_0.is_null() || enc.is_null() {
                    continue;
                }
                let mut ienc: ::core::ffi::c_int = SQLITE_UTF8;
                let mut rcf: ::core::ffi::c_int = SQLITE_ERROR;
                if strcmp(enc, b"utf16le\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    ienc = SQLITE_UTF16LE;
                } else if strcmp(
                    enc,
                    b"utf16be\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    ienc = SQLITE_UTF16BE;
                }
                ienc |= flags & (SQLITE_DETERMINISTIC | SQLITE_DIRECTONLY);
                if strcmp(type_0, b"w\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    rcf = sqlite3_create_window_function(
                        dbDst,
                        name,
                        nargs,
                        ienc,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        Some(
                            dummyUDF
                                as unsafe extern "C" fn(
                                    *mut sqlite3_context,
                                    ::core::ffi::c_int,
                                    *mut *mut sqlite3_value,
                                ) -> (),
                        ),
                        Some(
                            dummyUDFvalue
                                as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                        ),
                        None,
                        None,
                        None,
                    );
                } else if strcmp(type_0, b"a\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    rcf = sqlite3_create_function(
                        dbDst,
                        name,
                        nargs,
                        ienc,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        None,
                        Some(
                            dummyUDF
                                as unsafe extern "C" fn(
                                    *mut sqlite3_context,
                                    ::core::ffi::c_int,
                                    *mut *mut sqlite3_value,
                                ) -> (),
                        ),
                        Some(
                            dummyUDFvalue
                                as unsafe extern "C" fn(*mut sqlite3_context) -> (),
                        ),
                    );
                } else if strcmp(type_0, b"s\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    rcf = sqlite3_create_function(
                        dbDst,
                        name,
                        nargs,
                        ienc,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        Some(
                            dummyUDF
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
                if !(rcf != SQLITE_OK) {
                    continue;
                }
                rc = rcf;
                break;
            }
            sqlite3_finalize(pStmt);
            if rc == SQLITE_DONE {
                rc = SQLITE_OK;
            }
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_expert_new(
    mut db: *mut sqlite3,
    mut pzErrmsg: *mut *mut ::core::ffi::c_char,
) -> *mut sqlite3expert {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pNew: *mut sqlite3expert = ::core::ptr::null_mut::<sqlite3expert>();
        pNew = idxMalloc(&raw mut rc, ::core::mem::size_of::<sqlite3expert>() as i64_0)
            as *mut sqlite3expert;
        if rc == SQLITE_OK {
            (*pNew).db = db;
            (*pNew).iSample = 100 as ::core::ffi::c_int;
            rc = sqlite3_open(
                b":memory:\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*pNew).dbv,
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3_open(
                b":memory:\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*pNew).dbm,
            );
            if rc == SQLITE_OK {
                sqlite3_db_config(
                    (*pNew).dbm,
                    SQLITE_DBCONFIG_TRIGGER_EQP,
                    1 as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
            }
        }
        if rc == SQLITE_OK {
            rc = sqlite3_collation_needed(
                (*pNew).dbm,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    useDummyCS
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut sqlite3,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                        ) -> (),
                ),
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3_collation_needed(
                (*pNew).dbv,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    useDummyCS
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut sqlite3,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                        ) -> (),
                ),
            );
        }
        if rc == SQLITE_OK {
            rc = registerUDFs((*pNew).db, (*pNew).dbm);
        }
        if rc == SQLITE_OK {
            rc = registerUDFs((*pNew).db, (*pNew).dbv);
        }
        if rc == SQLITE_OK {
            let mut pSql: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            rc = idxPrintfPrepareStmt(
                (*pNew).db,
                &raw mut pSql,
                pzErrmsg,
                b"SELECT sql, name, substr(sql,1,14)=='create virtual' COLLATE nocase FROM sqlite_schema WHERE substr(name,1,7)!='sqlite_' COLLATE nocase ORDER BY 3 DESC, rowid\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            while rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pSql) {
                let mut zSql: *const ::core::ffi::c_char = sqlite3_column_text(
                    pSql,
                    0 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                let mut zName: *const ::core::ffi::c_char = sqlite3_column_text(
                    pSql,
                    1 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                let mut bExists: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                rc = expertDbContainsObject((*pNew).dbm, zName, &raw mut bExists);
                if rc == SQLITE_OK && !zSql.is_null()
                    && bExists == 0 as ::core::ffi::c_int
                {
                    rc = expertSchemaSql((*pNew).dbm, zSql, pzErrmsg);
                }
            }
            idxFinalize(&raw mut rc, pSql);
        }
        if rc == SQLITE_OK {
            rc = idxCreateVtabSchema(pNew, pzErrmsg);
        }
        if rc == SQLITE_OK {
            sqlite3_set_authorizer(
                (*pNew).dbv,
                Some(
                    idxAuthCallback
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                pNew as *mut ::core::ffi::c_void,
            );
        }
        if rc != SQLITE_OK {
            sqlite3_expert_destroy(pNew);
            pNew = ::core::ptr::null_mut::<sqlite3expert>();
        }
        return pNew;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_expert_config(
    mut p: *mut sqlite3expert,
    mut op: ::core::ffi::c_int,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut ap = c2rust_args.clone();
        match op {
            EXPERT_CONFIG_SAMPLE => {
                let mut iVal: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
                if iVal < 0 as ::core::ffi::c_int {
                    iVal = 0 as ::core::ffi::c_int;
                }
                if iVal > 100 as ::core::ffi::c_int {
                    iVal = 100 as ::core::ffi::c_int;
                }
                (*p).iSample = iVal;
            }
            _ => {
                rc = SQLITE_NOTFOUND;
            }
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_expert_sql(
    mut p: *mut sqlite3expert,
    mut zSql: *const ::core::ffi::c_char,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pScanOrig: *mut IdxScan = (*p).pScan;
        let mut pStmtOrig: *mut IdxStatement = (*p).pStatement;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zStmt: *const ::core::ffi::c_char = zSql;
        if (*p).bRun != 0 {
            return SQLITE_MISUSE;
        }
        while rc == SQLITE_OK && !zStmt.is_null()
            && *zStmt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            rc = idxPrepareStmt((*p).db, &raw mut pStmt, pzErr, zStmt);
            if rc != SQLITE_OK {
                break;
            }
            sqlite3_finalize(pStmt);
            rc = sqlite3_prepare_v2(
                (*p).dbv,
                zStmt,
                -1 as ::core::ffi::c_int,
                &raw mut pStmt,
                &raw mut zStmt,
            );
            if rc == SQLITE_OK {
                if !pStmt.is_null() {
                    let mut pNew: *mut IdxStatement = ::core::ptr::null_mut::<
                        IdxStatement,
                    >();
                    let mut z: *const ::core::ffi::c_char = sqlite3_sql(pStmt);
                    let mut n: i64_0 = strlen(z) as ::core::ffi::c_int as i64_0;
                    pNew = idxMalloc(
                        &raw mut rc,
                        (::core::mem::size_of::<IdxStatement>()
                            as ::core::ffi::c_ulonglong)
                            .wrapping_add(n as ::core::ffi::c_ulonglong)
                            .wrapping_add(1 as ::core::ffi::c_ulonglong) as i64_0,
                    ) as *mut IdxStatement;
                    if rc == SQLITE_OK {
                        (*pNew).zSql = pNew.offset(1 as ::core::ffi::c_int as isize)
                            as *mut IdxStatement as *mut ::core::ffi::c_char;
                        memcpy(
                            (*pNew).zSql as *mut ::core::ffi::c_void,
                            z as *const ::core::ffi::c_void,
                            (n as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                                as size_t,
                        );
                        (*pNew).pNext = (*p).pStatement;
                        if !(*p).pStatement.is_null() {
                            (*pNew).iId = (*(*p).pStatement).iId
                                + 1 as ::core::ffi::c_int;
                        }
                        (*p).pStatement = pNew;
                    }
                    sqlite3_finalize(pStmt);
                }
            } else {
                idxDatabaseError((*p).dbv, pzErr);
            }
        }
        if rc != SQLITE_OK {
            idxScanFree((*p).pScan, pScanOrig);
            idxStatementFree((*p).pStatement, pStmtOrig);
            (*p).pScan = pScanOrig;
            (*p).pStatement = pStmtOrig;
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_expert_analyze(
    mut p: *mut sqlite3expert,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pEntry: *mut IdxHashEntry = ::core::ptr::null_mut::<IdxHashEntry>();
        rc = idxProcessTriggers(p, pzErr);
        if rc == SQLITE_OK {
            rc = idxCreateCandidates(p);
        } else if rc == SQLITE_BUSY_TIMEOUT {
            if !pzErr.is_null() {
                *pzErr = sqlite3_mprintf(
                    b"Cannot find a unique index name to propose.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            return rc;
        }
        if rc == SQLITE_OK {
            rc = idxPopulateStat1(p, pzErr);
        }
        pEntry = (*p).hIdx.pFirst;
        while !pEntry.is_null() {
            (*p).zCandidates = idxAppendText(
                &raw mut rc,
                (*p).zCandidates,
                b"%s;%s%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*pEntry).zVal,
                if !(*pEntry).zVal2.is_null() {
                    b" -- stat1: \0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                (*pEntry).zVal2,
            );
            pEntry = (*pEntry).pNext;
        }
        if rc == SQLITE_OK {
            rc = idxFindIndexes(p, pzErr);
        }
        if rc == SQLITE_OK {
            (*p).bRun = 1 as ::core::ffi::c_int;
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_expert_count(
    mut p: *mut sqlite3expert,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nRet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !(*p).pStatement.is_null() {
            nRet = (*(*p).pStatement).iId + 1 as ::core::ffi::c_int;
        }
        return nRet;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_expert_report(
    mut p: *mut sqlite3expert,
    mut iStmt: ::core::ffi::c_int,
    mut eReport: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut zRet: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pStmt: *mut IdxStatement = ::core::ptr::null_mut::<IdxStatement>();
        if (*p).bRun == 0 as ::core::ffi::c_int {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        pStmt = (*p).pStatement;
        while !pStmt.is_null() && (*pStmt).iId != iStmt {
            pStmt = (*pStmt).pNext;
        }
        match eReport {
            EXPERT_REPORT_SQL => {
                if !pStmt.is_null() {
                    zRet = (*pStmt).zSql;
                }
            }
            EXPERT_REPORT_INDEXES => {
                if !pStmt.is_null() {
                    zRet = (*pStmt).zIdx;
                }
            }
            EXPERT_REPORT_PLAN => {
                if !pStmt.is_null() {
                    zRet = (*pStmt).zEQP;
                }
            }
            EXPERT_REPORT_CANDIDATES => {
                zRet = (*p).zCandidates;
            }
            _ => {}
        }
        return zRet;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_expert_destroy(mut p: *mut sqlite3expert) {
    unsafe {
        if !p.is_null() {
            sqlite3_close((*p).dbm);
            sqlite3_close((*p).dbv);
            idxScanFree((*p).pScan, ::core::ptr::null_mut::<IdxScan>());
            idxStatementFree((*p).pStatement, ::core::ptr::null_mut::<IdxStatement>());
            idxTableFree((*p).pTable);
            idxWriteFree((*p).pWrite);
            idxHashClear(&raw mut (*p).hIdx);
            sqlite3_free((*p).zCandidates as *mut ::core::ffi::c_void);
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }
}
