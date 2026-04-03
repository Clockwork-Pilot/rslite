use ::c2rust_bitfields;
unsafe extern "C" {
    fn sqlite3TestMakePointerStr(
        interp: *mut Tcl_Interp,
        zBuf: *mut ::core::ffi::c_char,
        p: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
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
    pub type Tcl_Command_;
    fn sqlite3_close(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_open(
        filename: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
    ) -> ::core::ffi::c_int;
    fn sqlite3_errcode(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_name(
        _: *mut sqlite3_stmt,
        N: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_data_count(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_thread_cleanup();
    fn sqlite3_mutex_alloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn Tcl_GetInt(
        interp: *mut Tcl_Interp,
        src: *const ::core::ffi::c_char,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_CmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn sqlite3TestTextToPtr(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sched_yield() -> ::core::ffi::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_detach(__th: pthread_t) -> ::core::ffi::c_int;
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
    pub trace: C2Rust_Unnamed_21,
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
    pub u1: C2Rust_Unnamed_17,
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
    pub u: C2Rust_Unnamed_13,
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
    pub fg: C2Rust_Unnamed_12,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: Bitmask,
    pub u1: C2Rust_Unnamed_11,
    pub u2: C2Rust_Unnamed_10,
    pub u3: C2Rust_Unnamed_9,
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
    pub u: C2Rust_Unnamed_8,
    pub pLeft: *mut Expr,
    pub pRight: *mut Expr,
    pub x: C2Rust_Unnamed_7,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ynVar,
    pub iAgg: i16_0,
    pub w: C2Rust_Unnamed_6,
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
    pub fg: C2Rust_Unnamed_5,
    pub u: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub x: C2Rust_Unnamed_4,
    pub iConstExprReg: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub iOrderByCol: u16_0,
    pub iAlias: u16_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
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
pub union C2Rust_Unnamed_6 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int,
}
pub type ynVar = i16_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub pList: *mut ExprList,
    pub pSelect: *mut Select,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
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
pub union C2Rust_Unnamed_9 {
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
pub union C2Rust_Unnamed_10 {
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
    pub idxType_bUnordered_uniqNotNull_isResized_isCovering_noSkipScan_hasStat1_bNoQuery_bAscKeyBug_bHasVCol_bHasExpr: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub colNotIdxed: Bitmask,
}
pub type Bitmask = u64_0;
pub type Pgno = u32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut ExprList,
    pub nRow: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
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
pub union C2Rust_Unnamed_13 {
    pub tab: C2Rust_Unnamed_16,
    pub view: C2Rust_Unnamed_15,
    pub vtab: C2Rust_Unnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub pSelect: *mut Select,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
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
pub union C2Rust_Unnamed_17 {
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
    pub u1: C2Rust_Unnamed_18,
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
pub union C2Rust_Unnamed_18 {
    pub cr: C2Rust_Unnamed_20,
    pub d: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
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
pub struct C2Rust_Unnamed_20 {
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
pub union C2Rust_Unnamed_21 {
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
pub type C2Rust_Unnamed_22 = ::core::ffi::c_uint;
pub const _ISalnum: C2Rust_Unnamed_22 = 8;
pub const _ISpunct: C2Rust_Unnamed_22 = 4;
pub const _IScntrl: C2Rust_Unnamed_22 = 2;
pub const _ISblank: C2Rust_Unnamed_22 = 1;
pub const _ISgraph: C2Rust_Unnamed_22 = 32768;
pub const _ISprint: C2Rust_Unnamed_22 = 16384;
pub const _ISspace: C2Rust_Unnamed_22 = 8192;
pub const _ISxdigit: C2Rust_Unnamed_22 = 4096;
pub const _ISdigit: C2Rust_Unnamed_22 = 2048;
pub const _ISalpha: C2Rust_Unnamed_22 = 1024;
pub const _ISlower: C2Rust_Unnamed_22 = 512;
pub const _ISupper: C2Rust_Unnamed_22 = 256;
pub type ClientData = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut ::core::ffi::c_char,
    pub freeProcDontUse: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ()>,
    pub errorLineDontUse: ::core::ffi::c_int,
}
pub type Tcl_Command = *mut Tcl_Command_;
pub type Tcl_CmdDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_CmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Thread {
    pub zFilename: *mut ::core::ffi::c_char,
    pub xOp: Option<unsafe extern "C" fn(*mut Thread) -> ()>,
    pub zArg: *mut ::core::ffi::c_char,
    pub opnum: ::core::ffi::c_int,
    pub busy: ::core::ffi::c_int,
    pub completed: ::core::ffi::c_int,
    pub db: *mut sqlite3,
    pub pStmt: *mut sqlite3_stmt,
    pub zErr: *mut ::core::ffi::c_char,
    pub zStaticErr: *mut ::core::ffi::c_char,
    pub rc: ::core::ffi::c_int,
    pub argc: ::core::ffi::c_int,
    pub argv: [*const ::core::ffi::c_char; 100],
    pub colv: [*const ::core::ffi::c_char; 100],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_CmdProc>,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_STATIC_APP1: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const N_THREAD: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
static mut threadset: [Thread; 26] = [Thread {
    zFilename: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    xOp: None,
    zArg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    opnum: 0,
    busy: 0,
    completed: 0,
    db: ::core::ptr::null::<sqlite3>() as *mut sqlite3,
    pStmt: ::core::ptr::null::<sqlite3_stmt>() as *mut sqlite3_stmt,
    zErr: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    zStaticErr: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    rc: 0,
    argc: 0,
    argv: [::core::ptr::null::<::core::ffi::c_char>(); 100],
    colv: [::core::ptr::null::<::core::ffi::c_char>(); 100],
}; 26];
unsafe extern "C" fn test_barrier() {
    unsafe {
        let mut pMutex: *mut sqlite3_mutex = sqlite3_mutex_alloc(
            SQLITE_MUTEX_STATIC_APP1,
        );
        sqlite3_mutex_enter(pMutex);
        sqlite3_mutex_leave(pMutex);
    }
}
unsafe extern "C" fn test_thread_main(
    mut pArg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut p: *mut Thread = pArg as *mut Thread;
        if !(*p).db.is_null() {
            sqlite3_close((*p).db);
        }
        sqlite3_open((*p).zFilename, &raw mut (*p).db);
        if SQLITE_OK != sqlite3_errcode((*p).db) {
            (*p).zErr = strdup(sqlite3_errmsg((*p).db));
            sqlite3_close((*p).db);
            (*p).db = ::core::ptr::null_mut::<sqlite3>();
        }
        (*p).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        test_barrier();
        (*p).completed = 1 as ::core::ffi::c_int;
        while (*p).opnum <= (*p).completed {
            sched_yield();
        }
        test_barrier();
        while (*p).xOp.is_some() {
            if !(*p).zErr.is_null() && (*p).zErr != (*p).zStaticErr {
                sqlite3_free((*p).zErr as *mut ::core::ffi::c_void);
                (*p).zErr = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            Some((*p).xOp.expect("non-null function pointer"))
                .expect("non-null function pointer")(p);
            test_barrier();
            (*p).completed += 1;
            while (*p).opnum <= (*p).completed {
                sched_yield();
            }
            test_barrier();
        }
        if !(*p).pStmt.is_null() {
            sqlite3_finalize((*p).pStmt);
            (*p).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
        if !(*p).db.is_null() {
            sqlite3_close((*p).db);
            (*p).db = ::core::ptr::null_mut::<sqlite3>();
        }
        if !(*p).zErr.is_null() && (*p).zErr != (*p).zStaticErr {
            sqlite3_free((*p).zErr as *mut ::core::ffi::c_void);
            (*p).zErr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        test_barrier();
        (*p).completed += 1;
        sqlite3_thread_cleanup();
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
}
unsafe extern "C" fn parse_thread_id(
    mut interp: *mut Tcl_Interp,
    mut zArg: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if zArg.is_null()
            || *zArg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            || *zArg.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            || *(*__ctype_b_loc())
                .offset(
                    *zArg.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                ) as ::core::ffi::c_int
                & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int == 0
        {
            Tcl_AppendResult(
                interp,
                b"thread ID must be an upper case letter\0".as_ptr()
                    as *const ::core::ffi::c_char,
                NULL,
            );
            return -1 as ::core::ffi::c_int;
        }
        return *zArg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - 'A' as i32;
    }
}
unsafe extern "C" fn tcl_thread_create(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut x: pthread_t = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy != 0 {
            Tcl_AppendResult(
                interp,
                b"thread \0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(1 as ::core::ffi::c_int as isize),
                b" is already running\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        threadset[i as usize].busy = 1 as ::core::ffi::c_int;
        sqlite3_free(threadset[i as usize].zFilename as *mut ::core::ffi::c_void);
        threadset[i as usize].zFilename = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(2 as ::core::ffi::c_int as isize),
        );
        threadset[i as usize].opnum = 1 as ::core::ffi::c_int;
        threadset[i as usize].completed = 0 as ::core::ffi::c_int;
        rc = pthread_create(
            &raw mut x,
            ::core::ptr::null::<pthread_attr_t>(),
            Some(
                test_thread_main
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                    ) -> *mut ::core::ffi::c_void,
            ),
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread
                as *mut ::core::ffi::c_void,
        );
        if rc != 0 {
            Tcl_AppendResult(
                interp,
                b"failed to create the thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            sqlite3_free(threadset[i as usize].zFilename as *mut ::core::ffi::c_void);
            threadset[i as usize].busy = 0 as ::core::ffi::c_int;
            return TCL_ERROR;
        }
        pthread_detach(x);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_thread_wait(mut p: *mut Thread) {
    unsafe {
        test_barrier();
        while (*p).opnum > (*p).completed {
            sched_yield();
        }
        test_barrier();
    }
}
unsafe extern "C" fn tcl_thread_wait(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_stop_thread(mut p: *mut Thread) {
    unsafe {
        test_thread_wait(p);
        (*p).xOp = None;
        (*p).opnum += 1;
        test_thread_wait(p);
        sqlite3_free((*p).zArg as *mut ::core::ffi::c_void);
        (*p).zArg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sqlite3_free((*p).zFilename as *mut ::core::ffi::c_void);
        (*p).zFilename = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*p).busy = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn tcl_thread_halt(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if *(*argv.offset(1 as ::core::ffi::c_int as isize))
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '*' as i32
            && *(*argv.offset(1 as ::core::ffi::c_int as isize))
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            i = 0 as ::core::ffi::c_int;
            while i < N_THREAD {
                if threadset[i as usize].busy != 0 {
                    test_stop_thread(
                        (&raw mut threadset as *mut Thread).offset(i as isize)
                            as *mut Thread,
                    );
                }
                i += 1;
            }
        } else {
            i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
            if i < 0 as ::core::ffi::c_int {
                return TCL_ERROR;
            }
            if threadset[i as usize].busy == 0 {
                Tcl_AppendResult(
                    interp,
                    b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                    NULL,
                );
                return TCL_ERROR;
            }
            test_stop_thread(
                (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn tcl_thread_argc(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            threadset[i as usize].argc,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn tcl_thread_argv(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID N\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if Tcl_GetInt(interp, *argv.offset(2 as ::core::ffi::c_int as isize), &raw mut n)
            != 0
        {
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        if n < 0 as ::core::ffi::c_int || n >= threadset[i as usize].argc {
            Tcl_AppendResult(
                interp,
                b"column number out of range\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        Tcl_AppendResult(interp, threadset[i as usize].argv[n as usize], NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn tcl_thread_colname(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID N\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if Tcl_GetInt(interp, *argv.offset(2 as ::core::ffi::c_int as isize), &raw mut n)
            != 0
        {
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        if n < 0 as ::core::ffi::c_int || n >= threadset[i as usize].argc {
            Tcl_AppendResult(
                interp,
                b"column number out of range\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        Tcl_AppendResult(interp, threadset[i as usize].colv[n as usize], NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn tcl_thread_result(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        zName = sqlite3ErrName(threadset[i as usize].rc);
        Tcl_AppendResult(interp, zName, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn tcl_thread_error(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        Tcl_AppendResult(interp, threadset[i as usize].zErr, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn do_compile(mut p: *mut Thread) {
    unsafe {
        if (*p).db.is_null() {
            (*p).zStaticErr = b"no database is open\0".as_ptr()
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*p).zErr = (*p).zStaticErr;
            (*p).rc = SQLITE_ERROR;
            return;
        }
        if !(*p).pStmt.is_null() {
            sqlite3_finalize((*p).pStmt);
            (*p).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
        (*p).rc = sqlite3_prepare(
            (*p).db,
            (*p).zArg,
            -1 as ::core::ffi::c_int,
            &raw mut (*p).pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
    }
}
unsafe extern "C" fn tcl_thread_compile(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID SQL\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        threadset[i as usize].xOp = Some(
            do_compile as unsafe extern "C" fn(*mut Thread) -> (),
        ) as Option<unsafe extern "C" fn(*mut Thread) -> ()>;
        sqlite3_free(threadset[i as usize].zArg as *mut ::core::ffi::c_void);
        threadset[i as usize].zArg = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(2 as ::core::ffi::c_int as isize),
        );
        test_barrier();
        threadset[i as usize].opnum += 1;
        return TCL_OK;
    }
}
unsafe extern "C" fn do_step(mut p: *mut Thread) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if (*p).pStmt.is_null() {
            (*p).zStaticErr = b"no virtual machine available\0".as_ptr()
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*p).zErr = (*p).zStaticErr;
            (*p).rc = SQLITE_ERROR;
            return;
        }
        (*p).rc = sqlite3_step((*p).pStmt);
        if (*p).rc == SQLITE_ROW {
            (*p).argc = sqlite3_column_count((*p).pStmt);
            i = 0 as ::core::ffi::c_int;
            while i < sqlite3_data_count((*p).pStmt) {
                (*p).argv[i as usize] = sqlite3_column_text((*p).pStmt, i)
                    as *mut ::core::ffi::c_char;
                i += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*p).argc {
                (*p).colv[i as usize] = sqlite3_column_name((*p).pStmt, i);
                i += 1;
            }
        }
    }
}
unsafe extern "C" fn tcl_thread_step(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" IDL\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        threadset[i as usize].xOp = Some(
            do_step as unsafe extern "C" fn(*mut Thread) -> (),
        ) as Option<unsafe extern "C" fn(*mut Thread) -> ()>;
        test_barrier();
        threadset[i as usize].opnum += 1;
        return TCL_OK;
    }
}
unsafe extern "C" fn do_finalize(mut p: *mut Thread) {
    unsafe {
        if (*p).pStmt.is_null() {
            (*p).zStaticErr = b"no virtual machine available\0".as_ptr()
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*p).zErr = (*p).zStaticErr;
            (*p).rc = SQLITE_ERROR;
            return;
        }
        (*p).rc = sqlite3_finalize((*p).pStmt);
        (*p).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
}
unsafe extern "C" fn tcl_thread_finalize(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" IDL\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        threadset[i as usize].xOp = Some(
            do_finalize as unsafe extern "C" fn(*mut Thread) -> (),
        ) as Option<unsafe extern "C" fn(*mut Thread) -> ()>;
        sqlite3_free(threadset[i as usize].zArg as *mut ::core::ffi::c_void);
        threadset[i as usize].zArg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        test_barrier();
        threadset[i as usize].opnum += 1;
        return TCL_OK;
    }
}
unsafe extern "C" fn tcl_thread_swap(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut temp: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID1 ID2\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        j = parse_thread_id(interp, *argv.offset(2 as ::core::ffi::c_int as isize));
        if j < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[j as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(j as isize) as *mut Thread,
        );
        temp = threadset[i as usize].db;
        threadset[i as usize].db = threadset[j as usize].db;
        threadset[j as usize].db = temp;
        return TCL_OK;
    }
}
unsafe extern "C" fn tcl_thread_db_get(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        unsafe extern "C" {
            #[link_name = "sqlite3TestMakePointerStr"]
            fn sqlite3TestMakePointerStr_0(
                _: *mut Tcl_Interp,
                _: *mut ::core::ffi::c_char,
                _: *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int;
        }
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        sqlite3TestMakePointerStr_0(
            interp,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            threadset[i as usize].db as *mut ::core::ffi::c_void,
        );
        threadset[i as usize].db = ::core::ptr::null_mut::<sqlite3>();
        Tcl_AppendResult(
            interp,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn tcl_thread_db_put(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID DB\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        threadset[i as usize].db = sqlite3TestTextToPtr(
            *argv.offset(2 as ::core::ffi::c_int as isize),
        ) as *mut sqlite3;
        return TCL_OK;
    }
}
unsafe extern "C" fn tcl_thread_stmt_get(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        i = parse_thread_id(interp, *argv.offset(1 as ::core::ffi::c_int as isize));
        if i < 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if threadset[i as usize].busy == 0 {
            Tcl_AppendResult(
                interp,
                b"no such thread\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        test_thread_wait(
            (&raw mut threadset as *mut Thread).offset(i as isize) as *mut Thread,
        );
        sqlite3TestMakePointerStr(
            interp,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            threadset[i as usize].pStmt as *mut ::core::ffi::c_void,
        );
        threadset[i as usize].pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        Tcl_AppendResult(
            interp,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest4_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aCmd: [C2Rust_Unnamed_23; 15] = unsafe {
            [
                C2Rust_Unnamed_23 {
                    zName: b"thread_create\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_create
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_wait\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_wait
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_halt\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_halt
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_argc\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_argc
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_argv\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_argv
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_colname\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_colname
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_result\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_result
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_error\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_error
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_compile\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_compile
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_step\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_step
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_finalize\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_finalize
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_swap\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_swap
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_db_get\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_db_get
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_db_put\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_db_put
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_23 {
                    zName: b"thread_stmt_get\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *mut *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        Option<Tcl_CmdProc>,
                    >(
                        Some(
                            tcl_thread_stmt_get
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
            ]
        };
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_23; 15]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_23>() as usize)
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
        return TCL_OK;
    }
}
