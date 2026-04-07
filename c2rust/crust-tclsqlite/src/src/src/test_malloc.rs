use ::c2rust_bitfields;
use ::libc;
unsafe extern "C" {
    fn getDbPointer(
        interp: *mut Tcl_Interp,
        zA: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
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
    pub type sqlite3_pcache;
    pub type Tcl_Command_;
    fn sqlite3_config(_: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3_db_config(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_memory_used() -> sqlite3_int64;
    fn sqlite3_memory_highwater(resetFlag: ::core::ffi::c_int) -> sqlite3_int64;
    fn sqlite3_test_control(op: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3_status(
        op: ::core::ffi::c_int,
        pCurrent: *mut ::core::ffi::c_int,
        pHighwater: *mut ::core::ffi::c_int,
        resetFlag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_db_status(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        pCur: *mut ::core::ffi::c_int,
        pHiwtr: *mut ::core::ffi::c_int,
        resetFlg: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn Tcl_Free(ptr: *mut ::core::ffi::c_char);
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
    fn Tcl_GetStringFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_NewListObj(
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> *mut Tcl_Obj;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_DeleteHashTable(tablePtr: *mut Tcl_HashTable);
    fn Tcl_FirstHashEntry(
        tablePtr: *mut Tcl_HashTable,
        searchPtr: *mut Tcl_HashSearch,
    ) -> *mut Tcl_HashEntry;
    fn Tcl_InitHashTable(tablePtr: *mut Tcl_HashTable, keyType: ::core::ffi::c_int);
    fn Tcl_NextHashEntry(searchPtr: *mut Tcl_HashSearch) -> *mut Tcl_HashEntry;
    fn Tcl_ObjSetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        newValuePtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
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
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3TestHexToBin(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3TestBinToHex(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
pub type intptr_t = isize;
pub type size_t = usize;
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
pub type Tcl_FreeProc = unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_HashKeyType {
    pub version: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub hashKeyProc: Option<Tcl_HashKeyProc>,
    pub compareKeysProc: Option<Tcl_CompareHashKeysProc>,
    pub allocEntryProc: Option<Tcl_AllocHashEntryProc>,
    pub freeEntryProc: Option<Tcl_FreeHashEntryProc>,
}
pub type Tcl_FreeHashEntryProc = unsafe extern "C" fn(*mut Tcl_HashEntry) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_HashEntry {
    pub nextPtr: *mut Tcl_HashEntry,
    pub tablePtr: *mut Tcl_HashTable,
    pub hash: *mut ::core::ffi::c_void,
    pub clientData: ClientData,
    pub key: C2Rust_Unnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_25 {
    pub oneWordValue: *mut ::core::ffi::c_char,
    pub objPtr: *mut Tcl_Obj,
    pub words: [::core::ffi::c_int; 1],
    pub string: [::core::ffi::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_HashTable {
    pub buckets: *mut *mut Tcl_HashEntry,
    pub staticBuckets: [*mut Tcl_HashEntry; 4],
    pub numBuckets: ::core::ffi::c_int,
    pub numEntries: ::core::ffi::c_int,
    pub rebuildSize: ::core::ffi::c_int,
    pub downShift: ::core::ffi::c_int,
    pub mask: ::core::ffi::c_int,
    pub keyType: ::core::ffi::c_int,
    pub findProc: Option<
        unsafe extern "C" fn(
            *mut Tcl_HashTable,
            *const ::core::ffi::c_char,
        ) -> *mut Tcl_HashEntry,
    >,
    pub createProc: Option<
        unsafe extern "C" fn(
            *mut Tcl_HashTable,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
        ) -> *mut Tcl_HashEntry,
    >,
    pub typePtr: *const Tcl_HashKeyType,
}
pub type Tcl_AllocHashEntryProc = unsafe extern "C" fn(
    *mut Tcl_HashTable,
    *mut ::core::ffi::c_void,
) -> *mut Tcl_HashEntry;
pub type Tcl_CompareHashKeysProc = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut Tcl_HashEntry,
) -> ::core::ffi::c_int;
pub type Tcl_HashKeyProc = unsafe extern "C" fn(
    *mut Tcl_HashTable,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_HashSearch {
    pub tablePtr: *mut Tcl_HashTable,
    pub nextIndex: ::core::ffi::c_int,
    pub nextEntryPtr: *mut Tcl_HashEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemFault {
    pub iCountdown: ::core::ffi::c_int,
    pub nRepeat: ::core::ffi::c_int,
    pub nBenign: ::core::ffi::c_int,
    pub nFail: ::core::ffi::c_int,
    pub nOkBefore: ::core::ffi::c_int,
    pub nOkAfter: ::core::ffi::c_int,
    pub enable: u8_0,
    pub isInstalled: ::core::ffi::c_int,
    pub isBenignMode: ::core::ffi::c_int,
    pub m: sqlite3_mem_methods,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MallocLog {
    pub nCall: ::core::ffi::c_int,
    pub nByte: ::core::ffi::c_int,
}
pub const MB_LOG_SYNC: MB_enum = 4;
pub const MB_LOG_CLEAR: MB_enum = 3;
pub const MB_LOG_DUMP: MB_enum = 2;
pub const MB_LOG_STOP: MB_enum = 1;
pub const MB_LOG_START: MB_enum = 0;
pub type MB_enum = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_26 {
    pub zName: *const ::core::ffi::c_char,
    pub op: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub zName: *const ::core::ffi::c_char,
    pub op: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
    pub clientData: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_MALLOC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_GETMALLOC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_PAGECACHE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_HEAP: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_MEMSTATUS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_LOOKASIDE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_URI: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_COVERING_INDEX_SCAN: ::core::ffi::c_int = 20
    as ::core::ffi::c_int;
pub const SQLITE_CONFIG_WIN32_HEAPSIZE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_PMASZ: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_LOOKASIDE: ::core::ffi::c_int = 1001 as ::core::ffi::c_int;
pub const SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS: ::core::ffi::c_int = 10
    as ::core::ffi::c_int;
pub const SQLITE_STATUS_MEMORY_USED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_STATUS_PAGECACHE_USED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_STATUS_PAGECACHE_OVERFLOW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_STATUS_SCRATCH_USED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_STATUS_SCRATCH_OVERFLOW: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_STATUS_MALLOC_SIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATUS_PARSER_STACK: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_STATUS_PAGECACHE_SIZE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_STATUS_SCRATCH_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_STATUS_MALLOC_COUNT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_LOOKASIDE_USED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_USED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_SCHEMA_USED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_STMT_USED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_LOOKASIDE_HIT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE: ::core::ffi::c_int = 5
    as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL: ::core::ffi::c_int = 6
    as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_HIT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_MISS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_WRITE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_DEFERRED_FKS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_USED_SHARED: ::core::ffi::c_int = 11
    as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_CACHE_SPILL: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_DBSTATUS_TEMPBUF_SPILL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_ONE_WORD_KEYS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_CUSTOM_PTR_KEYS: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
static mut memfault: MemFault = MemFault {
    iCountdown: 0,
    nRepeat: 0,
    nBenign: 0,
    nFail: 0,
    nOkBefore: 0,
    nOkAfter: 0,
    enable: 0,
    isInstalled: 0,
    isBenignMode: 0,
    m: sqlite3_mem_methods {
        xMalloc: None,
        xFree: None,
        xRealloc: None,
        xSize: None,
        xRoundup: None,
        xInit: None,
        xShutdown: None,
        pAppData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
};
unsafe extern "C" fn sqlite3Fault() {
    unsafe {
        static mut cnt: u64_0 = 0 as u64_0;
        cnt = cnt.wrapping_add(1);
        if cnt > (1 as ::core::ffi::c_int as u64_0) << 63 as ::core::ffi::c_int {
            abort();
        }
    }
}
unsafe extern "C" fn sqlite3FirstFault() {
    unsafe {
        static mut cnt2: u64_0 = 0 as u64_0;
        cnt2 = cnt2.wrapping_add(1);
        if cnt2 > (1 as ::core::ffi::c_int as u64_0) << 63 as ::core::ffi::c_int {
            abort();
        }
    }
}
unsafe extern "C" fn faultsimStep() -> ::core::ffi::c_int {
    unsafe {
        if memfault.enable == 0 {
            memfault.nOkAfter += 1;
            return 0 as ::core::ffi::c_int;
        }
        if memfault.iCountdown > 0 as ::core::ffi::c_int {
            memfault.iCountdown -= 1;
            memfault.nOkBefore += 1;
            return 0 as ::core::ffi::c_int;
        }
        if memfault.nFail == 0 as ::core::ffi::c_int {
            sqlite3FirstFault();
        }
        sqlite3Fault();
        memfault.nFail += 1;
        if memfault.isBenignMode > 0 as ::core::ffi::c_int {
            memfault.nBenign += 1;
        }
        memfault.nRepeat -= 1;
        if memfault.nRepeat <= 0 as ::core::ffi::c_int {
            memfault.enable = 0 as u8_0;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn faultsimMalloc(
    mut n: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if faultsimStep() == 0 {
            p = memfault.m.xMalloc.expect("non-null function pointer")(n);
        }
        return p;
    }
}
unsafe extern "C" fn faultsimRealloc(
    mut pOld: *mut ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if faultsimStep() == 0 {
            p = memfault.m.xRealloc.expect("non-null function pointer")(pOld, n);
        }
        return p;
    }
}
unsafe extern "C" fn faultsimConfig(
    mut nDelay: ::core::ffi::c_int,
    mut nRepeat: ::core::ffi::c_int,
) {
    unsafe {
        memfault.iCountdown = nDelay;
        memfault.nRepeat = nRepeat;
        memfault.nBenign = 0 as ::core::ffi::c_int;
        memfault.nFail = 0 as ::core::ffi::c_int;
        memfault.nOkBefore = 0 as ::core::ffi::c_int;
        memfault.nOkAfter = 0 as ::core::ffi::c_int;
        memfault.enable = (nDelay >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as u8_0;
        memfault.isBenignMode = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn faultsimFailures() -> ::core::ffi::c_int {
    unsafe {
        return memfault.nFail;
    }
}
unsafe extern "C" fn faultsimBenignFailures() -> ::core::ffi::c_int {
    unsafe {
        return memfault.nBenign;
    }
}
unsafe extern "C" fn faultsimPending() -> ::core::ffi::c_int {
    unsafe {
        if memfault.enable != 0 {
            return memfault.iCountdown
        } else {
            return -1 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn faultsimBeginBenign() {
    unsafe {
        memfault.isBenignMode += 1;
    }
}
unsafe extern "C" fn faultsimEndBenign() {
    unsafe {
        memfault.isBenignMode -= 1;
    }
}
unsafe extern "C" fn faultsimInstall(
    mut install: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        install = if install != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        if install == memfault.isInstalled {
            return SQLITE_ERROR;
        }
        if install != 0 {
            rc = sqlite3_config(SQLITE_CONFIG_GETMALLOC, &raw mut memfault.m);
            if rc == SQLITE_OK {
                let mut m: sqlite3_mem_methods = memfault.m;
                m.xMalloc = Some(
                    faultsimMalloc
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                        ) -> *mut ::core::ffi::c_void,
                )
                    as Option<
                        unsafe extern "C" fn(
                            ::core::ffi::c_int,
                        ) -> *mut ::core::ffi::c_void,
                    >;
                m.xRealloc = Some(
                    faultsimRealloc
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                        ) -> *mut ::core::ffi::c_void,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                        ) -> *mut ::core::ffi::c_void,
                    >;
                rc = sqlite3_config(SQLITE_CONFIG_MALLOC, &raw mut m);
            }
            sqlite3_test_control(
                SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS,
                Some(faultsimBeginBenign as unsafe extern "C" fn() -> ()),
                Some(faultsimEndBenign as unsafe extern "C" fn() -> ()),
            );
        } else {
            let mut m2: sqlite3_mem_methods = sqlite3_mem_methods {
                xMalloc: None,
                xFree: None,
                xRealloc: None,
                xSize: None,
                xRoundup: None,
                xInit: None,
                xShutdown: None,
                pAppData: ::core::ptr::null::<::core::ffi::c_void>()
                    as *mut ::core::ffi::c_void,
            };
            memset(
                &raw mut m2 as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sqlite3_mem_methods>() as size_t,
            );
            sqlite3_config(SQLITE_CONFIG_MALLOC, &raw mut m2);
            sqlite3_config(SQLITE_CONFIG_GETMALLOC, &raw mut m2);
            rc = sqlite3_config(SQLITE_CONFIG_MALLOC, &raw mut memfault.m);
            sqlite3_test_control(
                SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        if rc == SQLITE_OK {
            memfault.isInstalled = 1 as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn pointerToText(
    mut p: *mut ::core::ffi::c_void,
    mut z: *mut ::core::ffi::c_char,
) {
    unsafe {
        static mut zHex: [::core::ffi::c_char; 17] = unsafe {
            ::core::mem::transmute::<
                [u8; 17],
                [::core::ffi::c_char; 17],
            >(*b"0123456789abcdef\0")
        };
        let mut i: ::core::ffi::c_int = 0;
        let mut k: ::core::ffi::c_int = 0;
        let mut u: ::core::ffi::c_uint = 0;
        let mut n: sqlite3_uint64 = 0;
        if p.is_null() {
            strcpy(z, b"0\0".as_ptr() as *const ::core::ffi::c_char);
            return;
        }
        if ::core::mem::size_of::<sqlite3_uint64>() as usize
            == ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
        {
            memcpy(
                &raw mut n as *mut ::core::ffi::c_void,
                &raw mut p as *const ::core::ffi::c_void,
                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
            );
        } else if ::core::mem::size_of::<::core::ffi::c_uint>() as usize
            == ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
        {
            memcpy(
                &raw mut u as *mut ::core::ffi::c_void,
                &raw mut p as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_uint>() as size_t,
            );
            n = u as sqlite3_uint64;
        }
        i = 0 as ::core::ffi::c_int;
        k = (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
            .wrapping_mul(2 as usize)
            .wrapping_sub(1 as usize) as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                .wrapping_mul(2 as usize)
        {
            *z.offset(k as isize) = zHex[(n as ::core::ffi::c_ulonglong
                & 0xf as ::core::ffi::c_ulonglong) as usize];
            n >>= 4 as ::core::ffi::c_int;
            i += 1;
            k -= 1;
        }
        *z
            .offset(
                (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                    .wrapping_mul(2 as usize) as isize,
            ) = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn hexToInt(mut h: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        if h >= '0' as i32 && h <= '9' as i32 {
            return h - '0' as i32
        } else if h >= 'a' as i32 && h <= 'f' as i32 {
            return h - 'a' as i32 + 10 as ::core::ffi::c_int
        } else {
            return -1 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn textToPointer(
    mut z: *const ::core::ffi::c_char,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: sqlite3_uint64 = 0 as sqlite3_uint64;
        let mut i: ::core::ffi::c_int = 0;
        let mut u: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                .wrapping_mul(2 as usize)
            && *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            let mut v: ::core::ffi::c_int = 0;
            let c2rust_fresh0 = z;
            z = z.offset(1);
            v = hexToInt(*c2rust_fresh0 as ::core::ffi::c_int);
            if v < 0 as ::core::ffi::c_int {
                return TCL_ERROR;
            }
            n = (n as ::core::ffi::c_ulonglong)
                .wrapping_mul(16 as ::core::ffi::c_ulonglong)
                .wrapping_add(v as ::core::ffi::c_ulonglong) as sqlite3_uint64;
            i += 1;
        }
        if *z as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            return TCL_ERROR;
        }
        if ::core::mem::size_of::<sqlite3_uint64>() as usize
            == ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
        {
            memcpy(
                pp as *mut ::core::ffi::c_void,
                &raw mut n as *const ::core::ffi::c_void,
                ::core::mem::size_of::<sqlite3_uint64>() as size_t,
            );
        } else if ::core::mem::size_of::<::core::ffi::c_uint>() as usize
            == ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
        {
            u = n as ::core::ffi::c_uint;
            memcpy(
                pp as *mut ::core::ffi::c_void,
                &raw mut u as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_uint>() as size_t,
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_malloc(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nByte: ::core::ffi::c_int = 0;
        let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut zOut: [::core::ffi::c_char; 100] = [0; 100];
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"NBYTES\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut nByte,
        ) != 0
        {
            return TCL_ERROR;
        }
        p = sqlite3_malloc(nByte as ::core::ffi::c_uint as ::core::ffi::c_int);
        pointerToText(p, &raw mut zOut as *mut ::core::ffi::c_char);
        Tcl_AppendResult(interp, &raw mut zOut as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_realloc(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nByte: ::core::ffi::c_int = 0;
        let mut pPrior: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut zOut: [::core::ffi::c_char; 100] = [0; 100];
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"PRIOR NBYTES\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut nByte,
        ) != 0
        {
            return TCL_ERROR;
        }
        if textToPointer(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pPrior,
        ) != 0
        {
            Tcl_AppendResult(
                interp,
                b"bad pointer: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        p = sqlite3_realloc(pPrior, nByte as ::core::ffi::c_uint as ::core::ffi::c_int);
        pointerToText(p, &raw mut zOut as *mut ::core::ffi::c_char);
        Tcl_AppendResult(interp, &raw mut zOut as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_free(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pPrior: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"PRIOR\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if textToPointer(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut pPrior,
        ) != 0
        {
            Tcl_AppendResult(
                interp,
                b"bad pointer: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        sqlite3_free(pPrior);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_memset(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut size: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut zHex: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zBin: [::core::ffi::c_char; 100] = [0; 100];
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"ADDRESS SIZE HEX\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if textToPointer(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut p,
        ) != 0
        {
            Tcl_AppendResult(
                interp,
                b"bad pointer: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut size,
        ) != 0
        {
            return TCL_ERROR;
        }
        if size <= 0 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"size must be positive\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        zHex = Tcl_GetStringFromObj(
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut n,
        );
        if n as usize
            > (::core::mem::size_of::<[::core::ffi::c_char; 100]>() as usize)
                .wrapping_mul(2 as usize)
        {
            n = (::core::mem::size_of::<[::core::ffi::c_char; 100]>() as usize)
                .wrapping_mul(2 as usize) as ::core::ffi::c_int;
        }
        n = sqlite3TestHexToBin(zHex, n, &raw mut zBin as *mut ::core::ffi::c_char);
        if n == 0 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"no data\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        zOut = p as *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < size {
            *zOut.offset(i as isize) = zBin[(i % n) as usize];
            i += 1;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_memget(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut size: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut zBin: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zHex: [::core::ffi::c_char; 100] = [0; 100];
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"ADDRESS SIZE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if textToPointer(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut p,
        ) != 0
        {
            Tcl_AppendResult(
                interp,
                b"bad pointer: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut size,
        ) != 0
        {
            return TCL_ERROR;
        }
        if size <= 0 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"size must be positive\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        zBin = p as *mut ::core::ffi::c_char;
        while size > 0 as ::core::ffi::c_int {
            if size as usize
                > (::core::mem::size_of::<[::core::ffi::c_char; 100]>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(2 as usize)
            {
                n = (::core::mem::size_of::<[::core::ffi::c_char; 100]>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_div(2 as usize) as ::core::ffi::c_int;
            } else {
                n = size;
            }
            memcpy(
                &raw mut zHex as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                zBin as *const ::core::ffi::c_void,
                n as size_t,
            );
            zBin = zBin.offset(n as isize);
            size -= n;
            sqlite3TestBinToHex(&raw mut zHex as *mut ::core::ffi::c_char, n);
            Tcl_AppendResult(
                interp,
                &raw mut zHex as *mut ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_memory_used(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_SetObjResult(
            interp,
            Tcl_NewWideIntObj(sqlite3_memory_used() as Tcl_WideInt),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_memory_highwater(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut resetFlag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc != 1 as ::core::ffi::c_int && objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?RESET?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if objc == 2 as ::core::ffi::c_int {
            if Tcl_GetBooleanFromObj(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut resetFlag,
            ) != 0
            {
                return TCL_ERROR;
            }
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewWideIntObj(sqlite3_memory_highwater(resetFlag) as Tcl_WideInt),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_memdebug_backtrace(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut depth: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DEPT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut depth,
        ) != 0
        {
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_memdebug_dump(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_memdebug_malloc_count(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nMalloc: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(nMalloc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_memdebug_fail(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        let mut iFail: ::core::ffi::c_int = 0;
        let mut nRepeat: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut pBenignCnt: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut nBenign: ::core::ffi::c_int = 0;
        let mut nFail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"COUNTER ?OPTIONS?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut iFail,
        ) != 0
        {
            return TCL_ERROR;
        }
        ii = 2 as ::core::ffi::c_int;
        while ii < objc {
            let mut nOption: ::core::ffi::c_int = 0;
            let mut zOption: *mut ::core::ffi::c_char = Tcl_GetStringFromObj(
                *objv.offset(ii as isize),
                &raw mut nOption,
            );
            let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            if nOption > 1 as ::core::ffi::c_int
                && strncmp(
                    zOption,
                    b"-repeat\0".as_ptr() as *const ::core::ffi::c_char,
                    nOption as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                if ii == objc - 1 as ::core::ffi::c_int {
                    zErr = b"option requires an argument: \0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                } else if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset((ii + 1 as ::core::ffi::c_int) as isize),
                    &raw mut nRepeat,
                ) != 0
                {
                    return TCL_ERROR
                }
            } else if nOption > 1 as ::core::ffi::c_int
                && strncmp(
                    zOption,
                    b"-benigncnt\0".as_ptr() as *const ::core::ffi::c_char,
                    nOption as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                if ii == objc - 1 as ::core::ffi::c_int {
                    zErr = b"option requires an argument: \0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                } else {
                    pBenignCnt = *objv.offset((ii + 1 as ::core::ffi::c_int) as isize);
                }
            } else {
                zErr = b"unknown option: \0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            if !zErr.is_null() {
                Tcl_AppendResult(interp, zErr, zOption, NULL);
                return TCL_ERROR;
            }
            ii += 2 as ::core::ffi::c_int;
        }
        nBenign = faultsimBenignFailures();
        nFail = faultsimFailures();
        faultsimConfig(iFail, nRepeat);
        if !pBenignCnt.is_null() {
            Tcl_ObjSetVar2(
                interp,
                pBenignCnt,
                ::core::ptr::null_mut::<Tcl_Obj>(),
                Tcl_NewIntObj(nBenign),
                0 as ::core::ffi::c_int,
            );
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(nFail));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_memdebug_pending(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nPending: ::core::ffi::c_int = 0;
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        nPending = faultsimPending();
        Tcl_SetObjResult(interp, Tcl_NewIntObj(nPending));
        return TCL_OK;
    }
}
static mut sqlite3_memdebug_title_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn test_memdebug_settitle(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        sqlite3_memdebug_title_count += 1;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"TITLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
pub const MALLOC_LOG_FRAMES: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut aMallocLog: Tcl_HashTable = Tcl_HashTable {
    buckets: ::core::ptr::null::<*mut Tcl_HashEntry>() as *mut *mut Tcl_HashEntry,
    staticBuckets: [::core::ptr::null::<Tcl_HashEntry>() as *mut Tcl_HashEntry; 4],
    numBuckets: 0,
    numEntries: 0,
    rebuildSize: 0,
    downShift: 0,
    mask: 0,
    keyType: 0,
    findProc: None,
    createProc: None,
    typePtr: ::core::ptr::null::<Tcl_HashKeyType>(),
};
static mut mallocLogEnabled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn test_memdebug_log_clear() {
    unsafe {
        let mut search: Tcl_HashSearch = Tcl_HashSearch {
            tablePtr: ::core::ptr::null_mut::<Tcl_HashTable>(),
            nextIndex: 0,
            nextEntryPtr: ::core::ptr::null_mut::<Tcl_HashEntry>(),
        };
        let mut pEntry: *mut Tcl_HashEntry = ::core::ptr::null_mut::<Tcl_HashEntry>();
        pEntry = Tcl_FirstHashEntry(&raw mut aMallocLog, &raw mut search);
        while !pEntry.is_null() {
            let mut pLog: *mut MallocLog = (*pEntry).clientData as *mut MallocLog;
            Tcl_Free(pLog as *mut ::core::ffi::c_char);
            pEntry = Tcl_NextHashEntry(&raw mut search);
        }
        Tcl_DeleteHashTable(&raw mut aMallocLog);
        Tcl_InitHashTable(
            &raw mut aMallocLog,
            (10 as usize)
                .wrapping_mul(
                    (if ::core::mem::size_of::<::core::ffi::c_int>() as usize
                        >= ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                    {
                        1 as usize
                    } else {
                        (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            )
                    }),
                ) as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn test_memdebug_log(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        static mut isInit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iSub: ::core::ffi::c_int = 0;
        static mut MB_strs: [*const ::core::ffi::c_char; 5] = [
            b"start\0".as_ptr() as *const ::core::ffi::c_char,
            b"stop\0".as_ptr() as *const ::core::ffi::c_char,
            b"dump\0".as_ptr() as *const ::core::ffi::c_char,
            b"clear\0".as_ptr() as *const ::core::ffi::c_char,
            b"sync\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        if isInit == 0 {
            Tcl_InitHashTable(
                &raw mut aMallocLog,
                (10 as usize)
                    .wrapping_mul(
                        (if ::core::mem::size_of::<::core::ffi::c_int>() as usize
                            >= ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                                as usize
                        {
                            1 as usize
                        } else {
                            (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                                .wrapping_div(
                                    ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                )
                        }),
                    ) as ::core::ffi::c_int,
            );
            isInit = 1 as ::core::ffi::c_int;
        }
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUB-COMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut MB_strs as *mut *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as ::core::ffi::c_int,
            b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iSub,
        ) != 0
        {
            return TCL_ERROR;
        }
        match iSub as MB_enum as ::core::ffi::c_uint {
            0 => {
                mallocLogEnabled = 1 as ::core::ffi::c_int;
            }
            1 => {
                mallocLogEnabled = 0 as ::core::ffi::c_int;
            }
            2 => {
                let mut search: Tcl_HashSearch = Tcl_HashSearch {
                    tablePtr: ::core::ptr::null_mut::<Tcl_HashTable>(),
                    nextIndex: 0,
                    nextEntryPtr: ::core::ptr::null_mut::<Tcl_HashEntry>(),
                };
                let mut pEntry: *mut Tcl_HashEntry = ::core::ptr::null_mut::<
                    Tcl_HashEntry,
                >();
                let mut pRet: *mut Tcl_Obj = Tcl_NewObj();
                pEntry = Tcl_FirstHashEntry(&raw mut aMallocLog, &raw mut search);
                while !pEntry.is_null() {
                    let mut apElem: [*mut Tcl_Obj; 12] = [::core::ptr::null_mut::<
                        Tcl_Obj,
                    >(); 12];
                    let mut pLog: *mut MallocLog = (*pEntry).clientData
                        as *mut MallocLog;
                    let mut aKey: *mut Tcl_WideInt = (if aMallocLog.keyType
                        == TCL_ONE_WORD_KEYS || aMallocLog.keyType == TCL_CUSTOM_PTR_KEYS
                    {
                        (*pEntry).key.oneWordValue
                    } else {
                        &raw mut (*pEntry).key.string as *mut ::core::ffi::c_char
                    }) as *mut ::core::ffi::c_void as *mut Tcl_WideInt;
                    let mut ii: ::core::ffi::c_int = 0;
                    apElem[0 as ::core::ffi::c_int as usize] = Tcl_NewIntObj(
                        (*pLog).nCall,
                    );
                    apElem[1 as ::core::ffi::c_int as usize] = Tcl_NewIntObj(
                        (*pLog).nByte,
                    );
                    ii = 0 as ::core::ffi::c_int;
                    while ii < MALLOC_LOG_FRAMES {
                        apElem[(ii + 2 as ::core::ffi::c_int) as usize] = Tcl_NewWideIntObj(
                            *aKey.offset(ii as isize),
                        );
                        ii += 1;
                    }
                    Tcl_ListObjAppendElement(
                        interp,
                        pRet,
                        Tcl_NewListObj(
                            MALLOC_LOG_FRAMES + 2 as ::core::ffi::c_int,
                            &raw mut apElem as *mut *mut Tcl_Obj as *const *mut Tcl_Obj,
                        ),
                    );
                    pEntry = Tcl_NextHashEntry(&raw mut search);
                }
                Tcl_SetObjResult(interp, pRet);
            }
            3 => {
                test_memdebug_log_clear();
            }
            4 | _ => {}
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_config_pagecache(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sz: ::core::ffi::c_int = 0;
        let mut N: ::core::ffi::c_int = 0;
        let mut pRes: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        static mut buf: *mut ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >() as *mut ::core::ffi::c_char;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SIZE N\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut sz,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut N,
        ) != 0
        {
            return TCL_ERROR;
        }
        free(buf as *mut ::core::ffi::c_void);
        buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
        pRes = Tcl_NewObj();
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pRes,
            Tcl_NewIntObj(sqlite3Config.szPage),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pRes,
            Tcl_NewIntObj(sqlite3Config.nPage),
        );
        Tcl_SetObjResult(interp, pRes);
        if sz < 0 as ::core::ffi::c_int {
            sqlite3_config(
                SQLITE_CONFIG_PAGECACHE,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else {
            buf = malloc((sz * N) as size_t) as *mut ::core::ffi::c_char;
            sqlite3_config(SQLITE_CONFIG_PAGECACHE, buf, sz, N);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_alt_pcache(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut installFlag: ::core::ffi::c_int = 0;
        let mut discardChance: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut prngSeed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut highStress: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        unsafe extern "C" {
            #[link_name = "installTestPCache"]
            fn installTestPCache_0(
                _: ::core::ffi::c_int,
                _: ::core::ffi::c_uint,
                _: ::core::ffi::c_uint,
                _: ::core::ffi::c_uint,
            );
        }
        if objc < 2 as ::core::ffi::c_int || objc > 5 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"INSTALLFLAG DISCARDCHANCE PRNGSEEED HIGHSTRESS\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut installFlag,
        ) != 0
        {
            return TCL_ERROR;
        }
        if objc >= 3 as ::core::ffi::c_int
            && Tcl_GetIntFromObj(
                interp,
                *objv.offset(2 as ::core::ffi::c_int as isize),
                &raw mut discardChance,
            ) != 0
        {
            return TCL_ERROR;
        }
        if objc >= 4 as ::core::ffi::c_int
            && Tcl_GetIntFromObj(
                interp,
                *objv.offset(3 as ::core::ffi::c_int as isize),
                &raw mut prngSeed,
            ) != 0
        {
            return TCL_ERROR;
        }
        if objc >= 5 as ::core::ffi::c_int
            && Tcl_GetIntFromObj(
                interp,
                *objv.offset(4 as ::core::ffi::c_int as isize),
                &raw mut highStress,
            ) != 0
        {
            return TCL_ERROR;
        }
        if discardChance < 0 as ::core::ffi::c_int
            || discardChance > 100 as ::core::ffi::c_int
        {
            Tcl_AppendResult(
                interp,
                b"discard-chance should be between 0 and 100\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        installTestPCache_0(
            installFlag,
            discardChance as ::core::ffi::c_uint,
            prngSeed as ::core::ffi::c_uint,
            highStress as ::core::ffi::c_uint,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_config_memstatus(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut enable: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
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
            &raw mut enable,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_config(SQLITE_CONFIG_MEMSTATUS, enable);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_config_lookaside(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sz: ::core::ffi::c_int = 0;
        let mut cnt: ::core::ffi::c_int = 0;
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SIZE COUNT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut sz,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut cnt,
        ) != 0
        {
            return TCL_ERROR;
        }
        pRet = Tcl_NewObj();
        Tcl_ListObjAppendElement(interp, pRet, Tcl_NewIntObj(sqlite3Config.szLookaside));
        Tcl_ListObjAppendElement(interp, pRet, Tcl_NewIntObj(sqlite3Config.nLookaside));
        sqlite3_config(SQLITE_CONFIG_LOOKASIDE, sz, cnt);
        Tcl_SetObjResult(interp, pRet);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_db_config_lookaside(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut sz: ::core::ffi::c_int = 0;
        let mut cnt: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut bufid: ::core::ffi::c_int = 0;
        static mut azBuf: [[::core::ffi::c_char; 10000]; 2] = [[0; 10000]; 2];
        unsafe extern "C" {
            #[link_name = "getDbPointer"]
            fn getDbPointer_0(
                _: *mut Tcl_Interp,
                _: *const ::core::ffi::c_char,
                _: *mut *mut sqlite3,
            ) -> ::core::ffi::c_int;
        }
        if objc != 5 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"BUFID SIZE COUNT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer_0(
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
            &raw mut bufid,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut sz,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(4 as ::core::ffi::c_int as isize),
            &raw mut cnt,
        ) != 0
        {
            return TCL_ERROR;
        }
        if bufid == 0 as ::core::ffi::c_int {
            rc = sqlite3_db_config(
                db,
                SQLITE_DBCONFIG_LOOKASIDE,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sz,
                cnt,
            );
        } else if bufid >= 1 as ::core::ffi::c_int && bufid <= 2 as ::core::ffi::c_int
            && (sz * cnt) as usize
                <= ::core::mem::size_of::<[::core::ffi::c_char; 10000]>() as usize
        {
            rc = sqlite3_db_config(
                db,
                SQLITE_DBCONFIG_LOOKASIDE,
                &raw mut *(&raw mut azBuf as *mut [::core::ffi::c_char; 10000])
                    .offset(bufid as isize) as *mut ::core::ffi::c_char,
                sz,
                cnt,
            );
        } else {
            Tcl_AppendResult(
                interp,
                b"illegal arguments - see documentation\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_config_heap(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        static mut zBuf: *mut ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >() as *mut ::core::ffi::c_char;
        let mut nByte: ::core::ffi::c_int = 0;
        let mut nMinAlloc: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut aArg: *const *mut Tcl_Obj = objv.offset(1 as ::core::ffi::c_int as isize)
            as *const *mut Tcl_Obj;
        let mut nArg: ::core::ffi::c_int = objc - 1 as ::core::ffi::c_int;
        if nArg != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"NBYTE NMINALLOC\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *aArg.offset(0 as ::core::ffi::c_int as isize),
            &raw mut nByte,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *aArg.offset(1 as ::core::ffi::c_int as isize),
            &raw mut nMinAlloc,
        ) != 0
        {
            return TCL_ERROR;
        }
        if nByte == 0 as ::core::ffi::c_int {
            free(zBuf as *mut ::core::ffi::c_void);
            zBuf = ::core::ptr::null_mut::<::core::ffi::c_char>();
            rc = sqlite3_config(
                SQLITE_CONFIG_HEAP,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else {
            zBuf = realloc(zBuf as *mut ::core::ffi::c_void, nByte as size_t)
                as *mut ::core::ffi::c_char;
            rc = sqlite3_config(SQLITE_CONFIG_HEAP, zBuf, nByte, nMinAlloc);
        }
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
unsafe extern "C" fn test_config_heap_size(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nByte: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut aArg: *const *mut Tcl_Obj = objv.offset(1 as ::core::ffi::c_int as isize)
            as *const *mut Tcl_Obj;
        let mut nArg: ::core::ffi::c_int = objc - 1 as ::core::ffi::c_int;
        if nArg != 1 as ::core::ffi::c_int {
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
            *aArg.offset(0 as ::core::ffi::c_int as isize),
            &raw mut nByte,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_config(SQLITE_CONFIG_WIN32_HEAPSIZE, nByte);
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
unsafe extern "C" fn test_config_error(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 2 as ::core::ffi::c_int && objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"[DB]\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if objc == 2 as ::core::ffi::c_int {
            if getDbPointer(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut db,
            ) != 0
            {
                return TCL_ERROR;
            }
            if sqlite3_db_config(db, 99999 as ::core::ffi::c_int) != SQLITE_ERROR {
                Tcl_AppendResult(
                    interp,
                    b"sqlite3_db_config(db, 99999) does not return SQLITE_ERROR\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_ERROR;
            }
        } else if sqlite3_config(99999 as ::core::ffi::c_int) != SQLITE_ERROR {
            Tcl_AppendResult(
                interp,
                b"sqlite3_config(99999) does not return SQLITE_ERROR\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_config_uri(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut bOpenUri: ::core::ffi::c_int = 0;
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
            &raw mut bOpenUri,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_config(SQLITE_CONFIG_URI, bOpenUri);
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
unsafe extern "C" fn test_config_cis(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut bUseCis: ::core::ffi::c_int = 0;
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
            &raw mut bUseCis,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_config(SQLITE_CONFIG_COVERING_INDEX_SCAN, bUseCis);
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
unsafe extern "C" fn test_config_pmasz(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut iPmaSz: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"BOOL\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut iPmaSz,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_config(SQLITE_CONFIG_PMASZ, iPmaSz);
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
unsafe extern "C" fn test_dump_memsys3(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        match clientData as intptr_t as ::core::ffi::c_int {
            3 | 5 | _ => {}
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_status(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut iValue: ::core::ffi::c_int = 0;
        let mut mxValue: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut op: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut resetFlag: ::core::ffi::c_int = 0;
        let mut zOpName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        static mut aOp: [C2Rust_Unnamed_26; 10] = [
            C2Rust_Unnamed_26 {
                zName: b"SQLITE_STATUS_MEMORY_USED\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STATUS_MEMORY_USED,
            },
            C2Rust_Unnamed_26 {
                zName: b"SQLITE_STATUS_MALLOC_SIZE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STATUS_MALLOC_SIZE,
            },
            C2Rust_Unnamed_26 {
                zName: b"SQLITE_STATUS_PAGECACHE_USED\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STATUS_PAGECACHE_USED,
            },
            C2Rust_Unnamed_26 {
                zName: b"SQLITE_STATUS_PAGECACHE_OVERFLOW\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STATUS_PAGECACHE_OVERFLOW,
            },
            C2Rust_Unnamed_26 {
                zName: b"SQLITE_STATUS_PAGECACHE_SIZE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STATUS_PAGECACHE_SIZE,
            },
            C2Rust_Unnamed_26 {
                zName: b"SQLITE_STATUS_SCRATCH_USED\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STATUS_SCRATCH_USED,
            },
            C2Rust_Unnamed_26 {
                zName: b"SQLITE_STATUS_SCRATCH_OVERFLOW\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STATUS_SCRATCH_OVERFLOW,
            },
            C2Rust_Unnamed_26 {
                zName: b"SQLITE_STATUS_SCRATCH_SIZE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STATUS_SCRATCH_SIZE,
            },
            C2Rust_Unnamed_26 {
                zName: b"SQLITE_STATUS_PARSER_STACK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STATUS_PARSER_STACK,
            },
            C2Rust_Unnamed_26 {
                zName: b"SQLITE_STATUS_MALLOC_COUNT\0".as_ptr()
                    as *const ::core::ffi::c_char,
                op: SQLITE_STATUS_MALLOC_COUNT,
            },
        ];
        let mut pResult: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"PARAMETER RESETFLAG\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zOpName = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[C2Rust_Unnamed_26; 10]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_26>() as usize)
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
            >= (::core::mem::size_of::<[C2Rust_Unnamed_26; 10]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_26>() as usize)
                as ::core::ffi::c_int
        {
            if Tcl_GetIntFromObj(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut op,
            ) != 0
            {
                return TCL_ERROR;
            }
        }
        if Tcl_GetBooleanFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut resetFlag,
        ) != 0
        {
            return TCL_ERROR;
        }
        iValue = 0 as ::core::ffi::c_int;
        mxValue = 0 as ::core::ffi::c_int;
        rc = sqlite3_status(op, &raw mut iValue, &raw mut mxValue, resetFlag);
        pResult = Tcl_NewObj();
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pResult,
            Tcl_NewIntObj(rc),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pResult,
            Tcl_NewIntObj(iValue),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pResult,
            Tcl_NewIntObj(mxValue),
        );
        Tcl_SetObjResult(interp, pResult);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_db_status(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut iValue: ::core::ffi::c_int = 0;
        let mut mxValue: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut op: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut resetFlag: ::core::ffi::c_int = 0;
        let mut zOpName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        static mut aOp: [C2Rust_Unnamed_27; 14] = [
            C2Rust_Unnamed_27 {
                zName: b"LOOKASIDE_USED\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_LOOKASIDE_USED,
            },
            C2Rust_Unnamed_27 {
                zName: b"CACHE_USED\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_CACHE_USED,
            },
            C2Rust_Unnamed_27 {
                zName: b"SCHEMA_USED\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_SCHEMA_USED,
            },
            C2Rust_Unnamed_27 {
                zName: b"STMT_USED\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_STMT_USED,
            },
            C2Rust_Unnamed_27 {
                zName: b"LOOKASIDE_HIT\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_LOOKASIDE_HIT,
            },
            C2Rust_Unnamed_27 {
                zName: b"LOOKASIDE_MISS_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE,
            },
            C2Rust_Unnamed_27 {
                zName: b"LOOKASIDE_MISS_FULL\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL,
            },
            C2Rust_Unnamed_27 {
                zName: b"CACHE_HIT\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_CACHE_HIT,
            },
            C2Rust_Unnamed_27 {
                zName: b"CACHE_MISS\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_CACHE_MISS,
            },
            C2Rust_Unnamed_27 {
                zName: b"CACHE_WRITE\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_CACHE_WRITE,
            },
            C2Rust_Unnamed_27 {
                zName: b"DEFERRED_FKS\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_DEFERRED_FKS,
            },
            C2Rust_Unnamed_27 {
                zName: b"CACHE_USED_SHARED\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_CACHE_USED_SHARED,
            },
            C2Rust_Unnamed_27 {
                zName: b"CACHE_SPILL\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_CACHE_SPILL,
            },
            C2Rust_Unnamed_27 {
                zName: b"TEMPBUF_SPILL\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_DBSTATUS_TEMPBUF_SPILL,
            },
        ];
        let mut pResult: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB PARAMETER RESETFLAG\0".as_ptr() as *const ::core::ffi::c_char,
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
        zOpName = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if memcmp(
            zOpName as *const ::core::ffi::c_void,
            b"SQLITE_\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            7 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            zOpName = zOpName.offset(7 as ::core::ffi::c_int as isize);
        }
        if memcmp(
            zOpName as *const ::core::ffi::c_void,
            b"DBSTATUS_\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            9 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            zOpName = zOpName.offset(9 as ::core::ffi::c_int as isize);
        }
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[C2Rust_Unnamed_27; 14]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_27>() as usize)
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
            >= (::core::mem::size_of::<[C2Rust_Unnamed_27; 14]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_27>() as usize)
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
        iValue = 0 as ::core::ffi::c_int;
        mxValue = 0 as ::core::ffi::c_int;
        rc = sqlite3_db_status(db, op, &raw mut iValue, &raw mut mxValue, resetFlag);
        pResult = Tcl_NewObj();
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pResult,
            Tcl_NewIntObj(rc),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pResult,
            Tcl_NewIntObj(iValue),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pResult,
            Tcl_NewIntObj(mxValue),
        );
        Tcl_SetObjResult(interp, pResult);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_install_malloc_faultsim(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut isInstall: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if TCL_OK
            != Tcl_GetBooleanFromObj(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut isInstall,
            )
        {
            return TCL_ERROR;
        }
        rc = faultsimInstall(isInstall);
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
unsafe extern "C" fn test_install_memsys3(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_MISUSE;
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
unsafe extern "C" fn test_vfs_oom_test(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            static mut sqlite3_memdebug_vfs_oom_test: ::core::ffi::c_int;
        }
        if objc > 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?INTEGER?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        } else if objc == 2 as ::core::ffi::c_int {
            let mut iNew: ::core::ffi::c_int = 0;
            if Tcl_GetIntFromObj(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut iNew,
            ) != 0
            {
                return TCL_ERROR;
            }
            sqlite3_memdebug_vfs_oom_test = iNew;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_memdebug_vfs_oom_test));
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest_malloc_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aObjCmd: [C2Rust_Unnamed_28; 32] = unsafe {
            [
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_malloc\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_malloc
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_realloc\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_realloc
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_free\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_free
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"memset\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memset
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"memget\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memget
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_memory_used\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memory_used
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_memory_highwater\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memory_highwater
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_memdebug_backtrace\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memdebug_backtrace
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_memdebug_dump\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memdebug_dump
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_memdebug_fail\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memdebug_fail
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_memdebug_pending\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memdebug_pending
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_memdebug_settitle\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memdebug_settitle
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_memdebug_malloc_count\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memdebug_malloc_count
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_memdebug_log\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_memdebug_log
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_config_pagecache\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_config_pagecache
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_config_alt_pcache\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_alt_pcache
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_status\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_status
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_db_status\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_db_status
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"install_malloc_faultsim\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_install_malloc_faultsim
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_config_heap\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_config_heap
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_config_heap_size\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_config_heap_size
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_config_memstatus\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_config_memstatus
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_config_lookaside\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_config_lookaside
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_config_error\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_config_error
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_config_uri\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_config_uri
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_config_cis\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_config_cis
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_config_pmasz\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_config_pmasz
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_db_config_lookaside\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_db_config_lookaside
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_dump_memsys3\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_dump_memsys3
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 3 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_dump_memsys5\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_dump_memsys3
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 5 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_install_memsys3\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_install_memsys3
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
                C2Rust_Unnamed_28 {
                    zName: b"sqlite3_memdebug_vfs_oom_test\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        test_vfs_oom_test
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData: 0 as ::core::ffi::c_int,
                },
            ]
        };
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_28; 32]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_28>() as usize)
        {
            let mut c: ClientData = aObjCmd[i as usize].clientData as intptr_t
                as *mut ::core::ffi::c_void;
            Tcl_CreateObjCommand(
                interp,
                aObjCmd[i as usize].zName,
                aObjCmd[i as usize].xProc,
                c,
                None,
            );
            i += 1;
        }
        return TCL_OK;
    }
}
