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
    fn sqlite3_realloc(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
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
    fn sqlite3MultiValues(
        pParse: *mut Parse,
        pLeft: *mut Select,
        pRow: *mut ExprList,
    ) -> *mut Select;
    fn sqlite3MultiValuesEnd(pParse: *mut Parse, pVal: *mut Select);
    fn sqlite3WindowDelete(_: *mut sqlite3, _: *mut Window);
    fn sqlite3WindowListDelete(db: *mut sqlite3, p: *mut Window);
    fn sqlite3WindowAlloc(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: *mut Expr,
        _: u8_0,
    ) -> *mut Window;
    fn sqlite3WindowAttach(_: *mut Parse, _: *mut Expr, _: *mut Window);
    fn sqlite3WindowChain(_: *mut Parse, _: *mut Window, _: *mut Window);
    fn sqlite3WindowAssemble(
        _: *mut Parse,
        _: *mut Window,
        _: *mut ExprList,
        _: *mut ExprList,
        _: *mut Token,
    ) -> *mut Window;
    fn sqlite3DbMallocZero(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbMallocRawNN(_: *mut sqlite3, _: u64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3DbStrNDup(
        _: *mut sqlite3,
        _: *const ::core::ffi::c_char,
        _: u64_0,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3ErrorMsg(_: *mut Parse, _: *const ::core::ffi::c_char, ...);
    fn sqlite3DequoteExpr(_: *mut Expr);
    fn sqlite3DequoteNumber(_: *mut Parse, _: *mut Expr);
    fn sqlite3FinishCoding(_: *mut Parse);
    fn sqlite3ExprAlloc(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const Token,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3Expr(
        _: *mut sqlite3,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> *mut Expr;
    fn sqlite3ExprAttachSubtrees(_: *mut sqlite3, _: *mut Expr, _: *mut Expr, _: *mut Expr);
    fn sqlite3PExpr(_: *mut Parse, _: ::core::ffi::c_int, _: *mut Expr, _: *mut Expr) -> *mut Expr;
    fn sqlite3PExprAddSelect(_: *mut Parse, _: *mut Expr, _: *mut Select);
    fn sqlite3ExprAnd(_: *mut Parse, _: *mut Expr, _: *mut Expr) -> *mut Expr;
    fn sqlite3ExprFunction(
        _: *mut Parse,
        _: *mut ExprList,
        _: *const Token,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    fn sqlite3ExprAddFunctionOrderBy(_: *mut Parse, _: *mut Expr, _: *mut ExprList);
    fn sqlite3ExprAssignVarNumber(_: *mut Parse, _: *mut Expr, _: u32_0);
    fn sqlite3ExprDelete(_: *mut sqlite3, _: *mut Expr);
    fn sqlite3ExprUnmapAndDelete(_: *mut Parse, _: *mut Expr);
    fn sqlite3ExprListAppend(_: *mut Parse, _: *mut ExprList, _: *mut Expr) -> *mut ExprList;
    fn sqlite3ExprListAppendVector(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut IdList,
        _: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListToValues(
        _: *mut Parse,
        _: ::core::ffi::c_int,
        _: *mut ExprList,
    ) -> *mut Select;
    fn sqlite3ExprListSetSortOrder(_: *mut ExprList, _: ::core::ffi::c_int, _: ::core::ffi::c_int);
    fn sqlite3ExprListSetName(
        _: *mut Parse,
        _: *mut ExprList,
        _: *const Token,
        _: ::core::ffi::c_int,
    );
    fn sqlite3ExprListSetSpan(
        _: *mut Parse,
        _: *mut ExprList,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3ExprListDelete(_: *mut sqlite3, _: *mut ExprList);
    fn sqlite3Pragma(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: *mut Token,
        _: ::core::ffi::c_int,
    );
    fn sqlite3StartTable(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3AddColumn(_: *mut Parse, _: Token, _: Token);
    fn sqlite3AddNotNull(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3AddPrimaryKey(
        _: *mut Parse,
        _: *mut ExprList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3AddCheckConstraint(
        _: *mut Parse,
        _: *mut Expr,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3AddDefaultValue(
        _: *mut Parse,
        _: *mut Expr,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    );
    fn sqlite3AddCollateType(_: *mut Parse, _: *mut Token);
    fn sqlite3AddGenerated(_: *mut Parse, _: *mut Expr, _: *mut Token);
    fn sqlite3EndTable(_: *mut Parse, _: *mut Token, _: *mut Token, _: u32_0, _: *mut Select);
    fn sqlite3AddReturning(_: *mut Parse, _: *mut ExprList);
    fn sqlite3FaultSim(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3CreateView(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: *mut Token,
        _: *mut ExprList,
        _: *mut Select,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3DropTable(
        _: *mut Parse,
        _: *mut SrcList,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3Insert(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Select,
        _: *mut IdList,
        _: ::core::ffi::c_int,
        _: *mut Upsert,
    );
    fn sqlite3IdListAppend(_: *mut Parse, _: *mut IdList, _: *mut Token) -> *mut IdList;
    fn sqlite3SrcListAppendList(
        pParse: *mut Parse,
        p1: *mut SrcList,
        p2: *mut SrcList,
    ) -> *mut SrcList;
    fn sqlite3SrcListAppend(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Token,
        _: *mut Token,
    ) -> *mut SrcList;
    fn sqlite3SrcListAppendFromTerm(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Token,
        _: *mut Token,
        _: *mut Token,
        _: *mut Select,
        _: *mut OnOrUsing,
    ) -> *mut SrcList;
    fn sqlite3SrcListIndexedBy(_: *mut Parse, _: *mut SrcList, _: *mut Token);
    fn sqlite3SrcListFuncArgs(_: *mut Parse, _: *mut SrcList, _: *mut ExprList);
    fn sqlite3SrcListShiftJoinType(_: *mut Parse, _: *mut SrcList);
    fn sqlite3IdListDelete(_: *mut sqlite3, _: *mut IdList);
    fn sqlite3SrcListDelete(_: *mut sqlite3, _: *mut SrcList);
    fn sqlite3CreateIndex(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: *mut SrcList,
        _: *mut ExprList,
        _: ::core::ffi::c_int,
        _: *mut Token,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: u8_0,
    );
    fn sqlite3DropIndex(_: *mut Parse, _: *mut SrcList, _: ::core::ffi::c_int);
    fn sqlite3Select(_: *mut Parse, _: *mut Select, _: *mut SelectDest) -> ::core::ffi::c_int;
    fn sqlite3SelectNew(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut SrcList,
        _: *mut Expr,
        _: *mut ExprList,
        _: *mut Expr,
        _: *mut ExprList,
        _: u32_0,
        _: *mut Expr,
    ) -> *mut Select;
    fn sqlite3SelectDelete(_: *mut sqlite3, _: *mut Select);
    fn sqlite3DeleteFrom(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut Expr,
        _: *mut ExprList,
        _: *mut Expr,
    );
    fn sqlite3Update(
        _: *mut Parse,
        _: *mut SrcList,
        _: *mut ExprList,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: *mut ExprList,
        _: *mut Expr,
        _: *mut Upsert,
    );
    fn sqlite3Vacuum(_: *mut Parse, _: *mut Token, _: *mut Expr);
    fn sqlite3NameFromToken(_: *mut sqlite3, _: *const Token) -> *mut ::core::ffi::c_char;
    fn sqlite3BeginTransaction(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3EndTransaction(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3Savepoint(_: *mut Parse, _: ::core::ffi::c_int, _: *mut Token);
    fn sqlite3ExprIdToTrueFalse(_: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3ExprIsConstant(_: *mut Parse, _: *mut Expr) -> ::core::ffi::c_int;
    fn sqlite3BeginTrigger(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut IdList,
        _: *mut SrcList,
        _: *mut Expr,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    fn sqlite3FinishTrigger(_: *mut Parse, _: *mut TriggerStep, _: *mut Token);
    fn sqlite3DropTrigger(_: *mut Parse, _: *mut SrcList, _: ::core::ffi::c_int);
    fn sqlite3DeleteTriggerStep(_: *mut sqlite3, _: *mut TriggerStep);
    fn sqlite3TriggerSelectStep(
        _: *mut sqlite3,
        _: *mut Select,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut TriggerStep;
    fn sqlite3TriggerInsertStep(
        _: *mut Parse,
        _: *mut Token,
        _: *mut IdList,
        _: *mut Select,
        _: u8_0,
        _: *mut Upsert,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut TriggerStep;
    fn sqlite3TriggerUpdateStep(
        _: *mut Parse,
        _: *mut Token,
        _: *mut SrcList,
        _: *mut ExprList,
        _: *mut Expr,
        _: u8_0,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut TriggerStep;
    fn sqlite3TriggerDeleteStep(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Expr,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut TriggerStep;
    fn sqlite3JoinType(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: *mut Token,
    ) -> ::core::ffi::c_int;
    fn sqlite3CreateForeignKey(
        _: *mut Parse,
        _: *mut ExprList,
        _: *mut Token,
        _: *mut ExprList,
        _: ::core::ffi::c_int,
    );
    fn sqlite3DeferForeignKey(_: *mut Parse, _: ::core::ffi::c_int);
    fn sqlite3Attach(_: *mut Parse, _: *mut Expr, _: *mut Expr, _: *mut Expr);
    fn sqlite3Detach(_: *mut Parse, _: *mut Expr);
    fn sqlite3GetInt32(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> ::core::ffi::c_int;
    fn sqlite3ExprAddCollateToken(
        pParse: *const Parse,
        _: *mut Expr,
        _: *const Token,
        _: ::core::ffi::c_int,
    ) -> *mut Expr;
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
    fn sqlite3Reindex(_: *mut Parse, _: *mut Token, _: *mut Token);
    fn sqlite3AlterRenameTable(_: *mut Parse, _: *mut SrcList, _: *mut Token);
    fn sqlite3AlterRenameColumn(_: *mut Parse, _: *mut SrcList, _: *mut Token, _: *mut Token);
    fn sqlite3AlterFinishAddColumn(_: *mut Parse, _: *mut Token);
    fn sqlite3AlterBeginAddColumn(_: *mut Parse, _: *mut SrcList);
    fn sqlite3AlterDropColumn(_: *mut Parse, _: *mut SrcList, _: *const Token);
    fn sqlite3RenameTokenMap(
        _: *mut Parse,
        _: *const ::core::ffi::c_void,
        _: *const Token,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3RenameTokenRemap(
        _: *mut Parse,
        pTo: *const ::core::ffi::c_void,
        pFrom: *const ::core::ffi::c_void,
    );
    fn sqlite3Analyze(_: *mut Parse, _: *mut Token, _: *mut Token);
    fn sqlite3SelectOpName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3OomFault(_: *mut sqlite3) -> *mut ::core::ffi::c_void;
    fn sqlite3VtabBeginParse(
        _: *mut Parse,
        _: *mut Token,
        _: *mut Token,
        _: *mut Token,
        _: ::core::ffi::c_int,
    );
    fn sqlite3VtabFinishParse(_: *mut Parse, _: *mut Token);
    fn sqlite3VtabArgInit(_: *mut Parse);
    fn sqlite3VtabArgExtend(_: *mut Parse, _: *mut Token);
    fn sqlite3ExprListCheckLength(_: *mut Parse, _: *mut ExprList, _: *const ::core::ffi::c_char);
    fn sqlite3CteNew(
        _: *mut Parse,
        _: *mut Token,
        _: *mut ExprList,
        _: *mut Select,
        _: u8_0,
    ) -> *mut Cte;
    fn sqlite3WithAdd(_: *mut Parse, _: *mut With, _: *mut Cte) -> *mut With;
    fn sqlite3WithDelete(_: *mut sqlite3, _: *mut With);
    fn sqlite3WithPush(_: *mut Parse, _: *mut With, _: u8_0) -> *mut With;
    fn sqlite3UpsertNew(
        _: *mut sqlite3,
        _: *mut ExprList,
        _: *mut Expr,
        _: *mut ExprList,
        _: *mut Expr,
        _: *mut Upsert,
    ) -> *mut Upsert;
    fn sqlite3ExprSetHeightAndFlags(pParse: *mut Parse, p: *mut Expr);
    fn sqlite3ExprSetErrorOffset(_: *mut Expr, _: ::core::ffi::c_int);
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnOrUsing {
    pub pOn: *mut Expr,
    pub pUsing: *mut IdList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SelectDest {
    pub eDest: u8_0,
    pub iSDParm: ::core::ffi::c_int,
    pub iSDParm2: ::core::ffi::c_int,
    pub iSdst: ::core::ffi::c_int,
    pub nSdst: ::core::ffi::c_int,
    pub zAffSdst: *mut ::core::ffi::c_char,
    pub pOrderBy: *mut ExprList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yyParser {
    pub yytos: *mut yyStackEntry,
    pub pParse: *mut Parse,
    pub yystackEnd: *mut yyStackEntry,
    pub yystack: *mut yyStackEntry,
    pub yystk0: [yyStackEntry; 100],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yyStackEntry {
    pub stateno: ::core::ffi::c_ushort,
    pub major: ::core::ffi::c_ushort,
    pub minor: YYMINORTYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYMINORTYPE {
    pub yyinit: ::core::ffi::c_int,
    pub yy0: Token,
    pub yy9: u32_0,
    pub yy28: TrigEvent,
    pub yy125: *mut With,
    pub yy204: *mut IdList,
    pub yy205: FrameBound,
    pub yy319: *mut TriggerStep,
    pub yy342: *const ::core::ffi::c_char,
    pub yy361: *mut Cte,
    pub yy402: *mut ExprList,
    pub yy403: *mut Upsert,
    pub yy421: OnOrUsing,
    pub yy444: u8_0,
    pub yy481: C2RustUnnamed_22,
    pub yy483: *mut Window,
    pub yy502: ::core::ffi::c_int,
    pub yy563: *mut SrcList,
    pub yy590: *mut Expr,
    pub yy637: *mut Select,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub value: ::core::ffi::c_int,
    pub mask: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameBound {
    pub eType: ::core::ffi::c_int,
    pub pExpr: *mut Expr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TrigEvent {
    pub a: ::core::ffi::c_int,
    pub b: *mut IdList,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_COMPOUND_SELECT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TK_DEFERRED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const TK_NOT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const TK_EXISTS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const TK_BEFORE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const TK_CAST: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const TK_OR: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const TK_AND: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const TK_IS: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const TK_ISNOT: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const TK_BETWEEN: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
pub const TK_IN: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const TK_ISNULL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const TK_NOTNULL: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const TK_EQ: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const TK_ID: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const TK_INSTEAD: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const TK_RAISE: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const TK_CURRENT: ::core::ffi::c_int = 86 as ::core::ffi::c_int;
pub const TK_UNBOUNDED: ::core::ffi::c_int = 91 as ::core::ffi::c_int;
pub const TK_PLUS: ::core::ffi::c_int = 107 as ::core::ffi::c_int;
pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const TK_UPDATE: ::core::ffi::c_int = 130 as ::core::ffi::c_int;
pub const TK_ALL: ::core::ffi::c_int = 136 as ::core::ffi::c_int;
pub const TK_SELECT: ::core::ffi::c_int = 139 as ::core::ffi::c_int;
pub const TK_DOT: ::core::ffi::c_int = 142 as ::core::ffi::c_int;
pub const TK_LIMIT: ::core::ffi::c_int = 149 as ::core::ffi::c_int;
pub const TK_INTEGER: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
pub const TK_VARIABLE: ::core::ffi::c_int = 157 as ::core::ffi::c_int;
pub const TK_CASE: ::core::ffi::c_int = 158 as ::core::ffi::c_int;
pub const TK_FILTER: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const TK_UPLUS: ::core::ffi::c_int = 173 as ::core::ffi::c_int;
pub const TK_UMINUS: ::core::ffi::c_int = 174 as ::core::ffi::c_int;
pub const TK_REGISTER: ::core::ffi::c_int = 176 as ::core::ffi::c_int;
pub const TK_VECTOR: ::core::ffi::c_int = 177 as ::core::ffi::c_int;
pub const TK_ASTERISK: ::core::ffi::c_int = 180 as ::core::ffi::c_int;
pub const DBFLAG_EncodingFixed: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SAVEPOINT_BEGIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SAVEPOINT_RELEASE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SAVEPOINT_ROLLBACK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_SO_ASC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_SO_DESC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_SO_UNDEFINED: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TF_NoVisibleRowid: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const TF_Strict: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const OE_None: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OE_Rollback: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OE_Fail: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OE_Ignore: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OE_Replace: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OE_Restrict: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const OE_SetNull: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OE_SetDflt: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const OE_Cascade: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const OE_Default: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_APPDEF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_IDXTYPE_UNIQUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EP_HasFunc: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const EP_InfixFunc: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const EP_Collate: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const EP_Subquery: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
pub const EP_Leaf: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;
pub const EP_Propagate: ::core::ffi::c_int = EP_Collate | EP_Subquery | EP_HasFunc;
pub const JT_INNER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SF_Distinct: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SF_All: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SF_Compound: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SF_Values: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SF_MultiValue: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SF_NestedFrom: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SRT_Output: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const M10d_Yes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const M10d_Any: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const M10d_No: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn parserSyntaxError(mut pParse: *mut Parse, mut p: *mut Token) {
    sqlite3ErrorMsg(
        pParse,
        b"near \"%T\": syntax error\0" as *const u8 as *const ::core::ffi::c_char,
        p,
    );
}
unsafe extern "C" fn disableLookaside(mut pParse: *mut Parse) {
    let mut db: *mut sqlite3 = (*pParse).db;
    (*pParse).disableLookaside = (*pParse).disableLookaside.wrapping_add(1);
    memset(
        &raw mut (*pParse).u1.cr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<C2RustUnnamed_20>() as size_t,
    );
    (*db).lookaside.bDisable = (*db).lookaside.bDisable.wrapping_add(1);
    (*db).lookaside.sz = 0 as u16_0;
}
pub const YYWILDCARD: ::core::ffi::c_int = 102 as ::core::ffi::c_int;
unsafe extern "C" fn parserDoubleLinkSelect(mut pParse: *mut Parse, mut p: *mut Select) {
    if !(*p).pPrior.is_null() {
        let mut pNext: *mut Select = ::core::ptr::null_mut::<Select>();
        let mut pLoop: *mut Select = p;
        let mut mxSelect: ::core::ffi::c_int = 0;
        let mut cnt: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        loop {
            (*pLoop).pNext = pNext;
            (*pLoop).selFlags |= SF_Compound as u32_0;
            pNext = pLoop;
            pLoop = (*pLoop).pPrior;
            if pLoop.is_null() {
                break;
            }
            cnt += 1;
            if !(!(*pLoop).pOrderBy.is_null() || !(*pLoop).pLimit.is_null()) {
                continue;
            }
            sqlite3ErrorMsg(
                pParse,
                b"%s clause should come after %s not before\0" as *const u8
                    as *const ::core::ffi::c_char,
                if !(*pLoop).pOrderBy.is_null() {
                    b"ORDER BY\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"LIMIT\0" as *const u8 as *const ::core::ffi::c_char
                },
                sqlite3SelectOpName((*pNext).op as ::core::ffi::c_int),
            );
            break;
        }
        if (*p).selFlags & (SF_MultiValue | SF_Values) as u32_0 == 0 as u32_0
            && {
                mxSelect = (*(*pParse).db).aLimit[SQLITE_LIMIT_COMPOUND_SELECT as usize];
                mxSelect > 0 as ::core::ffi::c_int
            }
            && cnt > mxSelect
        {
            sqlite3ErrorMsg(
                pParse,
                b"too many terms in compound SELECT\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
}
pub const YYSTACKDEPTH: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
unsafe extern "C" fn attachWithToSelect(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
    mut pWith: *mut With,
) -> *mut Select {
    if !pSelect.is_null() {
        (*pSelect).pWith = pWith;
        parserDoubleLinkSelect(pParse, pSelect);
    } else {
        sqlite3WithDelete((*pParse).db, pWith);
    }
    return pSelect;
}
pub const YY_MAX_SHIFT: ::core::ffi::c_int = 582 as ::core::ffi::c_int;
pub const YY_MIN_SHIFTREDUCE: ::core::ffi::c_int = 845 as ::core::ffi::c_int;
unsafe extern "C" fn parserStackRealloc(
    mut pOld: *mut ::core::ffi::c_void,
    mut newSize: sqlite3_uint64,
) -> *mut ::core::ffi::c_void {
    return if sqlite3FaultSim(700 as ::core::ffi::c_int) != 0 {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    } else {
        sqlite3_realloc(pOld, newSize as ::core::ffi::c_int)
    };
}
pub const YY_MAX_SHIFTREDUCE: ::core::ffi::c_int = 1253 as ::core::ffi::c_int;
pub const YY_ACCEPT_ACTION: ::core::ffi::c_int = 1255 as ::core::ffi::c_int;
pub const YY_MIN_REDUCE: ::core::ffi::c_int = 1257 as ::core::ffi::c_int;
pub const YY_MIN_DSTRCTR: ::core::ffi::c_int = 206 as ::core::ffi::c_int;
static mut yy_action: [::core::ffi::c_ushort; 2207] = [
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1328 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    460 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1622 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    381 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1328 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1542 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    481 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    524 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    460 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    459 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    558 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    983 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    375 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    498 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    61 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    61 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    984 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    182 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    481 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    536 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    432 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    525 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    557 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    421 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    559 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    460 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    398 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1249 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1050 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1050 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1064 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    581 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1174 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1174 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1584 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    379 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    481 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1341 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    299 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1341 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    498 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    862 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    523 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    523 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1250 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1345 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1054 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    547 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1541 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    302 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    470 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    550 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    550 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    538 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    483 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1019 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1085 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    93 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    401 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1498 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    426 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    344 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    467 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    332 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    6 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    511 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    508 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    507 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    94 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1019 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    551 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    551 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    506 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    38 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    411 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    530 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    398 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1148 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    39 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    344 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    375 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    471 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    231 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    46 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    160 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    442 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    544 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1250 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    874 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    380 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    382 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    360 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    182 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    557 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1339 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    557 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    557 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    472 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    346 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    526 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    531 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    556 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    439 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1511 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    465 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1511 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1513 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    532 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    512 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    411 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    874 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    560 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1293 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    493 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    511 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    508 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    507 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1628 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    506 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1511 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    331 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    548 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    548 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    533 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    448 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    447 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    983 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    886 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    96 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1598 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    984 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1235 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1450 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    565 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    522 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    534 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1333 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1333 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1449 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1595 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    971 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    418 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    879 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    948 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1595 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    363 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    882 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1595 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    462 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1031 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    570 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    570 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    570 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    40 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    388 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    384 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    882 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1029 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    529 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    492 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    489 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1331 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1331 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    949 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    431 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    565 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    477 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    212 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    970 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1041 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    487 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    378 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1031 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    461 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1149 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1657 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    535 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1657 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    437 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    902 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    364 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    412 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    329 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1029 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    519 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1659 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    399 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    893 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    515 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    498 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    944 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1024 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    540 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    903 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    943 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    392 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    406 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    298 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1149 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1658 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1658 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    293 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    514 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1505 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    434 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    539 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1506 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    372 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    517 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    367 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    516 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1574 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    481 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    363 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    433 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1627 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    911 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    120 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    498 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    438 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    336 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    449 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    449 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    449 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1368 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    81 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    81 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1570 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1340 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    386 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    463 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1126 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1552 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    333 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    463 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    335 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    161 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    387 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    428 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    448 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    447 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    445 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1554 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    532 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    922 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    923 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    498 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    487 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    552 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    969 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    66 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    66 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    67 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    67 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    334 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1336 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    446 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    574 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    936 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    936 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    126 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    141 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    533 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    429 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    472 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    346 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    457 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    343 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    403 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    498 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1550 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    324 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    487 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    969 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    546 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    553 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    351 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    6 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    944 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    410 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    409 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    943 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    450 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    545 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1180 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    479 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    969 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    430 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    70 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    70 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1126 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    166 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    850 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    851 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    852 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    419 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    73 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    73 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    358 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1180 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    305 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    494 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    488 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    564 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    534 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1602 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    565 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    485 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    353 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    356 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1341 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    59 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    59 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    969 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    419 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    74 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    74 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    75 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    75 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    231 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1366 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    76 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    76 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    420 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    184 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    97 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    520 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    474 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    478 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    411 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    443 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    411 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    78 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    78 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    62 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    62 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1031 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    418 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    63 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    63 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    411 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    9 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    80 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    80 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    486 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    325 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    342 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    473 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    565 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    170 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    170 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    171 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    171 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    87 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    87 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    328 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    65 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    65 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    542 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    83 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    83 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    541 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    341 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    84 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    84 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    168 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    168 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    148 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    148 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1380 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1021 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    424 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    553 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    162 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    162 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    528 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    889 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    371 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1379 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    149 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    149 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    370 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    565 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    480 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1092 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1031 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    542 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    482 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    466 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    543 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1616 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    88 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    88 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    85 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    85 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    475 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    901 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    900 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    889 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    908 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    909 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    347 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    504 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    361 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    165 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1088 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    974 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1153 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1092 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    986 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    987 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    942 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    939 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    565 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    872 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    159 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    941 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1557 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1031 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    542 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    337 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1530 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1529 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    541 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    499 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1589 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    490 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    348 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1376 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    352 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    355 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    357 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    359 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1324 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    366 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    563 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    376 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1389 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1434 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1362 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    280 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1374 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    167 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1439 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1280 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1609 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1359 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    314 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1421 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1416 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1409 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1426 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    339 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    484 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    340 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    509 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1371 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1612 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1372 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1425 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    404 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1031 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1601 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    454 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    345 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    369 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1502 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1501 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    396 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    396 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    395 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    393 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1370 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1369 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    859 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1549 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    186 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    235 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1547 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    327 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    422 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    96 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    180 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    326 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    468 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    469 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    502 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1430 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    491 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    199 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    402 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    476 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    405 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1496 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    497 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    253 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1422 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1428 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1427 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    203 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1507 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    500 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    565 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    354 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    407 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    92 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    95 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    175 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    518 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1327 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1326 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1325 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    436 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1518 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    350 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    893 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1626 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    440 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    441 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1625 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    408 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    527 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    444 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    368 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1594 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1624 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1394 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    374 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1393 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    553 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    385 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1579 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1483 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    537 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1350 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    555 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    42 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    579 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1349 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    383 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    390 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    389 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1031 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    172 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    458 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    415 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    416 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    185 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1534 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1535 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1533 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1532 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    89 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    225 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    846 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    174 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    565 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    453 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1100 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    330 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    176 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    925 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    338 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    425 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    178 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    427 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    98 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    100 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    179 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    249 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    163 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    250 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    349 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    496 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    454 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    200 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    861 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    396 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    396 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    395 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    393 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    501 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    859 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    370 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    554 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    505 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    396 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    396 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    395 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    393 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    859 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    327 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    26 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1031 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    326 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    362 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    510 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    891 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    365 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    327 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    513 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    904 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    164 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    181 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    27 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    326 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    521 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1185 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1069 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    230 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    284 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1171 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    28 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    978 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    972 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    29 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    41 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1175 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1179 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    175 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    30 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    31 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1178 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    32 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1160 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    549 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    33 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    175 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1083 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1068 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1072 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    561 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1073 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1033 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    873 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    37 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    935 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1617 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    153 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    394 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    458 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    458 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];
static mut yy_lookahead: [::core::ffi::c_ushort; 2394] = [
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    225 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    235 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    225 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    298 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    31 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    39 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    87 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    89 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    199 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    199 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    299 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    70 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    59 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    88 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    93 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    81 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    74 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    162 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    120 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    74 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    73 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    73 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    29 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    33 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    66 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    120 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    26 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    93 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    31 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    39 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    166 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    167 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    200 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    212 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    253 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    284 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    305 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    230 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    67 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    88 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    75 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    141 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    203 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    93 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    100 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    96 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    120 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    126 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    159 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    93 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    161 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    212 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    160 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    78 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    80 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    27 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    42 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    74 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    162 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    160 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    67 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    141 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    165 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    95 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    27 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    9 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    42 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    199 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    78 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    80 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    302 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    61 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    153 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    62 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    153 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    98 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    84 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    85 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    61 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    231 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    30 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    38 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    32 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    40 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    159 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    63 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    65 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    293 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    166 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    92 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    83 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    149 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    159 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    148 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    280 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    141 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    250 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    249 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    6 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    163 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    164 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    62 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    37 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    162 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    76 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    41 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    126 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    141 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    30 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    32 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    40 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    30 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    32 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    97 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    28 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    37 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    40 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    98 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    89 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    76 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    76 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    87 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    94 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    163 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    163 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];
unsafe extern "C" fn tokenExpr(
    mut pParse: *mut Parse,
    mut op: ::core::ffi::c_int,
    mut t: Token,
) -> *mut Expr {
    let mut p: *mut Expr = sqlite3DbMallocRawNN(
        (*pParse).db,
        (::core::mem::size_of::<Expr>() as usize)
            .wrapping_add(t.n as usize)
            .wrapping_add(1 as usize) as u64_0,
    ) as *mut Expr;
    if !p.is_null() {
        (*p).op = op as u8_0;
        (*p).affExpr = 0 as ::core::ffi::c_char;
        (*p).flags = EP_Leaf as u32_0;
        (*p).pRight = ::core::ptr::null_mut::<Expr>();
        (*p).pLeft = (*p).pRight;
        (*p).pAggInfo = ::core::ptr::null_mut::<AggInfo>();
        memset(
            &raw mut (*p).x as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<C2RustUnnamed_7>() as size_t,
        );
        memset(
            &raw mut (*p).y as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<C2RustUnnamed_0>() as size_t,
        );
        (*p).op2 = 0 as u8_0;
        (*p).iTable = 0 as ::core::ffi::c_int;
        (*p).iColumn = 0 as ynVar;
        (*p).u.zToken =
            p.offset(1 as ::core::ffi::c_int as isize) as *mut Expr as *mut ::core::ffi::c_char;
        memcpy(
            (*p).u.zToken as *mut ::core::ffi::c_void,
            t.z as *const ::core::ffi::c_void,
            t.n as size_t,
        );
        *(*p).u.zToken.offset(t.n as isize) = 0 as ::core::ffi::c_char;
        (*p).w.iOfst =
            t.z.offset_from((*pParse).zTail) as ::core::ffi::c_long as ::core::ffi::c_int;
        if *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar).offset(
            *(*p).u.zToken.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                as isize,
        ) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
        {
            sqlite3DequoteExpr(p);
        }
        (*p).nHeight = 1 as ::core::ffi::c_int;
        if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
            return sqlite3RenameTokenMap(pParse, p as *mut ::core::ffi::c_void, &raw mut t)
                as *mut Expr;
        }
    }
    return p;
}
static mut yy_shift_ofst: [::core::ffi::c_ushort; 583] = [
    2029 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1801 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2043 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1380 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1380 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1496 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1642 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    740 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    348 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    736 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    736 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    736 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    736 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    40 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    340 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    445 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    489 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    593 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    637 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    741 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    785 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    889 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    909 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1023 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1043 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    554 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1802 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1910 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    549 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    542 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1017 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1017 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1001 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    350 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    586 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    465 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1096 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1096 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    825 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    549 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    847 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    904 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    488 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    947 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    947 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    967 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    967 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    617 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    765 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    765 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    697 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    444 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    708 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    660 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    745 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    510 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    663 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    864 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    839 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    839 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    839 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1353 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    494 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    775 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1375 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1341 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1341 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1487 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1487 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    777 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1011 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    584 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    584 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    584 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1471 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1471 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    880 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1530 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1095 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1731 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1731 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1794 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1794 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1683 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1685 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1815 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1796 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1824 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1824 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1824 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1824 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1828 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1701 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1685 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1685 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1701 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1815 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1796 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1701 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1796 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1701 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1828 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1697 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1800 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1828 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1848 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1828 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1828 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1848 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1766 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1766 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1766 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1823 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1870 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1870 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1848 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1766 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1767 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1766 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1823 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1766 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1766 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1727 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1872 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1783 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1783 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1848 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1813 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1813 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1825 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1825 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1777 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1781 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1906 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1774 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1777 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1789 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1792 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1701 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1919 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1935 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1935 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1949 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1949 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1949 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    357 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1525 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    835 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    332 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1540 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1437 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1539 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1536 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1548 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1583 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1620 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1633 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1670 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1671 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1674 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1553 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1682 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1506 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1675 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1358 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1607 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1589 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1678 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1681 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1624 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1687 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1688 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1561 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1693 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1696 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1623 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1521 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1976 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1980 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1962 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1822 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1972 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1973 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1965 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1967 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1851 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1840 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1862 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1969 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1969 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1971 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1853 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1977 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1982 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1999 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1858 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1871 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1969 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1873 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1941 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1968 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1969 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1855 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1952 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1954 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1955 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1956 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1881 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1896 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1981 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1874 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2013 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2014 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1998 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1905 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1860 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1957 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2008 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1966 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1947 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1983 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1894 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1921 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2020 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2018 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2026 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1915 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1923 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1984 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2036 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2040 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2047 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2041 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2003 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2012 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2050 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1979 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2049 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2056 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2011 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2048 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1934 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2064 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2065 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2061 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2066 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2068 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1993 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1950 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2071 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2072 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1985 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2062 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2075 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1959 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2073 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2076 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2078 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2010 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2022 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2069 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2031 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2021 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2082 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2094 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2083 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2095 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2093 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2096 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2086 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1986 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1987 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2100 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2073 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2009 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2004 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2005 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2006 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];
static mut yy_reduce_ofst: [::core::ffi::c_short; 413] = [
    -(67 as ::core::ffi::c_int) as ::core::ffi::c_short,
    1252 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(64 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(178 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(181 as ::core::ffi::c_int) as ::core::ffi::c_short,
    160 as ::core::ffi::c_int as ::core::ffi::c_short,
    1071 as ::core::ffi::c_int as ::core::ffi::c_short,
    143 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(184 as ::core::ffi::c_int) as ::core::ffi::c_short,
    137 as ::core::ffi::c_int as ::core::ffi::c_short,
    218 as ::core::ffi::c_int as ::core::ffi::c_short,
    220 as ::core::ffi::c_int as ::core::ffi::c_short,
    222 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(174 as ::core::ffi::c_int) as ::core::ffi::c_short,
    229 as ::core::ffi::c_int as ::core::ffi::c_short,
    268 as ::core::ffi::c_int as ::core::ffi::c_short,
    272 as ::core::ffi::c_int as ::core::ffi::c_short,
    275 as ::core::ffi::c_int as ::core::ffi::c_short,
    324 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(208 as ::core::ffi::c_int) as ::core::ffi::c_short,
    242 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(277 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(39 as ::core::ffi::c_int) as ::core::ffi::c_short,
    81 as ::core::ffi::c_int as ::core::ffi::c_short,
    537 as ::core::ffi::c_int as ::core::ffi::c_short,
    792 as ::core::ffi::c_int as ::core::ffi::c_short,
    810 as ::core::ffi::c_int as ::core::ffi::c_short,
    812 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(189 as ::core::ffi::c_int) as ::core::ffi::c_short,
    814 as ::core::ffi::c_int as ::core::ffi::c_short,
    831 as ::core::ffi::c_int as ::core::ffi::c_short,
    163 as ::core::ffi::c_int as ::core::ffi::c_short,
    865 as ::core::ffi::c_int as ::core::ffi::c_short,
    944 as ::core::ffi::c_int as ::core::ffi::c_short,
    887 as ::core::ffi::c_int as ::core::ffi::c_short,
    840 as ::core::ffi::c_int as ::core::ffi::c_short,
    964 as ::core::ffi::c_int as ::core::ffi::c_short,
    1077 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(187 as ::core::ffi::c_int) as ::core::ffi::c_short,
    292 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(133 as ::core::ffi::c_int) as ::core::ffi::c_short,
    274 as ::core::ffi::c_int as ::core::ffi::c_short,
    673 as ::core::ffi::c_int as ::core::ffi::c_short,
    558 as ::core::ffi::c_int as ::core::ffi::c_short,
    682 as ::core::ffi::c_int as ::core::ffi::c_short,
    795 as ::core::ffi::c_int as ::core::ffi::c_short,
    809 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(238 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(232 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(238 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(232 as ::core::ffi::c_int) as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    557 as ::core::ffi::c_int as ::core::ffi::c_short,
    712 as ::core::ffi::c_int as ::core::ffi::c_short,
    949 as ::core::ffi::c_int as ::core::ffi::c_short,
    966 as ::core::ffi::c_int as ::core::ffi::c_short,
    969 as ::core::ffi::c_int as ::core::ffi::c_short,
    971 as ::core::ffi::c_int as ::core::ffi::c_short,
    979 as ::core::ffi::c_int as ::core::ffi::c_short,
    1097 as ::core::ffi::c_int as ::core::ffi::c_short,
    1099 as ::core::ffi::c_int as ::core::ffi::c_short,
    1103 as ::core::ffi::c_int as ::core::ffi::c_short,
    1142 as ::core::ffi::c_int as ::core::ffi::c_short,
    1144 as ::core::ffi::c_int as ::core::ffi::c_short,
    1169 as ::core::ffi::c_int as ::core::ffi::c_short,
    1172 as ::core::ffi::c_int as ::core::ffi::c_short,
    1201 as ::core::ffi::c_int as ::core::ffi::c_short,
    1203 as ::core::ffi::c_int as ::core::ffi::c_short,
    1228 as ::core::ffi::c_int as ::core::ffi::c_short,
    1241 as ::core::ffi::c_int as ::core::ffi::c_short,
    1250 as ::core::ffi::c_int as ::core::ffi::c_short,
    1253 as ::core::ffi::c_int as ::core::ffi::c_short,
    1255 as ::core::ffi::c_int as ::core::ffi::c_short,
    1261 as ::core::ffi::c_int as ::core::ffi::c_short,
    1266 as ::core::ffi::c_int as ::core::ffi::c_short,
    1271 as ::core::ffi::c_int as ::core::ffi::c_short,
    1282 as ::core::ffi::c_int as ::core::ffi::c_short,
    1291 as ::core::ffi::c_int as ::core::ffi::c_short,
    1308 as ::core::ffi::c_int as ::core::ffi::c_short,
    1310 as ::core::ffi::c_int as ::core::ffi::c_short,
    1312 as ::core::ffi::c_int as ::core::ffi::c_short,
    1322 as ::core::ffi::c_int as ::core::ffi::c_short,
    1328 as ::core::ffi::c_int as ::core::ffi::c_short,
    1347 as ::core::ffi::c_int as ::core::ffi::c_short,
    1354 as ::core::ffi::c_int as ::core::ffi::c_short,
    1356 as ::core::ffi::c_int as ::core::ffi::c_short,
    1359 as ::core::ffi::c_int as ::core::ffi::c_short,
    1362 as ::core::ffi::c_int as ::core::ffi::c_short,
    1365 as ::core::ffi::c_int as ::core::ffi::c_short,
    1367 as ::core::ffi::c_int as ::core::ffi::c_short,
    1374 as ::core::ffi::c_int as ::core::ffi::c_short,
    1376 as ::core::ffi::c_int as ::core::ffi::c_short,
    1381 as ::core::ffi::c_int as ::core::ffi::c_short,
    1401 as ::core::ffi::c_int as ::core::ffi::c_short,
    1403 as ::core::ffi::c_int as ::core::ffi::c_short,
    1406 as ::core::ffi::c_int as ::core::ffi::c_short,
    1412 as ::core::ffi::c_int as ::core::ffi::c_short,
    1414 as ::core::ffi::c_int as ::core::ffi::c_short,
    1417 as ::core::ffi::c_int as ::core::ffi::c_short,
    1421 as ::core::ffi::c_int as ::core::ffi::c_short,
    1428 as ::core::ffi::c_int as ::core::ffi::c_short,
    1447 as ::core::ffi::c_int as ::core::ffi::c_short,
    1449 as ::core::ffi::c_int as ::core::ffi::c_short,
    1453 as ::core::ffi::c_int as ::core::ffi::c_short,
    1462 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(22 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(159 as ::core::ffi::c_int) as ::core::ffi::c_short,
    475 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(220 as ::core::ffi::c_int) as ::core::ffi::c_short,
    756 as ::core::ffi::c_int as ::core::ffi::c_short,
    38 as ::core::ffi::c_int as ::core::ffi::c_short,
    501 as ::core::ffi::c_int as ::core::ffi::c_short,
    841 as ::core::ffi::c_int as ::core::ffi::c_short,
    714 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    118 as ::core::ffi::c_int as ::core::ffi::c_short,
    337 as ::core::ffi::c_int as ::core::ffi::c_short,
    349 as ::core::ffi::c_int as ::core::ffi::c_short,
    363 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(56 as ::core::ffi::c_int) as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(205 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(205 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(205 as ::core::ffi::c_int) as ::core::ffi::c_short,
    687 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(172 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(130 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(57 as ::core::ffi::c_int) as ::core::ffi::c_short,
    790 as ::core::ffi::c_int as ::core::ffi::c_short,
    397 as ::core::ffi::c_int as ::core::ffi::c_short,
    528 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(271 as ::core::ffi::c_int) as ::core::ffi::c_short,
    136 as ::core::ffi::c_int as ::core::ffi::c_short,
    596 as ::core::ffi::c_int as ::core::ffi::c_short,
    596 as ::core::ffi::c_int as ::core::ffi::c_short,
    90 as ::core::ffi::c_int as ::core::ffi::c_short,
    316 as ::core::ffi::c_int as ::core::ffi::c_short,
    522 as ::core::ffi::c_int as ::core::ffi::c_short,
    541 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(37 as ::core::ffi::c_int) as ::core::ffi::c_short,
    715 as ::core::ffi::c_int as ::core::ffi::c_short,
    849 as ::core::ffi::c_int as ::core::ffi::c_short,
    977 as ::core::ffi::c_int as ::core::ffi::c_short,
    628 as ::core::ffi::c_int as ::core::ffi::c_short,
    856 as ::core::ffi::c_int as ::core::ffi::c_short,
    980 as ::core::ffi::c_int as ::core::ffi::c_short,
    991 as ::core::ffi::c_int as ::core::ffi::c_short,
    1081 as ::core::ffi::c_int as ::core::ffi::c_short,
    1102 as ::core::ffi::c_int as ::core::ffi::c_short,
    1135 as ::core::ffi::c_int as ::core::ffi::c_short,
    1083 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(162 as ::core::ffi::c_int) as ::core::ffi::c_short,
    208 as ::core::ffi::c_int as ::core::ffi::c_short,
    1258 as ::core::ffi::c_int as ::core::ffi::c_short,
    794 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(86 as ::core::ffi::c_int) as ::core::ffi::c_short,
    159 as ::core::ffi::c_int as ::core::ffi::c_short,
    41 as ::core::ffi::c_int as ::core::ffi::c_short,
    1109 as ::core::ffi::c_int as ::core::ffi::c_short,
    671 as ::core::ffi::c_int as ::core::ffi::c_short,
    852 as ::core::ffi::c_int as ::core::ffi::c_short,
    844 as ::core::ffi::c_int as ::core::ffi::c_short,
    932 as ::core::ffi::c_int as ::core::ffi::c_short,
    1175 as ::core::ffi::c_int as ::core::ffi::c_short,
    1254 as ::core::ffi::c_int as ::core::ffi::c_short,
    480 as ::core::ffi::c_int as ::core::ffi::c_short,
    1180 as ::core::ffi::c_int as ::core::ffi::c_short,
    100 as ::core::ffi::c_int as ::core::ffi::c_short,
    258 as ::core::ffi::c_int as ::core::ffi::c_short,
    1265 as ::core::ffi::c_int as ::core::ffi::c_short,
    1268 as ::core::ffi::c_int as ::core::ffi::c_short,
    1216 as ::core::ffi::c_int as ::core::ffi::c_short,
    1287 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(139 as ::core::ffi::c_int) as ::core::ffi::c_short,
    317 as ::core::ffi::c_int as ::core::ffi::c_short,
    344 as ::core::ffi::c_int as ::core::ffi::c_short,
    63 as ::core::ffi::c_int as ::core::ffi::c_short,
    339 as ::core::ffi::c_int as ::core::ffi::c_short,
    423 as ::core::ffi::c_int as ::core::ffi::c_short,
    563 as ::core::ffi::c_int as ::core::ffi::c_short,
    636 as ::core::ffi::c_int as ::core::ffi::c_short,
    676 as ::core::ffi::c_int as ::core::ffi::c_short,
    813 as ::core::ffi::c_int as ::core::ffi::c_short,
    908 as ::core::ffi::c_int as ::core::ffi::c_short,
    914 as ::core::ffi::c_int as ::core::ffi::c_short,
    950 as ::core::ffi::c_int as ::core::ffi::c_short,
    1078 as ::core::ffi::c_int as ::core::ffi::c_short,
    1084 as ::core::ffi::c_int as ::core::ffi::c_short,
    1098 as ::core::ffi::c_int as ::core::ffi::c_short,
    1363 as ::core::ffi::c_int as ::core::ffi::c_short,
    1384 as ::core::ffi::c_int as ::core::ffi::c_short,
    1407 as ::core::ffi::c_int as ::core::ffi::c_short,
    1439 as ::core::ffi::c_int as ::core::ffi::c_short,
    1464 as ::core::ffi::c_int as ::core::ffi::c_short,
    411 as ::core::ffi::c_int as ::core::ffi::c_short,
    1527 as ::core::ffi::c_int as ::core::ffi::c_short,
    1534 as ::core::ffi::c_int as ::core::ffi::c_short,
    1535 as ::core::ffi::c_int as ::core::ffi::c_short,
    1537 as ::core::ffi::c_int as ::core::ffi::c_short,
    1541 as ::core::ffi::c_int as ::core::ffi::c_short,
    1542 as ::core::ffi::c_int as ::core::ffi::c_short,
    1543 as ::core::ffi::c_int as ::core::ffi::c_short,
    1544 as ::core::ffi::c_int as ::core::ffi::c_short,
    1545 as ::core::ffi::c_int as ::core::ffi::c_short,
    1547 as ::core::ffi::c_int as ::core::ffi::c_short,
    1549 as ::core::ffi::c_int as ::core::ffi::c_short,
    1550 as ::core::ffi::c_int as ::core::ffi::c_short,
    990 as ::core::ffi::c_int as ::core::ffi::c_short,
    1164 as ::core::ffi::c_int as ::core::ffi::c_short,
    1492 as ::core::ffi::c_int as ::core::ffi::c_short,
    1551 as ::core::ffi::c_int as ::core::ffi::c_short,
    1552 as ::core::ffi::c_int as ::core::ffi::c_short,
    1556 as ::core::ffi::c_int as ::core::ffi::c_short,
    1217 as ::core::ffi::c_int as ::core::ffi::c_short,
    1558 as ::core::ffi::c_int as ::core::ffi::c_short,
    1559 as ::core::ffi::c_int as ::core::ffi::c_short,
    1560 as ::core::ffi::c_int as ::core::ffi::c_short,
    1473 as ::core::ffi::c_int as ::core::ffi::c_short,
    1413 as ::core::ffi::c_int as ::core::ffi::c_short,
    1563 as ::core::ffi::c_int as ::core::ffi::c_short,
    1510 as ::core::ffi::c_int as ::core::ffi::c_short,
    1568 as ::core::ffi::c_int as ::core::ffi::c_short,
    563 as ::core::ffi::c_int as ::core::ffi::c_short,
    1570 as ::core::ffi::c_int as ::core::ffi::c_short,
    1571 as ::core::ffi::c_int as ::core::ffi::c_short,
    1572 as ::core::ffi::c_int as ::core::ffi::c_short,
    1573 as ::core::ffi::c_int as ::core::ffi::c_short,
    1574 as ::core::ffi::c_int as ::core::ffi::c_short,
    1575 as ::core::ffi::c_int as ::core::ffi::c_short,
    1443 as ::core::ffi::c_int as ::core::ffi::c_short,
    1466 as ::core::ffi::c_int as ::core::ffi::c_short,
    1518 as ::core::ffi::c_int as ::core::ffi::c_short,
    1513 as ::core::ffi::c_int as ::core::ffi::c_short,
    1514 as ::core::ffi::c_int as ::core::ffi::c_short,
    1515 as ::core::ffi::c_int as ::core::ffi::c_short,
    1516 as ::core::ffi::c_int as ::core::ffi::c_short,
    1217 as ::core::ffi::c_int as ::core::ffi::c_short,
    1518 as ::core::ffi::c_int as ::core::ffi::c_short,
    1518 as ::core::ffi::c_int as ::core::ffi::c_short,
    1531 as ::core::ffi::c_int as ::core::ffi::c_short,
    1562 as ::core::ffi::c_int as ::core::ffi::c_short,
    1582 as ::core::ffi::c_int as ::core::ffi::c_short,
    1477 as ::core::ffi::c_int as ::core::ffi::c_short,
    1505 as ::core::ffi::c_int as ::core::ffi::c_short,
    1511 as ::core::ffi::c_int as ::core::ffi::c_short,
    1533 as ::core::ffi::c_int as ::core::ffi::c_short,
    1512 as ::core::ffi::c_int as ::core::ffi::c_short,
    1488 as ::core::ffi::c_int as ::core::ffi::c_short,
    1538 as ::core::ffi::c_int as ::core::ffi::c_short,
    1509 as ::core::ffi::c_int as ::core::ffi::c_short,
    1517 as ::core::ffi::c_int as ::core::ffi::c_short,
    1546 as ::core::ffi::c_int as ::core::ffi::c_short,
    1519 as ::core::ffi::c_int as ::core::ffi::c_short,
    1557 as ::core::ffi::c_int as ::core::ffi::c_short,
    1489 as ::core::ffi::c_int as ::core::ffi::c_short,
    1565 as ::core::ffi::c_int as ::core::ffi::c_short,
    1564 as ::core::ffi::c_int as ::core::ffi::c_short,
    1578 as ::core::ffi::c_int as ::core::ffi::c_short,
    1586 as ::core::ffi::c_int as ::core::ffi::c_short,
    1587 as ::core::ffi::c_int as ::core::ffi::c_short,
    1588 as ::core::ffi::c_int as ::core::ffi::c_short,
    1526 as ::core::ffi::c_int as ::core::ffi::c_short,
    1528 as ::core::ffi::c_int as ::core::ffi::c_short,
    1554 as ::core::ffi::c_int as ::core::ffi::c_short,
    1555 as ::core::ffi::c_int as ::core::ffi::c_short,
    1576 as ::core::ffi::c_int as ::core::ffi::c_short,
    1577 as ::core::ffi::c_int as ::core::ffi::c_short,
    1566 as ::core::ffi::c_int as ::core::ffi::c_short,
    1579 as ::core::ffi::c_int as ::core::ffi::c_short,
    1584 as ::core::ffi::c_int as ::core::ffi::c_short,
    1591 as ::core::ffi::c_int as ::core::ffi::c_short,
    1520 as ::core::ffi::c_int as ::core::ffi::c_short,
    1523 as ::core::ffi::c_int as ::core::ffi::c_short,
    1617 as ::core::ffi::c_int as ::core::ffi::c_short,
    1628 as ::core::ffi::c_int as ::core::ffi::c_short,
    1580 as ::core::ffi::c_int as ::core::ffi::c_short,
    1581 as ::core::ffi::c_int as ::core::ffi::c_short,
    1632 as ::core::ffi::c_int as ::core::ffi::c_short,
    1585 as ::core::ffi::c_int as ::core::ffi::c_short,
    1590 as ::core::ffi::c_int as ::core::ffi::c_short,
    1593 as ::core::ffi::c_int as ::core::ffi::c_short,
    1604 as ::core::ffi::c_int as ::core::ffi::c_short,
    1605 as ::core::ffi::c_int as ::core::ffi::c_short,
    1606 as ::core::ffi::c_int as ::core::ffi::c_short,
    1608 as ::core::ffi::c_int as ::core::ffi::c_short,
    1609 as ::core::ffi::c_int as ::core::ffi::c_short,
    1641 as ::core::ffi::c_int as ::core::ffi::c_short,
    1649 as ::core::ffi::c_int as ::core::ffi::c_short,
    1610 as ::core::ffi::c_int as ::core::ffi::c_short,
    1592 as ::core::ffi::c_int as ::core::ffi::c_short,
    1594 as ::core::ffi::c_int as ::core::ffi::c_short,
    1611 as ::core::ffi::c_int as ::core::ffi::c_short,
    1595 as ::core::ffi::c_int as ::core::ffi::c_short,
    1616 as ::core::ffi::c_int as ::core::ffi::c_short,
    1612 as ::core::ffi::c_int as ::core::ffi::c_short,
    1618 as ::core::ffi::c_int as ::core::ffi::c_short,
    1613 as ::core::ffi::c_int as ::core::ffi::c_short,
    1651 as ::core::ffi::c_int as ::core::ffi::c_short,
    1654 as ::core::ffi::c_int as ::core::ffi::c_short,
    1596 as ::core::ffi::c_int as ::core::ffi::c_short,
    1598 as ::core::ffi::c_int as ::core::ffi::c_short,
    1655 as ::core::ffi::c_int as ::core::ffi::c_short,
    1663 as ::core::ffi::c_int as ::core::ffi::c_short,
    1650 as ::core::ffi::c_int as ::core::ffi::c_short,
    1673 as ::core::ffi::c_int as ::core::ffi::c_short,
    1680 as ::core::ffi::c_int as ::core::ffi::c_short,
    1677 as ::core::ffi::c_int as ::core::ffi::c_short,
    1684 as ::core::ffi::c_int as ::core::ffi::c_short,
    1653 as ::core::ffi::c_int as ::core::ffi::c_short,
    1664 as ::core::ffi::c_int as ::core::ffi::c_short,
    1666 as ::core::ffi::c_int as ::core::ffi::c_short,
    1667 as ::core::ffi::c_int as ::core::ffi::c_short,
    1662 as ::core::ffi::c_int as ::core::ffi::c_short,
    1669 as ::core::ffi::c_int as ::core::ffi::c_short,
    1672 as ::core::ffi::c_int as ::core::ffi::c_short,
    1676 as ::core::ffi::c_int as ::core::ffi::c_short,
    1686 as ::core::ffi::c_int as ::core::ffi::c_short,
    1679 as ::core::ffi::c_int as ::core::ffi::c_short,
    1691 as ::core::ffi::c_int as ::core::ffi::c_short,
    1689 as ::core::ffi::c_int as ::core::ffi::c_short,
    1692 as ::core::ffi::c_int as ::core::ffi::c_short,
    1694 as ::core::ffi::c_int as ::core::ffi::c_short,
    1597 as ::core::ffi::c_int as ::core::ffi::c_short,
    1599 as ::core::ffi::c_int as ::core::ffi::c_short,
    1619 as ::core::ffi::c_int as ::core::ffi::c_short,
    1630 as ::core::ffi::c_int as ::core::ffi::c_short,
    1699 as ::core::ffi::c_int as ::core::ffi::c_short,
    1700 as ::core::ffi::c_int as ::core::ffi::c_short,
    1602 as ::core::ffi::c_int as ::core::ffi::c_short,
    1615 as ::core::ffi::c_int as ::core::ffi::c_short,
    1648 as ::core::ffi::c_int as ::core::ffi::c_short,
    1657 as ::core::ffi::c_int as ::core::ffi::c_short,
    1690 as ::core::ffi::c_int as ::core::ffi::c_short,
    1698 as ::core::ffi::c_int as ::core::ffi::c_short,
    1658 as ::core::ffi::c_int as ::core::ffi::c_short,
    1729 as ::core::ffi::c_int as ::core::ffi::c_short,
    1652 as ::core::ffi::c_int as ::core::ffi::c_short,
    1695 as ::core::ffi::c_int as ::core::ffi::c_short,
    1702 as ::core::ffi::c_int as ::core::ffi::c_short,
    1704 as ::core::ffi::c_int as ::core::ffi::c_short,
    1703 as ::core::ffi::c_int as ::core::ffi::c_short,
    1741 as ::core::ffi::c_int as ::core::ffi::c_short,
    1754 as ::core::ffi::c_int as ::core::ffi::c_short,
    1758 as ::core::ffi::c_int as ::core::ffi::c_short,
    1768 as ::core::ffi::c_int as ::core::ffi::c_short,
    1769 as ::core::ffi::c_int as ::core::ffi::c_short,
    1771 as ::core::ffi::c_int as ::core::ffi::c_short,
    1660 as ::core::ffi::c_int as ::core::ffi::c_short,
    1661 as ::core::ffi::c_int as ::core::ffi::c_short,
    1665 as ::core::ffi::c_int as ::core::ffi::c_short,
    1752 as ::core::ffi::c_int as ::core::ffi::c_short,
    1756 as ::core::ffi::c_int as ::core::ffi::c_short,
    1757 as ::core::ffi::c_int as ::core::ffi::c_short,
    1759 as ::core::ffi::c_int as ::core::ffi::c_short,
    1760 as ::core::ffi::c_int as ::core::ffi::c_short,
    1764 as ::core::ffi::c_int as ::core::ffi::c_short,
    1745 as ::core::ffi::c_int as ::core::ffi::c_short,
    1753 as ::core::ffi::c_int as ::core::ffi::c_short,
    1762 as ::core::ffi::c_int as ::core::ffi::c_short,
    1763 as ::core::ffi::c_int as ::core::ffi::c_short,
    1761 as ::core::ffi::c_int as ::core::ffi::c_short,
    1772 as ::core::ffi::c_int as ::core::ffi::c_short,
];
static mut yy_default: [::core::ffi::c_ushort; 583] = [
    1663 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1663 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1663 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1491 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1367 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1491 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1491 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1491 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1544 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1490 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1563 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1406 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1492 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1493 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1543 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1545 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1508 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1420 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1419 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1418 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1526 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1385 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1411 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1404 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1408 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1487 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1488 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1486 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1641 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1493 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1492 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1407 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1471 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1454 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1463 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1470 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1469 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1468 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1477 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1467 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1457 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1458 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1459 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1329 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1460 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1448 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1447 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1446 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1474 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1461 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1473 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1472 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1551 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1615 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1614 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1509 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1387 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1388 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1388 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1558 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1358 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1358 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1358 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1358 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1367 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1358 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1548 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1546 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1363 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1608 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1521 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1343 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1363 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1363 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1363 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1363 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1365 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1344 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1342 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1357 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1655 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1412 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1364 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1412 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1652 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1410 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1410 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1364 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1652 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1630 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1299 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1387 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1387 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1387 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1387 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1489 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1364 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1357 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1655 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1655 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1654 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1654 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1509 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1638 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1432 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1332 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1338 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1338 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1338 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1338 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1410 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1638 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1638 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1410 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1432 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1332 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1410 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1332 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1410 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1525 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1649 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1499 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1499 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1330 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1330 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1330 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1499 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1330 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1330 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1330 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1330 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1596 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1499 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1588 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1588 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1405 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1494 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1405 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1403 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1401 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1410 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1611 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1611 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1607 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1607 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1607 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1660 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1660 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1558 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1623 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1623 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1623 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1618 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1553 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1510 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1564 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1437 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1555 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1415 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1378 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1429 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1424 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1651 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1524 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1523 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1375 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1302 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1402 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1593 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1392 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1642 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1352 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1634 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1346 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1438 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1441 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];
static mut yyFallback: [::core::ffi::c_ushort; 187] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];
unsafe extern "C" fn binaryToUnaryIfNull(
    mut pParse: *mut Parse,
    mut pY: *mut Expr,
    mut pA: *mut Expr,
    mut op: ::core::ffi::c_int,
) {
    let mut db: *mut sqlite3 = (*pParse).db;
    if !pA.is_null()
        && !pY.is_null()
        && (*pY).op as ::core::ffi::c_int == TK_NULL
        && !((*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME)
    {
        (*pA).op = op as u8_0;
        sqlite3ExprDelete(db, (*pA).pRight);
        (*pA).pRight = ::core::ptr::null_mut::<Expr>();
    }
}
unsafe extern "C" fn parserAddExprIdListTerm(
    mut pParse: *mut Parse,
    mut pPrior: *mut ExprList,
    mut pIdToken: *mut Token,
    mut hasCollate: ::core::ffi::c_int,
    mut sortOrder: ::core::ffi::c_int,
) -> *mut ExprList {
    let mut p: *mut ExprList =
        sqlite3ExprListAppend(pParse, pPrior, ::core::ptr::null_mut::<Expr>());
    if (hasCollate != 0 || sortOrder != SQLITE_SO_UNDEFINED)
        && (*(*pParse).db).init.busy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        sqlite3ErrorMsg(
            pParse,
            b"syntax error after column name \"%.*s\"\0" as *const u8 as *const ::core::ffi::c_char,
            (*pIdToken).n,
            (*pIdToken).z,
        );
    }
    sqlite3ExprListSetName(pParse, p, pIdToken, 1 as ::core::ffi::c_int);
    return p;
}
unsafe extern "C" fn yyGrowStack(mut p: *mut yyParser) -> ::core::ffi::c_int {
    let mut oldSize: ::core::ffi::c_int = 1 as ::core::ffi::c_int
        + (*p).yystackEnd.offset_from((*p).yystack) as ::core::ffi::c_long as ::core::ffi::c_int;
    let mut newSize: ::core::ffi::c_int = 0;
    let mut idx: ::core::ffi::c_int = 0;
    let mut pNew: *mut yyStackEntry = ::core::ptr::null_mut::<yyStackEntry>();
    newSize = oldSize * 2 as ::core::ffi::c_int + 100 as ::core::ffi::c_int;
    idx = (*p).yytos.offset_from((*p).yystack) as ::core::ffi::c_long as ::core::ffi::c_int;
    if (*p).yystack == &raw mut (*p).yystk0 as *mut yyStackEntry {
        pNew = parserStackRealloc(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            (newSize as usize).wrapping_mul(::core::mem::size_of::<yyStackEntry>() as usize)
                as sqlite3_uint64,
        ) as *mut yyStackEntry;
        if pNew.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        memcpy(
            pNew as *mut ::core::ffi::c_void,
            (*p).yystack as *const ::core::ffi::c_void,
            (oldSize as size_t).wrapping_mul(::core::mem::size_of::<yyStackEntry>() as size_t),
        );
    } else {
        pNew = parserStackRealloc(
            (*p).yystack as *mut ::core::ffi::c_void,
            (newSize as usize).wrapping_mul(::core::mem::size_of::<yyStackEntry>() as usize)
                as sqlite3_uint64,
        ) as *mut yyStackEntry;
        if pNew.is_null() {
            return 1 as ::core::ffi::c_int;
        }
    }
    (*p).yystack = pNew;
    (*p).yytos = (*p).yystack.offset(idx as isize) as *mut yyStackEntry;
    (*p).yystackEnd =
        (*p).yystack
            .offset((newSize - 1 as ::core::ffi::c_int) as isize) as *mut yyStackEntry;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ParserInit(
    mut yypRawParser: *mut ::core::ffi::c_void,
    mut pParse: *mut Parse,
) {
    let mut yypParser: *mut yyParser = yypRawParser as *mut yyParser;
    (*yypParser).pParse = pParse;
    (*yypParser).yystack = &raw mut (*yypParser).yystk0 as *mut yyStackEntry;
    (*yypParser).yystackEnd = (*yypParser)
        .yystack
        .offset((YYSTACKDEPTH - 1 as ::core::ffi::c_int) as isize)
        as *mut yyStackEntry;
    (*yypParser).yytos = (*yypParser).yystack;
    (*(*yypParser)
        .yystack
        .offset(0 as ::core::ffi::c_int as isize))
    .stateno = 0 as ::core::ffi::c_ushort;
    (*(*yypParser)
        .yystack
        .offset(0 as ::core::ffi::c_int as isize))
    .major = 0 as ::core::ffi::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ParserAlloc(
    mut mallocProc: Option<unsafe extern "C" fn(u64_0) -> *mut ::core::ffi::c_void>,
    mut pParse: *mut Parse,
) -> *mut ::core::ffi::c_void {
    let mut yypParser: *mut yyParser = ::core::ptr::null_mut::<yyParser>();
    yypParser = Some(mallocProc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        ::core::mem::size_of::<yyParser>() as u64_0
    ) as *mut yyParser;
    if !yypParser.is_null() {
        (*yypParser).pParse = pParse;
        sqlite3ParserInit(yypParser as *mut ::core::ffi::c_void, pParse);
    }
    return yypParser as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn yy_destructor(
    mut yypParser: *mut yyParser,
    mut yymajor: ::core::ffi::c_ushort,
    mut yypminor: *mut YYMINORTYPE,
) {
    let mut pParse: *mut Parse = (*yypParser).pParse;
    match yymajor as ::core::ffi::c_int {
        206 | 241 | 242 | 254 | 256 => {
            sqlite3SelectDelete((*pParse).db, (*yypminor).yy637);
        }
        218 | 219 | 248 | 250 | 270 | 281 | 283 | 286 | 293 | 298 | 315 => {
            sqlite3ExprDelete((*pParse).db, (*yypminor).yy590);
        }
        223 | 233 | 234 | 246 | 249 | 251 | 255 | 257 | 264 | 271 | 280 | 282 | 314 => {
            sqlite3ExprListDelete((*pParse).db, (*yypminor).yy402);
        }
        240 | 247 | 259 | 260 | 265 => {
            sqlite3SrcListDelete((*pParse).db, (*yypminor).yy563);
        }
        243 => {
            sqlite3WithDelete((*pParse).db, (*yypminor).yy125);
        }
        253 | 310 => {
            sqlite3WindowListDelete((*pParse).db, (*yypminor).yy483);
        }
        266 | 273 => {
            sqlite3IdListDelete((*pParse).db, (*yypminor).yy204);
        }
        276 | 311 | 312 | 313 | 316 => {
            sqlite3WindowDelete((*pParse).db, (*yypminor).yy483);
        }
        289 | 294 => {
            sqlite3DeleteTriggerStep((*pParse).db, (*yypminor).yy319);
        }
        291 => {
            sqlite3IdListDelete((*pParse).db, (*yypminor).yy28.b);
        }
        318 | 319 | 320 => {
            sqlite3ExprDelete((*pParse).db, (*yypminor).yy205.pExpr);
        }
        _ => {}
    };
}
unsafe extern "C" fn yy_pop_parser_stack(mut pParser: *mut yyParser) {
    let mut yytos: *mut yyStackEntry = ::core::ptr::null_mut::<yyStackEntry>();
    let fresh0 = (*pParser).yytos;
    (*pParser).yytos = (*pParser).yytos.offset(-1);
    yytos = fresh0;
    yy_destructor(pParser, (*yytos).major, &raw mut (*yytos).minor);
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ParserFinalize(mut p: *mut ::core::ffi::c_void) {
    let mut pParser: *mut yyParser = p as *mut yyParser;
    let mut yytos: *mut yyStackEntry = (*pParser).yytos;
    while yytos > (*pParser).yystack {
        if (*yytos).major as ::core::ffi::c_int >= YY_MIN_DSTRCTR {
            yy_destructor(pParser, (*yytos).major, &raw mut (*yytos).minor);
        }
        yytos = yytos.offset(-1);
    }
    if (*pParser).yystack != &raw mut (*pParser).yystk0 as *mut yyStackEntry {
        sqlite3_free((*pParser).yystack as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ParserFree(
    mut p: *mut ::core::ffi::c_void,
    mut freeProc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    sqlite3ParserFinalize(p);
    Some(freeProc.expect("non-null function pointer")).expect("non-null function pointer")(p);
}
unsafe extern "C" fn yy_find_shift_action(
    mut iLookAhead: ::core::ffi::c_ushort,
    mut stateno: ::core::ffi::c_ushort,
) -> ::core::ffi::c_ushort {
    let mut i: ::core::ffi::c_int = 0;
    if stateno as ::core::ffi::c_int > YY_MAX_SHIFT {
        return stateno;
    }
    loop {
        i = yy_shift_ofst[stateno as usize] as ::core::ffi::c_int;
        i += iLookAhead as ::core::ffi::c_int;
        if yy_lookahead[i as usize] as ::core::ffi::c_int != iLookAhead as ::core::ffi::c_int {
            let mut iFallback: ::core::ffi::c_ushort = 0;
            iFallback = yyFallback[iLookAhead as usize];
            if iFallback as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                iLookAhead = iFallback;
            } else {
                let mut j: ::core::ffi::c_int = i - iLookAhead as ::core::ffi::c_int + YYWILDCARD;
                if yy_lookahead[j as usize] as ::core::ffi::c_int == YYWILDCARD
                    && iLookAhead as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                {
                    return yy_action[j as usize];
                }
                return yy_default[stateno as usize];
            }
        } else {
            return yy_action[i as usize];
        }
    }
}
unsafe extern "C" fn yy_find_reduce_action(
    mut stateno: ::core::ffi::c_ushort,
    mut iLookAhead: ::core::ffi::c_ushort,
) -> ::core::ffi::c_ushort {
    let mut i: ::core::ffi::c_int = 0;
    i = yy_reduce_ofst[stateno as usize] as ::core::ffi::c_int;
    i += iLookAhead as ::core::ffi::c_int;
    return yy_action[i as usize];
}
unsafe extern "C" fn yyStackOverflow(mut yypParser: *mut yyParser) {
    let mut pParse: *mut Parse = (*yypParser).pParse;
    while (*yypParser).yytos > (*yypParser).yystack {
        yy_pop_parser_stack(yypParser);
    }
    sqlite3OomFault((*pParse).db);
    (*yypParser).pParse = pParse;
}
unsafe extern "C" fn yy_shift(
    mut yypParser: *mut yyParser,
    mut yyNewState: ::core::ffi::c_ushort,
    mut yyMajor: ::core::ffi::c_ushort,
    mut yyMinor: Token,
) {
    let mut yytos: *mut yyStackEntry = ::core::ptr::null_mut::<yyStackEntry>();
    (*yypParser).yytos = (*yypParser).yytos.offset(1);
    yytos = (*yypParser).yytos;
    if yytos > (*yypParser).yystackEnd {
        if yyGrowStack(yypParser) != 0 {
            (*yypParser).yytos = (*yypParser).yytos.offset(-1);
            yyStackOverflow(yypParser);
            return;
        }
        yytos = (*yypParser).yytos;
    }
    if yyNewState as ::core::ffi::c_int > YY_MAX_SHIFT {
        yyNewState = (yyNewState as ::core::ffi::c_int + (YY_MIN_REDUCE - YY_MIN_SHIFTREDUCE))
            as ::core::ffi::c_ushort;
    }
    (*yytos).stateno = yyNewState;
    (*yytos).major = yyMajor;
    (*yytos).minor.yy0 = yyMinor;
}
static mut yyRuleInfoLhs: [::core::ffi::c_ushort; 409] = [
    191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    199 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    200 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    200 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    225 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    225 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    231 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    235 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    249 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    249 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    250 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    250 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    280 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    280 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    284 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    284 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    293 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    293 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    298 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    298 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    299 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    305 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    253 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    203 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    203 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    212 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    212 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    230 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    230 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    231 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    235 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    302 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    302 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];
static mut yyRuleInfoNRhs: [::core::ffi::c_schar; 409] = [
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(7 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(10 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(10 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(7 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(7 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(12 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(12 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(7 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
];
unsafe extern "C" fn yy_reduce(
    mut yypParser: *mut yyParser,
    mut yyruleno: ::core::ffi::c_uint,
    mut _yyLookahead: ::core::ffi::c_int,
    mut yyLookaheadToken: Token,
    mut pParse: *mut Parse,
) -> ::core::ffi::c_ushort {
    let mut yygoto: ::core::ffi::c_int = 0;
    let mut yyact: ::core::ffi::c_ushort = 0;
    let mut yymsp: *mut yyStackEntry = ::core::ptr::null_mut::<yyStackEntry>();
    let mut yysize: ::core::ffi::c_int = 0;
    yymsp = (*yypParser).yytos;
    let mut yylhsminor: YYMINORTYPE = YYMINORTYPE { yyinit: 0 };
    match yyruleno {
        0 => {
            if (*pParse).pReprepare.is_null() {
                (*pParse).explain = 1 as u8_0;
            }
        }
        1 => {
            if (*pParse).pReprepare.is_null() {
                (*pParse).explain = 2 as u8_0;
            }
        }
        2 => {
            sqlite3FinishCoding(pParse);
        }
        3 => {
            sqlite3BeginTransaction(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        4 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = TK_DEFERRED;
        }
        5 | 6 | 7 | 324 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 =
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).major as ::core::ffi::c_int;
        }
        8 | 9 => {
            sqlite3EndTransaction(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major as ::core::ffi::c_int,
            );
        }
        10 => {
            sqlite3Savepoint(
                pParse,
                SAVEPOINT_BEGIN,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        11 => {
            sqlite3Savepoint(
                pParse,
                SAVEPOINT_RELEASE,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        12 => {
            sqlite3Savepoint(
                pParse,
                SAVEPOINT_ROLLBACK,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        13 => {
            sqlite3StartTable(
                pParse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        14 => {
            disableLookaside(pParse);
        }
        15 | 18 | 47 | 62 | 72 | 81 | 100 | 246 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = 0 as ::core::ffi::c_int;
        }
        16 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = 1 as ::core::ffi::c_int;
        }
        17 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = ((*(*pParse).db).init.busy as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        19 => {
            sqlite3EndTable(
                pParse,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy9,
                ::core::ptr::null_mut::<Select>(),
            );
        }
        20 => {
            sqlite3EndTable(
                pParse,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Token>(),
                0 as u32_0,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy637,
            );
            sqlite3SelectDelete(
                (*pParse).db,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy637,
            );
        }
        21 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize)).minor.yy9 = 0 as u32_0;
        }
        22 => {
            yylhsminor.yy9 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy9
                | (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy9;
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy9 = yylhsminor.yy9;
        }
        23 => {
            if (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .n
                == 5 as ::core::ffi::c_uint
                && sqlite3_strnicmp(
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .z,
                    b"rowid\0" as *const u8 as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
            {
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy9 = (TF_WithoutRowid | TF_NoVisibleRowid) as u32_0;
            } else {
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy9 = 0 as u32_0;
                sqlite3ErrorMsg(
                    pParse,
                    b"unknown table option: %.*s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .n,
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .z,
                );
            }
        }
        24 => {
            if (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .n
                == 6 as ::core::ffi::c_uint
                && sqlite3_strnicmp(
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .z,
                    b"strict\0" as *const u8 as *const ::core::ffi::c_char,
                    6 as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
            {
                yylhsminor.yy9 = TF_Strict as u32_0;
            } else {
                yylhsminor.yy9 = 0 as u32_0;
                sqlite3ErrorMsg(
                    pParse,
                    b"unknown table option: %.*s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .n,
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .z,
                );
            }
            (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy9 = yylhsminor.yy9;
        }
        25 => {
            sqlite3AddColumn(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        26 | 65 | 106 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .n = 0 as ::core::ffi::c_uint;
            let ref mut fresh1 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .z;
            *fresh1 = ::core::ptr::null::<::core::ffi::c_char>();
        }
        27 => {
            (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n = ((*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .z
                .offset(
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .n as isize,
                ) as *const ::core::ffi::c_char)
                .offset_from(
                    (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0
                        .z,
                ) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint;
        }
        28 => {
            (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n = ((*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .z
                .offset(
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .n as isize,
                ) as *const ::core::ffi::c_char)
                .offset_from(
                    (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0
                        .z,
                ) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint;
        }
        29 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .n
                .wrapping_add(
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .z
                        .offset_from(
                            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                                .minor
                                .yy0
                                .z,
                        ) as ::core::ffi::c_long as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                );
        }
        30 => {
            let ref mut fresh2 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy342;
            *fresh2 = yyLookaheadToken.z;
        }
        31 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize)).minor.yy0 = yyLookaheadToken;
        }
        32 | 67 => {
            (*pParse).u1.cr.constraintName =
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0;
        }
        33 => {
            sqlite3AddDefaultValue(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z
                    .offset(
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy0
                            .n as isize,
                    ) as *const ::core::ffi::c_char,
            );
        }
        34 => {
            sqlite3AddDefaultValue(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z
                    .offset(1 as ::core::ffi::c_int as isize),
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy0
                    .z,
            );
        }
        35 => {
            sqlite3AddDefaultValue(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z
                    .offset(
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy0
                            .n as isize,
                    ) as *const ::core::ffi::c_char,
            );
        }
        36 => {
            let mut p: *mut Expr = sqlite3PExpr(
                pParse,
                TK_UMINUS,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
            sqlite3AddDefaultValue(
                pParse,
                p,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z
                    .offset(
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy0
                            .n as isize,
                    ) as *const ::core::ffi::c_char,
            );
        }
        37 => {
            let mut p_0: *mut Expr = tokenExpr(
                pParse,
                TK_STRING,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
            if !p_0.is_null() {
                sqlite3ExprIdToTrueFalse(p_0);
            }
            sqlite3AddDefaultValue(
                pParse,
                p_0,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy0
                    .z
                    .offset(
                        (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                            .minor
                            .yy0
                            .n as isize,
                    ),
            );
        }
        38 => {
            sqlite3AddNotNull(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
            );
        }
        39 => {
            sqlite3AddPrimaryKey(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        40 => {
            sqlite3CreateIndex(
                pParse,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<SrcList>(),
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Expr>(),
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                SQLITE_IDXTYPE_UNIQUE as u8_0,
            );
        }
        41 => {
            sqlite3AddCheckConstraint(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy0
                    .z,
            );
        }
        42 => {
            sqlite3CreateForeignKey(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
            );
        }
        43 => {
            sqlite3DeferForeignKey(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
            );
        }
        44 => {
            sqlite3AddCollateType(
                pParse,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        45 => {
            sqlite3AddGenerated(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Token>(),
            );
        }
        46 => {
            sqlite3AddGenerated(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        48 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = 1 as ::core::ffi::c_int;
        }
        49 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = OE_None * 0x101 as ::core::ffi::c_int;
        }
        50 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502
                & !(*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy481
                    .mask
                | (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy481
                    .value;
        }
        51 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .value = 0 as ::core::ffi::c_int;
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .mask = 0 as ::core::ffi::c_int;
        }
        52 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .value = 0 as ::core::ffi::c_int;
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .mask = 0 as ::core::ffi::c_int;
        }
        53 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .value = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502;
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .mask = 0xff as ::core::ffi::c_int;
        }
        54 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .value = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502
                << 8 as ::core::ffi::c_int;
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .mask = 0xff00 as ::core::ffi::c_int;
        }
        55 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = OE_SetNull;
        }
        56 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = OE_SetDflt;
        }
        57 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = OE_Cascade;
        }
        58 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = OE_Restrict;
        }
        59 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = OE_None;
        }
        60 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = 0 as ::core::ffi::c_int;
        }
        61 | 76 | 173 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502;
        }
        63 | 80 | 219 | 222 | 247 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = 1 as ::core::ffi::c_int;
        }
        64 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = 0 as ::core::ffi::c_int;
        }
        66 => {
            (*pParse).u1.cr.constraintName.n = 0 as ::core::ffi::c_uint;
        }
        68 => {
            sqlite3AddPrimaryKey(
                pParse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                0 as ::core::ffi::c_int,
            );
        }
        69 => {
            sqlite3CreateIndex(
                pParse,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<SrcList>(),
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Expr>(),
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                SQLITE_IDXTYPE_UNIQUE as u8_0,
            );
        }
        70 => {
            sqlite3AddCheckConstraint(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
            );
        }
        71 => {
            sqlite3CreateForeignKey(
                pParse,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
            sqlite3DeferForeignKey(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
            );
        }
        73 | 75 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = OE_Default;
        }
        74 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502;
        }
        77 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = OE_Ignore;
        }
        78 | 174 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = OE_Replace;
        }
        79 => {
            sqlite3DropTable(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy563,
                0 as ::core::ffi::c_int,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        82 => {
            sqlite3CreateView(
                pParse,
                &raw mut (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy637,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        83 => {
            sqlite3DropTable(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy563,
                1 as ::core::ffi::c_int,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        84 => {
            let mut dest: SelectDest = SelectDest {
                eDest: SRT_Output as u8_0,
                iSDParm: 0 as ::core::ffi::c_int,
                iSDParm2: 0 as ::core::ffi::c_int,
                iSdst: 0 as ::core::ffi::c_int,
                nSdst: 0 as ::core::ffi::c_int,
                zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                pOrderBy: ::core::ptr::null_mut::<ExprList>(),
            };
            if (*(*pParse).db).mDbFlags & DBFLAG_EncodingFixed as u32_0 != 0 as u32_0
                || sqlite3ReadSchema(pParse) == SQLITE_OK
            {
                sqlite3Select(
                    pParse,
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy637,
                    &raw mut dest,
                );
            }
            sqlite3SelectDelete(
                (*pParse).db,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy637,
            );
        }
        85 => {
            let ref mut fresh3 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh3 = attachWithToSelect(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy637,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy125,
            );
        }
        86 => {
            let ref mut fresh4 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh4 = attachWithToSelect(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy637,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy125,
            );
        }
        87 => {
            let mut p_1: *mut Select = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy637;
            if !p_1.is_null() {
                parserDoubleLinkSelect(pParse, p_1);
            }
        }
        88 => {
            let mut pRhs: *mut Select = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy637;
            let mut pLhs: *mut Select = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            if !pRhs.is_null() && !(*pRhs).pPrior.is_null() {
                let mut pFrom: *mut SrcList = ::core::ptr::null_mut::<SrcList>();
                let mut x: Token = Token {
                    z: ::core::ptr::null::<::core::ffi::c_char>(),
                    n: 0,
                };
                x.n = 0 as ::core::ffi::c_uint;
                parserDoubleLinkSelect(pParse, pRhs);
                pFrom = sqlite3SrcListAppendFromTerm(
                    pParse,
                    ::core::ptr::null_mut::<SrcList>(),
                    ::core::ptr::null_mut::<Token>(),
                    ::core::ptr::null_mut::<Token>(),
                    &raw mut x,
                    pRhs,
                    ::core::ptr::null_mut::<OnOrUsing>(),
                );
                pRhs = sqlite3SelectNew(
                    pParse,
                    ::core::ptr::null_mut::<ExprList>(),
                    pFrom,
                    ::core::ptr::null_mut::<Expr>(),
                    ::core::ptr::null_mut::<ExprList>(),
                    ::core::ptr::null_mut::<Expr>(),
                    ::core::ptr::null_mut::<ExprList>(),
                    0 as u32_0,
                    ::core::ptr::null_mut::<Expr>(),
                );
            }
            if !pRhs.is_null() {
                (*pRhs).op = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as u8_0;
                (*pRhs).pPrior = pLhs;
                if !pLhs.is_null() {
                    (*pLhs).selFlags &= !(SF_MultiValue as u32_0);
                }
                (*pRhs).selFlags &= !(SF_MultiValue as u32_0);
                if (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502
                    != TK_ALL
                {
                    (*pParse).hasCompound = 1 as u8_0;
                }
            } else {
                sqlite3SelectDelete((*pParse).db, pLhs);
            }
            let ref mut fresh5 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh5 = pRhs;
        }
        89 | 91 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 =
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).major as ::core::ffi::c_int;
        }
        90 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = TK_ALL;
        }
        92 => {
            let ref mut fresh6 = (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh6 = sqlite3SelectNew(
                pParse,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as u32_0,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        93 => {
            let ref mut fresh7 = (*yymsp.offset(-(9 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh7 = sqlite3SelectNew(
                pParse,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as u32_0,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            if !(*yymsp.offset(-(9 as ::core::ffi::c_int) as isize))
                .minor
                .yy637
                .is_null()
            {
                let ref mut fresh8 = (*(*yymsp.offset(-(9 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637)
                    .pWinDefn;
                *fresh8 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy483;
            } else {
                sqlite3WindowListDelete(
                    (*pParse).db,
                    (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy483,
                );
            }
        }
        94 => {
            let ref mut fresh9 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh9 = sqlite3SelectNew(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                ::core::ptr::null_mut::<SrcList>(),
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<ExprList>(),
                SF_Values as u32_0,
                ::core::ptr::null_mut::<Expr>(),
            );
        }
        95 => {
            sqlite3MultiValuesEnd(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy637,
            );
        }
        96 | 97 => {
            let ref mut fresh10 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh10 = sqlite3MultiValues(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
            );
        }
        98 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = SF_Distinct;
        }
        99 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = SF_All;
        }
        101 | 134 | 144 | 234 | 237 | 242 => {
            let ref mut fresh11 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy402;
            *fresh11 = ::core::ptr::null_mut::<ExprList>();
        }
        102 => {
            let ref mut fresh12 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh12 = sqlite3ExprListAppend(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            if (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .n
                > 0 as ::core::ffi::c_uint
            {
                sqlite3ExprListSetName(
                    pParse,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy402,
                    &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                    1 as ::core::ffi::c_int,
                );
            }
            sqlite3ExprListSetSpan(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy342,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy342,
            );
        }
        103 => {
            let mut p_2: *mut Expr = sqlite3Expr(
                (*pParse).db,
                TK_ASTERISK,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            sqlite3ExprSetErrorOffset(
                p_2,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy0
                    .z
                    .offset_from((*pParse).zTail) as ::core::ffi::c_long
                    as ::core::ffi::c_int,
            );
            let ref mut fresh13 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh13 = sqlite3ExprListAppend(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                p_2,
            );
        }
        104 => {
            let mut pRight: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pLeft: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let mut pDot: *mut Expr = ::core::ptr::null_mut::<Expr>();
            pRight = sqlite3PExpr(
                pParse,
                TK_ASTERISK,
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<Expr>(),
            );
            sqlite3ExprSetErrorOffset(
                pRight,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy0
                    .z
                    .offset_from((*pParse).zTail) as ::core::ffi::c_long
                    as ::core::ffi::c_int,
            );
            pLeft = tokenExpr(
                pParse,
                TK_ID,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            pDot = sqlite3PExpr(pParse, TK_DOT, pLeft, pRight);
            let ref mut fresh14 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh14 = sqlite3ExprListAppend(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                pDot,
            );
        }
        105 | 117 | 258 | 259 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0 = (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0;
        }
        107 | 110 => {
            let ref mut fresh15 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy563;
            *fresh15 = ::core::ptr::null_mut::<SrcList>();
        }
        108 => {
            let ref mut fresh16 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh16 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy563;
            sqlite3SrcListShiftJoinType(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
            );
        }
        109 => {
            if !(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
                && (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563)
                    .nSrc
                    > 0 as ::core::ffi::c_int
            {
                (*(&raw mut (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563)
                    .a as *mut SrcItem)
                    .offset(
                        ((*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy563)
                            .nSrc
                            - 1 as ::core::ffi::c_int) as isize,
                    ))
                .fg
                .jointype = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502 as u8_0;
            }
        }
        111 => {
            let ref mut fresh17 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh17 = sqlite3SrcListAppendFromTerm(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                ::core::ptr::null_mut::<Select>(),
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy421,
            );
        }
        112 => {
            let ref mut fresh18 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh18 = sqlite3SrcListAppendFromTerm(
                pParse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                ::core::ptr::null_mut::<Select>(),
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy421,
            );
            sqlite3SrcListIndexedBy(
                pParse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
        }
        113 => {
            let ref mut fresh19 = (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh19 = sqlite3SrcListAppendFromTerm(
                pParse,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                &raw mut (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                ::core::ptr::null_mut::<Select>(),
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy421,
            );
            sqlite3SrcListFuncArgs(
                pParse,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
            );
        }
        114 => {
            let ref mut fresh20 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh20 = sqlite3SrcListAppendFromTerm(
                pParse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Token>(),
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy421,
            );
        }
        115 => {
            if (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
                && (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .n
                    == 0 as ::core::ffi::c_uint
                && (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy421
                    .pOn
                    .is_null()
                && (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy421
                    .pUsing
                    .is_null()
            {
                let ref mut fresh21 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563;
                *fresh21 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563;
            } else if !(*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
                && (*(*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563)
                    .nSrc
                    == 1 as ::core::ffi::c_int
            {
                let ref mut fresh22 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563;
                *fresh22 = sqlite3SrcListAppendFromTerm(
                    pParse,
                    (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563,
                    ::core::ptr::null_mut::<Token>(),
                    ::core::ptr::null_mut::<Token>(),
                    &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0,
                    ::core::ptr::null_mut::<Select>(),
                    &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy421,
                );
                if !(*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563
                    .is_null()
                {
                    let mut pNew: *mut SrcItem = (&raw mut (*(*yymsp
                        .offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563)
                        .a as *mut SrcItem)
                        .offset(
                            ((*(*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                                .minor
                                .yy563)
                                .nSrc
                                - 1 as ::core::ffi::c_int) as isize,
                        ) as *mut SrcItem;
                    let mut pOld: *mut SrcItem = &raw mut (*(*yymsp
                        .offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563)
                        .a as *mut SrcItem;
                    (*pNew).zName = (*pOld).zName;
                    if (*pOld).fg.isSubquery() != 0 {
                        (*pNew)
                            .fg
                            .set_isSubquery(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*pNew).u4.pSubq = (*pOld).u4.pSubq;
                        (*pOld).u4.pSubq = ::core::ptr::null_mut::<Subquery>();
                        (*pOld)
                            .fg
                            .set_isSubquery(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        if (*(*(*pNew).u4.pSubq).pSelect).selFlags & SF_NestedFrom as u32_0
                            != 0 as u32_0
                        {
                            (*pNew)
                                .fg
                                .set_isNestedFrom(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        }
                    } else {
                        (*pNew).u4.zDatabase = (*pOld).u4.zDatabase;
                        (*pOld).u4.zDatabase = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    if (*pOld).fg.isTabFunc() != 0 {
                        (*pNew).u1.pFuncArg = (*pOld).u1.pFuncArg;
                        (*pOld).u1.pFuncArg = ::core::ptr::null_mut::<ExprList>();
                        (*pOld)
                            .fg
                            .set_isTabFunc(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*pNew)
                            .fg
                            .set_isTabFunc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    (*pOld).zName = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                sqlite3SrcListDelete(
                    (*pParse).db,
                    (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563,
                );
            } else {
                let mut pSubquery: *mut Select = ::core::ptr::null_mut::<Select>();
                sqlite3SrcListShiftJoinType(
                    pParse,
                    (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563,
                );
                pSubquery = sqlite3SelectNew(
                    pParse,
                    ::core::ptr::null_mut::<ExprList>(),
                    (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563,
                    ::core::ptr::null_mut::<Expr>(),
                    ::core::ptr::null_mut::<ExprList>(),
                    ::core::ptr::null_mut::<Expr>(),
                    ::core::ptr::null_mut::<ExprList>(),
                    SF_NestedFrom as u32_0,
                    ::core::ptr::null_mut::<Expr>(),
                );
                let ref mut fresh23 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563;
                *fresh23 = sqlite3SrcListAppendFromTerm(
                    pParse,
                    (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563,
                    ::core::ptr::null_mut::<Token>(),
                    ::core::ptr::null_mut::<Token>(),
                    &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0,
                    pSubquery,
                    &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy421,
                );
            }
        }
        116 | 131 => {
            let ref mut fresh24 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .z;
            *fresh24 = ::core::ptr::null::<::core::ffi::c_char>();
            (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .n = 0 as ::core::ffi::c_uint;
        }
        118 => {
            yylhsminor.yy563 = sqlite3SrcListAppend(
                pParse,
                ::core::ptr::null_mut::<SrcList>(),
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                ::core::ptr::null_mut::<Token>(),
            );
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME
                && !yylhsminor.yy563.is_null()
            {
                sqlite3RenameTokenMap(
                    pParse,
                    (*(&raw mut (*yylhsminor.yy563).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                    .zName as *const ::core::ffi::c_void,
                    &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                );
            }
            let ref mut fresh25 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy563;
            *fresh25 = yylhsminor.yy563;
        }
        119 => {
            yylhsminor.yy563 = sqlite3SrcListAppend(
                pParse,
                ::core::ptr::null_mut::<SrcList>(),
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME
                && !yylhsminor.yy563.is_null()
            {
                sqlite3RenameTokenMap(
                    pParse,
                    (*(&raw mut (*yylhsminor.yy563).a as *mut SrcItem)
                        .offset(0 as ::core::ffi::c_int as isize))
                    .zName as *const ::core::ffi::c_void,
                    &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                );
            }
            let ref mut fresh26 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh26 = yylhsminor.yy563;
        }
        120 => {
            let ref mut fresh27 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy563;
            *fresh27 = sqlite3SrcListAppend(
                pParse,
                ::core::ptr::null_mut::<SrcList>(),
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                ::core::ptr::null_mut::<Token>(),
            );
        }
        121 => {
            let ref mut fresh28 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh28 = sqlite3SrcListAppend(
                pParse,
                ::core::ptr::null_mut::<SrcList>(),
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        122 => {
            let ref mut fresh29 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh29 = sqlite3SrcListAppend(
                pParse,
                ::core::ptr::null_mut::<SrcList>(),
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            if !(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
            {
                let ref mut fresh30 = (*(&raw mut (*(*yymsp
                    .offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy563)
                    .a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                .zAlias;
                *fresh30 = sqlite3NameFromToken(
                    (*pParse).db,
                    &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                );
            }
        }
        123 => {
            let ref mut fresh31 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh31 = sqlite3SrcListAppend(
                pParse,
                ::core::ptr::null_mut::<SrcList>(),
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                ::core::ptr::null_mut::<Token>(),
            );
            if !(*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
            {
                let ref mut fresh32 = (*(&raw mut (*(*yymsp
                    .offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy563)
                    .a as *mut SrcItem)
                    .offset(0 as ::core::ffi::c_int as isize))
                .zAlias;
                *fresh32 = sqlite3NameFromToken(
                    (*pParse).db,
                    &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                );
            }
        }
        124 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = JT_INNER;
        }
        125 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = sqlite3JoinType(
                pParse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Token>(),
            );
        }
        126 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = sqlite3JoinType(
                pParse,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                ::core::ptr::null_mut::<Token>(),
            );
        }
        127 => {
            (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = sqlite3JoinType(
                pParse,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
        }
        128 => {
            let ref mut fresh33 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy421
                .pOn;
            *fresh33 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
            let ref mut fresh34 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy421
                .pUsing;
            *fresh34 = ::core::ptr::null_mut::<IdList>();
        }
        129 => {
            let ref mut fresh35 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy421
                .pOn;
            *fresh35 = ::core::ptr::null_mut::<Expr>();
            let ref mut fresh36 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy421
                .pUsing;
            *fresh36 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy204;
        }
        130 => {
            let ref mut fresh37 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy421
                .pOn;
            *fresh37 = ::core::ptr::null_mut::<Expr>();
            let ref mut fresh38 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy421
                .pUsing;
            *fresh38 = ::core::ptr::null_mut::<IdList>();
        }
        132 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy0 = (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0;
        }
        133 => {
            let ref mut fresh39 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .z;
            *fresh39 = ::core::ptr::null::<::core::ffi::c_char>();
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n = 1 as ::core::ffi::c_uint;
        }
        135 | 145 => {
            let ref mut fresh40 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh40 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy402;
        }
        136 => {
            let ref mut fresh41 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh41 = sqlite3ExprListAppend(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            sqlite3ExprListSetSortOrder(
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
            );
        }
        137 => {
            let ref mut fresh42 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh42 = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            sqlite3ExprListSetSortOrder(
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
            );
        }
        138 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = SQLITE_SO_ASC;
        }
        139 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = SQLITE_SO_DESC;
        }
        140 | 143 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = SQLITE_SO_UNDEFINED;
        }
        141 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = SQLITE_SO_ASC;
        }
        142 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = SQLITE_SO_DESC;
        }
        146 | 148 | 153 | 155 | 232 | 233 | 252 => {
            let ref mut fresh43 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
            *fresh43 = ::core::ptr::null_mut::<Expr>();
        }
        147 | 154 | 156 | 231 | 251 => {
            let ref mut fresh44 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh44 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
        }
        149 => {
            let ref mut fresh45 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh45 = sqlite3PExpr(
                pParse,
                TK_LIMIT,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
        }
        150 => {
            let ref mut fresh46 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh46 = sqlite3PExpr(
                pParse,
                TK_LIMIT,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        151 => {
            let ref mut fresh47 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh47 = sqlite3PExpr(
                pParse,
                TK_LIMIT,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
        }
        152 => {
            sqlite3SrcListIndexedBy(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            sqlite3DeleteFrom(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
            );
        }
        157 => {
            sqlite3AddReturning(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy402,
            );
            let ref mut fresh48 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh48 = ::core::ptr::null_mut::<Expr>();
        }
        158 => {
            sqlite3AddReturning(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy402,
            );
            let ref mut fresh49 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh49 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
        }
        159 => {
            sqlite3SrcListIndexedBy(
                pParse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            sqlite3ExprListCheckLength(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                b"set list\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
            {
                let mut pFromClause: *mut SrcList = (*yymsp
                    .offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
                if (*pFromClause).nSrc > 1 as ::core::ffi::c_int {
                    let mut pSubquery_0: *mut Select = ::core::ptr::null_mut::<Select>();
                    let mut as_0: Token = Token {
                        z: ::core::ptr::null::<::core::ffi::c_char>(),
                        n: 0,
                    };
                    pSubquery_0 = sqlite3SelectNew(
                        pParse,
                        ::core::ptr::null_mut::<ExprList>(),
                        pFromClause,
                        ::core::ptr::null_mut::<Expr>(),
                        ::core::ptr::null_mut::<ExprList>(),
                        ::core::ptr::null_mut::<Expr>(),
                        ::core::ptr::null_mut::<ExprList>(),
                        SF_NestedFrom as u32_0,
                        ::core::ptr::null_mut::<Expr>(),
                    );
                    as_0.n = 0 as ::core::ffi::c_uint;
                    as_0.z = ::core::ptr::null::<::core::ffi::c_char>();
                    pFromClause = sqlite3SrcListAppendFromTerm(
                        pParse,
                        ::core::ptr::null_mut::<SrcList>(),
                        ::core::ptr::null_mut::<Token>(),
                        ::core::ptr::null_mut::<Token>(),
                        &raw mut as_0,
                        pSubquery_0,
                        ::core::ptr::null_mut::<OnOrUsing>(),
                    );
                }
                let ref mut fresh50 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563;
                *fresh50 = sqlite3SrcListAppendList(
                    pParse,
                    (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563,
                    pFromClause,
                );
            }
            sqlite3Update(
                pParse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<Upsert>(),
            );
        }
        160 => {
            let ref mut fresh51 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh51 = sqlite3ExprListAppend(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            sqlite3ExprListSetName(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                1 as ::core::ffi::c_int,
            );
        }
        161 => {
            let ref mut fresh52 = (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh52 = sqlite3ExprListAppendVector(
                pParse,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        162 => {
            yylhsminor.yy402 = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            sqlite3ExprListSetName(
                pParse,
                yylhsminor.yy402,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                1 as ::core::ffi::c_int,
            );
            let ref mut fresh53 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh53 = yylhsminor.yy402;
        }
        163 => {
            let ref mut fresh54 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh54 = sqlite3ExprListAppendVector(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        164 => {
            sqlite3Insert(
                pParse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy403,
            );
        }
        165 => {
            sqlite3Insert(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                ::core::ptr::null_mut::<Select>(),
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                ::core::ptr::null_mut::<Upsert>(),
            );
        }
        166 => {
            let ref mut fresh55 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy403;
            *fresh55 = ::core::ptr::null_mut::<Upsert>();
        }
        167 => {
            let ref mut fresh56 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy403;
            *fresh56 = ::core::ptr::null_mut::<Upsert>();
            sqlite3AddReturning(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy402,
            );
        }
        168 => {
            let ref mut fresh57 = (*yymsp.offset(-(11 as ::core::ffi::c_int) as isize))
                .minor
                .yy403;
            *fresh57 = sqlite3UpsertNew(
                (*pParse).db,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy403,
            );
        }
        169 => {
            let ref mut fresh58 = (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                .minor
                .yy403;
            *fresh58 = sqlite3UpsertNew(
                (*pParse).db,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy403,
            );
        }
        170 => {
            let ref mut fresh59 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy403;
            *fresh59 = sqlite3UpsertNew(
                (*pParse).db,
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<Upsert>(),
            );
        }
        171 => {
            let ref mut fresh60 = (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                .minor
                .yy403;
            *fresh60 = sqlite3UpsertNew(
                (*pParse).db,
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Upsert>(),
            );
        }
        172 => {
            sqlite3AddReturning(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy402,
            );
        }
        175 => {
            let ref mut fresh61 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy204;
            *fresh61 = ::core::ptr::null_mut::<IdList>();
        }
        176 => {
            let ref mut fresh62 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy204;
            *fresh62 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy204;
        }
        177 => {
            let ref mut fresh63 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy204;
            *fresh63 = sqlite3IdListAppend(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        178 => {
            let ref mut fresh64 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy204;
            *fresh64 = sqlite3IdListAppend(
                pParse,
                ::core::ptr::null_mut::<IdList>(),
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        179 => {
            let ref mut fresh65 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh65 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
        }
        180 => {
            let ref mut fresh66 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
            *fresh66 = tokenExpr(
                pParse,
                TK_ID,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        181 => {
            let mut temp1: *mut Expr = tokenExpr(
                pParse,
                TK_ID,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            let mut temp2: *mut Expr = tokenExpr(
                pParse,
                TK_ID,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
            yylhsminor.yy590 = sqlite3PExpr(pParse, TK_DOT, temp1, temp2);
            let ref mut fresh67 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh67 = yylhsminor.yy590;
        }
        182 => {
            let mut temp1_0: *mut Expr = tokenExpr(
                pParse,
                TK_ID,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            let mut temp2_0: *mut Expr = tokenExpr(
                pParse,
                TK_ID,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            let mut temp3: *mut Expr = tokenExpr(
                pParse,
                TK_ID,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
            let mut temp4: *mut Expr = sqlite3PExpr(pParse, TK_DOT, temp2_0, temp3);
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME {
                sqlite3RenameTokenRemap(
                    pParse,
                    ::core::ptr::null::<::core::ffi::c_void>(),
                    temp1_0 as *const ::core::ffi::c_void,
                );
            }
            yylhsminor.yy590 = sqlite3PExpr(pParse, TK_DOT, temp1_0, temp4);
            let ref mut fresh68 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh68 = yylhsminor.yy590;
        }
        183 | 184 => {
            let ref mut fresh69 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
            *fresh69 = tokenExpr(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).major as ::core::ffi::c_int,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        185 => {
            yylhsminor.yy590 = sqlite3ExprAlloc(
                (*pParse).db,
                TK_INTEGER,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                1 as ::core::ffi::c_int,
            );
            if !yylhsminor.yy590.is_null() {
                (*yylhsminor.yy590).w.iOfst = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy0
                    .z
                    .offset_from((*pParse).zTail)
                    as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            }
            let ref mut fresh70 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
            *fresh70 = yylhsminor.yy590;
        }
        186 => {
            if !(*(*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .z
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '#' as i32
                && *(&raw const sqlite3CtypeMap as *const ::core::ffi::c_uchar).offset(
                    *(*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .z
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar as isize,
                ) as ::core::ffi::c_int
                    & 0x4 as ::core::ffi::c_int
                    != 0)
            {
                let mut n: u32_0 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy0
                    .n as u32_0;
                let ref mut fresh71 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590;
                *fresh71 = tokenExpr(
                    pParse,
                    TK_VARIABLE,
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                );
                sqlite3ExprAssignVarNumber(
                    pParse,
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy590,
                    n,
                );
            } else {
                let mut t: Token = (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0;
                if (*pParse).nested as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    parserSyntaxError(pParse, &raw mut t);
                    let ref mut fresh72 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy590;
                    *fresh72 = ::core::ptr::null_mut::<Expr>();
                } else {
                    let ref mut fresh73 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy590;
                    *fresh73 = sqlite3PExpr(
                        pParse,
                        TK_REGISTER,
                        ::core::ptr::null_mut::<Expr>(),
                        ::core::ptr::null_mut::<Expr>(),
                    );
                    if !(*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy590
                        .is_null()
                    {
                        sqlite3GetInt32(
                            t.z.offset(1 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_char,
                            &raw mut (*(*yymsp.offset(0 as ::core::ffi::c_int as isize))
                                .minor
                                .yy590)
                                .iTable,
                        );
                    }
                }
            }
        }
        187 => {
            let ref mut fresh74 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh74 = sqlite3ExprAddCollateToken(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                1 as ::core::ffi::c_int,
            );
        }
        188 => {
            let ref mut fresh75 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh75 = sqlite3ExprAlloc(
                (*pParse).db,
                TK_CAST,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                1 as ::core::ffi::c_int,
            );
            sqlite3ExprAttachSubtrees(
                (*pParse).db,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
        }
        189 => {
            yylhsminor.yy590 = sqlite3ExprFunction(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
            let ref mut fresh76 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh76 = yylhsminor.yy590;
        }
        190 => {
            yylhsminor.yy590 = sqlite3ExprFunction(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                &raw mut (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
            sqlite3ExprAddFunctionOrderBy(
                pParse,
                yylhsminor.yy590,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
            );
            let ref mut fresh77 = (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh77 = yylhsminor.yy590;
        }
        191 => {
            yylhsminor.yy590 = sqlite3ExprFunction(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                0 as ::core::ffi::c_int,
            );
            let ref mut fresh78 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh78 = yylhsminor.yy590;
        }
        192 => {
            yylhsminor.yy590 = sqlite3ExprFunction(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                &raw mut (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
            sqlite3WindowAttach(
                pParse,
                yylhsminor.yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy483,
            );
            let ref mut fresh79 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh79 = yylhsminor.yy590;
        }
        193 => {
            yylhsminor.yy590 = sqlite3ExprFunction(
                pParse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                &raw mut (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
            sqlite3WindowAttach(
                pParse,
                yylhsminor.yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy483,
            );
            sqlite3ExprAddFunctionOrderBy(
                pParse,
                yylhsminor.yy590,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
            );
            let ref mut fresh80 = (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh80 = yylhsminor.yy590;
        }
        194 => {
            yylhsminor.yy590 = sqlite3ExprFunction(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                0 as ::core::ffi::c_int,
            );
            sqlite3WindowAttach(
                pParse,
                yylhsminor.yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy483,
            );
            let ref mut fresh81 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh81 = yylhsminor.yy590;
        }
        195 => {
            yylhsminor.yy590 = sqlite3ExprFunction(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                0 as ::core::ffi::c_int,
            );
            let ref mut fresh82 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
            *fresh82 = yylhsminor.yy590;
        }
        196 => {
            let mut pList: *mut ExprList = sqlite3ExprListAppend(
                pParse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            let ref mut fresh83 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh83 = sqlite3PExpr(
                pParse,
                TK_VECTOR,
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<Expr>(),
            );
            if !(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                let ref mut fresh84 = (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .x
                    .pList;
                *fresh84 = pList;
                if (*pList).nExpr != 0 {
                    (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590)
                        .flags |= (*(*(&raw mut (*pList).a as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                    .pExpr)
                        .flags
                        & EP_Propagate as u32_0;
                }
            } else {
                sqlite3ExprListDelete((*pParse).db, pList);
            }
        }
        197 => {
            let ref mut fresh85 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh85 = sqlite3ExprAnd(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        198 | 199 | 200 | 201 | 202 | 203 | 204 => {
            let ref mut fresh86 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh86 = sqlite3PExpr(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major as ::core::ffi::c_int,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        205 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0 = (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0;
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n |= 0x80000000 as ::core::ffi::c_uint;
        }
        206 => {
            let mut pList_0: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut bNot: ::core::ffi::c_int = ((*yymsp
                .offset(-(1 as ::core::ffi::c_int) as isize))
            .minor
            .yy0
            .n & 0x80000000 as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n &= 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint;
            pList_0 = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            pList_0 = sqlite3ExprListAppend(
                pParse,
                pList_0,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            let ref mut fresh87 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh87 = sqlite3ExprFunction(
                pParse,
                pList_0,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                0 as ::core::ffi::c_int,
            );
            if bNot != 0 {
                let ref mut fresh88 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh88 = sqlite3PExpr(
                    pParse,
                    TK_NOT,
                    (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590,
                    ::core::ptr::null_mut::<Expr>(),
                );
            }
            if !(*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                (*(*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .flags |= EP_InfixFunc as u32_0;
            }
        }
        207 => {
            let mut pList_1: *mut ExprList = ::core::ptr::null_mut::<ExprList>();
            let mut bNot_0: ::core::ffi::c_int = ((*yymsp
                .offset(-(3 as ::core::ffi::c_int) as isize))
            .minor
            .yy0
            .n & 0x80000000 as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
            (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n &= 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint;
            pList_1 = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            pList_1 = sqlite3ExprListAppend(
                pParse,
                pList_1,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            pList_1 = sqlite3ExprListAppend(
                pParse,
                pList_1,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            let ref mut fresh89 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh89 = sqlite3ExprFunction(
                pParse,
                pList_1,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                0 as ::core::ffi::c_int,
            );
            if bNot_0 != 0 {
                let ref mut fresh90 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh90 = sqlite3PExpr(
                    pParse,
                    TK_NOT,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590,
                    ::core::ptr::null_mut::<Expr>(),
                );
            }
            if !(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .flags |= EP_InfixFunc as u32_0;
            }
        }
        208 => {
            let ref mut fresh91 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh91 = sqlite3PExpr(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).major as ::core::ffi::c_int,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
        }
        209 => {
            let ref mut fresh92 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh92 = sqlite3PExpr(
                pParse,
                TK_NOTNULL,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
        }
        210 => {
            let ref mut fresh93 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh93 = sqlite3PExpr(
                pParse,
                TK_IS,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            binaryToUnaryIfNull(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                TK_ISNULL,
            );
        }
        211 => {
            let ref mut fresh94 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh94 = sqlite3PExpr(
                pParse,
                TK_ISNOT,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            binaryToUnaryIfNull(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                TK_NOTNULL,
            );
        }
        212 => {
            let ref mut fresh95 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh95 = sqlite3PExpr(
                pParse,
                TK_IS,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            binaryToUnaryIfNull(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                TK_ISNULL,
            );
        }
        213 => {
            let ref mut fresh96 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh96 = sqlite3PExpr(
                pParse,
                TK_ISNOT,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            binaryToUnaryIfNull(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                TK_NOTNULL,
            );
        }
        214 | 215 => {
            let ref mut fresh97 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh97 = sqlite3PExpr(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major as ::core::ffi::c_int,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
        }
        216 => {
            let mut p_3: *mut Expr = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
            let mut op: u8_0 = ((*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major
                as ::core::ffi::c_int
                + (TK_UPLUS - TK_PLUS)) as u8_0;
            if !p_3.is_null() && (*p_3).op as ::core::ffi::c_int == TK_UPLUS {
                (*p_3).op = op;
                let ref mut fresh98 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh98 = p_3;
            } else {
                let ref mut fresh99 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh99 = sqlite3PExpr(
                    pParse,
                    op as ::core::ffi::c_int,
                    p_3,
                    ::core::ptr::null_mut::<Expr>(),
                );
            }
        }
        217 => {
            let mut pList_2: *mut ExprList = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            pList_2 = sqlite3ExprListAppend(
                pParse,
                pList_2,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            yylhsminor.yy590 = sqlite3ExprFunction(
                pParse,
                pList_2,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                0 as ::core::ffi::c_int,
            );
            let ref mut fresh100 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh100 = yylhsminor.yy590;
        }
        218 | 221 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = 0 as ::core::ffi::c_int;
        }
        220 => {
            let mut pList_3: *mut ExprList = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            pList_3 = sqlite3ExprListAppend(
                pParse,
                pList_3,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
            let ref mut fresh101 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh101 = sqlite3PExpr(
                pParse,
                TK_BETWEEN,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
            if !(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                let ref mut fresh102 = (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .x
                    .pList;
                *fresh102 = pList_3;
            } else {
                sqlite3ExprListDelete((*pParse).db, pList_3);
            }
            if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy502
                != 0
            {
                let ref mut fresh103 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh103 = sqlite3PExpr(
                    pParse,
                    TK_NOT,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590,
                    ::core::ptr::null_mut::<Expr>(),
                );
            }
        }
        223 => {
            if (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy402
                .is_null()
            {
                let mut pB: *mut Expr = sqlite3Expr(
                    (*pParse).db,
                    TK_STRING,
                    if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy502
                        != 0
                    {
                        b"true\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"false\0" as *const u8 as *const ::core::ffi::c_char
                    },
                );
                if !pB.is_null() {
                    sqlite3ExprIdToTrueFalse(pB);
                }
                if !((*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .flags
                    & 0x8 as ::core::ffi::c_int as u32_0
                    != 0 as u32_0)
                {
                    sqlite3ExprUnmapAndDelete(
                        pParse,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590,
                    );
                    let ref mut fresh104 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh104 = pB;
                } else {
                    let ref mut fresh105 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh105 = sqlite3PExpr(
                        pParse,
                        if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy502
                            != 0
                        {
                            TK_OR
                        } else {
                            TK_AND
                        },
                        pB,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590,
                    );
                }
            } else {
                let mut pRHS: *mut Expr = (*(&raw mut (*(*yymsp
                    .offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy402)
                    .a as *mut ExprList_item)
                    .offset(0 as ::core::ffi::c_int as isize))
                .pExpr;
                if (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402)
                    .nExpr
                    == 1 as ::core::ffi::c_int
                    && sqlite3ExprIsConstant(pParse, pRHS) != 0
                    && (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590)
                        .op as ::core::ffi::c_int
                        != TK_VECTOR
                {
                    let ref mut fresh106 = (*(&raw mut (*(*yymsp
                        .offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402)
                        .a as *mut ExprList_item)
                        .offset(0 as ::core::ffi::c_int as isize))
                    .pExpr;
                    *fresh106 = ::core::ptr::null_mut::<Expr>();
                    sqlite3ExprListDelete(
                        (*pParse).db,
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy402,
                    );
                    pRHS = sqlite3PExpr(pParse, TK_UPLUS, pRHS, ::core::ptr::null_mut::<Expr>());
                    let ref mut fresh107 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh107 = sqlite3PExpr(
                        pParse,
                        TK_EQ,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590,
                        pRHS,
                    );
                } else if (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402)
                    .nExpr
                    == 1 as ::core::ffi::c_int
                    && (*pRHS).op as ::core::ffi::c_int == TK_SELECT
                {
                    let ref mut fresh108 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh108 = sqlite3PExpr(
                        pParse,
                        TK_IN,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590,
                        ::core::ptr::null_mut::<Expr>(),
                    );
                    sqlite3PExprAddSelect(
                        pParse,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590,
                        (*pRHS).x.pSelect,
                    );
                    (*pRHS).x.pSelect = ::core::ptr::null_mut::<Select>();
                    sqlite3ExprListDelete(
                        (*pParse).db,
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy402,
                    );
                } else {
                    let ref mut fresh109 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh109 = sqlite3PExpr(
                        pParse,
                        TK_IN,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590,
                        ::core::ptr::null_mut::<Expr>(),
                    );
                    if (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590
                        .is_null()
                    {
                        sqlite3ExprListDelete(
                            (*pParse).db,
                            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                                .minor
                                .yy402,
                        );
                    } else if (*(*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590)
                        .pLeft)
                        .op as ::core::ffi::c_int
                        == TK_VECTOR
                    {
                        let mut nExpr: ::core::ffi::c_int = (*(*(*(*yymsp
                            .offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590)
                            .pLeft)
                            .x
                            .pList)
                            .nExpr;
                        let mut pSelectRHS: *mut Select = sqlite3ExprListToValues(
                            pParse,
                            nExpr,
                            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                                .minor
                                .yy402,
                        );
                        if !pSelectRHS.is_null() {
                            parserDoubleLinkSelect(pParse, pSelectRHS);
                            sqlite3PExprAddSelect(
                                pParse,
                                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                                    .minor
                                    .yy590,
                                pSelectRHS,
                            );
                        }
                    } else {
                        let ref mut fresh110 = (*(*yymsp
                            .offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590)
                            .x
                            .pList;
                        *fresh110 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy402;
                        sqlite3ExprSetHeightAndFlags(
                            pParse,
                            (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                                .minor
                                .yy590,
                        );
                    }
                }
                if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502
                    != 0
                {
                    let ref mut fresh111 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh111 = sqlite3PExpr(
                        pParse,
                        TK_NOT,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590,
                        ::core::ptr::null_mut::<Expr>(),
                    );
                }
            }
        }
        224 => {
            let ref mut fresh112 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh112 = sqlite3PExpr(
                pParse,
                TK_SELECT,
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<Expr>(),
            );
            sqlite3PExprAddSelect(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637,
            );
        }
        225 => {
            let ref mut fresh113 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh113 = sqlite3PExpr(
                pParse,
                TK_IN,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
            sqlite3PExprAddSelect(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637,
            );
            if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy502
                != 0
            {
                let ref mut fresh114 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh114 = sqlite3PExpr(
                    pParse,
                    TK_NOT,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590,
                    ::core::ptr::null_mut::<Expr>(),
                );
            }
        }
        226 => {
            let mut pSrc: *mut SrcList = sqlite3SrcListAppend(
                pParse,
                ::core::ptr::null_mut::<SrcList>(),
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            let mut pSelect: *mut Select = sqlite3SelectNew(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                pSrc,
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<ExprList>(),
                0 as u32_0,
                ::core::ptr::null_mut::<Expr>(),
            );
            if !(*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy402
                .is_null()
            {
                sqlite3SrcListFuncArgs(
                    pParse,
                    if !pSelect.is_null() {
                        pSrc
                    } else {
                        ::core::ptr::null_mut::<SrcList>()
                    },
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy402,
                );
            }
            let ref mut fresh115 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh115 = sqlite3PExpr(
                pParse,
                TK_IN,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
            sqlite3PExprAddSelect(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                pSelect,
            );
            if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy502
                != 0
            {
                let ref mut fresh116 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh116 = sqlite3PExpr(
                    pParse,
                    TK_NOT,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590,
                    ::core::ptr::null_mut::<Expr>(),
                );
            }
        }
        227 => {
            let mut p_4: *mut Expr = ::core::ptr::null_mut::<Expr>();
            let ref mut fresh117 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh117 = sqlite3PExpr(
                pParse,
                TK_EXISTS,
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<Expr>(),
            );
            p_4 = *fresh117;
            sqlite3PExprAddSelect(
                pParse,
                p_4,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637,
            );
        }
        228 => {
            let ref mut fresh118 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh118 = sqlite3PExpr(
                pParse,
                TK_CASE,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
            if !(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                let ref mut fresh119 = (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .x
                    .pList;
                *fresh119 = if !(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590
                    .is_null()
                {
                    sqlite3ExprListAppend(
                        pParse,
                        (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy402,
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590,
                    )
                } else {
                    (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy402
                };
                sqlite3ExprSetHeightAndFlags(
                    pParse,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590,
                );
            } else {
                sqlite3ExprListDelete(
                    (*pParse).db,
                    (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy402,
                );
                sqlite3ExprDelete(
                    (*pParse).db,
                    (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590,
                );
            }
        }
        229 => {
            let ref mut fresh120 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh120 = sqlite3ExprListAppend(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            let ref mut fresh121 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh121 = sqlite3ExprListAppend(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        230 => {
            let ref mut fresh122 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh122 = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
            );
            let ref mut fresh123 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh123 = sqlite3ExprListAppend(
                pParse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        235 => {
            let ref mut fresh124 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh124 = sqlite3ExprListAppend(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        236 => {
            let ref mut fresh125 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy402;
            *fresh125 = sqlite3ExprListAppend(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        238 | 243 => {
            let ref mut fresh126 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh126 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
        }
        239 => {
            sqlite3CreateIndex(
                pParse,
                &raw mut (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                sqlite3SrcListAppend(
                    pParse,
                    ::core::ptr::null_mut::<SrcList>(),
                    &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0,
                    ::core::ptr::null_mut::<Token>(),
                ),
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(10 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                &raw mut (*yymsp.offset(-(11 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                SQLITE_SO_ASC,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                SQLITE_IDXTYPE_APPDEF as u8_0,
            );
            if (*pParse).eParseMode as ::core::ffi::c_int >= PARSE_MODE_RENAME
                && !(*pParse).pNewIndex.is_null()
            {
                sqlite3RenameTokenMap(
                    pParse,
                    (*(*pParse).pNewIndex).zName as *const ::core::ffi::c_void,
                    &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0,
                );
            }
        }
        240 | 282 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = OE_Abort;
        }
        241 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = OE_None;
        }
        244 => {
            let ref mut fresh127 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh127 = parserAddExprIdListTerm(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
            );
        }
        245 => {
            let ref mut fresh128 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh128 = parserAddExprIdListTerm(
                pParse,
                ::core::ptr::null_mut::<ExprList>(),
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy502,
            );
        }
        248 => {
            sqlite3DropIndex(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy563,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        249 => {
            sqlite3Vacuum(
                pParse,
                ::core::ptr::null_mut::<Token>(),
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        250 => {
            sqlite3Vacuum(
                pParse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        253 => {
            sqlite3Pragma(
                pParse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                ::core::ptr::null_mut::<Token>(),
                0 as ::core::ffi::c_int,
            );
        }
        254 => {
            sqlite3Pragma(
                pParse,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                0 as ::core::ffi::c_int,
            );
        }
        255 => {
            sqlite3Pragma(
                pParse,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                0 as ::core::ffi::c_int,
            );
        }
        256 => {
            sqlite3Pragma(
                pParse,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                1 as ::core::ffi::c_int,
            );
        }
        257 => {
            sqlite3Pragma(
                pParse,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                1 as ::core::ffi::c_int,
            );
        }
        260 => {
            let mut all: Token = Token {
                z: ::core::ptr::null::<::core::ffi::c_char>(),
                n: 0,
            };
            all.z = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .z;
            all.n = ((*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy0
                .z
                .offset_from(
                    (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0
                        .z,
                ) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add(
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .n,
                );
            sqlite3FinishTrigger(
                pParse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy319,
                &raw mut all,
            );
        }
        261 => {
            sqlite3BeginTrigger(
                pParse,
                &raw mut (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy28
                    .a,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy28
                    .b,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(10 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
            (*yymsp.offset(-(10 as ::core::ffi::c_int) as isize))
                .minor
                .yy0 = if (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n
                == 0 as ::core::ffi::c_uint
            {
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
            } else {
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
            };
        }
        262 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 =
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).major as ::core::ffi::c_int;
        }
        263 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = TK_INSTEAD;
        }
        264 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = TK_BEFORE;
        }
        265 | 266 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy28
                .a = (*yymsp.offset(0 as ::core::ffi::c_int as isize)).major as ::core::ffi::c_int;
            let ref mut fresh129 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy28
                .b;
            *fresh129 = ::core::ptr::null_mut::<IdList>();
        }
        267 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy28
                .a = TK_UPDATE;
            let ref mut fresh130 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy28
                .b;
            *fresh130 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy204;
        }
        268 | 287 => {
            let ref mut fresh131 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
            *fresh131 = ::core::ptr::null_mut::<Expr>();
        }
        269 | 288 => {
            let ref mut fresh132 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh132 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
        }
        270 => {
            let ref mut fresh133 = (*(*(*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy319)
                .pLast)
                .pNext;
            *fresh133 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
            let ref mut fresh134 = (*(*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy319)
                .pLast;
            *fresh134 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
        }
        271 => {
            let ref mut fresh135 = (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy319)
                .pLast;
            *fresh135 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
        }
        272 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy0 = (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0;
            sqlite3ErrorMsg(
                pParse,
                b"qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        273 => {
            sqlite3ErrorMsg(
                pParse,
                b"the INDEXED BY clause is not allowed on UPDATE or DELETE statements within triggers\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        274 => {
            sqlite3ErrorMsg(
                pParse,
                b"the NOT INDEXED clause is not allowed on UPDATE or DELETE statements within triggers\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        275 => {
            yylhsminor.yy319 = sqlite3TriggerUpdateStep(
                pParse,
                &raw mut (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as u8_0,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy342,
            );
            let ref mut fresh136 = (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
            *fresh136 = yylhsminor.yy319;
        }
        276 => {
            yylhsminor.yy319 = sqlite3TriggerInsertStep(
                pParse,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as u8_0,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy403,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy342,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy342,
            );
            let ref mut fresh137 = (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
            *fresh137 = yylhsminor.yy319;
        }
        277 => {
            yylhsminor.yy319 = sqlite3TriggerDeleteStep(
                pParse,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy342,
            );
            let ref mut fresh138 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
            *fresh138 = yylhsminor.yy319;
        }
        278 => {
            yylhsminor.yy319 = sqlite3TriggerSelectStep(
                (*pParse).db,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy342,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy342,
            );
            let ref mut fresh139 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
            *fresh139 = yylhsminor.yy319;
        }
        279 => {
            let ref mut fresh140 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh140 = sqlite3PExpr(
                pParse,
                TK_RAISE,
                ::core::ptr::null_mut::<Expr>(),
                ::core::ptr::null_mut::<Expr>(),
            );
            if !(*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                (*(*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .affExpr = OE_Ignore as ::core::ffi::c_char;
            }
        }
        280 => {
            let ref mut fresh141 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh141 = sqlite3PExpr(
                pParse,
                TK_RAISE,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                ::core::ptr::null_mut::<Expr>(),
            );
            if !(*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                (*(*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .affExpr = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as ::core::ffi::c_char;
            }
        }
        281 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = OE_Rollback;
        }
        283 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy502 = OE_Fail;
        }
        284 => {
            sqlite3DropTrigger(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy563,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        285 => {
            sqlite3Attach(
                pParse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        286 => {
            sqlite3Detach(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590,
            );
        }
        289 => {
            sqlite3Reindex(
                pParse,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Token>(),
            );
        }
        290 => {
            sqlite3Reindex(
                pParse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        291 => {
            sqlite3Analyze(
                pParse,
                ::core::ptr::null_mut::<Token>(),
                ::core::ptr::null_mut::<Token>(),
            );
        }
        292 => {
            sqlite3Analyze(
                pParse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        293 => {
            sqlite3AlterRenameTable(
                pParse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        294 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n = ((*pParse).sLastToken.z.offset_from(
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
            ) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add((*pParse).sLastToken.n);
            sqlite3AlterFinishAddColumn(
                pParse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
        }
        295 => {
            sqlite3AlterDropColumn(
                pParse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        296 => {
            disableLookaside(pParse);
            sqlite3AlterBeginAddColumn(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy563,
            );
        }
        297 => {
            sqlite3AlterRenameColumn(
                pParse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        298 => {
            sqlite3VtabFinishParse(pParse, ::core::ptr::null_mut::<Token>());
        }
        299 => {
            sqlite3VtabFinishParse(
                pParse,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        300 => {
            sqlite3VtabBeginParse(
                pParse,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        301 => {
            sqlite3VtabArgInit(pParse);
        }
        302 | 303 | 304 => {
            sqlite3VtabArgExtend(
                pParse,
                &raw mut (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
        }
        305 | 306 => {
            sqlite3WithPush(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy125,
                1 as u8_0,
            );
        }
        307 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy444 = M10d_Any as u8_0;
        }
        308 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy444 = M10d_Yes as u8_0;
        }
        309 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy444 = M10d_No as u8_0;
        }
        310 => {
            let ref mut fresh142 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy361;
            *fresh142 = sqlite3CteNew(
                pParse,
                &raw mut (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy444,
            );
        }
        311 => {
            (*pParse).set_bHasWith(1 as bft as bft);
        }
        312 => {
            let ref mut fresh143 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy125;
            *fresh143 = sqlite3WithAdd(
                pParse,
                ::core::ptr::null_mut::<With>(),
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy361,
            );
        }
        313 => {
            let ref mut fresh144 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy125;
            *fresh144 = sqlite3WithAdd(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy125,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy361,
            );
        }
        314 => {
            sqlite3WindowChain(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy483,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy483,
            );
            let ref mut fresh145 = (*(*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy483)
                .pNextWin;
            *fresh145 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            yylhsminor.yy483 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy483;
            let ref mut fresh146 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh146 = yylhsminor.yy483;
        }
        315 => {
            if !(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483
                .is_null()
            {
                let ref mut fresh147 = (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy483)
                    .zName;
                *fresh147 = sqlite3DbStrNDup(
                    (*pParse).db,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0
                        .z,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0
                        .n as u64_0,
                );
            }
            yylhsminor.yy483 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            let ref mut fresh148 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh148 = yylhsminor.yy483;
        }
        316 => {
            let ref mut fresh149 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh149 = sqlite3WindowAssemble(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy483,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                ::core::ptr::null_mut::<Token>(),
            );
        }
        317 => {
            yylhsminor.yy483 = sqlite3WindowAssemble(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy483,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                &raw mut (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            let ref mut fresh150 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh150 = yylhsminor.yy483;
        }
        318 => {
            let ref mut fresh151 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh151 = sqlite3WindowAssemble(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy483,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                ::core::ptr::null_mut::<Token>(),
            );
        }
        319 => {
            yylhsminor.yy483 = sqlite3WindowAssemble(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy483,
                ::core::ptr::null_mut::<ExprList>(),
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            let ref mut fresh152 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh152 = yylhsminor.yy483;
        }
        320 => {
            yylhsminor.yy483 = sqlite3WindowAssemble(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy483,
                ::core::ptr::null_mut::<ExprList>(),
                ::core::ptr::null_mut::<ExprList>(),
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            let ref mut fresh153 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh153 = yylhsminor.yy483;
        }
        321 => {
            let ref mut fresh154 = (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy483;
            *fresh154 = sqlite3WindowAlloc(
                pParse,
                0 as ::core::ffi::c_int,
                TK_UNBOUNDED,
                ::core::ptr::null_mut::<Expr>(),
                TK_CURRENT,
                ::core::ptr::null_mut::<Expr>(),
                0 as u8_0,
            );
        }
        322 => {
            yylhsminor.yy483 = sqlite3WindowAlloc(
                pParse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .eType,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .pExpr,
                TK_CURRENT,
                ::core::ptr::null_mut::<Expr>(),
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy444,
            );
            let ref mut fresh155 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh155 = yylhsminor.yy483;
        }
        323 => {
            yylhsminor.yy483 = sqlite3WindowAlloc(
                pParse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .eType,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .pExpr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .eType,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .pExpr,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy444,
            );
            let ref mut fresh156 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh156 = yylhsminor.yy483;
        }
        325 | 327 => {
            yylhsminor.yy205 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy205;
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy205 = yylhsminor.yy205;
        }
        326 | 328 | 330 => {
            yylhsminor.yy205.eType =
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major as ::core::ffi::c_int;
            yylhsminor.yy205.pExpr = ::core::ptr::null_mut::<Expr>();
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy205 = yylhsminor.yy205;
        }
        329 => {
            yylhsminor.yy205.eType =
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).major as ::core::ffi::c_int;
            yylhsminor.yy205.pExpr = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy205 = yylhsminor.yy205;
        }
        331 => {
            (*yymsp.offset(1 as ::core::ffi::c_int as isize))
                .minor
                .yy444 = 0 as u8_0;
        }
        332 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy444 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy444;
        }
        333 | 334 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy444 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major as u8_0;
        }
        335 => {
            (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy444 = (*yymsp.offset(0 as ::core::ffi::c_int as isize)).major as u8_0;
        }
        336 => {
            let ref mut fresh157 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh157 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy483;
        }
        337 => {
            if !(*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy483
                .is_null()
            {
                let ref mut fresh158 = (*(*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy483)
                    .pFilter;
                *fresh158 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
            } else {
                sqlite3ExprDelete(
                    (*pParse).db,
                    (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590,
                );
            }
            yylhsminor.yy483 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy483;
            let ref mut fresh159 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh159 = yylhsminor.yy483;
        }
        338 => {
            yylhsminor.yy483 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy483;
            let ref mut fresh160 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy483;
            *fresh160 = yylhsminor.yy483;
        }
        339 => {
            yylhsminor.yy483 =
                sqlite3DbMallocZero((*pParse).db, ::core::mem::size_of::<Window>() as u64_0)
                    as *mut Window;
            if !yylhsminor.yy483.is_null() {
                (*yylhsminor.yy483).eFrmType = TK_FILTER as u8_0;
                (*yylhsminor.yy483).pFilter = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                    .minor
                    .yy590;
            } else {
                sqlite3ExprDelete(
                    (*pParse).db,
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy590,
                );
            }
            let ref mut fresh161 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy483;
            *fresh161 = yylhsminor.yy483;
        }
        340 => {
            let ref mut fresh162 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh162 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
        }
        341 => {
            let ref mut fresh163 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh163 = sqlite3DbMallocZero((*pParse).db, ::core::mem::size_of::<Window>() as u64_0)
                as *mut Window;
            if !(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483
                .is_null()
            {
                let ref mut fresh164 = (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy483)
                    .zName;
                *fresh164 = sqlite3DbStrNDup(
                    (*pParse).db,
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .z,
                    (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                        .minor
                        .yy0
                        .n as u64_0,
                );
            }
        }
        342 => {
            let ref mut fresh165 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh165 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
        }
        343 => {
            yylhsminor.yy590 = tokenExpr(
                pParse,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).major as ::core::ffi::c_int,
                (*yymsp.offset(0 as ::core::ffi::c_int as isize)).minor.yy0,
            );
            sqlite3DequoteNumber(pParse, yylhsminor.yy590);
            let ref mut fresh166 = (*yymsp.offset(0 as ::core::ffi::c_int as isize))
                .minor
                .yy590;
            *fresh166 = yylhsminor.yy590;
        }
        _ => {}
    }
    yygoto = yyRuleInfoLhs[yyruleno as usize] as ::core::ffi::c_int;
    yysize = yyRuleInfoNRhs[yyruleno as usize] as ::core::ffi::c_int;
    yyact = yy_find_reduce_action(
        (*yymsp.offset(yysize as isize)).stateno,
        yygoto as ::core::ffi::c_ushort,
    );
    yymsp = yymsp.offset((yysize + 1 as ::core::ffi::c_int) as isize);
    (*yypParser).yytos = yymsp;
    (*yymsp).stateno = yyact;
    (*yymsp).major = yygoto as ::core::ffi::c_ushort;
    return yyact;
}
unsafe extern "C" fn yy_syntax_error(
    mut yypParser: *mut yyParser,
    mut _yymajor: ::core::ffi::c_int,
    mut yyminor: Token,
) {
    let mut pParse: *mut Parse = (*yypParser).pParse;
    if *yyminor.z.offset(0 as ::core::ffi::c_int as isize) != 0 {
        parserSyntaxError(pParse, &raw mut yyminor);
    } else {
        sqlite3ErrorMsg(
            pParse,
            b"incomplete input\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*yypParser).pParse = pParse;
}
unsafe extern "C" fn yy_accept(mut yypParser: *mut yyParser) {
    let mut pParse: *mut Parse = (*yypParser).pParse;
    (*yypParser).pParse = pParse;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Parser(
    mut yyp: *mut ::core::ffi::c_void,
    mut yymajor: ::core::ffi::c_int,
    mut yyminor: Token,
) {
    let mut yyminorunion: YYMINORTYPE = YYMINORTYPE { yyinit: 0 };
    let mut yyact: ::core::ffi::c_ushort = 0;
    let mut yypParser: *mut yyParser = yyp as *mut yyParser;
    let mut pParse: *mut Parse = (*yypParser).pParse;
    yyact = (*(*yypParser).yytos).stateno;
    loop {
        yyact = yy_find_shift_action(yymajor as ::core::ffi::c_ushort, yyact);
        if yyact as ::core::ffi::c_int >= YY_MIN_REDUCE {
            let mut yyruleno: ::core::ffi::c_uint =
                (yyact as ::core::ffi::c_int - YY_MIN_REDUCE) as ::core::ffi::c_uint;
            if yyRuleInfoNRhs[yyruleno as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if (*yypParser).yytos >= (*yypParser).yystackEnd {
                    if yyGrowStack(yypParser) != 0 {
                        yyStackOverflow(yypParser);
                        break;
                    }
                }
            }
            yyact = yy_reduce(yypParser, yyruleno, yymajor, yyminor, pParse);
        } else if yyact as ::core::ffi::c_int <= YY_MAX_SHIFTREDUCE {
            yy_shift(yypParser, yyact, yymajor as ::core::ffi::c_ushort, yyminor);
            break;
        } else if yyact as ::core::ffi::c_int == YY_ACCEPT_ACTION {
            (*yypParser).yytos = (*yypParser).yytos.offset(-1);
            yy_accept(yypParser);
            return;
        } else {
            yyminorunion.yy0 = yyminor;
            yy_syntax_error(yypParser, yymajor, yyminor);
            yy_destructor(
                yypParser,
                yymajor as ::core::ffi::c_ushort,
                &raw mut yyminorunion,
            );
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3ParserFallback(
    mut iToken: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return yyFallback[iToken as usize] as ::core::ffi::c_int;
}
