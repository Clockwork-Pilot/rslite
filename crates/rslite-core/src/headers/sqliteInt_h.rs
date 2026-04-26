pub use crate::src::src::alter::RenameToken;

pub use crate::src::src::build::TableLock;

pub use crate::src::src::alter::RenameCtx;
pub use crate::src::src::select::CheckOnCtx;
pub use crate::src::src::vtab::VtabCtx;

pub use crate::src::src::select::WhereConst;
pub use crate::src::src::r#where::CoveringIndexCheck;

pub use crate::src::src::expr::IdxCover;
pub use crate::src::src::expr::RefSrcList;
pub use crate::src::src::window::WindowRewrite;

pub const SQLITE_MUTEX_STATIC_TEMPDIR: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_VFS1;

pub const SQLITE_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_DEFAULT_MEMSTATUS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_MALLOC_SOFT_LIMIT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

pub const SQLITE_DIGIT_SEPARATOR: ::core::ffi::c_int = '_' as i32;

pub const SQLITE_BIG_DBL: ::core::ffi::c_double = 1e99f64;

pub const OMIT_TEMPDB: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_MAX_FILE_FORMAT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_TEMP_STORE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_MAX_WORKER_THREADS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const SQLITE_DEFAULT_WORKER_THREADS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_DEFAULT_PCACHE_INITSZ: ::core::ffi::c_int = 20 as ::core::ffi::c_int;

pub const SQLITE_DEFAULT_SORTERREF_SIZE: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;

pub type I8_0 = crate::src::headers::stdlib::Int8T;

pub type Bft = ::core::ffi::c_uint;

pub const SQLITE_MAX_U32: crate::src::ext::rtree::rtree::U64_0 =
    ((1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U64_0) << 32 as ::core::ffi::c_int)
        .wrapping_sub(1 as crate::src::ext::rtree::rtree::U64_0);

pub type TRowcnt = crate::src::ext::rtree::rtree::U64_0;

pub type LogEst = crate::src::headers::stdlib::Int16T;

pub const LOGEST_MIN: ::core::ffi::c_int = -(32768 as ::core::ffi::c_int);

pub const SQLITE_PTRSIZE: ::core::ffi::c_int = crate::internal::__SIZEOF_POINTER__;

pub type Uptr = crate::src::headers::stdlib::UintptrT;

pub const SQLITE_BYTEORDER: ::core::ffi::c_int = 1234 as ::core::ffi::c_int;

pub const SQLITE_BIGENDIAN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_LITTLEENDIAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_UTF16NATIVE: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_UTF16LE;

pub const LARGEST_UINT64: crate::src::ext::rtree::rtree::U64_0 = 0xffffffff
    as crate::src::ext::rtree::rtree::U64_0
    | (0xffffffff as ::core::ffi::c_uint as crate::src::ext::rtree::rtree::U64_0)
        << 32 as ::core::ffi::c_int;

pub const SQLITE_MAX_MMAP_SIZE: ::core::ffi::c_int = 0x7fff0000 as ::core::ffi::c_int;

pub const SQLITE_DEFAULT_MMAP_SIZE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BusyHandler {
    pub xBusyHandler: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub pBusyArg: *mut ::core::ffi::c_void,
    pub nBusy: ::core::ffi::c_int,
}

pub const LEGACY_SCHEMA_TABLE: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_master\0") };

pub const LEGACY_TEMP_SCHEMA_TABLE: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"sqlite_temp_master\0")
};

pub const PREFERRED_SCHEMA_TABLE: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"sqlite_schema\0") };

pub const PREFERRED_TEMP_SCHEMA_TABLE: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"sqlite_temp_schema\0")
};

pub const SCHEMA_ROOT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub type StrAccum = crate::src::headers::sqliteInt_h::sqlite3_str;

pub type Bitmask = crate::src::ext::rtree::rtree::U64_0;

pub const BMS: ::core::ffi::c_int =
    (::core::mem::size_of::<crate::src::headers::sqliteInt_h::Bitmask>() as usize)
        .wrapping_mul(8_usize) as ::core::ffi::c_int;

pub const ALLBITS: crate::src::headers::sqliteInt_h::Bitmask =
    -(1 as ::core::ffi::c_int) as crate::src::headers::sqliteInt_h::Bitmask;

pub const TOPBIT: crate::src::headers::sqliteInt_h::Bitmask = (1 as ::core::ffi::c_int
    as crate::src::headers::sqliteInt_h::Bitmask) << (crate::src::headers::sqliteInt_h::BMS - 1 as ::core::ffi::c_int);

pub type VList = ::core::ffi::c_int;

