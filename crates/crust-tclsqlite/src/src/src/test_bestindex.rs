use ::c2rust_bitfields;
use ::libc;
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
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_int64(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> sqlite3_int64;
    fn sqlite3_column_value(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *mut sqlite3_value;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
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
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vtab_collation(
        _: *mut sqlite3_index_info,
        _: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_vtab_distinct(_: *mut sqlite3_index_info) -> ::core::ffi::c_int;
    fn sqlite3_vtab_in(
        _: *mut sqlite3_index_info,
        iCons: ::core::ffi::c_int,
        bHandle: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vtab_in_first(
        pVal: *mut sqlite3_value,
        ppOut: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vtab_in_next(
        pVal: *mut sqlite3_value,
        ppOut: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vtab_rhs_value(
        _: *mut sqlite3_index_info,
        _: ::core::ffi::c_int,
        ppVal: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
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
    fn Tcl_Alloc(size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char;
    fn Tcl_Free(ptr: *mut ::core::ffi::c_char);
    fn Tcl_DuplicateObj(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_GetBooleanFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetDoubleFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        doublePtr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_NewStringObj(
        bytes: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_DeleteCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetObjResult(interp: *mut Tcl_Interp) -> *mut Tcl_Obj;
    fn Tcl_GetStringResult(interp: *mut Tcl_Interp) -> *const ::core::ffi::c_char;
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
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn Tcl_GetWideIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        widePtr: *mut Tcl_WideInt,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn getDbPointer(
        interp: *mut Tcl_Interp,
        zA: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type size_t = usize;
pub type tRowcnt = u64_0;
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
pub struct tcl_vtab {
    pub base: sqlite3_vtab,
    pub interp: *mut Tcl_Interp,
    pub pCmd: *mut Tcl_Obj,
    pub pFindFunctionList: *mut TestFindFunction,
    pub db: *mut sqlite3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestFindFunction {
    pub pTab: *mut tcl_vtab,
    pub zName: *const ::core::ffi::c_char,
    pub pNext: *mut TestFindFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcl_cursor {
    pub base: sqlite3_vtab_cursor,
    pub pStmt: *mut sqlite3_stmt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestVtabContext {
    pub interp: *mut Tcl_Interp,
    pub pDefault: *mut Tcl_Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
    pub clientData: *mut ::core::ffi::c_void,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_SCHEMA: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2;
pub const SQLITE_INDEX_CONSTRAINT_GT: ::core::ffi::c_int = 4;
pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8;
pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16;
pub const SQLITE_INDEX_CONSTRAINT_GE: ::core::ffi::c_int = 32;
pub const SQLITE_INDEX_CONSTRAINT_MATCH: ::core::ffi::c_int = 64;
pub const SQLITE_INDEX_CONSTRAINT_LIKE: ::core::ffi::c_int = 65;
pub const SQLITE_INDEX_CONSTRAINT_GLOB: ::core::ffi::c_int = 66;
pub const SQLITE_INDEX_CONSTRAINT_REGEXP: ::core::ffi::c_int = 67;
pub const SQLITE_INDEX_CONSTRAINT_NE: ::core::ffi::c_int = 68;
pub const SQLITE_INDEX_CONSTRAINT_ISNOT: ::core::ffi::c_int = 69;
pub const SQLITE_INDEX_CONSTRAINT_ISNOTNULL: ::core::ffi::c_int = 70;
pub const SQLITE_INDEX_CONSTRAINT_ISNULL: ::core::ffi::c_int = 71;
pub const SQLITE_INDEX_CONSTRAINT_IS: ::core::ffi::c_int = 72;
pub const SQLITE_INDEX_CONSTRAINT_LIMIT: ::core::ffi::c_int = 73;
pub const SQLITE_INDEX_CONSTRAINT_OFFSET: ::core::ffi::c_int = 74;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_EVAL_GLOBAL: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
unsafe extern "C" fn tclDequote(mut z: *mut ::core::ffi::c_char) {
    unsafe {
        let mut q: ::core::ffi::c_char = *z.offset(0 as ::core::ffi::c_int as isize);
        if q as ::core::ffi::c_int == '[' as i32
            || q as ::core::ffi::c_int == '\'' as i32
            || q as ::core::ffi::c_int == '"' as i32
            || q as ::core::ffi::c_int == '`' as i32
        {
            let mut iIn: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if q as ::core::ffi::c_int == '[' as i32 {
                q = ']' as i32 as ::core::ffi::c_char;
            }
            while *z.offset(iIn as isize) != 0 {
                if *z.offset(iIn as isize) as ::core::ffi::c_int
                    == q as ::core::ffi::c_int
                {
                    if *z.offset((iIn + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int != q as ::core::ffi::c_int
                    {
                        iIn += 1;
                        break;
                    } else {
                        iIn += 2 as ::core::ffi::c_int;
                        let c2rust_fresh0 = iOut;
                        iOut = iOut + 1;
                        *z.offset(c2rust_fresh0 as isize) = q;
                    }
                } else {
                    let c2rust_fresh1 = iIn;
                    iIn = iIn + 1;
                    let c2rust_fresh2 = iOut;
                    iOut = iOut + 1;
                    *z.offset(c2rust_fresh2 as isize) = *z
                        .offset(c2rust_fresh1 as isize);
                }
            }
            *z.offset(iOut as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
    }
}
unsafe extern "C" fn tclConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCtx: *mut TestVtabContext = pAux as *mut TestVtabContext;
        let mut interp: *mut Tcl_Interp = (*pCtx).interp;
        let mut pTab: *mut tcl_vtab = ::core::ptr::null_mut::<tcl_vtab>();
        let mut zCmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pScript: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if argc != 4 as ::core::ffi::c_int
            && (argc != 3 as ::core::ffi::c_int || (*pCtx).pDefault.is_null())
        {
            *pzErr = sqlite3_mprintf(
                b"wrong number of arguments\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return SQLITE_ERROR;
        }
        if argc == 4 as ::core::ffi::c_int {
            zCmd = sqlite3_malloc64(
                strlen(*argv.offset(3 as ::core::ffi::c_int as isize))
                    .wrapping_add(1 as size_t) as sqlite3_uint64,
            ) as *mut ::core::ffi::c_char;
        }
        pTab = sqlite3_malloc64(::core::mem::size_of::<tcl_vtab>() as sqlite3_uint64)
            as *mut tcl_vtab;
        if (!zCmd.is_null() || argc == 3 as ::core::ffi::c_int) && !pTab.is_null() {
            memset(
                pTab as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<tcl_vtab>() as size_t,
            );
            if !zCmd.is_null() {
                memcpy(
                    zCmd as *mut ::core::ffi::c_void,
                    *argv.offset(3 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    strlen(*argv.offset(3 as ::core::ffi::c_int as isize))
                        .wrapping_add(1 as size_t),
                );
                tclDequote(zCmd);
                (*pTab).pCmd = Tcl_NewStringObj(zCmd, -1 as ::core::ffi::c_int);
            } else {
                (*pTab).pCmd = Tcl_DuplicateObj((*pCtx).pDefault);
            }
            (*pTab).interp = interp;
            (*pTab).db = db;
            (*(*pTab).pCmd).refCount += 1;
            pScript = Tcl_DuplicateObj((*pTab).pCmd);
            (*pScript).refCount += 1;
            Tcl_ListObjAppendElement(
                interp,
                pScript,
                Tcl_NewStringObj(
                    b"xConnect\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
            rc = Tcl_EvalObjEx(interp, pScript, TCL_EVAL_GLOBAL);
            if rc != TCL_OK {
                *pzErr = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    Tcl_GetStringResult(interp),
                );
                if sqlite3_stricmp(
                    *pzErr,
                    b"database schema has changed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    rc = SQLITE_SCHEMA;
                } else {
                    rc = SQLITE_ERROR;
                }
            } else {
                rc = sqlite3_declare_vtab(db, Tcl_GetStringResult(interp));
                if rc != SQLITE_OK {
                    *pzErr = sqlite3_mprintf(
                        b"declare_vtab: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        sqlite3_errmsg(db),
                    );
                }
            }
            if rc != SQLITE_OK {
                sqlite3_free(pTab as *mut ::core::ffi::c_void);
                pTab = ::core::ptr::null_mut::<tcl_vtab>();
            }
        } else {
            rc = SQLITE_NOMEM;
        }
        sqlite3_free(zCmd as *mut ::core::ffi::c_void);
        *ppVtab = if !pTab.is_null() {
            &raw mut (*pTab).base
        } else {
            ::core::ptr::null_mut::<sqlite3_vtab>()
        };
        return rc;
    }
}
unsafe extern "C" fn tclDisconnect(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut tcl_vtab = pVtab as *mut tcl_vtab;
        while !(*pTab).pFindFunctionList.is_null() {
            let mut p: *mut TestFindFunction = (*pTab).pFindFunctionList;
            (*pTab).pFindFunctionList = (*p).pNext;
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
        let mut _objPtr: *mut Tcl_Obj = (*pTab).pCmd;
        let c2rust_fresh3 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh3 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        sqlite3_free(pTab as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tclOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut tcl_cursor = ::core::ptr::null_mut::<tcl_cursor>();
        pCur = sqlite3_malloc(::core::mem::size_of::<tcl_cursor>() as ::core::ffi::c_int)
            as *mut tcl_cursor;
        if pCur.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pCur as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<tcl_cursor>() as size_t,
        );
        *ppCursor = &raw mut (*pCur).base;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tclClose(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut tcl_cursor = cur as *mut tcl_cursor;
        if !pCur.is_null() {
            sqlite3_finalize((*pCur).pStmt);
            sqlite3_free(pCur as *mut ::core::ffi::c_void);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tclNext(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut tcl_cursor = pVtabCursor as *mut tcl_cursor;
        if !(*pCsr).pStmt.is_null() {
            let mut pTab: *mut tcl_vtab = (*pVtabCursor).pVtab as *mut tcl_vtab;
            let mut rc: ::core::ffi::c_int = sqlite3_step((*pCsr).pStmt);
            if rc != SQLITE_ROW {
                let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                rc = sqlite3_finalize((*pCsr).pStmt);
                (*pCsr).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                if rc != SQLITE_OK {
                    zErr = sqlite3_errmsg((*pTab).db);
                    (*pTab).base.zErrMsg = sqlite3_mprintf(
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        zErr,
                    );
                }
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tclFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut tcl_cursor = pVtabCursor as *mut tcl_cursor;
        let mut pTab: *mut tcl_vtab = (*pVtabCursor).pVtab as *mut tcl_vtab;
        let mut interp: *mut Tcl_Interp = (*pTab).interp;
        let mut pScript: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pArg: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut ii: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        pScript = Tcl_DuplicateObj((*pTab).pCmd);
        (*pScript).refCount += 1;
        Tcl_ListObjAppendElement(
            interp,
            pScript,
            Tcl_NewStringObj(
                b"xFilter\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(interp, pScript, Tcl_NewIntObj(idxNum));
        Tcl_ListObjAppendElement(
            interp,
            pScript,
            Tcl_NewStringObj(
                if !idxStr.is_null() {
                    idxStr
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                -1 as ::core::ffi::c_int,
            ),
        );
        pArg = Tcl_NewObj();
        (*pArg).refCount += 1;
        ii = 0 as ::core::ffi::c_int;
        while ii < argc {
            let mut zVal: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(ii as isize),
            ) as *const ::core::ffi::c_char;
            let mut pVal: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
            if zVal.is_null() {
                let mut pMem: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                pVal = Tcl_NewObj();
                rc = sqlite3_vtab_in_first(*argv.offset(ii as isize), &raw mut pMem);
                while rc == SQLITE_OK && !pMem.is_null() {
                    let mut pVal2: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                    zVal = sqlite3_value_text(pMem) as *const ::core::ffi::c_char;
                    if !zVal.is_null() {
                        pVal2 = Tcl_NewStringObj(zVal, -1 as ::core::ffi::c_int);
                    } else {
                        pVal2 = Tcl_NewObj();
                    }
                    Tcl_ListObjAppendElement(interp, pVal, pVal2);
                    rc = sqlite3_vtab_in_next(*argv.offset(ii as isize), &raw mut pMem);
                }
            } else {
                pVal = Tcl_NewStringObj(zVal, -1 as ::core::ffi::c_int);
            }
            Tcl_ListObjAppendElement(interp, pArg, pVal);
            ii += 1;
        }
        Tcl_ListObjAppendElement(interp, pScript, pArg);
        let mut _objPtr: *mut Tcl_Obj = pArg;
        let c2rust_fresh4 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh4 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        rc = Tcl_EvalObjEx(interp, pScript, TCL_EVAL_GLOBAL);
        if rc != TCL_OK {
            let mut zErr: *const ::core::ffi::c_char = Tcl_GetStringResult(interp);
            rc = SQLITE_ERROR;
            (*pTab).base.zErrMsg = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                zErr,
            );
        } else {
            let mut pRes: *mut Tcl_Obj = Tcl_GetObjResult(interp);
            let mut apElem: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<*mut Tcl_Obj>();
            let mut nElem: ::core::ffi::c_int = 0;
            rc = Tcl_ListObjGetElements(interp, pRes, &raw mut nElem, &raw mut apElem);
            if rc != TCL_OK {
                let mut zErr_0: *const ::core::ffi::c_char = Tcl_GetStringResult(interp);
                rc = SQLITE_ERROR;
                (*pTab).base.zErrMsg = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    zErr_0,
                );
            } else {
                ii = 0 as ::core::ffi::c_int;
                while rc == SQLITE_OK && ii < nElem {
                    let mut zCmd: *const ::core::ffi::c_char = Tcl_GetString(
                        *apElem.offset(ii as isize),
                    );
                    let mut p: *mut Tcl_Obj = *apElem
                        .offset((ii + 1 as ::core::ffi::c_int) as isize);
                    if sqlite3_stricmp(
                        b"sql\0".as_ptr() as *const ::core::ffi::c_char,
                        zCmd,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut zSql: *const ::core::ffi::c_char = Tcl_GetString(p);
                        rc = sqlite3_prepare_v2(
                            (*pTab).db,
                            zSql,
                            -1 as ::core::ffi::c_int,
                            &raw mut (*pCsr).pStmt,
                            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                        );
                        if rc != SQLITE_OK {
                            let mut zErr_1: *const ::core::ffi::c_char = sqlite3_errmsg(
                                (*pTab).db,
                            );
                            (*pTab).base.zErrMsg = sqlite3_mprintf(
                                b"unexpected: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                zErr_1,
                            );
                        }
                    } else {
                        rc = SQLITE_ERROR;
                        (*pTab).base.zErrMsg = sqlite3_mprintf(
                            b"unexpected: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            zCmd,
                        );
                    }
                    ii += 2 as ::core::ffi::c_int;
                }
            }
        }
        if rc == SQLITE_OK {
            rc = tclNext(pVtabCursor);
        }
        return rc;
    }
}
unsafe extern "C" fn tclColumn(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut tcl_cursor = pVtabCursor as *mut tcl_cursor;
        sqlite3_result_value(
            ctx,
            sqlite3_column_value((*pCsr).pStmt, i + 1 as ::core::ffi::c_int),
        );
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tclRowid(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut tcl_cursor = pVtabCursor as *mut tcl_cursor;
        *pRowid = sqlite3_column_int64((*pCsr).pStmt, 0 as ::core::ffi::c_int)
            as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tclEof(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut tcl_cursor = pVtabCursor as *mut tcl_cursor;
        return (*pCsr).pStmt.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn testBestIndexObjConstraints(
    mut interp: *mut Tcl_Interp,
    mut pIdxInfo: *mut sqlite3_index_info,
) {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        let mut pRes: *mut Tcl_Obj = Tcl_NewObj();
        (*pRes).refCount += 1;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pIdxInfo).nConstraint {
            let mut pCons: *const sqlite3_index_constraint = (*pIdxInfo)
                .aConstraint
                .offset(ii as isize) as *mut sqlite3_index_constraint;
            let mut pElem: *mut Tcl_Obj = Tcl_NewObj();
            let mut zOp: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            (*pElem).refCount += 1;
            match (*pCons).op as ::core::ffi::c_int {
                SQLITE_INDEX_CONSTRAINT_EQ => {
                    zOp = b"eq\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_GT => {
                    zOp = b"gt\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_LE => {
                    zOp = b"le\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_LT => {
                    zOp = b"lt\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_GE => {
                    zOp = b"ge\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_MATCH => {
                    zOp = b"match\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_LIKE => {
                    zOp = b"like\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_GLOB => {
                    zOp = b"glob\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_REGEXP => {
                    zOp = b"regexp\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_NE => {
                    zOp = b"ne\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_ISNOT => {
                    zOp = b"isnot\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_ISNOTNULL => {
                    zOp = b"isnotnull\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_ISNULL => {
                    zOp = b"isnull\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_IS => {
                    zOp = b"is\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_LIMIT => {
                    zOp = b"limit\0".as_ptr() as *const ::core::ffi::c_char;
                }
                SQLITE_INDEX_CONSTRAINT_OFFSET => {
                    zOp = b"offset\0".as_ptr() as *const ::core::ffi::c_char;
                }
                _ => {}
            }
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pElem,
                Tcl_NewStringObj(
                    b"op\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
            if !zOp.is_null() {
                Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pElem,
                    Tcl_NewStringObj(zOp, -1 as ::core::ffi::c_int),
                );
            } else {
                Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pElem,
                    Tcl_NewIntObj((*pCons).op as ::core::ffi::c_int),
                );
            }
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pElem,
                Tcl_NewStringObj(
                    b"column\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pElem,
                Tcl_NewIntObj((*pCons).iColumn),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pElem,
                Tcl_NewStringObj(
                    b"usable\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pElem,
                Tcl_NewIntObj((*pCons).usable as ::core::ffi::c_int),
            );
            Tcl_ListObjAppendElement(::core::ptr::null_mut::<Tcl_Interp>(), pRes, pElem);
            let mut _objPtr: *mut Tcl_Obj = pElem;
            let c2rust_fresh5 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh5 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
            ii += 1;
        }
        Tcl_SetObjResult(interp, pRes);
        let mut _objPtr_0: *mut Tcl_Obj = pRes;
        let c2rust_fresh6 = (*_objPtr_0).refCount;
        (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
        if c2rust_fresh6 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr_0);
        }
    }
}
unsafe extern "C" fn testBestIndexObjOrderby(
    mut interp: *mut Tcl_Interp,
    mut pIdxInfo: *mut sqlite3_index_info,
) {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        let mut pRes: *mut Tcl_Obj = Tcl_NewObj();
        (*pRes).refCount += 1;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pIdxInfo).nOrderBy {
            let mut pOrder: *const sqlite3_index_orderby = (*pIdxInfo)
                .aOrderBy
                .offset(ii as isize) as *mut sqlite3_index_orderby;
            let mut pElem: *mut Tcl_Obj = Tcl_NewObj();
            (*pElem).refCount += 1;
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pElem,
                Tcl_NewStringObj(
                    b"column\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pElem,
                Tcl_NewIntObj((*pOrder).iColumn),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pElem,
                Tcl_NewStringObj(
                    b"desc\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pElem,
                Tcl_NewIntObj((*pOrder).desc as ::core::ffi::c_int),
            );
            Tcl_ListObjAppendElement(::core::ptr::null_mut::<Tcl_Interp>(), pRes, pElem);
            let mut _objPtr: *mut Tcl_Obj = pElem;
            let c2rust_fresh7 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh7 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
            ii += 1;
        }
        Tcl_SetObjResult(interp, pRes);
        let mut _objPtr_0: *mut Tcl_Obj = pRes;
        let c2rust_fresh8 = (*_objPtr_0).refCount;
        (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
        if c2rust_fresh8 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr_0);
        }
    }
}
unsafe extern "C" fn testBestIndexObj(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut azSub: [*const ::core::ffi::c_char; 8] = [
            b"constraints\0".as_ptr() as *const ::core::ffi::c_char,
            b"orderby\0".as_ptr() as *const ::core::ffi::c_char,
            b"mask\0".as_ptr() as *const ::core::ffi::c_char,
            b"distinct\0".as_ptr() as *const ::core::ffi::c_char,
            b"in\0".as_ptr() as *const ::core::ffi::c_char,
            b"rhs_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"collation\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut ii: ::core::ffi::c_int = 0;
        let mut pIdxInfo: *mut sqlite3_index_info = clientData
            as *mut sqlite3_index_info;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUB-COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut azSub as *mut *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as ::core::ffi::c_int,
            b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut ii,
        ) != 0
        {
            return TCL_ERROR;
        }
        if ii < 4 as ::core::ffi::c_int && objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if ii == 4 as ::core::ffi::c_int && objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"INDEX BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if ii == 5 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int
            && objc != 4 as ::core::ffi::c_int
        {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"INDEX ?DEFAULT?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        match ii {
            0 => {
                testBestIndexObjConstraints(interp, pIdxInfo);
            }
            1 => {
                testBestIndexObjOrderby(interp, pIdxInfo);
            }
            2 => {
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewWideIntObj((*pIdxInfo).colUsed as Tcl_WideInt),
                );
            }
            3 => {
                let mut bDistinct: ::core::ffi::c_int = sqlite3_vtab_distinct(pIdxInfo);
                Tcl_SetObjResult(interp, Tcl_NewIntObj(bDistinct));
            }
            4 => {
                let mut iCons: ::core::ffi::c_int = 0;
                let mut bHandle: ::core::ffi::c_int = 0;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iCons,
                ) != 0
                    || Tcl_GetBooleanFromObj(
                        interp,
                        *objv.offset(3 as ::core::ffi::c_int as isize),
                        &raw mut bHandle,
                    ) != 0
                {
                    return TCL_ERROR;
                }
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewIntObj(sqlite3_vtab_in(pIdxInfo, iCons, bHandle)),
                );
            }
            5 => {
                let mut iCons_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut rc: ::core::ffi::c_int = 0;
                let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                let mut zVal: *const ::core::ffi::c_char = b"\0".as_ptr()
                    as *const ::core::ffi::c_char;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iCons_0,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = sqlite3_vtab_rhs_value(pIdxInfo, iCons_0, &raw mut pVal);
                if rc != SQLITE_OK && rc != SQLITE_NOTFOUND {
                    Tcl_SetResult(
                        interp,
                        sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<Tcl_FreeProc>,
                        >(1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                    return TCL_ERROR;
                }
                if !pVal.is_null() {
                    zVal = sqlite3_value_text(pVal) as *const ::core::ffi::c_char;
                } else if objc == 4 as ::core::ffi::c_int {
                    zVal = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
                }
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(zVal, -1 as ::core::ffi::c_int),
                );
            }
            6 => {
                let mut iCons_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut zColl: *const ::core::ffi::c_char = b"\0".as_ptr()
                    as *const ::core::ffi::c_char;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iCons_1,
                ) != 0
                {
                    return TCL_ERROR;
                }
                zColl = sqlite3_vtab_collation(pIdxInfo, iCons_1);
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(zColl, -1 as ::core::ffi::c_int),
                );
            }
            _ => {}
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn tclBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut tcl_vtab = tab as *mut tcl_vtab;
        let mut interp: *mut Tcl_Interp = (*pTab).interp;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        static mut iNext: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
        let mut zHdl: [::core::ffi::c_char; 24] = [0; 24];
        let mut pScript: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        pScript = Tcl_DuplicateObj((*pTab).pCmd);
        (*pScript).refCount += 1;
        Tcl_ListObjAppendElement(
            interp,
            pScript,
            Tcl_NewStringObj(
                b"xBestIndex\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        let c2rust_fresh9 = iNext;
        iNext = iNext + 1;
        sqlite3_snprintf(
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>() as ::core::ffi::c_int,
            &raw mut zHdl as *mut ::core::ffi::c_char,
            b"bestindex%d\0".as_ptr() as *const ::core::ffi::c_char,
            c2rust_fresh9,
        );
        Tcl_CreateObjCommand(
            interp,
            &raw mut zHdl as *mut ::core::ffi::c_char,
            Some(
                testBestIndexObj
                    as unsafe extern "C" fn(
                        ClientData,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            pIdxInfo as ClientData,
            None,
        );
        Tcl_ListObjAppendElement(
            interp,
            pScript,
            Tcl_NewStringObj(
                &raw mut zHdl as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        rc = Tcl_EvalObjEx(interp, pScript, TCL_EVAL_GLOBAL);
        Tcl_DeleteCommand(interp, &raw mut zHdl as *mut ::core::ffi::c_char);
        let mut _objPtr: *mut Tcl_Obj = pScript;
        let c2rust_fresh10 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh10 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        if rc != TCL_OK {
            let mut zErr: *const ::core::ffi::c_char = Tcl_GetStringResult(interp);
            rc = SQLITE_ERROR;
            (*pTab).base.zErrMsg = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                zErr,
            );
        } else {
            let mut pRes: *mut Tcl_Obj = Tcl_GetObjResult(interp);
            let mut apElem: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<*mut Tcl_Obj>();
            let mut nElem: ::core::ffi::c_int = 0;
            rc = Tcl_ListObjGetElements(interp, pRes, &raw mut nElem, &raw mut apElem);
            if rc != TCL_OK {
                let mut zErr_0: *const ::core::ffi::c_char = Tcl_GetStringResult(interp);
                rc = SQLITE_ERROR;
                (*pTab).base.zErrMsg = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    zErr_0,
                );
            } else {
                let mut ii: ::core::ffi::c_int = 0;
                let mut iArgv: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                ii = 0 as ::core::ffi::c_int;
                while rc == SQLITE_OK && ii < nElem {
                    let mut zCmd: *const ::core::ffi::c_char = Tcl_GetString(
                        *apElem.offset(ii as isize),
                    );
                    let mut p: *mut Tcl_Obj = *apElem
                        .offset((ii + 1 as ::core::ffi::c_int) as isize);
                    if sqlite3_stricmp(
                        b"cost\0".as_ptr() as *const ::core::ffi::c_char,
                        zCmd,
                    ) == 0 as ::core::ffi::c_int
                    {
                        rc = Tcl_GetDoubleFromObj(
                            interp,
                            p,
                            &raw mut (*pIdxInfo).estimatedCost,
                        );
                    } else if sqlite3_stricmp(
                        b"orderby\0".as_ptr() as *const ::core::ffi::c_char,
                        zCmd,
                    ) == 0 as ::core::ffi::c_int
                    {
                        rc = Tcl_GetIntFromObj(
                            interp,
                            p,
                            &raw mut (*pIdxInfo).orderByConsumed,
                        );
                    } else if sqlite3_stricmp(
                        b"idxnum\0".as_ptr() as *const ::core::ffi::c_char,
                        zCmd,
                    ) == 0 as ::core::ffi::c_int
                    {
                        rc = Tcl_GetIntFromObj(interp, p, &raw mut (*pIdxInfo).idxNum);
                    } else if sqlite3_stricmp(
                        b"idxstr\0".as_ptr() as *const ::core::ffi::c_char,
                        zCmd,
                    ) == 0 as ::core::ffi::c_int
                    {
                        sqlite3_free((*pIdxInfo).idxStr as *mut ::core::ffi::c_void);
                        (*pIdxInfo).idxStr = sqlite3_mprintf(
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            Tcl_GetString(p),
                        );
                        (*pIdxInfo).needToFreeIdxStr = 1 as ::core::ffi::c_int;
                    } else if sqlite3_stricmp(
                        b"rows\0".as_ptr() as *const ::core::ffi::c_char,
                        zCmd,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut x: Tcl_WideInt = 0 as Tcl_WideInt;
                        rc = Tcl_GetWideIntFromObj(interp, p, &raw mut x);
                        (*pIdxInfo).estimatedRows = x as tRowcnt as sqlite3_int64;
                    } else if sqlite3_stricmp(
                        b"use\0".as_ptr() as *const ::core::ffi::c_char,
                        zCmd,
                    ) == 0 as ::core::ffi::c_int
                        || sqlite3_stricmp(
                            b"omit\0".as_ptr() as *const ::core::ffi::c_char,
                            zCmd,
                        ) == 0 as ::core::ffi::c_int
                    {
                        let mut iCons: ::core::ffi::c_int = 0;
                        rc = Tcl_GetIntFromObj(interp, p, &raw mut iCons);
                        if rc == SQLITE_OK {
                            if iCons < 0 as ::core::ffi::c_int
                                || iCons >= (*pIdxInfo).nConstraint
                            {
                                rc = SQLITE_ERROR;
                                (*pTab).base.zErrMsg = sqlite3_mprintf(
                                    b"unexpected: %d\0".as_ptr() as *const ::core::ffi::c_char,
                                    iCons,
                                );
                            } else {
                                let mut bOmit: ::core::ffi::c_int = (*zCmd
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int == 'o' as i32
                                    || *zCmd.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int == 'O' as i32) as ::core::ffi::c_int;
                                let c2rust_fresh11 = iArgv;
                                iArgv = iArgv + 1;
                                (*(*pIdxInfo).aConstraintUsage.offset(iCons as isize))
                                    .argvIndex = c2rust_fresh11;
                                (*(*pIdxInfo).aConstraintUsage.offset(iCons as isize))
                                    .omit = bOmit as ::core::ffi::c_uchar;
                            }
                        }
                    } else if sqlite3_stricmp(
                        b"constraint\0".as_ptr() as *const ::core::ffi::c_char,
                        zCmd,
                    ) == 0 as ::core::ffi::c_int
                    {
                        rc = SQLITE_CONSTRAINT;
                        (*pTab).base.zErrMsg = sqlite3_mprintf(
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            Tcl_GetString(p),
                        );
                    } else {
                        rc = SQLITE_ERROR;
                        (*pTab).base.zErrMsg = sqlite3_mprintf(
                            b"unexpected: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            zCmd,
                        );
                    }
                    if rc != SQLITE_OK && (*pTab).base.zErrMsg.is_null() {
                        let mut zErr_1: *const ::core::ffi::c_char = Tcl_GetStringResult(
                            interp,
                        );
                        (*pTab).base.zErrMsg = sqlite3_mprintf(
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            zErr_1,
                        );
                    }
                    ii += 2 as ::core::ffi::c_int;
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn tclFunction(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut TestFindFunction = sqlite3_user_data(pCtx)
            as *mut TestFindFunction;
        let mut interp: *mut Tcl_Interp = (*(*p).pTab).interp;
        let mut pScript: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut ii: ::core::ffi::c_int = 0;
        pScript = Tcl_DuplicateObj((*(*p).pTab).pCmd);
        (*pScript).refCount += 1;
        Tcl_ListObjAppendElement(
            interp,
            pScript,
            Tcl_NewStringObj(
                b"function\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(
            interp,
            pScript,
            Tcl_NewStringObj((*p).zName, -1 as ::core::ffi::c_int),
        );
        ii = 0 as ::core::ffi::c_int;
        while ii < nArg {
            let mut zArg: *const ::core::ffi::c_char = sqlite3_value_text(
                *apArg.offset(ii as isize),
            ) as *const ::core::ffi::c_char;
            Tcl_ListObjAppendElement(
                interp,
                pScript,
                if !zArg.is_null() {
                    Tcl_NewStringObj(zArg, -1 as ::core::ffi::c_int)
                } else {
                    Tcl_NewObj()
                },
            );
            ii += 1;
        }
        Tcl_EvalObjEx(interp, pScript, TCL_EVAL_GLOBAL);
        let mut _objPtr: *mut Tcl_Obj = pScript;
        let c2rust_fresh12 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh12 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        pRet = Tcl_GetObjResult(interp);
        sqlite3_result_text(
            pCtx,
            Tcl_GetString(pRet),
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
unsafe extern "C" fn tclFindFunction(
    mut tab: *mut sqlite3_vtab,
    mut nArg: ::core::ffi::c_int,
    mut zName: *const ::core::ffi::c_char,
    mut pxFunc: *mut Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
    mut ppArg: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iRet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pTab: *mut tcl_vtab = tab as *mut tcl_vtab;
        let mut interp: *mut Tcl_Interp = (*pTab).interp;
        let mut pScript: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        pScript = Tcl_DuplicateObj((*pTab).pCmd);
        (*pScript).refCount += 1;
        Tcl_ListObjAppendElement(
            interp,
            pScript,
            Tcl_NewStringObj(
                b"xFindFunction\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(interp, pScript, Tcl_NewIntObj(nArg));
        Tcl_ListObjAppendElement(
            interp,
            pScript,
            Tcl_NewStringObj(zName, -1 as ::core::ffi::c_int),
        );
        rc = Tcl_EvalObjEx(interp, pScript, TCL_EVAL_GLOBAL);
        let mut _objPtr: *mut Tcl_Obj = pScript;
        let c2rust_fresh13 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh13 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        if rc == SQLITE_OK {
            let mut pObj: *mut Tcl_Obj = Tcl_GetObjResult(interp);
            if Tcl_GetIntFromObj(interp, pObj, &raw mut iRet) != 0 {
                rc = SQLITE_ERROR;
            } else if iRet > 0 as ::core::ffi::c_int {
                let mut nName: sqlite3_int64 = strlen(zName) as sqlite3_int64;
                let mut nByte: sqlite3_int64 = ((nName as ::core::ffi::c_longlong
                    + 1 as ::core::ffi::c_longlong) as ::core::ffi::c_ulonglong)
                    .wrapping_add(
                        ::core::mem::size_of::<TestFindFunction>()
                            as ::core::ffi::c_ulonglong,
                    ) as sqlite3_int64;
                let mut pNew: *mut TestFindFunction = ::core::ptr::null_mut::<
                    TestFindFunction,
                >();
                pNew = sqlite3_malloc64(nByte as sqlite3_uint64)
                    as *mut TestFindFunction;
                if pNew.is_null() {
                    iRet = 0 as ::core::ffi::c_int;
                } else {
                    memset(
                        pNew as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        nByte as size_t,
                    );
                    (*pNew).zName = pNew.offset(1 as ::core::ffi::c_int as isize)
                        as *mut TestFindFunction as *const ::core::ffi::c_char;
                    memcpy(
                        (*pNew).zName as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        zName as *const ::core::ffi::c_void,
                        nName as size_t,
                    );
                    (*pNew).pTab = pTab;
                    (*pNew).pNext = (*pTab).pFindFunctionList;
                    (*pTab).pFindFunctionList = pNew;
                    *ppArg = pNew as *mut ::core::ffi::c_void;
                    *pxFunc = Some(
                        tclFunction
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                        >;
                }
            }
        }
        return iRet;
    }
}
unsafe extern "C" fn tclUpdate(
    mut tab: *mut sqlite3_vtab,
    mut nArg: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
    mut piRowid: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut tcl_vtab = tab as *mut tcl_vtab;
        let mut interp: *mut Tcl_Interp = (*pTab).interp;
        let mut pEval: *mut Tcl_Obj = Tcl_DuplicateObj((*pTab).pCmd);
        let mut pRes: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut rc: ::core::ffi::c_int = TCL_OK;
        (*pEval).refCount += 1;
        Tcl_ListObjAppendElement(
            interp,
            pEval,
            Tcl_NewStringObj(
                b"xUpdate\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        rc = Tcl_EvalObjEx(interp, pEval, TCL_EVAL_GLOBAL);
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh14 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh14 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        if rc == TCL_OK {
            let mut pRes_0: *mut Tcl_Obj = Tcl_GetObjResult(interp);
            let mut v: Tcl_WideInt = 0;
            rc = Tcl_GetWideIntFromObj(interp, pRes_0, &raw mut v);
            *piRowid = v;
        }
        if rc != TCL_OK {
            (*tab).zErrMsg = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetStringResult((*pTab).interp),
            );
            return rc;
        }
        return SQLITE_OK;
    }
}
static mut tclModule: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: Some(
            tclConnect
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
            tclConnect
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
            tclBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            tclDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            tclDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            tclOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            tclClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            tclFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            tclNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            tclEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            tclColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            tclRowid
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
        xFindFunction: Some(
            tclFindFunction
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
        xRename: None,
        xSavepoint: None,
        xRelease: None,
        xRollbackTo: None,
        xShadowName: None,
        xIntegrity: None,
    }
};
static mut tclModuleUpdate: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: Some(
            tclConnect
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
            tclConnect
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
            tclBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            tclDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            tclDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            tclOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            tclClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            tclFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            tclNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            tclEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            tclColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            tclRowid
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: Some(
            tclUpdate
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xBegin: None,
        xSync: None,
        xCommit: None,
        xRollback: None,
        xFindFunction: Some(
            tclFindFunction
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
        xRename: None,
        xSavepoint: None,
        xRelease: None,
        xRollbackTo: None,
        xShadowName: None,
        xIntegrity: None,
    }
};
unsafe extern "C" fn delTestVtabCtx(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        let mut pCtx: *mut TestVtabContext = p as *mut TestVtabContext;
        if !(*pCtx).pDefault.is_null() {
            let mut _objPtr: *mut Tcl_Obj = (*pCtx).pDefault;
            let c2rust_fresh15 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh15 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
        }
        Tcl_Free(pCtx as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn register_tcl_module(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB ?DEFAULT-CMD?\0".as_ptr() as *const ::core::ffi::c_char,
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
        let mut pMod: *mut sqlite3_module = &raw mut tclModule;
        let mut pCtx: *mut TestVtabContext = Tcl_Alloc(
            ::core::mem::size_of::<TestVtabContext>() as ::core::ffi::c_uint,
        ) as *mut ::core::ffi::c_void as *mut TestVtabContext;
        (*pCtx).interp = interp;
        (*pCtx).pDefault = ::core::ptr::null_mut::<Tcl_Obj>();
        if objc == 3 as ::core::ffi::c_int {
            (*pCtx).pDefault = *objv.offset(2 as ::core::ffi::c_int as isize);
            (*(*pCtx).pDefault).refCount += 1;
        }
        if objc == 3 as ::core::ffi::c_int {
            pMod = &raw mut tclModuleUpdate;
        }
        sqlite3_create_module_v2(
            db,
            b"tcl\0".as_ptr() as *const ::core::ffi::c_char,
            pMod,
            pCtx as *mut ::core::ffi::c_void,
            Some(delTestVtabCtx as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetesttcl_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aObjCmd: [C2Rust_Unnamed_25; 1] = unsafe {
            [
                C2Rust_Unnamed_25 {
                    zName: b"register_tcl_module\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        register_tcl_module
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
            ]
        };
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_25; 1]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_25>() as usize)
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
        return TCL_OK;
    }
}
