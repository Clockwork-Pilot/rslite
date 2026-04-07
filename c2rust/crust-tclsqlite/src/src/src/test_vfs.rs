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
    pub type Tcl_Command_;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(
        _: *mut sqlite3_vfs,
        makeDflt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vfs_unregister(_: *mut sqlite3_vfs) -> ::core::ffi::c_int;
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        op: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3OsClose(_: *mut sqlite3_file);
    fn sqlite3OsRead(
        _: *mut sqlite3_file,
        _: *mut ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsWrite(
        _: *mut sqlite3_file,
        _: *const ::core::ffi::c_void,
        amt: ::core::ffi::c_int,
        offset: i64_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsTruncate(_: *mut sqlite3_file, size: i64_0) -> ::core::ffi::c_int;
    fn sqlite3OsSync(_: *mut sqlite3_file, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3OsFileSize(_: *mut sqlite3_file, pSize: *mut i64_0) -> ::core::ffi::c_int;
    fn sqlite3OsLock(_: *mut sqlite3_file, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3OsUnlock(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsCheckReservedLock(
        id: *mut sqlite3_file,
        pResOut: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsFileControl(
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsSectorSize(id: *mut sqlite3_file) -> ::core::ffi::c_int;
    fn sqlite3OsDeviceCharacteristics(id: *mut sqlite3_file) -> ::core::ffi::c_int;
    fn sqlite3OsFetch(
        id: *mut sqlite3_file,
        _: i64_0,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsUnfetch(
        _: *mut sqlite3_file,
        _: i64_0,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsOpen(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: *mut sqlite3_file,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsDelete(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsAccess(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        pResOut: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsFullPathname(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsDlOpen(
        _: *mut sqlite3_vfs,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3OsDlError(
        _: *mut sqlite3_vfs,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    );
    fn sqlite3OsDlSym(
        _: *mut sqlite3_vfs,
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
    ) -> Option<unsafe extern "C" fn() -> ()>;
    fn sqlite3OsDlClose(_: *mut sqlite3_vfs, _: *mut ::core::ffi::c_void);
    fn sqlite3OsRandomness(
        _: *mut sqlite3_vfs,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3OsSleep(_: *mut sqlite3_vfs, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn Tcl_Alloc(size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char;
    fn Tcl_Free(ptr: *mut ::core::ffi::c_char);
    fn Tcl_DuplicateObj(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_GetBooleanFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetByteArrayFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_uchar;
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
    fn Tcl_ListObjGetElements(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objcPtr: *mut ::core::ffi::c_int,
        objvPtr: *mut *mut *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewByteArrayObj(
        bytes: *const ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_NewStringObj(
        bytes: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_BackgroundError(interp: *mut Tcl_Interp);
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
    fn Tcl_ResetResult(interp: *mut Tcl_Interp);
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_AppendObjToObj(objPtr: *mut Tcl_Obj, appendObjPtr: *mut Tcl_Obj);
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
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    static sqlite3CtypeMap: [::core::ffi::c_uchar; 0];
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
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Testvfs {
    pub zName: *mut ::core::ffi::c_char,
    pub pParent: *mut sqlite3_vfs,
    pub pVfs: *mut sqlite3_vfs,
    pub interp: *mut Tcl_Interp,
    pub pScript: *mut Tcl_Obj,
    pub pBuffer: *mut TestvfsBuffer,
    pub isNoshm: ::core::ffi::c_int,
    pub isFullshm: ::core::ffi::c_int,
    pub mask: ::core::ffi::c_int,
    pub ioerr_err: TestFaultInject,
    pub full_err: TestFaultInject,
    pub cantopen_err: TestFaultInject,
    pub iDevchar: ::core::ffi::c_int,
    pub iSectorsize: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestFaultInject {
    pub iCnt: ::core::ffi::c_int,
    pub eFault: ::core::ffi::c_int,
    pub nFail: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestvfsBuffer {
    pub zFile: *mut ::core::ffi::c_char,
    pub pgsz: ::core::ffi::c_int,
    pub aPage: [*mut u8_0; 1024],
    pub pFile: *mut TestvfsFd,
    pub pNext: *mut TestvfsBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestvfsFd {
    pub pVfs: *mut sqlite3_vfs,
    pub zFilename: *const ::core::ffi::c_char,
    pub pReal: *mut sqlite3_file,
    pub pShmId: *mut Tcl_Obj,
    pub pShm: *mut TestvfsBuffer,
    pub excllock: u32_0,
    pub sharedlock: u32_0,
    pub pNext: *mut TestvfsFd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestvfsFile {
    pub base: sqlite3_file,
    pub pFd: *mut TestvfsFd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct errcode {
    pub eCode: ::core::ffi::c_int,
    pub zCode: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fcntl {
    pub iFnctl: ::core::ffi::c_int,
    pub zFnctl: *const ::core::ffi::c_char,
}
pub const CMD_SECTORSIZE: DB_enum = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceFlag {
    pub zName: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int,
}
pub const CMD_DEVCHAR: DB_enum = 5;
pub const CMD_DELETE: DB_enum = 1;
pub const CMD_CANTOPENERR: DB_enum = 8;
pub const CMD_FULLERR: DB_enum = 7;
pub const CMD_IOERR: DB_enum = 3;
pub type DB_enum = ::core::ffi::c_uint;
pub const CMD_SCRIPT: DB_enum = 4;
pub const CMD_FILTER: DB_enum = 2;
pub const CMD_SHM: DB_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestvfsSubcmd {
    pub zName: *mut ::core::ffi::c_char,
    pub eCmd: DB_enum,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfsMethod {
    pub zName: *mut ::core::ffi::c_char,
    pub mask: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_LOCKED: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_FULL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SQLITE_CANTOPEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_IOERR_UNLOCK: ::core::ffi::c_int = SQLITE_IOERR
    | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_IOERR_LOCK: ::core::ffi::c_int = SQLITE_IOERR
    | (15 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_READONLY_CANTINIT: ::core::ffi::c_int = SQLITE_READONLY
    | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC512: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC1K: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC2K: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC4K: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC8K: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC16K: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC32K: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_ATOMIC64K: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SAFE_APPEND: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_SEQUENTIAL: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN: ::core::ffi::c_int = 0x800
    as ::core::ffi::c_int;
pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 0x1000
    as ::core::ffi::c_int;
pub const SQLITE_IOCAP_IMMUTABLE: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SQLITE_SYNC_NORMAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_SYNC_FULL: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const SQLITE_SYNC_DATAONLY: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_FILE_POINTER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_PRAGMA: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_ZIPVFS: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_BEGIN_ATOMIC_WRITE: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_COMMIT_ATOMIC_WRITE: ::core::ffi::c_int = 32
    as ::core::ffi::c_int;
pub const SQLITE_ACCESS_EXISTS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ACCESS_READWRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ACCESS_READ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_SHM_UNLOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_SHM_LOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_SHM_SHARED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_SHM_EXCLUSIVE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_EVAL_GLOBAL: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const FAULT_INJECT_TRANSIENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FAULT_INJECT_PERSISTENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TESTVFS_SHMOPEN_MASK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TESTVFS_SHMLOCK_MASK: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TESTVFS_SHMMAP_MASK: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const TESTVFS_SHMBARRIER_MASK: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const TESTVFS_SHMCLOSE_MASK: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const TESTVFS_OPEN_MASK: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const TESTVFS_SYNC_MASK: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const TESTVFS_DELETE_MASK: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const TESTVFS_CLOSE_MASK: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const TESTVFS_WRITE_MASK: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const TESTVFS_TRUNCATE_MASK: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const TESTVFS_ACCESS_MASK: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const TESTVFS_FULLPATHNAME_MASK: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const TESTVFS_READ_MASK: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const TESTVFS_UNLOCK_MASK: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const TESTVFS_LOCK_MASK: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const TESTVFS_CKLOCK_MASK: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const TESTVFS_FCNTL_MASK: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const TESTVFS_SLEEP_MASK: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
pub const TESTVFS_ALL_MASK: ::core::ffi::c_int = 0x3fffff as ::core::ffi::c_int;
static mut tvfs_io_methods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 3 as ::core::ffi::c_int,
        xClose: Some(
            tvfsClose as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            tvfsRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            tvfsWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            tvfsTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            tvfsSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            tvfsFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            tvfsLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            tvfsUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            tvfsCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            tvfsFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            tvfsSectorSize
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            tvfsDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: Some(
            tvfsShmMap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xShmLock: Some(
            tvfsShmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShmBarrier: Some(
            tvfsShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
        ),
        xShmUnmap: Some(
            tvfsShmUnmap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFetch: Some(
            tvfsFetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xUnfetch: Some(
            tvfsUnfetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn tvfsResultCode(
    mut p: *mut Testvfs,
    mut pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aCode: [errcode; 9] = [
            errcode {
                eCode: SQLITE_OK,
                zCode: b"SQLITE_OK\0".as_ptr() as *const ::core::ffi::c_char,
            },
            errcode {
                eCode: SQLITE_ERROR,
                zCode: b"SQLITE_ERROR\0".as_ptr() as *const ::core::ffi::c_char,
            },
            errcode {
                eCode: SQLITE_IOERR,
                zCode: b"SQLITE_IOERR\0".as_ptr() as *const ::core::ffi::c_char,
            },
            errcode {
                eCode: SQLITE_LOCKED,
                zCode: b"SQLITE_LOCKED\0".as_ptr() as *const ::core::ffi::c_char,
            },
            errcode {
                eCode: SQLITE_BUSY,
                zCode: b"SQLITE_BUSY\0".as_ptr() as *const ::core::ffi::c_char,
            },
            errcode {
                eCode: SQLITE_READONLY,
                zCode: b"SQLITE_READONLY\0".as_ptr() as *const ::core::ffi::c_char,
            },
            errcode {
                eCode: SQLITE_READONLY_CANTINIT,
                zCode: b"SQLITE_READONLY_CANTINIT\0".as_ptr()
                    as *const ::core::ffi::c_char,
            },
            errcode {
                eCode: SQLITE_NOTFOUND,
                zCode: b"SQLITE_NOTFOUND\0".as_ptr() as *const ::core::ffi::c_char,
            },
            errcode {
                eCode: -1 as ::core::ffi::c_int,
                zCode: b"SQLITE_OMIT\0".as_ptr() as *const ::core::ffi::c_char,
            },
        ];
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut i: ::core::ffi::c_int = 0;
        z = Tcl_GetStringResult((*p).interp);
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[errcode; 9]>() as usize)
                .wrapping_div(::core::mem::size_of::<errcode>() as usize)
                as ::core::ffi::c_int
        {
            if 0 as ::core::ffi::c_int == strcmp(z, aCode[i as usize].zCode) {
                *pRc = aCode[i as usize].eCode;
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn tvfsInjectFault(mut p: *mut TestFaultInject) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*p).eFault != 0 {
            (*p).iCnt -= 1;
            if (*p).iCnt == 0 as ::core::ffi::c_int
                || (*p).iCnt < 0 as ::core::ffi::c_int
                    && (*p).eFault == FAULT_INJECT_PERSISTENT
            {
                ret = 1 as ::core::ffi::c_int;
                (*p).nFail += 1;
            }
        }
        return ret;
    }
}
unsafe extern "C" fn tvfsInjectIoerr(mut p: *mut Testvfs) -> ::core::ffi::c_int {
    unsafe {
        return tvfsInjectFault(&raw mut (*p).ioerr_err);
    }
}
unsafe extern "C" fn tvfsInjectFullerr(mut p: *mut Testvfs) -> ::core::ffi::c_int {
    unsafe {
        return tvfsInjectFault(&raw mut (*p).full_err);
    }
}
unsafe extern "C" fn tvfsInjectCantopenerr(mut p: *mut Testvfs) -> ::core::ffi::c_int {
    unsafe {
        return tvfsInjectFault(&raw mut (*p).cantopen_err);
    }
}
unsafe extern "C" fn tvfsExecTcl(
    mut p: *mut Testvfs,
    mut zMethod: *const ::core::ffi::c_char,
    mut arg1: *mut Tcl_Obj,
    mut arg2: *mut Tcl_Obj,
    mut arg3: *mut Tcl_Obj,
    mut arg4: *mut Tcl_Obj,
) {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        pEval = Tcl_DuplicateObj((*p).pScript);
        (*(*p).pScript).refCount += 1;
        Tcl_ListObjAppendElement(
            (*p).interp,
            pEval,
            Tcl_NewStringObj(zMethod, -1 as ::core::ffi::c_int),
        );
        if !arg1.is_null() {
            Tcl_ListObjAppendElement((*p).interp, pEval, arg1);
        }
        if !arg2.is_null() {
            Tcl_ListObjAppendElement((*p).interp, pEval, arg2);
        }
        if !arg3.is_null() {
            Tcl_ListObjAppendElement((*p).interp, pEval, arg3);
        }
        if !arg4.is_null() {
            Tcl_ListObjAppendElement((*p).interp, pEval, arg4);
        }
        rc = Tcl_EvalObjEx((*p).interp, pEval, TCL_EVAL_GLOBAL);
        if rc != TCL_OK {
            Tcl_BackgroundError((*p).interp);
            Tcl_ResetResult((*p).interp);
        }
    }
}
unsafe extern "C" fn tvfsClose(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut pTestfile: *mut TestvfsFile = pFile as *mut TestvfsFile;
        let mut pFd: *mut TestvfsFd = (*pTestfile).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_CLOSE_MASK != 0 {
            tvfsExecTcl(
                p,
                b"xClose\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                (*pFd).pShmId,
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
        }
        if !(*pFd).pShmId.is_null() {
            let mut _objPtr: *mut Tcl_Obj = (*pFd).pShmId;
            let c2rust_fresh0 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh0 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
            (*pFd).pShmId = ::core::ptr::null_mut::<Tcl_Obj>();
        }
        if !(*pFile).pMethods.is_null() {
            Tcl_Free((*pFile).pMethods as *mut ::core::ffi::c_char);
        }
        sqlite3OsClose((*pFd).pReal);
        Tcl_Free(pFd as *mut ::core::ffi::c_char);
        (*pTestfile).pFd = ::core::ptr::null_mut::<TestvfsFd>();
        return SQLITE_OK;
    }
}
unsafe extern "C" fn tvfsRead(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *mut ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_READ_MASK != 0 {
            tvfsExecTcl(
                p,
                b"xRead\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                (*pFd).pShmId,
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            tvfsResultCode(p, &raw mut rc);
        }
        if rc == SQLITE_OK && (*p).mask & TESTVFS_READ_MASK != 0
            && tvfsInjectIoerr(p) != 0
        {
            rc = SQLITE_IOERR;
        }
        if rc == SQLITE_OK {
            rc = sqlite3OsRead((*pFd).pReal, zBuf, iAmt, iOfst as i64_0);
        }
        return rc;
    }
}
unsafe extern "C" fn tvfsWrite(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *const ::core::ffi::c_void,
    mut iAmt: ::core::ffi::c_int,
    mut iOfst: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_WRITE_MASK != 0 {
            tvfsExecTcl(
                p,
                b"xWrite\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                (*pFd).pShmId,
                Tcl_NewWideIntObj(iOfst as Tcl_WideInt),
                Tcl_NewIntObj(iAmt),
            );
            tvfsResultCode(p, &raw mut rc);
            if rc < 0 as ::core::ffi::c_int {
                return SQLITE_OK;
            }
        }
        if rc == SQLITE_OK && tvfsInjectFullerr(p) != 0 {
            rc = SQLITE_FULL;
        }
        if rc == SQLITE_OK && (*p).mask & TESTVFS_WRITE_MASK != 0
            && tvfsInjectIoerr(p) != 0
        {
            rc = SQLITE_IOERR;
        }
        if rc == SQLITE_OK {
            rc = sqlite3OsWrite((*pFd).pReal, zBuf, iAmt, iOfst as i64_0);
        }
        return rc;
    }
}
unsafe extern "C" fn tvfsTruncate(
    mut pFile: *mut sqlite3_file,
    mut size: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_TRUNCATE_MASK != 0 {
            tvfsExecTcl(
                p,
                b"xTruncate\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                (*pFd).pShmId,
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            tvfsResultCode(p, &raw mut rc);
        }
        if rc == SQLITE_OK {
            rc = sqlite3OsTruncate((*pFd).pReal, size as i64_0);
        }
        return rc;
    }
}
unsafe extern "C" fn tvfsSync(
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_SYNC_MASK != 0 {
            let mut zFlags: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            match flags {
                SQLITE_SYNC_NORMAL => {
                    zFlags = b"normal\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                }
                SQLITE_SYNC_FULL => {
                    zFlags = b"full\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                }
                18 => {
                    zFlags = b"normal|dataonly\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                }
                19 => {
                    zFlags = b"full|dataonly\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                }
                _ => {}
            }
            tvfsExecTcl(
                p,
                b"xSync\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                (*pFd).pShmId,
                Tcl_NewStringObj(zFlags, -1 as ::core::ffi::c_int),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            tvfsResultCode(p, &raw mut rc);
        }
        if rc == SQLITE_OK && tvfsInjectFullerr(p) != 0 {
            rc = SQLITE_FULL;
        }
        if rc == SQLITE_OK {
            rc = sqlite3OsSync((*pFd).pReal, flags);
        }
        return rc;
    }
}
unsafe extern "C" fn tvfsFileSize(
    mut pFile: *mut sqlite3_file,
    mut pSize: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        return sqlite3OsFileSize((*p).pReal, pSize as *mut i64_0);
    }
}
unsafe extern "C" fn tvfsLock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_LOCK_MASK != 0 {
            let mut zLock: [::core::ffi::c_char; 30] = [0; 30];
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 30]>()
                    as ::core::ffi::c_int,
                &raw mut zLock as *mut ::core::ffi::c_char,
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                eLock,
            );
            tvfsExecTcl(
                p,
                b"xLock\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                Tcl_NewStringObj(
                    &raw mut zLock as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
        }
        if (*p).mask & TESTVFS_LOCK_MASK != 0 && tvfsInjectIoerr(p) != 0 {
            return SQLITE_IOERR_LOCK;
        }
        return sqlite3OsLock((*pFd).pReal, eLock);
    }
}
unsafe extern "C" fn tvfsUnlock(
    mut pFile: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_UNLOCK_MASK != 0 {
            let mut zLock: [::core::ffi::c_char; 30] = [0; 30];
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 30]>()
                    as ::core::ffi::c_int,
                &raw mut zLock as *mut ::core::ffi::c_char,
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                eLock,
            );
            tvfsExecTcl(
                p,
                b"xUnlock\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                Tcl_NewStringObj(
                    &raw mut zLock as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
        }
        if (*p).mask & TESTVFS_UNLOCK_MASK != 0 && tvfsInjectIoerr(p) != 0 {
            return SQLITE_IOERR_UNLOCK;
        }
        return sqlite3OsUnlock((*pFd).pReal, eLock);
    }
}
unsafe extern "C" fn tvfsCheckReservedLock(
    mut pFile: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_CKLOCK_MASK != 0 {
            tvfsExecTcl(
                p,
                b"xCheckReservedLock\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
        }
        return sqlite3OsCheckReservedLock((*pFd).pReal, pResOut);
    }
}
unsafe extern "C" fn tvfsFileControl(
    mut pFile: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if op == SQLITE_FCNTL_PRAGMA {
            let mut argv: *mut *mut ::core::ffi::c_char = pArg
                as *mut *mut ::core::ffi::c_char;
            if sqlite3_stricmp(
                *argv.offset(1 as ::core::ffi::c_int as isize),
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut rc: ::core::ffi::c_int = SQLITE_ERROR;
                if !(*argv.offset(2 as ::core::ffi::c_int as isize)).is_null() {
                    let mut z: *const ::core::ffi::c_char = *argv
                        .offset(2 as ::core::ffi::c_int as isize);
                    let mut x: ::core::ffi::c_int = atoi(z);
                    if x != 0 {
                        rc = x;
                        while *(&raw const sqlite3CtypeMap
                            as *const ::core::ffi::c_uchar)
                            .offset(
                                *z.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                        {
                            z = z.offset(1);
                        }
                        while *(&raw const sqlite3CtypeMap
                            as *const ::core::ffi::c_uchar)
                            .offset(
                                *z.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uchar as isize,
                            ) as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                        {
                            z = z.offset(1);
                        }
                    }
                    if *z.offset(0 as ::core::ffi::c_int as isize) != 0 {
                        let ref mut c2rust_fresh1 = *argv
                            .offset(0 as ::core::ffi::c_int as isize);
                        *c2rust_fresh1 = sqlite3_mprintf(
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            z,
                        );
                    }
                }
                return rc;
            }
            if sqlite3_stricmp(
                *argv.offset(1 as ::core::ffi::c_int as isize),
                b"filename\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let ref mut c2rust_fresh2 = *argv
                    .offset(0 as ::core::ffi::c_int as isize);
                *c2rust_fresh2 = sqlite3_mprintf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pFd).zFilename,
                );
                return SQLITE_OK;
            }
        }
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_FCNTL_MASK != 0 {
            let mut aF: [Fcntl; 3] = [
                Fcntl {
                    iFnctl: SQLITE_FCNTL_BEGIN_ATOMIC_WRITE,
                    zFnctl: b"BEGIN_ATOMIC_WRITE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                },
                Fcntl {
                    iFnctl: SQLITE_FCNTL_COMMIT_ATOMIC_WRITE,
                    zFnctl: b"COMMIT_ATOMIC_WRITE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                },
                Fcntl {
                    iFnctl: SQLITE_FCNTL_ZIPVFS,
                    zFnctl: b"ZIPVFS\0".as_ptr() as *const ::core::ffi::c_char,
                },
            ];
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while (i as usize)
                < (::core::mem::size_of::<[Fcntl; 3]>() as usize)
                    .wrapping_div(::core::mem::size_of::<Fcntl>() as usize)
            {
                if op == aF[i as usize].iFnctl {
                    break;
                }
                i += 1;
            }
            if (i as usize)
                < (::core::mem::size_of::<[Fcntl; 3]>() as usize)
                    .wrapping_div(::core::mem::size_of::<Fcntl>() as usize)
            {
                let mut rc_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                tvfsExecTcl(
                    p,
                    b"xFileControl\0".as_ptr() as *const ::core::ffi::c_char,
                    Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                    Tcl_NewStringObj(aF[i as usize].zFnctl, -1 as ::core::ffi::c_int),
                    ::core::ptr::null_mut::<Tcl_Obj>(),
                    ::core::ptr::null_mut::<Tcl_Obj>(),
                );
                tvfsResultCode(p, &raw mut rc_0);
                if rc_0 != 0 {
                    return if rc_0 < 0 as ::core::ffi::c_int { SQLITE_OK } else { rc_0 };
                }
            }
        }
        return sqlite3OsFileControl((*pFd).pReal, op, pArg);
    }
}
unsafe extern "C" fn tvfsSectorSize(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if (*p).iSectorsize >= 0 as ::core::ffi::c_int {
            return (*p).iSectorsize;
        }
        return sqlite3OsSectorSize((*pFd).pReal);
    }
}
unsafe extern "C" fn tvfsDeviceCharacteristics(
    mut pFile: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if (*p).iDevchar >= 0 as ::core::ffi::c_int {
            return (*p).iDevchar;
        }
        return sqlite3OsDeviceCharacteristics((*pFd).pReal);
    }
}
unsafe extern "C" fn tvfsOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pFile: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pTestfile: *mut TestvfsFile = pFile as *mut TestvfsFile;
        let mut pFd: *mut TestvfsFd = ::core::ptr::null_mut::<TestvfsFd>();
        let mut pId: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut p: *mut Testvfs = (*pVfs).pAppData as *mut Testvfs;
        pFd = Tcl_Alloc(
            (::core::mem::size_of::<TestvfsFd>() as usize)
                .wrapping_add(
                    (*(*((*pVfs).pAppData as *mut Testvfs)).pParent).szOsFile as usize,
                ) as ::core::ffi::c_uint,
        ) as *mut ::core::ffi::c_void as *mut TestvfsFd;
        memset(
            pFd as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<TestvfsFd>() as size_t)
                .wrapping_add(
                    (*(*((*pVfs).pAppData as *mut Testvfs)).pParent).szOsFile as size_t,
                ),
        );
        (*pFd).pShm = ::core::ptr::null_mut::<TestvfsBuffer>();
        (*pFd).pShmId = ::core::ptr::null_mut::<Tcl_Obj>();
        (*pFd).zFilename = zName;
        (*pFd).pVfs = pVfs;
        (*pFd).pReal = pFd.offset(1 as ::core::ffi::c_int as isize) as *mut TestvfsFd
            as *mut sqlite3_file;
        memset(
            pTestfile as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TestvfsFile>() as size_t,
        );
        (*pTestfile).pFd = pFd;
        Tcl_ResetResult((*p).interp);
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_OPEN_MASK != 0 {
            let mut pArg: *mut Tcl_Obj = Tcl_NewObj();
            (*pArg).refCount += 1;
            if flags & SQLITE_OPEN_MAIN_DB != 0 {
                let mut z: *const ::core::ffi::c_char = zName
                    .offset(
                        (strlen
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            ) -> size_t)(zName)
                            .wrapping_add(1 as size_t) as isize,
                    ) as *const ::core::ffi::c_char;
                while *z != 0 {
                    Tcl_ListObjAppendElement(
                        ::core::ptr::null_mut::<Tcl_Interp>(),
                        pArg,
                        Tcl_NewStringObj(z, -1 as ::core::ffi::c_int),
                    );
                    z = z.offset(strlen(z).wrapping_add(1 as size_t) as isize);
                    Tcl_ListObjAppendElement(
                        ::core::ptr::null_mut::<Tcl_Interp>(),
                        pArg,
                        Tcl_NewStringObj(z, -1 as ::core::ffi::c_int),
                    );
                    z = z.offset(strlen(z).wrapping_add(1 as size_t) as isize);
                }
            }
            tvfsExecTcl(
                p,
                b"xOpen\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                pArg,
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            let mut _objPtr: *mut Tcl_Obj = pArg;
            let c2rust_fresh3 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh3 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
            if tvfsResultCode(p, &raw mut rc) != 0 {
                if rc != SQLITE_OK {
                    return rc;
                }
            } else {
                pId = Tcl_GetObjResult((*p).interp);
            }
        }
        if (*p).mask & TESTVFS_OPEN_MASK != 0 && tvfsInjectIoerr(p) != 0 {
            return SQLITE_IOERR;
        }
        if tvfsInjectCantopenerr(p) != 0 {
            return SQLITE_CANTOPEN;
        }
        if tvfsInjectFullerr(p) != 0 {
            return SQLITE_FULL;
        }
        if pId.is_null() {
            pId = Tcl_NewStringObj(
                b"anon\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            );
        }
        (*pId).refCount += 1;
        (*pFd).pShmId = pId;
        Tcl_ResetResult((*p).interp);
        rc = sqlite3OsOpen(
            (*((*pVfs).pAppData as *mut Testvfs)).pParent,
            zName,
            (*pFd).pReal,
            flags,
            pOutFlags,
        );
        if !(*(*pFd).pReal).pMethods.is_null() {
            let mut pMethods: *mut sqlite3_io_methods = ::core::ptr::null_mut::<
                sqlite3_io_methods,
            >();
            let mut nByte: ::core::ffi::c_int = 0;
            if (*pVfs).iVersion > 1 as ::core::ffi::c_int {
                nByte = ::core::mem::size_of::<sqlite3_io_methods>()
                    as ::core::ffi::c_int;
            } else {
                nByte = 104 as ::core::ffi::c_ulong as ::core::ffi::c_int;
            }
            pMethods = Tcl_Alloc(nByte as ::core::ffi::c_uint)
                as *mut ::core::ffi::c_void as *mut sqlite3_io_methods;
            memcpy(
                pMethods as *mut ::core::ffi::c_void,
                &raw mut tvfs_io_methods as *const ::core::ffi::c_void,
                nByte as size_t,
            );
            (*pMethods).iVersion = (*(*(*pFd).pReal).pMethods).iVersion;
            if (*pMethods).iVersion > (*pVfs).iVersion {
                (*pMethods).iVersion = (*pVfs).iVersion;
            }
            if (*pVfs).iVersion > 1 as ::core::ffi::c_int
                && (*((*pVfs).pAppData as *mut Testvfs)).isNoshm != 0
            {
                (*pMethods).xShmUnmap = None;
                (*pMethods).xShmLock = None;
                (*pMethods).xShmBarrier = None;
                (*pMethods).xShmMap = None;
            }
            (*pFile).pMethods = pMethods;
        }
        return rc;
    }
}
unsafe extern "C" fn tvfsDelete(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *mut Testvfs = (*pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_DELETE_MASK != 0 {
            tvfsExecTcl(
                p,
                b"xDelete\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj(zPath, -1 as ::core::ffi::c_int),
                Tcl_NewIntObj(dirSync),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            tvfsResultCode(p, &raw mut rc);
        }
        if rc == SQLITE_OK {
            rc = sqlite3OsDelete(
                (*((*pVfs).pAppData as *mut Testvfs)).pParent,
                zPath,
                dirSync,
            );
        }
        return rc;
    }
}
unsafe extern "C" fn tvfsAccess(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Testvfs = (*pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_ACCESS_MASK != 0 {
            let mut rc: ::core::ffi::c_int = 0;
            let mut zArg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            if flags == SQLITE_ACCESS_EXISTS {
                zArg = b"SQLITE_ACCESS_EXISTS\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            if flags == SQLITE_ACCESS_READWRITE {
                zArg = b"SQLITE_ACCESS_READWRITE\0".as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            if flags == SQLITE_ACCESS_READ {
                zArg = b"SQLITE_ACCESS_READ\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            }
            tvfsExecTcl(
                p,
                b"xAccess\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj(zPath, -1 as ::core::ffi::c_int),
                Tcl_NewStringObj(zArg, -1 as ::core::ffi::c_int),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            if tvfsResultCode(p, &raw mut rc) != 0 {
                if rc != SQLITE_OK {
                    return rc;
                }
            } else {
                let mut interp: *mut Tcl_Interp = (*p).interp;
                if TCL_OK
                    == Tcl_GetBooleanFromObj(
                        ::core::ptr::null_mut::<Tcl_Interp>(),
                        Tcl_GetObjResult(interp),
                        pResOut,
                    )
                {
                    return SQLITE_OK;
                }
            }
        }
        return sqlite3OsAccess(
            (*((*pVfs).pAppData as *mut Testvfs)).pParent,
            zPath,
            flags,
            pResOut,
        );
    }
}
unsafe extern "C" fn tvfsFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Testvfs = (*pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_FULLPATHNAME_MASK != 0 {
            let mut rc: ::core::ffi::c_int = 0;
            tvfsExecTcl(
                p,
                b"xFullPathname\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj(zPath, -1 as ::core::ffi::c_int),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            if tvfsResultCode(p, &raw mut rc) != 0 {
                if rc != SQLITE_OK {
                    return rc;
                }
            }
        }
        return sqlite3OsFullPathname(
            (*((*pVfs).pAppData as *mut Testvfs)).pParent,
            zPath,
            nOut,
            zOut,
        );
    }
}
unsafe extern "C" fn tvfsDlOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return sqlite3OsDlOpen((*((*pVfs).pAppData as *mut Testvfs)).pParent, zPath);
    }
}
unsafe extern "C" fn tvfsDlError(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zErrMsg: *mut ::core::ffi::c_char,
) {
    unsafe {
        sqlite3OsDlError((*((*pVfs).pAppData as *mut Testvfs)).pParent, nByte, zErrMsg);
    }
}
unsafe extern "C" fn tvfsDlSym(
    mut pVfs: *mut sqlite3_vfs,
    mut p: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    unsafe {
        return sqlite3OsDlSym((*((*pVfs).pAppData as *mut Testvfs)).pParent, p, zSym);
    }
}
unsafe extern "C" fn tvfsDlClose(
    mut pVfs: *mut sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    unsafe {
        sqlite3OsDlClose((*((*pVfs).pAppData as *mut Testvfs)).pParent, pHandle);
    }
}
unsafe extern "C" fn tvfsRandomness(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3OsRandomness(
            (*((*pVfs).pAppData as *mut Testvfs)).pParent,
            nByte,
            zBufOut,
        );
    }
}
unsafe extern "C" fn tvfsSleep(
    mut pVfs: *mut sqlite3_vfs,
    mut nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Testvfs = (*pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_SLEEP_MASK != 0 {
            tvfsExecTcl(
                p,
                b"xSleep\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewIntObj(nMicro),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
        }
        return sqlite3OsSleep((*((*pVfs).pAppData as *mut Testvfs)).pParent, nMicro);
    }
}
unsafe extern "C" fn tvfsCurrentTime(
    mut pVfs: *mut sqlite3_vfs,
    mut pTimeOut: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        return (*(*((*pVfs).pAppData as *mut Testvfs)).pParent)
            .xCurrentTime
            .expect(
                "non-null function pointer",
            )((*((*pVfs).pAppData as *mut Testvfs)).pParent, pTimeOut);
    }
}
unsafe extern "C" fn tvfsShmOpen(mut pFile: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Testvfs = ::core::ptr::null_mut::<Testvfs>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pBuffer: *mut TestvfsBuffer = ::core::ptr::null_mut::<TestvfsBuffer>();
        let mut pFd: *mut TestvfsFd = ::core::ptr::null_mut::<TestvfsFd>();
        pFd = (*(pFile as *mut TestvfsFile)).pFd;
        p = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        Tcl_ResetResult((*p).interp);
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_SHMOPEN_MASK != 0 {
            tvfsExecTcl(
                p,
                b"xShmOpen\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*pFd).zFilename, -1 as ::core::ffi::c_int),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            if tvfsResultCode(p, &raw mut rc) != 0 {
                if rc != SQLITE_OK {
                    return rc;
                }
            }
        }
        if (*p).mask & TESTVFS_SHMOPEN_MASK != 0 && tvfsInjectIoerr(p) != 0 {
            return SQLITE_IOERR;
        }
        pBuffer = (*p).pBuffer;
        while !pBuffer.is_null() {
            if 0 as ::core::ffi::c_int == strcmp((*pFd).zFilename, (*pBuffer).zFile) {
                break;
            }
            pBuffer = (*pBuffer).pNext;
        }
        if pBuffer.is_null() {
            let mut szName: ::core::ffi::c_int = strlen((*pFd).zFilename)
                as ::core::ffi::c_int;
            let mut nByte: ::core::ffi::c_int = (::core::mem::size_of::<TestvfsBuffer>()
                as usize)
                .wrapping_add(szName as usize)
                .wrapping_add(1 as usize) as ::core::ffi::c_int;
            pBuffer = Tcl_Alloc(nByte as ::core::ffi::c_uint) as *mut ::core::ffi::c_void
                as *mut TestvfsBuffer;
            memset(
                pBuffer as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nByte as size_t,
            );
            (*pBuffer).zFile = pBuffer.offset(1 as ::core::ffi::c_int as isize)
                as *mut TestvfsBuffer as *mut ::core::ffi::c_char;
            memcpy(
                (*pBuffer).zFile as *mut ::core::ffi::c_void,
                (*pFd).zFilename as *const ::core::ffi::c_void,
                (szName + 1 as ::core::ffi::c_int) as size_t,
            );
            (*pBuffer).pNext = (*p).pBuffer;
            (*p).pBuffer = pBuffer;
        }
        (*pFd).pNext = (*pBuffer).pFile;
        (*pBuffer).pFile = pFd;
        (*pFd).pShm = pBuffer;
        return rc;
    }
}
unsafe extern "C" fn tvfsAllocPage(
    mut p: *mut TestvfsBuffer,
    mut iPage: ::core::ffi::c_int,
    mut pgsz: ::core::ffi::c_int,
) {
    unsafe {
        if (*p).aPage[iPage as usize].is_null() {
            (*p).aPage[iPage as usize] = Tcl_Alloc(pgsz as ::core::ffi::c_uint)
                as *mut ::core::ffi::c_void as *mut u8_0;
            memset(
                (*p).aPage[iPage as usize] as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                pgsz as size_t,
            );
            (*p).pgsz = pgsz;
        }
    }
}
unsafe extern "C" fn tvfsShmMap(
    mut pFile: *mut sqlite3_file,
    mut iPage: ::core::ffi::c_int,
    mut pgsz: ::core::ffi::c_int,
    mut isWrite: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if (*p).isFullshm != 0 {
            let mut pReal: *mut sqlite3_file = (*pFd).pReal;
            return (*(*pReal).pMethods)
                .xShmMap
                .expect("non-null function pointer")(pReal, iPage, pgsz, isWrite, pp);
        }
        if (*pFd).pShm.is_null() {
            rc = tvfsShmOpen(pFile);
            if rc != SQLITE_OK {
                return rc;
            }
        }
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_SHMMAP_MASK != 0 {
            let mut pArg: *mut Tcl_Obj = Tcl_NewObj();
            (*pArg).refCount += 1;
            Tcl_ListObjAppendElement((*p).interp, pArg, Tcl_NewIntObj(iPage));
            Tcl_ListObjAppendElement((*p).interp, pArg, Tcl_NewIntObj(pgsz));
            Tcl_ListObjAppendElement((*p).interp, pArg, Tcl_NewIntObj(isWrite));
            tvfsExecTcl(
                p,
                b"xShmMap\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*(*pFd).pShm).zFile, -1 as ::core::ffi::c_int),
                (*pFd).pShmId,
                pArg,
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            tvfsResultCode(p, &raw mut rc);
            let mut _objPtr: *mut Tcl_Obj = pArg;
            let c2rust_fresh4 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh4 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
        }
        if rc == SQLITE_OK && (*p).mask & TESTVFS_SHMMAP_MASK != 0
            && tvfsInjectIoerr(p) != 0
        {
            rc = SQLITE_IOERR;
        }
        if rc == SQLITE_OK && isWrite != 0
            && (*(*pFd).pShm).aPage[iPage as usize].is_null()
        {
            tvfsAllocPage((*pFd).pShm, iPage, pgsz);
        }
        if rc == SQLITE_OK || rc == SQLITE_READONLY {
            *pp = (*(*pFd).pShm).aPage[iPage as usize] as *mut ::core::ffi::c_void;
        }
        return rc;
    }
}
unsafe extern "C" fn tvfsShmLock(
    mut pFile: *mut sqlite3_file,
    mut ofst: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        let mut nLock: ::core::ffi::c_int = 0;
        let mut zLock: [::core::ffi::c_char; 80] = [0; 80];
        if (*p).isFullshm != 0 {
            let mut pReal: *mut sqlite3_file = (*pFd).pReal;
            return (*(*pReal).pMethods)
                .xShmLock
                .expect("non-null function pointer")(pReal, ofst, n, flags);
        }
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_SHMLOCK_MASK != 0 {
            sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 80]>()
                    as ::core::ffi::c_int,
                &raw mut zLock as *mut ::core::ffi::c_char,
                b"%d %d\0".as_ptr() as *const ::core::ffi::c_char,
                ofst,
                n,
            );
            nLock = strlen(&raw mut zLock as *mut ::core::ffi::c_char)
                as ::core::ffi::c_int;
            if flags & SQLITE_SHM_LOCK != 0 {
                strcpy(
                    (&raw mut zLock as *mut ::core::ffi::c_char).offset(nLock as isize)
                        as *mut ::core::ffi::c_char,
                    b" lock\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                strcpy(
                    (&raw mut zLock as *mut ::core::ffi::c_char).offset(nLock as isize)
                        as *mut ::core::ffi::c_char,
                    b" unlock\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            nLock
                += strlen(
                    (&raw mut zLock as *mut ::core::ffi::c_char).offset(nLock as isize)
                        as *mut ::core::ffi::c_char,
                ) as ::core::ffi::c_int;
            if flags & SQLITE_SHM_SHARED != 0 {
                strcpy(
                    (&raw mut zLock as *mut ::core::ffi::c_char).offset(nLock as isize)
                        as *mut ::core::ffi::c_char,
                    b" shared\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                strcpy(
                    (&raw mut zLock as *mut ::core::ffi::c_char).offset(nLock as isize)
                        as *mut ::core::ffi::c_char,
                    b" exclusive\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            tvfsExecTcl(
                p,
                b"xShmLock\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*(*pFd).pShm).zFile, -1 as ::core::ffi::c_int),
                (*pFd).pShmId,
                Tcl_NewStringObj(
                    &raw mut zLock as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            tvfsResultCode(p, &raw mut rc);
        }
        if rc == SQLITE_OK && (*p).mask & TESTVFS_SHMLOCK_MASK != 0
            && tvfsInjectIoerr(p) != 0
        {
            rc = SQLITE_IOERR;
        }
        if rc == SQLITE_OK {
            let mut isLock: ::core::ffi::c_int = flags & SQLITE_SHM_LOCK;
            let mut isExcl: ::core::ffi::c_int = flags & SQLITE_SHM_EXCLUSIVE;
            let mut mask: u32_0 = ((((1 as ::core::ffi::c_int) << n)
                - 1 as ::core::ffi::c_int) << ofst) as u32_0;
            if isLock != 0 {
                let mut p2: *mut TestvfsFd = ::core::ptr::null_mut::<TestvfsFd>();
                p2 = (*(*pFd).pShm).pFile;
                while !p2.is_null() {
                    if !(p2 == pFd) {
                        if (*p2).excllock & mask != 0
                            || isExcl != 0 && (*p2).sharedlock & mask != 0
                        {
                            rc = SQLITE_BUSY;
                            break;
                        }
                    }
                    p2 = (*p2).pNext;
                }
                if rc == SQLITE_OK {
                    if isExcl != 0 {
                        (*pFd).excllock = ((*pFd).excllock as ::core::ffi::c_uint
                            | mask as ::core::ffi::c_uint) as u32_0;
                    }
                    if isExcl == 0 {
                        (*pFd).sharedlock = ((*pFd).sharedlock as ::core::ffi::c_uint
                            | mask as ::core::ffi::c_uint) as u32_0;
                    }
                }
            } else {
                if isExcl != 0 {
                    (*pFd).excllock = ((*pFd).excllock as ::core::ffi::c_uint
                        & !mask as ::core::ffi::c_uint) as u32_0;
                }
                if isExcl == 0 {
                    (*pFd).sharedlock = ((*pFd).sharedlock as ::core::ffi::c_uint
                        & !mask as ::core::ffi::c_uint) as u32_0;
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn tvfsShmBarrier(mut pFile: *mut sqlite3_file) {
    unsafe {
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_SHMBARRIER_MASK != 0 {
            let mut z: *const ::core::ffi::c_char = if !(*pFd).pShm.is_null() {
                (*(*pFd).pShm).zFile as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            };
            tvfsExecTcl(
                p,
                b"xShmBarrier\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj(z, -1 as ::core::ffi::c_int),
                (*pFd).pShmId,
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
        }
        if (*p).isFullshm != 0 {
            let mut pReal: *mut sqlite3_file = (*pFd).pReal;
            (*(*pReal).pMethods).xShmBarrier.expect("non-null function pointer")(pReal);
            return;
        }
    }
}
unsafe extern "C" fn tvfsShmUnmap(
    mut pFile: *mut sqlite3_file,
    mut deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        let mut p: *mut Testvfs = (*(*pFd).pVfs).pAppData as *mut Testvfs;
        let mut pBuffer: *mut TestvfsBuffer = (*pFd).pShm;
        let mut ppFd: *mut *mut TestvfsFd = ::core::ptr::null_mut::<*mut TestvfsFd>();
        if (*p).isFullshm != 0 {
            let mut pReal: *mut sqlite3_file = (*pFd).pReal;
            return (*(*pReal).pMethods)
                .xShmUnmap
                .expect("non-null function pointer")(pReal, deleteFlag);
        }
        if pBuffer.is_null() {
            return SQLITE_OK;
        }
        if !(*p).pScript.is_null() && (*p).mask & TESTVFS_SHMCLOSE_MASK != 0 {
            tvfsExecTcl(
                p,
                b"xShmUnmap\0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_NewStringObj((*(*pFd).pShm).zFile, -1 as ::core::ffi::c_int),
                (*pFd).pShmId,
                ::core::ptr::null_mut::<Tcl_Obj>(),
                ::core::ptr::null_mut::<Tcl_Obj>(),
            );
            tvfsResultCode(p, &raw mut rc);
        }
        ppFd = &raw mut (*pBuffer).pFile;
        while *ppFd != pFd {
            ppFd = &raw mut (**ppFd).pNext;
        }
        *ppFd = (*pFd).pNext;
        (*pFd).pNext = ::core::ptr::null_mut::<TestvfsFd>();
        if (*pBuffer).pFile.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            let mut pp: *mut *mut TestvfsBuffer = ::core::ptr::null_mut::<
                *mut TestvfsBuffer,
            >();
            pp = &raw mut (*p).pBuffer;
            while *pp != pBuffer {
                pp = &raw mut (**pp).pNext;
            }
            *pp = (**pp).pNext;
            i = 0 as ::core::ffi::c_int;
            while !(*pBuffer).aPage[i as usize].is_null() {
                Tcl_Free((*pBuffer).aPage[i as usize] as *mut ::core::ffi::c_char);
                i += 1;
            }
            Tcl_Free(pBuffer as *mut ::core::ffi::c_char);
        }
        (*pFd).pShm = ::core::ptr::null_mut::<TestvfsBuffer>();
        return rc;
    }
}
unsafe extern "C" fn tvfsFetch(
    mut pFile: *mut sqlite3_file,
    mut iOfst: sqlite3_int64,
    mut iAmt: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        return sqlite3OsFetch((*pFd).pReal, iOfst as i64_0, iAmt, pp);
    }
}
unsafe extern "C" fn tvfsUnfetch(
    mut pFile: *mut sqlite3_file,
    mut iOfst: sqlite3_int64,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFd: *mut TestvfsFd = (*(pFile as *mut TestvfsFile)).pFd;
        return sqlite3OsUnfetch((*pFd).pReal, iOfst as i64_0, p);
    }
}
unsafe extern "C" fn testvfs_obj_cmd(
    mut cd: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Testvfs = cd as *mut Testvfs;
        let mut aSubcmd: [TestvfsSubcmd; 10] = [
            TestvfsSubcmd {
                zName: b"shm\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                eCmd: CMD_SHM,
            },
            TestvfsSubcmd {
                zName: b"delete\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                eCmd: CMD_DELETE,
            },
            TestvfsSubcmd {
                zName: b"filter\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                eCmd: CMD_FILTER,
            },
            TestvfsSubcmd {
                zName: b"ioerr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                eCmd: CMD_IOERR,
            },
            TestvfsSubcmd {
                zName: b"fullerr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                eCmd: CMD_FULLERR,
            },
            TestvfsSubcmd {
                zName: b"cantopenerr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                eCmd: CMD_CANTOPENERR,
            },
            TestvfsSubcmd {
                zName: b"script\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                eCmd: CMD_SCRIPT,
            },
            TestvfsSubcmd {
                zName: b"devchar\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                eCmd: CMD_DEVCHAR,
            },
            TestvfsSubcmd {
                zName: b"sectorsize\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                eCmd: CMD_SECTORSIZE,
            },
            TestvfsSubcmd {
                zName: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                eCmd: CMD_SHM,
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUBCOMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aSubcmd as *mut TestvfsSubcmd as *const ::core::ffi::c_void,
            ::core::mem::size_of::<TestvfsSubcmd>() as ::core::ffi::c_int,
            b"subcommand\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut i,
        ) != 0
        {
            return TCL_ERROR;
        }
        Tcl_ResetResult(interp);
        match aSubcmd[i as usize].eCmd as ::core::ffi::c_uint {
            0 => {
                let mut pObj: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut rc: ::core::ffi::c_int = 0;
                let mut pBuffer: *mut TestvfsBuffer = ::core::ptr::null_mut::<
                    TestvfsBuffer,
                >();
                let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                if objc != 3 as ::core::ffi::c_int && objc != 4 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"FILE ?VALUE?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                zName = Tcl_Alloc((*(*p).pParent).mxPathname as ::core::ffi::c_uint)
                    as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char;
                rc = (*(*p).pParent)
                    .xFullPathname
                    .expect(
                        "non-null function pointer",
                    )(
                    (*p).pParent,
                    Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                    (*(*p).pParent).mxPathname,
                    zName,
                );
                if rc != SQLITE_OK {
                    Tcl_AppendResult(
                        interp,
                        b"failed to get full path: \0".as_ptr()
                            as *const ::core::ffi::c_char,
                        Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                        NULL,
                    );
                    Tcl_Free(zName);
                    return TCL_ERROR;
                }
                pBuffer = (*p).pBuffer;
                while !pBuffer.is_null() {
                    if 0 as ::core::ffi::c_int == strcmp((*pBuffer).zFile, zName) {
                        break;
                    }
                    pBuffer = (*pBuffer).pNext;
                }
                Tcl_Free(zName);
                if pBuffer.is_null() {
                    Tcl_AppendResult(
                        interp,
                        b"no such file: \0".as_ptr() as *const ::core::ffi::c_char,
                        Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                        NULL,
                    );
                    return TCL_ERROR;
                }
                if objc == 4 as ::core::ffi::c_int {
                    let mut n: ::core::ffi::c_int = 0;
                    let mut a: *mut u8_0 = Tcl_GetByteArrayFromObj(
                        *objv.offset(3 as ::core::ffi::c_int as isize),
                        &raw mut n,
                    ) as *mut u8_0;
                    let mut pgsz: ::core::ffi::c_int = (*pBuffer).pgsz;
                    if pgsz == 0 as ::core::ffi::c_int {
                        pgsz = 65536 as ::core::ffi::c_int;
                    }
                    i = 0 as ::core::ffi::c_int;
                    while i * pgsz < n {
                        let mut nByte: ::core::ffi::c_int = pgsz;
                        tvfsAllocPage(pBuffer, i, pgsz);
                        if n - i * pgsz < pgsz {
                            nByte = n;
                        }
                        memcpy(
                            (*pBuffer).aPage[i as usize] as *mut ::core::ffi::c_void,
                            a.offset((i * pgsz) as isize) as *mut u8_0
                                as *const ::core::ffi::c_void,
                            nByte as size_t,
                        );
                        i += 1;
                    }
                }
                pObj = Tcl_NewObj();
                i = 0 as ::core::ffi::c_int;
                while !(*pBuffer).aPage[i as usize].is_null() {
                    let mut pgsz_0: ::core::ffi::c_int = (*pBuffer).pgsz;
                    if pgsz_0 == 0 as ::core::ffi::c_int {
                        pgsz_0 = 65536 as ::core::ffi::c_int;
                    }
                    Tcl_AppendObjToObj(
                        pObj,
                        Tcl_NewByteArrayObj((*pBuffer).aPage[i as usize], pgsz_0),
                    );
                    i += 1;
                }
                Tcl_SetObjResult(interp, pObj);
            }
            2 => {
                static mut vfsmethod: [VfsMethod; 19] = [
                    VfsMethod {
                        zName: b"xShmOpen\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_SHMOPEN_MASK,
                    },
                    VfsMethod {
                        zName: b"xShmLock\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_SHMLOCK_MASK,
                    },
                    VfsMethod {
                        zName: b"xShmBarrier\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_SHMBARRIER_MASK,
                    },
                    VfsMethod {
                        zName: b"xShmUnmap\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_SHMCLOSE_MASK,
                    },
                    VfsMethod {
                        zName: b"xShmMap\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_SHMMAP_MASK,
                    },
                    VfsMethod {
                        zName: b"xSync\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_SYNC_MASK,
                    },
                    VfsMethod {
                        zName: b"xDelete\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_DELETE_MASK,
                    },
                    VfsMethod {
                        zName: b"xWrite\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_WRITE_MASK,
                    },
                    VfsMethod {
                        zName: b"xRead\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_READ_MASK,
                    },
                    VfsMethod {
                        zName: b"xTruncate\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_TRUNCATE_MASK,
                    },
                    VfsMethod {
                        zName: b"xOpen\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_OPEN_MASK,
                    },
                    VfsMethod {
                        zName: b"xClose\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_CLOSE_MASK,
                    },
                    VfsMethod {
                        zName: b"xAccess\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_ACCESS_MASK,
                    },
                    VfsMethod {
                        zName: b"xFullPathname\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_FULLPATHNAME_MASK,
                    },
                    VfsMethod {
                        zName: b"xUnlock\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_UNLOCK_MASK,
                    },
                    VfsMethod {
                        zName: b"xLock\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_LOCK_MASK,
                    },
                    VfsMethod {
                        zName: b"xCheckReservedLock\0".as_ptr()
                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                        mask: TESTVFS_CKLOCK_MASK,
                    },
                    VfsMethod {
                        zName: b"xFileControl\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_FCNTL_MASK,
                    },
                    VfsMethod {
                        zName: b"xSleep\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        mask: TESTVFS_SLEEP_MASK,
                    },
                ];
                let mut apElem: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<
                    *mut Tcl_Obj,
                >();
                let mut nElem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut mask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if objc != 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"LIST\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if Tcl_ListObjGetElements(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut nElem,
                    &raw mut apElem,
                ) != 0
                {
                    return TCL_ERROR;
                }
                Tcl_ResetResult(interp);
                i = 0 as ::core::ffi::c_int;
                while i < nElem {
                    let mut iMethod: ::core::ffi::c_int = 0;
                    let mut zElem: *mut ::core::ffi::c_char = Tcl_GetString(
                        *apElem.offset(i as isize),
                    );
                    iMethod = 0 as ::core::ffi::c_int;
                    while iMethod
                        < (::core::mem::size_of::<[VfsMethod; 19]>() as usize)
                            .wrapping_div(::core::mem::size_of::<VfsMethod>() as usize)
                            as ::core::ffi::c_int
                    {
                        if strcmp(zElem, vfsmethod[iMethod as usize].zName)
                            == 0 as ::core::ffi::c_int
                        {
                            mask |= vfsmethod[iMethod as usize].mask;
                            break;
                        } else {
                            iMethod += 1;
                        }
                    }
                    if iMethod
                        == (::core::mem::size_of::<[VfsMethod; 19]>() as usize)
                            .wrapping_div(::core::mem::size_of::<VfsMethod>() as usize)
                            as ::core::ffi::c_int
                    {
                        Tcl_AppendResult(
                            interp,
                            b"unknown method: \0".as_ptr() as *const ::core::ffi::c_char,
                            zElem,
                            NULL,
                        );
                        return TCL_ERROR;
                    }
                    i += 1;
                }
                (*p).mask = mask;
            }
            4 => {
                if objc == 3 as ::core::ffi::c_int {
                    let mut nByte_0: ::core::ffi::c_int = 0;
                    if !(*p).pScript.is_null() {
                        let mut _objPtr: *mut Tcl_Obj = (*p).pScript;
                        let c2rust_fresh5 = (*_objPtr).refCount;
                        (*_objPtr).refCount = (*_objPtr).refCount - 1;
                        if c2rust_fresh5 <= 1 as ::core::ffi::c_int {
                            TclFreeObj(_objPtr);
                        }
                        (*p).pScript = ::core::ptr::null_mut::<Tcl_Obj>();
                    }
                    Tcl_GetStringFromObj(
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut nByte_0,
                    );
                    if nByte_0 > 0 as ::core::ffi::c_int {
                        (*p).pScript = Tcl_DuplicateObj(
                            *objv.offset(2 as ::core::ffi::c_int as isize),
                        );
                        (*(*p).pScript).refCount += 1;
                    }
                } else if objc != 2 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"?SCRIPT?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                Tcl_ResetResult(interp);
                if !(*p).pScript.is_null() {
                    Tcl_SetObjResult(interp, (*p).pScript);
                }
            }
            8 | 3 | 7 => {
                let mut pTest: *mut TestFaultInject = ::core::ptr::null_mut::<
                    TestFaultInject,
                >();
                let mut iRet: ::core::ffi::c_int = 0;
                match aSubcmd[i as usize].eCmd as ::core::ffi::c_uint {
                    3 => {
                        pTest = &raw mut (*p).ioerr_err;
                    }
                    7 => {
                        pTest = &raw mut (*p).full_err;
                    }
                    8 => {
                        pTest = &raw mut (*p).cantopen_err;
                    }
                    _ => {}
                }
                iRet = (*pTest).nFail;
                (*pTest).nFail = 0 as ::core::ffi::c_int;
                (*pTest).eFault = 0 as ::core::ffi::c_int;
                (*pTest).iCnt = 0 as ::core::ffi::c_int;
                if objc == 4 as ::core::ffi::c_int {
                    let mut iCnt: ::core::ffi::c_int = 0;
                    let mut iPersist: ::core::ffi::c_int = 0;
                    if TCL_OK
                        != Tcl_GetIntFromObj(
                            interp,
                            *objv.offset(2 as ::core::ffi::c_int as isize),
                            &raw mut iCnt,
                        )
                        || TCL_OK
                            != Tcl_GetBooleanFromObj(
                                interp,
                                *objv.offset(3 as ::core::ffi::c_int as isize),
                                &raw mut iPersist,
                            )
                    {
                        return TCL_ERROR;
                    }
                    (*pTest).eFault = if iPersist != 0 {
                        FAULT_INJECT_PERSISTENT
                    } else {
                        FAULT_INJECT_TRANSIENT
                    };
                    (*pTest).iCnt = iCnt;
                } else if objc != 2 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"?CNT PERSIST?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                Tcl_SetObjResult(interp, Tcl_NewIntObj(iRet));
            }
            1 => {
                Tcl_DeleteCommand(
                    interp,
                    Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                );
            }
            5 => {
                let mut aFlag: [DeviceFlag; 16] = [
                    DeviceFlag {
                        zName: b"default\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: -1 as ::core::ffi::c_int,
                    },
                    DeviceFlag {
                        zName: b"atomic\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_ATOMIC,
                    },
                    DeviceFlag {
                        zName: b"atomic512\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_ATOMIC512,
                    },
                    DeviceFlag {
                        zName: b"atomic1k\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_ATOMIC1K,
                    },
                    DeviceFlag {
                        zName: b"atomic2k\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_ATOMIC2K,
                    },
                    DeviceFlag {
                        zName: b"atomic4k\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_ATOMIC4K,
                    },
                    DeviceFlag {
                        zName: b"atomic8k\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_ATOMIC8K,
                    },
                    DeviceFlag {
                        zName: b"atomic16k\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_ATOMIC16K,
                    },
                    DeviceFlag {
                        zName: b"atomic32k\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_ATOMIC32K,
                    },
                    DeviceFlag {
                        zName: b"atomic64k\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_ATOMIC64K,
                    },
                    DeviceFlag {
                        zName: b"sequential\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_SEQUENTIAL,
                    },
                    DeviceFlag {
                        zName: b"safe_append\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_SAFE_APPEND,
                    },
                    DeviceFlag {
                        zName: b"undeletable_when_open\0".as_ptr()
                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN,
                    },
                    DeviceFlag {
                        zName: b"powersafe_overwrite\0".as_ptr()
                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_POWERSAFE_OVERWRITE,
                    },
                    DeviceFlag {
                        zName: b"immutable\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        iValue: SQLITE_IOCAP_IMMUTABLE,
                    },
                    DeviceFlag {
                        zName: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        iValue: 0 as ::core::ffi::c_int,
                    },
                ];
                let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                let mut iFlag: ::core::ffi::c_int = 0;
                if objc > 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"?ATTR-LIST?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if objc == 3 as ::core::ffi::c_int {
                    let mut j: ::core::ffi::c_int = 0;
                    let mut iNew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut flags: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<
                        *mut Tcl_Obj,
                    >();
                    let mut nFlags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if Tcl_ListObjGetElements(
                        interp,
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut nFlags,
                        &raw mut flags,
                    ) != 0
                    {
                        return TCL_ERROR;
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < nFlags {
                        let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        if Tcl_GetIndexFromObjStruct(
                            interp,
                            *flags.offset(j as isize),
                            &raw mut aFlag as *mut DeviceFlag
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<DeviceFlag>() as ::core::ffi::c_int,
                            b"flag\0".as_ptr() as *const ::core::ffi::c_char,
                            0 as ::core::ffi::c_int,
                            &raw mut idx,
                        ) != 0
                        {
                            return TCL_ERROR;
                        }
                        if aFlag[idx as usize].iValue < 0 as ::core::ffi::c_int
                            && nFlags > 1 as ::core::ffi::c_int
                        {
                            Tcl_AppendResult(
                                interp,
                                b"bad flags: \0".as_ptr() as *const ::core::ffi::c_char,
                                Tcl_GetString(
                                    *objv.offset(2 as ::core::ffi::c_int as isize),
                                ),
                                NULL,
                            );
                            return TCL_ERROR;
                        }
                        iNew |= aFlag[idx as usize].iValue;
                        j += 1;
                    }
                    (*p).iDevchar = iNew | 0x10000000 as ::core::ffi::c_int;
                }
                pRet = Tcl_NewObj();
                iFlag = 0 as ::core::ffi::c_int;
                while (iFlag as usize)
                    < (::core::mem::size_of::<[DeviceFlag; 16]>() as usize)
                        .wrapping_div(::core::mem::size_of::<DeviceFlag>() as usize)
                {
                    if (*p).iDevchar & aFlag[iFlag as usize].iValue != 0 {
                        Tcl_ListObjAppendElement(
                            interp,
                            pRet,
                            Tcl_NewStringObj(
                                aFlag[iFlag as usize].zName,
                                -1 as ::core::ffi::c_int,
                            ),
                        );
                    }
                    iFlag += 1;
                }
                Tcl_SetObjResult(interp, pRet);
            }
            6 => {
                if objc > 3 as ::core::ffi::c_int {
                    Tcl_WrongNumArgs(
                        interp,
                        2 as ::core::ffi::c_int,
                        objv,
                        b"?VALUE?\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return TCL_ERROR;
                }
                if objc == 3 as ::core::ffi::c_int {
                    let mut iNew_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if Tcl_GetIntFromObj(
                        interp,
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut iNew_0,
                    ) != 0
                    {
                        return TCL_ERROR;
                    }
                    (*p).iSectorsize = iNew_0;
                }
                Tcl_SetObjResult(interp, Tcl_NewIntObj((*p).iSectorsize));
            }
            _ => {}
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn testvfs_obj_del(mut cd: ClientData) {
    unsafe {
        let mut p: *mut Testvfs = cd as *mut Testvfs;
        if !(*p).pScript.is_null() {
            let mut _objPtr: *mut Tcl_Obj = (*p).pScript;
            let c2rust_fresh6 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh6 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
        }
        sqlite3_vfs_unregister((*p).pVfs);
        memset(
            (*p).pVfs as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sqlite3_vfs>() as size_t,
        );
        Tcl_Free((*p).pVfs as *mut ::core::ffi::c_char);
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Testvfs>() as size_t,
        );
        Tcl_Free(p as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn testvfs_cmd(
    mut cd: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        static mut tvfs_vfs: sqlite3_vfs = unsafe {
            sqlite3_vfs {
                iVersion: 3 as ::core::ffi::c_int,
                szOsFile: 0 as ::core::ffi::c_int,
                mxPathname: 0 as ::core::ffi::c_int,
                pNext: ::core::ptr::null::<sqlite3_vfs>() as *mut sqlite3_vfs,
                zName: ::core::ptr::null::<::core::ffi::c_char>(),
                pAppData: ::core::ptr::null::<::core::ffi::c_void>()
                    as *mut ::core::ffi::c_void,
                xOpen: Some(
                    tvfsOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            *mut sqlite3_file,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xDelete: Some(
                    tvfsDelete
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xAccess: Some(
                    tvfsAccess
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xFullPathname: Some(
                    tvfsFullPathname
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                xDlOpen: Some(
                    tvfsDlOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        ) -> *mut ::core::ffi::c_void,
                ),
                xDlError: Some(
                    tvfsDlError
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> (),
                ),
                xDlSym: Some(
                    tvfsDlSym
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                        ) -> Option<unsafe extern "C" fn() -> ()>,
                ),
                xDlClose: Some(
                    tvfsDlClose
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                xRandomness: Some(
                    tvfsRandomness
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                xSleep: Some(
                    tvfsSleep
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xCurrentTime: Some(
                    tvfsCurrentTime
                        as unsafe extern "C" fn(
                            *mut sqlite3_vfs,
                            *mut ::core::ffi::c_double,
                        ) -> ::core::ffi::c_int,
                ),
                xGetLastError: None,
                xCurrentTimeInt64: None,
                xSetSystemCall: None,
                xGetSystemCall: None,
                xNextSystemCall: None,
            }
        };
        let mut p: *mut Testvfs = ::core::ptr::null_mut::<Testvfs>();
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut zVfs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nByte: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut isNoshm: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut isFullshm: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut isDefault: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut szOsFile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mxPathname: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iVersion: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
        if !(objc < 2 as ::core::ffi::c_int
            || 0 as ::core::ffi::c_int != objc % 2 as ::core::ffi::c_int)
        {
            i = 2 as ::core::ffi::c_int;
            loop {
                if !(i < objc) {
                    c2rust_current_block = 10692455896603418738;
                    break;
                }
                let mut nSwitch: ::core::ffi::c_int = 0;
                let mut zSwitch: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                zSwitch = Tcl_GetStringFromObj(
                    *objv.offset(i as isize),
                    &raw mut nSwitch,
                );
                if nSwitch > 2 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == strncmp(
                            b"-noshm\0".as_ptr() as *const ::core::ffi::c_char,
                            zSwitch,
                            nSwitch as size_t,
                        )
                {
                    if Tcl_GetBooleanFromObj(
                        interp,
                        *objv.offset((i + 1 as ::core::ffi::c_int) as isize),
                        &raw mut isNoshm,
                    ) != 0
                    {
                        return TCL_ERROR;
                    }
                    if isNoshm != 0 {
                        isFullshm = 0 as ::core::ffi::c_int;
                    }
                } else if nSwitch > 2 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == strncmp(
                            b"-default\0".as_ptr() as *const ::core::ffi::c_char,
                            zSwitch,
                            nSwitch as size_t,
                        )
                {
                    if Tcl_GetBooleanFromObj(
                        interp,
                        *objv.offset((i + 1 as ::core::ffi::c_int) as isize),
                        &raw mut isDefault,
                    ) != 0
                    {
                        return TCL_ERROR;
                    }
                } else if nSwitch > 2 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == strncmp(
                            b"-szosfile\0".as_ptr() as *const ::core::ffi::c_char,
                            zSwitch,
                            nSwitch as size_t,
                        )
                {
                    if Tcl_GetIntFromObj(
                        interp,
                        *objv.offset((i + 1 as ::core::ffi::c_int) as isize),
                        &raw mut szOsFile,
                    ) != 0
                    {
                        return TCL_ERROR;
                    }
                } else if nSwitch > 2 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == strncmp(
                            b"-mxpathname\0".as_ptr() as *const ::core::ffi::c_char,
                            zSwitch,
                            nSwitch as size_t,
                        )
                {
                    if Tcl_GetIntFromObj(
                        interp,
                        *objv.offset((i + 1 as ::core::ffi::c_int) as isize),
                        &raw mut mxPathname,
                    ) != 0
                    {
                        return TCL_ERROR;
                    }
                } else if nSwitch > 2 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == strncmp(
                            b"-iversion\0".as_ptr() as *const ::core::ffi::c_char,
                            zSwitch,
                            nSwitch as size_t,
                        )
                {
                    if Tcl_GetIntFromObj(
                        interp,
                        *objv.offset((i + 1 as ::core::ffi::c_int) as isize),
                        &raw mut iVersion,
                    ) != 0
                    {
                        return TCL_ERROR;
                    }
                } else {
                    if !(nSwitch > 2 as ::core::ffi::c_int
                        && 0 as ::core::ffi::c_int
                            == strncmp(
                                b"-fullshm\0".as_ptr() as *const ::core::ffi::c_char,
                                zSwitch,
                                nSwitch as size_t,
                            ))
                    {
                        c2rust_current_block = 16345638819093681963;
                        break;
                    }
                    if Tcl_GetBooleanFromObj(
                        interp,
                        *objv.offset((i + 1 as ::core::ffi::c_int) as isize),
                        &raw mut isFullshm,
                    ) != 0
                    {
                        return TCL_ERROR;
                    }
                    if isFullshm != 0 {
                        isNoshm = 0 as ::core::ffi::c_int;
                    }
                }
                i += 2 as ::core::ffi::c_int;
            }
            match c2rust_current_block {
                16345638819093681963 => {}
                _ => {
                    if (szOsFile as usize)
                        < ::core::mem::size_of::<TestvfsFile>() as usize
                    {
                        szOsFile = ::core::mem::size_of::<TestvfsFile>()
                            as ::core::ffi::c_int;
                    }
                    zVfs = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
                    nByte = (::core::mem::size_of::<Testvfs>() as usize)
                        .wrapping_add(strlen(zVfs) as ::core::ffi::c_int as usize)
                        .wrapping_add(1 as usize) as ::core::ffi::c_int;
                    p = Tcl_Alloc(nByte as ::core::ffi::c_uint)
                        as *mut ::core::ffi::c_void as *mut Testvfs;
                    memset(
                        p as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        nByte as size_t,
                    );
                    (*p).iDevchar = -1 as ::core::ffi::c_int;
                    (*p).iSectorsize = -1 as ::core::ffi::c_int;
                    Tcl_CreateObjCommand(
                        interp,
                        zVfs,
                        Some(
                            testvfs_obj_cmd
                                as unsafe extern "C" fn(
                                    ClientData,
                                    *mut Tcl_Interp,
                                    ::core::ffi::c_int,
                                    *const *mut Tcl_Obj,
                                ) -> ::core::ffi::c_int,
                        ),
                        p as ClientData,
                        Some(testvfs_obj_del as unsafe extern "C" fn(ClientData) -> ()),
                    );
                    (*p).pParent = sqlite3_vfs_find(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                    (*p).interp = interp;
                    (*p).zName = p.offset(1 as ::core::ffi::c_int as isize)
                        as *mut Testvfs as *mut ::core::ffi::c_char;
                    memcpy(
                        (*p).zName as *mut ::core::ffi::c_void,
                        zVfs as *const ::core::ffi::c_void,
                        strlen(zVfs).wrapping_add(1 as size_t),
                    );
                    pVfs = Tcl_Alloc(
                        ::core::mem::size_of::<sqlite3_vfs>() as ::core::ffi::c_uint,
                    ) as *mut ::core::ffi::c_void as *mut sqlite3_vfs;
                    memcpy(
                        pVfs as *mut ::core::ffi::c_void,
                        &raw mut tvfs_vfs as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<sqlite3_vfs>() as size_t,
                    );
                    (*pVfs).pAppData = p as *mut ::core::ffi::c_void;
                    (*pVfs).iVersion = iVersion;
                    (*pVfs).zName = (*p).zName;
                    (*pVfs).mxPathname = (*(*p).pParent).mxPathname;
                    if mxPathname >= 0 as ::core::ffi::c_int
                        && mxPathname < (*pVfs).mxPathname
                    {
                        (*pVfs).mxPathname = mxPathname;
                    }
                    (*pVfs).szOsFile = szOsFile;
                    (*p).pVfs = pVfs;
                    (*p).isNoshm = isNoshm;
                    (*p).isFullshm = isFullshm;
                    (*p).mask = TESTVFS_ALL_MASK;
                    sqlite3_vfs_register(pVfs, isDefault);
                    return TCL_OK;
                }
            }
        }
        Tcl_WrongNumArgs(
            interp,
            1 as ::core::ffi::c_int,
            objv,
            b"VFSNAME ?-noshm BOOL? ?-fullshm BOOL? ?-default BOOL? ?-mxpathname INT? ?-szosfile INT? ?-iversion INT?\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return TCL_ERROR;
    }
}
unsafe extern "C" fn test_vfs_shmlock(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut azArg1: [*const ::core::ffi::c_char; 3] = [
            b"shared\0".as_ptr() as *const ::core::ffi::c_char,
            b"exclusive\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut azArg2: [*const ::core::ffi::c_char; 3] = [
            b"lock\0".as_ptr() as *const ::core::ffi::c_char,
            b"unlock\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zDbname: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut iArg1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iArg2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iOffset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pFd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        if objc != 7 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB DBNAME (shared|exclusive) (lock|unlock) OFFSET N\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zDbname = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
            || Tcl_GetIndexFromObjStruct(
                interp,
                *objv.offset(3 as ::core::ffi::c_int as isize),
                &raw mut azArg1 as *mut *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as ::core::ffi::c_int,
                b"ARG\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                &raw mut iArg1,
            ) != 0
            || Tcl_GetIndexFromObjStruct(
                interp,
                *objv.offset(4 as ::core::ffi::c_int as isize),
                &raw mut azArg2 as *mut *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as ::core::ffi::c_int,
                b"ARG\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                &raw mut iArg2,
            ) != 0
            || Tcl_GetIntFromObj(
                interp,
                *objv.offset(5 as ::core::ffi::c_int as isize),
                &raw mut iOffset,
            ) != 0
            || Tcl_GetIntFromObj(
                interp,
                *objv.offset(6 as ::core::ffi::c_int as isize),
                &raw mut n,
            ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_file_control(
            db,
            zDbname,
            SQLITE_FCNTL_FILE_POINTER,
            &raw mut pFd as *mut ::core::ffi::c_void,
        );
        if pFd.is_null() {
            return TCL_ERROR;
        }
        rc = (*(*pFd).pMethods)
            .xShmLock
            .expect(
                "non-null function pointer",
            )(
            pFd,
            iOffset,
            n,
            (if iArg1 == 0 as ::core::ffi::c_int {
                SQLITE_SHM_SHARED
            } else {
                SQLITE_SHM_EXCLUSIVE
            })
                | (if iArg2 == 0 as ::core::ffi::c_int {
                    SQLITE_SHM_LOCK
                } else {
                    SQLITE_SHM_UNLOCK
                }),
        );
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_vfs_set_readmark(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zDbname: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut iSlot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iVal: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut pFd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut pShm: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut aShm: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
        let mut iOff: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int && objc != 5 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB DBNAME SLOT ?VALUE?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zDbname = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
            || Tcl_GetIntFromObj(
                interp,
                *objv.offset(3 as ::core::ffi::c_int as isize),
                &raw mut iSlot,
            ) != 0
            || objc == 5 as ::core::ffi::c_int
                && Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(4 as ::core::ffi::c_int as isize),
                    &raw mut iVal,
                ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_file_control(
            db,
            zDbname,
            SQLITE_FCNTL_FILE_POINTER,
            &raw mut pFd as *mut ::core::ffi::c_void,
        );
        if pFd.is_null() {
            return TCL_ERROR;
        }
        rc = (*(*pFd).pMethods)
            .xShmMap
            .expect(
                "non-null function pointer",
            )(
            pFd,
            0 as ::core::ffi::c_int,
            32 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut pShm,
        );
        if rc != SQLITE_OK {
            Tcl_SetObjResult(
                interp,
                Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
            );
            return TCL_ERROR;
        }
        if pShm.is_null() {
            Tcl_AppendResult(
                interp,
                b"*-shm is not yet mapped\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        aShm = pShm as *mut u32_0;
        iOff = 12 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int + iSlot;
        if objc == 5 as ::core::ffi::c_int {
            *aShm.offset(iOff as isize) = iVal as u32_0;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj(*aShm.offset(iOff as isize) as ::core::ffi::c_int),
        );
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetestvfs_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_CreateObjCommand(
            interp,
            b"testvfs\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                testvfs_cmd
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
        Tcl_CreateObjCommand(
            interp,
            b"vfs_shmlock\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                test_vfs_shmlock
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"vfs_set_readmark\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                test_vfs_set_readmark
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
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
