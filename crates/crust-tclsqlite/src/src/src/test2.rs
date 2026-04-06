use ::c2rust_bitfields;
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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Pager;
    pub type PCache;
    pub type Tcl_Command_;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_test_control(op: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3OsWrite(
        _: *mut sqlite3_file,
        _: *const ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsOpenMalloc(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: *mut *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsCloseFree(_: *mut sqlite3_file);
    fn sqlite3PagerOpen(
        _: *mut sqlite3_vfs,
        ppPager: *mut *mut Pager,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut DbPage) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerClose(pPager: *mut Pager, _: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3PagerSetPagesize(
        _: *mut Pager,
        _: *mut u32_0,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerSetCachesize(_: *mut Pager, _: ::core::ffi::c_int);
    fn sqlite3PagerGet(
        pPager: *mut Pager,
        pgno: Pgno,
        ppPage: *mut *mut DbPage,
        clrFlag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerLookup(pPager: *mut Pager, pgno: Pgno) -> *mut DbPage;
    fn sqlite3PagerUnref(_: *mut DbPage);
    fn sqlite3PagerWrite(_: *mut DbPage) -> ::core::ffi::c_int;
    fn sqlite3PagerGetData(_: *mut DbPage) -> *mut ::core::ffi::c_void;
    fn sqlite3PagerPagecount(_: *mut Pager, _: *mut ::core::ffi::c_int);
    fn sqlite3PagerCommitPhaseOne(
        _: *mut Pager,
        zSuper: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerCommitPhaseTwo(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerRollback(_: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerOpenSavepoint(
        pPager: *mut Pager,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerSavepoint(
        pPager: *mut Pager,
        op: ::core::ffi::c_int,
        iSavepoint: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3PagerSharedLock(pPager: *mut Pager) -> ::core::ffi::c_int;
    fn sqlite3PagerTruncateImage(_: *mut Pager, _: Pgno);
    fn sqlite3PagerPagenumber(_: *mut DbPage) -> Pgno;
    fn sqlite3PagerStats(_: *mut Pager) -> *mut ::core::ffi::c_int;
    fn Tcl_GetInt(
        interp: *mut Tcl_Interp,
        src: *const ::core::ffi::c_char,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_AppendElement(interp: *mut Tcl_Interp, element: *const ::core::ffi::c_char);
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_CmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_Eval(
        interp: *mut Tcl_Interp,
        script: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetStringResult(interp: *mut Tcl_Interp) -> *const ::core::ffi::c_char;
    fn Tcl_LinkVar(
        interp: *mut Tcl_Interp,
        varName: *const ::core::ffi::c_char,
        addr: *mut ::core::ffi::c_char,
        type_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_ResetResult(interp: *mut Tcl_Interp);
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn sqlite3TestTextToPtr(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    static mut sqlite3PendingByte: ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void,
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
    pub internalRep: C2Rust_Unnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_22 {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2Rust_Unnamed_24,
    pub ptrAndLongRep: C2Rust_Unnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_CmdProc>,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_BITVEC_TEST: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_FAULT_INSTALL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_PENDING_BYTE: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_LINK_INT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_LINK_READ_ONLY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SAVEPOINT_RELEASE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SAVEPOINT_ROLLBACK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut test_pagesize: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C" fn pager_test_reiniter(mut pNotUsed: *mut DbPage) {}
unsafe extern "C" fn pager_open(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pageSize: u32_0 = 0;
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut nPage: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FILENAME N-PAGE\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if Tcl_GetInt(
            interp,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut nPage,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3PagerOpen(
            sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>()),
            &raw mut pPager,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_MAIN_DB,
            Some(pager_test_reiniter as unsafe extern "C" fn(*mut DbPage) -> ()),
        );
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        sqlite3PagerSetCachesize(pPager, nPage);
        pageSize = test_pagesize as u32_0;
        sqlite3PagerSetPagesize(pPager, &raw mut pageSize, -1 as ::core::ffi::c_int);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%p\0".as_ptr() as *const ::core::ffi::c_char,
            pPager,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn pager_close(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        rc = sqlite3PagerClose(pPager, ::core::ptr::null_mut::<sqlite3>());
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn pager_rollback(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        rc = sqlite3PagerRollback(pPager);
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn pager_commit(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        rc = sqlite3PagerCommitPhaseOne(
            pPager,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
        );
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        rc = sqlite3PagerCommitPhaseTwo(pPager);
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn pager_stmt_begin(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        rc = sqlite3PagerOpenSavepoint(pPager, 1 as ::core::ffi::c_int);
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn pager_stmt_rollback(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        rc = sqlite3PagerSavepoint(pPager, SAVEPOINT_ROLLBACK, 0 as ::core::ffi::c_int);
        sqlite3PagerSavepoint(pPager, SAVEPOINT_RELEASE, 0 as ::core::ffi::c_int);
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn pager_stmt_commit(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        rc = sqlite3PagerSavepoint(pPager, SAVEPOINT_RELEASE, 0 as ::core::ffi::c_int);
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn pager_stats(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut i: ::core::ffi::c_int = 0;
        let mut a: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        a = sqlite3PagerStats(pPager);
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            static mut zName: [*mut ::core::ffi::c_char; 9] = [
                b"ref\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"page\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"max\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"size\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"state\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"err\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"hit\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"miss\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"ovfl\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            ];
            let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
            Tcl_AppendElement(interp, zName[i as usize]);
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                *a.offset(i as isize),
            );
            Tcl_AppendElement(interp, &raw mut zBuf as *mut ::core::ffi::c_char);
            i += 1;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn pager_pagecount(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        let mut nPage: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        sqlite3PagerPagecount(pPager, &raw mut nPage);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            nPage,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn page_get(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        let mut pPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        let mut pgno: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID PGNO\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        if Tcl_GetInt(
            interp,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut pgno,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3PagerSharedLock(pPager);
        if rc == SQLITE_OK {
            rc = sqlite3PagerGet(
                pPager,
                pgno as Pgno,
                &raw mut pPage,
                0 as ::core::ffi::c_int,
            );
        }
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%p\0".as_ptr() as *const ::core::ffi::c_char,
            pPage,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn page_lookup(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        let mut pPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        let mut pgno: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID PGNO\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        if Tcl_GetInt(
            interp,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut pgno,
        ) != 0
        {
            return TCL_ERROR;
        }
        pPage = sqlite3PagerLookup(pPager, pgno as Pgno);
        if !pPage.is_null() {
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>()
                    as ::core::ffi::c_int,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                b"%p\0".as_ptr() as *const ::core::ffi::c_char,
                pPage,
            );
            Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn pager_truncate(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
        let mut pgno: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID PGNO\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPager = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Pager;
        if Tcl_GetInt(
            interp,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut pgno,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3PagerTruncateImage(pPager, pgno as Pgno);
        return TCL_OK;
    }
}
unsafe extern "C" fn page_unref(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" PAGE\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPage = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut DbPage;
        sqlite3PagerUnref(pPage);
        return TCL_OK;
    }
}
unsafe extern "C" fn page_read(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        let mut pPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" PAGE\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPage = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut DbPage;
        memcpy(
            &raw mut zBuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            sqlite3PagerGetData(pPage),
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as size_t,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn page_number(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        let mut pPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" PAGE\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPage = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut DbPage;
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3PagerPagenumber(pPage),
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn page_write(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPage: *mut DbPage = ::core::ptr::null_mut::<DbPage>();
        let mut pData: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" PAGE DATA\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pPage = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut DbPage;
        rc = sqlite3PagerWrite(pPage);
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        pData = sqlite3PagerGetData(pPage) as *mut ::core::ffi::c_char;
        strncpy(
            pData,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            (test_pagesize - 1 as ::core::ffi::c_int) as size_t,
        );
        *pData.offset((test_pagesize - 1 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_char;
        return TCL_OK;
    }
}
unsafe extern "C" fn fake_big_file(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut fd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut offset: i64_0 = 0;
        let mut zFile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nFile: ::core::ffi::c_int = 0;
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" N-MEGABYTES FILE\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if Tcl_GetInt(interp, *argv.offset(1 as ::core::ffi::c_int as isize), &raw mut n)
            != 0
        {
            return TCL_ERROR;
        }
        pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        nFile = strlen(*argv.offset(2 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_int;
        zFile = sqlite3_malloc(nFile + 2 as ::core::ffi::c_int)
            as *mut ::core::ffi::c_char;
        if zFile.is_null() {
            return TCL_ERROR;
        }
        memcpy(
            zFile as *mut ::core::ffi::c_void,
            *argv.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (nFile + 1 as ::core::ffi::c_int) as size_t,
        );
        *zFile.offset((nFile + 1 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_char;
        rc = sqlite3OsOpenMalloc(
            pVfs,
            zFile,
            &raw mut fd,
            SQLITE_OPEN_CREATE | SQLITE_OPEN_READWRITE | SQLITE_OPEN_MAIN_DB,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if rc != 0 {
            Tcl_AppendResult(
                interp,
                b"open failed: \0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3ErrName(rc),
                NULL,
            );
            sqlite3_free(zFile as *mut ::core::ffi::c_void);
            return TCL_ERROR;
        }
        offset = n as i64_0;
        offset
            *= (1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int)
                as ::core::ffi::c_longlong;
        rc = sqlite3OsWrite(
            fd,
            b"Hello, World!\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            14 as ::core::ffi::c_int,
            offset,
        );
        sqlite3OsCloseFree(fd);
        sqlite3_free(zFile as *mut ::core::ffi::c_void);
        if rc != 0 {
            Tcl_AppendResult(
                interp,
                b"write failed: \0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3ErrName(rc),
                NULL,
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn testPendingByte(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pbyte: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" PENDING-BYTE\"\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            return TCL_ERROR;
        }
        if Tcl_GetInt(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut pbyte,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_test_control(SQLITE_TESTCTRL_PENDING_BYTE, pbyte);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
static mut faultSimInterp: *mut Tcl_Interp = ::core::ptr::null::<Tcl_Interp>()
    as *mut Tcl_Interp;
static mut faultSimScriptSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut faultSimScript: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
unsafe extern "C" fn faultSimCallback(mut x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let mut zInt: [::core::ffi::c_char; 30] = [0; 30];
        let mut i: ::core::ffi::c_int = 0;
        let mut isNeg: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if x == 0 as ::core::ffi::c_int {
            memcpy(
                faultSimScript.offset(faultSimScriptSize as isize)
                    as *mut ::core::ffi::c_void,
                b"0\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                2 as size_t,
            );
        } else {
            if x < 0 as ::core::ffi::c_int {
                isNeg = 1 as ::core::ffi::c_int;
                x = -x;
            } else {
                isNeg = 0 as ::core::ffi::c_int;
            }
            zInt[(::core::mem::size_of::<[::core::ffi::c_char; 30]>() as usize)
                .wrapping_sub(1 as usize) as usize] = 0 as ::core::ffi::c_char;
            i = (::core::mem::size_of::<[::core::ffi::c_char; 30]>() as usize)
                .wrapping_sub(2 as usize) as ::core::ffi::c_int;
            while i > 0 as ::core::ffi::c_int && x > 0 as ::core::ffi::c_int {
                zInt[i as usize] = (x % 10 as ::core::ffi::c_int + '0' as i32)
                    as ::core::ffi::c_char;
                i -= 1;
                x /= 10 as ::core::ffi::c_int;
            }
            if isNeg != 0 {
                let c2rust_fresh0 = i;
                i = i - 1;
                zInt[c2rust_fresh0 as usize] = '-' as i32 as ::core::ffi::c_char;
            }
            memcpy(
                faultSimScript.offset(faultSimScriptSize as isize)
                    as *mut ::core::ffi::c_void,
                (&raw mut zInt as *mut ::core::ffi::c_char)
                    .offset(i as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (::core::mem::size_of::<[::core::ffi::c_char; 30]>() as size_t)
                    .wrapping_sub(i as size_t)
                    .wrapping_sub(1 as size_t),
            );
        }
        rc = Tcl_Eval(faultSimInterp, faultSimScript);
        if rc != 0 {
            fprintf(
                stderr,
                b"fault simulator script failed: [%s]\0".as_ptr()
                    as *const ::core::ffi::c_char,
                faultSimScript,
            );
            rc = SQLITE_ERROR;
        } else {
            rc = atoi(Tcl_GetStringResult(faultSimInterp));
        }
        Tcl_ResetResult(faultSimInterp);
        return rc;
    }
}
unsafe extern "C" fn faultInstallCmd(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zScript: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nScript: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if argc != 1 as ::core::ffi::c_int && argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" SCRIPT\"\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        zScript = if argc == 2 as ::core::ffi::c_int {
            *argv.offset(1 as ::core::ffi::c_int as isize)
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        };
        nScript = strlen(zScript) as ::core::ffi::c_int;
        if !faultSimScript.is_null() {
            free(faultSimScript as *mut ::core::ffi::c_void);
            faultSimScript = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if nScript == 0 as ::core::ffi::c_int {
            rc = sqlite3_test_control(
                SQLITE_TESTCTRL_FAULT_INSTALL,
                0 as ::core::ffi::c_int,
            );
        } else {
            faultSimScript = malloc((nScript + 100 as ::core::ffi::c_int) as size_t)
                as *mut ::core::ffi::c_char;
            if faultSimScript.is_null() {
                Tcl_AppendResult(
                    interp,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
                return SQLITE_ERROR;
            }
            memcpy(
                faultSimScript as *mut ::core::ffi::c_void,
                zScript as *const ::core::ffi::c_void,
                nScript as size_t,
            );
            *faultSimScript.offset(nScript as isize) = ' ' as i32 as ::core::ffi::c_char;
            faultSimScriptSize = nScript + 1 as ::core::ffi::c_int;
            faultSimInterp = interp;
            rc = sqlite3_test_control(
                SQLITE_TESTCTRL_FAULT_INSTALL,
                Some(
                    faultSimCallback
                        as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
                ),
            );
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return SQLITE_OK;
    }
}
unsafe extern "C" fn testBitvecBuiltinTest(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sz: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut nProg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aProg: [::core::ffi::c_int; 100] = [0; 100];
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" SIZE PROGRAM\"\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        if Tcl_GetInt(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut sz,
        ) != 0
        {
            return TCL_ERROR;
        }
        z = *argv.offset(2 as ::core::ffi::c_int as isize);
        while nProg < 99 as ::core::ffi::c_int && *z as ::core::ffi::c_int != 0 {
            while *z as ::core::ffi::c_int != 0
                && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*z as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                    & 0x4 as ::core::ffi::c_int == 0
            {
                z = z.offset(1);
            }
            if *z as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
            let c2rust_fresh1 = nProg;
            nProg = nProg + 1;
            aProg[c2rust_fresh1 as usize] = atoi(z);
            while *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*z as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                & 0x4 as ::core::ffi::c_int != 0
            {
                z = z.offset(1);
            }
        }
        aProg[nProg as usize] = 0 as ::core::ffi::c_int;
        rc = sqlite3_test_control(
            SQLITE_TESTCTRL_BITVEC_TEST,
            sz,
            &raw mut aProg as *mut ::core::ffi::c_int,
        );
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest2_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            static mut sqlite3_io_error_persist: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_io_error_pending: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_io_error_hit: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_io_error_hardhit: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_diskfull_pending: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut sqlite3_diskfull: ::core::ffi::c_int;
        }
        static mut aCmd: [C2Rust_Unnamed_25; 20] = unsafe {
            [
                C2Rust_Unnamed_25 {
                    zName: b"pager_open\0".as_ptr() as *const ::core::ffi::c_char
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
                            pager_open
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"pager_close\0".as_ptr() as *const ::core::ffi::c_char
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
                            pager_close
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"pager_commit\0".as_ptr() as *const ::core::ffi::c_char
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
                            pager_commit
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"pager_rollback\0".as_ptr() as *const ::core::ffi::c_char
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
                            pager_rollback
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"pager_stmt_begin\0".as_ptr() as *const ::core::ffi::c_char
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
                            pager_stmt_begin
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"pager_stmt_commit\0".as_ptr() as *const ::core::ffi::c_char
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
                            pager_stmt_commit
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"pager_stmt_rollback\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
                            pager_stmt_rollback
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"pager_stats\0".as_ptr() as *const ::core::ffi::c_char
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
                            pager_stats
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"pager_pagecount\0".as_ptr() as *const ::core::ffi::c_char
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
                            pager_pagecount
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"page_get\0".as_ptr() as *const ::core::ffi::c_char
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
                            page_get
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"page_lookup\0".as_ptr() as *const ::core::ffi::c_char
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
                            page_lookup
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"page_unref\0".as_ptr() as *const ::core::ffi::c_char
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
                            page_unref
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"page_read\0".as_ptr() as *const ::core::ffi::c_char
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
                            page_read
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"page_write\0".as_ptr() as *const ::core::ffi::c_char
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
                            page_write
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"page_number\0".as_ptr() as *const ::core::ffi::c_char
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
                            page_number
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"pager_truncate\0".as_ptr() as *const ::core::ffi::c_char
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
                            pager_truncate
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"fake_big_file\0".as_ptr() as *const ::core::ffi::c_char
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
                            fake_big_file
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"sqlite3BitvecBuiltinTest\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
                            testBitvecBuiltinTest
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"sqlite3_test_control_pending_byte\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
                            testPendingByte
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *mut *const ::core::ffi::c_char,
                                ) -> ::core::ffi::c_int,
                        ),
                    ),
                },
                C2Rust_Unnamed_25 {
                    zName: b"sqlite3_test_control_fault_install\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
                            faultInstallCmd
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
            < (::core::mem::size_of::<[C2Rust_Unnamed_25; 20]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_25>() as usize)
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
        Tcl_LinkVar(
            interp,
            b"sqlite_io_error_pending\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_io_error_pending as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_io_error_persist\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_io_error_persist as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_io_error_hit\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_io_error_hit as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_io_error_hardhit\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_io_error_hardhit as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_diskfull_pending\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_diskfull_pending as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_diskfull\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3_diskfull as *mut ::core::ffi::c_char,
            TCL_LINK_INT,
        );
        Tcl_LinkVar(
            interp,
            b"sqlite_pending_byte\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sqlite3PendingByte as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        return TCL_OK;
    }
}
