use ::c2rust_bitfields;
use ::libc;
unsafe extern "C" {
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Bitvec;
    pub type sqlite3_mutex;
    pub type Pager;
    pub type PCache;
    pub type VtabCtx;
    pub type PreUpdate;
    pub type RenameToken;
    pub type Vdbe;
    pub type TableLock;
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
    fn sqlite3_mutex_free(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3PagerFile(_: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3PagerStats(_: *mut Pager) -> *mut ::core::ffi::c_int;
    fn sqlite3BtreeOpen(
        pVfs: *mut sqlite3_vfs,
        zFilename: *const ::core::ffi::c_char,
        db: *mut sqlite3,
        ppBtree: *mut *mut Btree,
        flags: ::core::ffi::c_int,
        vfsFlags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeClose(_: *mut Btree) -> ::core::ffi::c_int;
    fn sqlite3BtreeSetCacheSize(
        _: *mut Btree,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeBeginTrans(
        _: *mut Btree,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeLockTable(
        pBtree: *mut Btree,
        iTab: ::core::ffi::c_int,
        isWriteLock: u8_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCursor(
        _: *mut Btree,
        iTable: Pgno,
        wrFlag: ::core::ffi::c_int,
        _: *mut KeyInfo,
        pCursor: *mut BtCursor,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeCursorSize() -> ::core::ffi::c_int;
    fn sqlite3BtreeCloseCursor(_: *mut BtCursor) -> ::core::ffi::c_int;
    fn sqlite3BtreeInsert(
        _: *mut BtCursor,
        pPayload: *const BtreePayload,
        flags: ::core::ffi::c_int,
        seekResult: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeFirst(
        _: *mut BtCursor,
        pRes: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeNext(
        _: *mut BtCursor,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3BtreeEof(_: *mut BtCursor) -> ::core::ffi::c_int;
    fn sqlite3BtreePayloadSize(_: *mut BtCursor) -> u32_0;
    fn sqlite3BtreePager(_: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnter(_: *mut Btree);
    fn sqlite3BtreeLeave(_: *mut Btree);
    fn Tcl_Alloc(size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char;
    fn Tcl_Free(ptr: *mut ::core::ffi::c_char);
    fn Tcl_GetBoolean(
        interp: *mut Tcl_Interp,
        src: *const ::core::ffi::c_char,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetByteArrayFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_uchar;
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
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn sqlite3MutexAlloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3TestTextToPtr(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
    fn sqlite3PutVarint(_: *mut ::core::ffi::c_uchar, _: u64_0) -> ::core::ffi::c_int;
    fn sqlite3GetVarint(_: *const ::core::ffi::c_uchar, _: *mut u64_0) -> u8_0;
    fn sqlite3GetVarint32(_: *const ::core::ffi::c_uchar, _: *mut u32_0) -> u8_0;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
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
pub struct Btree {
    pub db: *mut sqlite3,
    pub pBt: *mut BtShared,
    pub inTrans: u8_0,
    pub sharable: u8_0,
    pub locked: u8_0,
    pub hasIncrblobCur: u8_0,
    pub wantToLock: ::core::ffi::c_int,
    pub nBackup: ::core::ffi::c_int,
    pub iBDataVersion: u32_0,
    pub pNext: *mut Btree,
    pub pPrev: *mut Btree,
    pub lock: BtLock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtLock {
    pub pBtree: *mut Btree,
    pub iTable: Pgno,
    pub eLock: u8_0,
    pub pNext: *mut BtLock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtShared {
    pub pPager: *mut Pager,
    pub db: *mut sqlite3,
    pub pCursor: *mut BtCursor,
    pub pPage1: *mut MemPage,
    pub openFlags: u8_0,
    pub autoVacuum: u8_0,
    pub incrVacuum: u8_0,
    pub bDoTruncate: u8_0,
    pub inTransaction: u8_0,
    pub max1bytePayload: u8_0,
    pub nReserveWanted: u8_0,
    pub btsFlags: u16_0,
    pub maxLocal: u16_0,
    pub minLocal: u16_0,
    pub maxLeaf: u16_0,
    pub minLeaf: u16_0,
    pub pageSize: u32_0,
    pub usableSize: u32_0,
    pub nTransaction: ::core::ffi::c_int,
    pub nPage: u32_0,
    pub pSchema: *mut ::core::ffi::c_void,
    pub xFreeSchema: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub mutex: *mut sqlite3_mutex,
    pub pHasContent: *mut Bitvec,
    pub nRef: ::core::ffi::c_int,
    pub pNext: *mut BtShared,
    pub pLock: *mut BtLock,
    pub pWriter: *mut Btree,
    pub pTmpSpace: *mut u8_0,
    pub nPreformatSize: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemPage {
    pub isInit: u8_0,
    pub intKey: u8_0,
    pub intKeyLeaf: u8_0,
    pub pgno: Pgno,
    pub leaf: u8_0,
    pub hdrOffset: u8_0,
    pub childPtrSize: u8_0,
    pub max1bytePayload: u8_0,
    pub nOverflow: u8_0,
    pub maxLocal: u16_0,
    pub minLocal: u16_0,
    pub cellOffset: u16_0,
    pub nFree: ::core::ffi::c_int,
    pub nCell: u16_0,
    pub maskPage: u16_0,
    pub aiOvfl: [u16_0; 4],
    pub apOvfl: [*mut u8_0; 4],
    pub pBt: *mut BtShared,
    pub aData: *mut u8_0,
    pub aDataEnd: *mut u8_0,
    pub aCellIdx: *mut u8_0,
    pub aDataOfst: *mut u8_0,
    pub pDbPage: *mut DbPage,
    pub xCellSize: Option<unsafe extern "C" fn(*mut MemPage, *mut u8_0) -> u16_0>,
    pub xParseCell: Option<
        unsafe extern "C" fn(*mut MemPage, *mut u8_0, *mut CellInfo) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellInfo {
    pub nKey: i64_0,
    pub pPayload: *mut u8_0,
    pub nPayload: u32_0,
    pub nLocal: u16_0,
    pub nSize: u16_0,
}
pub type DbPage = PgHdr;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtCursor {
    pub eState: u8_0,
    pub curFlags: u8_0,
    pub curPagerFlags: u8_0,
    pub hints: u8_0,
    pub skipNext: ::core::ffi::c_int,
    pub pBtree: *mut Btree,
    pub aOverflow: *mut Pgno,
    pub pKey: *mut ::core::ffi::c_void,
    pub pBt: *mut BtShared,
    pub pNext: *mut BtCursor,
    pub info: CellInfo,
    pub nKey: i64_0,
    pub pgnoRoot: Pgno,
    pub iPage: i8_0,
    pub curIntKey: u8_0,
    pub ix: u16_0,
    pub aiIdx: [u16_0; 19],
    pub pKeyInfo: *mut KeyInfo,
    pub pPage: *mut MemPage,
    pub apPage: [*mut MemPage; 19],
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
pub type i8_0 = int8_t;
pub type int8_t = __int8_t;
pub type __int8_t = i8;
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtreePayload {
    pub pKey: *const ::core::ffi::c_void,
    pub nKey: sqlite3_int64,
    pub pData: *const ::core::ffi::c_void,
    pub aMem: *mut sqlite3_value,
    pub nMem: u16_0,
    pub nData: ::core::ffi::c_int,
    pub nZero: ::core::ffi::c_int,
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
pub struct C2Rust_Unnamed_25 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_CmdProc>,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_MUTEX_RECURSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTREE_WRCSR: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
static mut sDb: sqlite3 = sqlite3 {
    pVfs: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
    pVdbe: ::core::ptr::null::<Vdbe>() as *mut Vdbe,
    pDfltColl: ::core::ptr::null::<CollSeq>() as *mut CollSeq,
    mutex: ::core::ptr::null::<sqlite3_mutex>() as *mut sqlite3_mutex,
    aDb: ::core::ptr::null::<Db>() as *mut Db,
    nDb: 0,
    mDbFlags: 0,
    flags: 0,
    lastRowid: 0,
    szMmap: 0,
    nSchemaLock: 0,
    openFlags: 0,
    errCode: 0,
    errByteOffset: 0,
    errMask: 0,
    iSysErrno: 0,
    dbOptFlags: 0,
    enc: 0,
    autoCommit: 0,
    temp_store: 0,
    mallocFailed: 0,
    bBenignMalloc: 0,
    dfltLockMode: 0,
    nextAutovac: 0,
    suppressErr: 0,
    vtabOnConflict: 0,
    isTransactionSavepoint: 0,
    mTrace: 0,
    noSharedCache: 0,
    nSqlExec: 0,
    eOpenState: 0,
    nextPagesize: 0,
    nChange: 0,
    nTotalChange: 0,
    aLimit: [0; 12],
    nMaxSorterMmap: 0,
    init: sqlite3InitInfo {
        newTnum: 0,
        iDb: 0,
        busy: 0,
        orphanTrigger_imposterTable_reopenMemdb: [0; 1],
        c2rust_padding: [0; 1],
        azInit: ::core::ptr::null::<*const ::core::ffi::c_char>()
            as *mut *const ::core::ffi::c_char,
    },
    nVdbeActive: 0,
    nVdbeRead: 0,
    nVdbeWrite: 0,
    nVdbeExec: 0,
    nVDestroy: 0,
    nExtension: 0,
    aExtension: ::core::ptr::null::<*mut ::core::ffi::c_void>()
        as *mut *mut ::core::ffi::c_void,
    trace: C2Rust_Unnamed_21 { xLegacy: None },
    pTraceArg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    xProfile: None,
    pProfileArg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    pCommitArg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    xCommitCallback: None,
    pRollbackArg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    xRollbackCallback: None,
    pUpdateArg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    xUpdateCallback: None,
    pAutovacPagesArg: ::core::ptr::null::<::core::ffi::c_void>()
        as *mut ::core::ffi::c_void,
    xAutovacDestr: None,
    xAutovacPages: None,
    pParse: ::core::ptr::null::<Parse>() as *mut Parse,
    pPreUpdateArg: ::core::ptr::null::<::core::ffi::c_void>()
        as *mut ::core::ffi::c_void,
    xPreUpdateCallback: None,
    pPreUpdate: ::core::ptr::null::<PreUpdate>() as *mut PreUpdate,
    xWalCallback: None,
    pWalArg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    xCollNeeded: None,
    xCollNeeded16: None,
    pCollNeededArg: ::core::ptr::null::<::core::ffi::c_void>()
        as *mut ::core::ffi::c_void,
    pErr: ::core::ptr::null::<sqlite3_value>() as *mut sqlite3_value,
    u1: C2Rust_Unnamed_17 {
        isInterrupted: 0,
    },
    lookaside: Lookaside {
        bDisable: 0,
        sz: 0,
        szTrue: 0,
        bMalloced: 0,
        nSlot: 0,
        anStat: [0; 3],
        pInit: ::core::ptr::null::<LookasideSlot>() as *mut LookasideSlot,
        pFree: ::core::ptr::null::<LookasideSlot>() as *mut LookasideSlot,
        pSmallInit: ::core::ptr::null::<LookasideSlot>() as *mut LookasideSlot,
        pSmallFree: ::core::ptr::null::<LookasideSlot>() as *mut LookasideSlot,
        pMiddle: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        pStart: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        pEnd: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        pTrueEnd: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    xAuth: None,
    pAuthArg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    xProgress: None,
    pProgressArg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    nProgressOps: 0,
    nVTrans: 0,
    aModule: Hash {
        htsize: 0,
        count: 0,
        first: ::core::ptr::null::<HashElem>() as *mut HashElem,
        ht: ::core::ptr::null::<_ht>() as *mut _ht,
    },
    pVtabCtx: ::core::ptr::null::<VtabCtx>() as *mut VtabCtx,
    aVTrans: ::core::ptr::null::<*mut VTable>() as *mut *mut VTable,
    pDisconnect: ::core::ptr::null::<VTable>() as *mut VTable,
    aFunc: Hash {
        htsize: 0,
        count: 0,
        first: ::core::ptr::null::<HashElem>() as *mut HashElem,
        ht: ::core::ptr::null::<_ht>() as *mut _ht,
    },
    aCollSeq: Hash {
        htsize: 0,
        count: 0,
        first: ::core::ptr::null::<HashElem>() as *mut HashElem,
        ht: ::core::ptr::null::<_ht>() as *mut _ht,
    },
    busyHandler: BusyHandler {
        xBusyHandler: None,
        pBusyArg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        nBusy: 0,
    },
    aDbStatic: [Db {
        zDbSName: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        pBt: ::core::ptr::null::<Btree>() as *mut Btree,
        safety_level: 0,
        bSyncSet: 0,
        pSchema: ::core::ptr::null::<Schema>() as *mut Schema,
    }; 2],
    pSavepoint: ::core::ptr::null::<Savepoint>() as *mut Savepoint,
    nAnalysisLimit: 0,
    busyTimeout: 0,
    nSavepoint: 0,
    nStatement: 0,
    nDeferredCons: 0,
    nDeferredImmCons: 0,
    pnBytesFreed: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
    pDbData: ::core::ptr::null::<DbClientData>() as *mut DbClientData,
    nSpill: 0,
};
static mut nRefSqlite3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn btree_open(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut nCache: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        let mut n: ::core::ffi::c_int = 0;
        let mut zFilename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FILENAME NCACHE FLAGS\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if Tcl_GetInt(
            interp,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut nCache,
        ) != 0
        {
            return TCL_ERROR;
        }
        nRefSqlite3 += 1;
        if nRefSqlite3 == 1 as ::core::ffi::c_int {
            sDb.pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
            sDb.mutex = sqlite3MutexAlloc(SQLITE_MUTEX_RECURSIVE);
            sqlite3_mutex_enter(sDb.mutex);
        }
        n = strlen(*argv.offset(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
        zFilename = sqlite3_malloc(n + 2 as ::core::ffi::c_int)
            as *mut ::core::ffi::c_char;
        if zFilename.is_null() {
            return TCL_ERROR;
        }
        memcpy(
            zFilename as *mut ::core::ffi::c_void,
            *argv.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (n + 1 as ::core::ffi::c_int) as size_t,
        );
        *zFilename.offset((n + 1 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_char;
        rc = sqlite3BtreeOpen(
            sDb.pVfs,
            zFilename,
            &raw mut sDb,
            &raw mut pBt,
            0 as ::core::ffi::c_int,
            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_MAIN_DB,
        );
        sqlite3_free(zFilename as *mut ::core::ffi::c_void);
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        sqlite3BtreeSetCacheSize(pBt, nCache);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%p\0".as_ptr() as *const ::core::ffi::c_char,
            pBt,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return TCL_OK;
    }
}
unsafe extern "C" fn btree_close(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
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
        pBt = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Btree;
        rc = sqlite3BtreeClose(pBt);
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        nRefSqlite3 -= 1;
        if nRefSqlite3 == 0 as ::core::ffi::c_int {
            sqlite3_mutex_leave(sDb.mutex);
            sqlite3_mutex_free(sDb.mutex);
            sDb.mutex = ::core::ptr::null_mut::<sqlite3_mutex>();
            sDb.pVfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn btree_begin_transaction(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
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
        pBt = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Btree;
        sqlite3BtreeEnter(pBt);
        rc = sqlite3BtreeBeginTrans(
            pBt,
            1 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        sqlite3BtreeLeave(pBt);
        if rc != SQLITE_OK {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn btree_pager_stats(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
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
        pBt = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Btree;
        sqlite3_mutex_enter((*(*pBt).db).mutex);
        sqlite3BtreeEnter(pBt);
        a = sqlite3PagerStats(sqlite3BtreePager(pBt) as *mut Pager);
        i = 0 as ::core::ffi::c_int;
        while i < 11 as ::core::ffi::c_int {
            static mut zName: [*mut ::core::ffi::c_char; 11] = [
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
                b"read\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                b"write\0".as_ptr() as *const ::core::ffi::c_char
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
        sqlite3BtreeLeave(pBt);
        sqlite3_mutex_leave((*(*pBt).db).mutex);
        return TCL_OK;
    }
}
unsafe extern "C" fn btree_cursor(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
        let mut iTable: ::core::ffi::c_int = 0;
        let mut pCur: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut wrFlag: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 30] = [0; 30];
        if argc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" ID TABLENUM WRITEABLE\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pBt = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Btree;
        if Tcl_GetInt(
            interp,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut iTable,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetBoolean(
            interp,
            *argv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut wrFlag,
        ) != 0
        {
            return TCL_ERROR;
        }
        if wrFlag != 0 {
            wrFlag = BTREE_WRCSR;
        }
        pCur = Tcl_Alloc(sqlite3BtreeCursorSize() as ::core::ffi::c_uint)
            as *mut ::core::ffi::c_void as *mut BtCursor;
        memset(
            pCur as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            sqlite3BtreeCursorSize() as size_t,
        );
        sqlite3_mutex_enter((*(*pBt).db).mutex);
        sqlite3BtreeEnter(pBt);
        rc = sqlite3BtreeLockTable(
            pBt,
            iTable,
            (wrFlag != 0) as ::core::ffi::c_int as u8_0,
        );
        if rc == SQLITE_OK {
            rc = sqlite3BtreeCursor(
                pBt,
                iTable as Pgno,
                wrFlag,
                ::core::ptr::null_mut::<KeyInfo>(),
                pCur,
            );
        }
        sqlite3BtreeLeave(pBt);
        sqlite3_mutex_leave((*(*pBt).db).mutex);
        if rc != 0 {
            Tcl_Free(pCur as *mut ::core::ffi::c_char);
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 30]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%p\0".as_ptr() as *const ::core::ffi::c_char,
            pCur,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btree_close_cursor(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
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
        pCur = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut BtCursor;
        let mut pBt: *mut Btree = (*pCur).pBtree;
        sqlite3_mutex_enter((*(*pBt).db).mutex);
        sqlite3BtreeEnter(pBt);
        rc = sqlite3BtreeCloseCursor(pCur);
        sqlite3BtreeLeave(pBt);
        sqlite3_mutex_leave((*(*pBt).db).mutex);
        Tcl_Free(pCur as *mut ::core::ffi::c_char);
        if rc != 0 {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btree_next(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
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
        pCur = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut BtCursor;
        sqlite3BtreeEnter((*pCur).pBtree);
        rc = sqlite3BtreeNext(pCur, 0 as ::core::ffi::c_int);
        if rc == SQLITE_DONE {
            res = 1 as ::core::ffi::c_int;
            rc = SQLITE_OK;
        }
        sqlite3BtreeLeave((*pCur).pBtree);
        if rc != 0 {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            res,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btree_first(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
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
        pCur = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut BtCursor;
        sqlite3BtreeEnter((*pCur).pBtree);
        rc = sqlite3BtreeFirst(pCur, &raw mut res);
        sqlite3BtreeLeave((*pCur).pBtree);
        if rc != 0 {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            res,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btree_eof(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
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
        pCur = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut BtCursor;
        sqlite3BtreeEnter((*pCur).pBtree);
        rc = sqlite3BtreeEof(pCur);
        sqlite3BtreeLeave((*pCur).pBtree);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 50]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btree_payload_size(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        let mut n: u32_0 = 0;
        let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
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
        pCur = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut BtCursor;
        sqlite3BtreeEnter((*pCur).pBtree);
        n = sqlite3BtreePayloadSize(pCur);
        sqlite3BtreeLeave((*pCur).pBtree);
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 50]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        );
        Tcl_AppendResult(interp, &raw mut zBuf as *mut ::core::ffi::c_char, NULL);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btree_varint_test(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut start: u32_0 = 0;
        let mut mult: u32_0 = 0;
        let mut count: u32_0 = 0;
        let mut incr: u32_0 = 0;
        let mut in_0: u64_0 = 0;
        let mut out: u64_0 = 0;
        let mut n1: ::core::ffi::c_int = 0;
        let mut n2: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_uchar; 100] = [0; 100];
        if argc != 5 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" START MULTIPLIER COUNT INCREMENT\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if Tcl_GetInt(
            interp,
            *argv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut start as *mut ::core::ffi::c_int,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetInt(
            interp,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut mult as *mut ::core::ffi::c_int,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetInt(
            interp,
            *argv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut count as *mut ::core::ffi::c_int,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetInt(
            interp,
            *argv.offset(4 as ::core::ffi::c_int as isize),
            &raw mut incr as *mut ::core::ffi::c_int,
        ) != 0
        {
            return TCL_ERROR;
        }
        in_0 = start as u64_0;
        in_0 = (in_0 as ::core::ffi::c_ulonglong)
            .wrapping_mul(mult as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
        i = 0 as ::core::ffi::c_int;
        while i < count as ::core::ffi::c_int {
            let mut zErr: [::core::ffi::c_char; 200] = [0; 200];
            n1 = sqlite3PutVarint(&raw mut zBuf as *mut ::core::ffi::c_uchar, in_0);
            if n1 > 9 as ::core::ffi::c_int || n1 < 1 as ::core::ffi::c_int {
                sqlite3_snprintf(
                    ::core::mem::size_of::<[::core::ffi::c_char; 200]>()
                        as ::core::ffi::c_int,
                    &raw mut zErr as *mut ::core::ffi::c_char,
                    b"putVarint returned %d - should be between 1 and 9\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    n1,
                );
                Tcl_AppendResult(
                    interp,
                    &raw mut zErr as *mut ::core::ffi::c_char,
                    NULL,
                );
                return TCL_ERROR;
            }
            n2 = sqlite3GetVarint(
                &raw mut zBuf as *mut ::core::ffi::c_uchar,
                &raw mut out,
            ) as ::core::ffi::c_int;
            if n1 != n2 {
                sqlite3_snprintf(
                    ::core::mem::size_of::<[::core::ffi::c_char; 200]>()
                        as ::core::ffi::c_int,
                    &raw mut zErr as *mut ::core::ffi::c_char,
                    b"putVarint returned %d and getVarint returned %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    n1,
                    n2,
                );
                Tcl_AppendResult(
                    interp,
                    &raw mut zErr as *mut ::core::ffi::c_char,
                    NULL,
                );
                return TCL_ERROR;
            }
            if in_0 != out {
                sqlite3_snprintf(
                    ::core::mem::size_of::<[::core::ffi::c_char; 200]>()
                        as ::core::ffi::c_int,
                    &raw mut zErr as *mut ::core::ffi::c_char,
                    b"Wrote 0x%016llx and got back 0x%016llx\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    in_0,
                    out,
                );
                Tcl_AppendResult(
                    interp,
                    &raw mut zErr as *mut ::core::ffi::c_char,
                    NULL,
                );
                return TCL_ERROR;
            }
            if in_0 as ::core::ffi::c_ulonglong & 0xffffffff as ::core::ffi::c_ulonglong
                == in_0
            {
                let mut out32: u32_0 = 0;
                n2 = (if (*(&raw mut zBuf as *mut ::core::ffi::c_uchar)
                    as ::core::ffi::c_int)
                    < 0x80 as ::core::ffi::c_int as u8_0 as ::core::ffi::c_int
                {
                    out32 = *(&raw mut zBuf as *mut ::core::ffi::c_uchar) as u32_0;
                    1 as ::core::ffi::c_int
                } else {
                    sqlite3GetVarint32(
                        &raw mut zBuf as *mut ::core::ffi::c_uchar,
                        &raw mut out32,
                    ) as ::core::ffi::c_int
                }) as u8_0 as ::core::ffi::c_int;
                out = out32 as u64_0;
                if n1 != n2 {
                    sqlite3_snprintf(
                        ::core::mem::size_of::<[::core::ffi::c_char; 200]>()
                            as ::core::ffi::c_int,
                        &raw mut zErr as *mut ::core::ffi::c_char,
                        b"putVarint returned %d and GetVarint32 returned %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        n1,
                        n2,
                    );
                    Tcl_AppendResult(
                        interp,
                        &raw mut zErr as *mut ::core::ffi::c_char,
                        NULL,
                    );
                    return TCL_ERROR;
                }
                if in_0 != out {
                    sqlite3_snprintf(
                        ::core::mem::size_of::<[::core::ffi::c_char; 200]>()
                            as ::core::ffi::c_int,
                        &raw mut zErr as *mut ::core::ffi::c_char,
                        b"Wrote 0x%016llx and got back 0x%016llx from GetVarint32\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        in_0,
                        out,
                    );
                    Tcl_AppendResult(
                        interp,
                        &raw mut zErr as *mut ::core::ffi::c_char,
                        NULL,
                    );
                    return TCL_ERROR;
                }
            }
            j = 0 as ::core::ffi::c_int;
            while j < 19 as ::core::ffi::c_int {
                sqlite3GetVarint(
                    &raw mut zBuf as *mut ::core::ffi::c_uchar,
                    &raw mut out,
                );
                j += 1;
            }
            in_0 = (in_0 as ::core::ffi::c_ulonglong)
                .wrapping_add(incr as ::core::ffi::c_ulonglong) as u64_0 as u64_0;
            i += 1;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn btree_from_db(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zBuf: [::core::ffi::c_char; 100] = [0; 100];
        let mut info: Tcl_CmdInfo = Tcl_CmdInfo {
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
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
        let mut iDb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if argc != 2 as ::core::ffi::c_int && argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" DB-HANDLE ?N?\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if 1 as ::core::ffi::c_int
            != Tcl_GetCommandInfo(
                interp,
                *argv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut info,
            )
        {
            Tcl_AppendResult(
                interp,
                b"No such db-handle: \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(1 as ::core::ffi::c_int as isize),
                b"\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        if argc == 3 as ::core::ffi::c_int {
            iDb = atoi(*argv.offset(2 as ::core::ffi::c_int as isize));
        }
        db = *(info.objClientData as *mut *mut sqlite3);
        pBt = (*(*db).aDb.offset(iDb as isize)).pBt;
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            b"%p\0".as_ptr() as *const ::core::ffi::c_char,
            pBt,
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
    }
}
unsafe extern "C" fn btree_ismemdb(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
        let mut res: ::core::ffi::c_int = 0;
        let mut pFile: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
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
        pBt = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Btree;
        sqlite3_mutex_enter((*(*pBt).db).mutex);
        sqlite3BtreeEnter(pBt);
        pFile = sqlite3PagerFile(sqlite3BtreePager(pBt) as *mut Pager);
        res = (*pFile).pMethods.is_null() as ::core::ffi::c_int;
        sqlite3BtreeLeave(pBt);
        sqlite3_mutex_leave((*(*pBt).db).mutex);
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj((res != 0 as ::core::ffi::c_int) as ::core::ffi::c_int),
        );
        return SQLITE_OK;
    }
}
unsafe extern "C" fn btree_set_cache_size(
    mut NotUsed: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nCache: ::core::ffi::c_int = 0;
        let mut pBt: *mut Btree = ::core::ptr::null_mut::<Btree>();
        if argc != 3 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" BT NCACHE\"\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        pBt = sqlite3TestTextToPtr(*argv.offset(1 as ::core::ffi::c_int as isize))
            as *mut Btree;
        if Tcl_GetInt(
            interp,
            *argv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut nCache,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_mutex_enter((*(*pBt).db).mutex);
        sqlite3BtreeEnter(pBt);
        sqlite3BtreeSetCacheSize(pBt, nCache);
        sqlite3BtreeLeave(pBt);
        sqlite3_mutex_leave((*(*pBt).db).mutex);
        return TCL_OK;
    }
}
unsafe extern "C" fn btree_insert(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut BtCursor = ::core::ptr::null_mut::<BtCursor>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut x: BtreePayload = BtreePayload {
            pKey: ::core::ptr::null::<::core::ffi::c_void>(),
            nKey: 0,
            pData: ::core::ptr::null::<::core::ffi::c_void>(),
            aMem: ::core::ptr::null_mut::<sqlite3_value>(),
            nMem: 0,
            nData: 0,
            nZero: 0,
        };
        let mut n: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?-intkey? CSR KEY VALUE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        memset(
            &raw mut x as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<BtreePayload>() as size_t,
        );
        if objc == 4 as ::core::ffi::c_int {
            if Tcl_GetIntFromObj(
                interp,
                *objv.offset(2 as ::core::ffi::c_int as isize),
                &raw mut rc,
            ) != 0
            {
                return TCL_ERROR;
            }
            x.nKey = rc as sqlite3_int64;
            x.pData = Tcl_GetByteArrayFromObj(
                *objv.offset(3 as ::core::ffi::c_int as isize),
                &raw mut n,
            ) as *mut ::core::ffi::c_void;
            x.nData = n;
        } else {
            x.pKey = Tcl_GetByteArrayFromObj(
                *objv.offset(2 as ::core::ffi::c_int as isize),
                &raw mut n,
            ) as *mut ::core::ffi::c_void;
            x.nKey = n as sqlite3_int64;
        }
        pCur = sqlite3TestTextToPtr(
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        ) as *mut BtCursor;
        sqlite3_mutex_enter((*(*(*pCur).pBtree).db).mutex);
        sqlite3BtreeEnter((*pCur).pBtree);
        rc = sqlite3BtreeInsert(
            pCur,
            &raw mut x,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        sqlite3BtreeLeave((*pCur).pBtree);
        sqlite3_mutex_leave((*(*(*pCur).pBtree).db).mutex);
        Tcl_ResetResult(interp);
        if rc != 0 {
            Tcl_AppendResult(interp, sqlite3ErrName(rc), NULL);
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest3_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aCmd: [C2Rust_Unnamed_25; 14] = unsafe {
            [
                C2Rust_Unnamed_25 {
                    zName: b"btree_open\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_open
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
                    zName: b"btree_close\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_close
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
                    zName: b"btree_begin_transaction\0".as_ptr()
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
                            btree_begin_transaction
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
                    zName: b"btree_pager_stats\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_pager_stats
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
                    zName: b"btree_cursor\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_cursor
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
                    zName: b"btree_close_cursor\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_close_cursor
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
                    zName: b"btree_next\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_next
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
                    zName: b"btree_eof\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_eof
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
                    zName: b"btree_payload_size\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_payload_size
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
                    zName: b"btree_first\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_first
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
                    zName: b"btree_varint_test\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_varint_test
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
                    zName: b"btree_from_db\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_from_db
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
                    zName: b"btree_ismemdb\0".as_ptr() as *const ::core::ffi::c_char
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
                            btree_ismemdb
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
                    zName: b"btree_set_cache_size\0".as_ptr()
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
                            btree_set_cache_size
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
            < (::core::mem::size_of::<[C2Rust_Unnamed_25; 14]>() as usize)
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
        Tcl_CreateObjCommand(
            interp,
            b"btree_insert\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                btree_insert
                    as unsafe extern "C" fn(
                        ClientData,
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
