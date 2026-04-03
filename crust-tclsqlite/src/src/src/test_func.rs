use ::c2rust_bitfields;
use ::libc;
unsafe extern "C" {
    fn getDbPointer(
        interp: *mut Tcl_Interp,
        zA: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
    ) -> ::core::ffi::c_int;
    fn Md5_Register(
        db: *mut sqlite3,
        pzErrMsg: *mut *mut ::core::ffi::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> ::core::ffi::c_int;
}
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
    pub type Tcl_Command_;
    fn sqlite3_initialize() -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_errmsg16(_: *mut sqlite3) -> *const ::core::ffi::c_void;
    fn sqlite3_limit(
        _: *mut sqlite3,
        id: ::core::ffi::c_int,
        newVal: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_value(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *mut sqlite3_value;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_text16(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_bytes16(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_frombind(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_subtype(_: *mut sqlite3_value) -> ::core::ffi::c_uint;
    fn sqlite3_aggregate_context(
        _: *mut sqlite3_context,
        nBytes: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_get_auxdata(
        _: *mut sqlite3_context,
        N: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_set_auxdata(
        _: *mut sqlite3_context,
        N: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
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
    fn sqlite3_result_zeroblob(_: *mut sqlite3_context, n: ::core::ffi::c_int);
    fn sqlite3_result_subtype(_: *mut sqlite3_context, _: ::core::ffi::c_uint);
    fn sqlite3_auto_extension(
        xEntryPoint: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn Tcl_AppendStringsToObj(objPtr: *mut Tcl_Obj, ...);
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewDoubleObj(doubleValue: ::core::ffi::c_double) -> *mut Tcl_Obj;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_NewStringObj(
        bytes: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
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
    fn sqlite3DbFree(_: *mut sqlite3, _: *mut ::core::ffi::c_void);
    fn sqlite3GetVarint(_: *const ::core::ffi::c_uchar, _: *mut u64_0) -> u8_0;
    fn sqlite3VdbeSerialTypeLen(_: u32_0) -> u32_0;
    fn sqlite3VdbeSerialGet(_: *const ::core::ffi::c_uchar, _: u32_0, _: *mut Mem);
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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type size_t = usize;
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
    pub internalRep: C2Rust_Unnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_26 {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2Rust_Unnamed_28,
    pub ptrAndLongRep: C2Rust_Unnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_27 {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_28 {
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
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_29 {
    pub i: sqlite3_uint64,
    pub r: ::core::ffi::c_double,
    pub x: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_30 {
    pub zName: *mut ::core::ffi::c_char,
    pub nArg: ::core::ffi::c_schar,
    pub eTextRep: ::core::ffi::c_uint,
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
pub struct C2Rust_Unnamed_31 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_FUNCTION_ARG: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ANY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_RESULT_SUBTYPE: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn testContextMalloc(
    mut context: *mut sqlite3_context,
    mut nByte: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = sqlite3_malloc(nByte)
            as *mut ::core::ffi::c_char;
        if z.is_null() && nByte > 0 as ::core::ffi::c_int {
            sqlite3_result_error_nomem(context);
        }
        return z as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn randStr(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        static mut zSrc: [::core::ffi::c_uchar; 79] = unsafe {
            ::core::mem::transmute::<
                [u8; 79],
                [::core::ffi::c_uchar; 79],
            >(
                *b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.-!,:*^+=_|?/<> \0",
            )
        };
        let mut iMin: ::core::ffi::c_int = 0;
        let mut iMax: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut r: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_uchar; 1000] = [0; 1000];
        iMin = sqlite3_value_int(*argv.offset(0 as ::core::ffi::c_int as isize));
        if iMin < 0 as ::core::ffi::c_int {
            iMin = 0 as ::core::ffi::c_int;
        }
        if iMin as usize
            >= ::core::mem::size_of::<[::core::ffi::c_uchar; 1000]>() as usize
        {
            iMin = (::core::mem::size_of::<[::core::ffi::c_uchar; 1000]>() as usize)
                .wrapping_sub(1 as usize) as ::core::ffi::c_int;
        }
        iMax = sqlite3_value_int(*argv.offset(1 as ::core::ffi::c_int as isize));
        if iMax < iMin {
            iMax = iMin;
        }
        if iMax as usize
            >= ::core::mem::size_of::<[::core::ffi::c_uchar; 1000]>() as usize
        {
            iMax = (::core::mem::size_of::<[::core::ffi::c_uchar; 1000]>() as usize)
                .wrapping_sub(1 as usize) as ::core::ffi::c_int;
        }
        n = iMin;
        if iMax > iMin {
            sqlite3_randomness(
                ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
                &raw mut r as *mut ::core::ffi::c_void,
            );
            r &= 0x7fffffff as ::core::ffi::c_int;
            n += r % (iMax + 1 as ::core::ffi::c_int - iMin);
        }
        sqlite3_randomness(
            n,
            &raw mut zBuf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        );
        i = 0 as ::core::ffi::c_int;
        while i < n {
            zBuf[i as usize] = zSrc[(zBuf[i as usize] as usize)
                .wrapping_rem(
                    (::core::mem::size_of::<[::core::ffi::c_uchar; 79]>() as usize)
                        .wrapping_sub(1 as usize),
                ) as usize];
            i += 1;
        }
        zBuf[n as usize] = 0 as ::core::ffi::c_uchar;
        sqlite3_result_text(
            context,
            &raw mut zBuf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            n,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
static mut test_destructor_count_var: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn destructor(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        let mut zVal: *mut ::core::ffi::c_char = p as *mut ::core::ffi::c_char;
        zVal = zVal.offset(-1);
        sqlite3_free(zVal as *mut ::core::ffi::c_void);
        test_destructor_count_var -= 1;
    }
}
unsafe extern "C" fn test_destructor(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zVal: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut len: ::core::ffi::c_int = 0;
        test_destructor_count_var += 1;
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_NULL
        {
            return;
        }
        len = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        zVal = testContextMalloc(pCtx, len + 3 as ::core::ffi::c_int)
            as *mut ::core::ffi::c_char;
        if zVal.is_null() {
            return;
        }
        *zVal.offset((len + 1 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_char;
        *zVal.offset((len + 2 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_char;
        zVal = zVal.offset(1);
        memcpy(
            zVal as *mut ::core::ffi::c_void,
            sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_void,
            len as size_t,
        );
        sqlite3_result_text(
            pCtx,
            zVal,
            -1 as ::core::ffi::c_int,
            Some(destructor as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    }
}
unsafe extern "C" fn test_destructor16(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zVal: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut len: ::core::ffi::c_int = 0;
        test_destructor_count_var += 1;
        if sqlite3_value_type(*argv.offset(0 as ::core::ffi::c_int as isize))
            == SQLITE_NULL
        {
            return;
        }
        len = sqlite3_value_bytes16(*argv.offset(0 as ::core::ffi::c_int as isize));
        zVal = testContextMalloc(pCtx, len + 3 as ::core::ffi::c_int)
            as *mut ::core::ffi::c_char;
        if zVal.is_null() {
            return;
        }
        *zVal.offset((len + 1 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_char;
        *zVal.offset((len + 2 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_char;
        zVal = zVal.offset(1);
        memcpy(
            zVal as *mut ::core::ffi::c_void,
            sqlite3_value_text16(*argv.offset(0 as ::core::ffi::c_int as isize)),
            len as size_t,
        );
        sqlite3_result_text16(
            pCtx,
            zVal as *const ::core::ffi::c_void,
            -1 as ::core::ffi::c_int,
            Some(destructor as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    }
}
unsafe extern "C" fn test_destructor_count(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_result_int(pCtx, test_destructor_count_var);
    }
}
unsafe extern "C" fn test_agg_errmsg16_step(
    mut a: *mut sqlite3_context,
    mut b: ::core::ffi::c_int,
    mut c: *mut *mut sqlite3_value,
) {}
unsafe extern "C" fn test_agg_errmsg16_final(mut ctx: *mut sqlite3_context) {
    unsafe {
        let mut z: *const ::core::ffi::c_void = ::core::ptr::null::<
            ::core::ffi::c_void,
        >();
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(ctx);
        sqlite3_aggregate_context(ctx, 2048 as ::core::ffi::c_int);
        z = sqlite3_errmsg16(db);
        sqlite3_result_text16(
            ctx,
            z,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
unsafe extern "C" fn free_test_auxdata(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        sqlite3_free(p);
    }
}
unsafe extern "C" fn test_auxdata(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut zRet: *mut ::core::ffi::c_char = testContextMalloc(
            pCtx,
            nArg * 2 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char;
        if zRet.is_null() {
            return;
        }
        memset(
            zRet as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (nArg * 2 as ::core::ffi::c_int) as size_t,
        );
        i = 0 as ::core::ffi::c_int;
        while i < nArg {
            let mut z: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(i as isize),
            ) as *mut ::core::ffi::c_char;
            if !z.is_null() {
                let mut n: ::core::ffi::c_int = 0;
                let mut zAux: *mut ::core::ffi::c_char = sqlite3_get_auxdata(pCtx, i)
                    as *mut ::core::ffi::c_char;
                if !zAux.is_null() {
                    *zRet.offset((i * 2 as ::core::ffi::c_int) as isize) = '1' as i32
                        as ::core::ffi::c_char;
                } else {
                    *zRet.offset((i * 2 as ::core::ffi::c_int) as isize) = '0' as i32
                        as ::core::ffi::c_char;
                }
                n = strlen(z) as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                zAux = testContextMalloc(pCtx, n) as *mut ::core::ffi::c_char;
                if !zAux.is_null() {
                    memcpy(
                        zAux as *mut ::core::ffi::c_void,
                        z as *const ::core::ffi::c_void,
                        n as size_t,
                    );
                    sqlite3_set_auxdata(
                        pCtx,
                        i,
                        zAux as *mut ::core::ffi::c_void,
                        Some(
                            free_test_auxdata
                                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                        ),
                    );
                }
                *zRet
                    .offset(
                        (i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                    ) = ' ' as i32 as ::core::ffi::c_char;
            }
            i += 1;
        }
        sqlite3_result_text(
            pCtx,
            zRet,
            2 as ::core::ffi::c_int * nArg - 1 as ::core::ffi::c_int,
            Some(
                free_test_auxdata as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
    }
}
unsafe extern "C" fn test_error(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_result_error(
            pCtx,
            sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
        );
        if nArg == 2 as ::core::ffi::c_int {
            sqlite3_result_error_code(
                pCtx,
                sqlite3_value_int(*argv.offset(1 as ::core::ffi::c_int as isize)),
            );
        }
    }
}
unsafe extern "C" fn counterFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pCounter: *mut ::core::ffi::c_int = sqlite3_get_auxdata(
            pCtx,
            0 as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_int;
        if pCounter.is_null() {
            pCounter = sqlite3_malloc(
                ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_int;
            if pCounter.is_null() {
                sqlite3_result_error_nomem(pCtx);
                return;
            }
            *pCounter = sqlite3_value_int(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            sqlite3_set_auxdata(
                pCtx,
                0 as ::core::ffi::c_int,
                pCounter as *mut ::core::ffi::c_void,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        } else {
            *pCounter += 1;
        }
        sqlite3_result_int(pCtx, *pCounter);
    }
}
unsafe extern "C" fn test_isolation(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_value_text16(*argv.offset(0 as ::core::ffi::c_int as isize));
        sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        sqlite3_value_text16(*argv.offset(0 as ::core::ffi::c_int as isize));
        sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
        sqlite3_result_value(pCtx, *argv.offset(1 as ::core::ffi::c_int as isize));
    }
}
unsafe extern "C" fn test_eval(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(pCtx);
        let mut zSql: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        zSql = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *mut ::core::ffi::c_char;
        rc = sqlite3_prepare_v2(
            db,
            zSql,
            -1 as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            rc = sqlite3_step(pStmt);
            if rc == SQLITE_ROW {
                sqlite3_result_value(
                    pCtx,
                    sqlite3_column_value(pStmt, 0 as ::core::ffi::c_int),
                );
            }
            rc = sqlite3_finalize(pStmt);
        }
        if rc != 0 {
            let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            zErr = sqlite3_mprintf(
                b"sqlite3_prepare_v2() error: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            sqlite3_result_text(
                pCtx,
                zErr,
                -1 as ::core::ffi::c_int,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
            sqlite3_result_error_code(pCtx, rc);
        }
    }
}
unsafe extern "C" fn testHexChar(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32
        {
            return c as ::core::ffi::c_int - '0' as i32
        } else if c as ::core::ffi::c_int >= 'a' as i32
            && c as ::core::ffi::c_int <= 'f' as i32
        {
            return c as ::core::ffi::c_int - 'a' as i32 + 10 as ::core::ffi::c_int
        } else if c as ::core::ffi::c_int >= 'A' as i32
            && c as ::core::ffi::c_int <= 'F' as i32
        {
            return c as ::core::ffi::c_int - 'A' as i32 + 10 as ::core::ffi::c_int
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn testHexToBin(
    mut zIn: *const ::core::ffi::c_char,
    mut zOut: *mut ::core::ffi::c_char,
) {
    unsafe {
        while *zIn.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && *zIn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            let c2rust_fresh0 = zOut;
            zOut = zOut.offset(1);
            *c2rust_fresh0 = ((testHexChar(*zIn.offset(0 as ::core::ffi::c_int as isize))
                << 4 as ::core::ffi::c_int)
                + testHexChar(*zIn.offset(1 as ::core::ffi::c_int as isize)))
                as ::core::ffi::c_char;
            zIn = zIn.offset(2 as ::core::ffi::c_int as isize);
        }
    }
}
unsafe extern "C" fn testHexToUtf16be(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut zIn: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        n = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        zIn = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        zOut = sqlite3_malloc(n / 2 as ::core::ffi::c_int) as *mut ::core::ffi::c_char;
        if zOut.is_null() {
            sqlite3_result_error_nomem(pCtx);
        } else {
            testHexToBin(zIn, zOut);
            sqlite3_result_text16be(
                pCtx,
                zOut as *const ::core::ffi::c_void,
                n / 2 as ::core::ffi::c_int,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        };
    }
}
unsafe extern "C" fn testHexToUtf8(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut zIn: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        n = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        zIn = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        zOut = sqlite3_malloc(n / 2 as ::core::ffi::c_int) as *mut ::core::ffi::c_char;
        if zOut.is_null() {
            sqlite3_result_error_nomem(pCtx);
        } else {
            testHexToBin(zIn, zOut);
            sqlite3_result_text(
                pCtx,
                zOut,
                n / 2 as ::core::ffi::c_int,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        };
    }
}
unsafe extern "C" fn testHexToUtf16le(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut zIn: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        n = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        zIn = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        zOut = sqlite3_malloc(n / 2 as ::core::ffi::c_int) as *mut ::core::ffi::c_char;
        if zOut.is_null() {
            sqlite3_result_error_nomem(pCtx);
        } else {
            testHexToBin(zIn, zOut);
            sqlite3_result_text16le(
                pCtx,
                zOut as *const ::core::ffi::c_void,
                n / 2 as ::core::ffi::c_int,
                Some(
                    sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        };
    }
}
unsafe extern "C" fn real2hex(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut v: C2Rust_Unnamed_29 = C2Rust_Unnamed_29 { i: 0 };
        let mut zOut: [::core::ffi::c_char; 20] = [0; 20];
        let mut i: ::core::ffi::c_int = 0;
        let mut bigEndian: ::core::ffi::c_int = 0;
        v.i = 1 as sqlite3_uint64;
        bigEndian = (v.x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        v.r = sqlite3_value_double(*argv.offset(0 as ::core::ffi::c_int as isize));
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            if bigEndian != 0 {
                zOut[(i * 2 as ::core::ffi::c_int) as usize] = ::core::mem::transmute::<
                    [u8; 17],
                    [::core::ffi::c_char; 17],
                >(
                    *b"0123456789abcdef\0",
                )[(v.x[i as usize] as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                    as usize];
                zOut[(i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] = ::core::mem::transmute::<
                    [u8; 17],
                    [::core::ffi::c_char; 17],
                >(
                    *b"0123456789abcdef\0",
                )[(v.x[i as usize] as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                    as usize];
            } else {
                zOut[(14 as ::core::ffi::c_int - i * 2 as ::core::ffi::c_int)
                    as usize] = ::core::mem::transmute::<
                    [u8; 17],
                    [::core::ffi::c_char; 17],
                >(
                    *b"0123456789abcdef\0",
                )[(v.x[i as usize] as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                    as usize];
                zOut[(14 as ::core::ffi::c_int - i * 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize] = ::core::mem::transmute::<
                    [u8; 17],
                    [::core::ffi::c_char; 17],
                >(
                    *b"0123456789abcdef\0",
                )[(v.x[i as usize] as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                    as usize];
            }
            i += 1;
        }
        zOut[16 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        sqlite3_result_text(
            context,
            &raw mut zOut as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
unsafe extern "C" fn test_extract(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut pRec: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pEndHdr: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pHdr: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pBody: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut nHdr: u64_0 = 0;
        let mut iIdx: ::core::ffi::c_int = 0;
        let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pRec = sqlite3_value_blob(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *mut u8_0;
        iIdx = sqlite3_value_int(*argv.offset(1 as ::core::ffi::c_int as isize));
        pHdr = pRec
            .offset(
                sqlite3GetVarint(pRec, &raw mut nHdr) as ::core::ffi::c_int as isize,
            );
        pEndHdr = pRec.offset(nHdr as isize) as *mut u8_0;
        pBody = pEndHdr;
        iCurrent = 0 as ::core::ffi::c_int;
        while pHdr < pEndHdr && iCurrent <= iIdx {
            let mut iSerialType: u64_0 = 0;
            let mut mem: Mem = Mem {
                u: MemValue { r: 0. },
                z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                n: 0,
                flags: 0,
                enc: 0,
                eSubtype: 0,
                db: ::core::ptr::null_mut::<sqlite3>(),
                szMalloc: 0,
                uTemp: 0,
                zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                xDel: None,
            };
            memset(
                &raw mut mem as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Mem>() as size_t,
            );
            mem.db = db;
            mem.enc = (*db).enc;
            pHdr = pHdr
                .offset(
                    sqlite3GetVarint(pHdr, &raw mut iSerialType) as ::core::ffi::c_int
                        as isize,
                );
            sqlite3VdbeSerialGet(pBody, iSerialType as u32_0, &raw mut mem);
            pBody = pBody
                .offset(sqlite3VdbeSerialTypeLen(iSerialType as u32_0) as isize);
            if iCurrent == iIdx {
                sqlite3_result_value(context, &raw mut mem);
            }
            if mem.szMalloc != 0 {
                sqlite3DbFree(db, mem.zMalloc as *mut ::core::ffi::c_void);
            }
            iCurrent += 1;
        }
    }
}
unsafe extern "C" fn test_decode(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
        let mut pRec: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pEndHdr: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pHdr: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut pBody: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut nHdr: u64_0 = 0;
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        pRet = Tcl_NewObj();
        (*pRet).refCount += 1;
        pRec = sqlite3_value_blob(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *mut u8_0;
        pHdr = pRec
            .offset(
                sqlite3GetVarint(pRec, &raw mut nHdr) as ::core::ffi::c_int as isize,
            );
        pEndHdr = pRec.offset(nHdr as isize) as *mut u8_0;
        pBody = pEndHdr;
        while pHdr < pEndHdr {
            let mut pVal: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
            let mut iSerialType: u64_0 = 0;
            let mut mem: Mem = Mem {
                u: MemValue { r: 0. },
                z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                n: 0,
                flags: 0,
                enc: 0,
                eSubtype: 0,
                db: ::core::ptr::null_mut::<sqlite3>(),
                szMalloc: 0,
                uTemp: 0,
                zMalloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                xDel: None,
            };
            memset(
                &raw mut mem as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<Mem>() as size_t,
            );
            mem.db = db;
            mem.enc = (*db).enc;
            pHdr = pHdr
                .offset(
                    sqlite3GetVarint(pHdr, &raw mut iSerialType) as ::core::ffi::c_int
                        as isize,
                );
            sqlite3VdbeSerialGet(pBody, iSerialType as u32_0, &raw mut mem);
            pBody = pBody
                .offset(sqlite3VdbeSerialTypeLen(iSerialType as u32_0) as isize);
            match sqlite3_value_type(&raw mut mem) {
                SQLITE_TEXT => {
                    pVal = Tcl_NewStringObj(
                        sqlite3_value_text(&raw mut mem) as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                }
                SQLITE_BLOB => {
                    let mut hexdigit: [::core::ffi::c_char; 16] = [
                        '0' as i32 as ::core::ffi::c_char,
                        '1' as i32 as ::core::ffi::c_char,
                        '2' as i32 as ::core::ffi::c_char,
                        '3' as i32 as ::core::ffi::c_char,
                        '4' as i32 as ::core::ffi::c_char,
                        '5' as i32 as ::core::ffi::c_char,
                        '6' as i32 as ::core::ffi::c_char,
                        '7' as i32 as ::core::ffi::c_char,
                        '8' as i32 as ::core::ffi::c_char,
                        '9' as i32 as ::core::ffi::c_char,
                        'a' as i32 as ::core::ffi::c_char,
                        'b' as i32 as ::core::ffi::c_char,
                        'c' as i32 as ::core::ffi::c_char,
                        'd' as i32 as ::core::ffi::c_char,
                        'e' as i32 as ::core::ffi::c_char,
                        'f' as i32 as ::core::ffi::c_char,
                    ];
                    let mut n: ::core::ffi::c_int = sqlite3_value_bytes(&raw mut mem);
                    let mut z: *mut u8_0 = sqlite3_value_blob(&raw mut mem) as *mut u8_0;
                    let mut i: ::core::ffi::c_int = 0;
                    pVal = Tcl_NewStringObj(
                        b"x'\0".as_ptr() as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                    i = 0 as ::core::ffi::c_int;
                    while i < n {
                        let mut hex: [::core::ffi::c_char; 3] = [0; 3];
                        hex[0 as ::core::ffi::c_int as usize] = hexdigit[(*z
                            .offset(i as isize) as ::core::ffi::c_int
                            >> 4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                            as usize];
                        hex[1 as ::core::ffi::c_int as usize] = hexdigit[(*z
                            .offset(i as isize) as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int) as usize];
                        hex[2 as ::core::ffi::c_int as usize] = '\0' as i32
                            as ::core::ffi::c_char;
                        Tcl_AppendStringsToObj(
                            pVal,
                            &raw mut hex as *mut ::core::ffi::c_char,
                            0 as ::core::ffi::c_int,
                        );
                        i += 1;
                    }
                    Tcl_AppendStringsToObj(
                        pVal,
                        b"'\0".as_ptr() as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                    );
                }
                SQLITE_FLOAT => {
                    pVal = Tcl_NewDoubleObj(sqlite3_value_double(&raw mut mem));
                }
                SQLITE_INTEGER => {
                    pVal = Tcl_NewWideIntObj(
                        sqlite3_value_int64(&raw mut mem) as Tcl_WideInt,
                    );
                }
                SQLITE_NULL => {
                    pVal = Tcl_NewStringObj(
                        b"NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                }
                _ => {}
            }
            Tcl_ListObjAppendElement(::core::ptr::null_mut::<Tcl_Interp>(), pRet, pVal);
            if mem.szMalloc != 0 {
                sqlite3DbFree(db, mem.zMalloc as *mut ::core::ffi::c_void);
            }
        }
        sqlite3_result_text(
            context,
            Tcl_GetString(pRet),
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        let mut _objPtr: *mut Tcl_Obj = pRet;
        let c2rust_fresh1 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh1 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
    }
}
unsafe extern "C" fn test_zeroblob(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut nZero: ::core::ffi::c_int = sqlite3_value_int(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        sqlite3_result_zeroblob(context, nZero);
    }
}
unsafe extern "C" fn test_getsubtype(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_result_int(
            context,
            sqlite3_value_subtype(*argv.offset(0 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn test_frombind(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut m: sqlite3_uint64 = 0 as sqlite3_uint64;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < argc && i < 63 as ::core::ffi::c_int {
            if sqlite3_value_frombind(*argv.offset(i as isize)) != 0 {
                m
                    |= ((1 as ::core::ffi::c_int as sqlite3_uint64) << i)
                        as ::core::ffi::c_ulonglong;
            }
            i += 1;
        }
        sqlite3_result_int64(context, m as sqlite3_int64);
    }
}
unsafe extern "C" fn test_setsubtype(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        sqlite3_result_value(context, *argv.offset(0 as ::core::ffi::c_int as isize));
        sqlite3_result_subtype(
            context,
            sqlite3_value_int(*argv.offset(1 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_uint,
        );
    }
}
unsafe extern "C" fn registerTestFunctions(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pThunk: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aFuncs: [C2Rust_Unnamed_30; 20] = unsafe {
            [
                C2Rust_Unnamed_30 {
                    zName: b"randstr\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 2 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        randStr
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_destructor\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_destructor
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_destructor16\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_destructor16
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"hex_to_utf16be\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        testHexToUtf16be
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"hex_to_utf16le\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        testHexToUtf16le
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"hex_to_utf8\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        testHexToUtf8
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_destructor_count\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    nArg: 0 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_destructor_count
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_auxdata\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: -1 as ::core::ffi::c_int as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_auxdata
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_error\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_error
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_error\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 2 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_error
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_eval\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_eval
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_isolation\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 2 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_isolation
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_counter\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        counterFunc
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"real2hex\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        real2hex
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_decode\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_decode
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_extract\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 2 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_extract
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_zeroblob\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: (SQLITE_UTF8 | SQLITE_DETERMINISTIC)
                        as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_zeroblob
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_getsubtype\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 1 as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_getsubtype
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_setsubtype\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: 2 as ::core::ffi::c_schar,
                    eTextRep: (SQLITE_UTF8 | SQLITE_RESULT_SUBTYPE)
                        as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_setsubtype
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
                C2Rust_Unnamed_30 {
                    zName: b"test_frombind\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    nArg: -1 as ::core::ffi::c_int as ::core::ffi::c_schar,
                    eTextRep: SQLITE_UTF8 as ::core::ffi::c_uint,
                    xFunc: Some(
                        test_frombind
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                },
            ]
        };
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_30; 20]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_30>() as usize)
        {
            sqlite3_create_function(
                db,
                aFuncs[i as usize].zName,
                aFuncs[i as usize].nArg as ::core::ffi::c_int,
                aFuncs[i as usize].eTextRep as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                aFuncs[i as usize].xFunc,
                None,
                None,
            );
            i += 1;
        }
        sqlite3_create_function(
            db,
            b"test_agg_errmsg16\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            SQLITE_ANY,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
            Some(
                test_agg_errmsg16_step
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            Some(
                test_agg_errmsg16_final
                    as unsafe extern "C" fn(*mut sqlite3_context) -> (),
            ),
        );
        return SQLITE_OK;
    }
}
unsafe extern "C" fn autoinstall_test_funcs(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            #[link_name = "Md5_Register"]
            fn Md5_Register_0(
                _: *mut sqlite3,
                _: *mut *mut ::core::ffi::c_char,
                _: *const sqlite3_api_routines,
            ) -> ::core::ffi::c_int;
        }
        let mut rc: ::core::ffi::c_int = sqlite3_auto_extension(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut *mut ::core::ffi::c_char,
                        *const sqlite3_api_routines,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    registerTestFunctions
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
        if rc == SQLITE_OK {
            rc = sqlite3_auto_extension(
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                    >,
                    Option<unsafe extern "C" fn() -> ()>,
                >(
                    Some(
                        Md5_Register_0
                            as unsafe extern "C" fn(
                                *mut sqlite3,
                                *mut *mut ::core::ffi::c_char,
                                *const sqlite3_api_routines,
                            ) -> ::core::ffi::c_int,
                    ),
                ),
            );
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(rc));
        return TCL_OK;
    }
}
unsafe extern "C" fn tStep(
    mut a: *mut sqlite3_context,
    mut b: ::core::ffi::c_int,
    mut c: *mut *mut sqlite3_value,
) {}
unsafe extern "C" fn tFinal(mut a: *mut sqlite3_context) {}
unsafe extern "C" fn abuse_create_function(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            #[link_name = "getDbPointer"]
            fn getDbPointer_0(
                _: *mut Tcl_Interp,
                _: *const ::core::ffi::c_char,
                _: *mut *mut sqlite3,
            ) -> ::core::ffi::c_int;
        }
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut mxArg: ::core::ffi::c_int = 0;
        if getDbPointer_0(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3_create_function(
            db,
            b"tx\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                tStep
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            Some(
                tStep
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            Some(tFinal as unsafe extern "C" fn(*mut sqlite3_context) -> ()),
        );
        if !(rc != SQLITE_MISUSE) {
            rc = sqlite3_create_function(
                db,
                b"tx\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    tStep
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                Some(
                    tStep
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                None,
            );
            if !(rc != SQLITE_MISUSE) {
                rc = sqlite3_create_function(
                    db,
                    b"tx\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    SQLITE_UTF8,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    Some(
                        tStep
                            as unsafe extern "C" fn(
                                *mut sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut sqlite3_value,
                            ) -> (),
                    ),
                    None,
                    Some(tFinal as unsafe extern "C" fn(*mut sqlite3_context) -> ()),
                );
                if !(rc != SQLITE_MISUSE) {
                    rc = sqlite3_create_function(
                        db,
                        b"tx\0".as_ptr() as *const ::core::ffi::c_char,
                        1 as ::core::ffi::c_int,
                        SQLITE_UTF8,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        None,
                        None,
                        Some(tFinal as unsafe extern "C" fn(*mut sqlite3_context) -> ()),
                    );
                    if !(rc != SQLITE_MISUSE) {
                        rc = sqlite3_create_function(
                            db,
                            b"tx\0".as_ptr() as *const ::core::ffi::c_char,
                            1 as ::core::ffi::c_int,
                            SQLITE_UTF8,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            None,
                            Some(
                                tStep
                                    as unsafe extern "C" fn(
                                        *mut sqlite3_context,
                                        ::core::ffi::c_int,
                                        *mut *mut sqlite3_value,
                                    ) -> (),
                            ),
                            None,
                        );
                        if !(rc != SQLITE_MISUSE) {
                            rc = sqlite3_create_function(
                                db,
                                b"tx\0".as_ptr() as *const ::core::ffi::c_char,
                                -2 as ::core::ffi::c_int,
                                SQLITE_UTF8,
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                Some(
                                    tStep
                                        as unsafe extern "C" fn(
                                            *mut sqlite3_context,
                                            ::core::ffi::c_int,
                                            *mut *mut sqlite3_value,
                                        ) -> (),
                                ),
                                None,
                                None,
                            );
                            if !(rc != SQLITE_MISUSE) {
                                rc = sqlite3_create_function(
                                    db,
                                    b"tx\0".as_ptr() as *const ::core::ffi::c_char,
                                    32768 as ::core::ffi::c_int,
                                    SQLITE_UTF8,
                                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                    Some(
                                        tStep
                                            as unsafe extern "C" fn(
                                                *mut sqlite3_context,
                                                ::core::ffi::c_int,
                                                *mut *mut sqlite3_value,
                                            ) -> (),
                                    ),
                                    None,
                                    None,
                                );
                                if !(rc != SQLITE_MISUSE) {
                                    rc = sqlite3_create_function(
                                        db,
                                        b"funcxx_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        1 as ::core::ffi::c_int,
                                        SQLITE_UTF8,
                                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                        Some(
                                            tStep
                                                as unsafe extern "C" fn(
                                                    *mut sqlite3_context,
                                                    ::core::ffi::c_int,
                                                    *mut *mut sqlite3_value,
                                                ) -> (),
                                        ),
                                        None,
                                        None,
                                    );
                                    if !(rc != SQLITE_MISUSE) {
                                        sqlite3_limit(
                                            db,
                                            SQLITE_LIMIT_FUNCTION_ARG,
                                            1000000 as ::core::ffi::c_int,
                                        );
                                        mxArg = sqlite3_limit(
                                            db,
                                            SQLITE_LIMIT_FUNCTION_ARG,
                                            -1 as ::core::ffi::c_int,
                                        );
                                        rc = sqlite3_create_function(
                                            db,
                                            b"nullx_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            mxArg,
                                            SQLITE_UTF8,
                                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                            Some(
                                                tStep
                                                    as unsafe extern "C" fn(
                                                        *mut sqlite3_context,
                                                        ::core::ffi::c_int,
                                                        *mut *mut sqlite3_value,
                                                    ) -> (),
                                            ),
                                            None,
                                            None,
                                        );
                                        if !(rc != SQLITE_OK) {
                                            return TCL_OK;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Tcl_AppendResult(
            interp,
            b"sqlite3_create_function abused test failed\0".as_ptr()
                as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        return TCL_ERROR;
    }
}
unsafe extern "C" fn rankfunc(
    mut pCtx: *mut sqlite3_context,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut aMatchinfo: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut nMatchinfo: ::core::ffi::c_int = 0;
        let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nPhrase: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iPhrase: ::core::ffi::c_int = 0;
        let mut score: ::core::ffi::c_double = 0.0f64;
        if !(nVal < 1 as ::core::ffi::c_int) {
            aMatchinfo = sqlite3_value_blob(
                *apVal.offset(0 as ::core::ffi::c_int as isize),
            ) as *mut ::core::ffi::c_int;
            nMatchinfo = (sqlite3_value_bytes(
                *apVal.offset(0 as ::core::ffi::c_int as isize),
            ) as usize)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as ::core::ffi::c_int;
            if nMatchinfo >= 2 as ::core::ffi::c_int {
                nPhrase = *aMatchinfo.offset(0 as ::core::ffi::c_int as isize);
                nCol = *aMatchinfo.offset(1 as ::core::ffi::c_int as isize);
            }
            if nMatchinfo
                != 2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * nCol * nPhrase
            {
                sqlite3_result_error(
                    pCtx,
                    b"invalid matchinfo blob passed to function rank()\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                );
                return;
            }
            if !(nVal != 1 as ::core::ffi::c_int + nCol) {
                iPhrase = 0 as ::core::ffi::c_int;
                while iPhrase < nPhrase {
                    let mut iCol: ::core::ffi::c_int = 0;
                    let mut aPhraseinfo: *mut ::core::ffi::c_int = aMatchinfo
                        .offset(
                            (2 as ::core::ffi::c_int
                                + iPhrase * nCol * 3 as ::core::ffi::c_int) as isize,
                        ) as *mut ::core::ffi::c_int;
                    iCol = 0 as ::core::ffi::c_int;
                    while iCol < nCol {
                        let mut nHitCount: ::core::ffi::c_int = *aPhraseinfo
                            .offset((3 as ::core::ffi::c_int * iCol) as isize);
                        let mut nGlobalHitCount: ::core::ffi::c_int = *aPhraseinfo
                            .offset(
                                (3 as ::core::ffi::c_int * iCol + 1 as ::core::ffi::c_int)
                                    as isize,
                            );
                        let mut weight: ::core::ffi::c_double = sqlite3_value_double(
                            *apVal.offset((iCol + 1 as ::core::ffi::c_int) as isize),
                        );
                        if nHitCount > 0 as ::core::ffi::c_int {
                            score
                                += nHitCount as ::core::ffi::c_double
                                    / nGlobalHitCount as ::core::ffi::c_double * weight;
                        }
                        iCol += 1;
                    }
                    iPhrase += 1;
                }
                sqlite3_result_double(pCtx, score);
                return;
            }
        }
        sqlite3_result_error(
            pCtx,
            b"wrong number of arguments to function rank()\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn install_fts3_rank_function(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
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
        sqlite3_create_function(
            db,
            b"rank\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                rankfunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest_func_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aObjCmd: [C2Rust_Unnamed_31; 3] = unsafe {
            [
                C2Rust_Unnamed_31 {
                    zName: b"autoinstall_test_functions\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        autoinstall_test_funcs
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_31 {
                    zName: b"abuse_create_function\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        abuse_create_function
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_31 {
                    zName: b"install_fts3_rank_function\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        install_fts3_rank_function
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
            ]
        };
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_31; 3]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_31>() as usize)
        {
            Tcl_CreateObjCommand(
                interp,
                aObjCmd[i as usize].zName,
                aObjCmd[i as usize].xProc,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            i += 1;
        }
        sqlite3_initialize();
        sqlite3_auto_extension(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut *mut ::core::ffi::c_char,
                        *const sqlite3_api_routines,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    registerTestFunctions
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
        sqlite3_auto_extension(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut *mut ::core::ffi::c_char,
                        *const sqlite3_api_routines,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    Md5_Register
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut *mut ::core::ffi::c_char,
                            *const sqlite3_api_routines,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
        return TCL_OK;
    }
}
