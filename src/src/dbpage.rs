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
    pub type Pager;
    pub type PCache;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_zeroblob(_: *mut sqlite3_context, n: ::core::ffi::c_int);
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
    fn sqlite3PagerGet(
        pPager: *mut Pager,
        pgno: Pgno,
        ppPage: *mut *mut DbPage,
        clrFlag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerUnref(_: *mut DbPage);
    fn sqlite3PagerUnrefPageOne(_: *mut DbPage);
    fn sqlite3PagerWrite(_: *mut DbPage) -> ::core::ffi::c_int;
    fn sqlite3PagerGetData(_: *mut DbPage) -> *mut ::core::ffi::c_void;
    fn sqlite3PagerTruncateImage(_: *mut Pager, _: Pgno);
    fn sqlite3BtreeGetPageSize(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeLastPage(_: *mut Btree) -> Pgno;
    fn sqlite3BtreeBeginTrans(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnter(_: *mut Btree);
    fn sqlite3BtreeLeave(_: *mut Btree);
    static mut sqlite3PendingByte: ::core::ffi::c_int;
    fn sqlite3FindDbName(_: *mut sqlite3, _: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub struct DbpageTable {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub iDbTrunc: ::core::ffi::c_int,
    pub pgnoTrunc: Pgno,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbpageCursor {
    pub base: sqlite3_vtab_cursor,
    pub pgno: Pgno,
    pub mxPgno: Pgno,
    pub pPager: *mut Pager,
    pub pPage1: *mut DbPage,
    pub iDb: ::core::ffi::c_int,
    pub szPage: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_SCAN_UNIQUE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_VTAB_DIRECTONLY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_VTAB_USES_ALL_SCHEMAS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_Defensive: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = SQLITE_NOMEM;
pub const DBPAGE_COLUMN_SCHEMA: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn dbpageConnect(
    mut db: *mut sqlite3,
    mut _pAux: *mut ::core::ffi::c_void,
    mut _argc: ::core::ffi::c_int,
    mut _argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut _pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pTab: *mut DbpageTable = ::core::ptr::null_mut::<DbpageTable>();
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    sqlite3_vtab_config(db, SQLITE_VTAB_DIRECTONLY);
    sqlite3_vtab_config(db, SQLITE_VTAB_USES_ALL_SCHEMAS);
    rc = sqlite3_declare_vtab(
        db,
        b"CREATE TABLE x(pgno INTEGER PRIMARY KEY, data BLOB, schema HIDDEN)\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if rc == SQLITE_OK {
        pTab = sqlite3_malloc64(::core::mem::size_of::<DbpageTable>() as sqlite3_uint64)
            as *mut DbpageTable;
        if pTab.is_null() {
            rc = SQLITE_NOMEM_BKPT;
        }
    }
    if rc == SQLITE_OK {
        memset(
            pTab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<DbpageTable>() as size_t,
        );
        (*pTab).db = db;
    }
    *ppVtab = pTab as *mut sqlite3_vtab;
    return rc;
}
unsafe extern "C" fn dbpageDisconnect(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    sqlite3_free(pVtab as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn dbpageBestIndex(
    mut _tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut iPlan: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*pIdxInfo).nConstraint {
        let mut p: *mut sqlite3_index_constraint =
            (*pIdxInfo).aConstraint.offset(i as isize) as *mut sqlite3_index_constraint;
        if !((*p).iColumn != DBPAGE_COLUMN_SCHEMA) {
            if !((*p).op as ::core::ffi::c_int != SQLITE_INDEX_CONSTRAINT_EQ) {
                if (*p).usable == 0 {
                    return SQLITE_CONSTRAINT;
                }
                iPlan = 2 as ::core::ffi::c_int;
                (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).argvIndex =
                    1 as ::core::ffi::c_int;
                (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).omit = 1 as ::core::ffi::c_uchar;
                break;
            }
        }
        i += 1;
    }
    (*pIdxInfo).estimatedCost = 1.0e6f64;
    i = 0 as ::core::ffi::c_int;
    while i < (*pIdxInfo).nConstraint {
        let mut p_0: *mut sqlite3_index_constraint =
            (*pIdxInfo).aConstraint.offset(i as isize) as *mut sqlite3_index_constraint;
        if (*p_0).usable as ::core::ffi::c_int != 0
            && (*p_0).iColumn <= 0 as ::core::ffi::c_int
            && (*p_0).op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_EQ
        {
            (*pIdxInfo).estimatedRows = 1 as sqlite3_int64;
            (*pIdxInfo).idxFlags = SQLITE_INDEX_SCAN_UNIQUE;
            (*pIdxInfo).estimatedCost = 1.0f64;
            (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).argvIndex = if iPlan != 0 {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            (*(*pIdxInfo).aConstraintUsage.offset(i as isize)).omit = 1 as ::core::ffi::c_uchar;
            iPlan |= 1 as ::core::ffi::c_int;
            break;
        } else {
            i += 1;
        }
    }
    (*pIdxInfo).idxNum = iPlan;
    if (*pIdxInfo).nOrderBy >= 1 as ::core::ffi::c_int
        && (*(*pIdxInfo)
            .aOrderBy
            .offset(0 as ::core::ffi::c_int as isize))
        .iColumn
            <= 0 as ::core::ffi::c_int
        && (*(*pIdxInfo)
            .aOrderBy
            .offset(0 as ::core::ffi::c_int as isize))
        .desc as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        (*pIdxInfo).orderByConsumed = 1 as ::core::ffi::c_int;
    }
    return SQLITE_OK;
}
unsafe extern "C" fn dbpageOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut DbpageCursor = ::core::ptr::null_mut::<DbpageCursor>();
    pCsr = sqlite3_malloc64(::core::mem::size_of::<DbpageCursor>() as sqlite3_uint64)
        as *mut DbpageCursor;
    if pCsr.is_null() {
        return SQLITE_NOMEM_BKPT;
    } else {
        memset(
            pCsr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<DbpageCursor>() as size_t,
        );
        (*pCsr).base.pVtab = pVTab;
        (*pCsr).pgno = 0 as Pgno;
    }
    *ppCursor = pCsr as *mut sqlite3_vtab_cursor;
    return SQLITE_OK;
}
unsafe extern "C" fn dbpageClose(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    if !(*pCsr).pPage1.is_null() {
        sqlite3PagerUnrefPageOne((*pCsr).pPage1);
    }
    sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn dbpageNext(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    (*pCsr).pgno = (*pCsr).pgno.wrapping_add(1);
    return rc;
}
unsafe extern "C" fn dbpageEof(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    return ((*pCsr).pgno > (*pCsr).mxPgno) as ::core::ffi::c_int;
}
unsafe extern "C" fn dbpageFilter(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut _idxStr: *const ::core::ffi::c_char,
    mut _argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    let mut pTab: *mut DbpageTable = (*pCursor).pVtab as *mut DbpageTable;
    let mut rc: ::core::ffi::c_int = 0;
    let mut db: *mut sqlite3 = (*pTab).db;
    let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
    (*pCsr).pgno = 1 as Pgno;
    (*pCsr).mxPgno = 0 as Pgno;
    if idxNum & 2 as ::core::ffi::c_int != 0 {
        let mut zSchema: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        zSchema = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        (*pCsr).iDb = sqlite3FindDbName(db, zSchema);
        if (*pCsr).iDb < 0 as ::core::ffi::c_int {
            return SQLITE_OK;
        }
    } else {
        (*pCsr).iDb = 0 as ::core::ffi::c_int;
    }
    pBt = (*(*db).aDb.offset((*pCsr).iDb as isize)).pBt;
    if pBt.is_null() {
        return SQLITE_OK;
    }
    (*pCsr).pPager = sqlite3BtreePager(pBt) as *mut Pager;
    (*pCsr).szPage = sqlite3BtreeGetPageSize(pBt);
    (*pCsr).mxPgno = sqlite3BtreeLastPage(pBt);
    if idxNum & 1 as ::core::ffi::c_int != 0 {
        (*pCsr).pgno =
            sqlite3_value_int(*argv.offset((idxNum >> 1 as ::core::ffi::c_int) as isize)) as Pgno;
        if (*pCsr).pgno < 1 as Pgno || (*pCsr).pgno > (*pCsr).mxPgno {
            (*pCsr).pgno = 1 as Pgno;
            (*pCsr).mxPgno = 0 as Pgno;
        } else {
            (*pCsr).mxPgno = (*pCsr).pgno;
        }
    }
    if !(*pCsr).pPage1.is_null() {
        sqlite3PagerUnrefPageOne((*pCsr).pPage1);
    }
    rc = sqlite3PagerGet(
        (*pCsr).pPager,
        1 as Pgno,
        &raw mut (*pCsr).pPage1,
        0 as ::core::ffi::c_int,
    );
    return rc;
}
unsafe extern "C" fn dbpageColumn(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    match i {
        0 => {
            sqlite3_result_int64(ctx, (*pCsr).pgno as sqlite3_int64);
        }
        1 => {
            let mut pDbPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
            if (*pCsr).pgno
                == (sqlite3PendingByte / (*pCsr).szPage + 1 as ::core::ffi::c_int) as Pgno
            {
                sqlite3_result_zeroblob(ctx, (*pCsr).szPage);
            } else {
                rc = sqlite3PagerGet(
                    (*pCsr).pPager,
                    (*pCsr).pgno,
                    &raw mut pDbPage,
                    0 as ::core::ffi::c_int,
                );
                if rc == SQLITE_OK {
                    sqlite3_result_blob(
                        ctx,
                        sqlite3PagerGetData(pDbPage),
                        (*pCsr).szPage,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
                    );
                }
                sqlite3PagerUnref(pDbPage);
            }
        }
        _ => {
            let mut db: *mut sqlite3 = sqlite3_context_db_handle(ctx);
            sqlite3_result_text(
                ctx,
                (*(*db).aDb.offset((*pCsr).iDb as isize)).zDbSName,
                -(1 as ::core::ffi::c_int),
                SQLITE_STATIC,
            );
        }
    }
    return rc;
}
unsafe extern "C" fn dbpageRowid(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    *pRowid = (*pCsr).pgno as sqlite_int64;
    return SQLITE_OK;
}
unsafe extern "C" fn dbpageBeginTrans(mut pTab: *mut DbpageTable) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = (*pTab).db;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while rc == SQLITE_OK && i < (*db).nDb {
        let mut pBt: *mut Btree = (*(*db).aDb.offset(i as isize)).pBt;
        if !pBt.is_null() {
            rc = sqlite3BtreeBeginTrans(
                pBt,
                1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        }
        i += 1;
    }
    return rc;
}
unsafe extern "C" fn dbpageUpdate(
    mut pVtab: *mut sqlite3_vtab,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
    mut _pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pTab: *mut DbpageTable = pVtab as *mut DbpageTable;
    let mut pgno: Pgno = 0;
    let mut pDbPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iDb: ::core::ffi::c_int = 0;
    let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
    let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
    let mut szPage: ::core::ffi::c_int = 0;
    let mut isInsert: ::core::ffi::c_int = 0;
    if (*(*pTab).db).flags & SQLITE_Defensive as u64_0 != 0 {
        zErr =
            b"read-only\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else if argc == 1 as ::core::ffi::c_int {
        zErr = b"cannot delete\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    } else {
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize)) == SQLITE_NULL {
            pgno = sqlite3_value_int64(*argv.offset(2 as ::core::ffi::c_int as isize)) as Pgno;
            isInsert = 1 as ::core::ffi::c_int;
            current_block = 7976072742316086414;
        } else {
            pgno = sqlite3_value_int64(*argv.offset(0 as ::core::ffi::c_int as isize)) as Pgno;
            if sqlite3_value_int(*argv.offset(1 as ::core::ffi::c_int as isize)) as Pgno != pgno {
                zErr = b"cannot insert\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                current_block = 12876300494006893209;
            } else {
                isInsert = 0 as ::core::ffi::c_int;
                current_block = 7976072742316086414;
            }
        }
        match current_block {
            12876300494006893209 => {}
            _ => {
                if sqlite3_value_type(*argv.offset(4 as ::core::ffi::c_int as isize)) == SQLITE_NULL
                {
                    iDb = 0 as ::core::ffi::c_int;
                    current_block = 5601891728916014340;
                } else {
                    let mut zSchema: *const ::core::ffi::c_char =
                        sqlite3_value_text(*argv.offset(4 as ::core::ffi::c_int as isize))
                            as *const ::core::ffi::c_char;
                    iDb = sqlite3FindDbName((*pTab).db, zSchema);
                    if iDb < 0 as ::core::ffi::c_int {
                        zErr = b"no such schema\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        current_block = 12876300494006893209;
                    } else {
                        current_block = 5601891728916014340;
                    }
                }
                match current_block {
                    12876300494006893209 => {}
                    _ => {
                        pBt = (*(*(*pTab).db).aDb.offset(iDb as isize)).pBt;
                        if pgno < 1 as Pgno || pBt.is_null() {
                            zErr = b"bad page number\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        } else {
                            szPage = sqlite3BtreeGetPageSize(pBt);
                            if sqlite3_value_type(*argv.offset(3 as ::core::ffi::c_int as isize))
                                != SQLITE_BLOB
                                || sqlite3_value_bytes(
                                    *argv.offset(3 as ::core::ffi::c_int as isize),
                                ) != szPage
                            {
                                if sqlite3_value_type(
                                    *argv.offset(3 as ::core::ffi::c_int as isize),
                                ) == SQLITE_NULL
                                    && isInsert != 0
                                    && pgno > 1 as Pgno
                                {
                                    (*pTab).iDbTrunc = iDb;
                                    (*pTab).pgnoTrunc = pgno.wrapping_sub(1 as Pgno);
                                    pgno = 1 as Pgno;
                                    current_block = 8693738493027456495;
                                } else {
                                    zErr = b"bad page value\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    current_block = 12876300494006893209;
                                }
                            } else {
                                current_block = 8693738493027456495;
                            }
                            match current_block {
                                12876300494006893209 => {}
                                _ => {
                                    if dbpageBeginTrans(pTab) != SQLITE_OK {
                                        zErr = b"failed to open transaction\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char;
                                    } else {
                                        pPager = sqlite3BtreePager(pBt) as *mut Pager;
                                        rc = sqlite3PagerGet(
                                            pPager,
                                            pgno,
                                            &raw mut pDbPage,
                                            0 as ::core::ffi::c_int,
                                        );
                                        if rc == SQLITE_OK {
                                            let mut pData: *const ::core::ffi::c_void =
                                                sqlite3_value_blob(
                                                    *argv.offset(3 as ::core::ffi::c_int as isize),
                                                );
                                            rc = sqlite3PagerWrite(pDbPage);
                                            if rc == SQLITE_OK && !pData.is_null() {
                                                let mut aPage: *mut ::core::ffi::c_uchar =
                                                    sqlite3PagerGetData(pDbPage)
                                                        as *mut ::core::ffi::c_uchar;
                                                memcpy(
                                                    aPage as *mut ::core::ffi::c_void,
                                                    pData,
                                                    szPage as size_t,
                                                );
                                                (*pTab).pgnoTrunc = 0 as Pgno;
                                            }
                                        }
                                        if rc != SQLITE_OK {
                                            (*pTab).pgnoTrunc = 0 as Pgno;
                                        }
                                        sqlite3PagerUnref(pDbPage);
                                        return rc;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    (*pTab).pgnoTrunc = 0 as Pgno;
    sqlite3_free((*pVtab).zErrMsg as *mut ::core::ffi::c_void);
    (*pVtab).zErrMsg = sqlite3_mprintf(b"%s\0" as *const u8 as *const ::core::ffi::c_char, zErr);
    return SQLITE_ERROR;
}
unsafe extern "C" fn dbpageBegin(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    let mut pTab: *mut DbpageTable = pVtab as *mut DbpageTable;
    (*pTab).pgnoTrunc = 0 as Pgno;
    return SQLITE_OK;
}
unsafe extern "C" fn dbpageSync(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    let mut pTab: *mut DbpageTable = pVtab as *mut DbpageTable;
    if (*pTab).pgnoTrunc > 0 as Pgno {
        let mut pBt: *mut Btree = (*(*(*pTab).db).aDb.offset((*pTab).iDbTrunc as isize)).pBt;
        let mut pPager: *mut Pager = sqlite3BtreePager(pBt) as *mut Pager;
        sqlite3BtreeEnter(pBt);
        if (*pTab).pgnoTrunc < sqlite3BtreeLastPage(pBt) {
            sqlite3PagerTruncateImage(pPager, (*pTab).pgnoTrunc);
        }
        sqlite3BtreeLeave(pBt);
    }
    (*pTab).pgnoTrunc = 0 as Pgno;
    return SQLITE_OK;
}
unsafe extern "C" fn dbpageRollbackTo(
    mut pVtab: *mut sqlite3_vtab,
    mut _notUsed1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pTab: *mut DbpageTable = pVtab as *mut DbpageTable;
    (*pTab).pgnoTrunc = 0 as Pgno;
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3DbpageRegister(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    static mut dbpage_module: sqlite3_module = unsafe {
        sqlite3_module {
            iVersion: 2 as ::core::ffi::c_int,
            xCreate: Some(
                dbpageConnect
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
                dbpageConnect
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
                dbpageBestIndex
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab,
                        *mut sqlite3_index_info,
                    ) -> ::core::ffi::c_int,
            ),
            xDisconnect: Some(
                dbpageDisconnect as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
            ),
            xDestroy: Some(
                dbpageDisconnect as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
            ),
            xOpen: Some(
                dbpageOpen
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab,
                        *mut *mut sqlite3_vtab_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xClose: Some(
                dbpageClose as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xFilter: Some(
                dbpageFilter
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab_cursor,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> ::core::ffi::c_int,
            ),
            xNext: Some(
                dbpageNext as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xEof: Some(
                dbpageEof as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xColumn: Some(
                dbpageColumn
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab_cursor,
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xRowid: Some(
                dbpageRowid
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab_cursor,
                        *mut sqlite_int64,
                    ) -> ::core::ffi::c_int,
            ),
            xUpdate: Some(
                dbpageUpdate
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                        *mut sqlite_int64,
                    ) -> ::core::ffi::c_int,
            ),
            xBegin: Some(
                dbpageBegin as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
            ),
            xSync: Some(
                dbpageSync as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
            ),
            xCommit: None,
            xRollback: None,
            xFindFunction: None,
            xRename: None,
            xSavepoint: None,
            xRelease: None,
            xRollbackTo: Some(
                dbpageRollbackTo
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xShadowName: None,
            xIntegrity: None,
        }
    };
    return sqlite3_create_module(
        db,
        b"sqlite_dbpage\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut dbpage_module,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
