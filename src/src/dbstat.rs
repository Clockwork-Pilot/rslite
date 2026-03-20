use ::c2rust_bitfields;

extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Btree;
    pub type VtabCtx;
    pub type PreUpdate;
    pub type RenameToken;
    pub type Vdbe;
    pub type TableLock;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type Pager;
    pub type PCache;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> sqlite3_int64;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
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
    fn sqlite3_str_new(_: *mut sqlite3) -> *mut sqlite3_str;
    fn sqlite3_str_finish(_: *mut sqlite3_str) -> *mut ::core::ffi::c_char;
    fn sqlite3_str_appendf(_: *mut sqlite3_str, zFormat: *const ::core::ffi::c_char, ...);
    fn sqlite3_vtab_config(_: *mut sqlite3, op: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
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
    fn sqlite3OsFileControl(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerGet(
        pPager: *mut Pager,
        pgno: Pgno,
        ppPage: *mut *mut DbPage,
        clrFlag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerUnref(_: *mut DbPage);
    fn sqlite3PagerGetData(_: *mut DbPage) -> *mut ::core::ffi::c_void;
    fn sqlite3PagerPagecount(_: *mut Pager, _: *mut ::core::ffi::c_int);
    fn sqlite3PagerFile(_: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3BtreeGetPageSize(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeGetReserveNoMutex(p: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnter(_: *mut Btree);
    fn sqlite3BtreeLeave(_: *mut Btree);
    fn sqlite3CorruptError(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3TokenInit(_: *mut Token, _: *mut ::core::ffi::c_char);
    fn sqlite3GetVarint(_: *const ::core::ffi::c_uchar, _: *mut u64_0) -> u8_0;
    fn sqlite3GetVarint32(_: *const ::core::ffi::c_uchar, _: *mut u32_0) -> u8_0;
    fn sqlite3FindDb(_: *mut sqlite3, _: *mut Token) -> ::core::ffi::c_int;
    fn sqlite3FindDbName(_: *mut sqlite3, _: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn sqlite3Get4byte(_: *const u8_0) -> u32_0;
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
    pub trace: C2RustUnnamed_21,
    pub pTraceArg: *mut ::core::ffi::c_void,
    pub xProfile: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, u64_0) -> (),
    >,
    pub pProfileArg: *mut ::core::ffi::c_void,
    pub pCommitArg: *mut ::core::ffi::c_void,
    pub xCommitCallback:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
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
    pub u1: C2RustUnnamed_17,
    pub lookaside: Lookaside,
    pub xAuth: sqlite3_xauth,
    pub pAuthArg: *mut ::core::ffi::c_void,
    pub xProgress: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
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
    pub u: C2RustUnnamed_13,
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
    pub fg: C2RustUnnamed_12,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: Bitmask,
    pub u1: C2RustUnnamed_11,
    pub u2: C2RustUnnamed_10,
    pub u3: C2RustUnnamed_9,
    pub u4: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
    pub u: C2RustUnnamed_8,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: C2RustUnnamed_7,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_0,
    pub w: C2RustUnnamed_6,
    pub pAggInfo: *mut AggInfo,
    pub y: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub pTab: *mut Table,
    pub pWin: *mut Window,
    pub nReg: ::core::ffi::c_int,
    pub sub: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
pub type i16_0 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = i16;
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
    pub fg: C2RustUnnamed_5,
    pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub x: C2RustUnnamed_4,
    pub iConstExprReg: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub iOrderByCol: u16_0,
    pub iAlias: u16_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
pub union C2RustUnnamed_6 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int,
}
pub type ynVar = i16_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub zToken: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int,
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
pub union C2RustUnnamed_9 {
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
pub union C2RustUnnamed_10 {
    pub pIBIndex: *mut Index,
    pub pCteUse: *mut CteUse,
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
    pub idxType_bUnordered_uniqNotNull_isResized_isCovering_noSkipScan_hasStat1_bNoQuery_bAscKeyBug_bHasVCol_bHasExpr:
        [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub colNotIdxed: Bitmask,
}
pub type Bitmask = u64_0;
pub type Pgno = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
    pub notIndexed_isIndexedBy_isSubquery_isTabFunc_isCorrelated_isMaterialized_viaCoroutine_isRecursive_fromDDL_isCte_notCte_isUsing_isOn_isSynthUsing_isNestedFrom_rowidUsed_fixedSchema_hadSchema_fromExists:
        [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub tab: C2RustUnnamed_16,
    pub view: C2RustUnnamed_15,
    pub vtab: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub nArg: ::core::ffi::c_int,
    pub azArg: *mut *mut ::core::ffi::c_char,
    pub p: *mut VTable,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub pSelect: *mut Select,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
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
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
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
pub union C2RustUnnamed_17 {
    pub isInterrupted: ::core::ffi::c_int,
    pub notUsed1: ::core::ffi::c_double,
}
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
    pub u1: C2RustUnnamed_18,
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
pub struct Token {
    pub z: *const ::core::ffi::c_char,
    pub n: ::core::ffi::c_uint,
}
pub type VList = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub cr: C2RustUnnamed_20,
    pub d: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
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
pub struct C2RustUnnamed_20 {
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
    pub xCleanup: Option<unsafe extern "C" fn(*mut sqlite3, *mut ::core::ffi::c_void) -> ()>,
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
pub type Mem = sqlite3_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AutoincInfo {
    pub pNext: *mut AutoincInfo,
    pub pTab: *mut Table,
    pub iDb: ::core::ffi::c_int,
    pub regCtr: ::core::ffi::c_int,
}
pub type yDbMask = ::core::ffi::c_uint;
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
pub type bft = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub xLegacy:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> ()>,
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
        unsafe extern "C" fn(*mut sqlite3_vfs, ::core::ffi::c_int, *mut ::core::ffi::c_char) -> (),
    >,
    pub xDlSym: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> Option<unsafe extern "C" fn() -> ()>,
    >,
    pub xDlClose: Option<unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> ()>,
    pub xRandomness: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSleep:
        Option<unsafe extern "C" fn(*mut sqlite3_vfs, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xCurrentTime: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_double) -> ::core::ffi::c_int,
    >,
    pub xGetLastError: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xCurrentTimeInt64:
        Option<unsafe extern "C" fn(*mut sqlite3_vfs, *mut sqlite3_int64) -> ::core::ffi::c_int>,
    pub xSetSystemCall: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            sqlite3_syscall_ptr,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetSystemCall: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, *const ::core::ffi::c_char) -> sqlite3_syscall_ptr,
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
    pub xTruncate:
        Option<unsafe extern "C" fn(*mut sqlite3_file, sqlite3_int64) -> ::core::ffi::c_int>,
    pub xSync:
        Option<unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xFileSize:
        Option<unsafe extern "C" fn(*mut sqlite3_file, *mut sqlite3_int64) -> ::core::ffi::c_int>,
    pub xLock:
        Option<unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xUnlock:
        Option<unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xCheckReservedLock: Option<
        unsafe extern "C" fn(*mut sqlite3_file, *mut ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xFileControl: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xSectorSize: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>,
    pub xDeviceCharacteristics:
        Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>,
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
    pub xShmUnmap:
        Option<unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int>,
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
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_str {
    pub db: *mut sqlite3,
    pub zText: *mut ::core::ffi::c_char,
    pub nAlloc: u32_0,
    pub mxAlloc: u32_0,
    pub nChar: u32_0,
    pub accError: u8_0,
    pub printfFlags: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PgHdr {
    pub pPage: *mut sqlite3_pcache_page,
    pub pData: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void,
    pub pCache: *mut PCache,
    pub pDirty: *mut PgHdr,
    pub pPager: *mut Pager,
    pub pgno: Pgno,
    pub flags: u16_0,
    pub nRef: i64_0,
    pub pDirtyNext: *mut PgHdr,
    pub pDirtyPrev: *mut PgHdr,
}
pub type DbPage = PgHdr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StatCursor {
    pub base: sqlite3_vtab_cursor,
    pub pStmt: *mut sqlite3_stmt,
    pub isEof: u8_0,
    pub isAgg: u8_0,
    pub iDb: ::core::ffi::c_int,
    pub aPage: [StatPage; 32],
    pub iPage: ::core::ffi::c_int,
    pub iPageno: u32_0,
    pub zName: *mut ::core::ffi::c_char,
    pub zPath: *mut ::core::ffi::c_char,
    pub zPagetype: *mut ::core::ffi::c_char,
    pub nPage: ::core::ffi::c_int,
    pub nCell: ::core::ffi::c_int,
    pub nMxPayload: ::core::ffi::c_int,
    pub nUnused: i64_0,
    pub nPayload: i64_0,
    pub iOffset: i64_0,
    pub szPage: i64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StatPage {
    pub iPgno: u32_0,
    pub aPg: *mut u8_0,
    pub iCell: ::core::ffi::c_int,
    pub zPath: *mut ::core::ffi::c_char,
    pub flags: u8_0,
    pub nCell: ::core::ffi::c_int,
    pub nUnused: ::core::ffi::c_int,
    pub aCell: *mut StatCell,
    pub iRightChildPg: u32_0,
    pub nMxPayload: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StatCell {
    pub nLocal: ::core::ffi::c_int,
    pub iChildPg: u32_0,
    pub nOvfl: ::core::ffi::c_int,
    pub aOvfl: *mut u32_0,
    pub nLastOvfl: ::core::ffi::c_int,
    pub iOvfl: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StatTable {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub iDb: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_SCAN_HEX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_VTAB_DIRECTONLY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const DBSTAT_PAGE_PADDING_BYTES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut zDbstatSchema: [::core::ffi::c_char; 258] = unsafe {
    ::core::mem::transmute::<
        [u8; 258],
        [::core::ffi::c_char; 258],
    >(
        *b"CREATE TABLE x( name       TEXT, path       TEXT, pageno     INTEGER, pagetype   TEXT, ncell      INTEGER, payload    INTEGER, unused     INTEGER, mx_payload INTEGER, pgoffset   INTEGER, pgsize     INTEGER, schema     TEXT HIDDEN, aggregate  BOOLEAN HIDDEN)\0",
    )
};
unsafe extern "C" fn statConnect(
    mut db: *mut sqlite3,
    mut _pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pTab: *mut StatTable = ::core::ptr::null_mut::<StatTable>();
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut iDb: ::core::ffi::c_int = 0;
    if argc >= 4 as ::core::ffi::c_int {
        let mut nm: Token = Token {
            z: ::core::ptr::null::<::core::ffi::c_char>(),
            n: 0,
        };
        sqlite3TokenInit(
            &raw mut nm,
            *argv.offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        );
        iDb = sqlite3FindDb(db, &raw mut nm);
        if iDb < 0 as ::core::ffi::c_int {
            *pzErr = sqlite3_mprintf(
                b"no such database: %s\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(3 as ::core::ffi::c_int as isize),
            );
            return SQLITE_ERROR;
        }
    } else {
        iDb = 0 as ::core::ffi::c_int;
    }
    sqlite3_vtab_config(db, SQLITE_VTAB_DIRECTONLY);
    rc = sqlite3_declare_vtab(db, &raw const zDbstatSchema as *const ::core::ffi::c_char);
    if rc == SQLITE_OK {
        pTab = sqlite3_malloc64(::core::mem::size_of::<StatTable>() as sqlite3_uint64)
            as *mut StatTable;
        if pTab.is_null() {
            rc = SQLITE_NOMEM_BKPT;
        }
    }
    if rc == SQLITE_OK {
        memset(
            pTab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<StatTable>() as size_t,
        );
        (*pTab).db = db;
        (*pTab).iDb = iDb;
    }
    *ppVtab = pTab as *mut sqlite3_vtab;
    return rc;
}
unsafe extern "C" fn statDisconnect(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    sqlite3_free(pVtab as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn statBestIndex(
    mut _tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut iSchema: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iName: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iAgg: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < (*pIdxInfo).nConstraint {
        if !((*(*pIdxInfo).aConstraint.offset(i as isize)).op as ::core::ffi::c_int
            != SQLITE_INDEX_CONSTRAINT_EQ)
        {
            if (*(*pIdxInfo).aConstraint.offset(i as isize)).usable as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                return SQLITE_CONSTRAINT;
            }
            match (*(*pIdxInfo).aConstraint.offset(i as isize)).iColumn {
                0 => {
                    iName = i;
                }
                10 => {
                    iSchema = i;
                }
                11 => {
                    iAgg = i;
                }
                _ => {}
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    if iSchema >= 0 as ::core::ffi::c_int {
        i += 1;
        (*(*pIdxInfo).aConstraintUsage.offset(iSchema as isize)).argvIndex = i;
        (*(*pIdxInfo).aConstraintUsage.offset(iSchema as isize)).omit = 1 as ::core::ffi::c_uchar;
        (*pIdxInfo).idxNum |= 0x1 as ::core::ffi::c_int;
    }
    if iName >= 0 as ::core::ffi::c_int {
        i += 1;
        (*(*pIdxInfo).aConstraintUsage.offset(iName as isize)).argvIndex = i;
        (*pIdxInfo).idxNum |= 0x2 as ::core::ffi::c_int;
    }
    if iAgg >= 0 as ::core::ffi::c_int {
        i += 1;
        (*(*pIdxInfo).aConstraintUsage.offset(iAgg as isize)).argvIndex = i;
        (*pIdxInfo).idxNum |= 0x4 as ::core::ffi::c_int;
    }
    (*pIdxInfo).estimatedCost = 1.0f64;
    if (*pIdxInfo).nOrderBy == 1 as ::core::ffi::c_int
        && (*(*pIdxInfo)
            .aOrderBy
            .offset(0 as ::core::ffi::c_int as isize))
        .iColumn
            == 0 as ::core::ffi::c_int
        && (*(*pIdxInfo)
            .aOrderBy
            .offset(0 as ::core::ffi::c_int as isize))
        .desc as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        || (*pIdxInfo).nOrderBy == 2 as ::core::ffi::c_int
            && (*(*pIdxInfo)
                .aOrderBy
                .offset(0 as ::core::ffi::c_int as isize))
            .iColumn
                == 0 as ::core::ffi::c_int
            && (*(*pIdxInfo)
                .aOrderBy
                .offset(0 as ::core::ffi::c_int as isize))
            .desc as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            && (*(*pIdxInfo)
                .aOrderBy
                .offset(1 as ::core::ffi::c_int as isize))
            .iColumn
                == 1 as ::core::ffi::c_int
            && (*(*pIdxInfo)
                .aOrderBy
                .offset(1 as ::core::ffi::c_int as isize))
            .desc as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
    {
        (*pIdxInfo).orderByConsumed = 1 as ::core::ffi::c_int;
        (*pIdxInfo).idxNum |= 0x8 as ::core::ffi::c_int;
    }
    (*pIdxInfo).idxFlags |= SQLITE_INDEX_SCAN_HEX;
    return SQLITE_OK;
}
unsafe extern "C" fn statOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pTab: *mut StatTable = pVTab as *mut StatTable;
    let mut pCsr: *mut StatCursor = ::core::ptr::null_mut::<StatCursor>();
    pCsr =
        sqlite3_malloc64(::core::mem::size_of::<StatCursor>() as sqlite3_uint64) as *mut StatCursor;
    if pCsr.is_null() {
        return SQLITE_NOMEM_BKPT;
    } else {
        memset(
            pCsr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<StatCursor>() as size_t,
        );
        (*pCsr).base.pVtab = pVTab;
        (*pCsr).iDb = (*pTab).iDb;
    }
    *ppCursor = pCsr as *mut sqlite3_vtab_cursor;
    return SQLITE_OK;
}
unsafe extern "C" fn statClearCells(mut p: *mut StatPage) {
    let mut i: ::core::ffi::c_int = 0;
    if !(*p).aCell.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nCell {
            sqlite3_free((*(*p).aCell.offset(i as isize)).aOvfl as *mut ::core::ffi::c_void);
            i += 1;
        }
        sqlite3_free((*p).aCell as *mut ::core::ffi::c_void);
    }
    (*p).nCell = 0 as ::core::ffi::c_int;
    (*p).aCell = ::core::ptr::null_mut::<StatCell>();
}
unsafe extern "C" fn statClearPage(mut p: *mut StatPage) {
    let mut aPg: *mut u8_0 = (*p).aPg;
    statClearCells(p);
    sqlite3_free((*p).zPath as *mut ::core::ffi::c_void);
    memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<StatPage>() as size_t,
    );
    (*p).aPg = aPg;
}
unsafe extern "C" fn statResetCsr(mut pCsr: *mut StatCursor) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[StatPage; 32]>() as usize)
            .wrapping_div(::core::mem::size_of::<StatPage>() as usize)
            as ::core::ffi::c_int
    {
        statClearPage((&raw mut (*pCsr).aPage as *mut StatPage).offset(i as isize) as *mut StatPage);
        sqlite3_free((*pCsr).aPage[i as usize].aPg as *mut ::core::ffi::c_void);
        (*pCsr).aPage[i as usize].aPg = ::core::ptr::null_mut::<u8_0>();
        i += 1;
    }
    sqlite3_reset((*pCsr).pStmt);
    (*pCsr).iPage = 0 as ::core::ffi::c_int;
    sqlite3_free((*pCsr).zPath as *mut ::core::ffi::c_void);
    (*pCsr).zPath = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pCsr).isEof = 0 as u8_0;
}
unsafe extern "C" fn statResetCounts(mut pCsr: *mut StatCursor) {
    (*pCsr).nCell = 0 as ::core::ffi::c_int;
    (*pCsr).nMxPayload = 0 as ::core::ffi::c_int;
    (*pCsr).nUnused = 0 as i64_0;
    (*pCsr).nPayload = 0 as i64_0;
    (*pCsr).szPage = 0 as i64_0;
    (*pCsr).nPage = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn statClose(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    statResetCsr(pCsr);
    sqlite3_finalize((*pCsr).pStmt);
    sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn getLocalPayload(
    mut nUsable: ::core::ffi::c_int,
    mut flags: u8_0,
    mut nTotal: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nLocal: ::core::ffi::c_int = 0;
    let mut nMinLocal: ::core::ffi::c_int = 0;
    let mut nMaxLocal: ::core::ffi::c_int = 0;
    if flags as ::core::ffi::c_int == 0xd as ::core::ffi::c_int {
        nMinLocal = (nUsable - 12 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int
            / 255 as ::core::ffi::c_int
            - 23 as ::core::ffi::c_int;
        nMaxLocal = nUsable - 35 as ::core::ffi::c_int;
    } else {
        nMinLocal = (nUsable - 12 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int
            / 255 as ::core::ffi::c_int
            - 23 as ::core::ffi::c_int;
        nMaxLocal = (nUsable - 12 as ::core::ffi::c_int) * 64 as ::core::ffi::c_int
            / 255 as ::core::ffi::c_int
            - 23 as ::core::ffi::c_int;
    }
    nLocal = nMinLocal + (nTotal - nMinLocal) % (nUsable - 4 as ::core::ffi::c_int);
    if nLocal > nMaxLocal {
        nLocal = nMinLocal;
    }
    return nLocal;
}
unsafe extern "C" fn statDecodePage(
    mut pBt: *mut Btree,
    mut p: *mut StatPage,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut nUnused: ::core::ffi::c_int = 0;
    let mut iOff: ::core::ffi::c_int = 0;
    let mut nHdr: ::core::ffi::c_int = 0;
    let mut isLeaf: ::core::ffi::c_int = 0;
    let mut szPage: ::core::ffi::c_int = 0;
    let mut aData: *mut u8_0 = (*p).aPg;
    let mut aHdr: *mut u8_0 = aData.offset(
        (if (*p).iPgno == 1 as u32_0 {
            100 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as isize,
    ) as *mut u8_0;
    (*p).flags = *aHdr.offset(0 as ::core::ffi::c_int as isize);
    if (*p).flags as ::core::ffi::c_int == 0xa as ::core::ffi::c_int
        || (*p).flags as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
    {
        isLeaf = 1 as ::core::ffi::c_int;
        nHdr = 8 as ::core::ffi::c_int;
        current_block = 3640593987805443782;
    } else if (*p).flags as ::core::ffi::c_int == 0x5 as ::core::ffi::c_int
        || (*p).flags as ::core::ffi::c_int == 0x2 as ::core::ffi::c_int
    {
        isLeaf = 0 as ::core::ffi::c_int;
        nHdr = 12 as ::core::ffi::c_int;
        current_block = 3640593987805443782;
    } else {
        current_block = 13503832839001051727;
    }
    match current_block {
        3640593987805443782 => {
            if (*p).iPgno == 1 as u32_0 {
                nHdr += 100 as ::core::ffi::c_int;
            }
            (*p).nCell = (*(aHdr.offset(3 as ::core::ffi::c_int as isize) as *mut u8_0)
                .offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(aHdr.offset(3 as ::core::ffi::c_int as isize) as *mut u8_0)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
            (*p).nMxPayload = 0 as ::core::ffi::c_int;
            szPage = sqlite3BtreeGetPageSize(pBt);
            nUnused = ((*(aHdr.offset(5 as ::core::ffi::c_int as isize) as *mut u8_0)
                .offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(aHdr.offset(5 as ::core::ffi::c_int as isize) as *mut u8_0)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int)
                - nHdr
                - 2 as ::core::ffi::c_int * (*p).nCell;
            nUnused += *aHdr.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            iOff = (*(aHdr.offset(1 as ::core::ffi::c_int as isize) as *mut u8_0)
                .offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(aHdr.offset(1 as ::core::ffi::c_int as isize) as *mut u8_0)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
            loop {
                if !(iOff != 0) {
                    current_block = 12147880666119273379;
                    break;
                }
                let mut iNext: ::core::ffi::c_int = 0;
                if iOff >= szPage {
                    current_block = 13503832839001051727;
                    break;
                }
                nUnused += (*(aData.offset((iOff + 2 as ::core::ffi::c_int) as isize) as *mut u8_0)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(aData.offset((iOff + 2 as ::core::ffi::c_int) as isize) as *mut u8_0)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                iNext = (*(aData.offset(iOff as isize) as *mut u8_0)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(aData.offset(iOff as isize) as *mut u8_0)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                if iNext < iOff + 4 as ::core::ffi::c_int && iNext > 0 as ::core::ffi::c_int {
                    current_block = 13503832839001051727;
                    break;
                }
                iOff = iNext;
            }
            match current_block {
                13503832839001051727 => {}
                _ => {
                    (*p).nUnused = nUnused;
                    (*p).iRightChildPg = if isLeaf != 0 {
                        0 as u32_0
                    } else {
                        sqlite3Get4byte(aHdr.offset(8 as ::core::ffi::c_int as isize) as *mut u8_0)
                    };
                    if (*p).nCell != 0 {
                        let mut i: ::core::ffi::c_int = 0;
                        let mut nUsable: ::core::ffi::c_int = 0;
                        sqlite3BtreeEnter(pBt);
                        nUsable = szPage - sqlite3BtreeGetReserveNoMutex(pBt);
                        sqlite3BtreeLeave(pBt);
                        (*p).aCell = sqlite3_malloc64(
                            (((*p).nCell + 1 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<StatCell>() as usize)
                                as sqlite3_uint64,
                        ) as *mut StatCell;
                        if (*p).aCell.is_null() {
                            return SQLITE_NOMEM_BKPT;
                        }
                        memset(
                            (*p).aCell as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (((*p).nCell + 1 as ::core::ffi::c_int) as size_t)
                                .wrapping_mul(::core::mem::size_of::<StatCell>() as size_t),
                        );
                        i = 0 as ::core::ffi::c_int;
                        loop {
                            if !(i < (*p).nCell) {
                                current_block = 5597585068398118923;
                                break;
                            }
                            let mut pCell: *mut StatCell =
                                (*p).aCell.offset(i as isize) as *mut StatCell;
                            iOff = (*(aData.offset((nHdr + i * 2 as ::core::ffi::c_int) as isize)
                                as *mut u8_0)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int
                                | *(aData.offset((nHdr + i * 2 as ::core::ffi::c_int) as isize)
                                    as *mut u8_0)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int;
                            if iOff < nHdr || iOff >= szPage {
                                current_block = 13503832839001051727;
                                break;
                            }
                            if isLeaf == 0 {
                                (*pCell).iChildPg =
                                    sqlite3Get4byte(aData.offset(iOff as isize) as *mut u8_0);
                                iOff += 4 as ::core::ffi::c_int;
                            }
                            if !((*p).flags as ::core::ffi::c_int == 0x5 as ::core::ffi::c_int) {
                                let mut nPayload: u32_0 = 0;
                                let mut nLocal: ::core::ffi::c_int = 0;
                                iOff += (if (*aData.offset(iOff as isize) as ::core::ffi::c_int)
                                    < 0x80 as ::core::ffi::c_int as u8_0 as ::core::ffi::c_int
                                {
                                    nPayload = *aData.offset(iOff as isize) as u32_0;
                                    1 as ::core::ffi::c_int
                                } else {
                                    sqlite3GetVarint32(
                                        aData.offset(iOff as isize) as *mut u8_0,
                                        &raw mut nPayload,
                                    ) as ::core::ffi::c_int
                                }) as u8_0
                                    as ::core::ffi::c_int;
                                if (*p).flags as ::core::ffi::c_int == 0xd as ::core::ffi::c_int {
                                    let mut dummy: u64_0 = 0;
                                    iOff += sqlite3GetVarint(
                                        aData.offset(iOff as isize) as *mut u8_0,
                                        &raw mut dummy,
                                    )
                                        as ::core::ffi::c_int;
                                }
                                if nPayload > (*p).nMxPayload as u32_0 {
                                    (*p).nMxPayload = nPayload as ::core::ffi::c_int;
                                }
                                nLocal = getLocalPayload(
                                    nUsable,
                                    (*p).flags,
                                    nPayload as ::core::ffi::c_int,
                                );
                                if nLocal < 0 as ::core::ffi::c_int {
                                    current_block = 13503832839001051727;
                                    break;
                                }
                                (*pCell).nLocal = nLocal;
                                if nPayload > nLocal as u32_0 {
                                    let mut j: ::core::ffi::c_int = 0;
                                    let mut nOvfl: ::core::ffi::c_int = nPayload
                                        .wrapping_sub(nLocal as u32_0)
                                        .wrapping_add(nUsable as u32_0)
                                        .wrapping_sub(4 as u32_0)
                                        .wrapping_sub(1 as u32_0)
                                        .wrapping_div((nUsable - 4 as ::core::ffi::c_int) as u32_0)
                                        as ::core::ffi::c_int;
                                    if iOff + nLocal + 4 as ::core::ffi::c_int > nUsable
                                        || nPayload > 0x7fffffff as u32_0
                                    {
                                        current_block = 13503832839001051727;
                                        break;
                                    }
                                    (*pCell).nLastOvfl =
                                        nPayload.wrapping_sub(nLocal as u32_0).wrapping_sub(
                                            ((nOvfl - 1 as ::core::ffi::c_int)
                                                * (nUsable - 4 as ::core::ffi::c_int))
                                                as u32_0,
                                        )
                                            as ::core::ffi::c_int;
                                    (*pCell).nOvfl = nOvfl;
                                    (*pCell).aOvfl = sqlite3_malloc64(
                                        (::core::mem::size_of::<u32_0>() as usize)
                                            .wrapping_mul(nOvfl as usize)
                                            as sqlite3_uint64,
                                    )
                                        as *mut u32_0;
                                    if (*pCell).aOvfl.is_null() {
                                        return SQLITE_NOMEM_BKPT;
                                    }
                                    *(*pCell).aOvfl.offset(0 as ::core::ffi::c_int as isize) =
                                        sqlite3Get4byte(
                                            aData.offset((iOff + nLocal) as isize) as *mut u8_0
                                        );
                                    j = 1 as ::core::ffi::c_int;
                                    while j < nOvfl {
                                        let mut rc: ::core::ffi::c_int = 0;
                                        let mut iPrev: u32_0 = *(*pCell)
                                            .aOvfl
                                            .offset((j - 1 as ::core::ffi::c_int) as isize);
                                        let mut pPg: *mut DbPage =
                                            ::core::ptr::null_mut::<DbPage>();
                                        rc = sqlite3PagerGet(
                                            sqlite3BtreePager(pBt) as *mut Pager,
                                            iPrev as Pgno,
                                            &raw mut pPg,
                                            0 as ::core::ffi::c_int,
                                        );
                                        if rc != SQLITE_OK {
                                            return rc;
                                        }
                                        *(*pCell).aOvfl.offset(j as isize) = sqlite3Get4byte(
                                            sqlite3PagerGetData(pPg) as *const u8_0,
                                        );
                                        sqlite3PagerUnref(pPg);
                                        j += 1;
                                    }
                                }
                            }
                            i += 1;
                        }
                    } else {
                        current_block = 5597585068398118923;
                    }
                    match current_block {
                        13503832839001051727 => {}
                        _ => return SQLITE_OK,
                    }
                }
            }
        }
        _ => {}
    }
    (*p).flags = 0 as u8_0;
    statClearCells(p);
    return SQLITE_OK;
}
unsafe extern "C" fn statSizeAndOffset(mut pCsr: *mut StatCursor) {
    let mut pTab: *mut StatTable = (*(pCsr as *mut sqlite3_vtab_cursor)).pVtab as *mut StatTable;
    let mut pBt: *mut Btree = (*(*(*pTab).db).aDb.offset((*pTab).iDb as isize)).pBt;
    let mut pPager: *mut Pager = sqlite3BtreePager(pBt) as *mut Pager;
    let mut fd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
    let mut x: [sqlite3_int64; 2] = [0; 2];
    fd = sqlite3PagerFile(pPager);
    x[0 as ::core::ffi::c_int as usize] = (*pCsr).iPageno as sqlite3_int64;
    if sqlite3OsFileControl(
        fd,
        230440 as ::core::ffi::c_int,
        &raw mut x as *mut ::core::ffi::c_void,
    ) == SQLITE_OK
    {
        (*pCsr).iOffset = x[0 as ::core::ffi::c_int as usize] as i64_0;
        (*pCsr).szPage += x[1 as ::core::ffi::c_int as usize] as sqlite_int64;
    } else {
        (*pCsr).szPage += sqlite3BtreeGetPageSize(pBt) as i64_0;
        (*pCsr).iOffset = (*pCsr).szPage * (*pCsr).iPageno.wrapping_sub(1 as u32_0) as i64_0;
    };
}
unsafe extern "C" fn statGetPage(
    mut pBt: *mut Btree,
    mut iPg: u32_0,
    mut pPg: *mut StatPage,
) -> ::core::ffi::c_int {
    let mut pgsz: ::core::ffi::c_int = sqlite3BtreeGetPageSize(pBt);
    let mut pDbPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
    let mut rc: ::core::ffi::c_int = 0;
    if (*pPg).aPg.is_null() {
        (*pPg).aPg = sqlite3_malloc(pgsz + DBSTAT_PAGE_PADDING_BYTES) as *mut u8_0;
        if (*pPg).aPg.is_null() {
            return SQLITE_NOMEM_BKPT;
        }
        memset(
            (*pPg).aPg.offset(pgsz as isize) as *mut u8_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            DBSTAT_PAGE_PADDING_BYTES as size_t,
        );
    }
    rc = sqlite3PagerGet(
        sqlite3BtreePager(pBt) as *mut Pager,
        iPg as Pgno,
        &raw mut pDbPage,
        0 as ::core::ffi::c_int,
    );
    if rc == SQLITE_OK {
        let mut a: *const u8_0 = sqlite3PagerGetData(pDbPage) as *const u8_0;
        memcpy(
            (*pPg).aPg as *mut ::core::ffi::c_void,
            a as *const ::core::ffi::c_void,
            pgsz as size_t,
        );
        sqlite3PagerUnref(pDbPage);
    }
    return rc;
}
unsafe extern "C" fn statNext(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut nPayload: ::core::ffi::c_int = 0;
    let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    let mut pTab: *mut StatTable = (*pCursor).pVtab as *mut StatTable;
    let mut pBt: *mut Btree = (*(*(*pTab).db).aDb.offset((*pCsr).iDb as isize)).pBt;
    let mut pPager: *mut Pager = sqlite3BtreePager(pBt) as *mut Pager;
    sqlite3_free((*pCsr).zPath as *mut ::core::ffi::c_void);
    (*pCsr).zPath = ::core::ptr::null_mut::<::core::ffi::c_char>();
    loop {
        if (*pCsr).iPage < 0 as ::core::ffi::c_int {
            statResetCounts(pCsr);
            rc = sqlite3_step((*pCsr).pStmt);
            if rc == SQLITE_ROW {
                let mut nPage: ::core::ffi::c_int = 0;
                let mut iRoot: u32_0 =
                    sqlite3_column_int64((*pCsr).pStmt, 1 as ::core::ffi::c_int) as u32_0;
                sqlite3PagerPagecount(pPager, &raw mut nPage);
                if nPage == 0 as ::core::ffi::c_int {
                    (*pCsr).isEof = 1 as u8_0;
                    return sqlite3_reset((*pCsr).pStmt);
                }
                rc = statGetPage(
                    pBt,
                    iRoot,
                    (&raw mut (*pCsr).aPage as *mut StatPage)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut StatPage,
                );
                (*pCsr).aPage[0 as ::core::ffi::c_int as usize].iPgno = iRoot;
                (*pCsr).aPage[0 as ::core::ffi::c_int as usize].iCell = 0 as ::core::ffi::c_int;
                if (*pCsr).isAgg == 0 {
                    z = sqlite3_mprintf(b"/\0" as *const u8 as *const ::core::ffi::c_char);
                    (*pCsr).aPage[0 as ::core::ffi::c_int as usize].zPath = z;
                    if z.is_null() {
                        rc = SQLITE_NOMEM_BKPT;
                    }
                }
                (*pCsr).iPage = 0 as ::core::ffi::c_int;
                (*pCsr).nPage = 1 as ::core::ffi::c_int;
            } else {
                (*pCsr).isEof = 1 as u8_0;
                return sqlite3_reset((*pCsr).pStmt);
            }
        } else {
            let mut p: *mut StatPage = (&raw mut (*pCsr).aPage as *mut StatPage)
                .offset((*pCsr).iPage as isize)
                as *mut StatPage;
            if (*pCsr).isAgg == 0 {
                statResetCounts(pCsr);
            }
            while (*p).iCell < (*p).nCell {
                let mut pCell: *mut StatCell =
                    (*p).aCell.offset((*p).iCell as isize) as *mut StatCell;
                while (*pCell).iOvfl < (*pCell).nOvfl {
                    let mut nUsable: ::core::ffi::c_int = 0;
                    let mut iOvfl: ::core::ffi::c_int = 0;
                    sqlite3BtreeEnter(pBt);
                    nUsable = sqlite3BtreeGetPageSize(pBt) - sqlite3BtreeGetReserveNoMutex(pBt);
                    sqlite3BtreeLeave(pBt);
                    (*pCsr).nPage += 1;
                    statSizeAndOffset(pCsr);
                    if (*pCell).iOvfl < (*pCell).nOvfl - 1 as ::core::ffi::c_int {
                        (*pCsr).nPayload += (nUsable - 4 as ::core::ffi::c_int) as i64_0;
                    } else {
                        (*pCsr).nPayload += (*pCell).nLastOvfl as i64_0;
                        (*pCsr).nUnused +=
                            (nUsable - 4 as ::core::ffi::c_int - (*pCell).nLastOvfl) as i64_0;
                    }
                    iOvfl = (*pCell).iOvfl;
                    (*pCell).iOvfl += 1;
                    if (*pCsr).isAgg == 0 {
                        (*pCsr).zName = sqlite3_column_text((*pCsr).pStmt, 0 as ::core::ffi::c_int)
                            as *mut ::core::ffi::c_char;
                        (*pCsr).iPageno = *(*pCell).aOvfl.offset(iOvfl as isize);
                        (*pCsr).zPagetype = b"overflow\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        z = sqlite3_mprintf(
                            b"%s%.3x+%.6x\0" as *const u8 as *const ::core::ffi::c_char,
                            (*p).zPath,
                            (*p).iCell,
                            iOvfl,
                        );
                        (*pCsr).zPath = z;
                        return if z.is_null() {
                            SQLITE_NOMEM_BKPT
                        } else {
                            SQLITE_OK
                        };
                    }
                }
                if (*p).iRightChildPg != 0 {
                    break;
                }
                (*p).iCell += 1;
            }
            if (*p).iRightChildPg == 0 || (*p).iCell > (*p).nCell {
                statClearPage(p);
                (*pCsr).iPage -= 1;
                if (*pCsr).isAgg as ::core::ffi::c_int != 0
                    && (*pCsr).iPage < 0 as ::core::ffi::c_int
                {
                    return SQLITE_OK;
                }
                continue;
            } else {
                (*pCsr).iPage += 1;
                if (*pCsr).iPage
                    >= (::core::mem::size_of::<[StatPage; 32]>() as usize)
                        .wrapping_div(::core::mem::size_of::<StatPage>() as usize)
                        as ::core::ffi::c_int
                {
                    statResetCsr(pCsr);
                    return sqlite3CorruptError(657 as ::core::ffi::c_int);
                }
                if (*p).iCell == (*p).nCell {
                    (*p.offset(1 as ::core::ffi::c_int as isize)).iPgno = (*p).iRightChildPg;
                } else {
                    (*p.offset(1 as ::core::ffi::c_int as isize)).iPgno =
                        (*(*p).aCell.offset((*p).iCell as isize)).iChildPg;
                }
                rc = statGetPage(
                    pBt,
                    (*p.offset(1 as ::core::ffi::c_int as isize)).iPgno,
                    p.offset(1 as ::core::ffi::c_int as isize) as *mut StatPage,
                );
                (*pCsr).nPage += 1;
                (*p.offset(1 as ::core::ffi::c_int as isize)).iCell = 0 as ::core::ffi::c_int;
                if (*pCsr).isAgg == 0 {
                    z = sqlite3_mprintf(
                        b"%s%.3x/\0" as *const u8 as *const ::core::ffi::c_char,
                        (*p).zPath,
                        (*p).iCell,
                    );
                    let ref mut fresh0 = (*p.offset(1 as ::core::ffi::c_int as isize)).zPath;
                    *fresh0 = z;
                    if z.is_null() {
                        rc = SQLITE_NOMEM_BKPT;
                    }
                }
                (*p).iCell += 1;
            }
        }
        if !(rc == SQLITE_OK) {
            break;
        }
        let mut i: ::core::ffi::c_int = 0;
        let mut p_0: *mut StatPage = (&raw mut (*pCsr).aPage as *mut StatPage)
            .offset((*pCsr).iPage as isize) as *mut StatPage;
        (*pCsr).zName =
            sqlite3_column_text((*pCsr).pStmt, 0 as ::core::ffi::c_int) as *mut ::core::ffi::c_char;
        (*pCsr).iPageno = (*p_0).iPgno;
        rc = statDecodePage(pBt, p_0);
        if !(rc == SQLITE_OK) {
            break;
        }
        statSizeAndOffset(pCsr);
        match (*p_0).flags as ::core::ffi::c_int {
            5 | 2 => {
                (*pCsr).zPagetype = b"internal\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            13 | 10 => {
                (*pCsr).zPagetype = b"leaf\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            _ => {
                (*pCsr).zPagetype = b"corrupted\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
        }
        (*pCsr).nCell += (*p_0).nCell;
        (*pCsr).nUnused += (*p_0).nUnused as i64_0;
        if (*p_0).nMxPayload > (*pCsr).nMxPayload {
            (*pCsr).nMxPayload = (*p_0).nMxPayload;
        }
        if (*pCsr).isAgg == 0 {
            z = sqlite3_mprintf(
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*p_0).zPath,
            );
            (*pCsr).zPath = z;
            if z.is_null() {
                rc = SQLITE_NOMEM_BKPT;
            }
        }
        nPayload = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*p_0).nCell {
            nPayload += (*(*p_0).aCell.offset(i as isize)).nLocal;
            i += 1;
        }
        (*pCsr).nPayload += nPayload as i64_0;
        if !((*pCsr).isAgg != 0) {
            break;
        }
    }
    return rc;
}
unsafe extern "C" fn statEof(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    return (*pCsr).isEof as ::core::ffi::c_int;
}
unsafe extern "C" fn statFilter(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut _idxStr: *const ::core::ffi::c_char,
    mut _argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    let mut pTab: *mut StatTable = (*pCursor).pVtab as *mut StatTable;
    let mut pSql: *mut sqlite3_str = ::core::ptr::null_mut::<sqlite3_str>();
    let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iArg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    statResetCsr(pCsr);
    sqlite3_finalize((*pCsr).pStmt);
    (*pCsr).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if idxNum & 0x1 as ::core::ffi::c_int != 0 {
        let fresh1 = iArg;
        iArg = iArg + 1;
        let mut zDbase: *const ::core::ffi::c_char =
            sqlite3_value_text(*argv.offset(fresh1 as isize)) as *const ::core::ffi::c_char;
        (*pCsr).iDb = sqlite3FindDbName((*pTab).db, zDbase);
        if (*pCsr).iDb < 0 as ::core::ffi::c_int {
            (*pCsr).iDb = 0 as ::core::ffi::c_int;
            (*pCsr).isEof = 1 as u8_0;
            return SQLITE_OK;
        }
    } else {
        (*pCsr).iDb = (*pTab).iDb;
    }
    if idxNum & 0x2 as ::core::ffi::c_int != 0 {
        let fresh2 = iArg;
        iArg = iArg + 1;
        zName = sqlite3_value_text(*argv.offset(fresh2 as isize)) as *const ::core::ffi::c_char;
    }
    if idxNum & 0x4 as ::core::ffi::c_int != 0 {
        let fresh3 = iArg;
        iArg = iArg + 1;
        (*pCsr).isAgg = (sqlite3_value_double(*argv.offset(fresh3 as isize)) != 0.0f64)
            as ::core::ffi::c_int as u8_0;
    } else {
        (*pCsr).isAgg = 0 as u8_0;
    }
    pSql = sqlite3_str_new((*pTab).db);
    sqlite3_str_appendf(
        pSql,
        b"SELECT * FROM (SELECT 'sqlite_schema' AS name,1 AS rootpage,'table' AS type UNION ALL SELECT name,rootpage,type FROM \"%w\".sqlite_schema WHERE rootpage!=0)\0"
            as *const u8 as *const ::core::ffi::c_char,
        (*(*(*pTab).db).aDb.offset((*pCsr).iDb as isize)).zDbSName,
    );
    if !zName.is_null() {
        sqlite3_str_appendf(
            pSql,
            b"WHERE name=%Q\0" as *const u8 as *const ::core::ffi::c_char,
            zName,
        );
    }
    if idxNum & 0x8 as ::core::ffi::c_int != 0 {
        sqlite3_str_appendf(
            pSql,
            b" ORDER BY name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    zSql = sqlite3_str_finish(pSql);
    if zSql.is_null() {
        return SQLITE_NOMEM_BKPT;
    } else {
        rc = sqlite3_prepare_v2(
            (*pTab).db,
            zSql,
            -(1 as ::core::ffi::c_int),
            &raw mut (*pCsr).pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
    }
    if rc == SQLITE_OK {
        (*pCsr).iPage = -(1 as ::core::ffi::c_int);
        rc = statNext(pCursor);
    }
    return rc;
}
unsafe extern "C" fn statColumn(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    match i {
        0 => {
            sqlite3_result_text(
                ctx,
                (*pCsr).zName,
                -(1 as ::core::ffi::c_int),
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
        }
        1 => {
            if (*pCsr).isAgg == 0 {
                sqlite3_result_text(
                    ctx,
                    (*pCsr).zPath,
                    -(1 as ::core::ffi::c_int),
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
                );
            }
        }
        2 => {
            if (*pCsr).isAgg != 0 {
                sqlite3_result_int64(ctx, (*pCsr).nPage as sqlite3_int64);
            } else {
                sqlite3_result_int64(ctx, (*pCsr).iPageno as sqlite3_int64);
            }
        }
        3 => {
            if (*pCsr).isAgg == 0 {
                sqlite3_result_text(
                    ctx,
                    (*pCsr).zPagetype,
                    -(1 as ::core::ffi::c_int),
                    SQLITE_STATIC,
                );
            }
        }
        4 => {
            sqlite3_result_int64(ctx, (*pCsr).nCell as sqlite3_int64);
        }
        5 => {
            sqlite3_result_int64(ctx, (*pCsr).nPayload as sqlite3_int64);
        }
        6 => {
            sqlite3_result_int64(ctx, (*pCsr).nUnused as sqlite3_int64);
        }
        7 => {
            sqlite3_result_int64(ctx, (*pCsr).nMxPayload as sqlite3_int64);
        }
        8 => {
            if (*pCsr).isAgg == 0 {
                sqlite3_result_int64(ctx, (*pCsr).iOffset as sqlite3_int64);
            }
        }
        9 => {
            sqlite3_result_int64(ctx, (*pCsr).szPage as sqlite3_int64);
        }
        10 => {
            let mut db: *mut sqlite3 = sqlite3_context_db_handle(ctx);
            let mut iDb: ::core::ffi::c_int = (*pCsr).iDb;
            sqlite3_result_text(
                ctx,
                (*(*db).aDb.offset(iDb as isize)).zDbSName,
                -(1 as ::core::ffi::c_int),
                SQLITE_STATIC,
            );
        }
        _ => {
            sqlite3_result_int(ctx, (*pCsr).isAgg as ::core::ffi::c_int);
        }
    }
    return SQLITE_OK;
}
unsafe extern "C" fn statRowid(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    *pRowid = (*pCsr).iPageno as sqlite_int64;
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DbstatRegister(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    static mut dbstat_module: sqlite3_module = unsafe {
        sqlite3_module {
            iVersion: 0 as ::core::ffi::c_int,
            xCreate: Some(
                statConnect
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
                statConnect
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
                statBestIndex
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab,
                        *mut sqlite3_index_info,
                    ) -> ::core::ffi::c_int,
            ),
            xDisconnect: Some(
                statDisconnect as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
            ),
            xDestroy: Some(
                statDisconnect as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
            ),
            xOpen: Some(
                statOpen
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab,
                        *mut *mut sqlite3_vtab_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xClose: Some(
                statClose as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xFilter: Some(
                statFilter
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab_cursor,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> ::core::ffi::c_int,
            ),
            xNext: Some(
                statNext as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xEof: Some(
                statEof as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xColumn: Some(
                statColumn
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab_cursor,
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xRowid: Some(
                statRowid
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
    return sqlite3_create_module(
        db,
        b"dbstat\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut dbstat_module,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
