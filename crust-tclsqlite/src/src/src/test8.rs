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
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
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
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_null(_: *mut sqlite3_context);
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
    fn atof(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_DStringAppendElement(
        dsPtr: *mut Tcl_DString,
        element: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_DStringFree(dsPtr: *mut Tcl_DString);
    fn Tcl_DStringInit(dsPtr: *mut Tcl_DString);
    fn Tcl_Eval(
        interp: *mut Tcl_Interp,
        script: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        infoPtr: *mut Tcl_CmdInfo,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetStringResult(interp: *mut Tcl_Interp) -> *const ::core::ffi::c_char;
    fn Tcl_GetVar2(
        interp: *mut Tcl_Interp,
        part1: *const ::core::ffi::c_char,
        part2: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn Tcl_ResetResult(interp: *mut Tcl_Interp);
    fn Tcl_SetResult(
        interp: *mut Tcl_Interp,
        result: *mut ::core::ffi::c_char,
        freeProc: Option<Tcl_FreeProc>,
    );
    fn Tcl_SetVar2(
        interp: *mut Tcl_Interp,
        part1: *const ::core::ffi::c_char,
        part2: *const ::core::ffi::c_char,
        newValue: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn sqlite3MallocZero(_: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
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
pub struct Tcl_DString {
    pub string: *mut ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub spaceAvl: ::core::ffi::c_int,
    pub staticSpace: [::core::ffi::c_char; 200],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct echo_vtab {
    pub base: sqlite3_vtab,
    pub interp: *mut Tcl_Interp,
    pub db: *mut sqlite3,
    pub isPattern: ::core::ffi::c_int,
    pub inTransaction: ::core::ffi::c_int,
    pub zThis: *mut ::core::ffi::c_char,
    pub zTableName: *mut ::core::ffi::c_char,
    pub zLogName: *mut ::core::ffi::c_char,
    pub nCol: ::core::ffi::c_int,
    pub aIndex: *mut ::core::ffi::c_int,
    pub aCol: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct echo_cursor {
    pub base: sqlite3_vtab_cursor,
    pub pStmt: *mut sqlite3_stmt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EchoModule {
    pub interp: *mut Tcl_Interp,
    pub db: *mut sqlite3,
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
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_ANY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2;
pub const SQLITE_INDEX_CONSTRAINT_GT: ::core::ffi::c_int = 4;
pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8;
pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16;
pub const SQLITE_INDEX_CONSTRAINT_GE: ::core::ffi::c_int = 32;
pub const SQLITE_INDEX_CONSTRAINT_MATCH: ::core::ffi::c_int = 64;
pub const SQLITE_INDEX_CONSTRAINT_LIKE: ::core::ffi::c_int = 65;
pub const SQLITE_INDEX_CONSTRAINT_GLOB: ::core::ffi::c_int = 66;
pub const SQLITE_INDEX_CONSTRAINT_REGEXP: ::core::ffi::c_int = 67;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_STATIC: Option<Tcl_FreeProc> = None;
pub const TCL_GLOBAL_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_APPEND_VALUE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TCL_LIST_ELEMENT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
unsafe extern "C" fn simulateVtabError(
    mut p: *mut echo_vtab,
    mut zMethod: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zVarname: [::core::ffi::c_char; 128] = [0; 128];
        zVarname[127 as ::core::ffi::c_int as usize] = '\0' as i32
            as ::core::ffi::c_char;
        sqlite3_snprintf(
            127 as ::core::ffi::c_int,
            &raw mut zVarname as *mut ::core::ffi::c_char,
            b"echo_module_fail(%s,%s)\0".as_ptr() as *const ::core::ffi::c_char,
            zMethod,
            (*p).zTableName,
        );
        zErr = Tcl_GetVar2(
            (*p).interp,
            &raw mut zVarname as *mut ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            1 as ::core::ffi::c_int,
        );
        if !zErr.is_null() {
            (*p).base.zErrMsg = sqlite3_mprintf(
                b"echo-vtab-error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                zErr,
            );
        }
        return !zErr.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn dequoteString(mut z: *mut ::core::ffi::c_char) {
    unsafe {
        let mut quote: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        if z.is_null() {
            return;
        }
        quote = *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        match quote {
            39 | 34 | 96 => {}
            91 => {
                quote = ']' as i32;
            }
            _ => return,
        }
        i = 1 as ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        while *z.offset(i as isize) != 0 {
            if *z.offset(i as isize) as ::core::ffi::c_int == quote {
                if *z.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == quote
                {
                    let c2rust_fresh0 = j;
                    j = j + 1;
                    *z.offset(c2rust_fresh0 as isize) = quote as ::core::ffi::c_char;
                    i += 1;
                } else {
                    let c2rust_fresh1 = j;
                    j = j + 1;
                    *z.offset(c2rust_fresh1 as isize) = 0 as ::core::ffi::c_char;
                    break;
                }
            } else {
                let c2rust_fresh2 = j;
                j = j + 1;
                *z.offset(c2rust_fresh2 as isize) = *z.offset(i as isize);
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn getColumnNames(
    mut db: *mut sqlite3,
    mut zTab: *const ::core::ffi::c_char,
    mut paCol: *mut *mut *mut ::core::ffi::c_char,
    mut pnCol: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut aCol: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            *mut ::core::ffi::c_char,
        >();
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        zSql = sqlite3_mprintf(
            b"SELECT * FROM %Q\0".as_ptr() as *const ::core::ffi::c_char,
            zTab,
        );
        if zSql.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            rc = sqlite3_prepare(
                db,
                zSql,
                -1 as ::core::ffi::c_int,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
            if rc == SQLITE_OK {
                let mut ii: ::core::ffi::c_int = 0;
                let mut nBytes: ::core::ffi::c_int = 0;
                let mut zSpace: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                nCol = sqlite3_column_count(pStmt);
                nBytes = (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                    .wrapping_mul(nCol as usize) as ::core::ffi::c_int;
                ii = 0 as ::core::ffi::c_int;
                loop {
                    if !(ii < nCol) {
                        c2rust_current_block = 12599329904712511516;
                        break;
                    }
                    let mut zName: *const ::core::ffi::c_char = sqlite3_column_name(
                        pStmt,
                        ii,
                    );
                    if zName.is_null() {
                        rc = SQLITE_NOMEM;
                        c2rust_current_block = 10923223827071745718;
                        break;
                    } else {
                        nBytes
                            += strlen(zName) as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int;
                        ii += 1;
                    }
                }
                match c2rust_current_block {
                    10923223827071745718 => {}
                    _ => {
                        aCol = sqlite3MallocZero(nBytes as u64_0)
                            as *mut *mut ::core::ffi::c_char;
                        if aCol.is_null() {
                            rc = SQLITE_NOMEM;
                            c2rust_current_block = 10923223827071745718;
                        } else {
                            zSpace = aCol.offset(nCol as isize)
                                as *mut *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            ii = 0 as ::core::ffi::c_int;
                            while ii < nCol {
                                let ref mut c2rust_fresh3 = *aCol.offset(ii as isize);
                                *c2rust_fresh3 = zSpace;
                                sqlite3_snprintf(
                                    nBytes,
                                    zSpace,
                                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                    sqlite3_column_name(pStmt, ii),
                                );
                                zSpace = zSpace
                                    .offset(
                                        (strlen(zSpace) as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int) as isize,
                                    );
                                ii += 1;
                            }
                            c2rust_current_block = 11298138898191919651;
                        }
                    }
                }
            } else {
                c2rust_current_block = 11298138898191919651;
            }
            match c2rust_current_block {
                10923223827071745718 => {}
                _ => {
                    *paCol = aCol;
                    *pnCol = nCol;
                }
            }
        }
        sqlite3_finalize(pStmt);
        return rc;
    }
}
unsafe extern "C" fn getIndexArray(
    mut db: *mut sqlite3,
    mut zTab: *const ::core::ffi::c_char,
    mut nCol: ::core::ffi::c_int,
    mut paIndex: *mut *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut aIndex: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
            ::core::ffi::c_int,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        aIndex = sqlite3MallocZero(
            (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                .wrapping_mul(nCol as usize) as u64_0,
        ) as *mut ::core::ffi::c_int;
        if aIndex.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            zSql = sqlite3_mprintf(
                b"PRAGMA index_list(%s)\0".as_ptr() as *const ::core::ffi::c_char,
                zTab,
            );
            if zSql.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                rc = sqlite3_prepare(
                    db,
                    zSql,
                    -1 as ::core::ffi::c_int,
                    &raw mut pStmt,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                sqlite3_free(zSql as *mut ::core::ffi::c_void);
                while !pStmt.is_null() && sqlite3_step(pStmt) == SQLITE_ROW {
                    let mut zIdx: *const ::core::ffi::c_char = sqlite3_column_text(
                        pStmt,
                        1 as ::core::ffi::c_int,
                    ) as *const ::core::ffi::c_char;
                    let mut pStmt2: *mut sqlite3_stmt = ::core::ptr::null_mut::<
                        sqlite3_stmt,
                    >();
                    if zIdx.is_null() {
                        continue;
                    }
                    zSql = sqlite3_mprintf(
                        b"PRAGMA index_info(%s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zIdx,
                    );
                    if zSql.is_null() {
                        rc = SQLITE_NOMEM;
                        break;
                    } else {
                        rc = sqlite3_prepare(
                            db,
                            zSql,
                            -1 as ::core::ffi::c_int,
                            &raw mut pStmt2,
                            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                        );
                        sqlite3_free(zSql as *mut ::core::ffi::c_void);
                        if !pStmt2.is_null() && sqlite3_step(pStmt2) == SQLITE_ROW {
                            let mut cid: ::core::ffi::c_int = sqlite3_column_int(
                                pStmt2,
                                1 as ::core::ffi::c_int,
                            );
                            *aIndex.offset(cid as isize) = 1 as ::core::ffi::c_int;
                        }
                        if !pStmt2.is_null() {
                            rc = sqlite3_finalize(pStmt2);
                        }
                        if rc != SQLITE_OK {
                            break;
                        }
                    }
                }
            }
        }
        if !pStmt.is_null() {
            let mut rc2: ::core::ffi::c_int = sqlite3_finalize(pStmt);
            if rc == SQLITE_OK {
                rc = rc2;
            }
        }
        if rc != SQLITE_OK {
            sqlite3_free(aIndex as *mut ::core::ffi::c_void);
            aIndex = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
        *paIndex = aIndex;
        return rc;
    }
}
unsafe extern "C" fn appendToEchoModule(
    mut interp: *mut Tcl_Interp,
    mut zArg: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut flags: ::core::ffi::c_int = TCL_APPEND_VALUE | TCL_LIST_ELEMENT
            | TCL_GLOBAL_ONLY;
        Tcl_SetVar2(
            interp,
            b"echo_module\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            if !zArg.is_null() {
                zArg
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            flags,
        );
    }
}
unsafe extern "C" fn echoDeclareVtab(
    mut pVtab: *mut echo_vtab,
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if !(*pVtab).zTableName.is_null() {
            let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            rc = sqlite3_prepare(
                db,
                b"SELECT sql FROM sqlite_schema WHERE type = 'table' AND name = ?\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc == SQLITE_OK {
                sqlite3_bind_text(
                    pStmt,
                    1 as ::core::ffi::c_int,
                    (*pVtab).zTableName,
                    -1 as ::core::ffi::c_int,
                    None,
                );
                if sqlite3_step(pStmt) == SQLITE_ROW {
                    let mut rc2: ::core::ffi::c_int = 0;
                    let mut zCreateTable: *const ::core::ffi::c_char = sqlite3_column_text(
                        pStmt,
                        0 as ::core::ffi::c_int,
                    ) as *const ::core::ffi::c_char;
                    rc = sqlite3_declare_vtab(db, zCreateTable);
                    rc2 = sqlite3_finalize(pStmt);
                    if rc == SQLITE_OK {
                        rc = rc2;
                    }
                } else {
                    rc = sqlite3_finalize(pStmt);
                    if rc == SQLITE_OK {
                        rc = SQLITE_ERROR;
                    }
                }
                if rc == SQLITE_OK {
                    rc = getColumnNames(
                        db,
                        (*pVtab).zTableName,
                        &raw mut (*pVtab).aCol,
                        &raw mut (*pVtab).nCol,
                    );
                }
                if rc == SQLITE_OK {
                    rc = getIndexArray(
                        db,
                        (*pVtab).zTableName,
                        (*pVtab).nCol,
                        &raw mut (*pVtab).aIndex,
                    );
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn echoDestructor(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut echo_vtab = pVtab as *mut echo_vtab;
        sqlite3_free((*p).aIndex as *mut ::core::ffi::c_void);
        sqlite3_free((*p).aCol as *mut ::core::ffi::c_void);
        sqlite3_free((*p).zThis as *mut ::core::ffi::c_void);
        sqlite3_free((*p).zTableName as *mut ::core::ffi::c_void);
        sqlite3_free((*p).zLogName as *mut ::core::ffi::c_void);
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn echoConstructor(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut pVtab: *mut echo_vtab = ::core::ptr::null_mut::<echo_vtab>();
        pVtab = sqlite3MallocZero(::core::mem::size_of::<echo_vtab>() as u64_0)
            as *mut echo_vtab;
        if pVtab.is_null() {
            return SQLITE_NOMEM;
        }
        (*pVtab).interp = (*(pAux as *mut EchoModule)).interp;
        (*pVtab).db = db;
        (*pVtab).zThis = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(2 as ::core::ffi::c_int as isize),
        );
        if (*pVtab).zThis.is_null() {
            echoDestructor(pVtab as *mut sqlite3_vtab);
            return SQLITE_NOMEM;
        }
        if argc > 3 as ::core::ffi::c_int {
            (*pVtab).zTableName = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(3 as ::core::ffi::c_int as isize),
            );
            dequoteString((*pVtab).zTableName);
            if !(*pVtab).zTableName.is_null()
                && *(*pVtab).zTableName.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == '*' as i32
            {
                let mut z: *mut ::core::ffi::c_char = sqlite3_mprintf(
                    b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                    *argv.offset(2 as ::core::ffi::c_int as isize),
                    (*pVtab).zTableName.offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                );
                sqlite3_free((*pVtab).zTableName as *mut ::core::ffi::c_void);
                (*pVtab).zTableName = z;
                (*pVtab).isPattern = 1 as ::core::ffi::c_int;
            }
            if (*pVtab).zTableName.is_null() {
                echoDestructor(pVtab as *mut sqlite3_vtab);
                return SQLITE_NOMEM;
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            appendToEchoModule((*pVtab).interp, *argv.offset(i as isize));
            i += 1;
        }
        rc = echoDeclareVtab(pVtab, db);
        if rc != SQLITE_OK {
            echoDestructor(pVtab as *mut sqlite3_vtab);
            return rc;
        }
        *ppVtab = &raw mut (*pVtab).base;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn echoCreate(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        appendToEchoModule(
            (*(pAux as *mut EchoModule)).interp,
            b"xCreate\0".as_ptr() as *const ::core::ffi::c_char,
        );
        rc = echoConstructor(db, pAux, argc, argv, ppVtab, pzErr);
        if rc == SQLITE_OK && argc == 5 as ::core::ffi::c_int {
            let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut pVtab: *mut echo_vtab = *(ppVtab as *mut *mut echo_vtab);
            (*pVtab).zLogName = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(4 as ::core::ffi::c_int as isize),
            );
            zSql = sqlite3_mprintf(
                b"CREATE TABLE %Q(logmsg)\0".as_ptr() as *const ::core::ffi::c_char,
                (*pVtab).zLogName,
            );
            rc = sqlite3_exec(
                db,
                zSql,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
            if rc != SQLITE_OK {
                *pzErr = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg(db),
                );
            }
        }
        if !(*ppVtab).is_null() && rc != SQLITE_OK {
            echoDestructor(*ppVtab);
            *ppVtab = ::core::ptr::null_mut::<sqlite3_vtab>();
        }
        if rc == SQLITE_OK {
            (**(ppVtab as *mut *mut echo_vtab)).inTransaction = 1 as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn echoConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        appendToEchoModule(
            (*(pAux as *mut EchoModule)).interp,
            b"xConnect\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return echoConstructor(db, pAux, argc, argv, ppVtab, pzErr);
    }
}
unsafe extern "C" fn echoDisconnect(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        appendToEchoModule(
            (*(pVtab as *mut echo_vtab)).interp,
            b"xDisconnect\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return echoDestructor(pVtab);
    }
}
unsafe extern "C" fn echoDestroy(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *mut echo_vtab = pVtab as *mut echo_vtab;
        appendToEchoModule(
            (*(pVtab as *mut echo_vtab)).interp,
            b"xDestroy\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !p.is_null() && !(*p).zLogName.is_null() {
            let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            zSql = sqlite3_mprintf(
                b"DROP TABLE %Q\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).zLogName,
            );
            rc = sqlite3_exec(
                (*p).db,
                zSql,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
        }
        if rc == SQLITE_OK {
            rc = echoDestructor(pVtab);
        }
        return rc;
    }
}
unsafe extern "C" fn echoOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCur: *mut echo_cursor = ::core::ptr::null_mut::<echo_cursor>();
        if simulateVtabError(
            pVTab as *mut echo_vtab,
            b"xOpen\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
            return SQLITE_ERROR;
        }
        pCur = sqlite3MallocZero(::core::mem::size_of::<echo_cursor>() as u64_0)
            as *mut echo_cursor;
        *ppCursor = pCur as *mut sqlite3_vtab_cursor;
        return if !pCur.is_null() { SQLITE_OK } else { SQLITE_NOMEM };
    }
}
unsafe extern "C" fn echoClose(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pCur: *mut echo_cursor = cur as *mut echo_cursor;
        let mut pStmt: *mut sqlite3_stmt = (*pCur).pStmt;
        (*pCur).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        sqlite3_free(pCur as *mut ::core::ffi::c_void);
        rc = sqlite3_finalize(pStmt);
        return rc;
    }
}
unsafe extern "C" fn echoEof(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    unsafe {
        return if !(*(cur as *mut echo_cursor)).pStmt.is_null() {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn echoNext(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pCur: *mut echo_cursor = cur as *mut echo_cursor;
        if simulateVtabError(
            (*cur).pVtab as *mut echo_vtab,
            b"xNext\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
            return SQLITE_ERROR;
        }
        if !(*pCur).pStmt.is_null() {
            rc = sqlite3_step((*pCur).pStmt);
            if rc == SQLITE_ROW {
                rc = SQLITE_OK;
            } else {
                rc = sqlite3_finalize((*pCur).pStmt);
                (*pCur).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            }
        }
        return rc;
    }
}
unsafe extern "C" fn echoColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iCol: ::core::ffi::c_int = i + 1 as ::core::ffi::c_int;
        let mut pStmt: *mut sqlite3_stmt = (*(cur as *mut echo_cursor)).pStmt;
        if simulateVtabError(
            (*cur).pVtab as *mut echo_vtab,
            b"xColumn\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
            return SQLITE_ERROR;
        }
        if pStmt.is_null() {
            sqlite3_result_null(ctx);
        } else {
            sqlite3_result_value(ctx, sqlite3_column_value(pStmt, iCol));
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn echoRowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = (*(cur as *mut echo_cursor)).pStmt;
        if simulateVtabError(
            (*cur).pVtab as *mut echo_vtab,
            b"xRowid\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
            return SQLITE_ERROR;
        }
        *pRowid = sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int) as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn hashString(
    mut zString: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut val: u32_0 = 0 as u32_0;
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        while *zString.offset(ii as isize) != 0 {
            val = (val << 3 as ::core::ffi::c_int)
                .wrapping_add(
                    *zString.offset(ii as isize) as ::core::ffi::c_int as u32_0,
                );
            ii += 1;
        }
        return (val & 0x7fffffff as u32_0) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn echoFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut pCur: *mut echo_cursor = pVtabCursor as *mut echo_cursor;
        let mut pVtab: *mut echo_vtab = (*pVtabCursor).pVtab as *mut echo_vtab;
        let mut db: *mut sqlite3 = (*pVtab).db;
        if simulateVtabError(pVtab, b"xFilter\0".as_ptr() as *const ::core::ffi::c_char)
            != 0
        {
            return SQLITE_ERROR;
        }
        appendToEchoModule(
            (*pVtab).interp,
            b"xFilter\0".as_ptr() as *const ::core::ffi::c_char,
        );
        appendToEchoModule((*pVtab).interp, idxStr);
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            appendToEchoModule(
                (*pVtab).interp,
                sqlite3_value_text(*argv.offset(i as isize))
                    as *const ::core::ffi::c_char,
            );
            i += 1;
        }
        sqlite3_finalize((*pCur).pStmt);
        (*pCur).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare(
            db,
            idxStr,
            -1 as ::core::ffi::c_int,
            &raw mut (*pCur).pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        i = 0 as ::core::ffi::c_int;
        while rc == SQLITE_OK && i < argc {
            rc = sqlite3_bind_value(
                (*pCur).pStmt,
                i + 1 as ::core::ffi::c_int,
                *argv.offset(i as isize),
            );
            i += 1;
        }
        if rc == SQLITE_OK {
            rc = echoNext(pVtabCursor);
        }
        return rc;
    }
}
unsafe extern "C" fn string_concat(
    mut pzStr: *mut *mut ::core::ffi::c_char,
    mut zAppend: *mut ::core::ffi::c_char,
    mut doFree: ::core::ffi::c_int,
    mut pRc: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut zIn: *mut ::core::ffi::c_char = *pzStr;
        if zAppend.is_null() && doFree != 0 && *pRc == SQLITE_OK {
            *pRc = SQLITE_NOMEM;
        }
        if *pRc != SQLITE_OK {
            sqlite3_free(zIn as *mut ::core::ffi::c_void);
            zIn = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            if !zIn.is_null() {
                let mut zTemp: *mut ::core::ffi::c_char = zIn;
                zIn = sqlite3_mprintf(
                    b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                    zIn,
                    zAppend,
                );
                sqlite3_free(zTemp as *mut ::core::ffi::c_void);
            } else {
                zIn = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    zAppend,
                );
            }
            if zIn.is_null() {
                *pRc = SQLITE_NOMEM;
            }
        }
        *pzStr = zIn;
        if doFree != 0 {
            sqlite3_free(zAppend as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn echoSelectList(
    mut pTab: *mut echo_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if sqlite3_libversion_number() < 3010000 as ::core::ffi::c_int {
            zRet = sqlite3_mprintf(b", *\0".as_ptr() as *const ::core::ffi::c_char);
        } else {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pTab).nCol {
                if (*pIdxInfo).colUsed
                    & (1 as ::core::ffi::c_int as sqlite3_uint64)
                        << (if i >= 63 as ::core::ffi::c_int {
                            63 as ::core::ffi::c_int
                        } else {
                            i
                        }) != 0
                {
                    zRet = sqlite3_mprintf(
                        b"%z, %s\0".as_ptr() as *const ::core::ffi::c_char,
                        zRet,
                        *(*pTab).aCol.offset(i as isize),
                    );
                } else {
                    zRet = sqlite3_mprintf(
                        b"%z, NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        zRet,
                    );
                }
                if zRet.is_null() {
                    break;
                }
                i += 1;
            }
        }
        return zRet;
    }
}
unsafe extern "C" fn echoBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        let mut zQuery: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zCol: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nArg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zSep: *const ::core::ffi::c_char = b"WHERE\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut pVtab: *mut echo_vtab = tab as *mut echo_vtab;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut interp: *mut Tcl_Interp = (*pVtab).interp;
        let mut nRow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut useIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut useCost: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut cost: ::core::ffi::c_double = 0 as ::core::ffi::c_int
            as ::core::ffi::c_double;
        let mut isIgnoreUsable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !Tcl_GetVar2(
                interp,
                b"echo_module_ignore_usable\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                1 as ::core::ffi::c_int,
            )
            .is_null()
        {
            isIgnoreUsable = 1 as ::core::ffi::c_int;
        }
        if simulateVtabError(
            pVtab,
            b"xBestIndex\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
            return SQLITE_ERROR;
        }
        if !Tcl_GetVar2(
                interp,
                b"echo_module_cost\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                1 as ::core::ffi::c_int,
            )
            .is_null()
        {
            cost = atof(
                Tcl_GetVar2(
                    interp,
                    b"echo_module_cost\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    1 as ::core::ffi::c_int,
                ),
            );
            useCost = 1 as ::core::ffi::c_int;
        } else {
            zQuery = sqlite3_mprintf(
                b"SELECT count(*) FROM %Q\0".as_ptr() as *const ::core::ffi::c_char,
                (*pVtab).zTableName,
            );
            if zQuery.is_null() {
                return SQLITE_NOMEM;
            }
            rc = sqlite3_prepare(
                (*pVtab).db,
                zQuery,
                -1 as ::core::ffi::c_int,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            sqlite3_free(zQuery as *mut ::core::ffi::c_void);
            if rc != SQLITE_OK {
                return rc;
            }
            sqlite3_step(pStmt);
            nRow = sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
            rc = sqlite3_finalize(pStmt);
            if rc != SQLITE_OK {
                return rc;
            }
        }
        zCol = echoSelectList(pVtab, pIdxInfo);
        if zCol.is_null() {
            return SQLITE_NOMEM;
        }
        zQuery = sqlite3_mprintf(
            b"SELECT rowid%z FROM %Q\0".as_ptr() as *const ::core::ffi::c_char,
            zCol,
            (*pVtab).zTableName,
        );
        if zQuery.is_null() {
            return SQLITE_NOMEM;
        }
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pIdxInfo).nConstraint {
            let mut pConstraint: *const sqlite3_index_constraint = ::core::ptr::null::<
                sqlite3_index_constraint,
            >();
            let mut pUsage: *mut sqlite3_index_constraint_usage = ::core::ptr::null_mut::<
                sqlite3_index_constraint_usage,
            >();
            let mut iCol: ::core::ffi::c_int = 0;
            pConstraint = (*pIdxInfo).aConstraint.offset(ii as isize)
                as *mut sqlite3_index_constraint;
            pUsage = (*pIdxInfo).aConstraintUsage.offset(ii as isize)
                as *mut sqlite3_index_constraint_usage
                as *mut sqlite3_index_constraint_usage;
            if !(isIgnoreUsable == 0 && (*pConstraint).usable == 0) {
                iCol = (*pConstraint).iColumn;
                if iCol < 0 as ::core::ffi::c_int
                    || *(*pVtab).aIndex.offset(iCol as isize) != 0
                {
                    let mut zNewCol: *mut ::core::ffi::c_char = (if iCol
                        >= 0 as ::core::ffi::c_int
                    {
                        *(*pVtab).aCol.offset(iCol as isize)
                            as *const ::core::ffi::c_char
                    } else {
                        b"rowid\0".as_ptr() as *const ::core::ffi::c_char
                    }) as *mut ::core::ffi::c_char;
                    let mut zOp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                        ::core::ffi::c_char,
                    >();
                    useIdx = 1 as ::core::ffi::c_int;
                    match (*pConstraint).op as ::core::ffi::c_int {
                        SQLITE_INDEX_CONSTRAINT_EQ => {
                            zOp = b"=\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        SQLITE_INDEX_CONSTRAINT_LT => {
                            zOp = b"<\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        SQLITE_INDEX_CONSTRAINT_GT => {
                            zOp = b">\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        SQLITE_INDEX_CONSTRAINT_LE => {
                            zOp = b"<=\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        SQLITE_INDEX_CONSTRAINT_GE => {
                            zOp = b">=\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        SQLITE_INDEX_CONSTRAINT_MATCH => {
                            zOp = b"LIKE\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        SQLITE_INDEX_CONSTRAINT_LIKE => {
                            zOp = b"like\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        SQLITE_INDEX_CONSTRAINT_GLOB => {
                            zOp = b"glob\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        SQLITE_INDEX_CONSTRAINT_REGEXP => {
                            zOp = b"regexp\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        _ => {}
                    }
                    if !zOp.is_null() {
                        if *zOp.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'L' as i32
                        {
                            zNew = sqlite3_mprintf(
                                b" %s %s LIKE (SELECT '%%'||?||'%%')\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                zSep,
                                zNewCol,
                            );
                        } else {
                            zNew = sqlite3_mprintf(
                                b" %s %s %s ?\0".as_ptr() as *const ::core::ffi::c_char,
                                zSep,
                                zNewCol,
                                zOp,
                            );
                        }
                        string_concat(
                            &raw mut zQuery,
                            zNew,
                            1 as ::core::ffi::c_int,
                            &raw mut rc,
                        );
                        zSep = b"AND\0".as_ptr() as *const ::core::ffi::c_char;
                        nArg += 1;
                        (*pUsage).argvIndex = nArg;
                        (*pUsage).omit = 1 as ::core::ffi::c_uchar;
                    }
                }
            }
            ii += 1;
        }
        if (*pIdxInfo).nOrderBy == 1 as ::core::ffi::c_int
            && ((*(*pIdxInfo).aOrderBy).iColumn < 0 as ::core::ffi::c_int
                || *(*pVtab).aIndex.offset((*(*pIdxInfo).aOrderBy).iColumn as isize)
                    != 0)
        {
            let mut iCol_0: ::core::ffi::c_int = (*(*pIdxInfo).aOrderBy).iColumn;
            let mut zNewCol_0: *mut ::core::ffi::c_char = (if iCol_0
                >= 0 as ::core::ffi::c_int
            {
                *(*pVtab).aCol.offset(iCol_0 as isize) as *const ::core::ffi::c_char
            } else {
                b"rowid\0".as_ptr() as *const ::core::ffi::c_char
            }) as *mut ::core::ffi::c_char;
            let mut zDir: *mut ::core::ffi::c_char = (if (*(*pIdxInfo).aOrderBy).desc
                as ::core::ffi::c_int != 0
            {
                b"DESC\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"ASC\0".as_ptr() as *const ::core::ffi::c_char
            }) as *mut ::core::ffi::c_char;
            zNew = sqlite3_mprintf(
                b" ORDER BY %s %s\0".as_ptr() as *const ::core::ffi::c_char,
                zNewCol_0,
                zDir,
            );
            string_concat(&raw mut zQuery, zNew, 1 as ::core::ffi::c_int, &raw mut rc);
            (*pIdxInfo).orderByConsumed = 1 as ::core::ffi::c_int;
        }
        appendToEchoModule(
            (*pVtab).interp,
            b"xBestIndex\0".as_ptr() as *const ::core::ffi::c_char,
        );
        appendToEchoModule((*pVtab).interp, zQuery);
        if zQuery.is_null() {
            return rc;
        }
        (*pIdxInfo).idxNum = hashString(zQuery);
        (*pIdxInfo).idxStr = zQuery;
        (*pIdxInfo).needToFreeIdxStr = 1 as ::core::ffi::c_int;
        if useCost != 0 {
            (*pIdxInfo).estimatedCost = cost;
        } else if useIdx != 0 {
            ii = 0 as ::core::ffi::c_int;
            while (ii as usize)
                < (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    .wrapping_mul(8 as usize)
                    .wrapping_sub(1 as usize)
            {
                if nRow & (1 as ::core::ffi::c_int) << ii != 0 {
                    (*pIdxInfo).estimatedCost = ii as ::core::ffi::c_double;
                }
                ii += 1;
            }
        } else {
            (*pIdxInfo).estimatedCost = nRow as ::core::ffi::c_double;
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn echoUpdate(
    mut tab: *mut sqlite3_vtab,
    mut nData: ::core::ffi::c_int,
    mut apData: *mut *mut sqlite3_value,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVtab: *mut echo_vtab = tab as *mut echo_vtab;
        let mut db: *mut sqlite3 = (*pVtab).db;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut bindArgZero: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bindArgOne: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        if simulateVtabError(pVtab, b"xUpdate\0".as_ptr() as *const ::core::ffi::c_char)
            != 0
        {
            return SQLITE_ERROR;
        }
        if nData > 1 as ::core::ffi::c_int
            && sqlite3_value_type(*apData.offset(0 as ::core::ffi::c_int as isize))
                == SQLITE_INTEGER
        {
            let mut zSep: *mut ::core::ffi::c_char = b" SET\0".as_ptr()
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            z = sqlite3_mprintf(
                b"UPDATE %Q\0".as_ptr() as *const ::core::ffi::c_char,
                (*pVtab).zTableName,
            );
            if z.is_null() {
                rc = SQLITE_NOMEM;
            }
            bindArgOne = (!(*apData.offset(1 as ::core::ffi::c_int as isize)).is_null()
                && sqlite3_value_type(*apData.offset(1 as ::core::ffi::c_int as isize))
                    == SQLITE_INTEGER) as ::core::ffi::c_int;
            bindArgZero = 1 as ::core::ffi::c_int;
            if bindArgOne != 0 {
                string_concat(
                    &raw mut z,
                    b" SET rowid=?1 \0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                    &raw mut rc,
                );
                zSep = b",\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            i = 2 as ::core::ffi::c_int;
            while i < nData {
                if !(*apData.offset(i as isize)).is_null() {
                    string_concat(
                        &raw mut z,
                        sqlite3_mprintf(
                            b"%s %Q=?%d\0".as_ptr() as *const ::core::ffi::c_char,
                            zSep,
                            *(*pVtab)
                                .aCol
                                .offset((i - 2 as ::core::ffi::c_int) as isize),
                            i,
                        ),
                        1 as ::core::ffi::c_int,
                        &raw mut rc,
                    );
                    zSep = b",\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                }
                i += 1;
            }
            string_concat(
                &raw mut z,
                sqlite3_mprintf(
                    b" WHERE rowid=?%d\0".as_ptr() as *const ::core::ffi::c_char,
                    nData,
                ),
                1 as ::core::ffi::c_int,
                &raw mut rc,
            );
        } else if nData == 1 as ::core::ffi::c_int
            && sqlite3_value_type(*apData.offset(0 as ::core::ffi::c_int as isize))
                == SQLITE_INTEGER
        {
            z = sqlite3_mprintf(
                b"DELETE FROM %Q WHERE rowid = ?1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pVtab).zTableName,
            );
            if z.is_null() {
                rc = SQLITE_NOMEM;
            }
            bindArgZero = 1 as ::core::ffi::c_int;
        } else if nData > 2 as ::core::ffi::c_int
            && sqlite3_value_type(*apData.offset(0 as ::core::ffi::c_int as isize))
                == SQLITE_NULL
        {
            let mut ii: ::core::ffi::c_int = 0;
            let mut zInsert: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut zValues: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            zInsert = sqlite3_mprintf(
                b"INSERT INTO %Q (\0".as_ptr() as *const ::core::ffi::c_char,
                (*pVtab).zTableName,
            );
            if zInsert.is_null() {
                rc = SQLITE_NOMEM;
            }
            if sqlite3_value_type(*apData.offset(1 as ::core::ffi::c_int as isize))
                == SQLITE_INTEGER
            {
                bindArgOne = 1 as ::core::ffi::c_int;
                zValues = sqlite3_mprintf(b"?\0".as_ptr() as *const ::core::ffi::c_char);
                string_concat(
                    &raw mut zInsert,
                    b"rowid\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                    &raw mut rc,
                );
            }
            ii = 2 as ::core::ffi::c_int;
            while ii < nData {
                string_concat(
                    &raw mut zInsert,
                    sqlite3_mprintf(
                        b"%s%Q\0".as_ptr() as *const ::core::ffi::c_char,
                        if !zValues.is_null() {
                            b", \0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        *(*pVtab).aCol.offset((ii - 2 as ::core::ffi::c_int) as isize),
                    ),
                    1 as ::core::ffi::c_int,
                    &raw mut rc,
                );
                string_concat(
                    &raw mut zValues,
                    sqlite3_mprintf(
                        b"%s?%d\0".as_ptr() as *const ::core::ffi::c_char,
                        if !zValues.is_null() {
                            b", \0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        ii,
                    ),
                    1 as ::core::ffi::c_int,
                    &raw mut rc,
                );
                ii += 1;
            }
            string_concat(&raw mut z, zInsert, 1 as ::core::ffi::c_int, &raw mut rc);
            string_concat(
                &raw mut z,
                b") VALUES(\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                &raw mut rc,
            );
            string_concat(&raw mut z, zValues, 1 as ::core::ffi::c_int, &raw mut rc);
            string_concat(
                &raw mut z,
                b")\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                &raw mut rc,
            );
        } else {
            return SQLITE_ERROR
        }
        if rc == SQLITE_OK {
            rc = sqlite3_prepare(
                db,
                z,
                -1 as ::core::ffi::c_int,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
        }
        sqlite3_free(z as *mut ::core::ffi::c_void);
        if rc == SQLITE_OK {
            if bindArgZero != 0 {
                sqlite3_bind_value(
                    pStmt,
                    nData,
                    *apData.offset(0 as ::core::ffi::c_int as isize),
                );
            }
            if bindArgOne != 0 {
                sqlite3_bind_value(
                    pStmt,
                    1 as ::core::ffi::c_int,
                    *apData.offset(1 as ::core::ffi::c_int as isize),
                );
            }
            i = 2 as ::core::ffi::c_int;
            while i < nData && rc == SQLITE_OK {
                if !(*apData.offset(i as isize)).is_null() {
                    rc = sqlite3_bind_value(pStmt, i, *apData.offset(i as isize));
                }
                i += 1;
            }
            if rc == SQLITE_OK {
                sqlite3_step(pStmt);
                rc = sqlite3_finalize(pStmt);
            } else {
                sqlite3_finalize(pStmt);
            }
        }
        if !pRowid.is_null() && rc == SQLITE_OK {
            *pRowid = sqlite3_last_insert_rowid(db) as sqlite_int64;
        }
        if rc != SQLITE_OK {
            (*tab).zErrMsg = sqlite3_mprintf(
                b"echo-vtab-error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
        }
        return rc;
    }
}
unsafe extern "C" fn echoTransactionCall(
    mut tab: *mut sqlite3_vtab,
    mut zCall: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pVtab: *mut echo_vtab = tab as *mut echo_vtab;
        z = sqlite3_mprintf(
            b"echo(%s)\0".as_ptr() as *const ::core::ffi::c_char,
            (*pVtab).zTableName,
        );
        if z.is_null() {
            return SQLITE_NOMEM;
        }
        appendToEchoModule((*pVtab).interp, zCall);
        appendToEchoModule((*pVtab).interp, z);
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn echoBegin(mut tab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pVtab: *mut echo_vtab = tab as *mut echo_vtab;
        let mut interp: *mut Tcl_Interp = (*pVtab).interp;
        let mut zVal: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if simulateVtabError(pVtab, b"xBegin\0".as_ptr() as *const ::core::ffi::c_char)
            != 0
        {
            return SQLITE_ERROR;
        }
        rc = echoTransactionCall(
            tab,
            b"xBegin\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if rc == SQLITE_OK {
            zVal = Tcl_GetVar2(
                interp,
                b"echo_module_begin_fail\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                1 as ::core::ffi::c_int,
            );
            if !zVal.is_null()
                && 0 as ::core::ffi::c_int == strcmp(zVal, (*pVtab).zTableName)
            {
                rc = SQLITE_ERROR;
            }
        }
        if rc == SQLITE_OK {
            (*pVtab).inTransaction = 1 as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn echoSync(mut tab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pVtab: *mut echo_vtab = tab as *mut echo_vtab;
        let mut interp: *mut Tcl_Interp = (*pVtab).interp;
        let mut zVal: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if simulateVtabError(pVtab, b"xSync\0".as_ptr() as *const ::core::ffi::c_char)
            != 0
        {
            return SQLITE_ERROR;
        }
        rc = echoTransactionCall(tab, b"xSync\0".as_ptr() as *const ::core::ffi::c_char);
        if rc == SQLITE_OK {
            zVal = Tcl_GetVar2(
                interp,
                b"echo_module_sync_fail\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                1 as ::core::ffi::c_int,
            );
            if !zVal.is_null()
                && 0 as ::core::ffi::c_int == strcmp(zVal, (*pVtab).zTableName)
            {
                rc = -1 as ::core::ffi::c_int;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn echoCommit(mut tab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        let mut pVtab: *mut echo_vtab = tab as *mut echo_vtab;
        let mut rc: ::core::ffi::c_int = 0;
        if simulateVtabError(pVtab, b"xCommit\0".as_ptr() as *const ::core::ffi::c_char)
            != 0
        {
            return SQLITE_ERROR;
        }
        sqlite3BeginBenignMalloc();
        rc = echoTransactionCall(
            tab,
            b"xCommit\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3EndBenignMalloc();
        (*pVtab).inTransaction = 0 as ::core::ffi::c_int;
        return rc;
    }
}
unsafe extern "C" fn echoRollback(mut tab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pVtab: *mut echo_vtab = tab as *mut echo_vtab;
        rc = echoTransactionCall(
            tab,
            b"xRollback\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*pVtab).inTransaction = 0 as ::core::ffi::c_int;
        return rc;
    }
}
unsafe extern "C" fn overloadedGlobFunction(
    mut pContext: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut interp: *mut Tcl_Interp = sqlite3_user_data(pContext) as *mut Tcl_Interp;
        let mut str: Tcl_DString = Tcl_DString {
            string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            length: 0,
            spaceAvl: 0,
            staticSpace: [0; 200],
        };
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        Tcl_DStringInit(&raw mut str);
        Tcl_DStringAppendElement(
            &raw mut str,
            b"::echo_glob_overload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < nArg {
            Tcl_DStringAppendElement(
                &raw mut str,
                sqlite3_value_text(*apArg.offset(i as isize)) as *mut ::core::ffi::c_char,
            );
            i += 1;
        }
        rc = Tcl_Eval(interp, str.string);
        Tcl_DStringFree(&raw mut str);
        if rc != 0 {
            sqlite3_result_error(
                pContext,
                Tcl_GetStringResult(interp),
                -1 as ::core::ffi::c_int,
            );
        } else {
            sqlite3_result_text(
                pContext,
                Tcl_GetStringResult(interp),
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        Tcl_ResetResult(interp);
    }
}
unsafe extern "C" fn echoFindFunction(
    mut vtab: *mut sqlite3_vtab,
    mut nArg: ::core::ffi::c_int,
    mut zFuncName: *const ::core::ffi::c_char,
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
        let mut pVtab: *mut echo_vtab = vtab as *mut echo_vtab;
        let mut interp: *mut Tcl_Interp = (*pVtab).interp;
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
        if strcmp(zFuncName, b"glob\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if Tcl_GetCommandInfo(
            interp,
            b"::echo_glob_overload\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut info,
        ) == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        *pxFunc = Some(
            overloadedGlobFunction
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
        *ppArg = interp as *mut ::core::ffi::c_void;
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn echoRename(
    mut vtab: *mut sqlite3_vtab,
    mut zNewName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *mut echo_vtab = vtab as *mut echo_vtab;
        if simulateVtabError(p, b"xRename\0".as_ptr() as *const ::core::ffi::c_char) != 0
        {
            return SQLITE_ERROR;
        }
        if (*p).isPattern != 0 {
            let mut nThis: ::core::ffi::c_int = strlen((*p).zThis) as ::core::ffi::c_int;
            let mut zSql: *mut ::core::ffi::c_char = sqlite3_mprintf(
                b"ALTER TABLE %s RENAME TO %s%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zTableName,
                zNewName,
                (*p).zTableName.offset(nThis as isize) as *mut ::core::ffi::c_char,
            );
            rc = sqlite3_exec(
                (*p).db,
                zSql,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
unsafe extern "C" fn echoSavepoint(
    mut pVTab: *mut sqlite3_vtab,
    mut iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn echoRelease(
    mut pVTab: *mut sqlite3_vtab,
    mut iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn echoRollbackTo(
    mut pVTab: *mut sqlite3_vtab,
    mut iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
static mut echoModule: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 1 as ::core::ffi::c_int,
        xCreate: Some(
            echoCreate
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
            echoConnect
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
            echoBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            echoDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            echoDestroy as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            echoOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            echoClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            echoFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            echoNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            echoEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            echoColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            echoRowid
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: Some(
            echoUpdate
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xBegin: Some(
            echoBegin as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            echoSync as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xCommit: Some(
            echoCommit as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xRollback: Some(
            echoRollback as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xFindFunction: Some(
            echoFindFunction
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
            echoRename
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSavepoint: None,
        xRelease: None,
        xRollbackTo: None,
        xShadowName: None,
        xIntegrity: None,
    }
};
static mut echoModuleV2: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 2 as ::core::ffi::c_int,
        xCreate: Some(
            echoCreate
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
            echoConnect
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
            echoBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            echoDisconnect
                as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            echoDestroy as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            echoOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            echoClose
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            echoFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            echoNext
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            echoEof
                as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            echoColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            echoRowid
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: Some(
            echoUpdate
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xBegin: Some(
            echoBegin as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            echoSync as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xCommit: Some(
            echoCommit as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xRollback: Some(
            echoRollback as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xFindFunction: Some(
            echoFindFunction
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
            echoRename
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSavepoint: Some(
            echoSavepoint
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRelease: Some(
            echoRelease
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRollbackTo: Some(
            echoRollbackTo
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShadowName: None,
        xIntegrity: None,
    }
};
unsafe extern "C" fn moduleDestroy(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        let mut pMod: *mut EchoModule = p as *mut EchoModule;
        sqlite3_create_function(
            (*pMod).db,
            b"function_that_does_not_exist_0982ma98\0".as_ptr()
                as *const ::core::ffi::c_char,
            SQLITE_ANY,
            1 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
            None,
            None,
        );
        sqlite3_free(p);
    }
}
unsafe extern "C" fn register_echo_module(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pMod: *mut EchoModule = ::core::ptr::null_mut::<EchoModule>();
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
        pMod = sqlite3_malloc(::core::mem::size_of::<EchoModule>() as ::core::ffi::c_int)
            as *mut EchoModule;
        (*pMod).interp = interp;
        (*pMod).db = db;
        rc = sqlite3_create_module_v2(
            db,
            b"echo\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut echoModule,
            pMod as *mut ::core::ffi::c_void,
            Some(moduleDestroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        if rc == SQLITE_OK {
            pMod = sqlite3_malloc(
                ::core::mem::size_of::<EchoModule>() as ::core::ffi::c_int,
            ) as *mut EchoModule;
            (*pMod).interp = interp;
            (*pMod).db = db;
            rc = sqlite3_create_module_v2(
                db,
                b"echo_v2\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut echoModuleV2,
                pMod as *mut ::core::ffi::c_void,
                Some(
                    moduleDestroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        }
        Tcl_SetResult(
            interp,
            sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
            TCL_STATIC,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn declare_vtab(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB SQL\0".as_ptr() as *const ::core::ffi::c_char,
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
        rc = sqlite3_declare_vtab(
            db,
            Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
        );
        if rc != SQLITE_OK {
            Tcl_SetResult(
                interp,
                sqlite3_errmsg(db) as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest8_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aObjCmd: [C2Rust_Unnamed_25; 2] = unsafe {
            [
                C2Rust_Unnamed_25 {
                    zName: b"register_echo_module\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        register_echo_module
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
                C2Rust_Unnamed_25 {
                    zName: b"sqlite3_declare_vtab\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        declare_vtab
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
            < (::core::mem::size_of::<[C2Rust_Unnamed_25; 2]>() as usize)
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