pub const SQLITE_DEFAULT_SYNCHRONOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Db {
    pub zDbSName: *mut ::core::ffi::c_char,
    pub pBt: *mut crate::src::headers::btreeInt_h::Btree,
    pub safety_level: crate::src::ext::rtree::rtree::U8_0,
    pub bSyncSet: crate::src::ext::rtree::rtree::U8_0,
    pub pSchema: *mut crate::src::headers::sqliteInt_h::Schema,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Schema {
    pub schema_cookie: ::core::ffi::c_int,
    pub iGeneration: ::core::ffi::c_int,
    pub tblHash: crate::src::src::hash::Hash,
    pub idxHash: crate::src::src::hash::Hash,
    pub trigHash: crate::src::src::hash::Hash,
    pub fkeyHash: crate::src::src::hash::Hash,
    pub pSeqTab: *mut crate::src::headers::sqliteInt_h::Table,
    pub file_format: crate::src::ext::rtree::rtree::U8_0,
    pub enc: crate::src::ext::rtree::rtree::U8_0,
    pub schemaFlags: crate::src::fts5::U16_0,
    pub cache_size: ::core::ffi::c_int,
}

pub const DB_SchemaLoaded: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const DB_UnresetViews: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const DB_ResetWanted: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const SQLITE_N_LIMIT: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_LIMIT_WORKER_THREADS + 1 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lookaside {
    pub bDisable: crate::src::ext::rtree::rtree::U32_0,
    pub sz: crate::src::fts5::U16_0,
    pub szTrue: crate::src::fts5::U16_0,
    pub bMalloced: crate::src::ext::rtree::rtree::U8_0,
    pub nSlot: crate::src::ext::rtree::rtree::U32_0,
    pub anStat: [crate::src::ext::rtree::rtree::U32_0; 3],
    pub pInit: *mut crate::src::headers::sqliteInt_h::LookasideSlot,
    pub pFree: *mut crate::src::headers::sqliteInt_h::LookasideSlot,
    pub pSmallInit: *mut crate::src::headers::sqliteInt_h::LookasideSlot,
    pub pSmallFree: *mut crate::src::headers::sqliteInt_h::LookasideSlot,
    pub pMiddle: *mut ::core::ffi::c_void,
    pub pStart: *mut ::core::ffi::c_void,
    pub pEnd: *mut ::core::ffi::c_void,
    pub pTrueEnd: *mut ::core::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookasideSlot {
    pub pNext: *mut crate::src::headers::sqliteInt_h::LookasideSlot,
}

pub const LOOKASIDE_SMALL: ::core::ffi::c_int = 128 as ::core::ffi::c_int;

pub const SQLITE_FUNC_HASH_SZ: ::core::ffi::c_int = 23 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncDefHash {
    pub a: [*mut crate::src::headers::sqliteInt_h::FuncDef; 23],
}

pub type Sqlite3Xauth = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;

pub const SQLITE_TRACE_LEGACY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const SQLITE_TRACE_XPROFILE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const SQLITE_TRACE_NONLEGACY_MASK: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;

pub const SQLITE_MAX_DB: ::core::ffi::c_int =
    crate::sqliteLimit_h::SQLITE_MAX_ATTACHED + 2 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3 {
    pub pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    pub pVdbe: *mut crate::src::headers::vdbeInt_h::Vdbe,
    pub pDfltColl: *mut crate::src::headers::sqliteInt_h::CollSeq,
    pub mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex,
    pub aDb: *mut crate::src::headers::sqliteInt_h::Db,
    pub nDb: ::core::ffi::c_int,
    pub mDbFlags: crate::src::ext::rtree::rtree::U32_0,
    pub flags: crate::src::ext::rtree::rtree::U64_0,
    pub lastRowid: crate::src::ext::rtree::rtree::I64_0,
    pub szMmap: crate::src::ext::rtree::rtree::I64_0,
    pub nSchemaLock: crate::src::ext::rtree::rtree::U32_0,
    pub openFlags: ::core::ffi::c_uint,
    pub errCode: ::core::ffi::c_int,
    pub errByteOffset: ::core::ffi::c_int,
    pub errMask: ::core::ffi::c_int,
    pub iSysErrno: ::core::ffi::c_int,
    pub dbOptFlags: crate::src::ext::rtree::rtree::U32_0,
    pub enc: crate::src::ext::rtree::rtree::U8_0,
    pub autoCommit: crate::src::ext::rtree::rtree::U8_0,
    pub temp_store: crate::src::ext::rtree::rtree::U8_0,
    pub mallocFailed: crate::src::ext::rtree::rtree::U8_0,
    pub bBenignMalloc: crate::src::ext::rtree::rtree::U8_0,
    pub dfltLockMode: crate::src::ext::rtree::rtree::U8_0,
    pub nextAutovac: ::core::ffi::c_schar,
    pub suppressErr: crate::src::ext::rtree::rtree::U8_0,
    pub vtabOnConflict: crate::src::ext::rtree::rtree::U8_0,
    pub isTransactionSavepoint: crate::src::ext::rtree::rtree::U8_0,
    pub mTrace: crate::src::ext::rtree::rtree::U8_0,
    pub noSharedCache: crate::src::ext::rtree::rtree::U8_0,
    pub nSqlExec: crate::src::ext::rtree::rtree::U8_0,
    pub eOpenState: crate::src::ext::rtree::rtree::U8_0,
    pub nextPagesize: ::core::ffi::c_int,
    pub nChange: crate::src::ext::rtree::rtree::I64_0,
    pub nTotalChange: crate::src::ext::rtree::rtree::I64_0,
    pub aLimit: [::core::ffi::c_int; 12],
    pub nMaxSorterMmap: ::core::ffi::c_int,
    pub init: crate::src::headers::sqliteInt_h::sqlite3InitInfo,
    pub nVdbeActive: ::core::ffi::c_int,
    pub nVdbeRead: ::core::ffi::c_int,
    pub nVdbeWrite: ::core::ffi::c_int,
    pub nVdbeExec: ::core::ffi::c_int,
    pub nVDestroy: ::core::ffi::c_int,
    pub nExtension: ::core::ffi::c_int,
    pub aExtension: *mut *mut ::core::ffi::c_void,
    pub trace: crate::src::headers::sqliteInt_h::__anon_union_0,
    pub pTraceArg: *mut ::core::ffi::c_void,
    pub xProfile: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            crate::src::ext::rtree::rtree::U64_0,
        ) -> (),
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
            crate::src::headers::sqlite3_h::SqliteInt64,
        ) -> (),
    >,
    pub pAutovacPagesArg: *mut ::core::ffi::c_void,
    pub xAutovacDestr: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xAutovacPages: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            crate::src::ext::rtree::rtree::U32_0,
            crate::src::ext::rtree::rtree::U32_0,
            crate::src::ext::rtree::rtree::U32_0,
        ) -> ::core::ffi::c_uint,
    >,
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub pPreUpdateArg: *mut ::core::ffi::c_void,
    pub xPreUpdateCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            crate::src::headers::sqlite3_h::Sqlite3Int64,
            crate::src::headers::sqlite3_h::Sqlite3Int64,
        ) -> (),
    >,
    pub pPreUpdate: *mut crate::src::headers::vdbeInt_h::PreUpdate,
    pub xWalCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pWalArg: *mut ::core::ffi::c_void,
    pub xCollNeeded: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub xCollNeeded16: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> (),
    >,
    pub pCollNeededArg: *mut ::core::ffi::c_void,
    pub pErr: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pub u1: crate::src::headers::sqliteInt_h::__anon_union_1,
    pub lookaside: crate::src::headers::sqliteInt_h::Lookaside,
    pub xAuth: crate::src::headers::sqliteInt_h::Sqlite3Xauth,
    pub pAuthArg: *mut ::core::ffi::c_void,
    pub xProgress: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub pProgressArg: *mut ::core::ffi::c_void,
    pub nProgressOps: ::core::ffi::c_uint,
    pub nVTrans: ::core::ffi::c_int,
    pub aModule: crate::src::src::hash::Hash,
    pub pVtabCtx: *mut crate::src::headers::sqliteInt_h::VtabCtx,
    pub aVTrans: *mut *mut crate::src::headers::sqliteInt_h::VTable,
    pub pDisconnect: *mut crate::src::headers::sqliteInt_h::VTable,
    pub aFunc: crate::src::src::hash::Hash,
    pub aCollSeq: crate::src::src::hash::Hash,
    pub busyHandler: crate::src::headers::sqliteInt_h::BusyHandler,
    pub aDbStatic: [crate::src::headers::sqliteInt_h::Db; 2],
    pub pSavepoint: *mut crate::src::headers::sqliteInt_h::Savepoint,
    pub nAnalysisLimit: ::core::ffi::c_int,
    pub busyTimeout: ::core::ffi::c_int,
    pub nSavepoint: ::core::ffi::c_int,
    pub nStatement: ::core::ffi::c_int,
    pub nDeferredCons: crate::src::ext::rtree::rtree::I64_0,
    pub nDeferredImmCons: crate::src::ext::rtree::rtree::I64_0,
    pub pnBytesFreed: *mut ::core::ffi::c_int,
    pub pDbData: *mut crate::src::headers::sqliteInt_h::DbClientData,
    pub nSpill: crate::src::ext::rtree::rtree::U64_0,
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct sqlite3InitInfo {
    pub newTnum: crate::src::src::pager::Pgno,
    pub iDb: crate::src::ext::rtree::rtree::U8_0,
    pub busy: crate::src::ext::rtree::rtree::U8_0,
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
pub union __anon_union_0 {
    pub xLegacy:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> ()>,
    pub xV2: Option<
        unsafe extern "C" fn(
            crate::src::ext::rtree::rtree::U32_0,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_1 {
    pub isInterrupted: ::core::ffi::c_int,
    pub notUsed1: ::core::ffi::c_double,
}

pub const SQLITE_WriteSchema: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_LegacyFileFmt: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SQLITE_FullColNames: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const SQLITE_FullFSync: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const SQLITE_CkptFullFSync: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const SQLITE_CacheSpill: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const SQLITE_ShortColNames: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const SQLITE_TrustedSchema: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const SQLITE_NullCallback: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const SQLITE_IgnoreChecks: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const SQLITE_StmtScanStatus: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const SQLITE_NoCkptOnClose: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const SQLITE_ReverseOrder: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const SQLITE_RecTriggers: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const SQLITE_ForeignKeys: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const SQLITE_AutoIndex: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;

pub const SQLITE_LoadExtension: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

pub const SQLITE_LoadExtFunc: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;

pub const SQLITE_EnableTrigger: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;

pub const SQLITE_DeferFKs: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;

pub const SQLITE_QueryOnly: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;

pub const SQLITE_CellSizeCk: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;

pub const SQLITE_Fts3Tokenizer: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;

pub const SQLITE_EnableQPSG: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;

pub const SQLITE_TriggerEQP: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;

pub const SQLITE_ResetDatabase: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;

pub const SQLITE_LegacyAlter: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;

pub const SQLITE_NoSchemaError: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;

pub const SQLITE_Defensive: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;

pub const SQLITE_DqsDDL: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;

pub const SQLITE_DqsDML: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;

pub const SQLITE_EnableView: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;

pub const SQLITE_CountRows: crate::src::ext::rtree::rtree::U64_0 =
    (0x1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U64_0) << 32 as ::core::ffi::c_int;

pub const SQLITE_CorruptRdOnly: crate::src::ext::rtree::rtree::U64_0 =
    (0x2 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U64_0) << 32 as ::core::ffi::c_int;

pub const SQLITE_ReadUncommit: crate::src::ext::rtree::rtree::U64_0 =
    (0x4 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U64_0) << 32 as ::core::ffi::c_int;

pub const SQLITE_FkNoAction: crate::src::ext::rtree::rtree::U64_0 =
    (0x8 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U64_0) << 32 as ::core::ffi::c_int;

pub const SQLITE_AttachCreate: crate::src::ext::rtree::rtree::U64_0 = (0x10 as ::core::ffi::c_int
    as crate::src::ext::rtree::rtree::U64_0)
    << 32 as ::core::ffi::c_int;

pub const SQLITE_AttachWrite: crate::src::ext::rtree::rtree::U64_0 = (0x20 as ::core::ffi::c_int
    as crate::src::ext::rtree::rtree::U64_0)
    << 32 as ::core::ffi::c_int;

pub const SQLITE_Comments: crate::src::ext::rtree::rtree::U64_0 = (0x40 as ::core::ffi::c_int
    as crate::src::ext::rtree::rtree::U64_0)
    << 32 as ::core::ffi::c_int;

pub const DBFLAG_SchemaChange: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const DBFLAG_PreferBuiltin: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const DBFLAG_Vacuum: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const DBFLAG_VacuumInto: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const DBFLAG_SchemaKnownOk: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const DBFLAG_InternalFunc: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const DBFLAG_EncodingFixed: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const SQLITE_STATE_OPEN: ::core::ffi::c_int = 0x76 as ::core::ffi::c_int;

pub const SQLITE_STATE_CLOSED: ::core::ffi::c_int = 0xce as ::core::ffi::c_int;

pub const SQLITE_STATE_SICK: ::core::ffi::c_int = 0xba as ::core::ffi::c_int;

pub const SQLITE_STATE_BUSY: ::core::ffi::c_int = 0x6d as ::core::ffi::c_int;

pub const SQLITE_STATE_ERROR: ::core::ffi::c_int = 0xd5 as ::core::ffi::c_int;

pub const SQLITE_STATE_ZOMBIE: ::core::ffi::c_int = 0xa7 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncDef {
    pub nArg: crate::src::fts5::I16_0,
    pub funcFlags: crate::src::ext::rtree::rtree::U32_0,
    pub pUserData: *mut ::core::ffi::c_void,
    pub pNext: *mut crate::src::headers::sqliteInt_h::FuncDef,
    pub xSFunc: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> (),
    >,
    pub xFinalize:
        Option<unsafe extern "C" fn(*mut crate::src::headers::vdbeInt_h::sqlite3_context) -> ()>,
    pub xValue:
        Option<unsafe extern "C" fn(*mut crate::src::headers::vdbeInt_h::sqlite3_context) -> ()>,
    pub xInverse: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> (),
    >,
    pub zName: *const ::core::ffi::c_char,
    pub u: crate::src::headers::sqliteInt_h::__anon_union_2,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_2 {
    pub pHash: *mut crate::src::headers::sqliteInt_h::FuncDef,
    pub pDestructor: *mut crate::src::headers::sqliteInt_h::FuncDestructor,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncDestructor {
    pub nRef: ::core::ffi::c_int,
    pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pUserData: *mut ::core::ffi::c_void,
}

pub const SQLITE_FUNC_ENCMASK: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;

pub const SQLITE_FUNC_LIKE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const SQLITE_FUNC_CASE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const SQLITE_FUNC_EPHEM: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const SQLITE_FUNC_NEEDCOLL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const SQLITE_FUNC_LENGTH: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const SQLITE_FUNC_TYPEOF: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const SQLITE_FUNC_COUNT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const SQLITE_FUNC_UNLIKELY: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const SQLITE_FUNC_CONSTANT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const SQLITE_FUNC_MINMAX: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const SQLITE_FUNC_SLOCHNG: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const SQLITE_FUNC_TEST: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const SQLITE_FUNC_RUNONLY: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;

pub const SQLITE_FUNC_WINDOW: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

pub const SQLITE_FUNC_INTERNAL: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;

pub const SQLITE_FUNC_DIRECT: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;

pub const SQLITE_FUNC_UNSAFE: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;

pub const SQLITE_FUNC_INLINE: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;

pub const SQLITE_FUNC_BUILTIN: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;

pub const SQLITE_FUNC_ANYORDER: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;

pub const INLINEFUNC_coalesce: ::core::ffi::c_int = 0;

pub const INLINEFUNC_implies_nonnull_row: ::core::ffi::c_int = 1;

pub const INLINEFUNC_expr_implies_expr: ::core::ffi::c_int = 2;

pub const INLINEFUNC_expr_compare: ::core::ffi::c_int = 3;

pub const INLINEFUNC_affinity: ::core::ffi::c_int = 4;

pub const INLINEFUNC_iif: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Savepoint {
    pub zName: *mut ::core::ffi::c_char,
    pub nDeferredCons: crate::src::ext::rtree::rtree::I64_0,
    pub nDeferredImmCons: crate::src::ext::rtree::rtree::I64_0,
    pub pNext: *mut crate::src::headers::sqliteInt_h::Savepoint,
}

pub const SAVEPOINT_BEGIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SAVEPOINT_BEGIN_1: ::core::ffi::c_int = 0;

pub const SAVEPOINT_RELEASE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SAVEPOINT_ROLLBACK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SAVEPOINT_ROLLBACK_1: ::core::ffi::c_int = 2;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Module {
    pub pModule: *const crate::src::headers::sqlite3_h::sqlite3_module,
    pub zName: *const ::core::ffi::c_char,
    pub nRefModule: ::core::ffi::c_int,
    pub pAux: *mut ::core::ffi::c_void,
    pub xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pEpoTab: *mut crate::src::headers::sqliteInt_h::Table,
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Column {
    pub zCnName: *mut ::core::ffi::c_char,
    #[bitfield(name = "notNull", ty = "::core::ffi::c_uint", bits = "0..=3")]
    #[bitfield(name = "eCType", ty = "::core::ffi::c_uint", bits = "4..=7")]
    pub notNull_eCType: [u8; 1],
    pub affinity: ::core::ffi::c_char,
    pub szEst: crate::src::ext::rtree::rtree::U8_0,
    pub hName: crate::src::ext::rtree::rtree::U8_0,
    pub iDflt: crate::src::fts5::U16_0,
    pub colFlags: crate::src::fts5::U16_0,
}

pub const COLTYPE_CUSTOM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const COLTYPE_ANY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const COLTYPE_BLOB: ::core::ffi::c_int = 2;

pub const COLTYPE_INT: ::core::ffi::c_int = 3;

pub const COLTYPE_INTEGER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const COLTYPE_INTEGER_1: ::core::ffi::c_int = 4;

pub const COLTYPE_REAL: ::core::ffi::c_int = 5;

pub const COLTYPE_TEXT: ::core::ffi::c_int = 6;

pub const SQLITE_N_STDTYPE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const COLFLAG_PRIMKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const COLFLAG_HIDDEN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const COLFLAG_HASTYPE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const COLFLAG_UNIQUE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const COLFLAG_VIRTUAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const COLFLAG_STORED: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const COLFLAG_NOTAVAIL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const COLFLAG_BUSY: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const COLFLAG_HASCOLL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const COLFLAG_NOEXPAND: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const COLFLAG_GENERATED: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;

pub const COLFLAG_NOINSERT: ::core::ffi::c_int = 0x62 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollSeq {
    pub zName: *mut ::core::ffi::c_char,
    pub enc: crate::src::ext::rtree::rtree::U8_0,
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

pub const SQLITE_SO_ASC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_SO_DESC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_SO_UNDEFINED: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);

pub const SQLITE_AFF_NONE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const SQLITE_AFF_BLOB: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;

pub const SQLITE_AFF_TEXT: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;

pub const SQLITE_AFF_NUMERIC: ::core::ffi::c_int = 0x43 as ::core::ffi::c_int;

pub const SQLITE_AFF_INTEGER: ::core::ffi::c_int = 0x44 as ::core::ffi::c_int;

pub const SQLITE_AFF_INTEGER_1: ::core::ffi::c_int = 68;

pub const SQLITE_AFF_REAL: ::core::ffi::c_int = 0x45 as ::core::ffi::c_int;

pub const SQLITE_AFF_REAL_1: ::core::ffi::c_int = 69;

pub const SQLITE_AFF_FLEXNUM: ::core::ffi::c_int = 0x46 as ::core::ffi::c_int;

pub const SQLITE_AFF_DEFER: ::core::ffi::c_int = 0x58 as ::core::ffi::c_int;

pub const SQLITE_AFF_MASK: ::core::ffi::c_int = 0x47 as ::core::ffi::c_int;

pub const SQLITE_JUMPIFNULL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const SQLITE_NULLEQ: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const SQLITE_NOTNULL: ::core::ffi::c_int = 0x90 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VTable {
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub pMod: *mut crate::src::headers::sqliteInt_h::Module,
    pub pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    pub nRef: ::core::ffi::c_int,
    pub bConstraint: crate::src::ext::rtree::rtree::U8_0,
    pub bAllSchemas: crate::src::ext::rtree::rtree::U8_0,
    pub eVtabRisk: crate::src::ext::rtree::rtree::U8_0,
    pub iSavepoint: ::core::ffi::c_int,
    pub pNext: *mut crate::src::headers::sqliteInt_h::VTable,
}

pub const SQLITE_VTABRISK_Low: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_VTABRISK_Normal: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_VTABRISK_High: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub zName: *mut ::core::ffi::c_char,
    pub aCol: *mut crate::src::headers::sqliteInt_h::Column,
    pub pIndex: *mut crate::src::headers::sqliteInt_h::Index,
    pub zColAff: *mut ::core::ffi::c_char,
    pub pCheck: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub tnum: crate::src::src::pager::Pgno,
    pub nTabRef: crate::src::ext::rtree::rtree::U32_0,
    pub tabFlags: crate::src::ext::rtree::rtree::U32_0,
    pub iPKey: crate::src::fts5::I16_0,
    pub nCol: crate::src::fts5::I16_0,
    pub nNVCol: crate::src::fts5::I16_0,
    pub nRowLogEst: crate::src::headers::sqliteInt_h::LogEst,
    pub szTabRow: crate::src::headers::sqliteInt_h::LogEst,
    pub keyConf: crate::src::ext::rtree::rtree::U8_0,
    pub eTabType: crate::src::ext::rtree::rtree::U8_0,
    pub u: crate::src::headers::sqliteInt_h::__anon_union_3,
    pub pTrigger: *mut crate::src::headers::sqliteInt_h::Trigger,
    pub pSchema: *mut crate::src::headers::sqliteInt_h::Schema,
    pub aHx: [crate::src::ext::rtree::rtree::U8_0; 16],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_3 {
    pub tab: crate::src::headers::sqliteInt_h::__anon_struct_0,
    pub view: crate::src::headers::sqliteInt_h::__anon_struct_1,
    pub vtab: crate::src::headers::sqliteInt_h::__anon_struct_2,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_0 {
    pub addColOffset: ::core::ffi::c_int,
    pub pFKey: *mut crate::src::headers::sqliteInt_h::FKey,
    pub pDfltList: *mut crate::src::headers::sqliteInt_h::ExprList,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_1 {
    pub pSelect: *mut crate::src::headers::sqliteInt_h::Select,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_2 {
    pub nArg: ::core::ffi::c_int,
    pub azArg: *mut *mut ::core::ffi::c_char,
    pub p: *mut crate::src::headers::sqliteInt_h::VTable,
}

pub const TF_Readonly: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const TF_HasHidden: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const TF_HasPrimaryKey: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const TF_Autoincrement: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const TF_HasStat1: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const TF_HasVirtual: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const TF_HasStored: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const TF_HasGenerated: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;

pub const TF_WithoutRowid: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const TF_MaybeReanalyze: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const TF_NoVisibleRowid: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const TF_OOOHidden: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const TF_HasNotNull: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const TF_Shadow: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const TF_Ephemeral: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const TF_Eponymous: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;

pub const TF_Strict: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

pub const TF_Imposter: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;

pub const TABTYP_NORM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TABTYP_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const TABTYP_VIEW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const ViewCanHaveRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FKey {
    pub pFrom: *mut crate::src::headers::sqliteInt_h::Table,
    pub pNextFrom: *mut crate::src::headers::sqliteInt_h::FKey,
    pub zTo: *mut ::core::ffi::c_char,
    pub pNextTo: *mut crate::src::headers::sqliteInt_h::FKey,
    pub pPrevTo: *mut crate::src::headers::sqliteInt_h::FKey,
    pub nCol: ::core::ffi::c_int,
    pub isDeferred: crate::src::ext::rtree::rtree::U8_0,
    pub aAction: [crate::src::ext::rtree::rtree::U8_0; 2],
    pub apTrigger: [*mut crate::src::headers::sqliteInt_h::Trigger; 2],
    pub aCol: [crate::src::headers::sqliteInt_h::sColMap; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sColMap {
    pub iFrom: ::core::ffi::c_int,
    pub zCol: *mut ::core::ffi::c_char,
}

pub const OE_None: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const OE_Rollback: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const OE_Rollback_1: ::core::ffi::c_int = 1;

pub const OE_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const OE_Fail: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const OE_Fail_1: ::core::ffi::c_int = 3;

pub const OE_Ignore: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const OE_Ignore_1: ::core::ffi::c_int = 4;

pub const OE_Replace: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const OE_Update: ::core::ffi::c_int = 6;

pub const OE_Restrict: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const OE_Restrict_1: ::core::ffi::c_int = 7;

pub const OE_SetNull: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const OE_SetNull_1: ::core::ffi::c_int = 8;

pub const OE_SetDflt: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const OE_SetDflt_1: ::core::ffi::c_int = 9;

pub const OE_Cascade: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const OE_Cascade_1: ::core::ffi::c_int = 10;

pub const OE_Default: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyInfo {
    pub nRef: crate::src::ext::rtree::rtree::U32_0,
    pub enc: crate::src::ext::rtree::rtree::U8_0,
    pub nKeyField: crate::src::fts5::U16_0,
    pub nAllField: crate::src::fts5::U16_0,
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub aSortFlags: *mut crate::src::ext::rtree::rtree::U8_0,
    pub aColl: [*mut crate::src::headers::sqliteInt_h::CollSeq; 0],
}

pub const KEYINFO_ORDER_DESC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const KEYINFO_ORDER_BIGNULL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnpackedRecord {
    pub pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo,
    pub aMem: *mut crate::src::src::vdbe::Mem,
    pub u: crate::src::headers::sqliteInt_h::__anon_union_4,
    pub n: ::core::ffi::c_int,
    pub nField: crate::src::fts5::U16_0,
    pub default_rc: crate::src::headers::sqliteInt_h::I8_0,
    pub errCode: crate::src::ext::rtree::rtree::U8_0,
    pub r1: crate::src::headers::sqliteInt_h::I8_0,
    pub r2: crate::src::headers::sqliteInt_h::I8_0,
    pub eqSeen: crate::src::ext::rtree::rtree::U8_0,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_4 {
    pub z: *mut ::core::ffi::c_char,
    pub i: crate::src::ext::rtree::rtree::I64_0,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CCurHint {
    pub iTabCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int,
    pub pIdx: *mut Index,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Index {
    pub zName: *mut ::core::ffi::c_char,
    pub aiColumn: *mut crate::src::fts5::I16_0,
    pub aiRowLogEst: *mut crate::src::headers::sqliteInt_h::LogEst,
    pub pTable: *mut crate::src::headers::sqliteInt_h::Table,
    pub zColAff: *mut ::core::ffi::c_char,
    pub pNext: *mut crate::src::headers::sqliteInt_h::Index,
    pub pSchema: *mut crate::src::headers::sqliteInt_h::Schema,
    pub aSortOrder: *mut crate::src::ext::rtree::rtree::U8_0,
    pub azColl: *mut *const ::core::ffi::c_char,
    pub pPartIdxWhere: *mut crate::src::headers::sqliteInt_h::Expr,
    pub aColExpr: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub tnum: crate::src::src::pager::Pgno,
    pub szIdxRow: crate::src::headers::sqliteInt_h::LogEst,
    pub nKeyCol: crate::src::fts5::U16_0,
    pub nColumn: crate::src::fts5::U16_0,
    pub onError: crate::src::ext::rtree::rtree::U8_0,
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
    pub colNotIdxed: crate::src::headers::sqliteInt_h::Bitmask,
}

pub const SQLITE_IDXTYPE_APPDEF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_IDXTYPE_UNIQUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_IDXTYPE_PRIMARYKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_IDXTYPE_IPK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const XN_ROWID: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);

pub const XN_EXPR: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub z: *const ::core::ffi::c_char,
    pub n: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo {
    pub directMode: crate::src::ext::rtree::rtree::U8_0,
    pub useSortingIdx: crate::src::ext::rtree::rtree::U8_0,
    pub nSortingColumn: crate::src::ext::rtree::rtree::U32_0,
    pub sortingIdx: ::core::ffi::c_int,
    pub sortingIdxPTab: ::core::ffi::c_int,
    pub iFirstReg: ::core::ffi::c_int,
    pub pGroupBy: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub aCol: *mut crate::src::headers::sqliteInt_h::AggInfo_col,
    pub nColumn: ::core::ffi::c_int,
    pub nAccumulator: ::core::ffi::c_int,
    pub aFunc: *mut crate::src::headers::sqliteInt_h::AggInfo_func,
    pub nFunc: ::core::ffi::c_int,
    pub selId: crate::src::ext::rtree::rtree::U32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo_col {
    pub pTab: *mut crate::src::headers::sqliteInt_h::Table,
    pub pCExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: ::core::ffi::c_int,
    pub iSorterColumn: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AggInfo_func {
    pub pFExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pFunc: *mut crate::src::headers::sqliteInt_h::FuncDef,
    pub iDistinct: ::core::ffi::c_int,
    pub iDistAddr: ::core::ffi::c_int,
    pub iOBTab: ::core::ffi::c_int,
    pub bOBPayload: crate::src::ext::rtree::rtree::U8_0,
    pub bOBUnique: crate::src::ext::rtree::rtree::U8_0,
    pub bUseSubtype: crate::src::ext::rtree::rtree::U8_0,
}

pub type YnVar = crate::src::fts5::I16_0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr {
    pub op: crate::src::ext::rtree::rtree::U8_0,
    pub affExpr: ::core::ffi::c_char,
    pub op2: crate::src::ext::rtree::rtree::U8_0,
    pub flags: crate::src::ext::rtree::rtree::U32_0,
    pub u: crate::src::headers::sqliteInt_h::__anon_union_5,
    pub pLeft: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pRight: *mut crate::src::headers::sqliteInt_h::Expr,
    pub x: crate::src::headers::sqliteInt_h::__anon_union_6,
    pub nHeight: ::core::ffi::c_int,
    pub iTable: ::core::ffi::c_int,
    pub iColumn: crate::src::headers::sqliteInt_h::YnVar,
    pub iAgg: crate::src::fts5::I16_0,
    pub w: crate::src::headers::sqliteInt_h::__anon_union_7,
    pub pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
    pub y: crate::src::headers::sqliteInt_h::__anon_union_8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_5 {
    pub zToken: *mut ::core::ffi::c_char,
    pub iValue: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_6 {
    pub pList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pSelect: *mut crate::src::headers::sqliteInt_h::Select,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_7 {
    pub iJoin: ::core::ffi::c_int,
    pub iOfst: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_8 {
    pub pTab: *mut crate::src::headers::sqliteInt_h::Table,
    pub pWin: *mut crate::src::headers::sqliteInt_h::Window,
    pub nReg: ::core::ffi::c_int,
    pub sub: crate::src::headers::sqliteInt_h::__anon_struct_3,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_3 {
    pub iAddr: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
}

pub const EP_OuterON: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const EP_InnerON: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const EP_Distinct: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const EP_HasFunc: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const EP_FixedCol: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const EP_DblQuoted: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const EP_InfixFunc: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const EP_Collate: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const EP_Commuted: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const EP_IntValue: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const EP_xIsSelect: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const EP_Skip: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const EP_Reduced: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const EP_TokenOnly: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

pub const EP_IfNullRow: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;

pub const EP_Subquery: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;

pub const EP_Leaf: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;

pub const EP_WinFunc: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;

pub const EP_Subrtn: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;

pub const EP_Quoted: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;

pub const EP_Static: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;

pub const EP_IsTrue: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;

pub const EP_IsFalse: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;

pub const EP_Propagate: ::core::ffi::c_int = crate::src::headers::sqliteInt_h::EP_Collate
    | crate::src::headers::sqliteInt_h::EP_Subquery
    | crate::src::headers::sqliteInt_h::EP_HasFunc;

pub const EXPR_FULLSIZE: usize = ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Expr>();

pub const EXPR_REDUCEDSIZE: ::core::ffi::c_ulong = 44 as ::core::ffi::c_ulong;

pub const EXPR_TOKENONLYSIZE: ::core::ffi::c_ulong = 16 as ::core::ffi::c_ulong;

pub const EXPRDUP_REDUCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprList {
    pub nExpr: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
    pub a: [crate::src::headers::sqliteInt_h::ExprList_item; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprList_item {
    pub pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pub zEName: *mut ::core::ffi::c_char,
    pub fg: crate::src::headers::sqliteInt_h::__anon_struct_4,
    pub u: crate::src::headers::sqliteInt_h::__anon_union_9,
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anon_struct_4 {
    pub sortFlags: crate::src::ext::rtree::rtree::U8_0,
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
pub union __anon_union_9 {
    pub x: crate::src::headers::sqliteInt_h::__anon_struct_5,
    pub iConstExprReg: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_5 {
    pub iOrderByCol: crate::src::fts5::U16_0,
    pub iAlias: crate::src::fts5::U16_0,
}

pub const ENAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const ENAME_SPAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const ENAME_TAB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const ENAME_ROWID: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdList {
    pub nId: ::core::ffi::c_int,
    pub a: [crate::src::headers::sqliteInt_h::IdList_item; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdList_item {
    pub zName: *mut ::core::ffi::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Subquery {
    pub pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    pub addrFillSub: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
    pub regResult: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SrcItem {
    pub zName: *mut ::core::ffi::c_char,
    pub zAlias: *mut ::core::ffi::c_char,
    pub pSTab: *mut crate::src::headers::sqliteInt_h::Table,
    pub fg: crate::src::headers::sqliteInt_h::__anon_struct_6,
    pub iCursor: ::core::ffi::c_int,
    pub colUsed: crate::src::headers::sqliteInt_h::Bitmask,
    pub u1: crate::src::headers::sqliteInt_h::__anon_union_10,
    pub u2: crate::src::headers::sqliteInt_h::__anon_union_11,
    pub u3: crate::src::headers::sqliteInt_h::__anon_union_12,
    pub u4: crate::src::headers::sqliteInt_h::__anon_union_13,
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anon_struct_6 {
    pub jointype: crate::src::ext::rtree::rtree::U8_0,
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
pub union __anon_union_10 {
    pub zIndexedBy: *mut ::core::ffi::c_char,
    pub pFuncArg: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub nRow: crate::src::ext::rtree::rtree::U32_0,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_11 {
    pub pIBIndex: *mut crate::src::headers::sqliteInt_h::Index,
    pub pCteUse: *mut crate::src::headers::sqliteInt_h::CteUse,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_12 {
    pub pOn: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pUsing: *mut crate::src::headers::sqliteInt_h::IdList,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_13 {
    pub pSchema: *mut crate::src::headers::sqliteInt_h::Schema,
    pub zDatabase: *mut ::core::ffi::c_char,
    pub pSubq: *mut crate::src::headers::sqliteInt_h::Subquery,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnOrUsing {
    pub pOn: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pUsing: *mut crate::src::headers::sqliteInt_h::IdList,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SrcList {
    pub nSrc: ::core::ffi::c_int,
    pub nAlloc: crate::src::ext::rtree::rtree::U32_0,
    pub a: [crate::src::headers::sqliteInt_h::SrcItem; 0],
}

pub const SZ_SRCLIST_1: usize = 8_usize
    .wrapping_add(::core::mem::size_of::<crate::src::headers::sqliteInt_h::SrcItem>() as usize);

pub const JT_INNER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const JT_CROSS: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const JT_NATURAL: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const JT_LEFT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const JT_RIGHT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const JT_OUTER: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const JT_LTORJ: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const JT_ERROR: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const WHERE_ORDERBY_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const WHERE_ORDERBY_MIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const WHERE_ORDERBY_MAX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const WHERE_ONEPASS_DESIRED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const WHERE_ONEPASS_MULTIROW: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const WHERE_DUPLICATES_OK: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const WHERE_OR_SUBCLAUSE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const WHERE_GROUPBY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const WHERE_DISTINCTBY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const WHERE_WANT_DISTINCT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const WHERE_SORTBYGROUP: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const WHERE_AGG_DISTINCT: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const WHERE_ORDERBY_LIMIT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const WHERE_RIGHT_JOIN: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const WHERE_KEEP_ALL_JOINS: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const WHERE_USE_LIMIT: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const WHERE_DISTINCT_NOOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const WHERE_DISTINCT_UNIQUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const WHERE_DISTINCT_ORDERED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const WHERE_DISTINCT_UNORDERED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameContext {
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub pSrcList: *mut crate::src::headers::sqliteInt_h::SrcList,
    pub uNC: crate::src::headers::sqliteInt_h::__anon_union_14,
    pub pNext: *mut crate::src::headers::sqliteInt_h::NameContext,
    pub nRef: ::core::ffi::c_int,
    pub nNcErr: ::core::ffi::c_int,
    pub ncFlags: ::core::ffi::c_int,
    pub nNestedSelect: crate::src::ext::rtree::rtree::U32_0,
    pub pWinSelect: *mut crate::src::headers::sqliteInt_h::Select,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_14 {
    pub pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
    pub pUpsert: *mut crate::src::headers::sqliteInt_h::Upsert,
    pub iBaseReg: ::core::ffi::c_int,
}

pub const NC_AllowAgg: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const NC_PartIdx: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const NC_IsCheck: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const NC_GenCol: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const NC_HasAgg: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const NC_IdxExpr: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const NC_SelfRef: ::core::ffi::c_int = 0x2e as ::core::ffi::c_int;

pub const NC_Subquery: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const NC_UEList: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const NC_UUpsert: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const NC_UBaseReg: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const NC_MinMaxAgg: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const NC_AllowWin: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const NC_HasWin: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;

pub const NC_IsDDL: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

pub const NC_InAggFunc: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;

pub const NC_FromDDL: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;

pub const NC_NoSelect: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;

pub const NC_Where: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;

pub const NC_OrderAgg: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upsert {
    pub pUpsertTarget: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pUpsertTargetWhere: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pUpsertSet: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pUpsertWhere: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pNextUpsert: *mut crate::src::headers::sqliteInt_h::Upsert,
    pub isDoUpdate: crate::src::ext::rtree::rtree::U8_0,
    pub isDup: crate::src::ext::rtree::rtree::U8_0,
    pub pToFree: *mut ::core::ffi::c_void,
    pub pUpsertIdx: *mut crate::src::headers::sqliteInt_h::Index,
    pub pUpsertSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
    pub regData: ::core::ffi::c_int,
    pub iDataCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Select {
    pub op: crate::src::ext::rtree::rtree::U8_0,
    pub nSelectRow: crate::src::headers::sqliteInt_h::LogEst,
    pub selFlags: crate::src::ext::rtree::rtree::U32_0,
    pub iLimit: ::core::ffi::c_int,
    pub iOffset: ::core::ffi::c_int,
    pub selId: crate::src::ext::rtree::rtree::U32_0,
    pub addrOpenEphm: [::core::ffi::c_int; 2],
    pub pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
    pub pWhere: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pGroupBy: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pHaving: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pPrior: *mut crate::src::headers::sqliteInt_h::Select,
    pub pNext: *mut crate::src::headers::sqliteInt_h::Select,
    pub pLimit: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pWith: *mut crate::src::headers::sqliteInt_h::With,
    pub pWin: *mut crate::src::headers::sqliteInt_h::Window,
    pub pWinDefn: *mut crate::src::headers::sqliteInt_h::Window,
}

pub const SF_Distinct: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SF_All: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SF_Resolved: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const SF_Aggregate: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const SF_UsesEphemeral: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const SF_Expanded: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const SF_HasTypeInfo: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const SF_Compound: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const SF_Values: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const SF_MultiValue: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const SF_NestedFrom: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const SF_Recursive: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const SF_FixedLimit: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const SF_Converted: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

pub const SF_IncludeHidden: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;

pub const SF_ComplexResult: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;

pub const SF_WinRewrite: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;

pub const SF_View: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;

pub const SF_NoopOrderBy: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;

pub const SF_UFSrcCheck: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;

pub const SF_PushDown: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;

pub const SF_MultiPart: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;

pub const SF_CopyCte: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;

pub const SF_OrderByReqd: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;

pub const SF_UpdateFrom: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;

pub const SF_Correlated: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;

pub const SF_OnToWhere: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;

pub const SRT_Union: ::core::ffi::c_int = 1;

pub const SRT_Except: ::core::ffi::c_int = 2;

pub const SRT_Exists: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SRT_Exists_1: ::core::ffi::c_int = 3;

pub const SRT_Discard: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SRT_DistFifo: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SRT_DistQueue: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const SRT_Queue: ::core::ffi::c_int = 7;

pub const SRT_Fifo: ::core::ffi::c_int = 8;

pub const SRT_Output: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const SRT_Mem: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const SRT_Mem_1: ::core::ffi::c_int = 10;

pub const SRT_Set: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const SRT_EphemTab: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

pub const SRT_EphemTab_1: ::core::ffi::c_int = 12;

pub const SRT_Coroutine: ::core::ffi::c_int = 13 as ::core::ffi::c_int;

pub const SRT_Table: ::core::ffi::c_int = 14;

pub const SRT_Table_1: ::core::ffi::c_int = 14 as ::core::ffi::c_int;

pub const SRT_Upfrom: ::core::ffi::c_int = 15;

pub const SRT_Upfrom_1: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SelectDest {
    pub eDest: crate::src::ext::rtree::rtree::U8_0,
    pub iSDParm: ::core::ffi::c_int,
    pub iSDParm2: ::core::ffi::c_int,
    pub iSdst: ::core::ffi::c_int,
    pub nSdst: ::core::ffi::c_int,
    pub zAffSdst: *mut ::core::ffi::c_char,
    pub pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AutoincInfo {
    pub pNext: *mut crate::src::headers::sqliteInt_h::AutoincInfo,
    pub pTab: *mut crate::src::headers::sqliteInt_h::Table,
    pub iDb: ::core::ffi::c_int,
    pub regCtr: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerPrg {
    pub pTrigger: *mut crate::src::headers::sqliteInt_h::Trigger,
    pub pNext: *mut crate::src::headers::sqliteInt_h::TriggerPrg,
    pub pProgram: *mut crate::src::src::vdbe::SubProgram,
    pub orconf: ::core::ffi::c_int,
    pub aColmask: [crate::src::ext::rtree::rtree::U32_0; 2],
}

pub type YDbMask = ::core::ffi::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexedExpr {
    pub pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pub iDataCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int,
    pub iIdxCol: ::core::ffi::c_int,
    pub bMaybeNullRow: crate::src::ext::rtree::rtree::U8_0,
    pub aff: crate::src::ext::rtree::rtree::U8_0,
    pub pIENext: *mut crate::src::headers::sqliteInt_h::IndexedExpr,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseCleanup {
    pub pNext: *mut crate::src::headers::sqliteInt_h::ParseCleanup,
    pub pPtr: *mut ::core::ffi::c_void,
    pub xCleanup: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Parse {
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub pVdbe: *mut crate::src::headers::vdbeInt_h::Vdbe,
    pub rc: ::core::ffi::c_int,
    pub nQueryLoop: crate::src::headers::sqliteInt_h::LogEst,
    pub nested: crate::src::ext::rtree::rtree::U8_0,
    pub nTempReg: crate::src::ext::rtree::rtree::U8_0,
    pub isMultiWrite: crate::src::ext::rtree::rtree::U8_0,
    pub mayAbort: crate::src::ext::rtree::rtree::U8_0,
    pub hasCompound: crate::src::ext::rtree::rtree::U8_0,
    pub disableLookaside: crate::src::ext::rtree::rtree::U8_0,
    pub prepFlags: crate::src::ext::rtree::rtree::U8_0,
    pub withinRJSubrtn: crate::src::ext::rtree::rtree::U8_0,
    pub bHasExists: crate::src::ext::rtree::rtree::U8_0,
    pub mSubrtnSig: crate::src::ext::rtree::rtree::U8_0,
    pub eTriggerOp: crate::src::ext::rtree::rtree::U8_0,
    pub bReturning: crate::src::ext::rtree::rtree::U8_0,
    pub eOrconf: crate::src::ext::rtree::rtree::U8_0,
    pub disableTriggers: crate::src::ext::rtree::rtree::U8_0,
    #[bitfield(name = "colNamesSet", ty = "Bft", bits = "0..=0")]
    #[bitfield(name = "bHasWith", ty = "Bft", bits = "1..=1")]
    #[bitfield(name = "okConstFactor", ty = "Bft", bits = "2..=2")]
    #[bitfield(name = "checkSchema", ty = "Bft", bits = "3..=3")]
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
    pub pConstExpr: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pIdxEpr: *mut crate::src::headers::sqliteInt_h::IndexedExpr,
    pub pIdxPartExpr: *mut crate::src::headers::sqliteInt_h::IndexedExpr,
    pub writeMask: crate::src::headers::sqliteInt_h::YDbMask,
    pub cookieMask: crate::src::headers::sqliteInt_h::YDbMask,
    pub nMaxArg: ::core::ffi::c_int,
    pub nSelect: ::core::ffi::c_int,
    pub nProgressSteps: crate::src::ext::rtree::rtree::U32_0,
    pub nTableLock: ::core::ffi::c_int,
    pub aTableLock: *mut crate::src::headers::sqliteInt_h::TableLock,
    pub pAinc: *mut crate::src::headers::sqliteInt_h::AutoincInfo,
    pub pToplevel: *mut crate::src::headers::sqliteInt_h::Parse,
    pub pTriggerTab: *mut crate::src::headers::sqliteInt_h::Table,
    pub pTriggerPrg: *mut crate::src::headers::sqliteInt_h::TriggerPrg,
    pub pCleanup: *mut crate::src::headers::sqliteInt_h::ParseCleanup,
    pub aTempReg: [::core::ffi::c_int; 8],
    pub pOuterParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub sNameToken: crate::src::headers::sqliteInt_h::Token,
    pub oldmask: crate::src::ext::rtree::rtree::U32_0,
    pub newmask: crate::src::ext::rtree::rtree::U32_0,
    pub u1: crate::src::headers::sqliteInt_h::__anon_union_15,
    pub sLastToken: crate::src::headers::sqliteInt_h::Token,
    pub nVar: crate::src::headers::sqliteInt_h::YnVar,
    pub iPkSortOrder: crate::src::ext::rtree::rtree::U8_0,
    pub explain: crate::src::ext::rtree::rtree::U8_0,
    pub eParseMode: crate::src::ext::rtree::rtree::U8_0,
    pub nVtabLock: ::core::ffi::c_int,
    pub nHeight: ::core::ffi::c_int,
    pub addrExplain: ::core::ffi::c_int,
    pub pVList: *mut crate::src::headers::sqliteInt_h::VList,
    pub pReprepare: *mut crate::src::headers::vdbeInt_h::Vdbe,
    pub zTail: *const ::core::ffi::c_char,
    pub pNewTable: *mut crate::src::headers::sqliteInt_h::Table,
    pub pNewIndex: *mut crate::src::headers::sqliteInt_h::Index,
    pub pNewTrigger: *mut crate::src::headers::sqliteInt_h::Trigger,
    pub zAuthContext: *const ::core::ffi::c_char,
    pub sArg: crate::src::headers::sqliteInt_h::Token,
    pub apVtabLock: *mut *mut crate::src::headers::sqliteInt_h::Table,
    pub pWith: *mut crate::src::headers::sqliteInt_h::With,
    pub pRename: *mut crate::src::headers::sqliteInt_h::RenameToken,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_15 {
    pub cr: crate::src::headers::sqliteInt_h::__anon_struct_7,
    pub d: crate::src::headers::sqliteInt_h::__anon_struct_8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_7 {
    pub addrCrTab: ::core::ffi::c_int,
    pub regRowid: ::core::ffi::c_int,
    pub regRoot: ::core::ffi::c_int,
    pub constraintName: crate::src::headers::sqliteInt_h::Token,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_8 {
    pub pReturning: *mut crate::src::headers::sqliteInt_h::Returning,
}

pub const PARSE_MODE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const PARSE_MODE_DECLARE_VTAB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const PARSE_MODE_RENAME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const PARSE_MODE_UNMAP: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const PARSE_HDR_SZ: ::core::ffi::c_ulong =
    (192 as ::core::ffi::c_ulong).wrapping_sub(8 as ::core::ffi::c_ulong);

pub const PARSE_RECURSE_SZ: ::core::ffi::c_ulong = 288 as ::core::ffi::c_ulong;

pub const PARSE_TAIL_SZ: usize = (::core::mem::size_of::<crate::src::headers::sqliteInt_h::Parse>()
    as usize)
    .wrapping_sub(crate::src::headers::sqliteInt_h::PARSE_RECURSE_SZ as usize);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuthContext {
    pub zAuthContext: *const ::core::ffi::c_char,
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
}

pub const OPFLAG_NCHANGE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const OPFLAG_NOCHNG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const OPFLAG_LASTROWID: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const OPFLAG_ISUPDATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const OPFLAG_APPEND: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const OPFLAG_USESEEKRESULT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const OPFLAG_ISNOOP: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const OPFLAG_TYPEOFARG: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const OPFLAG_BYTELENARG: ::core::ffi::c_int = 0xc0 as ::core::ffi::c_int;

pub const OPFLAG_BULKCSR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const OPFLAG_SEEKEQ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const OPFLAG_FORDELETE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const OPFLAG_P2ISREG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const OPFLAG_PERMUTE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const OPFLAG_SAVEPOSITION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const OPFLAG_AUXDELETE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const OPFLAG_PREFORMAT: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Trigger {
    pub zName: *mut ::core::ffi::c_char,
    pub table: *mut ::core::ffi::c_char,
    pub op: crate::src::ext::rtree::rtree::U8_0,
    pub tr_tm: crate::src::ext::rtree::rtree::U8_0,
    pub bReturning: crate::src::ext::rtree::rtree::U8_0,
    pub pWhen: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pColumns: *mut crate::src::headers::sqliteInt_h::IdList,
    pub pSchema: *mut crate::src::headers::sqliteInt_h::Schema,
    pub pTabSchema: *mut crate::src::headers::sqliteInt_h::Schema,
    pub step_list: *mut crate::src::headers::sqliteInt_h::TriggerStep,
    pub pNext: *mut crate::src::headers::sqliteInt_h::Trigger,
}

pub const TRIGGER_BEFORE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const TRIGGER_AFTER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerStep {
    pub op: crate::src::ext::rtree::rtree::U8_0,
    pub orconf: crate::src::ext::rtree::rtree::U8_0,
    pub pTrig: *mut crate::src::headers::sqliteInt_h::Trigger,
    pub pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    pub zTarget: *mut ::core::ffi::c_char,
    pub pFrom: *mut crate::src::headers::sqliteInt_h::SrcList,
    pub pWhere: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pExprList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pIdList: *mut crate::src::headers::sqliteInt_h::IdList,
    pub pUpsert: *mut crate::src::headers::sqliteInt_h::Upsert,
    pub zSpan: *mut ::core::ffi::c_char,
    pub pNext: *mut crate::src::headers::sqliteInt_h::TriggerStep,
    pub pLast: *mut crate::src::headers::sqliteInt_h::TriggerStep,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Returning {
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub pReturnEL: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub retTrig: crate::src::headers::sqliteInt_h::Trigger,
    pub retTStep: crate::src::headers::sqliteInt_h::TriggerStep,
    pub iRetCur: ::core::ffi::c_int,
    pub nRetCol: ::core::ffi::c_int,
    pub iRetReg: ::core::ffi::c_int,
    pub zName: [::core::ffi::c_char; 40],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_str {
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub zText: *mut ::core::ffi::c_char,
    pub nAlloc: crate::src::ext::rtree::rtree::U32_0,
    pub mxAlloc: crate::src::ext::rtree::rtree::U32_0,
    pub nChar: crate::src::ext::rtree::rtree::U32_0,
    pub accError: crate::src::ext::rtree::rtree::U8_0,
    pub printfFlags: crate::src::ext::rtree::rtree::U8_0,
}

pub const SQLITE_PRINTF_INTERNAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_PRINTF_SQLFUNC: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SQLITE_PRINTF_MALLOCED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RCStr {
    pub nRCRef: crate::src::ext::rtree::rtree::U64_0,
}

pub type InitData = crate::src::headers::sqliteInt_h::__anon_struct_9;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_9 {
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub pzErrMsg: *mut *mut ::core::ffi::c_char,
    pub iDb: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub mInitFlags: crate::src::ext::rtree::rtree::U32_0,
    pub nInitRow: crate::src::ext::rtree::rtree::U32_0,
    pub mxPage: crate::src::src::pager::Pgno,
}

pub const INITFLAG_AlterMask: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;

pub const INITFLAG_AlterRename: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const INITFLAG_AlterDrop: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const INITFLAG_AlterAdd: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sqlite3Config {
    pub bMemstat: ::core::ffi::c_int,
    pub bCoreMutex: crate::src::ext::rtree::rtree::U8_0,
    pub bFullMutex: crate::src::ext::rtree::rtree::U8_0,
    pub bOpenUri: crate::src::ext::rtree::rtree::U8_0,
    pub bUseCis: crate::src::ext::rtree::rtree::U8_0,
    pub bSmallMalloc: crate::src::ext::rtree::rtree::U8_0,
    pub bExtraSchemaChecks: crate::src::ext::rtree::rtree::U8_0,
    pub mxStrlen: ::core::ffi::c_int,
    pub neverCorrupt: ::core::ffi::c_int,
    pub szLookaside: ::core::ffi::c_int,
    pub nLookaside: ::core::ffi::c_int,
    pub nStmtSpill: ::core::ffi::c_int,
    pub m: crate::src::headers::sqlite3_h::sqlite3_mem_methods,
    pub mutex: crate::src::headers::sqlite3_h::sqlite3_mutex_methods,
    pub pcache2: crate::src::headers::sqlite3_h::sqlite3_pcache_methods2,
    pub pHeap: *mut ::core::ffi::c_void,
    pub nHeap: ::core::ffi::c_int,
    pub mnReq: ::core::ffi::c_int,
    pub mxReq: ::core::ffi::c_int,
    pub szMmap: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub mxMmap: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub pPage: *mut ::core::ffi::c_void,
    pub szPage: ::core::ffi::c_int,
    pub nPage: ::core::ffi::c_int,
    pub mxParserStack: ::core::ffi::c_int,
    pub sharedCacheEnabled: ::core::ffi::c_int,
    pub szPma: crate::src::ext::rtree::rtree::U32_0,
    pub isInit: ::core::ffi::c_int,
    pub inProgress: ::core::ffi::c_int,
    pub isMutexInit: ::core::ffi::c_int,
    pub isMallocInit: ::core::ffi::c_int,
    pub isPCacheInit: ::core::ffi::c_int,
    pub nRefInitMutex: ::core::ffi::c_int,
    pub pInitMutex: *mut crate::src::src::mutex_unix::sqlite3_mutex,
    pub xLog: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub pLogArg: *mut ::core::ffi::c_void,
    pub mxMemdbSize: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub xTestCallback: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub bLocaltimeFault: ::core::ffi::c_int,
    pub xAltLocaltime: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub iOnceResetThreshold: ::core::ffi::c_int,
    pub szSorterRef: crate::src::ext::rtree::rtree::U32_0,
    pub iPrngSeed: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Walker {
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub xExprCallback: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::Walker,
            *mut crate::src::headers::sqliteInt_h::Expr,
        ) -> ::core::ffi::c_int,
    >,
    pub xSelectCallback: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::Walker,
            *mut crate::src::headers::sqliteInt_h::Select,
        ) -> ::core::ffi::c_int,
    >,
    pub xSelectCallback2: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::Walker,
            *mut crate::src::headers::sqliteInt_h::Select,
        ) -> (),
    >,
    pub walkerDepth: ::core::ffi::c_int,
    pub eCode: crate::src::fts5::U16_0,
    pub mWFlags: crate::src::fts5::U16_0,
    pub u: crate::src::headers::sqliteInt_h::__anon_union_16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_16 {
    pub pNC: *mut crate::src::headers::sqliteInt_h::NameContext,
    pub n: ::core::ffi::c_int,
    pub iCur: ::core::ffi::c_int,
    pub pSrcList: *mut crate::src::headers::sqliteInt_h::SrcList,
    pub pCCurHint: *mut crate::src::headers::sqliteInt_h::CCurHint,
    pub pRefSrcList: *mut crate::src::headers::sqliteInt_h::RefSrcList,
    pub aiCol: *mut ::core::ffi::c_int,
    pub pIdxCover: *mut crate::src::headers::sqliteInt_h::IdxCover,
    pub pGroupBy: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    pub pRewrite: *mut crate::src::headers::sqliteInt_h::WindowRewrite,
    pub pConst: *mut crate::src::headers::sqliteInt_h::WhereConst,
    pub pRename: *mut RenameCtx,
    pub pTab: *mut crate::src::headers::sqliteInt_h::Table,
    pub pCovIdxCk: *mut crate::src::headers::sqliteInt_h::CoveringIndexCheck,
    pub pSrcItem: *mut crate::src::headers::sqliteInt_h::SrcItem,
    pub pFix: *mut crate::src::headers::sqliteInt_h::DbFixer,
    pub aMem: *mut crate::src::src::vdbe::Mem,
    pub pCheckOnCtx: *mut crate::src::headers::sqliteInt_h::CheckOnCtx,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbFixer {
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub w: crate::src::headers::sqliteInt_h::Walker,
    pub pSchema: *mut crate::src::headers::sqliteInt_h::Schema,
    pub bTemp: crate::src::ext::rtree::rtree::U8_0,
    pub zDb: *const ::core::ffi::c_char,
    pub zType: *const ::core::ffi::c_char,
    pub pName: *const crate::src::headers::sqliteInt_h::Token,
}

pub const WRC_Continue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const WRC_Prune: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const WRC_Abort: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cte {
    pub zName: *mut ::core::ffi::c_char,
    pub pCols: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    pub zCteErr: *const ::core::ffi::c_char,
    pub pUse: *mut crate::src::headers::sqliteInt_h::CteUse,
    pub eM10d: crate::src::ext::rtree::rtree::U8_0,
}

pub const M10d_Yes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const M10d_Any: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const M10d_No: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct With {
    pub nCte: ::core::ffi::c_int,
    pub bView: ::core::ffi::c_int,
    pub pOuter: *mut crate::src::headers::sqliteInt_h::With,
    pub a: [crate::src::headers::sqliteInt_h::Cte; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CteUse {
    pub nUse: ::core::ffi::c_int,
    pub addrM9e: ::core::ffi::c_int,
    pub regRtn: ::core::ffi::c_int,
    pub iCur: ::core::ffi::c_int,
    pub nRowEst: crate::src::headers::sqliteInt_h::LogEst,
    pub eM10d: crate::src::ext::rtree::rtree::U8_0,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbClientData {
    pub pNext: *mut crate::src::headers::sqliteInt_h::DbClientData,
    pub pData: *mut ::core::ffi::c_void,
    pub xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub zName: [::core::ffi::c_char; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Window {
    pub zName: *mut ::core::ffi::c_char,
    pub zBase: *mut ::core::ffi::c_char,
    pub pPartition: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub eFrmType: crate::src::ext::rtree::rtree::U8_0,
    pub eStart: crate::src::ext::rtree::rtree::U8_0,
    pub eEnd: crate::src::ext::rtree::rtree::U8_0,
    pub bImplicitFrame: crate::src::ext::rtree::rtree::U8_0,
    pub eExclude: crate::src::ext::rtree::rtree::U8_0,
    pub pStart: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pEnd: *mut crate::src::headers::sqliteInt_h::Expr,
    pub ppThis: *mut *mut crate::src::headers::sqliteInt_h::Window,
    pub pNextWin: *mut crate::src::headers::sqliteInt_h::Window,
    pub pFilter: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pWFunc: *mut crate::src::headers::sqliteInt_h::FuncDef,
    pub iEphCsr: ::core::ffi::c_int,
    pub regAccum: ::core::ffi::c_int,
    pub regResult: ::core::ffi::c_int,
    pub csrApp: ::core::ffi::c_int,
    pub regApp: ::core::ffi::c_int,
    pub regPart: ::core::ffi::c_int,
    pub pOwner: *mut crate::src::headers::sqliteInt_h::Expr,
    pub nBufferCol: ::core::ffi::c_int,
    pub iArgCol: ::core::ffi::c_int,
    pub regOne: ::core::ffi::c_int,
    pub regStartRowid: ::core::ffi::c_int,
    pub regEndRowid: ::core::ffi::c_int,
    pub bExprArgs: crate::src::ext::rtree::rtree::U8_0,
}

pub const SQLITE_NOMEM_BKPT: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_NOMEM;

pub const SQLITE_IOERR_NOMEM_BKPT: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR_NOMEM;

pub const EXP754: crate::src::ext::rtree::rtree::U64_0 = (0x7ff as ::core::ffi::c_int
    as crate::src::ext::rtree::rtree::U64_0)
    << 52 as ::core::ffi::c_int;

pub const MAN754: crate::src::ext::rtree::rtree::U64_0 =
    ((1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U64_0) << 52 as ::core::ffi::c_int)
        .wrapping_sub(1 as crate::src::ext::rtree::rtree::U64_0);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrintfArguments {
    pub nArg: ::core::ffi::c_int,
    pub nUsed: ::core::ffi::c_int,
    pub apArg: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpDecode {
    pub sign: ::core::ffi::c_char,
    pub isSpecial: ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub iDP: ::core::ffi::c_int,
    pub z: *mut ::core::ffi::c_char,
    pub zBuf: [::core::ffi::c_char; 24],
}

pub const ONEPASS_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const ONEPASS_SINGLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const ONEPASS_MULTI: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_ECEL_DUP: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_ECEL_FACTOR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SQLITE_ECEL_REF: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const SQLITE_ECEL_OMITREF: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const LOCATE_VIEW: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const LOCATE_NOERR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const IN_INDEX_ROWID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const IN_INDEX_EPH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const IN_INDEX_INDEX_ASC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const IN_INDEX_INDEX_DESC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const IN_INDEX_NOOP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const IN_INDEX_NOOP_OK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const IN_INDEX_MEMBERSHIP: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const IN_INDEX_LOOP: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
