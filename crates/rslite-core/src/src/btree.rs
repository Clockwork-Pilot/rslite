// =============== BEGIN btree_h ================
pub const SQLITE_DEFAULT_AUTOVACUUM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const BTREE_AUTOVACUUM_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const BTREE_AUTOVACUUM_FULL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const BTREE_AUTOVACUUM_INCR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const BTREE_OMIT_JOURNAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const BTREE_MEMORY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const BTREE_SINGLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const BTREE_UNORDERED: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const BTREE_INTKEY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const BTREE_BLOBKEY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const BTREE_FREE_PAGE_COUNT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const BTREE_SCHEMA_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const BTREE_FILE_FORMAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const BTREE_DEFAULT_CACHE_SIZE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const BTREE_LARGEST_ROOT_PAGE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const BTREE_TEXT_ENCODING: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const BTREE_USER_VERSION: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const BTREE_INCR_VACUUM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const BTREE_APPLICATION_ID: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const BTREE_DATA_VERSION: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

pub const BTREE_BULKLOAD: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const BTREE_SEEK_EQ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const BTREE_WRCSR: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const BTREE_SAVEPOSITION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const BTREE_AUXDELETE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const BTREE_APPEND: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const BTREE_PREFORMAT: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BtreePayload {
    pub pKey: *const ::core::ffi::c_void,
    pub nKey: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub pData: *const ::core::ffi::c_void,
    pub aMem: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pub nMem: crate::src::fts5::U16_0,
    pub nData: ::core::ffi::c_int,
    pub nZero: ::core::ffi::c_int,
}
pub use crate::__stddef_size_t_h::SizeT;
pub use crate::src::headers::stdlib::VaList;

pub use crate::fts3Int_h::LARGEST_INT64;
pub use crate::internal::__ATOMIC_RELAXED;
pub use crate::internal::BuiltinVaList;
pub use crate::internal::VaListTag;
pub use crate::internal::SQLITE_THREADSAFE;
pub use crate::pcache_h::PCache;
pub use crate::sqliteLimit_h::SQLITE_DEFAULT_CACHE_SIZE;
pub use crate::sqliteLimit_h::SQLITE_MAX_LENGTH;
pub use crate::sqliteLimit_h::SQLITE_MAX_PAGE_SIZE;
pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::I16_0;
pub use crate::src::fts5::U16_0;
pub use crate::src::headers::btreeInt_h::BTCF_AtLast;
pub use crate::src::headers::btreeInt_h::BTCF_Incrblob;
pub use crate::src::headers::btreeInt_h::BTCF_Multiple;
pub use crate::src::headers::btreeInt_h::BTCF_Pinned;
pub use crate::src::headers::btreeInt_h::BTCF_ValidNKey;
pub use crate::src::headers::btreeInt_h::BTCF_ValidOvfl;
pub use crate::src::headers::btreeInt_h::BTCF_WriteFlag;
pub use crate::src::headers::btreeInt_h::BTCURSOR_MAX_DEPTH;
pub use crate::src::headers::btreeInt_h::BTS_EXCLUSIVE;
pub use crate::src::headers::btreeInt_h::BTS_FAST_SECURE;
pub use crate::src::headers::btreeInt_h::BTS_INITIALLY_EMPTY;
pub use crate::src::headers::btreeInt_h::BTS_NO_WAL;
pub use crate::src::headers::btreeInt_h::BTS_PAGESIZE_FIXED;
pub use crate::src::headers::btreeInt_h::BTS_PENDING;
pub use crate::src::headers::btreeInt_h::BTS_READ_ONLY;
pub use crate::src::headers::btreeInt_h::BTS_SECURE_DELETE;
pub use crate::src::headers::btreeInt_h::BtCursor;
pub use crate::src::headers::btreeInt_h::BtLock;
pub use crate::src::headers::btreeInt_h::BtShared;
pub use crate::src::headers::btreeInt_h::Btree;
pub use crate::src::headers::btreeInt_h::CURSOR_FAULT;
pub use crate::src::headers::btreeInt_h::CURSOR_INVALID;
pub use crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK;
pub use crate::src::headers::btreeInt_h::CURSOR_SKIPNEXT;
pub use crate::src::headers::btreeInt_h::CURSOR_VALID;
pub use crate::src::headers::btreeInt_h::CellInfo;
pub use crate::src::headers::btreeInt_h::IntegrityCk;
pub use crate::src::headers::btreeInt_h::MemPage;
pub use crate::src::headers::btreeInt_h::PTF_INTKEY;
pub use crate::src::headers::btreeInt_h::PTF_LEAF;
pub use crate::src::headers::btreeInt_h::PTF_LEAFDATA;
pub use crate::src::headers::btreeInt_h::PTF_ZERODATA;
pub use crate::src::headers::btreeInt_h::PTRMAP_BTREE;
pub use crate::src::headers::btreeInt_h::PTRMAP_FREEPAGE;
pub use crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW1;
pub use crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW2;
pub use crate::src::headers::btreeInt_h::PTRMAP_ROOTPAGE;
pub use crate::src::headers::btreeInt_h::READ_LOCK;
pub use crate::src::headers::btreeInt_h::SQLITE_FILE_HEADER;
pub use crate::src::headers::btreeInt_h::TRANS_NONE;
pub use crate::src::headers::btreeInt_h::TRANS_READ;
pub use crate::src::headers::btreeInt_h::TRANS_WRITE;
pub use crate::src::headers::btreeInt_h::WRITE_LOCK;
pub use crate::src::headers::sqlite3_h::SQLITE_ABORT;
pub use crate::src::headers::sqlite3_h::SQLITE_BUSY;
pub use crate::src::headers::sqlite3_h::SQLITE_BUSY_SNAPSHOT;
pub use crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
pub use crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT_PINNED;
pub use crate::src::headers::sqlite3_h::SQLITE_DONE;
pub use crate::src::headers::sqlite3_h::SQLITE_EMPTY;
pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_PDB;
pub use crate::src::headers::sqlite3_h::SQLITE_INTERRUPT;
pub use crate::src::headers::sqlite3_h::SQLITE_IOERR;
pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_LOCKED;
pub use crate::src::headers::sqlite3_h::SQLITE_LOCKED_SHAREDCACHE;
pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_FAST;
pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_MAIN;
pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_OPEN;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_NOTADB;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_OK_SYMLINK;
pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_DB;
pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_MEMORY;
pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_SHAREDCACHE;
pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_TEMP_DB;
pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_URI;
pub use crate::src::headers::sqlite3_h::SQLITE_READONLY;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::sqlite3_file;
pub use crate::src::headers::sqlite3_h::Sqlite3Filename;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use crate::src::headers::sqlite3_h::sqlite3_index_info;
pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_module;
pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;
pub use crate::src::headers::sqlite3_h::Sqlite3Pcache;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;
pub use crate::src::headers::sqlite3_h::Sqlite3SyscallPtr;
pub use crate::src::headers::sqlite3_h::Sqlite3Uint64;
pub use crate::src::headers::sqlite3_h::sqlite3_vfs;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;
pub use crate::src::headers::sqliteInt_h::__anon_struct_0;
pub use crate::src::headers::sqliteInt_h::__anon_struct_1;
pub use crate::src::headers::sqliteInt_h::__anon_struct_2;
pub use crate::src::headers::sqliteInt_h::__anon_struct_3;
pub use crate::src::headers::sqliteInt_h::__anon_struct_4;
pub use crate::src::headers::sqliteInt_h::__anon_struct_5;
pub use crate::src::headers::sqliteInt_h::__anon_struct_6;
pub use crate::src::headers::sqliteInt_h::__anon_struct_7;
pub use crate::src::headers::sqliteInt_h::__anon_struct_8;
pub use crate::src::headers::sqliteInt_h::__anon_union_0;
pub use crate::src::headers::sqliteInt_h::__anon_union_1;
pub use crate::src::headers::sqliteInt_h::__anon_union_2;
pub use crate::src::headers::sqliteInt_h::__anon_union_3;
pub use crate::src::headers::sqliteInt_h::__anon_union_4;
pub use crate::src::headers::sqliteInt_h::__anon_union_5;
pub use crate::src::headers::sqliteInt_h::__anon_union_6;
pub use crate::src::headers::sqliteInt_h::__anon_union_7;
pub use crate::src::headers::sqliteInt_h::__anon_union_8;
pub use crate::src::headers::sqliteInt_h::__anon_union_9;
pub use crate::src::headers::sqliteInt_h::__anon_union_10;
pub use crate::src::headers::sqliteInt_h::__anon_union_11;
pub use crate::src::headers::sqliteInt_h::__anon_union_12;
pub use crate::src::headers::sqliteInt_h::__anon_union_13;
pub use crate::src::headers::sqliteInt_h::__anon_union_15;
pub use crate::src::headers::sqliteInt_h::AggInfo;
pub use crate::src::headers::sqliteInt_h::AggInfo_col;
pub use crate::src::headers::sqliteInt_h::AggInfo_func;
pub use crate::src::headers::sqliteInt_h::AutoincInfo;
pub use crate::src::headers::sqliteInt_h::Bitmask;
pub use crate::src::headers::sqliteInt_h::BusyHandler;
pub use crate::src::headers::sqliteInt_h::CollSeq;
pub use crate::src::headers::sqliteInt_h::Column;
pub use crate::src::headers::sqliteInt_h::Cte;
pub use crate::src::headers::sqliteInt_h::CteUse;
pub use crate::src::headers::sqliteInt_h::Db;
pub use crate::src::headers::sqliteInt_h::DbClientData;
pub use crate::src::headers::sqliteInt_h::Expr;
pub use crate::src::headers::sqliteInt_h::ExprList;
pub use crate::src::headers::sqliteInt_h::ExprList_item;
pub use crate::src::headers::sqliteInt_h::FKey;
pub use crate::src::headers::sqliteInt_h::FuncDef;
pub use crate::src::headers::sqliteInt_h::FuncDestructor;
pub use crate::src::headers::sqliteInt_h::IdList;
pub use crate::src::headers::sqliteInt_h::IdList_item;
pub use crate::src::headers::sqliteInt_h::Index;
pub use crate::src::headers::sqliteInt_h::IndexedExpr;
pub use crate::src::headers::sqliteInt_h::KeyInfo;
pub use crate::src::headers::sqliteInt_h::LogEst;
pub use crate::src::headers::sqliteInt_h::Lookaside;
pub use crate::src::headers::sqliteInt_h::LookasideSlot;
pub use crate::src::headers::sqliteInt_h::Module;
pub use crate::src::headers::sqliteInt_h::Parse;
pub use crate::src::headers::sqliteInt_h::ParseCleanup;
pub use crate::src::headers::sqliteInt_h::RenameToken;
pub use crate::src::headers::sqliteInt_h::Returning;
pub use crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK;
pub use crate::src::headers::sqliteInt_h::SCHEMA_ROOT;
pub use crate::src::headers::sqliteInt_h::SQLITE_CellSizeCk;
pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
pub use crate::src::headers::sqliteInt_h::SQLITE_PRINTF_INTERNAL;
pub use crate::src::headers::sqliteInt_h::SQLITE_ResetDatabase;
pub use crate::src::headers::sqliteInt_h::Savepoint;
pub use crate::src::headers::sqliteInt_h::Schema;
pub use crate::src::headers::sqliteInt_h::Select;
pub use crate::src::headers::sqliteInt_h::Sqlite3Config;
pub use crate::src::headers::sqliteInt_h::SrcItem;
pub use crate::src::headers::sqliteInt_h::SrcList;
pub use crate::src::headers::sqliteInt_h::StrAccum;
pub use crate::src::headers::sqliteInt_h::Subquery;
pub use crate::src::headers::sqliteInt_h::Table;
pub use crate::src::headers::sqliteInt_h::TableLock;
pub use crate::src::headers::sqliteInt_h::Token;
pub use crate::src::headers::sqliteInt_h::Trigger;
pub use crate::src::headers::sqliteInt_h::TriggerPrg;
pub use crate::src::headers::sqliteInt_h::TriggerStep;
pub use crate::src::headers::sqliteInt_h::UnpackedRecord;
pub use crate::src::headers::sqliteInt_h::Upsert;
pub use crate::src::headers::sqliteInt_h::VList;
pub use crate::src::headers::sqliteInt_h::VTable;
pub use crate::src::headers::sqliteInt_h::VtabCtx;
pub use crate::src::headers::sqliteInt_h::Window;
pub use crate::src::headers::sqliteInt_h::With;
pub use crate::src::headers::sqliteInt_h::Bft;
pub use crate::src::headers::sqliteInt_h::I8_0;
pub use crate::src::headers::sqliteInt_h::sColMap;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::headers::sqliteInt_h::sqlite3_str;
pub use crate::src::headers::sqliteInt_h::Sqlite3Xauth;
pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;
pub use crate::src::headers::sqliteInt_h::Uptr;
pub use crate::src::headers::sqliteInt_h::YDbMask;
pub use crate::src::headers::sqliteInt_h::YnVar;
pub use crate::src::headers::stdlib::Int8T;
pub use crate::src::headers::stdlib::Int16T;
pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint16T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::headers::stdlib::IntptrT;
pub use crate::src::headers::stdlib::UintptrT;
pub use crate::src::headers::vdbeInt_h::PreUpdate;
pub use crate::src::headers::vdbeInt_h::Vdbe;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::src::bitvec::Bitvec;
pub use crate::src::src::bitvec::sqlite3BitvecCreate;
pub use crate::src::src::bitvec::sqlite3BitvecDestroy;
pub use crate::src::src::bitvec::sqlite3BitvecSet;
pub use crate::src::src::bitvec::sqlite3BitvecSize;
pub use crate::src::src::bitvec::sqlite3BitvecTestNotNull;
pub use crate::src::src::btmutex::sqlite3BtreeEnter;
pub use crate::src::src::btmutex::sqlite3BtreeLeave;
pub use crate::src::src::build::sqlite3WritableSchema;
pub use crate::src::src::global::sqlite3Config;
pub use crate::src::src::global::sqlite3PendingByte;
pub use crate::src::src::hash::_ht;
pub use crate::src::src::hash::Hash;
pub use crate::src::src::hash::HashElem;
pub use crate::src::src::main::sqlite3CorruptError;
pub use crate::src::src::main::sqlite3InvokeBusyHandler;
pub use crate::src::src::main::sqlite3TempInMemory;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3DbFree;
pub use crate::src::src::malloc::sqlite3DbMallocRaw;
pub use crate::src::src::malloc::sqlite3DbMallocZero;
pub use crate::src::src::malloc::sqlite3Malloc;
pub use crate::src::src::malloc::sqlite3MallocSize;
pub use crate::src::src::malloc::sqlite3MallocZero;
pub use crate::src::src::malloc::sqlite3Realloc;
pub use crate::src::src::mutex::sqlite3_mutex_enter;
pub use crate::src::src::mutex::sqlite3_mutex_free;
pub use crate::src::src::mutex::sqlite3_mutex_leave;
pub use crate::src::src::mutex::sqlite3MutexAlloc;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::pager::DbPage;
pub use crate::src::src::pager::PAGER_GET_NOCONTENT;
pub use crate::src::src::pager::PAGER_GET_READONLY;
pub use crate::src::src::pager::Pager;
pub use crate::src::src::pager::Pgno;
pub use crate::src::src::pager::sqlite3PagerBegin;
pub use crate::src::src::pager::sqlite3PagerCheckpoint;
pub use crate::src::src::pager::sqlite3PagerClearCache;
pub use crate::src::src::pager::sqlite3PagerClose;
pub use crate::src::src::pager::sqlite3PagerCommitPhaseOne;
pub use crate::src::src::pager::sqlite3PagerCommitPhaseTwo;
pub use crate::src::src::pager::sqlite3PagerDataVersion;
pub use crate::src::src::pager::sqlite3PagerDirectReadOk;
pub use crate::src::src::pager::sqlite3PagerDontWrite;
pub use crate::src::src::pager::sqlite3PagerFile;
pub use crate::src::src::pager::sqlite3PagerFilename;
pub use crate::src::src::pager::sqlite3PagerGet;
pub use crate::src::src::pager::sqlite3PagerGetData;
pub use crate::src::src::pager::sqlite3PagerGetExtra;
pub use crate::src::src::pager::sqlite3PagerIsreadonly;
pub use crate::src::src::pager::sqlite3PagerJournalname;
pub use crate::src::src::pager::sqlite3PagerLookup;
pub use crate::src::src::pager::sqlite3PagerMaxPageCount;
pub use crate::src::src::pager::sqlite3PagerMovepage;
pub use crate::src::src::pager::sqlite3PagerOpen;
pub use crate::src::src::pager::sqlite3PagerOpenSavepoint;
pub use crate::src::src::pager::sqlite3PagerOpenWal;
pub use crate::src::src::pager::sqlite3PagerPageRefcount;
pub use crate::src::src::pager::sqlite3PagerPagecount;
pub use crate::src::src::pager::sqlite3PagerReadFileheader;
pub use crate::src::src::pager::sqlite3PagerRef;
pub use crate::src::src::pager::sqlite3PagerRekey;
pub use crate::src::src::pager::sqlite3PagerRollback;
pub use crate::src::src::pager::sqlite3PagerSavepoint;
pub use crate::src::src::pager::sqlite3PagerSetBusyHandler;
pub use crate::src::src::pager::sqlite3PagerSetCachesize;
pub use crate::src::src::pager::sqlite3PagerSetFlags;
pub use crate::src::src::pager::sqlite3PagerSetMmapLimit;
pub use crate::src::src::pager::sqlite3PagerSetPagesize;
pub use crate::src::src::pager::sqlite3PagerSetSpillsize;
pub use crate::src::src::pager::sqlite3PagerSharedLock;
pub use crate::src::src::pager::sqlite3PagerTempSpace;
pub use crate::src::src::pager::sqlite3PagerTruncateImage;
pub use crate::src::src::pager::sqlite3PagerUnref;
pub use crate::src::src::pager::sqlite3PagerUnrefNotNull;
pub use crate::src::src::pager::sqlite3PagerUnrefPageOne;
pub use crate::src::src::pager::sqlite3PagerVfs;
pub use crate::src::src::pager::sqlite3PagerWrite;
pub use crate::src::src::pcache::PgHdr;
pub use crate::src::src::pcache1::sqlite3PageFree;
pub use crate::src::src::pcache1::sqlite3PageMalloc;
pub use crate::src::src::printf::sqlite3_str_append;
pub use crate::src::src::printf::sqlite3_str_reset;
pub use crate::src::src::printf::sqlite3_str_vappendf_args;
pub use crate::src::src::printf::sqlite3StrAccumFinish;
pub use crate::src::src::printf::sqlite3StrAccumInit;
pub use crate::src::src::util::sqlite3AbsInt32;
pub use crate::src::src::util::sqlite3FaultSim;
pub use crate::src::src::util::sqlite3Get4byte;
pub use crate::src::src::util::sqlite3GetVarint;
pub use crate::src::src::util::sqlite3Put4byte;
pub use crate::src::src::util::sqlite3PutVarint;
pub use crate::src::src::util::sqlite3Strlen30;
pub use crate::src::src::vdbe::Mem;
pub use crate::src::src::vdbe::RecordCompare;
pub use crate::src::src::vdbe::SubProgram;
pub use crate::src::src::vdbe::SubrtnSig;
pub use crate::src::src::vdbe::VdbeOp;
pub use crate::src::src::vdbe::p4union;
pub use crate::src::src::vdbeaux::sqlite3VdbeAllocUnpackedRecord;
pub use crate::src::src::vdbeaux::sqlite3VdbeFindCompare;
pub use crate::src::src::vdbeaux::sqlite3VdbeRecordCompare;
pub use crate::src::src::vdbeaux::sqlite3VdbeRecordUnpack;
pub use crate::src::src::vdbemem::sqlite3MemSetArrayInt64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellArray {
    pub nCell: ::core::ffi::c_int,
    pub pRef: *mut crate::src::headers::btreeInt_h::MemPage,
    pub apCell: *mut *mut crate::src::ext::rtree::rtree::U8_0,
    pub szCell: *mut crate::src::fts5::U16_0,
    pub apEnd: [*mut crate::src::ext::rtree::rtree::U8_0; 6],
    pub ixNx: [::core::ffi::c_int; 6],
}

static mut zMagicHeader: [::core::ffi::c_char; 16] =
    crate::src::headers::btreeInt_h::SQLITE_FILE_HEADER;

pub const BTALLOC_ANY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const BTALLOC_EXACT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const BTALLOC_LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub static mut sqlite3SharedCacheList: *mut crate::src::headers::btreeInt_h::BtShared =
    ::core::ptr::null::<crate::src::headers::btreeInt_h::BtShared>()
        as *mut crate::src::headers::btreeInt_h::BtShared;
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3_enable_shared_cache(
    enable: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    crate::src::src::global::sqlite3Config.sharedCacheEnabled = enable;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn querySharedCacheTableLock(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    iTab: crate::src::src::pager::Pgno,
    eLock: crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    let mut pIter: *mut crate::src::headers::btreeInt_h::BtLock;
    if (*p).sharable == 0 {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    let __pBt_ref = unsafe { &mut *pBt };
    if __pBt_ref.pWriter != p
        && __pBt_ref.btsFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTS_EXCLUSIVE
            != 0 as ::core::ffi::c_int
    {
        return crate::src::headers::sqlite3_h::SQLITE_LOCKED_SHAREDCACHE;
    }
    pIter = __pBt_ref.pLock;
    while !pIter.is_null() {
        if (*pIter).pBtree != p
            && (*pIter).iTable == iTab
            && (*pIter).eLock as ::core::ffi::c_int != eLock as ::core::ffi::c_int
        {
            if eLock as ::core::ffi::c_int == crate::src::headers::btreeInt_h::WRITE_LOCK {
                __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
                    | crate::src::headers::btreeInt_h::BTS_PENDING)
                    as crate::src::fts5::U16_0;
            }
            return crate::src::headers::sqlite3_h::SQLITE_LOCKED_SHAREDCACHE;
        }
        pIter = (*pIter).pNext;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn setSharedCacheTableLock(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    iTable: crate::src::src::pager::Pgno,
    eLock: crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    let mut pLock: *mut crate::src::headers::btreeInt_h::BtLock =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtLock>();
    let mut pIter: *mut crate::src::headers::btreeInt_h::BtLock;
    pIter = (*pBt).pLock;
    while !pIter.is_null() {
        if (*pIter).iTable == iTable && (*pIter).pBtree == p {
            pLock = pIter;
            break;
        } else {
            pIter = (*pIter).pNext;
        }
    }
    if pLock.is_null() {
        pLock = crate::src::src::malloc::sqlite3MallocZero(::core::mem::size_of::<
            crate::src::headers::btreeInt_h::BtLock,
        >()
            as crate::src::ext::rtree::rtree::U64_0)
            as *mut crate::src::headers::btreeInt_h::BtLock;
        if pLock.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
        (*pLock).iTable = iTable;
        (*pLock).pBtree = p;
        (*pLock).pNext = (*pBt).pLock;
        (*pBt).pLock = pLock;
    }
    if eLock as ::core::ffi::c_int > (*pLock).eLock as ::core::ffi::c_int {
        (*pLock).eLock = eLock;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn clearAllSharedCacheTableLocks(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    let __pBt_ref = unsafe { &mut *pBt };
    let mut ppIter: *mut *mut crate::src::headers::btreeInt_h::BtLock = &raw mut __pBt_ref.pLock;
    while !(*ppIter).is_null() {
        let pLock: *mut crate::src::headers::btreeInt_h::BtLock = *ppIter;
        if (*pLock).pBtree == p {
            *ppIter = (*pLock).pNext;
            if (*pLock).iTable != 1 as crate::src::src::pager::Pgno {
                crate::src::src::malloc::sqlite3_free(pLock as *mut ::core::ffi::c_void);
            }
        } else {
            ppIter = &raw mut (*pLock).pNext;
        }
    }
    if __pBt_ref.pWriter == p {
        __pBt_ref.pWriter = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
        __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
            & !(crate::src::headers::btreeInt_h::BTS_EXCLUSIVE
                | crate::src::headers::btreeInt_h::BTS_PENDING))
            as crate::src::fts5::U16_0;
    } else if __pBt_ref.nTransaction == 2 as ::core::ffi::c_int {
        __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
            & !crate::src::headers::btreeInt_h::BTS_PENDING)
            as crate::src::fts5::U16_0;
    }
}

unsafe extern "C" fn downgradeAllSharedCacheTableLocks(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    if (*pBt).pWriter == p {
        let mut pLock: *mut crate::src::headers::btreeInt_h::BtLock;
        let __pBt_ref = unsafe { &mut *pBt };
        __pBt_ref.pWriter = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
        __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
            & !(crate::src::headers::btreeInt_h::BTS_EXCLUSIVE
                | crate::src::headers::btreeInt_h::BTS_PENDING))
            as crate::src::fts5::U16_0;
        pLock = __pBt_ref.pLock;
        while !pLock.is_null() {
            (*pLock).eLock =
                crate::src::headers::btreeInt_h::READ_LOCK as crate::src::ext::rtree::rtree::U8_0;
            pLock = (*pLock).pNext;
        }
    }
}

unsafe extern "C" fn invalidateAllOverflowCache(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
) {
    let mut p: *mut crate::src::headers::btreeInt_h::BtCursor;
    p = (*pBt).pCursor;
    while !p.is_null() {
        (*p).curFlags = ((*p).curFlags as ::core::ffi::c_int
            & !crate::src::headers::btreeInt_h::BTCF_ValidOvfl)
            as crate::src::ext::rtree::rtree::U8_0;
        p = (*p).pNext;
    }
}

unsafe extern "C" fn invalidateIncrblobCursors(
    pBtree: *mut crate::src::headers::btreeInt_h::Btree,
    pgnoRoot: crate::src::src::pager::Pgno,
    iRow: crate::src::ext::rtree::rtree::I64_0,
    isClearTable: ::core::ffi::c_int,
) {
    let mut p: *mut crate::src::headers::btreeInt_h::BtCursor;
    (*pBtree).hasIncrblobCur = 0 as crate::src::ext::rtree::rtree::U8_0;
    p = (*(*pBtree).pBt).pCursor;
    while !p.is_null() {
        if (*p).curFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTCF_Incrblob
            != 0 as ::core::ffi::c_int
        {
            (*pBtree).hasIncrblobCur = 1 as crate::src::ext::rtree::rtree::U8_0;
            if (*p).pgnoRoot == pgnoRoot && (isClearTable != 0 || (*p).info.nKey == iRow) {
                (*p).eState = crate::src::headers::btreeInt_h::CURSOR_INVALID
                    as crate::src::ext::rtree::rtree::U8_0;
            }
        }
        p = (*p).pNext;
    }
}

unsafe extern "C" fn btreeSetHasContent(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pgno: crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pBt).pHasContent.is_null() {
        let __pBt_ref = unsafe { &mut *pBt };
        __pBt_ref.pHasContent = crate::src::src::bitvec::sqlite3BitvecCreate(__pBt_ref.nPage);
        if __pBt_ref.pHasContent.is_null() {
            rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && pgno <= crate::src::src::bitvec::sqlite3BitvecSize((*pBt).pHasContent)
    {
        rc = crate::src::src::bitvec::sqlite3BitvecSet(
            (*pBt).pHasContent,
            pgno as crate::src::ext::rtree::rtree::U32_0,
        );
    }
    rc
}

unsafe extern "C" fn btreeGetHasContent(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pgno: crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    let p: *mut crate::src::src::bitvec::Bitvec = (*pBt).pHasContent;
    (!p.is_null()
        && (pgno > crate::src::src::bitvec::sqlite3BitvecSize(p)
            || crate::src::src::bitvec::sqlite3BitvecTestNotNull(
                p,
                pgno as crate::src::ext::rtree::rtree::U32_0,
            ) != 0)) as ::core::ffi::c_int
}

unsafe extern "C" fn btreeClearHasContent(pBt: *mut crate::src::headers::btreeInt_h::BtShared) {
    crate::src::src::bitvec::sqlite3BitvecDestroy((*pBt).pHasContent);
    (*pBt).pHasContent = ::core::ptr::null_mut::<crate::src::src::bitvec::Bitvec>();
}

unsafe extern "C" fn btreeReleaseAllCursorPages(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) {
    let mut i: ::core::ffi::c_int;
    if (*pCur).iPage as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        let __pCur_ref = unsafe { &mut *pCur };
        while i < __pCur_ref.iPage as ::core::ffi::c_int {
            releasePageNotNull(__pCur_ref.apPage[i as usize]);
            i += 1;
        }
        releasePageNotNull(__pCur_ref.pPage);
        __pCur_ref.iPage = -(1 as ::core::ffi::c_int) as crate::src::headers::sqliteInt_h::I8_0;
    }
}

unsafe extern "C" fn saveCursorKey(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pCur).curIntKey != 0 {
        (*pCur).nKey = sqlite3BtreeIntegerKey(pCur);
    } else {
        
        (*pCur).nKey = sqlite3BtreePayloadSize(pCur) as crate::src::ext::rtree::rtree::I64_0;
        let pKey: *mut ::core::ffi::c_void = crate::src::src::malloc::sqlite3Malloc(
            ((*pCur).nKey
                + 9 as crate::src::ext::rtree::rtree::I64_0
                + 8 as crate::src::ext::rtree::rtree::I64_0)
                as crate::src::ext::rtree::rtree::U64_0,
        );
        if !pKey.is_null() {
            rc = sqlite3BtreePayload(
                pCur,
                0 as crate::src::ext::rtree::rtree::U32_0,
                (*pCur).nKey as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0,
                pKey,
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                ::libc::memset(
                    (pKey as *mut crate::src::ext::rtree::rtree::U8_0).offset((*pCur).nKey as isize)
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                        as crate::__stddef_size_t_h::SizeT,
                );
                (*pCur).pKey = pKey;
            } else {
                crate::src::src::malloc::sqlite3_free(pKey);
            }
        } else {
            rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
    }
    rc
}

unsafe extern "C" fn saveCursorPosition(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    
    let __pCur_ref = unsafe { &mut *pCur };
    if __pCur_ref.curFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTCF_Pinned != 0
    {
        return crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT_PINNED;
    }
    if __pCur_ref.eState as ::core::ffi::c_int == crate::src::headers::btreeInt_h::CURSOR_SKIPNEXT {
        __pCur_ref.eState =
            crate::src::headers::btreeInt_h::CURSOR_VALID as crate::src::ext::rtree::rtree::U8_0;
    } else {
        __pCur_ref.skipNext = 0 as ::core::ffi::c_int;
    }
    let rc: ::core::ffi::c_int = saveCursorKey(pCur);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        btreeReleaseAllCursorPages(pCur);
        __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK
            as crate::src::ext::rtree::rtree::U8_0;
    }
    __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
        & !(crate::src::headers::btreeInt_h::BTCF_ValidNKey
            | crate::src::headers::btreeInt_h::BTCF_ValidOvfl
            | crate::src::headers::btreeInt_h::BTCF_AtLast))
        as crate::src::ext::rtree::rtree::U8_0;
    rc
}

unsafe extern "C" fn saveAllCursors(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    iRoot: crate::src::src::pager::Pgno,
    pExcept: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let mut p: *mut crate::src::headers::btreeInt_h::BtCursor;
    p = (*pBt).pCursor;
    while !p.is_null() {
        if p != pExcept && (0 as crate::src::src::pager::Pgno == iRoot || (*p).pgnoRoot == iRoot) {
            break;
        }
        p = (*p).pNext;
    }
    if !p.is_null() {
        return saveCursorsOnList(p, iRoot, pExcept);
    }
    if !pExcept.is_null() {
        (*pExcept).curFlags = ((*pExcept).curFlags as ::core::ffi::c_int
            & !crate::src::headers::btreeInt_h::BTCF_Multiple)
            as crate::src::ext::rtree::rtree::U8_0;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[inline(never)]
unsafe extern "C" fn saveCursorsOnList(
    mut p: *mut crate::src::headers::btreeInt_h::BtCursor,
    iRoot: crate::src::src::pager::Pgno,
    pExcept: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    loop {
        if p != pExcept && (0 as crate::src::src::pager::Pgno == iRoot || (*p).pgnoRoot == iRoot) {
            if (*p).eState as ::core::ffi::c_int == crate::src::headers::btreeInt_h::CURSOR_VALID
                || (*p).eState as ::core::ffi::c_int
                    == crate::src::headers::btreeInt_h::CURSOR_SKIPNEXT
            {
                let rc: ::core::ffi::c_int = saveCursorPosition(p);
                if crate::src::headers::sqlite3_h::SQLITE_OK != rc {
                    return rc;
                }
            } else {
                btreeReleaseAllCursorPages(p);
            }
        }
        p = (*p).pNext;
        if p.is_null() {
            break;
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeClearCursor(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) {
    let __pCur_ref = unsafe { &mut *pCur };
    crate::src::src::malloc::sqlite3_free(__pCur_ref.pKey);
    __pCur_ref.pKey = ::core::ptr::null_mut::<::core::ffi::c_void>();
    __pCur_ref.eState =
        crate::src::headers::btreeInt_h::CURSOR_INVALID as crate::src::ext::rtree::rtree::U8_0;
}

unsafe extern "C" fn btreeMoveto(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pKey: *const ::core::ffi::c_void,
    nKey: crate::src::ext::rtree::rtree::I64_0,
    bias: ::core::ffi::c_int,
    pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    let pIdxKey: *mut crate::src::headers::sqliteInt_h::UnpackedRecord;
    if !pKey.is_null() {
        let pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo =
            (*pCur).pKeyInfo as *mut crate::src::headers::sqliteInt_h::KeyInfo;
        pIdxKey = crate::src::src::vdbeaux::sqlite3VdbeAllocUnpackedRecord(
            pKeyInfo as *mut crate::src::headers::sqliteInt_h::KeyInfo,
        ) as *mut crate::src::headers::sqliteInt_h::UnpackedRecord;
        if pIdxKey.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
        crate::src::src::vdbeaux::sqlite3VdbeRecordUnpack(
            nKey as ::core::ffi::c_int,
            pKey,
            pIdxKey as *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
        );
        if (*pIdxKey).nField as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*pIdxKey).nField as ::core::ffi::c_int > (*pKeyInfo).nAllField as ::core::ffi::c_int
        {
            rc = crate::src::src::main::sqlite3CorruptError(877 as ::core::ffi::c_int);
        } else {
            rc = sqlite3BtreeIndexMoveto(pCur, pIdxKey, pRes);
        }
        crate::src::src::malloc::sqlite3DbFree(
            (*(*pCur).pKeyInfo).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pIdxKey as *mut ::core::ffi::c_void,
        );
    } else {
        rc = sqlite3BtreeTableMoveto(pCur, nKey, bias, pRes);
    }
    rc
}

unsafe extern "C" fn btreeRestoreCursorPosition(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    let mut skipNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*pCur).eState as ::core::ffi::c_int == crate::src::headers::btreeInt_h::CURSOR_FAULT {
        return (*pCur).skipNext;
    }
    (*pCur).eState =
        crate::src::headers::btreeInt_h::CURSOR_INVALID as crate::src::ext::rtree::rtree::U8_0;
    if crate::src::src::util::sqlite3FaultSim(410 as ::core::ffi::c_int) != 0 {
        rc = crate::src::headers::sqlite3_h::SQLITE_IOERR;
    } else {
        rc = btreeMoveto(
            pCur,
            (*pCur).pKey,
            (*pCur).nKey,
            0 as ::core::ffi::c_int,
            &raw mut skipNext,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pCur_ref = unsafe { &mut *pCur };
        crate::src::src::malloc::sqlite3_free(__pCur_ref.pKey);
        __pCur_ref.pKey = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if skipNext != 0 {
            __pCur_ref.skipNext = skipNext;
        }
        if __pCur_ref.skipNext != 0
            && __pCur_ref.eState as ::core::ffi::c_int
                == crate::src::headers::btreeInt_h::CURSOR_VALID
        {
            __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_SKIPNEXT
                as crate::src::ext::rtree::rtree::U8_0;
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCursorHasMoved(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    (crate::src::headers::btreeInt_h::CURSOR_VALID
        != *(pCur as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int)
        as ::core::ffi::c_int
}
pub unsafe extern "C" fn sqlite3BtreeFakeValidCursor()
-> *mut crate::src::headers::btreeInt_h::BtCursor {
    static mut fakeCursor: crate::src::ext::rtree::rtree::U8_0 =
        crate::src::headers::btreeInt_h::CURSOR_VALID as crate::src::ext::rtree::rtree::U8_0;
    &raw mut fakeCursor as *mut crate::src::headers::btreeInt_h::BtCursor
}
pub unsafe extern "C" fn sqlite3BtreeCursorRestore(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pDifferentRow: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let rc: ::core::ffi::c_int = if (*pCur).eState as ::core::ffi::c_int
        >= crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK
    {
        btreeRestoreCursorPosition(pCur)
    } else {
        crate::src::headers::sqlite3_h::SQLITE_OK
    };
    if rc != 0 {
        *pDifferentRow = 1 as ::core::ffi::c_int;
        return rc;
    }
    if (*pCur).eState as ::core::ffi::c_int != crate::src::headers::btreeInt_h::CURSOR_VALID {
        *pDifferentRow = 1 as ::core::ffi::c_int;
    } else {
        *pDifferentRow = 0 as ::core::ffi::c_int;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCursorHintFlags(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    x: ::core::ffi::c_uint,
) {
    (*pCur).hints = x as crate::src::ext::rtree::rtree::U8_0;
}

unsafe extern "C" fn ptrmapPageno(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pgno: crate::src::src::pager::Pgno,
) -> crate::src::src::pager::Pgno {
    
    
    let mut ret: crate::src::src::pager::Pgno;
    if pgno < 2 as crate::src::src::pager::Pgno {
        return 0 as crate::src::src::pager::Pgno;
    }
    let nPagesPerMapPage: ::core::ffi::c_int = (*pBt)
        .usableSize
        .wrapping_div(5 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
        as ::core::ffi::c_int;
    let iPtrMap: crate::src::src::pager::Pgno = pgno
        .wrapping_sub(2 as crate::src::src::pager::Pgno)
        .wrapping_div(nPagesPerMapPage as crate::src::src::pager::Pgno);
    ret = iPtrMap
        .wrapping_mul(nPagesPerMapPage as crate::src::src::pager::Pgno)
        .wrapping_add(2 as crate::src::src::pager::Pgno);
    if ret
        == (crate::src::src::global::sqlite3PendingByte as crate::src::ext::rtree::rtree::U32_0)
            .wrapping_div((*pBt).pageSize)
            .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
    {
        ret = ret.wrapping_add(1);
    }
    ret
}

unsafe extern "C" fn ptrmapPut(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    key: crate::src::src::pager::Pgno,
    eType: crate::src::ext::rtree::rtree::U8_0,
    parent: crate::src::src::pager::Pgno,
    pRC: *mut ::core::ffi::c_int,
) {
    let mut pDbPage: *mut crate::src::src::pager::DbPage =
        ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
    let pPtrmap: *mut crate::src::ext::rtree::rtree::U8_0;
    
    let offset: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    if *pRC != 0 {
        return;
    }
    if key == 0 as crate::src::src::pager::Pgno {
        *pRC = crate::src::src::main::sqlite3CorruptError(1075 as ::core::ffi::c_int);
        return;
    }
    let iPtrmap: crate::src::src::pager::Pgno = ptrmapPageno(pBt, key);
    rc = crate::src::src::pager::sqlite3PagerGet(
        (*pBt).pPager,
        iPtrmap,
        &raw mut pDbPage as *mut _ as *mut *mut crate::src::src::pcache::PgHdr,
        0 as ::core::ffi::c_int,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        *pRC = rc;
        return;
    }
    if *(crate::src::src::pager::sqlite3PagerGetExtra(
        pDbPage as *mut crate::src::src::pcache::PgHdr,
    ) as *mut ::core::ffi::c_char)
        .offset(0_isize) as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        *pRC = crate::src::src::main::sqlite3CorruptError(1088 as ::core::ffi::c_int);
    } else {
        offset = (5 as crate::src::src::pager::Pgno).wrapping_mul(
            key.wrapping_sub(iPtrmap)
                .wrapping_sub(1 as crate::src::src::pager::Pgno),
        ) as ::core::ffi::c_int;
        if offset < 0 as ::core::ffi::c_int {
            *pRC = crate::src::src::main::sqlite3CorruptError(1093 as ::core::ffi::c_int);
        } else {
            pPtrmap = crate::src::src::pager::sqlite3PagerGetData(
                pDbPage as *mut crate::src::src::pcache::PgHdr,
            ) as *mut crate::src::ext::rtree::rtree::U8_0;
            if eType as ::core::ffi::c_int != *pPtrmap.offset(offset as isize) as ::core::ffi::c_int
                || crate::src::src::util::sqlite3Get4byte(
                    pPtrmap.offset((offset + 1 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0,
                ) != parent
            {
                rc = crate::src::src::pager::sqlite3PagerWrite(
                    pDbPage as *mut crate::src::src::pcache::PgHdr,
                );
                *pRC = rc;
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    *pPtrmap.offset(offset as isize) = eType;
                    crate::src::src::util::sqlite3Put4byte(
                        pPtrmap.offset((offset + 1 as ::core::ffi::c_int) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0,
                        parent as crate::src::ext::rtree::rtree::U32_0,
                    );
                }
            }
        }
    }
    crate::src::src::pager::sqlite3PagerUnref(pDbPage as *mut crate::src::src::pcache::PgHdr);
}

unsafe extern "C" fn ptrmapGet(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    key: crate::src::src::pager::Pgno,
    pEType: *mut crate::src::ext::rtree::rtree::U8_0,
    pPgno: *mut crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    let mut pDbPage: *mut crate::src::src::pager::DbPage =
        ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
    
    
    
    
    let iPtrmap: ::core::ffi::c_int = ptrmapPageno(pBt, key) as ::core::ffi::c_int;
    let rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerGet(
        (*pBt).pPager,
        iPtrmap as crate::src::src::pager::Pgno,
        &raw mut pDbPage as *mut _ as *mut *mut crate::src::src::pcache::PgHdr,
        0 as ::core::ffi::c_int,
    );
    if rc != 0 as ::core::ffi::c_int {
        return rc;
    }
    let pPtrmap: *mut crate::src::ext::rtree::rtree::U8_0 = crate::src::src::pager::sqlite3PagerGetData(pDbPage as *mut crate::src::src::pcache::PgHdr)
            as *mut crate::src::ext::rtree::rtree::U8_0;
    let offset: ::core::ffi::c_int = (5 as crate::src::src::pager::Pgno).wrapping_mul(
        key.wrapping_sub(iPtrmap as crate::src::src::pager::Pgno)
            .wrapping_sub(1 as crate::src::src::pager::Pgno),
    ) as ::core::ffi::c_int;
    if offset < 0 as ::core::ffi::c_int {
        crate::src::src::pager::sqlite3PagerUnref(pDbPage as *mut crate::src::src::pcache::PgHdr);
        return crate::src::src::main::sqlite3CorruptError(1138 as ::core::ffi::c_int);
    }
    *pEType = *pPtrmap.offset(offset as isize);
    if !pPgno.is_null() {
        *pPgno = crate::src::src::util::sqlite3Get4byte(
            pPtrmap.offset((offset + 1 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0,
        ) as crate::src::src::pager::Pgno;
    }
    crate::src::src::pager::sqlite3PagerUnref(pDbPage as *mut crate::src::src::pcache::PgHdr);
    if (*pEType as ::core::ffi::c_int) < 1 as ::core::ffi::c_int
        || *pEType as ::core::ffi::c_int > 5 as ::core::ffi::c_int
    {
        return crate::src::src::main::sqlite3CorruptError(1146 as ::core::ffi::c_int);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[inline(never)]
unsafe extern "C" fn btreeParseCellAdjustSizeForOverflow(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut crate::src::ext::rtree::rtree::U8_0,
    pInfo: *mut crate::src::headers::btreeInt_h::CellInfo,
) {
    
    
    
    let __pPage_ref = unsafe { &mut *pPage };
    let minLocal: ::core::ffi::c_int = __pPage_ref.minLocal as ::core::ffi::c_int;
    let maxLocal: ::core::ffi::c_int = __pPage_ref.maxLocal as ::core::ffi::c_int;
    let __pInfo_ref = unsafe { &mut *pInfo };
    let surplus: ::core::ffi::c_int = (minLocal as crate::src::ext::rtree::rtree::U32_0).wrapping_add(
        (*pInfo)
            .nPayload
            .wrapping_sub(minLocal as crate::src::ext::rtree::rtree::U32_0)
            .wrapping_rem(
                (*__pPage_ref.pBt)
                    .usableSize
                    .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0),
            ),
    ) as ::core::ffi::c_int;
    if surplus <= maxLocal {
        __pInfo_ref.nLocal = surplus as crate::src::fts5::U16_0;
    } else {
        __pInfo_ref.nLocal = minLocal as crate::src::fts5::U16_0;
    }
    __pInfo_ref.nSize = ((__pInfo_ref.pPayload.offset(__pInfo_ref.nLocal as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset_from(pCell) as ::core::ffi::c_long
        as crate::src::fts5::U16_0 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int) as crate::src::fts5::U16_0;
}

unsafe extern "C" fn btreePayloadToLocal(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    nPayload: crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    
    let maxLocal: ::core::ffi::c_int = (*pPage).maxLocal as ::core::ffi::c_int;
    if nPayload <= maxLocal as crate::src::ext::rtree::rtree::I64_0 {
        nPayload as ::core::ffi::c_int
    } else {
        
        
        let minLocal: ::core::ffi::c_int = (*pPage).minLocal as ::core::ffi::c_int;
        let surplus: ::core::ffi::c_int = (minLocal as crate::src::ext::rtree::rtree::I64_0
            + (nPayload - minLocal as crate::src::ext::rtree::rtree::I64_0)
                % (*(*pPage).pBt)
                    .usableSize
                    .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0)
                    as crate::src::ext::rtree::rtree::I64_0)
            as ::core::ffi::c_int;
        if surplus <= maxLocal {
            surplus
        } else {
            minLocal
        }
    }
}

unsafe extern "C" fn btreeParseCellPtrNoPayload(
    mut _pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut crate::src::ext::rtree::rtree::U8_0,
    pInfo: *mut crate::src::headers::btreeInt_h::CellInfo,
) {
    let __pInfo_ref = unsafe { &mut *pInfo };
    __pInfo_ref.nSize = (4 as ::core::ffi::c_int
        + crate::src::src::util::sqlite3GetVarint(
            pCell.offset(4_isize) as *mut crate::src::ext::rtree::rtree::U8_0,
            &raw mut __pInfo_ref.nKey as *mut crate::src::ext::rtree::rtree::U64_0,
        ) as ::core::ffi::c_int) as crate::src::fts5::U16_0;
    __pInfo_ref.nPayload = 0 as crate::src::ext::rtree::rtree::U32_0;
    __pInfo_ref.nLocal = 0 as crate::src::fts5::U16_0;
    __pInfo_ref.pPayload = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
}

unsafe extern "C" fn btreeParseCellPtr(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut crate::src::ext::rtree::rtree::U8_0,
    pInfo: *mut crate::src::headers::btreeInt_h::CellInfo,
) {
    let mut pIter: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut nPayload: crate::src::ext::rtree::rtree::U32_0;
    let mut iKey: crate::src::ext::rtree::rtree::U64_0;
    pIter = pCell;
    nPayload = *pIter as crate::src::ext::rtree::rtree::U32_0;
    if nPayload >= 0x80 as crate::src::ext::rtree::rtree::U32_0 {
        let pEnd: *mut crate::src::ext::rtree::rtree::U8_0 =
            pIter.offset(8_isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        nPayload &= 0x7f as crate::src::ext::rtree::rtree::U32_0;
        loop {
            pIter = pIter.offset(1);
            nPayload = nPayload << 7 as ::core::ffi::c_int
                | (*pIter as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::U32_0;
            if !(*pIter as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int && pIter < pEnd) {
                break;
            }
        }
    }
    pIter = pIter.offset(1);
    iKey = *pIter as crate::src::ext::rtree::rtree::U64_0;
    if iKey >= 0x80 as crate::src::ext::rtree::rtree::U64_0 {
        let mut x: crate::src::ext::rtree::rtree::U8_0;
        pIter = pIter.offset(1);
        x = *pIter;
        iKey = iKey << 7 as ::core::ffi::c_int ^ x as crate::src::ext::rtree::rtree::U64_0;
        if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
            pIter = pIter.offset(1);
            x = *pIter;
            iKey = iKey << 7 as ::core::ffi::c_int ^ x as crate::src::ext::rtree::rtree::U64_0;
            if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                pIter = pIter.offset(1);
                x = *pIter;
                iKey = iKey << 7 as ::core::ffi::c_int
                    ^ 0x10204000 as crate::src::ext::rtree::rtree::U64_0
                    ^ x as crate::src::ext::rtree::rtree::U64_0;
                if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                    pIter = pIter.offset(1);
                    x = *pIter;
                    iKey = iKey << 7 as ::core::ffi::c_int
                        ^ 0x4000 as crate::src::ext::rtree::rtree::U64_0
                        ^ x as crate::src::ext::rtree::rtree::U64_0;
                    if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                        pIter = pIter.offset(1);
                        x = *pIter;
                        iKey = iKey << 7 as ::core::ffi::c_int
                            ^ 0x4000 as crate::src::ext::rtree::rtree::U64_0
                            ^ x as crate::src::ext::rtree::rtree::U64_0;
                        if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                            pIter = pIter.offset(1);
                            x = *pIter;
                            iKey = iKey << 7 as ::core::ffi::c_int
                                ^ 0x4000 as crate::src::ext::rtree::rtree::U64_0
                                ^ x as crate::src::ext::rtree::rtree::U64_0;
                            if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                                pIter = pIter.offset(1);
                                x = *pIter;
                                iKey = iKey << 7 as ::core::ffi::c_int
                                    ^ 0x4000 as crate::src::ext::rtree::rtree::U64_0
                                    ^ x as crate::src::ext::rtree::rtree::U64_0;
                                if x as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                                    pIter = pIter.offset(1);
                                    iKey = iKey << 8 as ::core::ffi::c_int
                                        ^ 0x8000 as crate::src::ext::rtree::rtree::U64_0
                                        ^ *pIter as crate::src::ext::rtree::rtree::U64_0;
                                }
                            }
                        }
                    }
                }
            } else {
                iKey ^= 0x204000 as crate::src::ext::rtree::rtree::U64_0;
            }
        } else {
            iKey ^= 0x4000 as crate::src::ext::rtree::rtree::U64_0;
        }
    }
    pIter = pIter.offset(1);
    let __pInfo_ref = unsafe { &mut *pInfo };
    __pInfo_ref.nKey = *(&raw mut iKey as *mut crate::src::ext::rtree::rtree::I64_0);
    __pInfo_ref.nPayload = nPayload;
    __pInfo_ref.pPayload = pIter;
    if nPayload <= (*pPage).maxLocal as crate::src::ext::rtree::rtree::U32_0 {
        __pInfo_ref.nSize = (nPayload as crate::src::fts5::U16_0 as ::core::ffi::c_int
            + pIter.offset_from(pCell) as ::core::ffi::c_long as crate::src::fts5::U16_0
                as ::core::ffi::c_int) as crate::src::fts5::U16_0;
        if (__pInfo_ref.nSize as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
            __pInfo_ref.nSize = 4 as crate::src::fts5::U16_0;
        }
        __pInfo_ref.nLocal = nPayload as crate::src::fts5::U16_0;
    } else {
        btreeParseCellAdjustSizeForOverflow(pPage, pCell, pInfo);
    };
}

unsafe extern "C" fn btreeParseCellPtrIndex(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut crate::src::ext::rtree::rtree::U8_0,
    pInfo: *mut crate::src::headers::btreeInt_h::CellInfo,
) {
    let mut pIter: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut nPayload: crate::src::ext::rtree::rtree::U32_0;
    pIter = pCell.offset((*pPage).childPtrSize as ::core::ffi::c_int as isize);
    nPayload = *pIter as crate::src::ext::rtree::rtree::U32_0;
    if nPayload >= 0x80 as crate::src::ext::rtree::rtree::U32_0 {
        let pEnd: *mut crate::src::ext::rtree::rtree::U8_0 =
            pIter.offset(8_isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        nPayload &= 0x7f as crate::src::ext::rtree::rtree::U32_0;
        loop {
            pIter = pIter.offset(1);
            nPayload = nPayload << 7 as ::core::ffi::c_int
                | (*pIter as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::U32_0;
            if !(*pIter as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int && pIter < pEnd) {
                break;
            }
        }
    }
    pIter = pIter.offset(1);
    let __pInfo_ref = unsafe { &mut *pInfo };
    __pInfo_ref.nKey = nPayload as crate::src::ext::rtree::rtree::I64_0;
    __pInfo_ref.nPayload = nPayload;
    __pInfo_ref.pPayload = pIter;
    if nPayload <= (*pPage).maxLocal as crate::src::ext::rtree::rtree::U32_0 {
        __pInfo_ref.nSize = (nPayload as crate::src::fts5::U16_0 as ::core::ffi::c_int
            + pIter.offset_from(pCell) as ::core::ffi::c_long as crate::src::fts5::U16_0
                as ::core::ffi::c_int) as crate::src::fts5::U16_0;
        if (__pInfo_ref.nSize as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
            __pInfo_ref.nSize = 4 as crate::src::fts5::U16_0;
        }
        __pInfo_ref.nLocal = nPayload as crate::src::fts5::U16_0;
    } else {
        btreeParseCellAdjustSizeForOverflow(pPage, pCell, pInfo);
    };
}

unsafe extern "C" fn btreeParseCell(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    iCell: ::core::ffi::c_int,
    pInfo: *mut crate::src::headers::btreeInt_h::CellInfo,
) {
    let __pPage_ref = unsafe { &mut *pPage };
    __pPage_ref.xParseCell.expect("non-null function pointer")(
        pPage,
        __pPage_ref.aData.offset(
            (__pPage_ref.maskPage as ::core::ffi::c_int
                & ((*((*pPage)
                    .aCellIdx
                    .offset((2 as ::core::ffi::c_int * iCell) as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(0_isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *((*pPage)
                        .aCellIdx
                        .offset((2 as ::core::ffi::c_int * iCell) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(1_isize) as ::core::ffi::c_int)) as isize,
        ),
        pInfo,
    );
}

unsafe extern "C" fn cellSizePtr(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut crate::src::ext::rtree::rtree::U8_0,
) -> crate::src::fts5::U16_0 {
    let mut pIter: *mut crate::src::ext::rtree::rtree::U8_0 = pCell.offset(4_isize);
    let pEnd: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut nSize: crate::src::ext::rtree::rtree::U32_0;
    nSize = *pIter as crate::src::ext::rtree::rtree::U32_0;
    if nSize >= 0x80 as crate::src::ext::rtree::rtree::U32_0 {
        pEnd = pIter.offset(8_isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        nSize &= 0x7f as crate::src::ext::rtree::rtree::U32_0;
        loop {
            pIter = pIter.offset(1);
            nSize = nSize << 7 as ::core::ffi::c_int
                | (*pIter as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::U32_0;
            if !(*pIter as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int && pIter < pEnd) {
                break;
            }
        }
    }
    pIter = pIter.offset(1);
    if nSize <= (*pPage).maxLocal as crate::src::ext::rtree::rtree::U32_0 {
        nSize = nSize
            .wrapping_add(pIter.offset_from(pCell) as ::core::ffi::c_long
                as crate::src::ext::rtree::rtree::U32_0);
    } else {
        let __pPage_ref = unsafe { &mut *pPage };
        let minLocal: ::core::ffi::c_int = __pPage_ref.minLocal as ::core::ffi::c_int;
        nSize = (minLocal as crate::src::ext::rtree::rtree::U32_0).wrapping_add(
            nSize
                .wrapping_sub(minLocal as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_rem(
                    (*__pPage_ref.pBt)
                        .usableSize
                        .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0),
                ),
        );
        if nSize > __pPage_ref.maxLocal as crate::src::ext::rtree::rtree::U32_0 {
            nSize = minLocal as crate::src::ext::rtree::rtree::U32_0;
        }
        nSize = nSize.wrapping_add(
            (4 as ::core::ffi::c_int
                + pIter.offset_from(pCell) as ::core::ffi::c_long as crate::src::fts5::U16_0
                    as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U32_0,
        );
    }
    nSize as crate::src::fts5::U16_0
}

unsafe extern "C" fn cellSizePtrIdxLeaf(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut crate::src::ext::rtree::rtree::U8_0,
) -> crate::src::fts5::U16_0 {
    let mut pIter: *mut crate::src::ext::rtree::rtree::U8_0 = pCell;
    let pEnd: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut nSize: crate::src::ext::rtree::rtree::U32_0;
    nSize = *pIter as crate::src::ext::rtree::rtree::U32_0;
    if nSize >= 0x80 as crate::src::ext::rtree::rtree::U32_0 {
        pEnd = pIter.offset(8_isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        nSize &= 0x7f as crate::src::ext::rtree::rtree::U32_0;
        loop {
            pIter = pIter.offset(1);
            nSize = nSize << 7 as ::core::ffi::c_int
                | (*pIter as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::U32_0;
            if !(*pIter as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int && pIter < pEnd) {
                break;
            }
        }
    }
    pIter = pIter.offset(1);
    if nSize <= (*pPage).maxLocal as crate::src::ext::rtree::rtree::U32_0 {
        nSize = nSize
            .wrapping_add(pIter.offset_from(pCell) as ::core::ffi::c_long
                as crate::src::ext::rtree::rtree::U32_0);
        if nSize < 4 as crate::src::ext::rtree::rtree::U32_0 {
            nSize = 4 as crate::src::ext::rtree::rtree::U32_0;
        }
    } else {
        let __pPage_ref = unsafe { &mut *pPage };
        let minLocal: ::core::ffi::c_int = __pPage_ref.minLocal as ::core::ffi::c_int;
        nSize = (minLocal as crate::src::ext::rtree::rtree::U32_0).wrapping_add(
            nSize
                .wrapping_sub(minLocal as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_rem(
                    (*__pPage_ref.pBt)
                        .usableSize
                        .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0),
                ),
        );
        if nSize > __pPage_ref.maxLocal as crate::src::ext::rtree::rtree::U32_0 {
            nSize = minLocal as crate::src::ext::rtree::rtree::U32_0;
        }
        nSize = nSize.wrapping_add(
            (4 as ::core::ffi::c_int
                + pIter.offset_from(pCell) as ::core::ffi::c_long as crate::src::fts5::U16_0
                    as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U32_0,
        );
    }
    nSize as crate::src::fts5::U16_0
}

unsafe extern "C" fn cellSizePtrNoPayload(
    mut _pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut crate::src::ext::rtree::rtree::U8_0,
) -> crate::src::fts5::U16_0 {
    let mut pIter: *mut crate::src::ext::rtree::rtree::U8_0 = pCell.offset(4_isize);
    
    let pEnd: *mut crate::src::ext::rtree::rtree::U8_0 = pIter.offset(9_isize);
    loop {
        let fresh0 = pIter;
        pIter = pIter.offset(1);
        if !(*fresh0 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 && pIter < pEnd) {
            break;
        }
    }
    pIter.offset_from(pCell) as ::core::ffi::c_long as crate::src::fts5::U16_0
}

unsafe extern "C" fn cellSizePtrTableLeaf(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut crate::src::ext::rtree::rtree::U8_0,
) -> crate::src::fts5::U16_0 {
    let mut pIter: *mut crate::src::ext::rtree::rtree::U8_0 = pCell;
    let pEnd: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut nSize: crate::src::ext::rtree::rtree::U32_0;
    nSize = *pIter as crate::src::ext::rtree::rtree::U32_0;
    if nSize >= 0x80 as crate::src::ext::rtree::rtree::U32_0 {
        pEnd = pIter.offset(8_isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        nSize &= 0x7f as crate::src::ext::rtree::rtree::U32_0;
        loop {
            pIter = pIter.offset(1);
            nSize = nSize << 7 as ::core::ffi::c_int
                | (*pIter as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::U32_0;
            if !(*pIter as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int && pIter < pEnd) {
                break;
            }
        }
    }
    pIter = pIter.offset(1);
    let fresh1 = pIter;
    pIter = pIter.offset(1);
    if *fresh1 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
        && {
            let fresh2 = pIter;
            pIter = pIter.offset(1);
            *fresh2 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
        }
        && {
            let fresh3 = pIter;
            pIter = pIter.offset(1);
            *fresh3 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
        }
        && {
            let fresh4 = pIter;
            pIter = pIter.offset(1);
            *fresh4 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
        }
        && {
            let fresh5 = pIter;
            pIter = pIter.offset(1);
            *fresh5 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
        }
        && {
            let fresh6 = pIter;
            pIter = pIter.offset(1);
            *fresh6 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
        }
        && {
            let fresh7 = pIter;
            pIter = pIter.offset(1);
            *fresh7 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
        }
        && {
            let fresh8 = pIter;
            pIter = pIter.offset(1);
            *fresh8 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
        }
    {
        pIter = pIter.offset(1);
    }
    if nSize <= (*pPage).maxLocal as crate::src::ext::rtree::rtree::U32_0 {
        nSize = nSize
            .wrapping_add(pIter.offset_from(pCell) as ::core::ffi::c_long
                as crate::src::ext::rtree::rtree::U32_0);
        if nSize < 4 as crate::src::ext::rtree::rtree::U32_0 {
            nSize = 4 as crate::src::ext::rtree::rtree::U32_0;
        }
    } else {
        let __pPage_ref = unsafe { &mut *pPage };
        let minLocal: ::core::ffi::c_int = __pPage_ref.minLocal as ::core::ffi::c_int;
        nSize = (minLocal as crate::src::ext::rtree::rtree::U32_0).wrapping_add(
            nSize
                .wrapping_sub(minLocal as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_rem(
                    (*__pPage_ref.pBt)
                        .usableSize
                        .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0),
                ),
        );
        if nSize > __pPage_ref.maxLocal as crate::src::ext::rtree::rtree::U32_0 {
            nSize = minLocal as crate::src::ext::rtree::rtree::U32_0;
        }
        nSize = nSize.wrapping_add(
            (4 as ::core::ffi::c_int
                + pIter.offset_from(pCell) as ::core::ffi::c_long as crate::src::fts5::U16_0
                    as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U32_0,
        );
    }
    nSize as crate::src::fts5::U16_0
}

unsafe extern "C" fn ptrmapPutOvflPtr(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pSrc: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut crate::src::ext::rtree::rtree::U8_0,
    pRC: *mut ::core::ffi::c_int,
) {
    let mut info: crate::src::headers::btreeInt_h::CellInfo = unsafe { ::core::mem::zeroed() };
    if *pRC != 0 {
        return;
    }
    (*pPage).xParseCell.expect("non-null function pointer")(pPage, pCell, &raw mut info);
    if (info.nLocal as crate::src::ext::rtree::rtree::U32_0) < info.nPayload {
        
        if (pCell as crate::src::headers::sqliteInt_h::Uptr)
            < (*pSrc).aDataEnd as crate::src::headers::sqliteInt_h::Uptr
            && pCell.offset(info.nLocal as ::core::ffi::c_int as isize)
                as crate::src::headers::sqliteInt_h::Uptr
                > (*pSrc).aDataEnd as crate::src::headers::sqliteInt_h::Uptr
        {
            *pRC = crate::src::src::main::sqlite3CorruptError(1591 as ::core::ffi::c_int);
            return;
        }
        let ovfl: crate::src::src::pager::Pgno = crate::src::src::util::sqlite3Get4byte(
            pCell.offset((info.nSize as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0,
        ) as crate::src::src::pager::Pgno;
        ptrmapPut(
            (*pPage).pBt,
            ovfl,
            crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW1
                as crate::src::ext::rtree::rtree::U8_0,
            (*pPage).pgno,
            pRC,
        );
    }
}

unsafe extern "C" fn defragmentPage(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    nMaxFrag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let current_block: u64;
    let mut i: ::core::ffi::c_int;
    let mut pc: ::core::ffi::c_int;
    
    let mut size: ::core::ffi::c_int;
    
    
    let mut cbrk: ::core::ffi::c_int = 0;
    
    
    let temp: *mut ::core::ffi::c_uchar;
    let src: *mut ::core::ffi::c_uchar;
    
    let iCellLast: ::core::ffi::c_int;
    let iCellStart: ::core::ffi::c_int;
    let __pPage_ref = unsafe { &mut *pPage };
    let data: *mut ::core::ffi::c_uchar = __pPage_ref.aData as *mut ::core::ffi::c_uchar;
    let hdr: ::core::ffi::c_int = __pPage_ref.hdrOffset as ::core::ffi::c_int;
    let cellOffset: ::core::ffi::c_int = __pPage_ref.cellOffset as ::core::ffi::c_int;
    let nCell: ::core::ffi::c_int = __pPage_ref.nCell as ::core::ffi::c_int;
    let iCellFirst: ::core::ffi::c_int = cellOffset + 2 as ::core::ffi::c_int * nCell;
    let usableSize: ::core::ffi::c_int = (*__pPage_ref.pBt).usableSize as ::core::ffi::c_int;
    if *data.offset((hdr + 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int <= nMaxFrag {
        let iFree: ::core::ffi::c_int = (*(data.offset((hdr + 1 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar)
            .offset(0_isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(data.offset((hdr + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_uchar)
                .offset(1_isize) as ::core::ffi::c_int;
        if iFree > usableSize - 4 as ::core::ffi::c_int {
            return crate::src::src::main::sqlite3CorruptError(1649 as ::core::ffi::c_int);
        }
        if iFree != 0 {
            let iFree2: ::core::ffi::c_int =
                (*(data.offset(iFree as isize) as *mut ::core::ffi::c_uchar).offset(0_isize)
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(data.offset(iFree as isize) as *mut ::core::ffi::c_uchar).offset(1_isize)
                        as ::core::ffi::c_int;
            if iFree2 > usableSize - 4 as ::core::ffi::c_int {
                return crate::src::src::main::sqlite3CorruptError(1652 as ::core::ffi::c_int);
            }
            if 0 as ::core::ffi::c_int == iFree2
                || *data.offset(iFree2 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && *data.offset((iFree2 + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
            {
                let pEnd: *mut crate::src::ext::rtree::rtree::U8_0 = data
                    .offset((cellOffset + nCell * 2 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0;
                let mut pAddr: *mut crate::src::ext::rtree::rtree::U8_0;
                let mut sz2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut sz: ::core::ffi::c_int =
                    (*(data.offset((iFree + 2 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar)
                        .offset(0_isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(data.offset((iFree + 2 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar)
                            .offset(1_isize) as ::core::ffi::c_int;
                let top: ::core::ffi::c_int =
                    (*(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar)
                        .offset(0_isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar)
                            .offset(1_isize) as ::core::ffi::c_int;
                if top >= iFree {
                    return crate::src::src::main::sqlite3CorruptError(1660 as ::core::ffi::c_int);
                }
                if iFree2 != 0 {
                    if iFree + sz > iFree2 {
                        return crate::src::src::main::sqlite3CorruptError(
                            1663 as ::core::ffi::c_int,
                        );
                    }
                    sz2 = (*(data.offset((iFree2 + 2 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar)
                        .offset(0_isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(data.offset((iFree2 + 2 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar)
                            .offset(1_isize) as ::core::ffi::c_int;
                    if iFree2 + sz2 > usableSize {
                        return crate::src::src::main::sqlite3CorruptError(
                            1665 as ::core::ffi::c_int,
                        );
                    }
                    ::core::ptr::copy(
                        data.offset((iFree + sz) as isize) as *mut ::core::ffi::c_uchar
                            as *const u8,
                        data.offset((iFree + sz + sz2) as isize) as *mut ::core::ffi::c_uchar
                            as *mut u8,
                        (iFree2 - (iFree + sz)) as usize,
                    );
                    sz += sz2;
                } else if iFree + sz > usableSize {
                    return crate::src::src::main::sqlite3CorruptError(1669 as ::core::ffi::c_int);
                }
                cbrk = top + sz;
                ::core::ptr::copy(
                    data.offset(top as isize) as *mut ::core::ffi::c_uchar as *const u8,
                    data.offset(cbrk as isize) as *mut ::core::ffi::c_uchar as *mut u8,
                    (iFree - top) as usize,
                );
                pAddr = data.offset(cellOffset as isize) as *mut ::core::ffi::c_uchar
                    as *mut crate::src::ext::rtree::rtree::U8_0;
                while pAddr < pEnd {
                    pc = (*pAddr.offset(0_isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *pAddr.offset(1_isize) as ::core::ffi::c_int;
                    if pc < iFree {
                        *pAddr.offset(0_isize) = ((pc + sz) >> 8 as ::core::ffi::c_int)
                            as crate::src::ext::rtree::rtree::U8_0;
                        *pAddr.offset(1_isize) =
                            (pc + sz) as crate::src::ext::rtree::rtree::U8_0;
                    } else if pc < iFree2 {
                        *pAddr.offset(0_isize) = ((pc + sz2) >> 8 as ::core::ffi::c_int)
                            as crate::src::ext::rtree::rtree::U8_0;
                        *pAddr.offset(1_isize) =
                            (pc + sz2) as crate::src::ext::rtree::rtree::U8_0;
                    }
                    pAddr = pAddr.offset(2_isize);
                }
                current_block = 13707613154239713890;
            } else {
                current_block = 12997042908615822766;
            }
        } else {
            current_block = 12997042908615822766;
        }
    } else {
        current_block = 12997042908615822766;
    }
    if current_block == 12997042908615822766 {
        cbrk = usableSize;
        iCellLast = usableSize - 4 as ::core::ffi::c_int;
        iCellStart = (*(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar)
            .offset(0_isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_uchar)
                .offset(1_isize) as ::core::ffi::c_int;
        if nCell > 0 as ::core::ffi::c_int {
            temp = crate::src::src::pager::sqlite3PagerTempSpace((*__pPage_ref.pBt).pPager)
                as *mut ::core::ffi::c_uchar;
            ::core::ptr::copy_nonoverlapping(
                data as *const u8,
                temp as *mut u8,
                usableSize as usize,
            );
            src = temp;
            i = 0 as ::core::ffi::c_int;
            while i < nCell {
                
                let pAddr_0: *mut crate::src::ext::rtree::rtree::U8_0 = data.offset((cellOffset + i * 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_uchar
                    as *mut crate::src::ext::rtree::rtree::U8_0;
                pc = (*pAddr_0.offset(0_isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *pAddr_0.offset(1_isize) as ::core::ffi::c_int;
                if pc > iCellLast {
                    return crate::src::src::main::sqlite3CorruptError(
                        1702 as ::core::ffi::c_int,
                    );
                }
                size = __pPage_ref.xCellSize.expect("non-null function pointer")(
                    pPage,
                    src.offset(pc as isize) as *mut crate::src::ext::rtree::rtree::U8_0,
                ) as ::core::ffi::c_int;
                cbrk -= size;
                if cbrk < iCellStart || pc + size > usableSize {
                    return crate::src::src::main::sqlite3CorruptError(
                        1708 as ::core::ffi::c_int,
                    );
                }
                *pAddr_0.offset(0_isize) =
                    (cbrk >> 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0;
                *pAddr_0.offset(1_isize) = cbrk as crate::src::ext::rtree::rtree::U8_0;
                ::core::ptr::copy_nonoverlapping(
                    src.offset(pc as isize) as *mut ::core::ffi::c_uchar as *const u8,
                    data.offset(cbrk as isize) as *mut ::core::ffi::c_uchar as *mut u8,
                    size as usize,
                );
                i += 1;
            }
        }
        *data.offset((hdr + 7 as ::core::ffi::c_int) as isize) = 0 as ::core::ffi::c_uchar;
    }
    if *data.offset((hdr + 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int + cbrk
        - iCellFirst
        != __pPage_ref.nFree
    {
        return crate::src::src::main::sqlite3CorruptError(1722 as ::core::ffi::c_int);
    }
    *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_uchar)
        .offset(0_isize) = (cbrk >> 8 as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::U8_0
        as ::core::ffi::c_uchar;
    *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_uchar)
        .offset(1_isize) = cbrk as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_uchar;
    *data.offset((hdr + 1 as ::core::ffi::c_int) as isize) = 0 as ::core::ffi::c_uchar;
    *data.offset((hdr + 2 as ::core::ffi::c_int) as isize) = 0 as ::core::ffi::c_uchar;
    ::libc::memset(
        data.offset(iCellFirst as isize) as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (cbrk - iCellFirst) as crate::__stddef_size_t_h::SizeT,
    );
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn pageFindSlot(
    pPg: *mut crate::src::headers::btreeInt_h::MemPage,
    nByte: ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) -> *mut crate::src::ext::rtree::rtree::U8_0 {
    let __pPg_ref = unsafe { &mut *pPg };
    let hdr: ::core::ffi::c_int = __pPg_ref.hdrOffset as ::core::ffi::c_int;
    let aData: *mut crate::src::ext::rtree::rtree::U8_0 = __pPg_ref.aData;
    let mut iAddr: ::core::ffi::c_int = hdr + 1 as ::core::ffi::c_int;
    let mut pTmp: *mut crate::src::ext::rtree::rtree::U8_0 =
        aData.offset(iAddr as isize) as *mut crate::src::ext::rtree::rtree::U8_0;
    let mut pc: ::core::ffi::c_int = (*pTmp.offset(0_isize) as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *pTmp.offset(1_isize) as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int;
    let maxPC: ::core::ffi::c_int = (*__pPg_ref.pBt)
        .usableSize
        .wrapping_sub(nByte as crate::src::ext::rtree::rtree::U32_0)
        as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int;
    while pc <= maxPC {
        pTmp = aData.offset((pc + 2 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0;
        size = (*pTmp.offset(0_isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | *pTmp.offset(1_isize) as ::core::ffi::c_int;
        x = size - nByte;
        if x >= 0 as ::core::ffi::c_int {
            if x < 4 as ::core::ffi::c_int {
                if *aData.offset((hdr + 7 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    > 57 as ::core::ffi::c_int
                {
                    return ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
                }
                ::core::ptr::copy_nonoverlapping(
                    aData.offset(pc as isize) as *mut crate::src::ext::rtree::rtree::U8_0
                        as *const u8,
                    aData.offset(iAddr as isize) as *mut crate::src::ext::rtree::rtree::U8_0
                        as *mut u8,
                    2_usize,
                );
                let fresh20 = &mut *aData.offset((hdr + 7 as ::core::ffi::c_int) as isize);
                *fresh20 = (*fresh20 as ::core::ffi::c_int
                    + x as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::U8_0;
                return aData.offset(pc as isize) as *mut crate::src::ext::rtree::rtree::U8_0;
            } else if x + pc > maxPC {
                *pRc = crate::src::src::main::sqlite3CorruptError(1779 as ::core::ffi::c_int);
                return ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
            } else {
                *(aData.offset((pc + 2 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(0_isize) =
                    (x >> 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0;
                *(aData.offset((pc + 2 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(1_isize) = x as crate::src::ext::rtree::rtree::U8_0;
            }
            return aData.offset((pc + x) as isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        }
        iAddr = pc;
        pTmp = aData.offset(pc as isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        pc = (*pTmp.offset(0_isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | *pTmp.offset(1_isize) as ::core::ffi::c_int;
        if pc <= iAddr {
            if pc != 0 {
                *pRc = crate::src::src::main::sqlite3CorruptError(1794 as ::core::ffi::c_int);
            }
            return ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
        }
    }
    if pc > maxPC + nByte - 4 as ::core::ffi::c_int {
        *pRc = crate::src::src::main::sqlite3CorruptError(1801 as ::core::ffi::c_int);
    }
    ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>()
}
#[inline(always)]
unsafe extern "C" fn allocateSpace(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    nByte: ::core::ffi::c_int,
    pIdx: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pPage_ref = unsafe { &*pPage };
    let hdr: ::core::ffi::c_int = __pPage_ref.hdrOffset as ::core::ffi::c_int;
    let data: *mut crate::src::ext::rtree::rtree::U8_0 = __pPage_ref.aData;
    let mut top: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    
    
    let gap: ::core::ffi::c_int = __pPage_ref.cellOffset as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int * __pPage_ref.nCell as ::core::ffi::c_int;
    let pTmp: *mut crate::src::ext::rtree::rtree::U8_0 = data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0;
    top = (*pTmp.offset(0_isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | *pTmp.offset(1_isize) as ::core::ffi::c_int;
    if gap > top {
        if top == 0 as ::core::ffi::c_int
            && (*__pPage_ref.pBt).usableSize == 65536 as crate::src::ext::rtree::rtree::U32_0
        {
            top = 65536 as ::core::ffi::c_int;
        } else {
            return crate::src::src::main::sqlite3CorruptError(1849 as ::core::ffi::c_int);
        }
    } else if top > (*__pPage_ref.pBt).usableSize as ::core::ffi::c_int {
        return crate::src::src::main::sqlite3CorruptError(1852 as ::core::ffi::c_int);
    }
    if (*data.offset((hdr + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != 0
        || *data.offset((hdr + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != 0)
        && gap + 2 as ::core::ffi::c_int <= top
    {
        let pSpace: *mut crate::src::ext::rtree::rtree::U8_0 =
            pageFindSlot(pPage, nByte, &raw mut rc);
        if !pSpace.is_null() {
            
            let g2: ::core::ffi::c_int = pSpace.offset_from(data) as ::core::ffi::c_long as ::core::ffi::c_int;
            *pIdx = g2;
            if g2 <= gap {
                return crate::src::src::main::sqlite3CorruptError(1869 as ::core::ffi::c_int);
            } else {
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        } else if rc != 0 {
            return rc;
        }
    }
    if gap + 2 as ::core::ffi::c_int + nByte > top {
        rc = defragmentPage(
            pPage,
            if (4 as ::core::ffi::c_int) < __pPage_ref.nFree - (2 as ::core::ffi::c_int + nByte) {
                4 as ::core::ffi::c_int
            } else {
                __pPage_ref.nFree - (2 as ::core::ffi::c_int + nByte)
            },
        );
        if rc != 0 {
            return rc;
        }
        top = ((((*(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(0_isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0)
                .offset(1_isize) as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) & 0xffff as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int;
    }
    top -= nByte;
    *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(0_isize) =
        (top >> 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0;
    *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(1_isize) = top as crate::src::ext::rtree::rtree::U8_0;
    *pIdx = top;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn freeSpace(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    mut iStart: ::core::ffi::c_int,
    mut iSize: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut iPtr: ::core::ffi::c_int;
    let mut iFreeBlk: ::core::ffi::c_int;
    
    let mut nFrag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let iOrigSize: ::core::ffi::c_int = iSize;
    
    let mut iEnd: ::core::ffi::c_int = iStart + iSize;
    let __pPage_ref = unsafe { &mut *pPage };
    let data: *mut ::core::ffi::c_uchar = __pPage_ref.aData as *mut ::core::ffi::c_uchar;
    
    let hdr: crate::src::ext::rtree::rtree::U8_0 = __pPage_ref.hdrOffset;
    iPtr = hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    if *data.offset((iPtr + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && *data.offset(iPtr as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        iFreeBlk = 0 as ::core::ffi::c_int;
    } else {
        loop {
            iFreeBlk = (*(data.offset(iPtr as isize) as *mut ::core::ffi::c_uchar)
                .offset(0_isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(data.offset(iPtr as isize) as *mut ::core::ffi::c_uchar).offset(1_isize)
                    as ::core::ffi::c_int;
            if (iFreeBlk >= iStart) {
                break;
            }
            if iFreeBlk <= iPtr {
                if iFreeBlk == 0 as ::core::ffi::c_int {
                    break;
                }
                return crate::src::src::main::sqlite3CorruptError(1948 as ::core::ffi::c_int);
            } else {
                iPtr = iFreeBlk;
            }
        }
        if iFreeBlk > (*__pPage_ref.pBt).usableSize as ::core::ffi::c_int - 4 as ::core::ffi::c_int
        {
            return crate::src::src::main::sqlite3CorruptError(1953 as ::core::ffi::c_int);
        }
        if iFreeBlk != 0 && iEnd + 3 as ::core::ffi::c_int >= iFreeBlk {
            nFrag = iFreeBlk - iEnd;
            if iEnd > iFreeBlk {
                return crate::src::src::main::sqlite3CorruptError(1965 as ::core::ffi::c_int);
            }
            iEnd = iFreeBlk
                + ((*(data.offset((iFreeBlk + 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_uchar)
                    .offset(0_isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(data.offset((iFreeBlk + 2 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar)
                        .offset(1_isize) as ::core::ffi::c_int);
            if iEnd > (*__pPage_ref.pBt).usableSize as ::core::ffi::c_int {
                return crate::src::src::main::sqlite3CorruptError(1968 as ::core::ffi::c_int);
            }
            iSize = iEnd - iStart;
            iFreeBlk = (*(data.offset(iFreeBlk as isize) as *mut ::core::ffi::c_uchar)
                .offset(0_isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(data.offset(iFreeBlk as isize) as *mut ::core::ffi::c_uchar).offset(1_isize)
                    as ::core::ffi::c_int;
        }
        if iPtr > hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
            let iPtrEnd: ::core::ffi::c_int = iPtr
                + ((*(data.offset((iPtr + 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_uchar)
                    .offset(0_isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(data.offset((iPtr + 2 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar)
                        .offset(1_isize) as ::core::ffi::c_int);
            if iPtrEnd + 3 as ::core::ffi::c_int >= iStart {
                if iPtrEnd > iStart {
                    return crate::src::src::main::sqlite3CorruptError(1981 as ::core::ffi::c_int);
                }
                nFrag += iStart - iPtrEnd;
                iSize = iEnd - iPtr;
                iStart = iPtr;
            }
        }
        if nFrag
            > *data.offset((hdr as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
        {
            return crate::src::src::main::sqlite3CorruptError(1987 as ::core::ffi::c_int);
        }
        let fresh21 = &mut *data.offset((hdr as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize);
        *fresh21 = (*fresh21 as ::core::ffi::c_int
            - nFrag as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
    }
    let pTmp: *mut crate::src::ext::rtree::rtree::U8_0 = data.offset((hdr as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_uchar as *mut crate::src::ext::rtree::rtree::U8_0;
    let x: ::core::ffi::c_int = (*pTmp.offset(0_isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | *pTmp.offset(1_isize) as ::core::ffi::c_int;
    if (*__pPage_ref.pBt).btsFlags as ::core::ffi::c_int
        & crate::src::headers::btreeInt_h::BTS_FAST_SECURE
        != 0
    {
        ::libc::memset(
            data.offset(iStart as isize) as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            iSize as crate::__stddef_size_t_h::SizeT,
        );
    }
    if iStart <= x {
        if iStart < x {
            return crate::src::src::main::sqlite3CorruptError(2001 as ::core::ffi::c_int);
        }
        if iPtr != hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
            return crate::src::src::main::sqlite3CorruptError(2002 as ::core::ffi::c_int);
        }
        *(data.offset((hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar)
            .offset(0_isize) = (iFreeBlk >> 8 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U8_0
            as ::core::ffi::c_uchar;
        *(data.offset((hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar)
            .offset(1_isize) =
            iFreeBlk as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_uchar;
        *(data.offset((hdr as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar)
            .offset(0_isize) = (iEnd >> 8 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U8_0
            as ::core::ffi::c_uchar;
        *(data.offset((hdr as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_uchar)
            .offset(1_isize) =
            iEnd as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_uchar;
    } else {
        *(data.offset(iPtr as isize) as *mut ::core::ffi::c_uchar).offset(0_isize) =
            (iStart >> 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0
                as ::core::ffi::c_uchar;
        *(data.offset(iPtr as isize) as *mut ::core::ffi::c_uchar).offset(1_isize) =
            iStart as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_uchar;
        *(data.offset(iStart as isize) as *mut ::core::ffi::c_uchar).offset(0_isize) =
            (iFreeBlk >> 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0
                as ::core::ffi::c_uchar;
        *(data.offset(iStart as isize) as *mut ::core::ffi::c_uchar).offset(1_isize) =
            iFreeBlk as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_uchar;
        *(data.offset((iStart + 2 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_uchar)
            .offset(0_isize) =
            (iSize as crate::src::fts5::U16_0 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_uchar;
        *(data.offset((iStart + 2 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_uchar)
            .offset(1_isize) = iSize as crate::src::fts5::U16_0
            as crate::src::ext::rtree::rtree::U8_0
            as ::core::ffi::c_uchar;
    }
    __pPage_ref.nFree += iOrigSize;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn decodeFlags(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    flagByte: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*pPage).pBt;
    (*pPage).max1bytePayload = (*pBt).max1bytePayload;
    if flagByte
        >= crate::src::headers::btreeInt_h::PTF_ZERODATA | crate::src::headers::btreeInt_h::PTF_LEAF
    {
        (*pPage).childPtrSize = 0 as crate::src::ext::rtree::rtree::U8_0;
        (*pPage).leaf = 1 as crate::src::ext::rtree::rtree::U8_0;
        if flagByte
            == crate::src::headers::btreeInt_h::PTF_LEAFDATA
                | crate::src::headers::btreeInt_h::PTF_INTKEY
                | crate::src::headers::btreeInt_h::PTF_LEAF
        {
            let __pPage_ref = unsafe { &mut *pPage };
            __pPage_ref.intKeyLeaf = 1 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.xCellSize = Some(
                cellSizePtrTableLeaf
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
                >;
            __pPage_ref.xParseCell = Some(
                btreeParseCellPtr
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
                >;
            __pPage_ref.intKey = 1 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.maxLocal = (*pBt).maxLeaf;
            __pPage_ref.minLocal = (*pBt).minLeaf;
        } else if flagByte
            == crate::src::headers::btreeInt_h::PTF_ZERODATA
                | crate::src::headers::btreeInt_h::PTF_LEAF
        {
            let __pPage_ref = unsafe { &mut *pPage };
            __pPage_ref.intKey = 0 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.intKeyLeaf = 0 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.xCellSize = Some(
                cellSizePtrIdxLeaf
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
                >;
            __pPage_ref.xParseCell = Some(
                btreeParseCellPtrIndex
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
                >;
            __pPage_ref.maxLocal = (*pBt).maxLocal;
            __pPage_ref.minLocal = (*pBt).minLocal;
        } else {
            let __pPage_ref = unsafe { &mut *pPage };
            __pPage_ref.intKey = 0 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.intKeyLeaf = 0 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.xCellSize = Some(
                cellSizePtrIdxLeaf
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
                >;
            __pPage_ref.xParseCell = Some(
                btreeParseCellPtrIndex
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
                >;
            return crate::src::src::main::sqlite3CorruptError(2057 as ::core::ffi::c_int);
        }
    } else {
        (*pPage).childPtrSize = 4 as crate::src::ext::rtree::rtree::U8_0;
        (*pPage).leaf = 0 as crate::src::ext::rtree::rtree::U8_0;
        if flagByte == 0x2 as ::core::ffi::c_int {
            let __pPage_ref = unsafe { &mut *pPage };
            __pPage_ref.intKey = 0 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.intKeyLeaf = 0 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.xCellSize = Some(
                cellSizePtr
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
                >;
            __pPage_ref.xParseCell = Some(
                btreeParseCellPtrIndex
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
                >;
            __pPage_ref.maxLocal = (*pBt).maxLocal;
            __pPage_ref.minLocal = (*pBt).minLocal;
        } else if flagByte
            == crate::src::headers::btreeInt_h::PTF_LEAFDATA
                | crate::src::headers::btreeInt_h::PTF_INTKEY
        {
            let __pPage_ref = unsafe { &mut *pPage };
            __pPage_ref.intKeyLeaf = 0 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.xCellSize = Some(
                cellSizePtrNoPayload
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
                >;
            __pPage_ref.xParseCell = Some(
                btreeParseCellPtrNoPayload
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
                >;
            __pPage_ref.intKey = 1 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.maxLocal = (*pBt).maxLeaf;
            __pPage_ref.minLocal = (*pBt).minLeaf;
        } else {
            let __pPage_ref = unsafe { &mut *pPage };
            __pPage_ref.intKey = 0 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.intKeyLeaf = 0 as crate::src::ext::rtree::rtree::U8_0;
            __pPage_ref.xCellSize = Some(
                cellSizePtr
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                    ) -> crate::src::fts5::U16_0,
                >;
            __pPage_ref.xParseCell = Some(
                btreeParseCellPtrIndex
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::MemPage,
                        *mut crate::src::ext::rtree::rtree::U8_0,
                        *mut crate::src::headers::btreeInt_h::CellInfo,
                    ) -> (),
                >;
            return crate::src::src::main::sqlite3CorruptError(2081 as ::core::ffi::c_int);
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn btreeComputeFreeSpace(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
) -> ::core::ffi::c_int {
    let mut pc: ::core::ffi::c_int;
    
    
    
    let mut nFree: ::core::ffi::c_int;
    
    
    
    let __pPage_ref = unsafe { &mut *pPage };
    let usableSize: ::core::ffi::c_int = (*__pPage_ref.pBt).usableSize as ::core::ffi::c_int;
    let hdr: crate::src::ext::rtree::rtree::U8_0 = __pPage_ref.hdrOffset;
    let data: *mut crate::src::ext::rtree::rtree::U8_0 = __pPage_ref.aData;
    let top: ::core::ffi::c_int = ((((*(data.offset((hdr as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(0_isize) as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *(data.offset((hdr as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(1_isize) as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int) & 0xffff as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int;
    let iCellFirst: ::core::ffi::c_int = hdr as ::core::ffi::c_int
        + 8 as ::core::ffi::c_int
        + __pPage_ref.childPtrSize as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int * __pPage_ref.nCell as ::core::ffi::c_int;
    let iCellLast: ::core::ffi::c_int = usableSize - 4 as ::core::ffi::c_int;
    pc = (*(data.offset((hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(0_isize) as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *(data.offset((hdr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(1_isize) as ::core::ffi::c_int;
    nFree = *data.offset((hdr as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        + top;
    if pc > 0 as ::core::ffi::c_int {
        let mut next: crate::src::ext::rtree::rtree::U32_0;
        let mut size: crate::src::ext::rtree::rtree::U32_0;
        if pc < top {
            return crate::src::src::main::sqlite3CorruptError(2132 as ::core::ffi::c_int);
        }
        loop {
            if pc > iCellLast {
                return crate::src::src::main::sqlite3CorruptError(2137 as ::core::ffi::c_int);
            }
            next = ((*(data.offset(pc as isize) as *mut crate::src::ext::rtree::rtree::U8_0)
                .offset(0_isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(data.offset(pc as isize) as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(1_isize) as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U32_0;
            size = ((*(data.offset((pc + 2 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0)
                .offset(0_isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(data.offset((pc + 2 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(1_isize) as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U32_0;
            nFree = (nFree as crate::src::ext::rtree::rtree::U32_0).wrapping_add(size)
                as ::core::ffi::c_int;
            if next
                <= (pc as crate::src::ext::rtree::rtree::U32_0)
                    .wrapping_add(size)
                    .wrapping_add(3 as crate::src::ext::rtree::rtree::U32_0)
            {
                break;
            }
            pc = next as ::core::ffi::c_int;
        }
        if next > 0 as crate::src::ext::rtree::rtree::U32_0 {
            return crate::src::src::main::sqlite3CorruptError(2147 as ::core::ffi::c_int);
        }
        if (pc as crate::src::ext::rtree::rtree::U32_0).wrapping_add(size)
            > usableSize as crate::src::ext::rtree::rtree::U32_0
        {
            return crate::src::src::main::sqlite3CorruptError(2151 as ::core::ffi::c_int);
        }
    }
    if nFree > usableSize || nFree < iCellFirst {
        return crate::src::src::main::sqlite3CorruptError(2163 as ::core::ffi::c_int);
    }
    __pPage_ref.nFree = (nFree - iCellFirst) as crate::src::fts5::U16_0 as ::core::ffi::c_int;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[inline(never)]
unsafe extern "C" fn btreeCellSizeCheck(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
) -> ::core::ffi::c_int {
    
    let mut iCellLast: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut sz: ::core::ffi::c_int;
    let mut pc: ::core::ffi::c_int;
    
    
    
    let __pPage_ref = unsafe { &mut *pPage };
    let iCellFirst: ::core::ffi::c_int = __pPage_ref.cellOffset as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int * __pPage_ref.nCell as ::core::ffi::c_int;
    let usableSize: ::core::ffi::c_int = (*__pPage_ref.pBt).usableSize as ::core::ffi::c_int;
    iCellLast = usableSize - 4 as ::core::ffi::c_int;
    let data: *mut crate::src::ext::rtree::rtree::U8_0 = __pPage_ref.aData;
    let cellOffset: ::core::ffi::c_int = __pPage_ref.cellOffset as ::core::ffi::c_int;
    if __pPage_ref.leaf == 0 {
        iCellLast -= 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < __pPage_ref.nCell as ::core::ffi::c_int {
        pc = (*(data.offset((cellOffset + i * 2 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(0_isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(data.offset((cellOffset + i * 2 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0)
                .offset(1_isize) as ::core::ffi::c_int;
        if pc < iCellFirst || pc > iCellLast {
            return crate::src::src::main::sqlite3CorruptError(2194 as ::core::ffi::c_int);
        }
        sz = __pPage_ref.xCellSize.expect("non-null function pointer")(
            pPage,
            data.offset(pc as isize) as *mut crate::src::ext::rtree::rtree::U8_0,
        ) as ::core::ffi::c_int;
        if pc + sz > usableSize {
            return crate::src::src::main::sqlite3CorruptError(2199 as ::core::ffi::c_int);
        }
        i += 1;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn btreeInitPage(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
) -> ::core::ffi::c_int {
    
    
    let __pPage_ref = unsafe { &mut *pPage };
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __pPage_ref.pBt;
    let data: *mut crate::src::ext::rtree::rtree::U8_0 = (*pPage)
        .aData
        .offset(__pPage_ref.hdrOffset as ::core::ffi::c_int as isize);
    if decodeFlags(pPage, *data.offset(0_isize) as ::core::ffi::c_int) != 0 {
        return crate::src::src::main::sqlite3CorruptError(2231 as ::core::ffi::c_int);
    }
    __pPage_ref.maskPage = (*pBt)
        .pageSize
        .wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::fts5::U16_0;
    __pPage_ref.nOverflow = 0 as crate::src::ext::rtree::rtree::U8_0;
    __pPage_ref.cellOffset = (__pPage_ref.hdrOffset as ::core::ffi::c_int
        + 8 as ::core::ffi::c_int
        + __pPage_ref.childPtrSize as ::core::ffi::c_int)
        as crate::src::fts5::U16_0;
    __pPage_ref.aCellIdx = data
        .offset(__pPage_ref.childPtrSize as ::core::ffi::c_int as isize)
        .offset(8_isize);
    __pPage_ref.aDataEnd = __pPage_ref.aData.offset((*pBt).pageSize as isize);
    __pPage_ref.aDataOfst = (*pPage)
        .aData
        .offset(__pPage_ref.childPtrSize as ::core::ffi::c_int as isize);
    __pPage_ref.nCell = ((*(data.offset(3_isize) as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(0_isize) as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *(data.offset(3_isize) as *mut crate::src::ext::rtree::rtree::U8_0).offset(1_isize)
            as ::core::ffi::c_int) as crate::src::fts5::U16_0;
    if __pPage_ref.nCell as crate::src::ext::rtree::rtree::U32_0
        > (*pBt)
            .pageSize
            .wrapping_sub(8 as crate::src::ext::rtree::rtree::U32_0)
            .wrapping_div(6 as crate::src::ext::rtree::rtree::U32_0)
    {
        return crate::src::src::main::sqlite3CorruptError(2245 as ::core::ffi::c_int);
    }
    __pPage_ref.nFree = -(1 as ::core::ffi::c_int);
    __pPage_ref.isInit = 1 as crate::src::ext::rtree::rtree::U8_0;
    if (*(*pBt).db).flags
        & crate::src::headers::sqliteInt_h::SQLITE_CellSizeCk
            as crate::src::ext::rtree::rtree::U64_0
        != 0
    {
        return btreeCellSizeCheck(pPage);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn zeroPage(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    flags: ::core::ffi::c_int,
) {
    let __pPage_ref = unsafe { &mut *pPage };
    let data: *mut ::core::ffi::c_uchar = __pPage_ref.aData as *mut ::core::ffi::c_uchar;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __pPage_ref.pBt;
    let hdr: ::core::ffi::c_int = __pPage_ref.hdrOffset as ::core::ffi::c_int;
    
    let __pBt_ref = unsafe { &mut *pBt };
    if __pBt_ref.btsFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTS_FAST_SECURE
        != 0
    {
        ::libc::memset(
            data.offset(hdr as isize) as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __pBt_ref
                .usableSize
                .wrapping_sub(hdr as crate::src::ext::rtree::rtree::U32_0)
                as crate::__stddef_size_t_h::SizeT,
        );
    }
    *data.offset(hdr as isize) = flags as ::core::ffi::c_char as ::core::ffi::c_uchar;
    let first: ::core::ffi::c_int = hdr
        + (if flags & crate::src::headers::btreeInt_h::PTF_LEAF == 0 as ::core::ffi::c_int {
            12 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        });
    ::libc::memset(
        data.offset((hdr + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_uchar
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        4 as crate::__stddef_size_t_h::SizeT,
    );
    *data.offset((hdr + 7 as ::core::ffi::c_int) as isize) = 0 as ::core::ffi::c_uchar;
    *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_uchar)
        .offset(0_isize) = (__pBt_ref.usableSize >> 8 as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::U8_0
        as ::core::ffi::c_uchar;
    *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_uchar)
        .offset(1_isize) =
        __pBt_ref.usableSize as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_uchar;
    __pPage_ref.nFree = __pBt_ref
        .usableSize
        .wrapping_sub(first as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::fts5::U16_0 as ::core::ffi::c_int;
    decodeFlags(pPage, flags);
    __pPage_ref.cellOffset = first as crate::src::fts5::U16_0;
    __pPage_ref.aDataEnd = data.offset(__pBt_ref.pageSize as isize) as *mut ::core::ffi::c_uchar
        as *mut crate::src::ext::rtree::rtree::U8_0;
    __pPage_ref.aCellIdx = data.offset(first as isize) as *mut ::core::ffi::c_uchar
        as *mut crate::src::ext::rtree::rtree::U8_0;
    __pPage_ref.aDataOfst = data.offset(__pPage_ref.childPtrSize as isize)
        as *mut ::core::ffi::c_uchar
        as *mut crate::src::ext::rtree::rtree::U8_0;
    __pPage_ref.nOverflow = 0 as crate::src::ext::rtree::rtree::U8_0;
    __pPage_ref.maskPage = __pBt_ref
        .pageSize
        .wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::fts5::U16_0;
    __pPage_ref.nCell = 0 as crate::src::fts5::U16_0;
    __pPage_ref.isInit = 1 as crate::src::ext::rtree::rtree::U8_0;
}

unsafe extern "C" fn btreePageFromDbPage(
    pDbPage: *mut crate::src::src::pager::DbPage,
    pgno: crate::src::src::pager::Pgno,
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
) -> *mut crate::src::headers::btreeInt_h::MemPage {
    let pPage: *mut crate::src::headers::btreeInt_h::MemPage =
        crate::src::src::pager::sqlite3PagerGetExtra(pDbPage as *mut crate::src::src::pcache::PgHdr)
            as *mut crate::src::headers::btreeInt_h::MemPage;
    if pgno != (*pPage).pgno {
        let __pPage_ref = unsafe { &mut *pPage };
        __pPage_ref.aData = crate::src::src::pager::sqlite3PagerGetData(
            pDbPage as *mut crate::src::src::pcache::PgHdr,
        ) as *mut crate::src::ext::rtree::rtree::U8_0;
        __pPage_ref.pDbPage = pDbPage;
        __pPage_ref.pBt = pBt;
        __pPage_ref.pgno = pgno;
        __pPage_ref.hdrOffset = (if pgno == 1 as crate::src::src::pager::Pgno {
            100 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as crate::src::ext::rtree::rtree::U8_0;
    }
    pPage
}

unsafe extern "C" fn btreeGetPage(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pgno: crate::src::src::pager::Pgno,
    ppPage: *mut *mut crate::src::headers::btreeInt_h::MemPage,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let mut pDbPage: *mut crate::src::src::pager::DbPage =
        ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
    let rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerGet(
        (*pBt).pPager,
        pgno,
        &raw mut pDbPage as *mut _ as *mut *mut crate::src::src::pcache::PgHdr,
        flags,
    );
    if rc != 0 {
        return rc;
    }
    *ppPage = btreePageFromDbPage(pDbPage, pgno, pBt);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn btreePageLookup(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pgno: crate::src::src::pager::Pgno,
) -> *mut crate::src::headers::btreeInt_h::MemPage {
    
    let pDbPage: *mut crate::src::src::pager::DbPage = crate::src::src::pager::sqlite3PagerLookup((*pBt).pPager, pgno)
        as *mut crate::src::src::pcache::PgHdr;
    if !pDbPage.is_null() {
        return btreePageFromDbPage(pDbPage, pgno, pBt);
    }
    ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>()
}

unsafe extern "C" fn btreePagecount(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
) -> crate::src::src::pager::Pgno {
    (*pBt).nPage as crate::src::src::pager::Pgno
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeLastPage(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> crate::src::src::pager::Pgno {
    btreePagecount((*p).pBt)
}

unsafe extern "C" fn getAndInitPage(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pgno: crate::src::src::pager::Pgno,
    ppPage: *mut *mut crate::src::headers::btreeInt_h::MemPage,
    bReadOnly: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pDbPage: *mut crate::src::src::pager::DbPage =
        ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
    
    if pgno > btreePagecount(pBt) {
        *ppPage = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
        return crate::src::src::main::sqlite3CorruptError(2388 as ::core::ffi::c_int);
    }
    rc = crate::src::src::pager::sqlite3PagerGet(
        (*pBt).pPager,
        pgno,
        &raw mut pDbPage as *mut _ as *mut *mut crate::src::src::pcache::PgHdr,
        bReadOnly,
    );
    if rc != 0 {
        *ppPage = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
        return rc;
    }
    let pPage: *mut crate::src::headers::btreeInt_h::MemPage = crate::src::src::pager::sqlite3PagerGetExtra(
        pDbPage as *mut crate::src::src::pcache::PgHdr,
    ) as *mut crate::src::headers::btreeInt_h::MemPage;
    if (*pPage).isInit as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        btreePageFromDbPage(pDbPage, pgno, pBt);
        rc = btreeInitPage(pPage);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            releasePage(pPage);
            *ppPage = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
            return rc;
        }
    }
    *ppPage = pPage;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn releasePageNotNull(pPage: *mut crate::src::headers::btreeInt_h::MemPage) {
    crate::src::src::pager::sqlite3PagerUnrefNotNull(
        (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
    );
}

unsafe extern "C" fn releasePage(pPage: *mut crate::src::headers::btreeInt_h::MemPage) {
    if !pPage.is_null() {
        releasePageNotNull(pPage);
    }
}

unsafe extern "C" fn releasePageOne(pPage: *mut crate::src::headers::btreeInt_h::MemPage) {
    crate::src::src::pager::sqlite3PagerUnrefPageOne(
        (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
    );
}

unsafe extern "C" fn btreeGetUnusedPage(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pgno: crate::src::src::pager::Pgno,
    ppPage: *mut *mut crate::src::headers::btreeInt_h::MemPage,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int = btreeGetPage(pBt, pgno, ppPage, flags);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if crate::src::src::pager::sqlite3PagerPageRefcount(
            (**ppPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
        ) > 1 as ::core::ffi::c_int
        {
            releasePage(*ppPage);
            *ppPage = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
            return crate::src::src::main::sqlite3CorruptError(2460 as ::core::ffi::c_int);
        }
        (**ppPage).isInit = 0 as crate::src::ext::rtree::rtree::U8_0;
    } else {
        *ppPage = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    }
    rc
}

unsafe extern "C" fn pageReinit(pData: *mut crate::src::src::pager::DbPage) {
    
    let pPage: *mut crate::src::headers::btreeInt_h::MemPage = crate::src::src::pager::sqlite3PagerGetExtra(pData as *mut crate::src::src::pcache::PgHdr)
            as *mut crate::src::headers::btreeInt_h::MemPage;
    if (*pPage).isInit != 0 {
        (*pPage).isInit = 0 as crate::src::ext::rtree::rtree::U8_0;
        if crate::src::src::pager::sqlite3PagerPageRefcount(
            pData as *mut crate::src::src::pcache::PgHdr,
        ) > 1 as ::core::ffi::c_int
        {
            btreeInitPage(pPage);
        }
    }
}

unsafe extern "C" fn btreeInvokeBusyHandler(
    pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared =
        pArg as *mut crate::src::headers::btreeInt_h::BtShared;
    crate::src::src::main::sqlite3InvokeBusyHandler(
        &raw mut (*(*pBt).db).busyHandler as *mut _
            as *mut crate::src::headers::sqliteInt_h::BusyHandler,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeOpen(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zFilename: *const ::core::ffi::c_char,
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    ppBtree: *mut *mut crate::src::headers::btreeInt_h::Btree,
    mut flags: ::core::ffi::c_int,
    mut vfsFlags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pBt: *mut crate::src::headers::btreeInt_h::BtShared =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtShared>();
    
    let mut mutexOpen: *mut crate::src::src::mutex_unix::sqlite3_mutex =
        ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let nReserve: crate::src::ext::rtree::rtree::U8_0;
    let mut zDbHeader: [::core::ffi::c_uchar; 100] = [0; 100];
    let isTempDb: ::core::ffi::c_int = (zFilename.is_null()
        || *zFilename.offset(0_isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let isMemdb: ::core::ffi::c_int = (!zFilename.is_null()
        && ::libc::strcmp(
            zFilename,
            b":memory:\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || isTempDb != 0
            && crate::src::src::main::sqlite3TempInMemory(
                db as *const crate::src::headers::sqliteInt_h::sqlite3,
            ) != 0
        || vfsFlags & crate::src::headers::sqlite3_h::SQLITE_OPEN_MEMORY != 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if isMemdb != 0 {
        flags |= crate::src::src::btree::BTREE_MEMORY;
    }
    if vfsFlags & crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_DB != 0 as ::core::ffi::c_int
        && (isMemdb != 0 || isTempDb != 0)
    {
        vfsFlags = vfsFlags & !crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_DB
            | crate::src::headers::sqlite3_h::SQLITE_OPEN_TEMP_DB;
    }
    let p: *mut crate::src::headers::btreeInt_h::Btree = crate::src::src::malloc::sqlite3MallocZero(::core::mem::size_of::<
        crate::src::headers::btreeInt_h::Btree,
    >() as crate::src::ext::rtree::rtree::U64_0)
        as *mut crate::src::headers::btreeInt_h::Btree;
    if p.is_null() {
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    (*p).inTrans =
        crate::src::headers::btreeInt_h::TRANS_NONE as crate::src::ext::rtree::rtree::U8_0;
    (*p).db = db;
    (*p).lock.pBtree = p;
    (*p).lock.iTable = 1 as crate::src::src::pager::Pgno;
    if isTempDb == 0 as ::core::ffi::c_int
        && (isMemdb == 0 as ::core::ffi::c_int
            || vfsFlags & crate::src::headers::sqlite3_h::SQLITE_OPEN_URI
                != 0 as ::core::ffi::c_int)
        && vfsFlags & crate::src::headers::sqlite3_h::SQLITE_OPEN_SHAREDCACHE != 0 {
            let nFilename: ::core::ffi::c_int =
                crate::src::src::util::sqlite3Strlen30(zFilename) + 1 as ::core::ffi::c_int;
            let nFullPathname: ::core::ffi::c_int =
                (*pVfs).mxPathname + 1 as ::core::ffi::c_int;
            let zFullPathname: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3Malloc(
                (if nFullPathname > nFilename {
                    nFullPathname
                } else {
                    nFilename
                }) as crate::src::ext::rtree::rtree::U64_0,
            )
                as *mut ::core::ffi::c_char;
            
            (*p).sharable = 1 as crate::src::ext::rtree::rtree::U8_0;
            if zFullPathname.is_null() {
                crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
                return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
            }
            if isMemdb != 0 {
                ::core::ptr::copy_nonoverlapping(
                    zFilename as *const u8,
                    zFullPathname as *mut u8,
                    nFilename as usize,
                );
            } else {
                rc = crate::src::src::os::sqlite3OsFullPathname(
                    pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                    zFilename,
                    nFullPathname,
                    zFullPathname,
                );
                if rc != 0 {
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK_SYMLINK {
                        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                    } else {
                        crate::src::src::malloc::sqlite3_free(
                            zFullPathname as *mut ::core::ffi::c_void,
                        );
                        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
                        return rc;
                    }
                }
            }
            mutexOpen = crate::src::src::mutex::sqlite3MutexAlloc(
                crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_OPEN,
            );
            crate::src::src::mutex::sqlite3_mutex_enter(mutexOpen);
            let mutexShared: *mut crate::src::src::mutex_unix::sqlite3_mutex = crate::src::src::mutex::sqlite3MutexAlloc(
                crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_MAIN,
            );
            crate::src::src::mutex::sqlite3_mutex_enter(mutexShared);
            pBt = sqlite3SharedCacheList;
            while !pBt.is_null() {
                if 0 as ::core::ffi::c_int
                    == ::libc::strcmp(
                        zFullPathname,
                        crate::src::src::pager::sqlite3PagerFilename(
                            (*pBt).pPager,
                            0 as ::core::ffi::c_int,
                        ),
                    )
                    && crate::src::src::pager::sqlite3PagerVfs((*pBt).pPager)
                        as *mut crate::src::headers::sqlite3_h::sqlite3_vfs
                        == pVfs
                {
                    let mut iDb: ::core::ffi::c_int;
                    iDb = (*db).nDb - 1 as ::core::ffi::c_int;
                    while iDb >= 0 as ::core::ffi::c_int {
                        let pExisting: *mut crate::src::headers::btreeInt_h::Btree =
                            (*(*db).aDb.offset(iDb as isize)).pBt;
                        if !pExisting.is_null() && (*pExisting).pBt == pBt {
                            crate::src::src::mutex::sqlite3_mutex_leave(mutexShared);
                            crate::src::src::mutex::sqlite3_mutex_leave(mutexOpen);
                            crate::src::src::malloc::sqlite3_free(
                                zFullPathname as *mut ::core::ffi::c_void,
                            );
                            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
                            return crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
                        }
                        iDb -= 1;
                    }
                    (*p).pBt = pBt;
                    (*pBt).nRef += 1;
                    break;
                } else {
                    pBt = (*pBt).pNext;
                }
            }
            crate::src::src::mutex::sqlite3_mutex_leave(mutexShared);
            crate::src::src::malloc::sqlite3_free(zFullPathname as *mut ::core::ffi::c_void);
        }
    if pBt.is_null() {
        ::libc::memset(
            (&raw mut zDbHeader as *mut ::core::ffi::c_uchar).offset(16_isize)
                as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            8 as crate::__stddef_size_t_h::SizeT,
        );
        pBt = crate::src::src::malloc::sqlite3MallocZero(::core::mem::size_of::<
            crate::src::headers::btreeInt_h::BtShared,
        >()
            as crate::src::ext::rtree::rtree::U64_0)
            as *mut crate::src::headers::btreeInt_h::BtShared;
        if pBt.is_null() {
            rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
            current_block = 16994982316810399751;
        } else {
            rc = crate::src::src::pager::sqlite3PagerOpen(
                pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                &raw mut (*pBt).pPager,
                zFilename,
                ::core::mem::size_of::<crate::src::headers::btreeInt_h::MemPage>()
                    as ::core::ffi::c_int,
                flags,
                vfsFlags,
                ::core::mem::transmute(Some(
                    pageReinit as unsafe extern "C" fn(*mut crate::src::src::pager::DbPage) -> (),
                )),
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::pager::sqlite3PagerSetMmapLimit(
                    (*pBt).pPager,
                    (*db).szMmap as crate::src::headers::sqlite3_h::Sqlite3Int64,
                );
                rc = crate::src::src::pager::sqlite3PagerReadFileheader(
                    (*pBt).pPager,
                    ::core::mem::size_of::<[::core::ffi::c_uchar; 100]>() as ::core::ffi::c_int,
                    &raw mut zDbHeader as *mut ::core::ffi::c_uchar,
                );
            }
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                current_block = 16994982316810399751;
            } else {
                let __pBt_ref = unsafe { &mut *pBt };
                __pBt_ref.openFlags = flags as crate::src::ext::rtree::rtree::U8_0;
                __pBt_ref.db = db;
                crate::src::src::pager::sqlite3PagerSetBusyHandler(
                    __pBt_ref.pPager,
                    Some(
                        btreeInvokeBusyHandler
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
                    ),
                    pBt as *mut ::core::ffi::c_void,
                );
                (*p).pBt = pBt;
                __pBt_ref.pCursor =
                    ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                __pBt_ref.pPage1 =
                    ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                if crate::src::src::pager::sqlite3PagerIsreadonly(__pBt_ref.pPager) != 0 {
                    __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
                        | crate::src::headers::btreeInt_h::BTS_READ_ONLY)
                        as crate::src::fts5::U16_0;
                }
                __pBt_ref.pageSize = ((zDbHeader[16 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | (zDbHeader[17 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                        << 16 as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::U32_0;
                if __pBt_ref.pageSize < 512 as crate::src::ext::rtree::rtree::U32_0
                    || __pBt_ref.pageSize
                        > crate::sqliteLimit_h::SQLITE_MAX_PAGE_SIZE
                            as crate::src::ext::rtree::rtree::U32_0
                    || __pBt_ref
                        .pageSize
                        .wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0)
                        & __pBt_ref.pageSize
                        != 0 as crate::src::ext::rtree::rtree::U32_0
                {
                    __pBt_ref.pageSize = 0 as crate::src::ext::rtree::rtree::U32_0;
                    if !zFilename.is_null() && isMemdb == 0 {
                        __pBt_ref.autoVacuum =
                            (if crate::src::src::btree::SQLITE_DEFAULT_AUTOVACUUM != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }) as crate::src::ext::rtree::rtree::U8_0;
                        __pBt_ref.incrVacuum = (if crate::src::src::btree::SQLITE_DEFAULT_AUTOVACUUM
                            == 2 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                            as crate::src::ext::rtree::rtree::U8_0;
                    }
                    nReserve = 0 as crate::src::ext::rtree::rtree::U8_0;
                } else {
                    nReserve = zDbHeader[20 as ::core::ffi::c_int as usize]
                        as crate::src::ext::rtree::rtree::U8_0;
                    __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
                        | crate::src::headers::btreeInt_h::BTS_PAGESIZE_FIXED)
                        as crate::src::fts5::U16_0;
                    __pBt_ref.autoVacuum = (if crate::src::src::util::sqlite3Get4byte(
                        (&raw mut zDbHeader as *mut ::core::ffi::c_uchar).offset(
                            (36 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut ::core::ffi::c_uchar,
                    ) != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                        as crate::src::ext::rtree::rtree::U8_0;
                    __pBt_ref.incrVacuum = (if crate::src::src::util::sqlite3Get4byte(
                        (&raw mut zDbHeader as *mut ::core::ffi::c_uchar).offset(
                            (36 as ::core::ffi::c_int
                                + 7 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut ::core::ffi::c_uchar,
                    ) != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                        as crate::src::ext::rtree::rtree::U8_0;
                }
                rc = crate::src::src::pager::sqlite3PagerSetPagesize(
                    __pBt_ref.pPager,
                    &raw mut __pBt_ref.pageSize,
                    nReserve as ::core::ffi::c_int,
                );
                if rc != 0 {
                    current_block = 16994982316810399751;
                } else {
                    __pBt_ref.usableSize = __pBt_ref
                        .pageSize
                        .wrapping_sub(nReserve as crate::src::ext::rtree::rtree::U32_0);
                    __pBt_ref.nRef = 1 as ::core::ffi::c_int;
                    if (*p).sharable != 0 {
                        
                        let mutexShared_0: *mut crate::src::src::mutex_unix::sqlite3_mutex = crate::src::src::mutex::sqlite3MutexAlloc(2 as ::core::ffi::c_int);
                        if crate::internal::SQLITE_THREADSAFE != 0
                            && crate::src::src::global::sqlite3Config.bCoreMutex
                                as ::core::ffi::c_int
                                != 0
                        {
                            __pBt_ref.mutex = crate::src::src::mutex::sqlite3MutexAlloc(
                                crate::src::headers::sqlite3_h::SQLITE_MUTEX_FAST,
                            );
                            if __pBt_ref.mutex.is_null() {
                                rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                                current_block = 16994982316810399751;
                            } else {
                                current_block = 2750570471926810434;
                            }
                        } else {
                            current_block = 2750570471926810434;
                        }
                        match current_block {
                            16994982316810399751 => {}
                            _ => {
                                crate::src::src::mutex::sqlite3_mutex_enter(mutexShared_0);
                                __pBt_ref.pNext = sqlite3SharedCacheList;
                                sqlite3SharedCacheList = pBt;
                                crate::src::src::mutex::sqlite3_mutex_leave(mutexShared_0);
                                current_block = 7301440000599063274;
                            }
                        }
                    } else {
                        current_block = 7301440000599063274;
                    }
                }
            }
        }
    } else {
        current_block = 7301440000599063274;
    }
    if current_block == 7301440000599063274 {
        if (*p).sharable != 0 {
            let mut i: ::core::ffi::c_int;
            let mut pSib: *mut crate::src::headers::btreeInt_h::Btree;
            i = 0 as ::core::ffi::c_int;
            while i < (*db).nDb {
                pSib = (*(*db).aDb.offset(i as isize)).pBt;
                if !pSib.is_null() && (*pSib).sharable as ::core::ffi::c_int != 0 {
                    while !(*pSib).pPrev.is_null() {
                        pSib = (*pSib).pPrev;
                    }
                    if ((*p).pBt as crate::src::headers::sqliteInt_h::Uptr)
                        < (*pSib).pBt as crate::src::headers::sqliteInt_h::Uptr
                    {
                        (*p).pNext = pSib;
                        (*p).pPrev =
                            ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
                        (*pSib).pPrev = p;
                    } else {
                        let __p_ref = unsafe { &mut *p };
                        while !(*pSib).pNext.is_null()
                            && ((*(*pSib).pNext).pBt as crate::src::headers::sqliteInt_h::Uptr)
                                < __p_ref.pBt as crate::src::headers::sqliteInt_h::Uptr
                        {
                            pSib = (*pSib).pNext;
                        }
                        __p_ref.pNext = (*pSib).pNext;
                        __p_ref.pPrev = pSib;
                        if !__p_ref.pNext.is_null() {
                            (*__p_ref.pNext).pPrev = p;
                        }
                        (*pSib).pNext = p;
                    }
                    break;
                } else {
                    i += 1;
                }
            }
        }
        *ppBtree = p;
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        if !pBt.is_null() && !(*pBt).pPager.is_null() {
            crate::src::src::pager::sqlite3PagerClose(
                (*pBt).pPager,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>()
                    as *mut crate::src::headers::sqliteInt_h::sqlite3,
            );
        }
        crate::src::src::malloc::sqlite3_free(pBt as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        *ppBtree = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
    } else {
        
        if sqlite3BtreeSchema(p, 0 as ::core::ffi::c_int, None).is_null() {
            sqlite3BtreeSetCacheSize(p, crate::sqliteLimit_h::SQLITE_DEFAULT_CACHE_SIZE);
        }
        let pFile: *mut crate::src::headers::sqlite3_h::sqlite3_file = crate::src::src::pager::sqlite3PagerFile((*pBt).pPager)
            as *mut crate::src::headers::sqlite3_h::sqlite3_file;
        if !(*pFile).pMethods.is_null() {
            crate::src::src::os::sqlite3OsFileControlHint(
                pFile as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                crate::src::headers::sqlite3_h::SQLITE_FCNTL_PDB,
                &raw mut (*pBt).db as *mut ::core::ffi::c_void,
            );
        }
    }
    if !mutexOpen.is_null() {
        crate::src::src::mutex::sqlite3_mutex_leave(mutexOpen);
    }
    rc
}

unsafe extern "C" fn removeFromSharingList(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
) -> ::core::ffi::c_int {
    
    let mut pList: *mut crate::src::headers::btreeInt_h::BtShared;
    let mut removed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let pMainMtx: *mut crate::src::src::mutex_unix::sqlite3_mutex = crate::src::src::mutex::sqlite3MutexAlloc(2 as ::core::ffi::c_int);
    crate::src::src::mutex::sqlite3_mutex_enter(pMainMtx);
    (*pBt).nRef -= 1;
    if (*pBt).nRef <= 0 as ::core::ffi::c_int {
        if sqlite3SharedCacheList == pBt {
            sqlite3SharedCacheList = (*pBt).pNext;
        } else {
            pList = sqlite3SharedCacheList;
            while !pList.is_null() && (*pList).pNext != pBt {
                pList = (*pList).pNext;
            }
            if !pList.is_null() {
                (*pList).pNext = (*pBt).pNext;
            }
        }
        crate::src::src::mutex::sqlite3_mutex_free((*pBt).mutex);
        removed = 1 as ::core::ffi::c_int;
    }
    crate::src::src::mutex::sqlite3_mutex_leave(pMainMtx);
    removed
}
#[inline(never)]
unsafe extern "C" fn allocateTempSpace(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
) -> ::core::ffi::c_int {
    let __pBt_ref = unsafe { &mut *pBt };
    __pBt_ref.pTmpSpace =
        crate::src::src::pcache1::sqlite3PageMalloc(__pBt_ref.pageSize as ::core::ffi::c_int)
            as *mut crate::src::ext::rtree::rtree::U8_0;
    if __pBt_ref.pTmpSpace.is_null() {
        let pCur: *mut crate::src::headers::btreeInt_h::BtCursor = __pBt_ref.pCursor;
        __pBt_ref.pCursor = (*pCur).pNext;
        ::libc::memset(
            pCur as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::src::headers::btreeInt_h::BtCursor>()
                as crate::__stddef_size_t_h::SizeT,
        );
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    ::libc::memset(
        __pBt_ref.pTmpSpace as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        8 as crate::__stddef_size_t_h::SizeT,
    );
    __pBt_ref.pTmpSpace = __pBt_ref.pTmpSpace.offset(4_isize);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn freeTempSpace(pBt: *mut crate::src::headers::btreeInt_h::BtShared) {
    if !(*pBt).pTmpSpace.is_null() {
        let __pBt_ref = unsafe { &mut *pBt };
        __pBt_ref.pTmpSpace = __pBt_ref
            .pTmpSpace
            .offset(-(4 as ::core::ffi::c_int as isize));
        crate::src::src::pcache1::sqlite3PageFree(__pBt_ref.pTmpSpace as *mut ::core::ffi::c_void);
        __pBt_ref.pTmpSpace = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeClose(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    let __p_ref = unsafe { &mut *p };
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __p_ref.pBt;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    sqlite3BtreeRollback(
        p,
        crate::src::headers::sqlite3_h::SQLITE_OK,
        0 as ::core::ffi::c_int,
    );
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    if __p_ref.sharable == 0 || removeFromSharingList(pBt) != 0 {
        let __pBt_ref = unsafe { &mut *pBt };
        crate::src::src::pager::sqlite3PagerClose(
            __pBt_ref.pPager,
            __p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        );
        if __pBt_ref.xFreeSchema.is_some() && !__pBt_ref.pSchema.is_null() {
            __pBt_ref.xFreeSchema.expect("non-null function pointer")(__pBt_ref.pSchema);
        }
        crate::src::src::malloc::sqlite3DbFree(
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>()
                as *mut crate::src::headers::sqliteInt_h::sqlite3,
            __pBt_ref.pSchema,
        );
        freeTempSpace(pBt);
        crate::src::src::malloc::sqlite3_free(pBt as *mut ::core::ffi::c_void);
    }
    if !__p_ref.pPrev.is_null() {
        (*__p_ref.pPrev).pNext = __p_ref.pNext;
    }
    if !__p_ref.pNext.is_null() {
        (*__p_ref.pNext).pPrev = __p_ref.pPrev;
    }
    crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSetCacheSize(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    mxPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    crate::src::src::pager::sqlite3PagerSetCachesize((*pBt).pPager, mxPage);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSetSpillSize(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    mxPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let res: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerSetSpillsize((*pBt).pPager, mxPage);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    res
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSetMmapLimit(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    szMmap: crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    crate::src::src::pager::sqlite3PagerSetMmapLimit((*pBt).pPager, szMmap);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSetPagerFlags(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    pgFlags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    crate::src::src::pager::sqlite3PagerSetFlags((*pBt).pPager, pgFlags);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSetPageSize(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    mut pageSize: ::core::ffi::c_int,
    mut nReserve: ::core::ffi::c_int,
    iFix: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let __pBt_ref = unsafe { &mut *pBt };
    __pBt_ref.nReserveWanted = nReserve as crate::src::ext::rtree::rtree::U8_0;
    let x: ::core::ffi::c_int = __pBt_ref.pageSize.wrapping_sub(__pBt_ref.usableSize) as ::core::ffi::c_int;
    if x == nReserve
        && (pageSize == 0 as ::core::ffi::c_int
            || pageSize as crate::src::ext::rtree::rtree::U32_0 == __pBt_ref.pageSize)
    {
        crate::src::src::btmutex::sqlite3BtreeLeave(
            p as *mut crate::src::headers::btreeInt_h::Btree,
        );
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if nReserve < x {
        nReserve = x;
    }
    if __pBt_ref.btsFlags as ::core::ffi::c_int
        & crate::src::headers::btreeInt_h::BTS_PAGESIZE_FIXED
        != 0
    {
        crate::src::src::btmutex::sqlite3BtreeLeave(
            p as *mut crate::src::headers::btreeInt_h::Btree,
        );
        return crate::src::headers::sqlite3_h::SQLITE_READONLY;
    }
    if pageSize >= 512 as ::core::ffi::c_int
        && pageSize <= crate::sqliteLimit_h::SQLITE_MAX_PAGE_SIZE
        && (pageSize - 1 as ::core::ffi::c_int) & pageSize == 0 as ::core::ffi::c_int
    {
        if nReserve > 32 as ::core::ffi::c_int && pageSize == 512 as ::core::ffi::c_int {
            pageSize = 1024 as ::core::ffi::c_int;
        }
        __pBt_ref.pageSize = pageSize as crate::src::ext::rtree::rtree::U32_0;
        freeTempSpace(pBt);
    }
    let rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerSetPagesize(
        __pBt_ref.pPager,
        &raw mut __pBt_ref.pageSize,
        nReserve,
    );
    __pBt_ref.usableSize = __pBt_ref
        .pageSize
        .wrapping_sub(nReserve as crate::src::fts5::U16_0 as crate::src::ext::rtree::rtree::U32_0);
    if iFix != 0 {
        __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
            | crate::src::headers::btreeInt_h::BTS_PAGESIZE_FIXED)
            as crate::src::fts5::U16_0;
    }
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeGetPageSize(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    (*(*p).pBt).pageSize as ::core::ffi::c_int
}
pub unsafe extern "C" fn sqlite3BtreeGetReserveNoMutex(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    
    let __pBt_ref = &*(*p).pBt;
    let n: ::core::ffi::c_int = __pBt_ref.pageSize.wrapping_sub(__pBt_ref.usableSize) as ::core::ffi::c_int;
    n
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeGetRequestedReserve(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    
    
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let n1: ::core::ffi::c_int = (*(*p).pBt).nReserveWanted as ::core::ffi::c_int;
    let n2: ::core::ffi::c_int = sqlite3BtreeGetReserveNoMutex(p);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    if n1 > n2 { n1 } else { n2 }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeMaxPageCount(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    mxPage: crate::src::src::pager::Pgno,
) -> crate::src::src::pager::Pgno {
    
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let n: crate::src::src::pager::Pgno = crate::src::src::pager::sqlite3PagerMaxPageCount((*(*p).pBt).pPager, mxPage);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    n
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSecureDelete(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    newFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    if p.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    if newFlag >= 0 as ::core::ffi::c_int {
        let __p_ref = unsafe { &mut *p };
        (*__p_ref.pBt).btsFlags = ((*__p_ref.pBt).btsFlags as ::core::ffi::c_int
            & !crate::src::headers::btreeInt_h::BTS_FAST_SECURE)
            as crate::src::fts5::U16_0;
        (*__p_ref.pBt).btsFlags = ((*__p_ref.pBt).btsFlags as ::core::ffi::c_int
            | (crate::src::headers::btreeInt_h::BTS_SECURE_DELETE * newFlag)
                as crate::src::fts5::U16_0 as ::core::ffi::c_int)
            as crate::src::fts5::U16_0;
    }
    let b: ::core::ffi::c_int = ((*(*p).pBt).btsFlags as ::core::ffi::c_int
        & crate::src::headers::btreeInt_h::BTS_FAST_SECURE)
        / crate::src::headers::btreeInt_h::BTS_SECURE_DELETE;
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    b
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSetAutoVacuum(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    autoVacuum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let av: crate::src::ext::rtree::rtree::U8_0 =
        autoVacuum as crate::src::ext::rtree::rtree::U8_0;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    if (*pBt).btsFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTS_PAGESIZE_FIXED
        != 0 as ::core::ffi::c_int
        && (if av as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) != (*pBt).autoVacuum as ::core::ffi::c_int
    {
        rc = crate::src::headers::sqlite3_h::SQLITE_READONLY;
    } else {
        (*pBt).autoVacuum = (if av as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as crate::src::ext::rtree::rtree::U8_0;
        (*pBt).incrVacuum = (if av as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as crate::src::ext::rtree::rtree::U8_0;
    }
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeGetAutoVacuum(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let __pBt_ref = &*(*p).pBt;
    let rc: ::core::ffi::c_int = if __pBt_ref.autoVacuum == 0 {
        crate::src::src::btree::BTREE_AUTOVACUUM_NONE
    } else if __pBt_ref.incrVacuum == 0 {
        crate::src::src::btree::BTREE_AUTOVACUUM_FULL
    } else {
        crate::src::src::btree::BTREE_AUTOVACUUM_INCR
    };
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}

unsafe extern "C" fn lockBtree(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int;
    let mut pPage1: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let mut nPage: crate::src::ext::rtree::rtree::U32_0;
    let mut nPageFile: crate::src::ext::rtree::rtree::U32_0 =
        0 as crate::src::ext::rtree::rtree::U32_0;
    let __pBt_ref = unsafe { &mut *pBt };
    rc = crate::src::src::pager::sqlite3PagerSharedLock(__pBt_ref.pPager);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    rc = btreeGetPage(
        pBt,
        1 as crate::src::src::pager::Pgno,
        &raw mut pPage1,
        0 as ::core::ffi::c_int,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    let __pPage1_ref = unsafe { &mut *pPage1 };
    nPage = crate::src::src::util::sqlite3Get4byte(__pPage1_ref.aData.offset(28_isize));
    crate::src::src::pager::sqlite3PagerPagecount(
        __pBt_ref.pPager,
        &raw mut nPageFile as *mut ::core::ffi::c_int,
    );
    if nPage == 0 as crate::src::ext::rtree::rtree::U32_0
        || ::libc::memcmp(
            __pPage1_ref.aData.offset(24_isize) as *const ::core::ffi::c_void,
            __pPage1_ref.aData.offset(92_isize) as *const ::core::ffi::c_void,
            4 as crate::__stddef_size_t_h::SizeT,
        ) != 0 as ::core::ffi::c_int
    {
        nPage = nPageFile;
    }
    if (*__pBt_ref.db).flags
        & crate::src::headers::sqliteInt_h::SQLITE_ResetDatabase
            as crate::src::ext::rtree::rtree::U64_0
        != 0 as crate::src::ext::rtree::rtree::U64_0
    {
        nPage = 0 as crate::src::ext::rtree::rtree::U32_0;
    }
    if nPage > 0 as crate::src::ext::rtree::rtree::U32_0 {
        let pageSize: crate::src::ext::rtree::rtree::U32_0;
        let usableSize: crate::src::ext::rtree::rtree::U32_0;
        let page1: *mut crate::src::ext::rtree::rtree::U8_0 = __pPage1_ref.aData;
        rc = crate::src::headers::sqlite3_h::SQLITE_NOTADB;
        if ::libc::memcmp(
            page1 as *const ::core::ffi::c_void,
            &raw const zMagicHeader as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            16 as crate::__stddef_size_t_h::SizeT,
        ) != 0 as ::core::ffi::c_int
        {
            current_block = 5106468414449065519;
        } else {
            if *page1.offset(18_isize) as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
                __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
                    | crate::src::headers::btreeInt_h::BTS_READ_ONLY)
                    as crate::src::fts5::U16_0;
            }
            if *page1.offset(19_isize) as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
                current_block = 5106468414449065519;
            } else {
                if *page1.offset(19_isize) as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                    && __pBt_ref.btsFlags as ::core::ffi::c_int
                        & crate::src::headers::btreeInt_h::BTS_NO_WAL
                        == 0 as ::core::ffi::c_int
                {
                    let mut isOpen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    rc = crate::src::src::pager::sqlite3PagerOpenWal(
                        __pBt_ref.pPager,
                        &raw mut isOpen,
                    );
                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                        current_block = 5106468414449065519;
                    } else {
                        if isOpen == 0 as ::core::ffi::c_int {
                            releasePageOne(pPage1);
                            return crate::src::headers::sqlite3_h::SQLITE_OK;
                        }
                        rc = crate::src::headers::sqlite3_h::SQLITE_NOTADB;
                        current_block = 17788412896529399552;
                    }
                } else {
                    current_block = 17788412896529399552;
                }
                match current_block {
                    5106468414449065519 => {}
                    _ => {
                        if ::libc::memcmp(
                            page1.offset(21_isize) as *mut crate::src::ext::rtree::rtree::U8_0
                                as *const ::core::ffi::c_void,
                            b"@  \0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            3 as crate::__stddef_size_t_h::SizeT,
                        ) != 0 as ::core::ffi::c_int
                        {
                            current_block = 5106468414449065519;
                        } else {
                            pageSize = ((*page1.offset(16_isize) as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int
                                | (*page1.offset(17_isize) as ::core::ffi::c_int)
                                    << 16 as ::core::ffi::c_int)
                                as crate::src::ext::rtree::rtree::U32_0;
                            if pageSize.wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0)
                                & pageSize
                                != 0 as crate::src::ext::rtree::rtree::U32_0
                                || pageSize
                                    > crate::sqliteLimit_h::SQLITE_MAX_PAGE_SIZE
                                        as crate::src::ext::rtree::rtree::U32_0
                                || pageSize <= 256 as crate::src::ext::rtree::rtree::U32_0
                            {
                                current_block = 5106468414449065519;
                            } else {
                                usableSize = pageSize.wrapping_sub(*page1.offset(20_isize)
                                    as crate::src::ext::rtree::rtree::U32_0);
                                if pageSize != __pBt_ref.pageSize {
                                    releasePageOne(pPage1);
                                    __pBt_ref.usableSize = usableSize;
                                    __pBt_ref.pageSize = pageSize;
                                    __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
                                        | crate::src::headers::btreeInt_h::BTS_PAGESIZE_FIXED)
                                        as crate::src::fts5::U16_0;
                                    freeTempSpace(pBt);
                                    rc = crate::src::src::pager::sqlite3PagerSetPagesize(
                                        __pBt_ref.pPager,
                                        &raw mut __pBt_ref.pageSize,
                                        pageSize.wrapping_sub(usableSize) as ::core::ffi::c_int,
                                    );
                                    return rc;
                                }
                                if nPage > nPageFile {
                                    if crate::src::src::build::sqlite3WritableSchema(
                                        __pBt_ref.db
                                            as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        rc = crate::src::src::main::sqlite3CorruptError(
                                            3403 as ::core::ffi::c_int,
                                        );
                                        current_block = 5106468414449065519;
                                    } else {
                                        nPage = nPageFile;
                                        current_block = 6717214610478484138;
                                    }
                                } else {
                                    current_block = 6717214610478484138;
                                }
                                match current_block {
                                    5106468414449065519 => {}
                                    _ => {
                                        if usableSize < 480 as crate::src::ext::rtree::rtree::U32_0
                                        {
                                            current_block = 5106468414449065519;
                                        } else {
                                            __pBt_ref.btsFlags = (__pBt_ref.btsFlags
                                                as ::core::ffi::c_int
                                                | crate::src::headers::btreeInt_h::BTS_PAGESIZE_FIXED)
                                                as crate::src::fts5::U16_0;
                                            __pBt_ref.pageSize = pageSize;
                                            __pBt_ref.usableSize = usableSize;
                                            __pBt_ref.autoVacuum =
                                                (if crate::src::src::util::sqlite3Get4byte(
                                                    page1.offset(
                                                        (36 as ::core::ffi::c_int
                                                            + 4 as ::core::ffi::c_int
                                                                * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as *mut crate::src::ext::rtree::rtree::U8_0,
                                                ) != 0
                                                {
                                                    1 as ::core::ffi::c_int
                                                } else {
                                                    0 as ::core::ffi::c_int
                                                })
                                                    as crate::src::ext::rtree::rtree::U8_0;
                                            __pBt_ref.incrVacuum =
                                                (if crate::src::src::util::sqlite3Get4byte(
                                                    page1.offset(
                                                        (36 as ::core::ffi::c_int
                                                            + 7 as ::core::ffi::c_int
                                                                * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as *mut crate::src::ext::rtree::rtree::U8_0,
                                                ) != 0
                                                {
                                                    1 as ::core::ffi::c_int
                                                } else {
                                                    0 as ::core::ffi::c_int
                                                })
                                                    as crate::src::ext::rtree::rtree::U8_0;
                                            current_block = 11793792312832361944;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            11793792312832361944 => {}
            _ => {
                releasePageOne(pPage1);
                __pBt_ref.pPage1 =
                    ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                return rc;
            }
        }
    }
    __pBt_ref.maxLocal = (*pBt)
        .usableSize
        .wrapping_sub(12 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_mul(64 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_div(255 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_sub(23 as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::fts5::U16_0;
    __pBt_ref.minLocal = (*pBt)
        .usableSize
        .wrapping_sub(12 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_mul(32 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_div(255 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_sub(23 as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::fts5::U16_0;
    __pBt_ref.maxLeaf = __pBt_ref
        .usableSize
        .wrapping_sub(35 as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::fts5::U16_0;
    __pBt_ref.minLeaf = (*pBt)
        .usableSize
        .wrapping_sub(12 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_mul(32 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_div(255 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_sub(23 as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::fts5::U16_0;
    if __pBt_ref.maxLocal as ::core::ffi::c_int > 127 as ::core::ffi::c_int {
        __pBt_ref.max1bytePayload = 127 as crate::src::ext::rtree::rtree::U8_0;
    } else {
        __pBt_ref.max1bytePayload = __pBt_ref.maxLocal as crate::src::ext::rtree::rtree::U8_0;
    }
    __pBt_ref.pPage1 = pPage1;
    __pBt_ref.nPage = nPage;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unlockBtreeIfUnused(pBt: *mut crate::src::headers::btreeInt_h::BtShared) {
    if (*pBt).inTransaction as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_NONE
        && !(*pBt).pPage1.is_null()
    {
        let pPage1: *mut crate::src::headers::btreeInt_h::MemPage = (*pBt).pPage1;
        (*pBt).pPage1 = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
        releasePageOne(pPage1);
    }
}

unsafe extern "C" fn newDatabase(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
) -> ::core::ffi::c_int {
    
    
    
    let __pBt_ref = unsafe { &mut *pBt };
    if __pBt_ref.nPage > 0 as crate::src::ext::rtree::rtree::U32_0 {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    let pP1: *mut crate::src::headers::btreeInt_h::MemPage = __pBt_ref.pPage1;
    let data: *mut ::core::ffi::c_uchar = (*pP1).aData as *mut ::core::ffi::c_uchar;
    let rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerWrite(
        (*pP1).pDbPage as *mut crate::src::src::pcache::PgHdr,
    );
    if rc != 0 {
        return rc;
    }
    ::core::ptr::copy_nonoverlapping(
        &raw const zMagicHeader as *const ::core::ffi::c_char as *const u8,
        data as *mut u8,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as usize,
    );
    *data.offset(16_isize) = (__pBt_ref.pageSize >> 8 as ::core::ffi::c_int
        & 0xff as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::ext::rtree::rtree::U8_0
        as ::core::ffi::c_uchar;
    *data.offset(17_isize) = (__pBt_ref.pageSize >> 16 as ::core::ffi::c_int
        & 0xff as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::ext::rtree::rtree::U8_0
        as ::core::ffi::c_uchar;
    *data.offset(18_isize) = 1 as ::core::ffi::c_uchar;
    *data.offset(19_isize) = 1 as ::core::ffi::c_uchar;
    *data.offset(20_isize) = __pBt_ref.pageSize.wrapping_sub(__pBt_ref.usableSize)
        as crate::src::ext::rtree::rtree::U8_0
        as ::core::ffi::c_uchar;
    *data.offset(21_isize) = 64 as ::core::ffi::c_uchar;
    *data.offset(22_isize) = 32 as ::core::ffi::c_uchar;
    *data.offset(23_isize) = 32 as ::core::ffi::c_uchar;
    ::libc::memset(
        data.offset(24_isize) as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (100 as ::core::ffi::c_int - 24 as ::core::ffi::c_int) as crate::__stddef_size_t_h::SizeT,
    );
    zeroPage(
        pP1,
        crate::src::headers::btreeInt_h::PTF_INTKEY
            | crate::src::headers::btreeInt_h::PTF_LEAF
            | crate::src::headers::btreeInt_h::PTF_LEAFDATA,
    );
    __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
        | crate::src::headers::btreeInt_h::BTS_PAGESIZE_FIXED)
        as crate::src::fts5::U16_0;
    crate::src::src::util::sqlite3Put4byte(
        data.offset(
            (36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::ext::rtree::rtree::U8_0,
        __pBt_ref.autoVacuum as crate::src::ext::rtree::rtree::U32_0,
    );
    crate::src::src::util::sqlite3Put4byte(
        data.offset(
            (36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::ext::rtree::rtree::U8_0,
        __pBt_ref.incrVacuum as crate::src::ext::rtree::rtree::U32_0,
    );
    __pBt_ref.nPage = 1 as crate::src::ext::rtree::rtree::U32_0;
    *data.offset(31_isize) = 1 as ::core::ffi::c_uchar;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeNewDb(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    (*(*p).pBt).nPage = 0 as crate::src::ext::rtree::rtree::U32_0;
    let rc: ::core::ffi::c_int = newDatabase((*p).pBt);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[inline(never)]
unsafe extern "C" fn btreeBeginTrans(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    wrflag: ::core::ffi::c_int,
    pSchemaVersion: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __p_ref = unsafe { &mut *p };
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __p_ref.pBt;
    let pPager: *mut crate::src::src::pager::Pager = (*pBt).pPager;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    if !(__p_ref.inTrans as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_WRITE
        || __p_ref.inTrans as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_READ
            && wrflag == 0)
    {
        if (*__p_ref.db).flags
            & crate::src::headers::sqliteInt_h::SQLITE_ResetDatabase
                as crate::src::ext::rtree::rtree::U64_0
            != 0
            && crate::src::src::pager::sqlite3PagerIsreadonly(pPager) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            (*pBt).btsFlags = ((*pBt).btsFlags as ::core::ffi::c_int
                & !crate::src::headers::btreeInt_h::BTS_READ_ONLY)
                as crate::src::fts5::U16_0;
        }
        if (*pBt).btsFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTS_READ_ONLY
            != 0 as ::core::ffi::c_int
            && wrflag != 0
        {
            rc = crate::src::headers::sqlite3_h::SQLITE_READONLY;
        } else {
            let mut pBlock: *mut crate::src::headers::sqliteInt_h::sqlite3 =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>();
            if wrflag != 0
                && (*pBt).inTransaction as ::core::ffi::c_int
                    == crate::src::headers::btreeInt_h::TRANS_WRITE
                || (*pBt).btsFlags as ::core::ffi::c_int
                    & crate::src::headers::btreeInt_h::BTS_PENDING
                    != 0 as ::core::ffi::c_int
            {
                pBlock = (*(*pBt).pWriter).db;
            } else if wrflag > 1 as ::core::ffi::c_int {
                let mut pIter: *mut crate::src::headers::btreeInt_h::BtLock;
                pIter = (*pBt).pLock;
                while !pIter.is_null() {
                    if (*pIter).pBtree != p {
                        pBlock = (*(*pIter).pBtree).db;
                        break;
                    } else {
                        pIter = (*pIter).pNext;
                    }
                }
            }
            if !pBlock.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_LOCKED_SHAREDCACHE;
            } else {
                rc = querySharedCacheTableLock(
                    p,
                    crate::src::headers::sqliteInt_h::SCHEMA_ROOT as crate::src::src::pager::Pgno,
                    crate::src::headers::btreeInt_h::READ_LOCK
                        as crate::src::ext::rtree::rtree::U8_0,
                );
                if (crate::src::headers::sqlite3_h::SQLITE_OK == rc) {
                    let __pBt_ref = unsafe { &mut *pBt };
                    __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
                        & !crate::src::headers::btreeInt_h::BTS_INITIALLY_EMPTY)
                        as crate::src::fts5::U16_0;
                    if __pBt_ref.nPage == 0 as crate::src::ext::rtree::rtree::U32_0 {
                        __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
                            | crate::src::headers::btreeInt_h::BTS_INITIALLY_EMPTY)
                            as crate::src::fts5::U16_0;
                    }
                    loop {
                        while __pBt_ref.pPage1.is_null() && {
                            rc = lockBtree(pBt);
                            crate::src::headers::sqlite3_h::SQLITE_OK == rc
                        } {}
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && wrflag != 0 {
                            if __pBt_ref.btsFlags as ::core::ffi::c_int
                                & crate::src::headers::btreeInt_h::BTS_READ_ONLY
                                != 0 as ::core::ffi::c_int
                            {
                                rc = crate::src::headers::sqlite3_h::SQLITE_READONLY;
                            } else {
                                rc = crate::src::src::pager::sqlite3PagerBegin(
                                    pPager,
                                    (wrflag > 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                    crate::src::src::main::sqlite3TempInMemory(
                                        __p_ref.db
                                            as *const crate::src::headers::sqliteInt_h::sqlite3,
                                    ),
                                );
                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                    rc = newDatabase(pBt);
                                } else if rc == crate::src::headers::sqlite3_h::SQLITE_BUSY_SNAPSHOT
                                    && __pBt_ref.inTransaction as ::core::ffi::c_int
                                        == crate::src::headers::btreeInt_h::TRANS_NONE
                                {
                                    rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                                }
                            }
                        }
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            unlockBtreeIfUnused(pBt);
                        }
                        if !(rc & 0xff as ::core::ffi::c_int
                            == crate::src::headers::sqlite3_h::SQLITE_BUSY
                            && __pBt_ref.inTransaction as ::core::ffi::c_int
                                == crate::src::headers::btreeInt_h::TRANS_NONE
                            && btreeInvokeBusyHandler(pBt as *mut ::core::ffi::c_void) != 0)
                        {
                            break;
                        }
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        if __p_ref.inTrans as ::core::ffi::c_int
                            == crate::src::headers::btreeInt_h::TRANS_NONE
                        {
                            __pBt_ref.nTransaction += 1;
                            if __p_ref.sharable != 0 {
                                __p_ref.lock.eLock = crate::src::headers::btreeInt_h::READ_LOCK
                                    as crate::src::ext::rtree::rtree::U8_0;
                                __p_ref.lock.pNext = __pBt_ref.pLock;
                                __pBt_ref.pLock = &raw mut __p_ref.lock;
                            }
                        }
                        __p_ref.inTrans = (if wrflag != 0 {
                            crate::src::headers::btreeInt_h::TRANS_WRITE
                        } else {
                            crate::src::headers::btreeInt_h::TRANS_READ
                        })
                            as crate::src::ext::rtree::rtree::U8_0;
                        if __p_ref.inTrans as ::core::ffi::c_int
                            > __pBt_ref.inTransaction as ::core::ffi::c_int
                        {
                            __pBt_ref.inTransaction = __p_ref.inTrans;
                        }
                        if wrflag != 0 {
                            let pPage1: *mut crate::src::headers::btreeInt_h::MemPage =
                                __pBt_ref.pPage1;
                            __pBt_ref.pWriter = p;
                            __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
                                & !crate::src::headers::btreeInt_h::BTS_EXCLUSIVE)
                                as crate::src::fts5::U16_0;
                            if wrflag > 1 as ::core::ffi::c_int {
                                __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
                                    | crate::src::headers::btreeInt_h::BTS_EXCLUSIVE)
                                    as crate::src::fts5::U16_0;
                            }
                            if __pBt_ref.nPage
                                != crate::src::src::util::sqlite3Get4byte(
                                    (*pPage1).aData.offset(28_isize)
                                        as *mut crate::src::ext::rtree::rtree::U8_0,
                                )
                            {
                                rc = crate::src::src::pager::sqlite3PagerWrite(
                                    (*pPage1).pDbPage as *mut crate::src::src::pcache::PgHdr,
                                );
                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                    crate::src::src::util::sqlite3Put4byte(
                                        (*pPage1).aData.offset(28_isize)
                                            as *mut crate::src::ext::rtree::rtree::U8_0,
                                        __pBt_ref.nPage,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if !pSchemaVersion.is_null() {
            *pSchemaVersion =
                crate::src::src::util::sqlite3Get4byte((*(*pBt).pPage1).aData.offset(40_isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    as ::core::ffi::c_int;
        }
        if wrflag != 0 {
            rc =
                crate::src::src::pager::sqlite3PagerOpenSavepoint(pPager, (*__p_ref.db).nSavepoint);
        }
    }
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeBeginTrans(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    wrflag: ::core::ffi::c_int,
    pSchemaVersion: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let __p_ref = unsafe { &*p };
    if __p_ref.sharable as ::core::ffi::c_int != 0
        || __p_ref.inTrans as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_NONE
        || __p_ref.inTrans as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_READ
            && wrflag != 0 as ::core::ffi::c_int
    {
        return btreeBeginTrans(p, wrflag, pSchemaVersion);
    }
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __p_ref.pBt;
    if !pSchemaVersion.is_null() {
        *pSchemaVersion =
            crate::src::src::util::sqlite3Get4byte((*(*pBt).pPage1).aData.offset(40_isize)
                as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
    }
    if wrflag != 0 {
        crate::src::src::pager::sqlite3PagerOpenSavepoint(
            (*pBt).pPager,
            (*__p_ref.db).nSavepoint,
        )
    } else {
        crate::src::headers::sqlite3_h::SQLITE_OK
    }
}

unsafe extern "C" fn setChildPtrmaps(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    
    let mut rc: ::core::ffi::c_int;
    let __pPage_ref = unsafe { &mut *pPage };
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __pPage_ref.pBt;
    let pgno: crate::src::src::pager::Pgno = __pPage_ref.pgno;
    rc = if __pPage_ref.isInit as ::core::ffi::c_int != 0 {
        crate::src::headers::sqlite3_h::SQLITE_OK
    } else {
        btreeInitPage(pPage)
    };
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    let nCell: ::core::ffi::c_int = __pPage_ref.nCell as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nCell {
        let pCell: *mut crate::src::ext::rtree::rtree::U8_0 = __pPage_ref.aData.offset(
            (__pPage_ref.maskPage as ::core::ffi::c_int
                & ((*((*pPage)
                    .aCellIdx
                    .offset((2 as ::core::ffi::c_int * i) as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(0_isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *((*pPage)
                        .aCellIdx
                        .offset((2 as ::core::ffi::c_int * i) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(1_isize) as ::core::ffi::c_int)) as isize,
        );
        ptrmapPutOvflPtr(pPage, pPage, pCell, &raw mut rc);
        if __pPage_ref.leaf == 0 {
            let childPgno: crate::src::src::pager::Pgno =
                crate::src::src::util::sqlite3Get4byte(pCell) as crate::src::src::pager::Pgno;
            ptrmapPut(
                pBt,
                childPgno,
                crate::src::headers::btreeInt_h::PTRMAP_BTREE
                    as crate::src::ext::rtree::rtree::U8_0,
                pgno,
                &raw mut rc,
            );
        }
        i += 1;
    }
    if __pPage_ref.leaf == 0 {
        let childPgno_0: crate::src::src::pager::Pgno = crate::src::src::util::sqlite3Get4byte(
            __pPage_ref.aData.offset(
                (__pPage_ref.hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::ext::rtree::rtree::U8_0,
        )
            as crate::src::src::pager::Pgno;
        ptrmapPut(
            pBt,
            childPgno_0,
            crate::src::headers::btreeInt_h::PTRMAP_BTREE as crate::src::ext::rtree::rtree::U8_0,
            pgno,
            &raw mut rc,
        );
    }
    rc
}

unsafe extern "C" fn modifyPagePointer(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    iFrom: crate::src::src::pager::Pgno,
    iTo: crate::src::src::pager::Pgno,
    eType: crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    if eType as ::core::ffi::c_int == crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW2 {
        if crate::src::src::util::sqlite3Get4byte((*pPage).aData) != iFrom {
            return crate::src::src::main::sqlite3CorruptError(3858 as ::core::ffi::c_int);
        }
        crate::src::src::util::sqlite3Put4byte(
            (*pPage).aData,
            iTo as crate::src::ext::rtree::rtree::U32_0,
        );
    } else {
        let mut i: ::core::ffi::c_int;
        
        
        let rc: ::core::ffi::c_int = if (*pPage).isInit as ::core::ffi::c_int != 0 {
            crate::src::headers::sqlite3_h::SQLITE_OK
        } else {
            btreeInitPage(pPage)
        };
        if rc != 0 {
            return rc;
        }
        let nCell: ::core::ffi::c_int = (*pPage).nCell as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < nCell {
            let __pPage_ref = unsafe { &mut *pPage };
            let pCell: *mut crate::src::ext::rtree::rtree::U8_0 = __pPage_ref.aData.offset(
                (__pPage_ref.maskPage as ::core::ffi::c_int
                    & ((*((*pPage)
                        .aCellIdx
                        .offset((2 as ::core::ffi::c_int * i) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(0_isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * i) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(1_isize) as ::core::ffi::c_int)) as isize,
            );
            if eType as ::core::ffi::c_int == crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW1 {
                let mut info: crate::src::headers::btreeInt_h::CellInfo =
                    unsafe { ::core::mem::zeroed() };
                __pPage_ref.xParseCell.expect("non-null function pointer")(
                    pPage,
                    pCell,
                    &raw mut info,
                );
                if (info.nLocal as crate::src::ext::rtree::rtree::U32_0) < info.nPayload {
                    if pCell.offset(info.nSize as ::core::ffi::c_int as isize)
                        > __pPage_ref
                            .aData
                            .offset((*__pPage_ref.pBt).usableSize as isize)
                    {
                        return crate::src::src::main::sqlite3CorruptError(
                            3877 as ::core::ffi::c_int,
                        );
                    }
                    if iFrom
                        == crate::src::src::util::sqlite3Get4byte(
                            pCell
                                .offset(info.nSize as ::core::ffi::c_int as isize)
                                .offset(-(4 as ::core::ffi::c_int as isize)),
                        )
                    {
                        crate::src::src::util::sqlite3Put4byte(
                            pCell
                                .offset(info.nSize as ::core::ffi::c_int as isize)
                                .offset(-(4 as ::core::ffi::c_int as isize)),
                            iTo as crate::src::ext::rtree::rtree::U32_0,
                        );
                        break;
                    }
                }
            } else {
                if pCell.offset(4_isize)
                    > __pPage_ref
                        .aData
                        .offset((*__pPage_ref.pBt).usableSize as isize)
                {
                    return crate::src::src::main::sqlite3CorruptError(3886 as ::core::ffi::c_int);
                }
                if crate::src::src::util::sqlite3Get4byte(pCell) == iFrom {
                    crate::src::src::util::sqlite3Put4byte(
                        pCell,
                        iTo as crate::src::ext::rtree::rtree::U32_0,
                    );
                    break;
                }
            }
            i += 1;
        }
        if i == nCell {
            let __pPage_ref = unsafe { &mut *pPage };
            if eType as ::core::ffi::c_int != crate::src::headers::btreeInt_h::PTRMAP_BTREE
                || crate::src::src::util::sqlite3Get4byte(__pPage_ref.aData.offset(
                    (__pPage_ref.hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                        as isize,
                )
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    != iFrom
            {
                return crate::src::src::main::sqlite3CorruptError(3898 as ::core::ffi::c_int);
            }
            crate::src::src::util::sqlite3Put4byte(
                __pPage_ref.aData.offset(
                    (__pPage_ref.hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                        as isize,
                ) as *mut crate::src::ext::rtree::rtree::U8_0,
                iTo as crate::src::ext::rtree::rtree::U32_0,
            );
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn relocatePage(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pDbPage: *mut crate::src::headers::btreeInt_h::MemPage,
    eType: crate::src::ext::rtree::rtree::U8_0,
    iPtrPage: crate::src::src::pager::Pgno,
    iFreePage: crate::src::src::pager::Pgno,
    isCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pPtrPage: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let __pDbPage_ref = unsafe { &mut *pDbPage };
    let iDbPage: crate::src::src::pager::Pgno = __pDbPage_ref.pgno;
    let pPager: *mut crate::src::src::pager::Pager = (*pBt).pPager;
    let mut rc: ::core::ffi::c_int;
    if iDbPage < 3 as crate::src::src::pager::Pgno {
        return crate::src::src::main::sqlite3CorruptError(3933 as ::core::ffi::c_int);
    }
    rc = crate::src::src::pager::sqlite3PagerMovepage(
        pPager,
        __pDbPage_ref.pDbPage as *mut crate::src::src::pcache::PgHdr,
        iFreePage,
        isCommit,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    __pDbPage_ref.pgno = iFreePage;
    if eType as ::core::ffi::c_int == crate::src::headers::btreeInt_h::PTRMAP_BTREE
        || eType as ::core::ffi::c_int == crate::src::headers::btreeInt_h::PTRMAP_ROOTPAGE
    {
        rc = setChildPtrmaps(pDbPage);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
    } else {
        let nextOvfl: crate::src::src::pager::Pgno =
            crate::src::src::util::sqlite3Get4byte(__pDbPage_ref.aData)
                as crate::src::src::pager::Pgno;
        if nextOvfl != 0 as crate::src::src::pager::Pgno {
            ptrmapPut(
                pBt,
                nextOvfl,
                crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW2
                    as crate::src::ext::rtree::rtree::U8_0,
                iFreePage,
                &raw mut rc,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
        }
    }
    if eType as ::core::ffi::c_int != crate::src::headers::btreeInt_h::PTRMAP_ROOTPAGE {
        rc = btreeGetPage(pBt, iPtrPage, &raw mut pPtrPage, 0 as ::core::ffi::c_int);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        rc = crate::src::src::pager::sqlite3PagerWrite(
            (*pPtrPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            releasePage(pPtrPage);
            return rc;
        }
        rc = modifyPagePointer(pPtrPage, iDbPage, iFreePage, eType);
        releasePage(pPtrPage);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            ptrmapPut(pBt, iFreePage, eType, iPtrPage, &raw mut rc);
        }
    }
    rc
}

unsafe extern "C" fn incrVacuumStep(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    nFin: crate::src::src::pager::Pgno,
    mut iLastPg: crate::src::src::pager::Pgno,
    bCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let nFreeList: crate::src::src::pager::Pgno;
    let mut rc: ::core::ffi::c_int;
    if (ptrmapPageno(pBt, iLastPg) != iLastPg)
        && iLastPg
            != (crate::src::src::global::sqlite3PendingByte as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_div((*pBt).pageSize)
                .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
    {
        let mut eType: crate::src::ext::rtree::rtree::U8_0 = 0;
        let mut iPtrPage: crate::src::src::pager::Pgno = 0;
        nFreeList =
            crate::src::src::util::sqlite3Get4byte((*(*pBt).pPage1).aData.offset(36_isize)
                as *mut crate::src::ext::rtree::rtree::U8_0)
                as crate::src::src::pager::Pgno;
        if nFreeList == 0 as crate::src::src::pager::Pgno {
            return crate::src::headers::sqlite3_h::SQLITE_DONE;
        }
        rc = ptrmapGet(pBt, iLastPg, &raw mut eType, &raw mut iPtrPage);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        if eType as ::core::ffi::c_int == crate::src::headers::btreeInt_h::PTRMAP_ROOTPAGE {
            return crate::src::src::main::sqlite3CorruptError(4031 as ::core::ffi::c_int);
        }
        if eType as ::core::ffi::c_int == crate::src::headers::btreeInt_h::PTRMAP_FREEPAGE {
            if bCommit == 0 as ::core::ffi::c_int {
                let mut iFreePg: crate::src::src::pager::Pgno = 0;
                let mut pFreePg: *mut crate::src::headers::btreeInt_h::MemPage =
                    ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                rc = allocateBtreePage(
                    pBt,
                    &raw mut pFreePg,
                    &raw mut iFreePg,
                    iLastPg,
                    BTALLOC_EXACT as crate::src::ext::rtree::rtree::U8_0,
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    return rc;
                }
                releasePage(pFreePg);
            }
        } else {
            let mut iFreePg_0: crate::src::src::pager::Pgno = 0;
            let mut pLastPg: *mut crate::src::headers::btreeInt_h::MemPage =
                ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
            let mut eMode: crate::src::ext::rtree::rtree::U8_0 =
                BTALLOC_ANY as crate::src::ext::rtree::rtree::U8_0;
            let mut iNear: crate::src::src::pager::Pgno = 0 as crate::src::src::pager::Pgno;
            rc = btreeGetPage(pBt, iLastPg, &raw mut pLastPg, 0 as ::core::ffi::c_int);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            if bCommit == 0 as ::core::ffi::c_int {
                eMode = BTALLOC_LE as crate::src::ext::rtree::rtree::U8_0;
                iNear = nFin;
            }
            loop {
                let mut pFreePg_0: *mut crate::src::headers::btreeInt_h::MemPage =
                    ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                let dbSize: crate::src::src::pager::Pgno = btreePagecount(pBt);
                rc = allocateBtreePage(pBt, &raw mut pFreePg_0, &raw mut iFreePg_0, iNear, eMode);
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    releasePage(pLastPg);
                    return rc;
                }
                releasePage(pFreePg_0);
                if iFreePg_0 > dbSize {
                    releasePage(pLastPg);
                    return crate::src::src::main::sqlite3CorruptError(4083 as ::core::ffi::c_int);
                }
                if !(bCommit != 0 && iFreePg_0 > nFin) {
                    break;
                }
            }
            rc = relocatePage(pBt, pLastPg, eType, iPtrPage, iFreePg_0, bCommit);
            releasePage(pLastPg);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
        }
    }
    if bCommit == 0 as ::core::ffi::c_int {
        loop {
            iLastPg = iLastPg.wrapping_sub(1);
            if !(iLastPg
                == (crate::src::src::global::sqlite3PendingByte
                    as crate::src::ext::rtree::rtree::U32_0)
                    .wrapping_div((*pBt).pageSize)
                    .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
                || ptrmapPageno(pBt, iLastPg) == iLastPg)
            {
                break;
            }
        }
        (*pBt).bDoTruncate = 1 as crate::src::ext::rtree::rtree::U8_0;
        (*pBt).nPage = iLastPg as crate::src::ext::rtree::rtree::U32_0;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn finalDbSize(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    nOrig: crate::src::src::pager::Pgno,
    nFree: crate::src::src::pager::Pgno,
) -> crate::src::src::pager::Pgno {
    
    
    let mut nFin: crate::src::src::pager::Pgno;
    let __pBt_ref = unsafe { &mut *pBt };
    let nEntry: ::core::ffi::c_int = __pBt_ref
        .usableSize
        .wrapping_div(5 as crate::src::ext::rtree::rtree::U32_0) as ::core::ffi::c_int;
    let nPtrmap: crate::src::src::pager::Pgno = nFree
        .wrapping_sub(nOrig)
        .wrapping_add(ptrmapPageno(pBt, nOrig))
        .wrapping_add(nEntry as crate::src::src::pager::Pgno)
        .wrapping_div(nEntry as crate::src::src::pager::Pgno);
    nFin = nOrig.wrapping_sub(nFree).wrapping_sub(nPtrmap);
    if nOrig
        > (crate::src::src::global::sqlite3PendingByte as crate::src::ext::rtree::rtree::U32_0)
            .wrapping_div(__pBt_ref.pageSize)
            .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
        && nFin
            < (crate::src::src::global::sqlite3PendingByte as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_div(__pBt_ref.pageSize)
                .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
    {
        nFin = nFin.wrapping_sub(1);
    }
    while ptrmapPageno(pBt, nFin) == nFin
        || nFin
            == (crate::src::src::global::sqlite3PendingByte as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_div(__pBt_ref.pageSize)
                .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
    {
        nFin = nFin.wrapping_sub(1);
    }
    nFin
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeIncrVacuum(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    if (*pBt).autoVacuum == 0 {
        rc = crate::src::headers::sqlite3_h::SQLITE_DONE;
    } else {
        let nOrig: crate::src::src::pager::Pgno = btreePagecount(pBt);
        let nFree: crate::src::src::pager::Pgno =
            crate::src::src::util::sqlite3Get4byte((*(*pBt).pPage1).aData.offset(36_isize)
                as *mut crate::src::ext::rtree::rtree::U8_0)
                as crate::src::src::pager::Pgno;
        let nFin: crate::src::src::pager::Pgno = finalDbSize(pBt, nOrig, nFree);
        if nOrig < nFin || nFree >= nOrig {
            rc = crate::src::src::main::sqlite3CorruptError(4151 as ::core::ffi::c_int);
        } else if nFree > 0 as crate::src::src::pager::Pgno {
            rc = saveAllCursors(
                pBt,
                0 as crate::src::src::pager::Pgno,
                ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>(),
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                invalidateAllOverflowCache(pBt);
                rc = incrVacuumStep(pBt, nFin, nOrig, 0 as ::core::ffi::c_int);
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let __pBt_ref = unsafe { &mut *pBt };
                rc = crate::src::src::pager::sqlite3PagerWrite(
                    (*__pBt_ref.pPage1).pDbPage as *mut crate::src::src::pcache::PgHdr,
                );
                crate::src::src::util::sqlite3Put4byte(
                    (*__pBt_ref.pPage1).aData.offset(28_isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0,
                    __pBt_ref.nPage,
                );
            }
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_DONE;
        }
    }
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}

unsafe extern "C" fn autoVacuumCommit(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    
    
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    let pPager: *mut crate::src::src::pager::Pager = (*pBt).pPager;
    invalidateAllOverflowCache(pBt);
    if (*pBt).incrVacuum == 0 {
        
        
        let mut nVac: crate::src::src::pager::Pgno;
        let mut iFree: crate::src::src::pager::Pgno;
        
        let nOrig: crate::src::src::pager::Pgno = btreePagecount(pBt);
        if ptrmapPageno(pBt, nOrig) == nOrig
            || nOrig
                == (crate::src::src::global::sqlite3PendingByte
                    as crate::src::ext::rtree::rtree::U32_0)
                    .wrapping_div((*pBt).pageSize)
                    .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
        {
            return crate::src::src::main::sqlite3CorruptError(4202 as ::core::ffi::c_int);
        }
        let nFree: crate::src::src::pager::Pgno = crate::src::src::util::sqlite3Get4byte((*(*pBt).pPage1).aData.offset(36_isize)
                as *mut crate::src::ext::rtree::rtree::U8_0)
                as crate::src::src::pager::Pgno;
        db = (*p).db;
        if (*db).xAutovacPages.is_some() {
            let mut iDb: ::core::ffi::c_int;
            iDb = 0 as ::core::ffi::c_int;
            let __db_ref = unsafe { &mut *db };
            while iDb < __db_ref.nDb {
                if (*__db_ref.aDb.offset(iDb as isize)).pBt == p {
                    break;
                }
                iDb += 1;
            }
            nVac = __db_ref.xAutovacPages.expect("non-null function pointer")(
                __db_ref.pAutovacPagesArg,
                (*__db_ref.aDb.offset(iDb as isize)).zDbSName,
                nOrig as crate::src::ext::rtree::rtree::U32_0,
                nFree as crate::src::ext::rtree::rtree::U32_0,
                (*pBt).pageSize,
            ) as crate::src::src::pager::Pgno;
            if nVac > nFree {
                nVac = nFree;
            }
            if nVac == 0 as crate::src::src::pager::Pgno {
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        } else {
            nVac = nFree;
        }
        let nFin: crate::src::src::pager::Pgno = finalDbSize(pBt, nOrig, nVac);
        if nFin > nOrig {
            return crate::src::src::main::sqlite3CorruptError(4229 as ::core::ffi::c_int);
        }
        if nFin < nOrig {
            rc = saveAllCursors(
                pBt,
                0 as crate::src::src::pager::Pgno,
                ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>(),
            );
        }
        iFree = nOrig;
        while iFree > nFin && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = incrVacuumStep(pBt, nFin, iFree, (nVac == nFree) as ::core::ffi::c_int);
            iFree = iFree.wrapping_sub(1);
        }
        if (rc == crate::src::headers::sqlite3_h::SQLITE_DONE
            || rc == crate::src::headers::sqlite3_h::SQLITE_OK)
            && nFree > 0 as crate::src::src::pager::Pgno
        {
            let __pBt_ref = unsafe { &mut *pBt };
            rc = crate::src::src::pager::sqlite3PagerWrite(
                (*__pBt_ref.pPage1).pDbPage as *mut crate::src::src::pcache::PgHdr,
            );
            if nVac == nFree {
                crate::src::src::util::sqlite3Put4byte(
                    (*__pBt_ref.pPage1).aData.offset(32_isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0,
                    0 as crate::src::ext::rtree::rtree::U32_0,
                );
                crate::src::src::util::sqlite3Put4byte(
                    (*__pBt_ref.pPage1).aData.offset(36_isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0,
                    0 as crate::src::ext::rtree::rtree::U32_0,
                );
            }
            crate::src::src::util::sqlite3Put4byte(
                (*__pBt_ref.pPage1).aData.offset(28_isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0,
                nFin as crate::src::ext::rtree::rtree::U32_0,
            );
            __pBt_ref.bDoTruncate = 1 as crate::src::ext::rtree::rtree::U8_0;
            __pBt_ref.nPage = nFin as crate::src::ext::rtree::rtree::U32_0;
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::pager::sqlite3PagerRollback(pPager);
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCommitPhaseOne(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    zSuperJrnl: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*p).inTrans as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_WRITE {
        let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
        crate::src::src::btmutex::sqlite3BtreeEnter(
            p as *mut crate::src::headers::btreeInt_h::Btree,
        );
        let __pBt_ref = unsafe { &*pBt };
        if __pBt_ref.autoVacuum != 0 {
            rc = autoVacuumCommit(p);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::btmutex::sqlite3BtreeLeave(
                    p as *mut crate::src::headers::btreeInt_h::Btree,
                );
                return rc;
            }
        }
        if __pBt_ref.bDoTruncate != 0 {
            crate::src::src::pager::sqlite3PagerTruncateImage(
                __pBt_ref.pPager,
                __pBt_ref.nPage as crate::src::src::pager::Pgno,
            );
        }
        rc = crate::src::src::pager::sqlite3PagerCommitPhaseOne(
            __pBt_ref.pPager,
            zSuperJrnl,
            0 as ::core::ffi::c_int,
        );
        crate::src::src::btmutex::sqlite3BtreeLeave(
            p as *mut crate::src::headers::btreeInt_h::Btree,
        );
    }
    rc
}

unsafe extern "C" fn btreeEndTransaction(p: *mut crate::src::headers::btreeInt_h::Btree) {
    let __p_ref = unsafe { &mut *p };
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __p_ref.pBt;
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __p_ref.db;
    (*pBt).bDoTruncate = 0 as crate::src::ext::rtree::rtree::U8_0;
    if __p_ref.inTrans as ::core::ffi::c_int > crate::src::headers::btreeInt_h::TRANS_NONE
        && (*db).nVdbeRead > 1 as ::core::ffi::c_int
    {
        downgradeAllSharedCacheTableLocks(p);
        __p_ref.inTrans =
            crate::src::headers::btreeInt_h::TRANS_READ as crate::src::ext::rtree::rtree::U8_0;
    } else {
        if __p_ref.inTrans as ::core::ffi::c_int != crate::src::headers::btreeInt_h::TRANS_NONE {
            clearAllSharedCacheTableLocks(p);
            (*pBt).nTransaction -= 1;
            if 0 as ::core::ffi::c_int == (*pBt).nTransaction {
                (*pBt).inTransaction = crate::src::headers::btreeInt_h::TRANS_NONE
                    as crate::src::ext::rtree::rtree::U8_0;
            }
        }
        __p_ref.inTrans =
            crate::src::headers::btreeInt_h::TRANS_NONE as crate::src::ext::rtree::rtree::U8_0;
        unlockBtreeIfUnused(pBt);
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCommitPhaseTwo(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    bCleanup: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*p).inTrans as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_NONE {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    if (*p).inTrans as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_WRITE {
        
        let __p_ref = unsafe { &mut *p };
        let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __p_ref.pBt;
        let rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerCommitPhaseTwo((*pBt).pPager);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK && bCleanup == 0 as ::core::ffi::c_int {
            crate::src::src::btmutex::sqlite3BtreeLeave(
                p as *mut crate::src::headers::btreeInt_h::Btree,
            );
            return rc;
        }
        __p_ref.iBDataVersion = __p_ref.iBDataVersion.wrapping_sub(1);
        (*pBt).inTransaction =
            crate::src::headers::btreeInt_h::TRANS_READ as crate::src::ext::rtree::rtree::U8_0;
        btreeClearHasContent(pBt);
    }
    btreeEndTransaction(p);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCommit(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc = sqlite3BtreeCommitPhaseOne(p, ::core::ptr::null::<::core::ffi::c_char>());
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3BtreeCommitPhaseTwo(p, 0 as ::core::ffi::c_int);
    }
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeTripAllCursors(
    pBtree: *mut crate::src::headers::btreeInt_h::Btree,
    errCode: ::core::ffi::c_int,
    writeOnly: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut crate::src::headers::btreeInt_h::BtCursor;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !pBtree.is_null() {
        crate::src::src::btmutex::sqlite3BtreeEnter(
            pBtree as *mut crate::src::headers::btreeInt_h::Btree,
        );
        p = (*(*pBtree).pBt).pCursor;
        while !p.is_null() {
            if writeOnly != 0
                && (*p).curFlags as ::core::ffi::c_int
                    & crate::src::headers::btreeInt_h::BTCF_WriteFlag
                    == 0 as ::core::ffi::c_int
            {
                if (*p).eState as ::core::ffi::c_int
                    == crate::src::headers::btreeInt_h::CURSOR_VALID
                    || (*p).eState as ::core::ffi::c_int
                        == crate::src::headers::btreeInt_h::CURSOR_SKIPNEXT
                {
                    rc = saveCursorPosition(p);
                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                        sqlite3BtreeTripAllCursors(pBtree, rc, 0 as ::core::ffi::c_int);
                        break;
                    }
                }
            } else {
                sqlite3BtreeClearCursor(p);
                (*p).eState = crate::src::headers::btreeInt_h::CURSOR_FAULT
                    as crate::src::ext::rtree::rtree::U8_0;
                (*p).skipNext = errCode;
            }
            btreeReleaseAllCursorPages(p);
            p = (*p).pNext;
        }
        crate::src::src::btmutex::sqlite3BtreeLeave(
            pBtree as *mut crate::src::headers::btreeInt_h::Btree,
        );
    }
    rc
}

unsafe extern "C" fn btreeSetNPage(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pPage1: *mut crate::src::headers::btreeInt_h::MemPage,
) {
    let mut nPage: ::core::ffi::c_int = crate::src::src::util::sqlite3Get4byte(
        (*pPage1).aData.offset(28_isize) as *mut crate::src::ext::rtree::rtree::U8_0,
    ) as ::core::ffi::c_int;
    if nPage == 0 as ::core::ffi::c_int {
        crate::src::src::pager::sqlite3PagerPagecount((*pBt).pPager, &raw mut nPage);
    }
    (*pBt).nPage = nPage as crate::src::ext::rtree::rtree::U32_0;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeRollback(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    mut tripCode: ::core::ffi::c_int,
    mut writeOnly: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    let mut pPage1: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    if tripCode == crate::src::headers::sqlite3_h::SQLITE_OK {
        tripCode = saveAllCursors(
            pBt,
            0 as crate::src::src::pager::Pgno,
            ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>(),
        );
        rc = tripCode;
        if rc != 0 {
            writeOnly = 0 as ::core::ffi::c_int;
        }
    } else {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if tripCode != 0 {
        let rc2: ::core::ffi::c_int = sqlite3BtreeTripAllCursors(p, tripCode, writeOnly);
        if rc2 != crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    if (*p).inTrans as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_WRITE {
        
        let rc2_0: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerRollback((*pBt).pPager);
        if rc2_0 != crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2_0;
        }
        if btreeGetPage(
            pBt,
            1 as crate::src::src::pager::Pgno,
            &raw mut pPage1,
            0 as ::core::ffi::c_int,
        ) == crate::src::headers::sqlite3_h::SQLITE_OK
        {
            btreeSetNPage(pBt, pPage1);
            releasePageOne(pPage1);
        }
        (*pBt).inTransaction =
            crate::src::headers::btreeInt_h::TRANS_READ as crate::src::ext::rtree::rtree::U8_0;
        btreeClearHasContent(pBt);
    }
    btreeEndTransaction(p);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeBeginStmt(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    iStatement: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerOpenSavepoint((*pBt).pPager, iStatement);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSavepoint(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    op: ::core::ffi::c_int,
    iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !p.is_null()
        && (*p).inTrans as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_WRITE
    {
        let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
        crate::src::src::btmutex::sqlite3BtreeEnter(
            p as *mut crate::src::headers::btreeInt_h::Btree,
        );
        if op == crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK {
            rc = saveAllCursors(
                pBt,
                0 as crate::src::src::pager::Pgno,
                ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>(),
            );
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = crate::src::src::pager::sqlite3PagerSavepoint((*pBt).pPager, op, iSavepoint);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if iSavepoint < 0 as ::core::ffi::c_int
                && (*pBt).btsFlags as ::core::ffi::c_int
                    & crate::src::headers::btreeInt_h::BTS_INITIALLY_EMPTY
                    != 0 as ::core::ffi::c_int
            {
                (*pBt).nPage = 0 as crate::src::ext::rtree::rtree::U32_0;
            }
            rc = newDatabase(pBt);
            btreeSetNPage(pBt, (*pBt).pPage1);
        }
        crate::src::src::btmutex::sqlite3BtreeLeave(
            p as *mut crate::src::headers::btreeInt_h::Btree,
        );
    }
    rc
}

unsafe extern "C" fn btreeCursor(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    mut iTable: crate::src::src::pager::Pgno,
    wrFlag: ::core::ffi::c_int,
    pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo,
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    let mut pX: *mut crate::src::headers::btreeInt_h::BtCursor;
    if iTable <= 1 as crate::src::src::pager::Pgno {
        if iTable < 1 as crate::src::src::pager::Pgno {
            return crate::src::src::main::sqlite3CorruptError(4693 as ::core::ffi::c_int);
        } else if btreePagecount(pBt) == 0 as crate::src::src::pager::Pgno {
            iTable = 0 as crate::src::src::pager::Pgno;
        }
    }
    let __pCur_ref = unsafe { &mut *pCur };
    __pCur_ref.pgnoRoot = iTable;
    __pCur_ref.iPage = -(1 as ::core::ffi::c_int) as crate::src::headers::sqliteInt_h::I8_0;
    __pCur_ref.pKeyInfo = pKeyInfo;
    __pCur_ref.pBtree = p;
    __pCur_ref.pBt = pBt;
    __pCur_ref.curFlags = 0 as crate::src::ext::rtree::rtree::U8_0;
    let __pBt_ref = unsafe { &mut *pBt };
    pX = __pBt_ref.pCursor;
    while !pX.is_null() {
        if (*pX).pgnoRoot == iTable {
            (*pX).curFlags = ((*pX).curFlags as ::core::ffi::c_int
                | crate::src::headers::btreeInt_h::BTCF_Multiple)
                as crate::src::ext::rtree::rtree::U8_0;
            __pCur_ref.curFlags = crate::src::headers::btreeInt_h::BTCF_Multiple
                as crate::src::ext::rtree::rtree::U8_0;
        }
        pX = (*pX).pNext;
    }
    __pCur_ref.eState =
        crate::src::headers::btreeInt_h::CURSOR_INVALID as crate::src::ext::rtree::rtree::U8_0;
    __pCur_ref.pNext = __pBt_ref.pCursor;
    __pBt_ref.pCursor = pCur;
    if wrFlag != 0 {
        __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
            | crate::src::headers::btreeInt_h::BTCF_WriteFlag)
            as crate::src::ext::rtree::rtree::U8_0;
        __pCur_ref.curPagerFlags = 0 as crate::src::ext::rtree::rtree::U8_0;
        if __pBt_ref.pTmpSpace.is_null() {
            return allocateTempSpace(pBt);
        }
    } else {
        __pCur_ref.curPagerFlags =
            crate::src::src::pager::PAGER_GET_READONLY as crate::src::ext::rtree::rtree::U8_0;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn btreeCursorWithLock(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    iTable: crate::src::src::pager::Pgno,
    wrFlag: ::core::ffi::c_int,
    pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo,
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let rc: ::core::ffi::c_int = btreeCursor(p, iTable, wrFlag, pKeyInfo, pCur);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCursor(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    iTable: crate::src::src::pager::Pgno,
    wrFlag: ::core::ffi::c_int,
    pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo,
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    if (*p).sharable != 0 {
        btreeCursorWithLock(p, iTable, wrFlag, pKeyInfo, pCur)
    } else {
        btreeCursor(p, iTable, wrFlag, pKeyInfo, pCur)
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCursorSize() -> ::core::ffi::c_int {
    ((::core::mem::size_of::<crate::src::headers::btreeInt_h::BtCursor>() as usize)
        .wrapping_add(7_usize)
        & !(7 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCursorZero(
    p: *mut crate::src::headers::btreeInt_h::BtCursor,
) {
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        32 as crate::__stddef_size_t_h::SizeT,
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCloseCursor(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let pBtree: *mut crate::src::headers::btreeInt_h::Btree = (*pCur).pBtree;
    if !pBtree.is_null() {
        let __pCur_ref = unsafe { &mut *pCur };
        let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __pCur_ref.pBt;
        crate::src::src::btmutex::sqlite3BtreeEnter(
            pBtree as *mut crate::src::headers::btreeInt_h::Btree,
        );
        let __pBt_ref = unsafe { &mut *pBt };
        if __pBt_ref.pCursor == pCur {
            __pBt_ref.pCursor = __pCur_ref.pNext;
        } else {
            let mut pPrev: *mut crate::src::headers::btreeInt_h::BtCursor = __pBt_ref.pCursor;
            loop {
                if (*pPrev).pNext == pCur {
                    (*pPrev).pNext = __pCur_ref.pNext;
                    break;
                } else {
                    pPrev = (*pPrev).pNext;
                    if pPrev.is_null() {
                        break;
                    }
                }
            }
        }
        btreeReleaseAllCursorPages(pCur);
        unlockBtreeIfUnused(pBt);
        crate::src::src::malloc::sqlite3_free(__pCur_ref.aOverflow as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(__pCur_ref.pKey);
        if __pBt_ref.openFlags as ::core::ffi::c_int & crate::src::src::btree::BTREE_SINGLE != 0
            && __pBt_ref.pCursor.is_null()
        {
            sqlite3BtreeClose(pBtree);
        } else {
            crate::src::src::btmutex::sqlite3BtreeLeave(
                pBtree as *mut crate::src::headers::btreeInt_h::Btree,
            );
        }
        __pCur_ref.pBtree = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[inline(never)]
unsafe extern "C" fn getCellInfo(pCur: *mut crate::src::headers::btreeInt_h::BtCursor) {
    if (*pCur).info.nSize as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let __pCur_ref = unsafe { &mut *pCur };
        __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
            | crate::src::headers::btreeInt_h::BTCF_ValidNKey)
            as crate::src::ext::rtree::rtree::U8_0;
        btreeParseCell(
            __pCur_ref.pPage,
            __pCur_ref.ix as ::core::ffi::c_int,
            &raw mut __pCur_ref.info,
        );
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCursorIsValidNN(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    ((*pCur).eState as ::core::ffi::c_int == crate::src::headers::btreeInt_h::CURSOR_VALID)
        as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeIntegerKey(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> crate::src::ext::rtree::rtree::I64_0 {
    getCellInfo(pCur);
    (*pCur).info.nKey
}
pub unsafe extern "C" fn sqlite3BtreeCursorPin(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) {
    (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
        | crate::src::headers::btreeInt_h::BTCF_Pinned)
        as crate::src::ext::rtree::rtree::U8_0;
}
pub unsafe extern "C" fn sqlite3BtreeCursorUnpin(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) {
    (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
        & !crate::src::headers::btreeInt_h::BTCF_Pinned)
        as crate::src::ext::rtree::rtree::U8_0;
}
pub unsafe extern "C" fn sqlite3BtreeOffset(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> crate::src::ext::rtree::rtree::I64_0 {
    getCellInfo(pCur);
    let __pCur_ref = unsafe { &mut *pCur };
    (*__pCur_ref.pBt).pageSize as crate::src::ext::rtree::rtree::I64_0
        * ((*__pCur_ref.pPage).pgno as crate::src::ext::rtree::rtree::I64_0
            - 1 as crate::src::ext::rtree::rtree::I64_0)
        + __pCur_ref
            .info
            .pPayload
            .offset_from((*__pCur_ref.pPage).aData) as ::core::ffi::c_long
            as crate::src::ext::rtree::rtree::I64_0
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreePayloadSize(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> crate::src::ext::rtree::rtree::U32_0 {
    getCellInfo(pCur);
    (*pCur).info.nPayload
}
pub unsafe extern "C" fn sqlite3BtreeMaxRecordSize(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> crate::src::headers::sqlite3_h::Sqlite3Int64 {
    let __pBt_ref = &*(*pCur).pBt;
    __pBt_ref.pageSize as crate::src::headers::sqlite3_h::Sqlite3Int64
        * __pBt_ref.nPage as crate::src::headers::sqlite3_h::Sqlite3Int64
}

unsafe extern "C" fn getOverflowPage(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    ovfl: crate::src::src::pager::Pgno,
    ppPage: *mut *mut crate::src::headers::btreeInt_h::MemPage,
    pPgnoNext: *mut crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    let mut next: crate::src::src::pager::Pgno = 0 as crate::src::src::pager::Pgno;
    let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pBt).autoVacuum != 0 {
        let mut pgno: crate::src::src::pager::Pgno = 0;
        let mut iGuess: crate::src::src::pager::Pgno =
            ovfl.wrapping_add(1 as crate::src::src::pager::Pgno);
        let mut eType: crate::src::ext::rtree::rtree::U8_0 = 0;
        while ptrmapPageno(pBt, iGuess) == iGuess
            || iGuess
                == (crate::src::src::global::sqlite3PendingByte
                    as crate::src::ext::rtree::rtree::U32_0)
                    .wrapping_div((*pBt).pageSize)
                    .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
        {
            iGuess = iGuess.wrapping_add(1);
        }
        if iGuess <= btreePagecount(pBt) {
            rc = ptrmapGet(pBt, iGuess, &raw mut eType, &raw mut pgno);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                && eType as ::core::ffi::c_int == crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW2
                && pgno == ovfl
            {
                next = iGuess;
                rc = crate::src::headers::sqlite3_h::SQLITE_DONE;
            }
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = btreeGetPage(
            pBt,
            ovfl,
            &raw mut pPage,
            if ppPage.is_null() {
                crate::src::src::pager::PAGER_GET_READONLY
            } else {
                0 as ::core::ffi::c_int
            },
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            next = crate::src::src::util::sqlite3Get4byte((*pPage).aData)
                as crate::src::src::pager::Pgno;
        }
    }
    *pPgnoNext = next;
    if !ppPage.is_null() {
        *ppPage = pPage;
    } else {
        releasePage(pPage);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
        crate::src::headers::sqlite3_h::SQLITE_OK
    } else {
        rc
    }
}

unsafe extern "C" fn copyPayload(
    pPayload: *mut ::core::ffi::c_void,
    pBuf: *mut ::core::ffi::c_void,
    nByte: ::core::ffi::c_int,
    eOp: ::core::ffi::c_int,
    pDbPage: *mut crate::src::src::pager::DbPage,
) -> ::core::ffi::c_int {
    if eOp != 0 {
        let rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerWrite(
            pDbPage as *mut crate::src::src::pcache::PgHdr,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        ::libc::memcpy(pPayload, pBuf, nByte as crate::__stddef_size_t_h::SizeT);
    } else {
        ::libc::memcpy(pBuf, pPayload, nByte as crate::__stddef_size_t_h::SizeT);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn accessPayload(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    mut offset: crate::src::ext::rtree::rtree::U32_0,
    mut amt: crate::src::ext::rtree::rtree::U32_0,
    mut pBuf: *mut ::core::ffi::c_uchar,
    eOp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut aPayload: *mut ::core::ffi::c_uchar;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let __pCur_ref = unsafe { &mut *pCur };
    let pPage: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __pCur_ref.pBt;
    let pBufStart: *mut ::core::ffi::c_uchar = pBuf;
    if __pCur_ref.ix as ::core::ffi::c_int >= (*pPage).nCell as ::core::ffi::c_int {
        return crate::src::src::main::sqlite3CorruptError(5117 as ::core::ffi::c_int);
    }
    getCellInfo(pCur);
    aPayload = __pCur_ref.info.pPayload as *mut ::core::ffi::c_uchar;
    if aPayload.offset_from((*pPage).aData) as ::core::ffi::c_long
        as crate::src::headers::sqliteInt_h::Uptr
        > (*pBt)
            .usableSize
            .wrapping_sub(__pCur_ref.info.nLocal as crate::src::ext::rtree::rtree::U32_0)
            as crate::src::headers::sqliteInt_h::Uptr
    {
        return crate::src::src::main::sqlite3CorruptError(5132 as ::core::ffi::c_int);
    }
    if offset < __pCur_ref.info.nLocal as crate::src::ext::rtree::rtree::U32_0 {
        let mut a: ::core::ffi::c_int = amt as ::core::ffi::c_int;
        if (a as crate::src::ext::rtree::rtree::U32_0).wrapping_add(offset)
            > __pCur_ref.info.nLocal as crate::src::ext::rtree::rtree::U32_0
        {
            a = (__pCur_ref.info.nLocal as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_sub(offset) as ::core::ffi::c_int;
        }
        rc = copyPayload(
            aPayload.offset(offset as isize) as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_void,
            pBuf as *mut ::core::ffi::c_void,
            a,
            eOp,
            (*pPage).pDbPage,
        );
        offset = 0 as crate::src::ext::rtree::rtree::U32_0;
        pBuf = pBuf.offset(a as isize);
        amt = amt.wrapping_sub(a as crate::src::ext::rtree::rtree::U32_0);
    } else {
        offset =
            offset.wrapping_sub(__pCur_ref.info.nLocal as crate::src::ext::rtree::rtree::U32_0);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && amt > 0 as crate::src::ext::rtree::rtree::U32_0
    {
        let ovflSize: crate::src::ext::rtree::rtree::U32_0 = (*pBt)
            .usableSize
            .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0);
        let mut nextPage: crate::src::src::pager::Pgno;
        nextPage = crate::src::src::util::sqlite3Get4byte(
            aPayload.offset(__pCur_ref.info.nLocal as isize) as *mut ::core::ffi::c_uchar,
        ) as crate::src::src::pager::Pgno;
        if __pCur_ref.curFlags as ::core::ffi::c_int
            & crate::src::headers::btreeInt_h::BTCF_ValidOvfl
            == 0 as ::core::ffi::c_int
        {
            let nOvfl: ::core::ffi::c_int = (*pCur)
                .info
                .nPayload
                .wrapping_sub(__pCur_ref.info.nLocal as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_add(ovflSize)
                .wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_div(ovflSize)
                as ::core::ffi::c_int;
            if __pCur_ref.aOverflow.is_null()
                || nOvfl
                    * ::core::mem::size_of::<crate::src::src::pager::Pgno>() as ::core::ffi::c_int
                    > crate::src::src::malloc::sqlite3MallocSize(
                        __pCur_ref.aOverflow as *const ::core::ffi::c_void,
                    )
            {
                let aNew: *mut crate::src::src::pager::Pgno;
                if crate::src::src::util::sqlite3FaultSim(413 as ::core::ffi::c_int) != 0 {
                    aNew = ::core::ptr::null_mut::<crate::src::src::pager::Pgno>();
                } else {
                    aNew = crate::src::src::malloc::sqlite3Realloc(
                        __pCur_ref.aOverflow as *mut ::core::ffi::c_void,
                        ((nOvfl * 2 as ::core::ffi::c_int) as usize).wrapping_mul(
                            ::core::mem::size_of::<crate::src::src::pager::Pgno>() as usize,
                        ) as crate::src::ext::rtree::rtree::U64_0,
                    ) as *mut crate::src::src::pager::Pgno;
                }
                if aNew.is_null() {
                    return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                } else {
                    __pCur_ref.aOverflow = aNew;
                }
            }
            ::libc::memset(
                __pCur_ref.aOverflow as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (nOvfl as crate::__stddef_size_t_h::SizeT)
                    .wrapping_mul(::core::mem::size_of::<crate::src::src::pager::Pgno>()
                        as crate::__stddef_size_t_h::SizeT),
            );
            __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
                | crate::src::headers::btreeInt_h::BTCF_ValidOvfl)
                as crate::src::ext::rtree::rtree::U8_0;
        } else if *(*pCur)
            .aOverflow
            .offset(offset.wrapping_div(ovflSize) as isize)
            != 0
        {
            iIdx = offset.wrapping_div(ovflSize) as ::core::ffi::c_int;
            nextPage = *__pCur_ref.aOverflow.offset(iIdx as isize);
            offset = offset.wrapping_rem(ovflSize);
        }
        while nextPage != 0 {
            if nextPage > (*pBt).nPage {
                return crate::src::src::main::sqlite3CorruptError(5203 as ::core::ffi::c_int);
            }
            *__pCur_ref.aOverflow.offset(iIdx as isize) = nextPage;
            if offset >= ovflSize {
                if *(*pCur)
                    .aOverflow
                    .offset((iIdx + 1 as ::core::ffi::c_int) as isize)
                    != 0
                {
                    nextPage = *(*pCur)
                        .aOverflow
                        .offset((iIdx + 1 as ::core::ffi::c_int) as isize);
                } else {
                    rc = getOverflowPage(
                        pBt,
                        nextPage,
                        ::core::ptr::null_mut::<*mut crate::src::headers::btreeInt_h::MemPage>(),
                        &raw mut nextPage,
                    );
                }
                offset = offset.wrapping_sub(ovflSize);
            } else {
                let mut a_0: ::core::ffi::c_int = amt as ::core::ffi::c_int;
                if (a_0 as crate::src::ext::rtree::rtree::U32_0).wrapping_add(offset) > ovflSize {
                    a_0 = ovflSize.wrapping_sub(offset) as ::core::ffi::c_int;
                }
                if eOp == 0 as ::core::ffi::c_int
                    && offset == 0 as crate::src::ext::rtree::rtree::U32_0
                    && crate::src::src::pager::sqlite3PagerDirectReadOk((*pBt).pPager, nextPage)
                        != 0
                    && pBuf.offset(-(4 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_uchar
                        >= pBufStart
                {
                    let fd: *mut crate::src::headers::sqlite3_h::sqlite3_file =
                        crate::src::src::pager::sqlite3PagerFile((*pBt).pPager)
                            as *mut crate::src::headers::sqlite3_h::sqlite3_file;
                    let mut aSave: [crate::src::ext::rtree::rtree::U8_0; 4] = [0; 4];
                    let aWrite: *mut crate::src::ext::rtree::rtree::U8_0 = pBuf
                        .offset(-(4 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0;
                    ::core::ptr::copy_nonoverlapping(
                        aWrite as *const u8,
                        &raw mut aSave as *mut crate::src::ext::rtree::rtree::U8_0 as *mut u8,
                        4_usize,
                    );
                    rc = crate::src::src::os::sqlite3OsRead(
                        fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                        aWrite as *mut ::core::ffi::c_void,
                        a_0 + 4 as ::core::ffi::c_int,
                        (*pBt).pageSize as crate::src::ext::rtree::rtree::I64_0
                            * nextPage.wrapping_sub(1 as crate::src::src::pager::Pgno)
                                as crate::src::ext::rtree::rtree::I64_0,
                    );
                    nextPage = crate::src::src::util::sqlite3Get4byte(aWrite)
                        as crate::src::src::pager::Pgno;
                    ::core::ptr::copy_nonoverlapping(
                        &raw mut aSave as *mut crate::src::ext::rtree::rtree::U8_0 as *const u8,
                        aWrite as *mut u8,
                        4_usize,
                    );
                } else {
                    let mut pDbPage: *mut crate::src::src::pager::DbPage =
                        ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
                    rc = crate::src::src::pager::sqlite3PagerGet(
                        (*pBt).pPager,
                        nextPage,
                        &raw mut pDbPage as *mut _ as *mut *mut crate::src::src::pcache::PgHdr,
                        if eOp == 0 as ::core::ffi::c_int {
                            crate::src::src::pager::PAGER_GET_READONLY
                        } else {
                            0 as ::core::ffi::c_int
                        },
                    );
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        aPayload = crate::src::src::pager::sqlite3PagerGetData(
                            pDbPage as *mut crate::src::src::pcache::PgHdr,
                        ) as *mut ::core::ffi::c_uchar;
                        nextPage = crate::src::src::util::sqlite3Get4byte(aPayload)
                            as crate::src::src::pager::Pgno;
                        rc = copyPayload(
                            aPayload.offset(
                                offset.wrapping_add(4 as crate::src::ext::rtree::rtree::U32_0)
                                    as isize,
                            ) as *mut ::core::ffi::c_uchar
                                as *mut ::core::ffi::c_void,
                            pBuf as *mut ::core::ffi::c_void,
                            a_0,
                            eOp,
                            pDbPage,
                        );
                        crate::src::src::pager::sqlite3PagerUnref(
                            pDbPage as *mut crate::src::src::pcache::PgHdr,
                        );
                        offset = 0 as crate::src::ext::rtree::rtree::U32_0;
                    }
                }
                amt = amt.wrapping_sub(a_0 as crate::src::ext::rtree::rtree::U32_0);
                if amt == 0 as crate::src::ext::rtree::rtree::U32_0 {
                    return rc;
                }
                pBuf = pBuf.offset(a_0 as isize);
            }
            if rc != 0 {
                break;
            }
            iIdx += 1;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && amt > 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return crate::src::src::main::sqlite3CorruptError(5287 as ::core::ffi::c_int);
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreePayload(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    offset: crate::src::ext::rtree::rtree::U32_0,
    amt: crate::src::ext::rtree::rtree::U32_0,
    pBuf: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    accessPayload(
        pCur,
        offset,
        amt,
        pBuf as *mut ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int,
    )
}
#[inline(never)]
unsafe extern "C" fn accessPayloadChecked(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    offset: crate::src::ext::rtree::rtree::U32_0,
    amt: crate::src::ext::rtree::rtree::U32_0,
    pBuf: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    
    if (*pCur).eState as ::core::ffi::c_int == crate::src::headers::btreeInt_h::CURSOR_INVALID {
        return crate::src::headers::sqlite3_h::SQLITE_ABORT;
    }
    let rc: ::core::ffi::c_int = btreeRestoreCursorPosition(pCur);
    if rc != 0 {
        rc
    } else {
        accessPayload(
            pCur,
            offset,
            amt,
            pBuf as *mut ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int,
        )
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreePayloadChecked(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    offset: crate::src::ext::rtree::rtree::U32_0,
    amt: crate::src::ext::rtree::rtree::U32_0,
    pBuf: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if (*pCur).eState as ::core::ffi::c_int == crate::src::headers::btreeInt_h::CURSOR_VALID {
        accessPayload(
            pCur,
            offset,
            amt,
            pBuf as *mut ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int,
        )
    } else {
        accessPayloadChecked(pCur, offset, amt, pBuf)
    }
}

unsafe extern "C" fn fetchPayload(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pAmt: *mut crate::src::ext::rtree::rtree::U32_0,
) -> *const ::core::ffi::c_void {
    let mut amt: ::core::ffi::c_int;
    let __pCur_ref = unsafe { &mut *pCur };
    amt = __pCur_ref.info.nLocal as ::core::ffi::c_int;
    if amt
        > (*__pCur_ref.pPage)
            .aDataEnd
            .offset_from(__pCur_ref.info.pPayload) as ::core::ffi::c_long
            as ::core::ffi::c_int
    {
        amt = if 0 as ::core::ffi::c_int
            > (*__pCur_ref.pPage)
                .aDataEnd
                .offset_from(__pCur_ref.info.pPayload) as ::core::ffi::c_long
                as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            (*__pCur_ref.pPage)
                .aDataEnd
                .offset_from(__pCur_ref.info.pPayload) as ::core::ffi::c_long
                as ::core::ffi::c_int
        };
    }
    *pAmt = amt as crate::src::ext::rtree::rtree::U32_0;
    __pCur_ref.info.pPayload as *mut ::core::ffi::c_void
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreePayloadFetch(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pAmt: *mut crate::src::ext::rtree::rtree::U32_0,
) -> *const ::core::ffi::c_void {
    fetchPayload(pCur, pAmt)
}

unsafe extern "C" fn moveToChild(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    newPgno: crate::src::ext::rtree::rtree::U32_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let __pCur_ref = unsafe { &mut *pCur };
    if __pCur_ref.iPage as ::core::ffi::c_int
        >= crate::src::headers::btreeInt_h::BTCURSOR_MAX_DEPTH - 1 as ::core::ffi::c_int
    {
        return crate::src::src::main::sqlite3CorruptError(5425 as ::core::ffi::c_int);
    }
    __pCur_ref.info.nSize = 0 as crate::src::fts5::U16_0;
    __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
        & !(crate::src::headers::btreeInt_h::BTCF_ValidNKey
            | crate::src::headers::btreeInt_h::BTCF_ValidOvfl))
        as crate::src::ext::rtree::rtree::U8_0;
    __pCur_ref.aiIdx[__pCur_ref.iPage as usize] = __pCur_ref.ix;
    __pCur_ref.apPage[__pCur_ref.iPage as usize] = __pCur_ref.pPage;
    __pCur_ref.ix = 0 as crate::src::fts5::U16_0;
    __pCur_ref.iPage += 1;
    rc = getAndInitPage(
        __pCur_ref.pBt,
        newPgno as crate::src::src::pager::Pgno,
        &raw mut __pCur_ref.pPage,
        __pCur_ref.curPagerFlags as ::core::ffi::c_int,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && (((*__pCur_ref.pPage).nCell as ::core::ffi::c_int) < 1 as ::core::ffi::c_int
            || (*__pCur_ref.pPage).intKey as ::core::ffi::c_int
                != __pCur_ref.curIntKey as ::core::ffi::c_int)
    {
        releasePage(__pCur_ref.pPage);
        rc = crate::src::src::main::sqlite3CorruptError(5439 as ::core::ffi::c_int);
    }
    if rc != 0 {
        __pCur_ref.iPage -= 1;
        __pCur_ref.pPage = __pCur_ref.apPage[__pCur_ref.iPage as usize];
    }
    rc
}

unsafe extern "C" fn moveToParent(pCur: *mut crate::src::headers::btreeInt_h::BtCursor) {
    
    let __pCur_ref = unsafe { &mut *pCur };
    __pCur_ref.info.nSize = 0 as crate::src::fts5::U16_0;
    __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
        & !(crate::src::headers::btreeInt_h::BTCF_ValidNKey
            | crate::src::headers::btreeInt_h::BTCF_ValidOvfl))
        as crate::src::ext::rtree::rtree::U8_0;
    __pCur_ref.ix = __pCur_ref.aiIdx
        [(__pCur_ref.iPage as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize];
    let pLeaf: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
    __pCur_ref.iPage -= 1;
    __pCur_ref.pPage = __pCur_ref.apPage[__pCur_ref.iPage as usize];
    releasePageNotNull(pLeaf);
}

unsafe extern "C" fn moveToRoot(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let current_block: u64;
    let mut pRoot: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pCur_ref = unsafe { &mut *pCur };
    if __pCur_ref.iPage as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        if __pCur_ref.iPage != 0 {
            releasePageNotNull(__pCur_ref.pPage);
            loop {
                __pCur_ref.iPage -= 1;
                if (__pCur_ref.iPage == 0) {
                    break;
                }
                releasePageNotNull(__pCur_ref.apPage[__pCur_ref.iPage as usize]);
            }
            __pCur_ref.pPage = __pCur_ref.apPage[0 as ::core::ffi::c_int as usize];
            pRoot = __pCur_ref.pPage;
            current_block = 10084261848283180480;
        } else {
            current_block = 17478428563724192186;
        }
    } else {
        if __pCur_ref.pgnoRoot == 0 as crate::src::src::pager::Pgno {
            __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_INVALID
                as crate::src::ext::rtree::rtree::U8_0;
            return crate::src::headers::sqlite3_h::SQLITE_EMPTY;
        } else {
            if __pCur_ref.eState as ::core::ffi::c_int
                >= crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK
            {
                if __pCur_ref.eState as ::core::ffi::c_int
                    == crate::src::headers::btreeInt_h::CURSOR_FAULT
                {
                    return __pCur_ref.skipNext;
                }
                sqlite3BtreeClearCursor(pCur);
            }
            rc = getAndInitPage(
                __pCur_ref.pBt,
                __pCur_ref.pgnoRoot,
                &raw mut __pCur_ref.pPage,
                __pCur_ref.curPagerFlags as ::core::ffi::c_int,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_INVALID
                    as crate::src::ext::rtree::rtree::U8_0;
                return rc;
            }
            __pCur_ref.iPage = 0 as crate::src::headers::sqliteInt_h::I8_0;
            __pCur_ref.curIntKey = (*__pCur_ref.pPage).intKey;
        }
        current_block = 17478428563724192186;
    }
    if current_block == 17478428563724192186 {
        pRoot = __pCur_ref.pPage;
        if (*pRoot).isInit as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (__pCur_ref.pKeyInfo
                == ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::KeyInfo>())
                as ::core::ffi::c_int
                != (*pRoot).intKey as ::core::ffi::c_int
        {
            return crate::src::src::main::sqlite3CorruptError(5574 as ::core::ffi::c_int);
        }
    }
    __pCur_ref.ix = 0 as crate::src::fts5::U16_0;
    __pCur_ref.info.nSize = 0 as crate::src::fts5::U16_0;
    __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
        & !(crate::src::headers::btreeInt_h::BTCF_AtLast
            | crate::src::headers::btreeInt_h::BTCF_ValidNKey
            | crate::src::headers::btreeInt_h::BTCF_ValidOvfl))
        as crate::src::ext::rtree::rtree::U8_0;
    if (*pRoot).nCell as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        __pCur_ref.eState =
            crate::src::headers::btreeInt_h::CURSOR_VALID as crate::src::ext::rtree::rtree::U8_0;
    } else if (*pRoot).leaf == 0 {
        
        let __pRoot_ref = unsafe { &mut *pRoot };
        if __pRoot_ref.pgno != 1 as crate::src::src::pager::Pgno {
            return crate::src::src::main::sqlite3CorruptError(5586 as ::core::ffi::c_int);
        }
        let subpage: crate::src::src::pager::Pgno = crate::src::src::util::sqlite3Get4byte(__pRoot_ref.aData.offset(
            (__pRoot_ref.hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
        )
            as *mut crate::src::ext::rtree::rtree::U8_0)
            as crate::src::src::pager::Pgno;
        __pCur_ref.eState =
            crate::src::headers::btreeInt_h::CURSOR_VALID as crate::src::ext::rtree::rtree::U8_0;
        rc = moveToChild(pCur, subpage as crate::src::ext::rtree::rtree::U32_0);
    } else {
        __pCur_ref.eState =
            crate::src::headers::btreeInt_h::CURSOR_INVALID as crate::src::ext::rtree::rtree::U8_0;
        rc = crate::src::headers::sqlite3_h::SQLITE_EMPTY;
    }
    rc
}

unsafe extern "C" fn moveToLeftmost(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let mut pgno: crate::src::src::pager::Pgno;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && {
        pPage = (*pCur).pPage;
        (*pPage).leaf == 0
    } {
        let __pPage_ref = unsafe { &mut *pPage };
        pgno = crate::src::src::util::sqlite3Get4byte(
            __pPage_ref.aData.offset(
                (__pPage_ref.maskPage as ::core::ffi::c_int
                    & ((*(__pPage_ref.aCellIdx.offset(
                        (2 as ::core::ffi::c_int * (*pCur).ix as ::core::ffi::c_int) as isize,
                    ) as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(0_isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(__pPage_ref.aCellIdx.offset(
                            (2 as ::core::ffi::c_int * (*pCur).ix as ::core::ffi::c_int) as isize,
                        ) as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(1_isize) as ::core::ffi::c_int)) as isize,
            ),
        ) as crate::src::src::pager::Pgno;
        rc = moveToChild(pCur, pgno as crate::src::ext::rtree::rtree::U32_0);
    }
    rc
}

unsafe extern "C" fn moveToRightmost(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let mut pgno: crate::src::src::pager::Pgno;
    let mut rc: ::core::ffi::c_int;
    let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage;
    loop {
        pPage = (*pCur).pPage;
        if ((*pPage).leaf != 0) {
            break;
        }
        pgno = crate::src::src::util::sqlite3Get4byte(
            (*pPage).aData.offset(
                ((*pPage).hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::ext::rtree::rtree::U8_0,
        ) as crate::src::src::pager::Pgno;
        (*pCur).ix = (*pPage).nCell;
        rc = moveToChild(pCur, pgno as crate::src::ext::rtree::rtree::U32_0);
        if rc != 0 {
            return rc;
        }
    }
    (*pCur).ix =
        ((*pPage).nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::src::fts5::U16_0;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeFirst(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    rc = moveToRoot(pCur);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        *pRes = 0 as ::core::ffi::c_int;
        rc = moveToLeftmost(pCur);
    } else if rc == crate::src::headers::sqlite3_h::SQLITE_EMPTY {
        *pRes = 1 as ::core::ffi::c_int;
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeIsEmpty(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    if (*pCur).eState as ::core::ffi::c_int == crate::src::headers::btreeInt_h::CURSOR_VALID {
        *pRes = 0 as ::core::ffi::c_int;
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc = moveToRoot(pCur);
    if rc == crate::src::headers::sqlite3_h::SQLITE_EMPTY {
        *pRes = 1 as ::core::ffi::c_int;
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    } else {
        *pRes = 0 as ::core::ffi::c_int;
    }
    rc
}
#[inline(never)]
unsafe extern "C" fn btreeLast(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = moveToRoot(pCur);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        *pRes = 0 as ::core::ffi::c_int;
        rc = moveToRightmost(pCur);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                | crate::src::headers::btreeInt_h::BTCF_AtLast)
                as crate::src::ext::rtree::rtree::U8_0;
        } else {
            (*pCur).curFlags = ((*pCur).curFlags as ::core::ffi::c_int
                & !crate::src::headers::btreeInt_h::BTCF_AtLast)
                as crate::src::ext::rtree::rtree::U8_0;
        }
    } else if rc == crate::src::headers::sqlite3_h::SQLITE_EMPTY {
        *pRes = 1 as ::core::ffi::c_int;
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeLast(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if crate::src::headers::btreeInt_h::CURSOR_VALID == (*pCur).eState as ::core::ffi::c_int
        && (*pCur).curFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTCF_AtLast
            != 0 as ::core::ffi::c_int
    {
        *pRes = 0 as ::core::ffi::c_int;
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    btreeLast(pCur, pRes)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeTableMoveto(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    intKey: crate::src::ext::rtree::rtree::I64_0,
    biasRight: ::core::ffi::c_int,
    pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int;
    let __pCur_ref = unsafe { &mut *pCur };
    if __pCur_ref.eState as ::core::ffi::c_int == crate::src::headers::btreeInt_h::CURSOR_VALID
        && __pCur_ref.curFlags as ::core::ffi::c_int
            & crate::src::headers::btreeInt_h::BTCF_ValidNKey
            != 0 as ::core::ffi::c_int
    {
        if __pCur_ref.info.nKey == intKey {
            *pRes = 0 as ::core::ffi::c_int;
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        if __pCur_ref.info.nKey < intKey {
            if __pCur_ref.curFlags as ::core::ffi::c_int
                & crate::src::headers::btreeInt_h::BTCF_AtLast
                != 0 as ::core::ffi::c_int
            {
                *pRes = -(1 as ::core::ffi::c_int);
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
            if __pCur_ref.info.nKey + 1 as crate::src::ext::rtree::rtree::I64_0 == intKey {
                *pRes = 0 as ::core::ffi::c_int;
                rc = sqlite3BtreeNext(pCur, 0 as ::core::ffi::c_int);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    getCellInfo(pCur);
                    if __pCur_ref.info.nKey == intKey {
                        return crate::src::headers::sqlite3_h::SQLITE_OK;
                    }
                } else if rc != crate::src::headers::sqlite3_h::SQLITE_DONE {
                    return rc;
                }
            }
        }
    }
    rc = moveToRoot(pCur);
    if rc != 0 {
        if rc == crate::src::headers::sqlite3_h::SQLITE_EMPTY {
            *pRes = -(1 as ::core::ffi::c_int);
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        return rc;
    }
    loop {
        let mut lwr: ::core::ffi::c_int;
        let mut upr: ::core::ffi::c_int;
        let mut idx: ::core::ffi::c_int;
        let mut c: ::core::ffi::c_int = 0;
        let chldPg: crate::src::src::pager::Pgno;
        let pPage: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
        let mut pCell: *mut crate::src::ext::rtree::rtree::U8_0;
        lwr = 0 as ::core::ffi::c_int;
        upr = (*pPage).nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        idx = upr >> (1 as ::core::ffi::c_int - biasRight);
        loop {
            let mut nCellKey: crate::src::ext::rtree::rtree::I64_0 = 0;
            let __pPage_ref = unsafe { &mut *pPage };
            pCell = __pPage_ref.aDataOfst.offset(
                (__pPage_ref.maskPage as ::core::ffi::c_int
                    & ((*((*pPage)
                        .aCellIdx
                        .offset((2 as ::core::ffi::c_int * idx) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(0_isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * idx) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(1_isize) as ::core::ffi::c_int)) as isize,
            );
            if __pPage_ref.intKeyLeaf != 0 {
                loop {
                    let fresh11 = pCell;
                    pCell = pCell.offset(1);
                    if (0x80 as ::core::ffi::c_int > *fresh11 as ::core::ffi::c_int) {
                        break;
                    }
                    if pCell >= __pPage_ref.aDataEnd {
                        return crate::src::src::main::sqlite3CorruptError(
                            5859 as ::core::ffi::c_int,
                        );
                    }
                }
            }
            crate::src::src::util::sqlite3GetVarint(
                pCell,
                &raw mut nCellKey as *mut crate::src::ext::rtree::rtree::U64_0,
            );
            if nCellKey < intKey {
                lwr = idx + 1 as ::core::ffi::c_int;
                if lwr > upr {
                    c = -(1 as ::core::ffi::c_int);
                    current_block = 981995395831942902;
                    break;
                }
            } else if nCellKey > intKey {
                upr = idx - 1 as ::core::ffi::c_int;
                if lwr > upr {
                    c = 1 as ::core::ffi::c_int;
                    current_block = 981995395831942902;
                    break;
                }
            } else {
                __pCur_ref.ix = idx as crate::src::fts5::U16_0;
                if __pPage_ref.leaf == 0 {
                    lwr = idx;
                    current_block = 15432772999000535839;
                    break;
                } else {
                    __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
                        | crate::src::headers::btreeInt_h::BTCF_ValidNKey)
                        as crate::src::ext::rtree::rtree::U8_0;
                    __pCur_ref.info.nKey = nCellKey;
                    __pCur_ref.info.nSize = 0 as crate::src::fts5::U16_0;
                    *pRes = 0 as ::core::ffi::c_int;
                    return crate::src::headers::sqlite3_h::SQLITE_OK;
                }
            }
            idx = (lwr + upr) >> 1 as ::core::ffi::c_int;
        }
        match current_block {
            981995395831942902
                if (*pPage).leaf != 0 => {
                    __pCur_ref.ix = idx as crate::src::fts5::U16_0;
                    *pRes = c;
                    rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                    break;
                }
            _ => {}
        }
        if lwr >= (*pPage).nCell as ::core::ffi::c_int {
            chldPg = crate::src::src::util::sqlite3Get4byte((*pPage).aData.offset(
                ((*pPage).hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
            )
                as *mut crate::src::ext::rtree::rtree::U8_0)
                as crate::src::src::pager::Pgno;
        } else {
            let __pPage_ref = unsafe { &mut *pPage };
            chldPg = crate::src::src::util::sqlite3Get4byte(
                __pPage_ref.aData.offset(
                    (__pPage_ref.maskPage as ::core::ffi::c_int
                        & ((*((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * lwr) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(0_isize) as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * lwr) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(1_isize)
                                as ::core::ffi::c_int)) as isize,
                ),
            ) as crate::src::src::pager::Pgno;
        }
        __pCur_ref.ix = lwr as crate::src::fts5::U16_0;
        rc = moveToChild(pCur, chldPg as crate::src::ext::rtree::rtree::U32_0);
        if rc != 0 {
            break;
        }
    }
    __pCur_ref.info.nSize = 0 as crate::src::fts5::U16_0;
    rc
}

unsafe extern "C" fn indexCellCompare(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    idx: ::core::ffi::c_int,
    pIdxKey: *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
    xRecordCompare: crate::src::src::vdbe::RecordCompare,
) -> ::core::ffi::c_int {
    let c: ::core::ffi::c_int;
    let mut nCell: ::core::ffi::c_int;
    let __pPage_ref = unsafe { &mut *pPage };
    let pCell: *mut crate::src::ext::rtree::rtree::U8_0 = __pPage_ref.aDataOfst.offset(
        (__pPage_ref.maskPage as ::core::ffi::c_int
            & ((*((*pPage)
                .aCellIdx
                .offset((2 as ::core::ffi::c_int * idx) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0)
                .offset(0_isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *((*pPage)
                    .aCellIdx
                    .offset((2 as ::core::ffi::c_int * idx) as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(1_isize) as ::core::ffi::c_int)) as isize,
    );
    nCell = *pCell.offset(0_isize) as ::core::ffi::c_int;
    if nCell <= __pPage_ref.max1bytePayload as ::core::ffi::c_int {
        c = xRecordCompare.expect("non-null function pointer")(
            nCell,
            pCell.offset(1_isize) as *mut crate::src::ext::rtree::rtree::U8_0
                as *mut ::core::ffi::c_void,
            pIdxKey,
        );
    } else if *pCell.offset(1_isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0 && {
        nCell = ((nCell & 0x7f as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
            + *pCell.offset(1_isize) as ::core::ffi::c_int;
        nCell <= __pPage_ref.maxLocal as ::core::ffi::c_int
    } {
        c = xRecordCompare.expect("non-null function pointer")(
            nCell,
            pCell.offset(2_isize) as *mut crate::src::ext::rtree::rtree::U8_0
                as *mut ::core::ffi::c_void,
            pIdxKey,
        );
    } else {
        c = 99 as ::core::ffi::c_int;
    }
    c
}

unsafe extern "C" fn cursorOnLastPage(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*pCur).iPage as ::core::ffi::c_int {
        let pPage: *mut crate::src::headers::btreeInt_h::MemPage = (*pCur).apPage[i as usize];
        if ((*pCur).aiIdx[i as usize] as ::core::ffi::c_int) < (*pPage).nCell as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    1 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeIndexMoveto(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pIdxKey: *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
    pRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let current_block: u64;
    let mut rc: ::core::ffi::c_int;
    
    let xRecordCompare: crate::src::src::vdbe::RecordCompare = crate::src::src::vdbeaux::sqlite3VdbeFindCompare(
        pIdxKey as *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
    )
        as ::std::option::Option<
            unsafe extern "C" fn(
                _: i32,
                _: *const ::libc::c_void,
                _: *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
            ) -> i32,
        >;
    (*pIdxKey).errCode = 0 as crate::src::ext::rtree::rtree::U8_0;
    let __pCur_ref = unsafe { &mut *pCur };
    if __pCur_ref.eState as ::core::ffi::c_int == crate::src::headers::btreeInt_h::CURSOR_VALID
        && (*__pCur_ref.pPage).leaf as ::core::ffi::c_int != 0
        && cursorOnLastPage(pCur) != 0
    {
        let c: ::core::ffi::c_int;
        if __pCur_ref.ix as ::core::ffi::c_int
            == (*__pCur_ref.pPage).nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            && {
                c = indexCellCompare(
                    __pCur_ref.pPage,
                    __pCur_ref.ix as ::core::ffi::c_int,
                    pIdxKey,
                    xRecordCompare,
                );
                c <= 0 as ::core::ffi::c_int
            }
            && (*pIdxKey).errCode as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_OK
        {
            *pRes = c;
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        if __pCur_ref.iPage as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && indexCellCompare(
                __pCur_ref.pPage,
                0 as ::core::ffi::c_int,
                pIdxKey,
                xRecordCompare,
            ) <= 0 as ::core::ffi::c_int
            && (*pIdxKey).errCode as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_OK
        {
            __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
                & !(crate::src::headers::btreeInt_h::BTCF_ValidOvfl
                    | crate::src::headers::btreeInt_h::BTCF_AtLast))
                as crate::src::ext::rtree::rtree::U8_0;
            if (*__pCur_ref.pPage).isInit == 0 {
                return crate::src::src::main::sqlite3CorruptError(6054 as ::core::ffi::c_int);
            }
            current_block = 2719512138335094285;
        } else {
            (*pIdxKey).errCode =
                crate::src::headers::sqlite3_h::SQLITE_OK as crate::src::ext::rtree::rtree::U8_0;
            current_block = 4166486009154926805;
        }
    } else {
        current_block = 4166486009154926805;
    }
    if current_block == 4166486009154926805 {
        rc = moveToRoot(pCur);
        if rc != 0 {
            if rc == crate::src::headers::sqlite3_h::SQLITE_EMPTY {
                *pRes = -(1 as ::core::ffi::c_int);
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
            return rc;
        }
    }
    's_125: loop {
        let mut lwr: ::core::ffi::c_int;
        let mut upr: ::core::ffi::c_int;
        let mut idx: ::core::ffi::c_int;
        let mut c_0: ::core::ffi::c_int;
        let chldPg: crate::src::src::pager::Pgno;
        let pPage: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
        let mut pCell: *mut crate::src::ext::rtree::rtree::U8_0;
        lwr = 0 as ::core::ffi::c_int;
        upr = (*pPage).nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        idx = upr >> 1 as ::core::ffi::c_int;
        loop {
            let mut nCell: ::core::ffi::c_int;
            let __pPage_ref = unsafe { &mut *pPage };
            pCell = __pPage_ref.aDataOfst.offset(
                (__pPage_ref.maskPage as ::core::ffi::c_int
                    & ((*((*pPage)
                        .aCellIdx
                        .offset((2 as ::core::ffi::c_int * idx) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(0_isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * idx) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(1_isize) as ::core::ffi::c_int)) as isize,
            );
            nCell = *pCell.offset(0_isize) as ::core::ffi::c_int;
            if nCell <= __pPage_ref.max1bytePayload as ::core::ffi::c_int {
                c_0 = xRecordCompare.expect("non-null function pointer")(
                    nCell,
                    pCell.offset(1_isize) as *mut crate::src::ext::rtree::rtree::U8_0
                        as *mut ::core::ffi::c_void,
                    pIdxKey,
                );
            } else if *pCell.offset(1_isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                == 0
                && {
                    nCell = ((nCell & 0x7f as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                        + *pCell.offset(1_isize) as ::core::ffi::c_int;
                    nCell <= __pPage_ref.maxLocal as ::core::ffi::c_int
                }
            {
                c_0 = xRecordCompare.expect("non-null function pointer")(
                    nCell,
                    pCell.offset(2_isize) as *mut crate::src::ext::rtree::rtree::U8_0
                        as *mut ::core::ffi::c_void,
                    pIdxKey,
                );
            } else {
                let pCellKey: *mut ::core::ffi::c_void;
                let pCellBody: *mut crate::src::ext::rtree::rtree::U8_0 =
                    pCell.offset(-(__pPage_ref.childPtrSize as ::core::ffi::c_int as isize));
                let nOverrun: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
                __pPage_ref.xParseCell.expect("non-null function pointer")(
                    pPage,
                    pCellBody,
                    &raw mut __pCur_ref.info,
                );
                nCell = __pCur_ref.info.nKey as ::core::ffi::c_int;
                if nCell < 2 as ::core::ffi::c_int
                    || (nCell as crate::src::ext::rtree::rtree::U32_0)
                        .wrapping_div((*__pCur_ref.pBt).usableSize)
                        > (*__pCur_ref.pBt).nPage
                {
                    rc = crate::src::src::main::sqlite3CorruptError(6141 as ::core::ffi::c_int);
                    break 's_125;
                } else {
                    pCellKey = crate::src::src::malloc::sqlite3Malloc(
                        (nCell as crate::src::ext::rtree::rtree::U64_0)
                            .wrapping_add(nOverrun as crate::src::ext::rtree::rtree::U64_0),
                    );
                    if pCellKey.is_null() {
                        rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                        break 's_125;
                    } else {
                        __pCur_ref.ix = idx as crate::src::fts5::U16_0;
                        rc = accessPayload(
                            pCur,
                            0 as crate::src::ext::rtree::rtree::U32_0,
                            nCell as crate::src::ext::rtree::rtree::U32_0,
                            pCellKey as *mut ::core::ffi::c_uchar,
                            0 as ::core::ffi::c_int,
                        );
                        ::libc::memset(
                            (pCellKey as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(nCell as isize)
                                as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            nOverrun as crate::__stddef_size_t_h::SizeT,
                        );
                        __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
                            & !crate::src::headers::btreeInt_h::BTCF_ValidOvfl)
                            as crate::src::ext::rtree::rtree::U8_0;
                        if rc != 0 {
                            crate::src::src::malloc::sqlite3_free(pCellKey);
                            break 's_125;
                        } else {
                            c_0 = crate::src::src::vdbeaux::sqlite3VdbeRecordCompare(
                                nCell,
                                pCellKey,
                                pIdxKey as *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
                            );
                            crate::src::src::malloc::sqlite3_free(pCellKey);
                        }
                    }
                }
            }
            if c_0 < 0 as ::core::ffi::c_int {
                lwr = idx + 1 as ::core::ffi::c_int;
            } else if c_0 > 0 as ::core::ffi::c_int {
                upr = idx - 1 as ::core::ffi::c_int;
            } else {
                *pRes = 0 as ::core::ffi::c_int;
                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                __pCur_ref.ix = idx as crate::src::fts5::U16_0;
                if (*pIdxKey).errCode != 0 {
                    rc = crate::src::src::main::sqlite3CorruptError(6173 as ::core::ffi::c_int);
                }
                break 's_125;
            }
            if lwr > upr {
                break;
            }
            idx = (lwr + upr) >> 1 as ::core::ffi::c_int;
        }
        if (*pPage).leaf != 0 {
            __pCur_ref.ix = idx as crate::src::fts5::U16_0;
            *pRes = c_0;
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
            break;
        } else {
            if lwr >= (*pPage).nCell as ::core::ffi::c_int {
                chldPg = crate::src::src::util::sqlite3Get4byte((*pPage).aData.offset(
                    ((*pPage).hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    as crate::src::src::pager::Pgno;
            } else {
                let __pPage_ref = unsafe { &mut *pPage };
                chldPg = crate::src::src::util::sqlite3Get4byte(
                    __pPage_ref.aData.offset(
                        (__pPage_ref.maskPage as ::core::ffi::c_int
                            & ((*((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * lwr) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(0_isize)
                                as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int
                                | *((*pPage)
                                    .aCellIdx
                                    .offset((2 as ::core::ffi::c_int * lwr) as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0)
                                    .offset(1_isize)
                                    as ::core::ffi::c_int)) as isize,
                    ),
                ) as crate::src::src::pager::Pgno;
            }
            __pCur_ref.info.nSize = 0 as crate::src::fts5::U16_0;
            __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
                & !(crate::src::headers::btreeInt_h::BTCF_ValidNKey
                    | crate::src::headers::btreeInt_h::BTCF_ValidOvfl))
                as crate::src::ext::rtree::rtree::U8_0;
            if __pCur_ref.iPage as ::core::ffi::c_int
                >= crate::src::headers::btreeInt_h::BTCURSOR_MAX_DEPTH - 1 as ::core::ffi::c_int
            {
                return crate::src::src::main::sqlite3CorruptError(6204 as ::core::ffi::c_int);
            }
            __pCur_ref.aiIdx[__pCur_ref.iPage as usize] = lwr as crate::src::fts5::U16_0;
            __pCur_ref.apPage[__pCur_ref.iPage as usize] = __pCur_ref.pPage;
            __pCur_ref.ix = 0 as crate::src::fts5::U16_0;
            __pCur_ref.iPage += 1;
            rc = getAndInitPage(
                __pCur_ref.pBt,
                chldPg,
                &raw mut __pCur_ref.pPage,
                __pCur_ref.curPagerFlags as ::core::ffi::c_int,
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                && (((*__pCur_ref.pPage).nCell as ::core::ffi::c_int) < 1 as ::core::ffi::c_int
                    || (*__pCur_ref.pPage).intKey as ::core::ffi::c_int
                        != __pCur_ref.curIntKey as ::core::ffi::c_int)
            {
                releasePage(__pCur_ref.pPage);
                rc = crate::src::src::main::sqlite3CorruptError(6215 as ::core::ffi::c_int);
            }
            if (rc == 0) {
                continue;
            }
            __pCur_ref.iPage -= 1;
            __pCur_ref.pPage = __pCur_ref.apPage[__pCur_ref.iPage as usize];
            break;
        }
    }
    __pCur_ref.info.nSize = 0 as crate::src::fts5::U16_0;
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeEof(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    (crate::src::headers::btreeInt_h::CURSOR_VALID != (*pCur).eState as ::core::ffi::c_int)
        as ::core::ffi::c_int
}
pub unsafe extern "C" fn sqlite3BtreeRowCountEst(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> crate::src::ext::rtree::rtree::I64_0 {
    let mut n: crate::src::ext::rtree::rtree::I64_0;
    let mut i: crate::src::ext::rtree::rtree::U8_0;
    let __pCur_ref = unsafe { &*pCur };
    if __pCur_ref.eState as ::core::ffi::c_int != crate::src::headers::btreeInt_h::CURSOR_VALID {
        return 0 as crate::src::ext::rtree::rtree::I64_0;
    }
    if (*__pCur_ref.pPage).leaf as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::I64_0;
    }
    n = (*__pCur_ref.pPage).nCell as crate::src::ext::rtree::rtree::I64_0;
    i = 0 as crate::src::ext::rtree::rtree::U8_0;
    while (i as ::core::ffi::c_int) < __pCur_ref.iPage as ::core::ffi::c_int {
        n *= ((*__pCur_ref.apPage[i as usize]).nCell as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::I64_0;
        i = i.wrapping_add(1);
    }
    n
}
#[inline(never)]
unsafe extern "C" fn btreeNext(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    
    let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage;
    let __pCur_ref = unsafe { &mut *pCur };
    if __pCur_ref.eState as ::core::ffi::c_int != crate::src::headers::btreeInt_h::CURSOR_VALID {
        rc = if __pCur_ref.eState as ::core::ffi::c_int
            >= crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK
        {
            btreeRestoreCursorPosition(pCur)
        } else {
            crate::src::headers::sqlite3_h::SQLITE_OK
        };
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        if crate::src::headers::btreeInt_h::CURSOR_INVALID
            == __pCur_ref.eState as ::core::ffi::c_int
        {
            return crate::src::headers::sqlite3_h::SQLITE_DONE;
        }
        if __pCur_ref.eState as ::core::ffi::c_int
            == crate::src::headers::btreeInt_h::CURSOR_SKIPNEXT
        {
            __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_VALID
                as crate::src::ext::rtree::rtree::U8_0;
            if __pCur_ref.skipNext > 0 as ::core::ffi::c_int {
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        }
    }
    pPage = __pCur_ref.pPage;
    __pCur_ref.ix = __pCur_ref.ix.wrapping_add(1);
    let idx: ::core::ffi::c_int = __pCur_ref.ix as ::core::ffi::c_int;
    if crate::src::src::util::sqlite3FaultSim(412 as ::core::ffi::c_int) != 0 {
        (*pPage).isInit = 0 as crate::src::ext::rtree::rtree::U8_0;
    }
    if (*pPage).isInit == 0 {
        return crate::src::src::main::sqlite3CorruptError(6316 as ::core::ffi::c_int);
    }
    if idx >= (*pPage).nCell as ::core::ffi::c_int {
        if (*pPage).leaf == 0 {
            rc = moveToChild(
                pCur,
                crate::src::src::util::sqlite3Get4byte((*pPage).aData.offset(
                    ((*pPage).hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                    as *mut crate::src::ext::rtree::rtree::U8_0),
            );
            if rc != 0 {
                return rc;
            }
            return moveToLeftmost(pCur);
        }
        loop {
            if __pCur_ref.iPage as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_INVALID
                    as crate::src::ext::rtree::rtree::U8_0;
                return crate::src::headers::sqlite3_h::SQLITE_DONE;
            }
            moveToParent(pCur);
            pPage = __pCur_ref.pPage;
            if ((__pCur_ref.ix as ::core::ffi::c_int) < (*pPage).nCell as ::core::ffi::c_int) {
                break;
            }
        }
        if (*pPage).intKey != 0 {
            return sqlite3BtreeNext(pCur, 0 as ::core::ffi::c_int);
        } else {
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    if (*pPage).leaf != 0 {
        crate::src::headers::sqlite3_h::SQLITE_OK
    } else {
        moveToLeftmost(pCur)
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeNext(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    mut _flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let __pCur_ref = unsafe { &mut *pCur };
    __pCur_ref.info.nSize = 0 as crate::src::fts5::U16_0;
    __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
        & !(crate::src::headers::btreeInt_h::BTCF_ValidNKey
            | crate::src::headers::btreeInt_h::BTCF_ValidOvfl))
        as crate::src::ext::rtree::rtree::U8_0;
    if __pCur_ref.eState as ::core::ffi::c_int != crate::src::headers::btreeInt_h::CURSOR_VALID {
        return btreeNext(pCur);
    }
    let pPage: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
    __pCur_ref.ix = __pCur_ref.ix.wrapping_add(1);
    if __pCur_ref.ix as ::core::ffi::c_int >= (*pPage).nCell as ::core::ffi::c_int {
        __pCur_ref.ix = __pCur_ref.ix.wrapping_sub(1);
        return btreeNext(pCur);
    }
    if (*pPage).leaf != 0 {
        crate::src::headers::sqlite3_h::SQLITE_OK
    } else {
        moveToLeftmost(pCur)
    }
}
#[inline(never)]
unsafe extern "C" fn btreePrevious(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage;
    if (*pCur).eState as ::core::ffi::c_int != crate::src::headers::btreeInt_h::CURSOR_VALID {
        let __pCur_ref = unsafe { &mut *pCur };
        rc = if __pCur_ref.eState as ::core::ffi::c_int
            >= crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK
        {
            btreeRestoreCursorPosition(pCur)
        } else {
            crate::src::headers::sqlite3_h::SQLITE_OK
        };
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        if crate::src::headers::btreeInt_h::CURSOR_INVALID
            == __pCur_ref.eState as ::core::ffi::c_int
        {
            return crate::src::headers::sqlite3_h::SQLITE_DONE;
        }
        if crate::src::headers::btreeInt_h::CURSOR_SKIPNEXT
            == __pCur_ref.eState as ::core::ffi::c_int
        {
            __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_VALID
                as crate::src::ext::rtree::rtree::U8_0;
            if __pCur_ref.skipNext < 0 as ::core::ffi::c_int {
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        }
    }
    pPage = (*pCur).pPage;
    if crate::src::src::util::sqlite3FaultSim(412 as ::core::ffi::c_int) != 0 {
        (*pPage).isInit = 0 as crate::src::ext::rtree::rtree::U8_0;
    }
    if (*pPage).isInit == 0 {
        return crate::src::src::main::sqlite3CorruptError(6409 as ::core::ffi::c_int);
    }
    if (*pPage).leaf == 0 {
        let idx: ::core::ffi::c_int = (*pCur).ix as ::core::ffi::c_int;
        let __pPage_ref = unsafe { &mut *pPage };
        rc = moveToChild(
            pCur,
            crate::src::src::util::sqlite3Get4byte(
                __pPage_ref.aData.offset(
                    (__pPage_ref.maskPage as ::core::ffi::c_int
                        & ((*((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * idx) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(0_isize) as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * idx) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(1_isize)
                                as ::core::ffi::c_int)) as isize,
                ),
            ),
        );
        if rc != 0 {
            return rc;
        }
        rc = moveToRightmost(pCur);
    } else {
        let __pCur_ref = unsafe { &mut *pCur };
        while __pCur_ref.ix as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if __pCur_ref.iPage as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_INVALID
                    as crate::src::ext::rtree::rtree::U8_0;
                return crate::src::headers::sqlite3_h::SQLITE_DONE;
            }
            moveToParent(pCur);
        }
        __pCur_ref.ix = __pCur_ref.ix.wrapping_sub(1);
        pPage = __pCur_ref.pPage;
        if (*pPage).intKey as ::core::ffi::c_int != 0 && (*pPage).leaf == 0 {
            rc = sqlite3BtreePrevious(pCur, 0 as ::core::ffi::c_int);
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreePrevious(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    mut _flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pCur_ref = unsafe { &mut *pCur };
    __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
        & !(crate::src::headers::btreeInt_h::BTCF_AtLast
            | crate::src::headers::btreeInt_h::BTCF_ValidOvfl
            | crate::src::headers::btreeInt_h::BTCF_ValidNKey))
        as crate::src::ext::rtree::rtree::U8_0;
    __pCur_ref.info.nSize = 0 as crate::src::fts5::U16_0;
    if __pCur_ref.eState as ::core::ffi::c_int != crate::src::headers::btreeInt_h::CURSOR_VALID
        || __pCur_ref.ix as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*__pCur_ref.pPage).leaf as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        return btreePrevious(pCur);
    }
    __pCur_ref.ix = __pCur_ref.ix.wrapping_sub(1);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn allocateBtreePage(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    ppPage: *mut *mut crate::src::headers::btreeInt_h::MemPage,
    pPgno: *mut crate::src::src::pager::Pgno,
    nearby: crate::src::src::pager::Pgno,
    eMode: crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    
    let mut rc: ::core::ffi::c_int;
    
    let mut k: crate::src::ext::rtree::rtree::U32_0;
    let mut pTrunk: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let mut pPrevTrunk: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    
    let pPage1: *mut crate::src::headers::btreeInt_h::MemPage = (*pBt).pPage1;
    let mxPage: crate::src::src::pager::Pgno = btreePagecount(pBt);
    let n: crate::src::ext::rtree::rtree::U32_0 = crate::src::src::util::sqlite3Get4byte(
        (*pPage1).aData.offset(36_isize) as *mut crate::src::ext::rtree::rtree::U8_0
    );
    if n >= mxPage {
        return crate::src::src::main::sqlite3CorruptError(6499 as ::core::ffi::c_int);
    }
    if n > 0 as crate::src::ext::rtree::rtree::U32_0 {
        let mut iTrunk: crate::src::src::pager::Pgno;
        let mut searchList: crate::src::ext::rtree::rtree::U8_0 =
            0 as crate::src::ext::rtree::rtree::U8_0;
        let mut nSearch: crate::src::ext::rtree::rtree::U32_0 =
            0 as crate::src::ext::rtree::rtree::U32_0;
        if eMode as ::core::ffi::c_int == BTALLOC_EXACT {
            if nearby <= mxPage {
                let mut eType: crate::src::ext::rtree::rtree::U8_0 = 0;
                rc = ptrmapGet(
                    pBt,
                    nearby,
                    &raw mut eType,
                    ::core::ptr::null_mut::<crate::src::src::pager::Pgno>(),
                );
                if rc != 0 {
                    return rc;
                }
                if eType as ::core::ffi::c_int == crate::src::headers::btreeInt_h::PTRMAP_FREEPAGE {
                    searchList = 1 as crate::src::ext::rtree::rtree::U8_0;
                }
            }
        } else if eMode as ::core::ffi::c_int == BTALLOC_LE {
            searchList = 1 as crate::src::ext::rtree::rtree::U8_0;
        }
        rc = crate::src::src::pager::sqlite3PagerWrite(
            (*pPage1).pDbPage as *mut crate::src::src::pcache::PgHdr,
        );
        if rc != 0 {
            return rc;
        }
        crate::src::src::util::sqlite3Put4byte(
            (*pPage1).aData.offset(36_isize) as *mut crate::src::ext::rtree::rtree::U8_0,
            n.wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0),
        );
        loop {
            pPrevTrunk = pTrunk;
            if !pPrevTrunk.is_null() {
                iTrunk =
                    crate::src::src::util::sqlite3Get4byte((*pPrevTrunk).aData.offset(0_isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        as crate::src::src::pager::Pgno;
            } else {
                iTrunk =
                    crate::src::src::util::sqlite3Get4byte((*pPage1).aData.offset(32_isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        as crate::src::src::pager::Pgno;
            }
            if iTrunk > mxPage || {
                let fresh9 = nSearch;
                nSearch = nSearch.wrapping_add(1);
                fresh9 > n
            } {
                rc = crate::src::src::main::sqlite3CorruptError(6555 as ::core::ffi::c_int);
            } else {
                rc = btreeGetUnusedPage(pBt, iTrunk, &raw mut pTrunk, 0 as ::core::ffi::c_int);
            }
            if rc != 0 {
                pTrunk = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                break;
            } else {
                k =
                    crate::src::src::util::sqlite3Get4byte((*pTrunk).aData.offset(4_isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0);
                if k == 0 as crate::src::ext::rtree::rtree::U32_0 && searchList == 0 {
                    rc = crate::src::src::pager::sqlite3PagerWrite(
                        (*pTrunk).pDbPage as *mut crate::src::src::pcache::PgHdr,
                    );
                    if rc != 0 {
                        break;
                    }
                    *pPgno = iTrunk;
                    ::core::ptr::copy_nonoverlapping(
                        (*pTrunk).aData.offset(0_isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0
                            as *const u8,
                        (*pPage1).aData.offset(32_isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0
                            as *mut u8,
                        4_usize,
                    );
                    *ppPage = pTrunk;
                    pTrunk = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                } else if k
                    > (*pBt)
                        .usableSize
                        .wrapping_div(4 as crate::src::ext::rtree::rtree::U32_0)
                        .wrapping_sub(2 as crate::src::ext::rtree::rtree::U32_0)
                {
                    rc = crate::src::src::main::sqlite3CorruptError(6584 as ::core::ffi::c_int);
                    break;
                } else if searchList as ::core::ffi::c_int != 0
                    && (nearby == iTrunk
                        || iTrunk < nearby && eMode as ::core::ffi::c_int == BTALLOC_LE)
                {
                    *pPgno = iTrunk;
                    *ppPage = pTrunk;
                    searchList = 0 as crate::src::ext::rtree::rtree::U8_0;
                    rc = crate::src::src::pager::sqlite3PagerWrite(
                        (*pTrunk).pDbPage as *mut crate::src::src::pcache::PgHdr,
                    );
                    if rc != 0 {
                        break;
                    }
                    if k == 0 as crate::src::ext::rtree::rtree::U32_0 {
                        if pPrevTrunk.is_null() {
                            ::core::ptr::copy_nonoverlapping(
                                (*pTrunk).aData.offset(0_isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0
                                    as *const u8,
                                (*pPage1).aData.offset(32_isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0
                                    as *mut u8,
                                4_usize,
                            );
                        } else {
                            rc = crate::src::src::pager::sqlite3PagerWrite(
                                (*pPrevTrunk).pDbPage as *mut crate::src::src::pcache::PgHdr,
                            );
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                break;
                            }
                            ::core::ptr::copy_nonoverlapping(
                                (*pTrunk).aData.offset(0_isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0
                                    as *const u8,
                                (*pPrevTrunk).aData.offset(0_isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0
                                    as *mut u8,
                                4_usize,
                            );
                        }
                    } else {
                        let mut pNewTrunk: *mut crate::src::headers::btreeInt_h::MemPage =
                            ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                        let iNewTrunk: crate::src::src::pager::Pgno =
                            crate::src::src::util::sqlite3Get4byte(
                                (*pTrunk).aData.offset(8_isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0,
                            ) as crate::src::src::pager::Pgno;
                        if iNewTrunk > mxPage {
                            rc = crate::src::src::main::sqlite3CorruptError(
                                6618 as ::core::ffi::c_int,
                            );
                            break;
                        } else {
                            rc = btreeGetUnusedPage(
                                pBt,
                                iNewTrunk,
                                &raw mut pNewTrunk,
                                0 as ::core::ffi::c_int,
                            );
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                break;
                            }
                            rc = crate::src::src::pager::sqlite3PagerWrite(
                                (*pNewTrunk).pDbPage as *mut crate::src::src::pcache::PgHdr,
                            );
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                releasePage(pNewTrunk);
                                break;
                            } else {
                                let __pNewTrunk_ref = unsafe { &mut *pNewTrunk };
                                ::core::ptr::copy_nonoverlapping(
                                    (*pTrunk).aData.offset(0_isize)
                                        as *mut crate::src::ext::rtree::rtree::U8_0
                                        as *const u8,
                                    __pNewTrunk_ref.aData.offset(0_isize)
                                        as *mut crate::src::ext::rtree::rtree::U8_0
                                        as *mut u8,
                                    4_usize,
                                );
                                crate::src::src::util::sqlite3Put4byte(
                                    __pNewTrunk_ref.aData.offset(4_isize)
                                        as *mut crate::src::ext::rtree::rtree::U8_0,
                                    k.wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0),
                                );
                                ::core::ptr::copy_nonoverlapping(
                                    (*pTrunk).aData.offset(12_isize)
                                        as *mut crate::src::ext::rtree::rtree::U8_0
                                        as *const u8,
                                    __pNewTrunk_ref.aData.offset(8_isize)
                                        as *mut crate::src::ext::rtree::rtree::U8_0
                                        as *mut u8,
                                    k.wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0)
                                        .wrapping_mul(4 as crate::src::ext::rtree::rtree::U32_0)
                                        as usize,
                                );
                                releasePage(pNewTrunk);
                                if pPrevTrunk.is_null() {
                                    crate::src::src::util::sqlite3Put4byte(
                                        (*pPage1).aData.offset(32_isize)
                                            as *mut crate::src::ext::rtree::rtree::U8_0,
                                        iNewTrunk as crate::src::ext::rtree::rtree::U32_0,
                                    );
                                } else {
                                    rc = crate::src::src::pager::sqlite3PagerWrite(
                                        (*pPrevTrunk).pDbPage
                                            as *mut crate::src::src::pcache::PgHdr,
                                    );
                                    if rc != 0 {
                                        break;
                                    }
                                    crate::src::src::util::sqlite3Put4byte(
                                        (*pPrevTrunk).aData.offset(0_isize)
                                            as *mut crate::src::ext::rtree::rtree::U8_0,
                                        iNewTrunk as crate::src::ext::rtree::rtree::U32_0,
                                    );
                                }
                            }
                        }
                    }
                    pTrunk = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                } else if k > 0 as crate::src::ext::rtree::rtree::U32_0 {
                    let mut closest: crate::src::ext::rtree::rtree::U32_0;
                    let mut iPage: crate::src::src::pager::Pgno;
                    let aData: *mut ::core::ffi::c_uchar =
                        (*pTrunk).aData as *mut ::core::ffi::c_uchar;
                    if nearby > 0 as crate::src::src::pager::Pgno {
                        let mut i: crate::src::ext::rtree::rtree::U32_0;
                        closest = 0 as crate::src::ext::rtree::rtree::U32_0;
                        if eMode as ::core::ffi::c_int == BTALLOC_LE {
                            i = 0 as crate::src::ext::rtree::rtree::U32_0;
                            while i < k {
                                iPage = crate::src::src::util::sqlite3Get4byte(aData.offset(
                                    (8 as crate::src::ext::rtree::rtree::U32_0).wrapping_add(
                                        i.wrapping_mul(4 as crate::src::ext::rtree::rtree::U32_0),
                                    ) as isize,
                                )
                                    as *mut ::core::ffi::c_uchar)
                                    as crate::src::src::pager::Pgno;
                                if iPage <= nearby {
                                    closest = i;
                                    break;
                                } else {
                                    i = i.wrapping_add(1);
                                }
                            }
                        } else {
                            let mut dist: ::core::ffi::c_int;
                            dist = crate::src::src::util::sqlite3AbsInt32(
                                crate::src::src::util::sqlite3Get4byte(
                                    aData.offset(8_isize) as *mut ::core::ffi::c_uchar
                                )
                                .wrapping_sub(nearby as crate::src::ext::rtree::rtree::U32_0)
                                    as ::core::ffi::c_int,
                            );
                            i = 1 as crate::src::ext::rtree::rtree::U32_0;
                            while i < k {
                                let d2: ::core::ffi::c_int =
                                    crate::src::src::util::sqlite3AbsInt32(
                                        crate::src::src::util::sqlite3Get4byte(
                                            aData.offset(
                                                (8 as crate::src::ext::rtree::rtree::U32_0)
                                                    .wrapping_add(i.wrapping_mul(
                                                        4 as crate::src::ext::rtree::rtree::U32_0,
                                                    ))
                                                    as isize,
                                            )
                                                as *mut ::core::ffi::c_uchar,
                                        )
                                        .wrapping_sub(
                                            nearby as crate::src::ext::rtree::rtree::U32_0,
                                        )
                                            as ::core::ffi::c_int,
                                    );
                                if d2 < dist {
                                    closest = i;
                                    dist = d2;
                                }
                                i = i.wrapping_add(1);
                            }
                        }
                    } else {
                        closest = 0 as crate::src::ext::rtree::rtree::U32_0;
                    }
                    iPage = crate::src::src::util::sqlite3Get4byte(aData.offset(
                        (8 as crate::src::ext::rtree::rtree::U32_0).wrapping_add(
                            closest.wrapping_mul(4 as crate::src::ext::rtree::rtree::U32_0),
                        ) as isize,
                    )
                        as *mut ::core::ffi::c_uchar)
                        as crate::src::src::pager::Pgno;
                    if iPage > mxPage || iPage < 2 as crate::src::src::pager::Pgno {
                        rc = crate::src::src::main::sqlite3CorruptError(6683 as ::core::ffi::c_int);
                        break;
                    } else if searchList == 0
                        || (iPage == nearby
                            || iPage < nearby && eMode as ::core::ffi::c_int == BTALLOC_LE)
                    {
                        
                        *pPgno = iPage;
                        rc = crate::src::src::pager::sqlite3PagerWrite(
                            (*pTrunk).pDbPage as *mut crate::src::src::pcache::PgHdr,
                        );
                        if rc != 0 {
                            break;
                        }
                        if closest < k.wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0) {
                            ::core::ptr::copy_nonoverlapping(
                                aData.offset(
                                    (4 as crate::src::ext::rtree::rtree::U32_0).wrapping_add(
                                        k.wrapping_mul(4 as crate::src::ext::rtree::rtree::U32_0),
                                    ) as isize,
                                ) as *mut ::core::ffi::c_uchar
                                    as *const u8,
                                aData.offset(
                                    (8 as crate::src::ext::rtree::rtree::U32_0).wrapping_add(
                                        closest.wrapping_mul(
                                            4 as crate::src::ext::rtree::rtree::U32_0,
                                        ),
                                    ) as isize,
                                ) as *mut ::core::ffi::c_uchar
                                    as *mut u8,
                                4_usize,
                            );
                        }
                        crate::src::src::util::sqlite3Put4byte(
                            aData.offset(4_isize) as *mut crate::src::ext::rtree::rtree::U8_0,
                            k.wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0),
                        );
                        let noContent: ::core::ffi::c_int = if btreeGetHasContent(pBt, *pPgno) == 0 {
                            crate::src::src::pager::PAGER_GET_NOCONTENT
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        rc = btreeGetUnusedPage(pBt, *pPgno, ppPage, noContent);
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            rc = crate::src::src::pager::sqlite3PagerWrite(
                                (**ppPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
                            );
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                releasePage(*ppPage);
                                *ppPage = ::core::ptr::null_mut::<
                                    crate::src::headers::btreeInt_h::MemPage,
                                >();
                            }
                        }
                        searchList = 0 as crate::src::ext::rtree::rtree::U8_0;
                    }
                }
                releasePage(pPrevTrunk);
                pPrevTrunk = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                if (searchList == 0) {
                    break;
                }
            }
        }
    } else {
        let __pBt_ref = unsafe { &mut *pBt };
        let bNoContent: ::core::ffi::c_int =
            if 0 as ::core::ffi::c_int == __pBt_ref.bDoTruncate as ::core::ffi::c_int {
                crate::src::src::pager::PAGER_GET_NOCONTENT
            } else {
                0 as ::core::ffi::c_int
            };
        rc = crate::src::src::pager::sqlite3PagerWrite(
            (*__pBt_ref.pPage1).pDbPage as *mut crate::src::src::pcache::PgHdr,
        );
        if rc != 0 {
            return rc;
        }
        __pBt_ref.nPage = __pBt_ref.nPage.wrapping_add(1);
        if __pBt_ref.nPage
            == (crate::src::src::global::sqlite3PendingByte as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_div(__pBt_ref.pageSize)
                .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
        {
            __pBt_ref.nPage = __pBt_ref.nPage.wrapping_add(1);
        }
        if __pBt_ref.autoVacuum as ::core::ffi::c_int != 0
            && ptrmapPageno(pBt, __pBt_ref.nPage as crate::src::src::pager::Pgno) == __pBt_ref.nPage
        {
            let mut pPg: *mut crate::src::headers::btreeInt_h::MemPage =
                ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
            rc = btreeGetUnusedPage(
                pBt,
                __pBt_ref.nPage as crate::src::src::pager::Pgno,
                &raw mut pPg,
                bNoContent,
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::src::src::pager::sqlite3PagerWrite(
                    (*pPg).pDbPage as *mut crate::src::src::pcache::PgHdr,
                );
                releasePage(pPg);
            }
            if rc != 0 {
                return rc;
            }
            __pBt_ref.nPage = __pBt_ref.nPage.wrapping_add(1);
            if __pBt_ref.nPage
                == (crate::src::src::global::sqlite3PendingByte
                    as crate::src::ext::rtree::rtree::U32_0)
                    .wrapping_div(__pBt_ref.pageSize)
                    .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
            {
                __pBt_ref.nPage = __pBt_ref.nPage.wrapping_add(1);
            }
        }
        crate::src::src::util::sqlite3Put4byte(
            (*__pBt_ref.pPage1).aData.offset(28_isize),
            __pBt_ref.nPage,
        );
        *pPgno = __pBt_ref.nPage as crate::src::src::pager::Pgno;
        rc = btreeGetUnusedPage(pBt, *pPgno, ppPage, bNoContent);
        if rc != 0 {
            return rc;
        }
        rc = crate::src::src::pager::sqlite3PagerWrite(
            (**ppPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            releasePage(*ppPage);
            *ppPage = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
        }
    }
    releasePage(pTrunk);
    releasePage(pPrevTrunk);
    rc
}

unsafe extern "C" fn freePage2(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pMemPage: *mut crate::src::headers::btreeInt_h::MemPage,
    iPage: crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pTrunk: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let mut iTrunk: crate::src::src::pager::Pgno = 0 as crate::src::src::pager::Pgno;
    let pPage1: *mut crate::src::headers::btreeInt_h::MemPage = (*pBt).pPage1;
    let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage;
    let mut rc: ::core::ffi::c_int;
    let nFree: crate::src::ext::rtree::rtree::U32_0;
    if iPage < 2 as crate::src::src::pager::Pgno || iPage > (*pBt).nPage {
        return crate::src::src::main::sqlite3CorruptError(6810 as ::core::ffi::c_int);
    }
    if !pMemPage.is_null() {
        pPage = pMemPage;
        crate::src::src::pager::sqlite3PagerRef(
            (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
        );
    } else {
        pPage = btreePageLookup(pBt, iPage);
    }
    rc = crate::src::src::pager::sqlite3PagerWrite(
        (*pPage1).pDbPage as *mut crate::src::src::pcache::PgHdr,
    );
    if (rc == 0) {
        nFree = crate::src::src::util::sqlite3Get4byte(
            (*pPage1).aData.offset(36_isize) as *mut crate::src::ext::rtree::rtree::U8_0
        );
        crate::src::src::util::sqlite3Put4byte(
            (*pPage1).aData.offset(36_isize) as *mut crate::src::ext::rtree::rtree::U8_0,
            nFree.wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0),
        );
        if (*pBt).btsFlags as ::core::ffi::c_int
            & crate::src::headers::btreeInt_h::BTS_SECURE_DELETE
            != 0
        {
            if pPage.is_null() && {
                rc = btreeGetPage(pBt, iPage, &raw mut pPage, 0 as ::core::ffi::c_int);
                rc != 0 as ::core::ffi::c_int
            } || {
                rc = crate::src::src::pager::sqlite3PagerWrite(
                    (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
                );
                rc != 0 as ::core::ffi::c_int
            } {
                current_block = 4426220020876744259;
            } else {
                ::libc::memset(
                    (*pPage).aData as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (*(*pPage).pBt).pageSize as crate::__stddef_size_t_h::SizeT,
                );
                current_block = 5143058163439228106;
            }
        } else {
            current_block = 5143058163439228106;
        }
        match current_block {
            4426220020876744259 => {}
            _ => {
                if (*pBt).autoVacuum != 0 {
                    ptrmapPut(
                        pBt,
                        iPage,
                        crate::src::headers::btreeInt_h::PTRMAP_FREEPAGE
                            as crate::src::ext::rtree::rtree::U8_0,
                        0 as crate::src::src::pager::Pgno,
                        &raw mut rc,
                    );
                    if rc != 0 {
                        current_block = 4426220020876744259;
                    } else {
                        current_block = 2838571290723028321;
                    }
                } else {
                    current_block = 2838571290723028321;
                }
                match current_block {
                    4426220020876744259 => {}
                    _ => {
                        if nFree != 0 as crate::src::ext::rtree::rtree::U32_0 {
                            let nLeaf: crate::src::ext::rtree::rtree::U32_0;
                            iTrunk = crate::src::src::util::sqlite3Get4byte(
                                (*pPage1).aData.offset(32_isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0,
                            ) as crate::src::src::pager::Pgno;
                            if iTrunk > btreePagecount(pBt) {
                                rc = crate::src::src::main::sqlite3CorruptError(
                                    6857 as ::core::ffi::c_int,
                                );
                                current_block = 4426220020876744259;
                            } else {
                                rc = btreeGetPage(
                                    pBt,
                                    iTrunk,
                                    &raw mut pTrunk,
                                    0 as ::core::ffi::c_int,
                                );
                                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                    current_block = 4426220020876744259;
                                } else {
                                    nLeaf = crate::src::src::util::sqlite3Get4byte(
                                        (*pTrunk).aData.offset(4_isize)
                                            as *mut crate::src::ext::rtree::rtree::U8_0,
                                    );
                                    if nLeaf
                                        > (*pBt)
                                            .usableSize
                                            .wrapping_div(4 as crate::src::ext::rtree::rtree::U32_0)
                                            .wrapping_sub(2 as crate::src::ext::rtree::rtree::U32_0)
                                    {
                                        rc = crate::src::src::main::sqlite3CorruptError(
                                            6868 as ::core::ffi::c_int,
                                        );
                                        current_block = 4426220020876744259;
                                    } else if nLeaf
                                        < (*pBt)
                                            .usableSize
                                            .wrapping_div(4 as crate::src::ext::rtree::rtree::U32_0)
                                            .wrapping_sub(8 as crate::src::ext::rtree::rtree::U32_0)
                                    {
                                        rc = crate::src::src::pager::sqlite3PagerWrite(
                                            (*pTrunk).pDbPage
                                                as *mut crate::src::src::pcache::PgHdr,
                                        );
                                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                            crate::src::src::util::sqlite3Put4byte(
                                                (*pTrunk).aData.offset(4_isize)
                                                    as *mut crate::src::ext::rtree::rtree::U8_0,
                                                nLeaf.wrapping_add(
                                                    1 as crate::src::ext::rtree::rtree::U32_0,
                                                ),
                                            );
                                            crate::src::src::util::sqlite3Put4byte(
                                                (*pTrunk).aData.offset(
                                                    (8 as crate::src::ext::rtree::rtree::U32_0)
                                                        .wrapping_add(nLeaf.wrapping_mul(
                                                        4 as crate::src::ext::rtree::rtree::U32_0,
                                                    ))
                                                        as isize,
                                                )
                                                    as *mut crate::src::ext::rtree::rtree::U8_0,
                                                iPage as crate::src::ext::rtree::rtree::U32_0,
                                            );
                                            if !pPage.is_null()
                                                && (*pBt).btsFlags as ::core::ffi::c_int
                                                    & crate::src::headers::btreeInt_h::BTS_SECURE_DELETE
                                                    == 0 as ::core::ffi::c_int
                                            {
                                                crate::src::src::pager::sqlite3PagerDontWrite((*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr);
                                            }
                                            rc = btreeSetHasContent(pBt, iPage);
                                        }
                                        current_block = 4426220020876744259;
                                    } else {
                                        current_block = 4090602189656566074;
                                    }
                                }
                            }
                        } else {
                            current_block = 4090602189656566074;
                        }
                        match current_block {
                            4426220020876744259 => {}
                            _ => {
                                if !(pPage.is_null() && {
                                    rc = btreeGetPage(
                                        pBt,
                                        iPage,
                                        &raw mut pPage,
                                        0 as ::core::ffi::c_int,
                                    );
                                    crate::src::headers::sqlite3_h::SQLITE_OK != rc
                                }) {
                                    rc = crate::src::src::pager::sqlite3PagerWrite(
                                        (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
                                    );
                                    if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                                        crate::src::src::util::sqlite3Put4byte(
                                            (*pPage).aData,
                                            iTrunk as crate::src::ext::rtree::rtree::U32_0,
                                        );
                                        crate::src::src::util::sqlite3Put4byte(
                                            (*pPage).aData.offset(4_isize)
                                                as *mut crate::src::ext::rtree::rtree::U8_0,
                                            0 as crate::src::ext::rtree::rtree::U32_0,
                                        );
                                        crate::src::src::util::sqlite3Put4byte(
                                            (*pPage1).aData.offset(32_isize)
                                                as *mut crate::src::ext::rtree::rtree::U8_0,
                                            iPage as crate::src::ext::rtree::rtree::U32_0,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !pPage.is_null() {
        (*pPage).isInit = 0 as crate::src::ext::rtree::rtree::U8_0;
    }
    releasePage(pPage);
    releasePage(pTrunk);
    rc
}

unsafe extern "C" fn freePage(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pRC: *mut ::core::ffi::c_int,
) {
    if *pRC == crate::src::headers::sqlite3_h::SQLITE_OK {
        *pRC = freePage2((*pPage).pBt, pPage, (*pPage).pgno);
    }
}
#[inline(never)]
unsafe extern "C" fn clearCellOverflow(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut ::core::ffi::c_uchar,
    pInfo: *mut crate::src::headers::btreeInt_h::CellInfo,
) -> ::core::ffi::c_int {
    
    let mut ovflPgno: crate::src::src::pager::Pgno;
    let mut rc: ::core::ffi::c_int;
    let mut nOvfl: ::core::ffi::c_int;
    
    let __pInfo_ref = unsafe { &mut *pInfo };
    if pCell.offset(__pInfo_ref.nSize as ::core::ffi::c_int as isize) > (*pPage).aDataEnd {
        return crate::src::src::main::sqlite3CorruptError(6957 as ::core::ffi::c_int);
    }
    ovflPgno = crate::src::src::util::sqlite3Get4byte(
        pCell
            .offset(__pInfo_ref.nSize as ::core::ffi::c_int as isize)
            .offset(-(4 as ::core::ffi::c_int as isize)),
    ) as crate::src::src::pager::Pgno;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*pPage).pBt;
    let ovflPageSize: crate::src::ext::rtree::rtree::U32_0 = (*pBt)
        .usableSize
        .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0);
    nOvfl = (*pInfo)
        .nPayload
        .wrapping_sub(__pInfo_ref.nLocal as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_add(ovflPageSize)
        .wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0)
        .wrapping_div(ovflPageSize) as ::core::ffi::c_int;
    loop {
        let fresh10 = nOvfl;
        nOvfl -= 1;
        if (fresh10 == 0) {
            break;
        }
        let mut iNext: crate::src::src::pager::Pgno = 0 as crate::src::src::pager::Pgno;
        let mut pOvfl: *mut crate::src::headers::btreeInt_h::MemPage =
            ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
        if ovflPgno < 2 as crate::src::src::pager::Pgno || ovflPgno > btreePagecount(pBt) {
            return crate::src::src::main::sqlite3CorruptError(6974 as ::core::ffi::c_int);
        }
        if nOvfl != 0 {
            rc = getOverflowPage(pBt, ovflPgno, &raw mut pOvfl, &raw mut iNext);
            if rc != 0 {
                return rc;
            }
        }
        if (!pOvfl.is_null() || {
            pOvfl = btreePageLookup(pBt, ovflPgno);
            !pOvfl.is_null()
        }) && crate::src::src::pager::sqlite3PagerPageRefcount(
            (*pOvfl).pDbPage as *mut crate::src::src::pcache::PgHdr,
        ) != 1 as ::core::ffi::c_int
        {
            rc = crate::src::src::main::sqlite3CorruptError(6994 as ::core::ffi::c_int);
        } else {
            rc = freePage2(pBt, pOvfl, ovflPgno);
        }
        if !pOvfl.is_null() {
            crate::src::src::pager::sqlite3PagerUnref(
                (*pOvfl).pDbPage as *mut crate::src::src::pcache::PgHdr,
            );
        }
        if rc != 0 {
            return rc;
        }
        ovflPgno = iNext;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fillInCell(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pCell: *mut ::core::ffi::c_uchar,
    pX: *const crate::src::src::btree::BtreePayload,
    pnSize: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nPayload: ::core::ffi::c_int;
    let mut pSrc: *const crate::src::ext::rtree::rtree::U8_0;
    let mut nSrc: ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    
    let mut spaceLeft: ::core::ffi::c_int;
    let mut pToRelease: *mut crate::src::headers::btreeInt_h::MemPage;
    let mut pPrior: *mut ::core::ffi::c_uchar;
    let mut pPayload: *mut ::core::ffi::c_uchar;
    
    let mut pgnoOvfl: crate::src::src::pager::Pgno;
    let mut nHeader: ::core::ffi::c_int;
    let __pPage_ref = unsafe { &mut *pPage };
    nHeader = __pPage_ref.childPtrSize as ::core::ffi::c_int;
    if __pPage_ref.intKey != 0 {
        let __pX_ref = unsafe { &*pX };
        nPayload = __pX_ref.nData + __pX_ref.nZero;
        pSrc = __pX_ref.pData as *const crate::src::ext::rtree::rtree::U8_0;
        nSrc = __pX_ref.nData;
        nHeader += (if (nPayload as crate::src::ext::rtree::rtree::U32_0)
            < 0x80 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        {
            *pCell.offset(nHeader as isize) = nPayload as ::core::ffi::c_uchar;
            1 as ::core::ffi::c_int
        } else {
            crate::src::src::util::sqlite3PutVarint(
                pCell.offset(nHeader as isize) as *mut ::core::ffi::c_uchar,
                nPayload as crate::src::ext::rtree::rtree::U64_0,
            )
        }) as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int;
        nHeader += crate::src::src::util::sqlite3PutVarint(
            pCell.offset(nHeader as isize) as *mut ::core::ffi::c_uchar,
            *(&raw const __pX_ref.nKey as *mut crate::src::ext::rtree::rtree::U64_0),
        );
    } else {
        nPayload = (*pX).nKey as ::core::ffi::c_int;
        nSrc = nPayload;
        pSrc = (*pX).pKey as *const crate::src::ext::rtree::rtree::U8_0;
        nHeader += (if (nPayload as crate::src::ext::rtree::rtree::U32_0)
            < 0x80 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        {
            *pCell.offset(nHeader as isize) = nPayload as ::core::ffi::c_uchar;
            1 as ::core::ffi::c_int
        } else {
            crate::src::src::util::sqlite3PutVarint(
                pCell.offset(nHeader as isize) as *mut ::core::ffi::c_uchar,
                nPayload as crate::src::ext::rtree::rtree::U64_0,
            )
        }) as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int;
    }
    pPayload = pCell.offset(nHeader as isize) as *mut ::core::ffi::c_uchar;
    if nPayload <= __pPage_ref.maxLocal as ::core::ffi::c_int {
        n = nHeader + nPayload;
        if n < 4 as ::core::ffi::c_int {
            n = 4 as ::core::ffi::c_int;
            *pPayload.offset(nPayload as isize) = 0 as ::core::ffi::c_uchar;
        }
        *pnSize = n;
        ::core::ptr::copy_nonoverlapping(pSrc as *const u8, pPayload as *mut u8, nSrc as usize);
        ::libc::memset(
            pPayload.offset(nSrc as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (nPayload - nSrc) as crate::__stddef_size_t_h::SizeT,
        );
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    let mn: ::core::ffi::c_int = __pPage_ref.minLocal as ::core::ffi::c_int;
    n = (mn as crate::src::ext::rtree::rtree::U32_0).wrapping_add(
        ((nPayload - mn) as crate::src::ext::rtree::rtree::U32_0).wrapping_rem(
            (*__pPage_ref.pBt)
                .usableSize
                .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0),
        ),
    ) as ::core::ffi::c_int;
    if n > __pPage_ref.maxLocal as ::core::ffi::c_int {
        n = mn;
    }
    spaceLeft = n;
    *pnSize = n + nHeader + 4 as ::core::ffi::c_int;
    pPrior = pCell.offset((nHeader + n) as isize) as *mut ::core::ffi::c_uchar;
    pToRelease = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    pgnoOvfl = 0 as crate::src::src::pager::Pgno;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __pPage_ref.pBt;
    loop {
        n = nPayload;
        if n > spaceLeft {
            n = spaceLeft;
        }
        if nSrc >= n {
            ::core::ptr::copy_nonoverlapping(pSrc as *const u8, pPayload as *mut u8, n as usize);
        } else if nSrc > 0 as ::core::ffi::c_int {
            n = nSrc;
            ::core::ptr::copy_nonoverlapping(pSrc as *const u8, pPayload as *mut u8, n as usize);
        } else {
            ::libc::memset(
                pPayload as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                n as crate::__stddef_size_t_h::SizeT,
            );
        }
        nPayload -= n;
        if nPayload <= 0 as ::core::ffi::c_int {
            break;
        }
        pPayload = pPayload.offset(n as isize);
        pSrc = pSrc.offset(n as isize);
        nSrc -= n;
        spaceLeft -= n;
        if spaceLeft == 0 as ::core::ffi::c_int {
            let mut pOvfl: *mut crate::src::headers::btreeInt_h::MemPage =
                ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
            let pgnoPtrmap: crate::src::src::pager::Pgno = pgnoOvfl;
            let __pBt_ref = unsafe { &mut *pBt };
            if __pBt_ref.autoVacuum != 0 {
                loop {
                    pgnoOvfl = pgnoOvfl.wrapping_add(1);
                    if !(ptrmapPageno(pBt, pgnoOvfl) == pgnoOvfl
                        || pgnoOvfl
                            == (crate::src::src::global::sqlite3PendingByte
                                as crate::src::ext::rtree::rtree::U32_0)
                                .wrapping_div(__pBt_ref.pageSize)
                                .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0))
                    {
                        break;
                    }
                }
            }
            rc = allocateBtreePage(
                pBt,
                &raw mut pOvfl,
                &raw mut pgnoOvfl,
                pgnoOvfl,
                0 as crate::src::ext::rtree::rtree::U8_0,
            );
            if __pBt_ref.autoVacuum as ::core::ffi::c_int != 0
                && rc == crate::src::headers::sqlite3_h::SQLITE_OK
            {
                let eType: crate::src::ext::rtree::rtree::U8_0 = (if pgnoPtrmap != 0 {
                    crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW2
                } else {
                    crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW1
                })
                    as crate::src::ext::rtree::rtree::U8_0;
                ptrmapPut(pBt, pgnoOvfl, eType, pgnoPtrmap, &raw mut rc);
                if rc != 0 {
                    releasePage(pOvfl);
                }
            }
            if rc != 0 {
                releasePage(pToRelease);
                return rc;
            }
            crate::src::src::util::sqlite3Put4byte(
                pPrior as *mut crate::src::ext::rtree::rtree::U8_0,
                pgnoOvfl as crate::src::ext::rtree::rtree::U32_0,
            );
            releasePage(pToRelease);
            pToRelease = pOvfl;
            pPrior = (*pOvfl).aData as *mut ::core::ffi::c_uchar;
            crate::src::src::util::sqlite3Put4byte(
                pPrior as *mut crate::src::ext::rtree::rtree::U8_0,
                0 as crate::src::ext::rtree::rtree::U32_0,
            );
            pPayload = (*pOvfl).aData.offset(4_isize) as *mut crate::src::ext::rtree::rtree::U8_0
                as *mut ::core::ffi::c_uchar;
            spaceLeft = __pBt_ref
                .usableSize
                .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0)
                as ::core::ffi::c_int;
        }
    }
    releasePage(pToRelease);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn dropCell(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    idx: ::core::ffi::c_int,
    sz: ::core::ffi::c_int,
    pRC: *mut ::core::ffi::c_int,
) {
    
    
    
    
    
    if *pRC != 0 {
        return;
    }
    let __pPage_ref = unsafe { &mut *pPage };
    let data: *mut crate::src::ext::rtree::rtree::U8_0 = __pPage_ref.aData;
    let ptr: *mut crate::src::ext::rtree::rtree::U8_0 = (*pPage)
        .aCellIdx
        .offset((2 as ::core::ffi::c_int * idx) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0;
    let pc: crate::src::ext::rtree::rtree::U32_0 = ((*ptr.offset(0_isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | *ptr.offset(1_isize) as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::U32_0;
    let hdr: ::core::ffi::c_int = __pPage_ref.hdrOffset as ::core::ffi::c_int;
    if pc.wrapping_add(sz as crate::src::ext::rtree::rtree::U32_0) > (*__pPage_ref.pBt).usableSize {
        *pRC = crate::src::src::main::sqlite3CorruptError(7250 as ::core::ffi::c_int);
        return;
    }
    let rc: ::core::ffi::c_int = freeSpace(pPage, pc as ::core::ffi::c_int, sz);
    if rc != 0 {
        *pRC = rc;
        return;
    }
    __pPage_ref.nCell = __pPage_ref.nCell.wrapping_sub(1);
    if __pPage_ref.nCell as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        ::libc::memset(
            data.offset((hdr + 1 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            4 as crate::__stddef_size_t_h::SizeT,
        );
        *data.offset((hdr + 7 as ::core::ffi::c_int) as isize) =
            0 as crate::src::ext::rtree::rtree::U8_0;
        *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(0_isize) = ((*__pPage_ref.pBt).usableSize >> 8 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U8_0;
        *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(1_isize) =
            (*__pPage_ref.pBt).usableSize as crate::src::ext::rtree::rtree::U8_0;
        __pPage_ref.nFree = (*__pPage_ref.pBt)
            .usableSize
            .wrapping_sub(__pPage_ref.hdrOffset as crate::src::ext::rtree::rtree::U32_0)
            .wrapping_sub(__pPage_ref.childPtrSize as crate::src::ext::rtree::rtree::U32_0)
            .wrapping_sub(8 as crate::src::ext::rtree::rtree::U32_0)
            as ::core::ffi::c_int;
    } else {
        ::core::ptr::copy(
            ptr.offset(2_isize) as *const u8,
            ptr as *mut u8,
            (2 as ::core::ffi::c_int * (__pPage_ref.nCell as ::core::ffi::c_int - idx)) as usize,
        );
        *(data.offset((hdr + 3 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(0_isize) = (__pPage_ref.nCell as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U8_0;
        *(data.offset((hdr + 3 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(1_isize) = __pPage_ref.nCell as crate::src::ext::rtree::rtree::U8_0;
        __pPage_ref.nFree += 2 as ::core::ffi::c_int;
    };
}

unsafe extern "C" fn insertCell(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    i: ::core::ffi::c_int,
    mut pCell: *mut crate::src::ext::rtree::rtree::U8_0,
    sz: ::core::ffi::c_int,
    pTemp: *mut crate::src::ext::rtree::rtree::U8_0,
    iChild: crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let j: ::core::ffi::c_int;
    let data: *mut crate::src::ext::rtree::rtree::U8_0;
    let pIns: *mut crate::src::ext::rtree::rtree::U8_0;
    if (*pPage).nOverflow as ::core::ffi::c_int != 0
        || sz + 2 as ::core::ffi::c_int > (*pPage).nFree
    {
        if !pTemp.is_null() {
            ::core::ptr::copy_nonoverlapping(pCell as *const u8, pTemp as *mut u8, sz as usize);
            pCell = pTemp;
        }
        crate::src::src::util::sqlite3Put4byte(
            pCell,
            iChild as crate::src::ext::rtree::rtree::U32_0,
        );
        let __pPage_ref = unsafe { &mut *pPage };
        let fresh22 = __pPage_ref.nOverflow;
        __pPage_ref.nOverflow = __pPage_ref.nOverflow.wrapping_add(1);
        j = fresh22 as ::core::ffi::c_int;
        __pPage_ref.apOvfl[j as usize] = pCell;
        __pPage_ref.aiOvfl[j as usize] = i as crate::src::fts5::U16_0;
    } else {
        let __pPage_ref = unsafe { &mut *pPage };
        let mut rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerWrite(
            __pPage_ref.pDbPage as *mut crate::src::src::pcache::PgHdr,
        );
        if rc != 0 as ::core::ffi::c_int {
            return rc;
        }
        data = __pPage_ref.aData;
        rc = allocateSpace(pPage, sz, &raw mut idx);
        if rc != 0 {
            return rc;
        }
        __pPage_ref.nFree -=
            (2 as ::core::ffi::c_int + sz) as crate::src::fts5::U16_0 as ::core::ffi::c_int;
        ::core::ptr::copy_nonoverlapping(
            pCell.offset(4_isize) as *const u8,
            data.offset((idx + 4 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0 as *mut u8,
            (sz - 4 as ::core::ffi::c_int) as usize,
        );
        crate::src::src::util::sqlite3Put4byte(
            data.offset(idx as isize) as *mut crate::src::ext::rtree::rtree::U8_0,
            iChild as crate::src::ext::rtree::rtree::U32_0,
        );
        pIns = (*pPage)
            .aCellIdx
            .offset((i * 2 as ::core::ffi::c_int) as isize);
        ::core::ptr::copy(
            pIns as *const u8,
            pIns.offset(2_isize) as *mut u8,
            (2 as ::core::ffi::c_int * (__pPage_ref.nCell as ::core::ffi::c_int - i)) as usize,
        );
        *pIns.offset(0_isize) =
            (idx >> 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0;
        *pIns.offset(1_isize) = idx as crate::src::ext::rtree::rtree::U8_0;
        __pPage_ref.nCell = __pPage_ref.nCell.wrapping_add(1);
        let fresh23 = &mut *data.offset(
            (__pPage_ref.hdrOffset as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *fresh23 = (*fresh23).wrapping_add(1);
        if *fresh23 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let fresh24 = &mut *data.offset(
                (__pPage_ref.hdrOffset as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
            );
            *fresh24 = (*fresh24).wrapping_add(1);
        }
        if (*__pPage_ref.pBt).autoVacuum != 0 {
            let mut rc2: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
            ptrmapPutOvflPtr(pPage, pPage, pCell, &raw mut rc2);
            if rc2 != 0 {
                return rc2;
            }
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn insertCellFast(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    i: ::core::ffi::c_int,
    pCell: *mut crate::src::ext::rtree::rtree::U8_0,
    sz: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let j: ::core::ffi::c_int;
    let data: *mut crate::src::ext::rtree::rtree::U8_0;
    let pIns: *mut crate::src::ext::rtree::rtree::U8_0;
    if sz + 2 as ::core::ffi::c_int > (*pPage).nFree {
        let __pPage_ref = unsafe { &mut *pPage };
        let fresh28 = __pPage_ref.nOverflow;
        __pPage_ref.nOverflow = __pPage_ref.nOverflow.wrapping_add(1);
        j = fresh28 as ::core::ffi::c_int;
        __pPage_ref.apOvfl[j as usize] = pCell;
        __pPage_ref.aiOvfl[j as usize] = i as crate::src::fts5::U16_0;
    } else {
        let __pPage_ref = unsafe { &mut *pPage };
        let mut rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerWrite(
            __pPage_ref.pDbPage as *mut crate::src::src::pcache::PgHdr,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        data = __pPage_ref.aData;
        rc = allocateSpace(pPage, sz, &raw mut idx);
        if rc != 0 {
            return rc;
        }
        __pPage_ref.nFree -=
            (2 as ::core::ffi::c_int + sz) as crate::src::fts5::U16_0 as ::core::ffi::c_int;
        ::core::ptr::copy_nonoverlapping(
            pCell as *const u8,
            data.offset(idx as isize) as *mut crate::src::ext::rtree::rtree::U8_0 as *mut u8,
            sz as usize,
        );
        pIns = (*pPage)
            .aCellIdx
            .offset((i * 2 as ::core::ffi::c_int) as isize);
        ::core::ptr::copy(
            pIns as *const u8,
            pIns.offset(2_isize) as *mut u8,
            (2 as ::core::ffi::c_int * (__pPage_ref.nCell as ::core::ffi::c_int - i)) as usize,
        );
        *pIns.offset(0_isize) =
            (idx >> 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0;
        *pIns.offset(1_isize) = idx as crate::src::ext::rtree::rtree::U8_0;
        __pPage_ref.nCell = __pPage_ref.nCell.wrapping_add(1);
        let fresh29 = &mut *data.offset(
            (__pPage_ref.hdrOffset as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *fresh29 = (*fresh29).wrapping_add(1);
        if *fresh29 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let fresh30 = &mut *data.offset(
                (__pPage_ref.hdrOffset as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
            );
            *fresh30 = (*fresh30).wrapping_add(1);
        }
        if (*__pPage_ref.pBt).autoVacuum != 0 {
            let mut rc2: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
            ptrmapPutOvflPtr(pPage, pPage, pCell, &raw mut rc2);
            if rc2 != 0 {
                return rc2;
            }
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

pub const NB: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

unsafe extern "C" fn populateCellCache(
    p: *mut CellArray,
    mut idx: ::core::ffi::c_int,
    mut N: ::core::ffi::c_int,
) {
    let pRef: *mut crate::src::headers::btreeInt_h::MemPage = (*p).pRef;
    let szCell: *mut crate::src::fts5::U16_0 = (*p).szCell;
    while N > 0 as ::core::ffi::c_int {
        if *szCell.offset(idx as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *szCell.offset(idx as isize) = (*pRef).xCellSize.expect("non-null function pointer")(
                pRef,
                *(*p).apCell.offset(idx as isize),
            );
        }
        idx += 1;
        N -= 1;
    }
}
#[inline(never)]
unsafe extern "C" fn computeCellSize(
    p: *mut CellArray,
    N: ::core::ffi::c_int,
) -> crate::src::fts5::U16_0 {
    let __p_ref = unsafe { &mut *p };
    *__p_ref.szCell.offset(N as isize) = (*__p_ref.pRef)
        .xCellSize
        .expect("non-null function pointer")(
        __p_ref.pRef, *__p_ref.apCell.offset(N as isize)
    );
    *__p_ref.szCell.offset(N as isize)
}

unsafe extern "C" fn cachedCellSize(
    p: *mut CellArray,
    N: ::core::ffi::c_int,
) -> crate::src::fts5::U16_0 {
    if *(*p).szCell.offset(N as isize) != 0 {
        return *(*p).szCell.offset(N as isize);
    }
    computeCellSize(p, N)
}

unsafe extern "C" fn rebuildPage(
    pCArray: *mut CellArray,
    iFirst: ::core::ffi::c_int,
    nCell: ::core::ffi::c_int,
    pPg: *mut crate::src::headers::btreeInt_h::MemPage,
) -> ::core::ffi::c_int {
    let __pPg_ref = unsafe { &mut *pPg };
    let hdr: ::core::ffi::c_int = __pPg_ref.hdrOffset as ::core::ffi::c_int;
    let aData: *mut crate::src::ext::rtree::rtree::U8_0 = __pPg_ref.aData;
    let usableSize: ::core::ffi::c_int = (*__pPg_ref.pBt).usableSize as ::core::ffi::c_int;
    let pEnd: *mut crate::src::ext::rtree::rtree::U8_0 =
        aData.offset(usableSize as isize) as *mut crate::src::ext::rtree::rtree::U8_0;
    let mut i: ::core::ffi::c_int = iFirst;
    let mut j: crate::src::ext::rtree::rtree::U32_0;
    let iEnd: ::core::ffi::c_int = i + nCell;
    let mut pCellptr: *mut crate::src::ext::rtree::rtree::U8_0 = __pPg_ref.aCellIdx;
    let pTmp: *mut crate::src::ext::rtree::rtree::U8_0 =
        crate::src::src::pager::sqlite3PagerTempSpace((*__pPg_ref.pBt).pPager)
            as *mut crate::src::ext::rtree::rtree::U8_0;
    let mut pData: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut k: ::core::ffi::c_int;
    let mut pSrcEnd: *mut crate::src::ext::rtree::rtree::U8_0;
    j = ((*(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(0_isize) as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(1_isize) as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::U32_0;
    if j > usableSize as crate::src::ext::rtree::rtree::U32_0 {
        j = 0 as crate::src::ext::rtree::rtree::U32_0;
    }
    ::core::ptr::copy_nonoverlapping(
        aData.offset(j as isize) as *mut crate::src::ext::rtree::rtree::U8_0 as *const u8,
        pTmp.offset(j as isize) as *mut crate::src::ext::rtree::rtree::U8_0 as *mut u8,
        (usableSize as crate::src::ext::rtree::rtree::U32_0).wrapping_sub(j) as usize,
    );
    k = 0 as ::core::ffi::c_int;
    while (*pCArray).ixNx[k as usize] <= i {
        k += 1;
    }
    pSrcEnd = (*pCArray).apEnd[k as usize];
    pData = pEnd;
    loop {
        let __pCArray_ref = unsafe { &mut *pCArray };
        let mut pCell: *mut crate::src::ext::rtree::rtree::U8_0 =
            *__pCArray_ref.apCell.offset(i as isize);
        let sz: crate::src::fts5::U16_0 = *__pCArray_ref.szCell.offset(i as isize);
        if pCell as crate::src::headers::sqliteInt_h::Uptr
            >= aData.offset(j as isize) as crate::src::headers::sqliteInt_h::Uptr
            && (pCell as crate::src::headers::sqliteInt_h::Uptr)
                < pEnd as crate::src::headers::sqliteInt_h::Uptr
        {
            if pCell.offset(sz as ::core::ffi::c_int as isize)
                as crate::src::headers::sqliteInt_h::Uptr
                > pEnd as crate::src::headers::sqliteInt_h::Uptr
            {
                return crate::src::src::main::sqlite3CorruptError(7640 as ::core::ffi::c_int);
            }
            pCell = pTmp.offset(pCell.offset_from(aData) as ::core::ffi::c_long as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0;
        } else if pCell.offset(sz as ::core::ffi::c_int as isize)
            as crate::src::headers::sqliteInt_h::Uptr
            > pSrcEnd as crate::src::headers::sqliteInt_h::Uptr
            && (pCell as crate::src::headers::sqliteInt_h::Uptr)
                < pSrcEnd as crate::src::headers::sqliteInt_h::Uptr
        {
            return crate::src::src::main::sqlite3CorruptError(7645 as ::core::ffi::c_int);
        }
        pData = pData.offset(-(sz as ::core::ffi::c_int as isize));
        *pCellptr.offset(0_isize) = (pData.offset_from(aData) as ::core::ffi::c_long
            >> 8 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U8_0;
        *pCellptr.offset(1_isize) =
            pData.offset_from(aData) as ::core::ffi::c_long as crate::src::ext::rtree::rtree::U8_0;
        pCellptr = pCellptr.offset(2_isize);
        if pData < pCellptr {
            return crate::src::src::main::sqlite3CorruptError(7651 as ::core::ffi::c_int);
        }
        ::core::ptr::copy(pCell as *const u8, pData as *mut u8, sz as usize);
        i += 1;
        if i >= iEnd {
            break;
        }
        if __pCArray_ref.ixNx[k as usize] <= i {
            k += 1;
            pSrcEnd = __pCArray_ref.apEnd[k as usize];
        }
    }
    __pPg_ref.nCell = nCell as crate::src::fts5::U16_0;
    __pPg_ref.nOverflow = 0 as crate::src::ext::rtree::rtree::U8_0;
    *(aData.offset((hdr + 1 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(0_isize) =
        (0 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0;
    *(aData.offset((hdr + 1 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(1_isize) = 0 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U8_0;
    *(aData.offset((hdr + 3 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(0_isize) = (__pPg_ref.nCell as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::U8_0;
    *(aData.offset((hdr + 3 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(1_isize) = __pPg_ref.nCell as crate::src::ext::rtree::rtree::U8_0;
    *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(0_isize) = (pData.offset_from(aData) as ::core::ffi::c_long
        >> 8 as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::U8_0;
    *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0)
        .offset(1_isize) =
        pData.offset_from(aData) as ::core::ffi::c_long as crate::src::ext::rtree::rtree::U8_0;
    *aData.offset((hdr + 7 as ::core::ffi::c_int) as isize) =
        0 as crate::src::ext::rtree::rtree::U8_0;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn pageInsertArray(
    pPg: *mut crate::src::headers::btreeInt_h::MemPage,
    pBegin: *mut crate::src::ext::rtree::rtree::U8_0,
    ppData: *mut *mut crate::src::ext::rtree::rtree::U8_0,
    mut pCellptr: *mut crate::src::ext::rtree::rtree::U8_0,
    iFirst: ::core::ffi::c_int,
    nCell: ::core::ffi::c_int,
    pCArray: *mut CellArray,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = iFirst;
    let aData: *mut crate::src::ext::rtree::rtree::U8_0 = (*pPg).aData;
    let mut pData: *mut crate::src::ext::rtree::rtree::U8_0 = *ppData;
    let iEnd: ::core::ffi::c_int = iFirst + nCell;
    let mut k: ::core::ffi::c_int;
    let mut pEnd: *mut crate::src::ext::rtree::rtree::U8_0;
    if iEnd <= iFirst {
        return 0 as ::core::ffi::c_int;
    }
    k = 0 as ::core::ffi::c_int;
    while (*pCArray).ixNx[k as usize] <= i {
        k += 1;
    }
    pEnd = (*pCArray).apEnd[k as usize];
    loop {
        
        let mut rc: ::core::ffi::c_int = 0;
        let mut pSlot: *mut crate::src::ext::rtree::rtree::U8_0;
        let __pCArray_ref = unsafe { &mut *pCArray };
        let sz: ::core::ffi::c_int = *__pCArray_ref.szCell.offset(i as isize) as ::core::ffi::c_int;
        if *aData.offset(1_isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && *aData.offset(2_isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || {
                pSlot = pageFindSlot(pPg, sz, &raw mut rc);
                pSlot.is_null()
            }
        {
            if (pData.offset_from(pBegin) as ::core::ffi::c_long) < sz as ::core::ffi::c_long {
                return 1 as ::core::ffi::c_int;
            }
            pData = pData.offset(-(sz as isize));
            pSlot = pData;
        }
        if (*__pCArray_ref.apCell.offset(i as isize)).offset(sz as isize)
            as crate::src::headers::sqliteInt_h::Uptr
            > pEnd as crate::src::headers::sqliteInt_h::Uptr
            && (*__pCArray_ref.apCell.offset(i as isize) as crate::src::headers::sqliteInt_h::Uptr)
                < pEnd as crate::src::headers::sqliteInt_h::Uptr
        {
            crate::src::src::main::sqlite3CorruptError(7738 as ::core::ffi::c_int);
            return 1 as ::core::ffi::c_int;
        }
        ::core::ptr::copy(
            *__pCArray_ref.apCell.offset(i as isize) as *const u8,
            pSlot as *mut u8,
            sz as usize,
        );
        *pCellptr.offset(0_isize) = (pSlot.offset_from(aData) as ::core::ffi::c_long
            >> 8 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U8_0;
        *pCellptr.offset(1_isize) =
            pSlot.offset_from(aData) as ::core::ffi::c_long as crate::src::ext::rtree::rtree::U8_0;
        pCellptr = pCellptr.offset(2_isize);
        i += 1;
        if i >= iEnd {
            break;
        }
        if __pCArray_ref.ixNx[k as usize] <= i {
            k += 1;
            pEnd = __pCArray_ref.apEnd[k as usize];
        }
    }
    *ppData = pData;
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn pageFreeArray(
    pPg: *mut crate::src::headers::btreeInt_h::MemPage,
    iFirst: ::core::ffi::c_int,
    nCell: ::core::ffi::c_int,
    pCArray: *mut CellArray,
) -> ::core::ffi::c_int {
    let __pPg_ref = unsafe { &*pPg };
    let aData: *mut crate::src::ext::rtree::rtree::U8_0 = __pPg_ref.aData;
    let pEnd: *mut crate::src::ext::rtree::rtree::U8_0 = aData
        .offset((*__pPg_ref.pBt).usableSize as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0;
    let pStart: *mut crate::src::ext::rtree::rtree::U8_0 = aData.offset(
        (__pPg_ref.hdrOffset as ::core::ffi::c_int
            + 8 as ::core::ffi::c_int
            + __pPg_ref.childPtrSize as ::core::ffi::c_int) as isize,
    )
        as *mut crate::src::ext::rtree::rtree::U8_0;
    let mut nRet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let iEnd: ::core::ffi::c_int = iFirst + nCell;
    let mut nFree: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut aOfst: [::core::ffi::c_int; 10] = [0; 10];
    let mut aAfter: [::core::ffi::c_int; 10] = [0; 10];
    i = iFirst;
    while i < iEnd {
        let pCell: *mut crate::src::ext::rtree::rtree::U8_0 =
            *(*pCArray).apCell.offset(i as isize);
        if pCell as crate::src::headers::sqliteInt_h::Uptr
            >= pStart as crate::src::headers::sqliteInt_h::Uptr
            && (pCell as crate::src::headers::sqliteInt_h::Uptr)
                < pEnd as crate::src::headers::sqliteInt_h::Uptr
        {
            
            
            
            let sz: ::core::ffi::c_int = *(*pCArray).szCell.offset(i as isize) as ::core::ffi::c_int;
            let iOfst: ::core::ffi::c_int = pCell.offset_from(aData) as ::core::ffi::c_long as crate::src::fts5::U16_0
                as ::core::ffi::c_int;
            let iAfter: ::core::ffi::c_int = iOfst + sz;
            j = 0 as ::core::ffi::c_int;
            while j < nFree {
                if aOfst[j as usize] == iAfter {
                    aOfst[j as usize] = iOfst;
                    break;
                } else if aAfter[j as usize] == iOfst {
                    aAfter[j as usize] = iAfter;
                    break;
                } else {
                    j += 1;
                }
            }
            if j >= nFree {
                if nFree
                    >= (::core::mem::size_of::<[::core::ffi::c_int; 10]>() as usize)
                        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        as ::core::ffi::c_int
                {
                    j = 0 as ::core::ffi::c_int;
                    while j < nFree {
                        freeSpace(
                            pPg,
                            aOfst[j as usize],
                            aAfter[j as usize] - aOfst[j as usize],
                        );
                        j += 1;
                    }
                    nFree = 0 as ::core::ffi::c_int;
                }
                aOfst[nFree as usize] = iOfst;
                aAfter[nFree as usize] = iAfter;
                if aData.offset(iAfter as isize) as *mut crate::src::ext::rtree::rtree::U8_0 > pEnd
                {
                    return 0 as ::core::ffi::c_int;
                }
                nFree += 1;
            }
            nRet += 1;
        }
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < nFree {
        freeSpace(
            pPg,
            aOfst[j as usize],
            aAfter[j as usize] - aOfst[j as usize],
        );
        j += 1;
    }
    nRet
}

unsafe extern "C" fn editPage(
    pPg: *mut crate::src::headers::btreeInt_h::MemPage,
    iOld: ::core::ffi::c_int,
    iNew: ::core::ffi::c_int,
    nNew: ::core::ffi::c_int,
    pCArray: *mut CellArray,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let __pPg_ref = unsafe { &mut *pPg };
    let aData: *mut crate::src::ext::rtree::rtree::U8_0 = __pPg_ref.aData;
    let hdr: ::core::ffi::c_int = __pPg_ref.hdrOffset as ::core::ffi::c_int;
    let pBegin: *mut crate::src::ext::rtree::rtree::U8_0 = (*pPg)
        .aCellIdx
        .offset((nNew * 2 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0;
    let mut nCell: ::core::ffi::c_int = __pPg_ref.nCell as ::core::ffi::c_int;
    let mut pData: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut pCellptr: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut i: ::core::ffi::c_int;
    let iOldEnd: ::core::ffi::c_int =
        iOld + __pPg_ref.nCell as ::core::ffi::c_int + __pPg_ref.nOverflow as ::core::ffi::c_int;
    let iNewEnd: ::core::ffi::c_int = iNew + nNew;
    if iOld < iNew {
        let nShift: ::core::ffi::c_int = pageFreeArray(pPg, iOld, iNew - iOld, pCArray);
        if nShift > nCell {
            return crate::src::src::main::sqlite3CorruptError(7860 as ::core::ffi::c_int);
        }
        ::core::ptr::copy(
            (*pPg)
                .aCellIdx
                .offset((nShift * 2 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0 as *const u8,
            __pPg_ref.aCellIdx as *mut u8,
            (nCell * 2 as ::core::ffi::c_int) as usize,
        );
        nCell -= nShift;
    }
    if iNewEnd < iOldEnd {
        let nTail: ::core::ffi::c_int = pageFreeArray(pPg, iNewEnd, iOldEnd - iNewEnd, pCArray);
        nCell -= nTail;
    }
    pData = aData.offset(
        ((*(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(0_isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0)
                .offset(1_isize) as ::core::ffi::c_int) as isize,
    ) as *mut crate::src::ext::rtree::rtree::U8_0;
    if (pData >= pBegin)
        && (pData <= __pPg_ref.aDataEnd) {
            if iNew < iOld {
                let nAdd: ::core::ffi::c_int = if nNew < iOld - iNew {
                    nNew
                } else {
                    iOld - iNew
                };
                pCellptr = __pPg_ref.aCellIdx;
                ::core::ptr::copy(
                    pCellptr as *const u8,
                    pCellptr.offset((nAdd * 2 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0 as *mut u8,
                    (nCell * 2 as ::core::ffi::c_int) as usize,
                );
                if pageInsertArray(pPg, pBegin, &raw mut pData, pCellptr, iNew, nAdd, pCArray) != 0
                {
                    current_block = 4831265723658566639;
                } else {
                    nCell += nAdd;
                    current_block = 12147880666119273379;
                }
            } else {
                current_block = 12147880666119273379;
            }
            match current_block {
                4831265723658566639 => {}
                _ => {
                    i = 0 as ::core::ffi::c_int;
                    loop {
                        if (i >= __pPg_ref.nOverflow as ::core::ffi::c_int) {
                            current_block = 11459959175219260272;
                            break;
                        }
                        let iCell: ::core::ffi::c_int =
                            iOld + __pPg_ref.aiOvfl[i as usize] as ::core::ffi::c_int - iNew;
                        if iCell >= 0 as ::core::ffi::c_int && iCell < nNew {
                            pCellptr = (*pPg)
                                .aCellIdx
                                .offset((iCell * 2 as ::core::ffi::c_int) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0;
                            if nCell > iCell {
                                ::core::ptr::copy(
                                    pCellptr as *const u8,
                                    pCellptr.offset(2_isize)
                                        as *mut crate::src::ext::rtree::rtree::U8_0
                                        as *mut u8,
                                    ((nCell - iCell) * 2 as ::core::ffi::c_int) as usize,
                                );
                            }
                            nCell += 1;
                            cachedCellSize(pCArray, iCell + iNew);
                            if pageInsertArray(
                                pPg,
                                pBegin,
                                &raw mut pData,
                                pCellptr,
                                iCell + iNew,
                                1 as ::core::ffi::c_int,
                                pCArray,
                            ) != 0
                            {
                                current_block = 4831265723658566639;
                                break;
                            }
                        }
                        i += 1;
                    }
                    match current_block {
                        4831265723658566639 => {}
                        _ => {
                            pCellptr = (*pPg)
                                .aCellIdx
                                .offset((nCell * 2 as ::core::ffi::c_int) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0;
                            if (pageInsertArray(
                                pPg,
                                pBegin,
                                &raw mut pData,
                                pCellptr,
                                iNew + nCell,
                                nNew - nCell,
                                pCArray,
                            ) == 0)
                            {
                                __pPg_ref.nCell = nNew as crate::src::fts5::U16_0;
                                __pPg_ref.nOverflow = 0 as crate::src::ext::rtree::rtree::U8_0;
                                *(aData.offset((hdr + 3 as ::core::ffi::c_int) as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0)
                                    .offset(0_isize) = (__pPg_ref.nCell as ::core::ffi::c_int
                                    >> 8 as ::core::ffi::c_int)
                                    as crate::src::ext::rtree::rtree::U8_0;
                                *(aData.offset((hdr + 3 as ::core::ffi::c_int) as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0)
                                    .offset(1_isize) =
                                    __pPg_ref.nCell as crate::src::ext::rtree::rtree::U8_0;
                                *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0)
                                    .offset(0_isize) = (pData.offset_from(aData)
                                    as ::core::ffi::c_long
                                    >> 8 as ::core::ffi::c_int)
                                    as crate::src::ext::rtree::rtree::U8_0;
                                *(aData.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0)
                                    .offset(1_isize) = pData.offset_from(aData)
                                    as ::core::ffi::c_long
                                    as crate::src::ext::rtree::rtree::U8_0;
                                return crate::src::headers::sqlite3_h::SQLITE_OK;
                            }
                        }
                    }
                }
            }
        }
    if nNew < 1 as ::core::ffi::c_int {
        return crate::src::src::main::sqlite3CorruptError(7938 as ::core::ffi::c_int);
    }
    populateCellCache(pCArray, iNew, nNew);
    rebuildPage(pCArray, iNew, nNew, pPg)
}

unsafe extern "C" fn balance_quick(
    pParent: *mut crate::src::headers::btreeInt_h::MemPage,
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pSpace: *mut crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*pPage).pBt;
    let mut pNew: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let mut rc: ::core::ffi::c_int;
    let mut pgnoNew: crate::src::src::pager::Pgno = 0;
    if (*pPage).nCell as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::src::src::main::sqlite3CorruptError(7978 as ::core::ffi::c_int);
    }
    rc = allocateBtreePage(
        pBt,
        &raw mut pNew,
        &raw mut pgnoNew,
        0 as crate::src::src::pager::Pgno,
        0 as crate::src::ext::rtree::rtree::U8_0,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut pOut: *mut crate::src::ext::rtree::rtree::U8_0 =
            pSpace.offset(4_isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        let __pPage_ref = unsafe { &mut *pPage };
        let mut pCell: *mut crate::src::ext::rtree::rtree::U8_0 =
            __pPage_ref.apOvfl[0 as ::core::ffi::c_int as usize];
        let mut szCell: crate::src::fts5::U16_0 =
            __pPage_ref.xCellSize.expect("non-null function pointer")(pPage, pCell);
        let mut pStop: *mut crate::src::ext::rtree::rtree::U8_0;
        let mut b: CellArray = unsafe { ::core::mem::zeroed() };
        zeroPage(
            pNew,
            crate::src::headers::btreeInt_h::PTF_INTKEY
                | crate::src::headers::btreeInt_h::PTF_LEAFDATA
                | crate::src::headers::btreeInt_h::PTF_LEAF,
        );
        b.nCell = 1 as ::core::ffi::c_int;
        b.pRef = pPage;
        b.apCell = &raw mut pCell;
        b.szCell = &raw mut szCell;
        b.apEnd[0 as ::core::ffi::c_int as usize] = __pPage_ref.aDataEnd;
        b.ixNx[0 as ::core::ffi::c_int as usize] = 2 as ::core::ffi::c_int;
        b.ixNx[(NB * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] =
            0x7fffffff as ::core::ffi::c_int;
        rc = rebuildPage(
            &raw mut b,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            pNew,
        );
        if rc != 0 {
            releasePage(pNew);
            return rc;
        }
        (*pNew).nFree = (*pBt)
            .usableSize
            .wrapping_sub((*pNew).cellOffset as crate::src::ext::rtree::rtree::U32_0)
            .wrapping_sub(2 as crate::src::ext::rtree::rtree::U32_0)
            .wrapping_sub(szCell as crate::src::ext::rtree::rtree::U32_0)
            as ::core::ffi::c_int;
        if (*pBt).autoVacuum != 0 {
            ptrmapPut(
                pBt,
                pgnoNew,
                crate::src::headers::btreeInt_h::PTRMAP_BTREE
                    as crate::src::ext::rtree::rtree::U8_0,
                (*pParent).pgno,
                &raw mut rc,
            );
            if szCell as ::core::ffi::c_int > (*pNew).minLocal as ::core::ffi::c_int {
                ptrmapPutOvflPtr(pNew, pNew, pCell, &raw mut rc);
            }
        }
        pCell = __pPage_ref.aData.offset(
            (__pPage_ref.maskPage as ::core::ffi::c_int
                & ((*(__pPage_ref.aCellIdx.offset(
                    (2 as ::core::ffi::c_int
                        * (__pPage_ref.nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                        as isize,
                ) as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(0_isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(__pPage_ref.aCellIdx.offset(
                        (2 as ::core::ffi::c_int
                            * (__pPage_ref.nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                            as isize,
                    ) as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(1_isize) as ::core::ffi::c_int)) as isize,
        );
        pStop = pCell.offset(9_isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        loop {
            let fresh25 = pCell;
            pCell = pCell.offset(1);
            if !(*fresh25 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 && pCell < pStop)
            {
                break;
            }
        }
        pStop = pCell.offset(9_isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        loop {
            let fresh26 = pCell;
            pCell = pCell.offset(1);
            let fresh27 = pOut;
            pOut = pOut.offset(1);
            *fresh27 = *fresh26;
            if !(*fresh27 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 && pCell < pStop)
            {
                break;
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = insertCell(
                pParent,
                (*pParent).nCell as ::core::ffi::c_int,
                pSpace,
                pOut.offset_from(pSpace) as ::core::ffi::c_long as ::core::ffi::c_int,
                ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
                __pPage_ref.pgno,
            );
        }
        crate::src::src::util::sqlite3Put4byte(
            (*pParent).aData.offset(
                ((*pParent).hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::ext::rtree::rtree::U8_0,
            pgnoNew as crate::src::ext::rtree::rtree::U32_0,
        );
        releasePage(pNew);
    }
    rc
}

unsafe extern "C" fn copyNodeContent(
    pFrom: *mut crate::src::headers::btreeInt_h::MemPage,
    pTo: *mut crate::src::headers::btreeInt_h::MemPage,
    pRC: *mut ::core::ffi::c_int,
) {
    if *pRC == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pFrom_ref = unsafe { &*pFrom };
        let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __pFrom_ref.pBt;
        let aFrom: *mut crate::src::ext::rtree::rtree::U8_0 = __pFrom_ref.aData;
        let __pTo_ref = unsafe { &mut *pTo };
        let aTo: *mut crate::src::ext::rtree::rtree::U8_0 = __pTo_ref.aData;
        let iFromHdr: ::core::ffi::c_int = __pFrom_ref.hdrOffset as ::core::ffi::c_int;
        let iToHdr: ::core::ffi::c_int = if __pTo_ref.pgno == 1 as crate::src::src::pager::Pgno {
            100 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut rc: ::core::ffi::c_int;
        
        let iData: ::core::ffi::c_int = (*(aFrom.offset((iFromHdr + 5 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0)
            .offset(0_isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *(aFrom.offset((iFromHdr + 5 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0)
                .offset(1_isize) as ::core::ffi::c_int;
        ::core::ptr::copy_nonoverlapping(
            aFrom.offset(iData as isize) as *mut crate::src::ext::rtree::rtree::U8_0 as *const u8,
            aTo.offset(iData as isize) as *mut crate::src::ext::rtree::rtree::U8_0 as *mut u8,
            (*pBt)
                .usableSize
                .wrapping_sub(iData as crate::src::ext::rtree::rtree::U32_0) as usize,
        );
        ::core::ptr::copy_nonoverlapping(
            aFrom.offset(iFromHdr as isize) as *mut crate::src::ext::rtree::rtree::U8_0
                as *const u8,
            aTo.offset(iToHdr as isize) as *mut crate::src::ext::rtree::rtree::U8_0 as *mut u8,
            (__pFrom_ref.cellOffset as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * __pFrom_ref.nCell as ::core::ffi::c_int)
                as usize,
        );
        __pTo_ref.isInit = 0 as crate::src::ext::rtree::rtree::U8_0;
        rc = btreeInitPage(pTo);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = btreeComputeFreeSpace(pTo);
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            *pRC = rc;
            return;
        }
        if (*pBt).autoVacuum != 0 {
            *pRC = setChildPtrmaps(pTo);
        }
    }
}

unsafe extern "C" fn balance_nonroot(
    pParent: *mut crate::src::headers::btreeInt_h::MemPage,
    iParentIdx: ::core::ffi::c_int,
    aOvflSpace: *mut crate::src::ext::rtree::rtree::U8_0,
    isRoot: ::core::ffi::c_int,
    bBulk: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    
    let mut nMaxCells: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nNew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int;
    let nxDiv: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let leafCorrection: crate::src::fts5::U16_0;
    let leafData: ::core::ffi::c_int;
    let usableSpace: ::core::ffi::c_int;
    let pageFlags: ::core::ffi::c_int;
    let mut iSpace1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iOvflSpace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let szScratch: crate::src::ext::rtree::rtree::U64_0;
    let mut apOld: [*mut crate::src::headers::btreeInt_h::MemPage; 3] =
        unsafe { ::core::mem::zeroed() };
    let mut apNew: [*mut crate::src::headers::btreeInt_h::MemPage; 5] =
        [::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>(); 5];
    let pRight: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut apDiv: [*mut crate::src::ext::rtree::rtree::U8_0; 2] =
        [::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(); 2];
    let mut cntNew: [::core::ffi::c_int; 5] = [0; 5];
    let mut cntOld: [::core::ffi::c_int; 5] = [0; 5];
    let mut szNew: [::core::ffi::c_int; 5] = [0; 5];
    let aSpace1: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut pgno: crate::src::src::pager::Pgno;
    let mut abDone: [crate::src::ext::rtree::rtree::U8_0; 5] = unsafe { ::core::mem::zeroed() };
    let mut aPgno: [crate::src::src::pager::Pgno; 5] = [0; 5];
    let mut b: CellArray = unsafe { ::core::mem::zeroed() };
    b.ixNx[(NB * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] =
        0x7fffffff as ::core::ffi::c_int;
    let __pParent_ref = unsafe { &mut *pParent };
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __pParent_ref.pBt;
    if aOvflSpace.is_null() {
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    i = __pParent_ref.nOverflow as ::core::ffi::c_int + __pParent_ref.nCell as ::core::ffi::c_int;
    if i < 2 as ::core::ffi::c_int {
        nxDiv = 0 as ::core::ffi::c_int;
    } else {
        if iParentIdx == 0 as ::core::ffi::c_int {
            nxDiv = 0 as ::core::ffi::c_int;
        } else if iParentIdx == i {
            nxDiv = i - 2 as ::core::ffi::c_int + bBulk;
        } else {
            nxDiv = iParentIdx - 1 as ::core::ffi::c_int;
        }
        i = 2 as ::core::ffi::c_int - bBulk;
    }
    let nOld: ::core::ffi::c_int = i + 1 as ::core::ffi::c_int;
    if i + nxDiv - __pParent_ref.nOverflow as ::core::ffi::c_int
        == __pParent_ref.nCell as ::core::ffi::c_int
    {
        pRight = (*pParent).aData.offset(
            (__pParent_ref.hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::ext::rtree::rtree::U8_0;
    } else {
        pRight = __pParent_ref.aData.offset(
            (__pParent_ref.maskPage as ::core::ffi::c_int
                & ((*(__pParent_ref.aCellIdx.offset(
                    (2 as ::core::ffi::c_int
                        * (i + nxDiv - __pParent_ref.nOverflow as ::core::ffi::c_int))
                        as isize,
                ) as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(0_isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(__pParent_ref.aCellIdx.offset(
                        (2 as ::core::ffi::c_int
                            * (i + nxDiv - __pParent_ref.nOverflow as ::core::ffi::c_int))
                            as isize,
                    ) as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(1_isize) as ::core::ffi::c_int)) as isize,
        );
    }
    pgno = crate::src::src::util::sqlite3Get4byte(pRight) as crate::src::src::pager::Pgno;
    loop {
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = getAndInitPage(
                pBt,
                pgno,
                (&raw mut apOld as *mut *mut crate::src::headers::btreeInt_h::MemPage)
                    .offset(i as isize)
                    as *mut *mut crate::src::headers::btreeInt_h::MemPage,
                0 as ::core::ffi::c_int,
            );
        }
        if rc != 0 {
            current_block = 4198108429590484834;
            break;
        } else {
            if (*apOld[i as usize]).nFree < 0 as ::core::ffi::c_int {
                rc = btreeComputeFreeSpace(apOld[i as usize]);
                if rc != 0 {
                    ::libc::memset(
                        &raw mut apOld as *mut *mut crate::src::headers::btreeInt_h::MemPage
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (i as crate::__stddef_size_t_h::SizeT).wrapping_mul(
                            ::core::mem::size_of::<*mut crate::src::headers::btreeInt_h::MemPage>()
                                as crate::__stddef_size_t_h::SizeT,
                        ),
                    );
                    current_block = 4198108429590484834;
                    break;
                }
            }
            nMaxCells += (*apOld[i as usize]).nCell as ::core::ffi::c_int
                + (::core::mem::size_of::<[*mut crate::src::ext::rtree::rtree::U8_0; 4]>() as usize)
                    .wrapping_div(
                        ::core::mem::size_of::<*mut crate::src::ext::rtree::rtree::U8_0>() as usize,
                    ) as ::core::ffi::c_int;
            let fresh13 = i;
            i -= 1;
            if fresh13 == 0 as ::core::ffi::c_int {
                current_block = 13303144130133872306;
                break;
            }
            if __pParent_ref.nOverflow as ::core::ffi::c_int != 0
                && i + nxDiv
                    == __pParent_ref.aiOvfl[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            {
                apDiv[i as usize] = __pParent_ref.apOvfl[0 as ::core::ffi::c_int as usize];
                pgno = crate::src::src::util::sqlite3Get4byte(apDiv[i as usize])
                    as crate::src::src::pager::Pgno;
                szNew[i as usize] = __pParent_ref.xCellSize.expect("non-null function pointer")(
                    pParent,
                    apDiv[i as usize],
                ) as ::core::ffi::c_int;
                __pParent_ref.nOverflow = 0 as crate::src::ext::rtree::rtree::U8_0;
            } else {
                apDiv[i as usize] = __pParent_ref.aData.offset(
                    (__pParent_ref.maskPage as ::core::ffi::c_int
                        & ((*(__pParent_ref.aCellIdx.offset(
                            (2 as ::core::ffi::c_int
                                * (i + nxDiv - __pParent_ref.nOverflow as ::core::ffi::c_int))
                                as isize,
                        ) as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(0_isize) as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *(__pParent_ref.aCellIdx.offset(
                                (2 as ::core::ffi::c_int
                                    * (i + nxDiv - __pParent_ref.nOverflow as ::core::ffi::c_int))
                                    as isize,
                            )
                                as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(1_isize)
                                as ::core::ffi::c_int)) as isize,
                );
                pgno = crate::src::src::util::sqlite3Get4byte(apDiv[i as usize])
                    as crate::src::src::pager::Pgno;
                szNew[i as usize] = __pParent_ref.xCellSize.expect("non-null function pointer")(
                    pParent,
                    apDiv[i as usize],
                ) as ::core::ffi::c_int;
                if (*pBt).btsFlags as ::core::ffi::c_int
                    & crate::src::headers::btreeInt_h::BTS_FAST_SECURE
                    != 0
                {
                    
                    let iOff: ::core::ffi::c_int = apDiv[i as usize] as crate::src::headers::stdlib::IntptrT
                        as ::core::ffi::c_int
                        - __pParent_ref.aData as crate::src::headers::stdlib::IntptrT
                            as ::core::ffi::c_int;
                    if iOff + szNew[i as usize] <= (*pBt).usableSize as ::core::ffi::c_int {
                        ::core::ptr::copy_nonoverlapping(
                            apDiv[i as usize] as *const u8,
                            aOvflSpace.offset(iOff as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0
                                as *mut u8,
                            szNew[i as usize] as usize,
                        );
                        apDiv[i as usize] = aOvflSpace.offset(
                            (*(&raw mut apDiv as *mut *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(i as isize))
                            .offset_from(__pParent_ref.aData)
                                as ::core::ffi::c_long as isize,
                        )
                            as *mut crate::src::ext::rtree::rtree::U8_0;
                    }
                }
                dropCell(
                    pParent,
                    i + nxDiv - __pParent_ref.nOverflow as ::core::ffi::c_int,
                    szNew[i as usize],
                    &raw mut rc,
                );
            }
        }
    }
    if current_block == 13303144130133872306 {
        nMaxCells = (nMaxCells + 3 as ::core::ffi::c_int) & !(3 as ::core::ffi::c_int);
        szScratch = (nMaxCells as usize)
            .wrapping_mul(
                ::core::mem::size_of::<*mut crate::src::ext::rtree::rtree::U8_0>() as usize,
            )
            .wrapping_add(
                (nMaxCells as usize)
                    .wrapping_mul(::core::mem::size_of::<crate::src::fts5::U16_0>() as usize),
            )
            .wrapping_add((*pBt).pageSize as usize)
            as crate::src::ext::rtree::rtree::U64_0;
        b.apCell = crate::src::src::malloc::sqlite3DbMallocRaw(
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>()
                as *mut crate::src::headers::sqliteInt_h::sqlite3,
            szScratch,
        ) as *mut *mut crate::src::ext::rtree::rtree::U8_0;
        if b.apCell.is_null() {
            rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        } else {
            b.szCell = b.apCell.offset(nMaxCells as isize)
                as *mut *mut crate::src::ext::rtree::rtree::U8_0
                as *mut crate::src::fts5::U16_0;
            aSpace1 = b.szCell.offset(nMaxCells as isize) as *mut crate::src::fts5::U16_0
                as *mut crate::src::ext::rtree::rtree::U8_0;
            b.pRef = apOld[0 as ::core::ffi::c_int as usize];
            leafCorrection = ((*b.pRef).leaf as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as crate::src::fts5::U16_0;
            leafData = (*b.pRef).intKeyLeaf as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            loop {
                if (i >= nOld) {
                    current_block = 14953815020842398287;
                    break;
                }
                let pOld: *mut crate::src::headers::btreeInt_h::MemPage = apOld[i as usize];
                let __pOld_ref = unsafe { &mut *pOld };
                let mut limit: ::core::ffi::c_int = __pOld_ref.nCell as ::core::ffi::c_int;
                let aData: *mut crate::src::ext::rtree::rtree::U8_0 = __pOld_ref.aData;
                let maskPage: crate::src::fts5::U16_0 = __pOld_ref.maskPage;
                let mut piCell: *mut crate::src::ext::rtree::rtree::U8_0 =
                    aData.offset(__pOld_ref.cellOffset as ::core::ffi::c_int as isize);
                let piEnd: *mut crate::src::ext::rtree::rtree::U8_0;
                if *__pOld_ref.aData.offset(0_isize) as ::core::ffi::c_int
                    != *(*apOld[0 as ::core::ffi::c_int as usize])
                        .aData
                        .offset(0_isize) as ::core::ffi::c_int
                {
                    rc = crate::src::src::main::sqlite3CorruptError(8402 as ::core::ffi::c_int);
                    current_block = 4198108429590484834;
                    break;
                } else {
                    ::libc::memset(
                        b.szCell.offset(b.nCell as isize) as *mut crate::src::fts5::U16_0
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (::core::mem::size_of::<crate::src::fts5::U16_0>()
                            as crate::__stddef_size_t_h::SizeT)
                            .wrapping_mul(
                                (limit + __pOld_ref.nOverflow as ::core::ffi::c_int)
                                    as crate::__stddef_size_t_h::SizeT,
                            ),
                    );
                    if __pOld_ref.nOverflow as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        if limit
                            < __pOld_ref.aiOvfl[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                        {
                            rc = crate::src::src::main::sqlite3CorruptError(
                                8426 as ::core::ffi::c_int,
                            );
                            current_block = 4198108429590484834;
                            break;
                        } else {
                            limit = __pOld_ref.aiOvfl[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int;
                            j = 0 as ::core::ffi::c_int;
                            while j < limit {
                                let fresh14 = &mut *b.apCell.offset(b.nCell as isize);
                                *fresh14 = aData.offset(
                                    (maskPage as ::core::ffi::c_int
                                        & ((*piCell.offset(0_isize) as ::core::ffi::c_int)
                                            << 8 as ::core::ffi::c_int
                                            | *piCell.offset(1_isize) as ::core::ffi::c_int))
                                        as isize,
                                );
                                piCell = piCell.offset(2_isize);
                                b.nCell += 1;
                                j += 1;
                            }
                            k = 0 as ::core::ffi::c_int;
                            while k < __pOld_ref.nOverflow as ::core::ffi::c_int {
                                let fresh15 = &mut *b.apCell.offset(b.nCell as isize);
                                *fresh15 = __pOld_ref.apOvfl[k as usize];
                                b.nCell += 1;
                                k += 1;
                            }
                        }
                    }
                    piEnd = aData
                        .offset(__pOld_ref.cellOffset as ::core::ffi::c_int as isize)
                        .offset(
                            (2 as ::core::ffi::c_int * __pOld_ref.nCell as ::core::ffi::c_int)
                                as isize,
                        );
                    while piCell < piEnd {
                        let fresh16 = &mut *b.apCell.offset(b.nCell as isize);
                        *fresh16 = aData.offset(
                            (maskPage as ::core::ffi::c_int
                                & ((*piCell.offset(0_isize) as ::core::ffi::c_int)
                                    << 8 as ::core::ffi::c_int
                                    | *piCell.offset(1_isize) as ::core::ffi::c_int))
                                as isize,
                        );
                        piCell = piCell.offset(2_isize);
                        b.nCell += 1;
                    }
                    cntOld[i as usize] = b.nCell;
                    if i < nOld - 1 as ::core::ffi::c_int && leafData == 0 {
                        let sz: crate::src::fts5::U16_0 =
                            szNew[i as usize] as crate::src::fts5::U16_0;
                        
                        *b.szCell.offset(b.nCell as isize) = sz;
                        let pTemp: *mut crate::src::ext::rtree::rtree::U8_0 = aSpace1.offset(iSpace1 as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0;
                        iSpace1 += sz as ::core::ffi::c_int;
                        ::core::ptr::copy_nonoverlapping(
                            apDiv[i as usize] as *const u8,
                            pTemp as *mut u8,
                            sz as usize,
                        );
                        let fresh17 = &mut *b.apCell.offset(b.nCell as isize);
                        *fresh17 = pTemp.offset(leafCorrection as ::core::ffi::c_int as isize);
                        *b.szCell.offset(b.nCell as isize) = (*b.szCell.offset(b.nCell as isize)
                            as ::core::ffi::c_int
                            - leafCorrection as ::core::ffi::c_int)
                            as crate::src::fts5::U16_0;
                        if __pOld_ref.leaf == 0 {
                            ::core::ptr::copy_nonoverlapping(
                                __pOld_ref.aData.offset(8_isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0
                                    as *const u8,
                                *b.apCell.offset(b.nCell as isize) as *mut u8,
                                4_usize,
                            );
                        } else {
                            while (*b.szCell.offset(b.nCell as isize) as ::core::ffi::c_int)
                                < 4 as ::core::ffi::c_int
                            {
                                let fresh18 = iSpace1;
                                iSpace1 += 1;
                                *aSpace1.offset(fresh18 as isize) =
                                    0 as crate::src::ext::rtree::rtree::U8_0;
                                let fresh19 = &mut *b.szCell.offset(b.nCell as isize);
                                *fresh19 = (*fresh19).wrapping_add(1);
                            }
                        }
                        b.nCell += 1;
                    }
                    i += 1;
                }
            }
            match current_block {
                4198108429590484834 => {}
                _ => {
                    usableSpace = (*pBt)
                        .usableSize
                        .wrapping_sub(12 as crate::src::ext::rtree::rtree::U32_0)
                        .wrapping_add(leafCorrection as crate::src::ext::rtree::rtree::U32_0)
                        as ::core::ffi::c_int;
                    k = 0 as ::core::ffi::c_int;
                    i = k;
                    while i < nOld {
                        let p: *mut crate::src::headers::btreeInt_h::MemPage =
                            apOld[i as usize];
                        let __p_ref = unsafe { &mut *p };
                        b.apEnd[k as usize] = __p_ref.aDataEnd;
                        b.ixNx[k as usize] = cntOld[i as usize];
                        if k != 0
                            && b.ixNx[k as usize]
                                == b.ixNx[(k - 1 as ::core::ffi::c_int) as usize]
                        {
                            k -= 1;
                        }
                        if leafData == 0 {
                            k += 1;
                            b.apEnd[k as usize] = __pParent_ref.aDataEnd;
                            b.ixNx[k as usize] = cntOld[i as usize] + 1 as ::core::ffi::c_int;
                        }
                        szNew[i as usize] = usableSpace - __p_ref.nFree;
                        j = 0 as ::core::ffi::c_int;
                        while j < __p_ref.nOverflow as ::core::ffi::c_int {
                            szNew[i as usize] += 2 as ::core::ffi::c_int
                                + __p_ref.xCellSize.expect("non-null function pointer")(
                                    p,
                                    __p_ref.apOvfl[j as usize],
                                ) as ::core::ffi::c_int;
                            j += 1;
                        }
                        cntNew[i as usize] = cntOld[i as usize];
                        i += 1;
                        k += 1;
                    }
                    k = nOld;
                    i = 0 as ::core::ffi::c_int;
                    's_661: loop {
                        if (i >= k) {
                            current_block = 4183419379601546972;
                            break;
                        }
                        let mut sz_0: ::core::ffi::c_int;
                        while szNew[i as usize] > usableSpace {
                            if i + 1 as ::core::ffi::c_int >= k {
                                k = i + 2 as ::core::ffi::c_int;
                                if k > NB + 2 as ::core::ffi::c_int {
                                    rc = crate::src::src::main::sqlite3CorruptError(
                                        8527 as ::core::ffi::c_int,
                                    );
                                    current_block = 4198108429590484834;
                                    break 's_661;
                                } else {
                                    szNew[(k - 1 as ::core::ffi::c_int) as usize] =
                                        0 as ::core::ffi::c_int;
                                    cntNew[(k - 1 as ::core::ffi::c_int) as usize] = b.nCell;
                                }
                            }
                            sz_0 = 2 as ::core::ffi::c_int
                                + cachedCellSize(
                                    &raw mut b,
                                    cntNew[i as usize] - 1 as ::core::ffi::c_int,
                                ) as ::core::ffi::c_int;
                            szNew[i as usize] -= sz_0;
                            if leafData == 0 {
                                if cntNew[i as usize] < b.nCell {
                                    sz_0 = 2 as ::core::ffi::c_int
                                        + cachedCellSize(&raw mut b, cntNew[i as usize])
                                            as ::core::ffi::c_int;
                                } else {
                                    sz_0 = 0 as ::core::ffi::c_int;
                                }
                            }
                            szNew[(i + 1 as ::core::ffi::c_int) as usize] += sz_0;
                            cntNew[i as usize] -= 1;
                        }
                        while cntNew[i as usize] < b.nCell {
                            sz_0 = 2 as ::core::ffi::c_int
                                + cachedCellSize(&raw mut b, cntNew[i as usize])
                                    as ::core::ffi::c_int;
                            if szNew[i as usize] + sz_0 > usableSpace {
                                break;
                            }
                            szNew[i as usize] += sz_0;
                            cntNew[i as usize] += 1;
                            if leafData == 0 {
                                if cntNew[i as usize] < b.nCell {
                                    sz_0 = 2 as ::core::ffi::c_int
                                        + cachedCellSize(&raw mut b, cntNew[i as usize])
                                            as ::core::ffi::c_int;
                                } else {
                                    sz_0 = 0 as ::core::ffi::c_int;
                                }
                            }
                            szNew[(i + 1 as ::core::ffi::c_int) as usize] -= sz_0;
                        }
                        if cntNew[i as usize] >= b.nCell {
                            k = i + 1 as ::core::ffi::c_int;
                        } else if cntNew[i as usize]
                            <= (if i > 0 as ::core::ffi::c_int {
                                cntNew[(i - 1 as ::core::ffi::c_int) as usize]
                            } else {
                                0 as ::core::ffi::c_int
                            })
                        {
                            rc = crate::src::src::main::sqlite3CorruptError(
                                8560 as ::core::ffi::c_int,
                            );
                            current_block = 4198108429590484834;
                            break;
                        }
                        i += 1;
                    }
                    match current_block {
                        4198108429590484834 => {}
                        _ => {
                            i = k - 1 as ::core::ffi::c_int;
                            loop {
                                if (i <= 0 as ::core::ffi::c_int) {
                                    current_block = 18312853480645871422;
                                    break;
                                }
                                let mut szRight: ::core::ffi::c_int = szNew[i as usize];
                                let mut szLeft: ::core::ffi::c_int =
                                    szNew[(i - 1 as ::core::ffi::c_int) as usize];
                                let mut r: ::core::ffi::c_int;
                                let mut d: ::core::ffi::c_int;
                                r = cntNew[(i - 1 as ::core::ffi::c_int) as usize]
                                    - 1 as ::core::ffi::c_int;
                                d = r + 1 as ::core::ffi::c_int - leafData;
                                cachedCellSize(&raw mut b, d);
                                loop {
                                    
                                    
                                    let szR: ::core::ffi::c_int = cachedCellSize(&raw mut b, r) as ::core::ffi::c_int;
                                    let szD: ::core::ffi::c_int = *b.szCell.offset(d as isize) as ::core::ffi::c_int;
                                    if szRight != 0 as ::core::ffi::c_int
                                        && (bBulk != 0
                                            || szRight + szD + 2 as ::core::ffi::c_int
                                                > szLeft
                                                    - (szR
                                                        + (if i == k - 1 as ::core::ffi::c_int {
                                                            0 as ::core::ffi::c_int
                                                        } else {
                                                            2 as ::core::ffi::c_int
                                                        })))
                                    {
                                        break;
                                    }
                                    szRight += szD + 2 as ::core::ffi::c_int;
                                    szLeft -= szR + 2 as ::core::ffi::c_int;
                                    cntNew[(i - 1 as ::core::ffi::c_int) as usize] = r;
                                    r -= 1;
                                    d -= 1;
                                    if (r < 0 as ::core::ffi::c_int) {
                                        break;
                                    }
                                }
                                szNew[i as usize] = szRight;
                                szNew[(i - 1 as ::core::ffi::c_int) as usize] = szLeft;
                                if cntNew[(i - 1 as ::core::ffi::c_int) as usize]
                                    <= (if i > 1 as ::core::ffi::c_int {
                                        cntNew[(i - 2 as ::core::ffi::c_int) as usize]
                                    } else {
                                        0 as ::core::ffi::c_int
                                    })
                                {
                                    rc = crate::src::src::main::sqlite3CorruptError(
                                        8604 as ::core::ffi::c_int,
                                    );
                                    current_block = 4198108429590484834;
                                    break;
                                } else {
                                    i -= 1;
                                }
                            }
                            match current_block {
                                4198108429590484834 => {}
                                _ => {
                                    pageFlags = *(*apOld[0 as ::core::ffi::c_int as usize])
                                        .aData
                                        .offset(0_isize)
                                        as ::core::ffi::c_int;
                                    i = 0 as ::core::ffi::c_int;
                                    loop {
                                        if (i >= k) {
                                            current_block = 8499731551232998623;
                                            break;
                                        }
                                        let mut pNew: *mut crate::src::headers::btreeInt_h::MemPage =
                                            ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                                        if i < nOld {
                                            apNew[i as usize] = apOld[i as usize];
                                            pNew = apNew[i as usize];
                                            apOld[i as usize] = ::core::ptr::null_mut::<
                                                crate::src::headers::btreeInt_h::MemPage,
                                            >(
                                            );
                                            rc = crate::src::src::pager::sqlite3PagerWrite(
                                                (*pNew).pDbPage
                                                    as *mut crate::src::src::pcache::PgHdr,
                                            );
                                            nNew += 1;
                                            if crate::src::src::pager::sqlite3PagerPageRefcount(
                                                (*pNew).pDbPage
                                                    as *mut crate::src::src::pcache::PgHdr,
                                            ) != 1 as ::core::ffi::c_int
                                                + (i == iParentIdx - nxDiv)
                                                    as ::core::ffi::c_int
                                                && rc
                                                    == crate::src::headers::sqlite3_h::SQLITE_OK
                                            {
                                                rc = crate::src::src::main::sqlite3CorruptError(
                                                    8637 as ::core::ffi::c_int,
                                                );
                                            }
                                            if rc != 0 {
                                                current_block = 4198108429590484834;
                                                break;
                                            }
                                        } else {
                                            rc = allocateBtreePage(
                                                pBt,
                                                &raw mut pNew,
                                                &raw mut pgno,
                                                if bBulk != 0 {
                                                    1 as crate::src::src::pager::Pgno
                                                } else {
                                                    pgno
                                                },
                                                0 as crate::src::ext::rtree::rtree::U8_0,
                                            );
                                            if rc != 0 {
                                                current_block = 4198108429590484834;
                                                break;
                                            }
                                            zeroPage(pNew, pageFlags);
                                            apNew[i as usize] = pNew;
                                            nNew += 1;
                                            cntOld[i as usize] = b.nCell;
                                            if (*pBt).autoVacuum != 0 {
                                                ptrmapPut(
                                                    pBt,
                                                    (*pNew).pgno,
                                                    crate::src::headers::btreeInt_h::PTRMAP_BTREE as crate::src::ext::rtree::rtree::U8_0,
                                                    __pParent_ref.pgno,
                                                    &raw mut rc,
                                                );
                                                if rc
                                                    != crate::src::headers::sqlite3_h::SQLITE_OK
                                                {
                                                    current_block = 4198108429590484834;
                                                    break;
                                                }
                                            }
                                        }
                                        i += 1;
                                    }
                                    match current_block {
                                        4198108429590484834 => {}
                                        _ => {
                                            i = 0 as ::core::ffi::c_int;
                                            while i < nNew {
                                                aPgno[i as usize] = (*apNew[i as usize]).pgno;
                                                i += 1;
                                            }
                                            i = 0 as ::core::ffi::c_int;
                                            while i < nNew - 1 as ::core::ffi::c_int {
                                                let mut iB: ::core::ffi::c_int = i;
                                                j = i + 1 as ::core::ffi::c_int;
                                                while j < nNew {
                                                    if (*apNew[j as usize]).pgno
                                                        < (*apNew[iB as usize]).pgno
                                                    {
                                                        iB = j;
                                                    }
                                                    j += 1;
                                                }
                                                if iB != i {
                                                    let pgnoA: crate::src::src::pager::Pgno =
                                                        (*apNew[i as usize]).pgno;
                                                    let pgnoB: crate::src::src::pager::Pgno =
                                                        (*apNew[iB as usize]).pgno;
                                                    let pgnoTemp: crate::src::src::pager::Pgno =
                                                        (crate::src::src::global::sqlite3PendingByte as crate::src::src::pager::Pgno)
                                                            .wrapping_div(
                                                                (*pBt).pageSize as crate::src::src::pager::Pgno,
                                                            )
                                                            .wrapping_add(1 as crate::src::src::pager::Pgno);
                                                    let fgA: crate::src::fts5::U16_0 =
                                                        (*(*apNew[i as usize]).pDbPage).flags;
                                                    let fgB: crate::src::fts5::U16_0 =
                                                        (*(*apNew[iB as usize]).pDbPage).flags;
                                                    crate::src::src::pager::sqlite3PagerRekey(
                                                        
                                                        (*apNew[i as usize]).pDbPage as *mut crate::src::src::pcache::PgHdr,
                                                        pgnoTemp,
                                                        fgB,
                                                    );
                                                    crate::src::src::pager::sqlite3PagerRekey(
                                                        
                                                        (*apNew[iB as usize]).pDbPage as *mut crate::src::src::pcache::PgHdr,
                                                        pgnoA,
                                                        fgA,
                                                    );
                                                    crate::src::src::pager::sqlite3PagerRekey(
                                                        
                                                        (*apNew[i as usize]).pDbPage as *mut crate::src::src::pcache::PgHdr,
                                                        pgnoB,
                                                        fgB,
                                                    );
                                                    (*apNew[i as usize]).pgno = pgnoB;
                                                    (*apNew[iB as usize]).pgno = pgnoA;
                                                }
                                                i += 1;
                                            }
                                            crate::src::src::util::sqlite3Put4byte(
                                                pRight,
                                                (*apNew
                                                    [(nNew - 1 as ::core::ffi::c_int) as usize])
                                                    .pgno
                                                    as crate::src::ext::rtree::rtree::U32_0,
                                            );
                                            if pageFlags
                                                & crate::src::headers::btreeInt_h::PTF_LEAF
                                                == 0 as ::core::ffi::c_int
                                                && nOld != nNew
                                            {
                                                let pOld_0: *mut crate::src::headers::btreeInt_h::MemPage;
                                                if nNew > nOld {
                                                    pOld_0 = apNew[(nOld
                                                        - 1 as ::core::ffi::c_int)
                                                        as usize];
                                                } else {
                                                    pOld_0 = apOld[(nOld
                                                        - 1 as ::core::ffi::c_int)
                                                        as usize];
                                                }
                                                ::core::ptr::copy_nonoverlapping(
                (*pOld_0).aData.offset(
                                                        8_isize,
                                                    )
                                                        as *mut crate::src::ext::rtree::rtree::U8_0 as *const u8,
                (**(&raw mut apNew as *mut *mut crate::src::headers::btreeInt_h::MemPage)
                                                        .offset(
                                                            (nNew - 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        ))
                                                    .aData
                                                    .offset(8_isize)
                                                        as *mut crate::src::ext::rtree::rtree::U8_0 as *mut u8,
                4_usize,
            );
                                            }
                                            if (*pBt).autoVacuum != 0 {
                                                let mut pOld_1: *mut crate::src::headers::btreeInt_h::MemPage;
                                                pOld_1 =
                                                    apNew[0 as ::core::ffi::c_int as usize];
                                                let mut pNew_0: *mut crate::src::headers::btreeInt_h::MemPage = pOld_1;
                                                let mut cntOldNext: ::core::ffi::c_int =
                                                    (*pNew_0).nCell as ::core::ffi::c_int
                                                        + (*pNew_0).nOverflow
                                                            as ::core::ffi::c_int;
                                                let mut iNew: ::core::ffi::c_int =
                                                    0 as ::core::ffi::c_int;
                                                let mut iOld: ::core::ffi::c_int =
                                                    0 as ::core::ffi::c_int;
                                                i = 0 as ::core::ffi::c_int;
                                                loop {
                                                    if (i >= b.nCell) {
                                                        current_block = 14065265019409502504;
                                                        break;
                                                    }
                                                    let pCell: *mut crate::src::ext::rtree::rtree::U8_0 =
                                                        *b.apCell.offset(i as isize);
                                                    while i == cntOldNext {
                                                        iOld += 1;
                                                        pOld_1 = if iOld < nNew {
                                                            apNew[iOld as usize]
                                                        } else {
                                                            apOld[iOld as usize]
                                                        };
                                                        cntOldNext += (*pOld_1).nCell
                                                            as ::core::ffi::c_int
                                                            + (*pOld_1).nOverflow
                                                                as ::core::ffi::c_int
                                                            + (leafData == 0)
                                                                as ::core::ffi::c_int;
                                                    }
                                                    if i == cntNew[iNew as usize] {
                                                        iNew += 1;
                                                        pNew_0 = apNew[iNew as usize];
                                                        if leafData == 0 {
                                                            current_block =
                                                                12638391263490919476;
                                                        } else {
                                                            current_block =
                                                                18368972393688527475;
                                                        }
                                                    } else {
                                                        current_block = 18368972393688527475;
                                                    }
                                                    match current_block {
                                                        18368972393688527475
                                                            if (iOld >= nNew
                                                                || (*pNew_0).pgno
                                                                    != aPgno[iOld as usize]
                                                                || !(pCell as crate::src::headers::sqliteInt_h::Uptr
                                                                    >= (*pOld_1).aData as crate::src::headers::sqliteInt_h::Uptr
                                                                    && (pCell as crate::src::headers::sqliteInt_h::Uptr)
                                                                        < (*pOld_1).aDataEnd
                                                                            as crate::src::headers::sqliteInt_h::Uptr))
                                                            => {
                                                                if leafCorrection == 0 {
                                                                    ptrmapPut(
                                                                        pBt,
                                                                        crate::src::src::util::sqlite3Get4byte(pCell)
                                                                            as crate::src::src::pager::Pgno,
                                                                        crate::src::headers::btreeInt_h::PTRMAP_BTREE as crate::src::ext::rtree::rtree::U8_0,
                                                                        (*pNew_0).pgno,
                                                                        &raw mut rc,
                                                                    );
                                                                }
                                                                if cachedCellSize(&raw mut b, i)
                                                                    as ::core::ffi::c_int
                                                                    > (*pNew_0).minLocal
                                                                        as ::core::ffi::c_int
                                                                {
                                                                    ptrmapPutOvflPtr(
                                                                        pNew_0,
                                                                        pOld_1,
                                                                        pCell,
                                                                        &raw mut rc,
                                                                    );
                                                                }
                                                                if rc != 0 {
                                                                    current_block =
                                                                        4198108429590484834;
                                                                    break;
                                                                }
                                                            }
                                                        _ => {}
                                                    }
                                                    i += 1;
                                                }
                                            } else {
                                                current_block = 14065265019409502504;
                                            }
                                            match current_block {
                                                4198108429590484834 => {}
                                                _ => {
                                                    i = 0 as ::core::ffi::c_int;
                                                    loop {
                                                        if (i >= nNew - 1 as ::core::ffi::c_int)
                                                        {
                                                            current_block =
                                                                14203403603292796089;
                                                            break;
                                                        }
                                                        let mut pCell_0: *mut crate::src::ext::rtree::rtree::U8_0;
                                                        let mut pTemp_0: *mut crate::src::ext::rtree::rtree::U8_0;
                                                        let mut sz_1: ::core::ffi::c_int;
                                                        
                                                        let pNew_1: *mut crate::src::headers::btreeInt_h::MemPage =
                                                            apNew[i as usize];
                                                        j = cntNew[i as usize];
                                                        pCell_0 = *b.apCell.offset(j as isize);
                                                        sz_1 = *b.szCell.offset(j as isize)
                                                            as ::core::ffi::c_int
                                                            + leafCorrection
                                                                as ::core::ffi::c_int;
                                                        pTemp_0 = aOvflSpace
                                                            .offset(iOvflSpace as isize)
                                                            as *mut crate::src::ext::rtree::rtree::U8_0;
                                                        if (*pNew_1).leaf == 0 {
                                                            ::core::ptr::copy_nonoverlapping(
                pCell_0 as *const u8,
                (*pNew_1).aData.offset(8_isize)
                                                                    as *mut crate::src::ext::rtree::rtree::U8_0 as *mut u8,
                4_usize,
            );
                                                        } else if leafData != 0 {
                                                            let mut info: crate::src::headers::btreeInt_h::CellInfo = unsafe { ::core::mem::zeroed() };
                                                            j -= 1;
                                                            (*pNew_1).xParseCell.expect(
                                                                "non-null function pointer",
                                                            )(
                                                                pNew_1,
                                                                *b.apCell.offset(j as isize),
                                                                &raw mut info,
                                                            );
                                                            pCell_0 = pTemp_0;
                                                            sz_1 = 4 as ::core::ffi::c_int
                                                                + crate::src::src::util::sqlite3PutVarint(
                                                                    pCell_0.offset(4_isize)
                                                                        as *mut ::core::ffi::c_uchar,
                                                                    info.nKey as crate::src::ext::rtree::rtree::U64_0,
                                                                );
                                                            pTemp_0 =
                                                                ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
                                                        } else {
                                                            pCell_0 = pCell_0.offset(
                                                                -(4 as ::core::ffi::c_int
                                                                    as isize),
                                                            );
                                                            if *b.szCell.offset(j as isize)
                                                                as ::core::ffi::c_int
                                                                == 4 as ::core::ffi::c_int
                                                            {
                                                                sz_1 = (*pParent)
                                                                    .xCellSize
                                                                    .expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    pParent, pCell_0
                                                                )
                                                                    as ::core::ffi::c_int;
                                                            }
                                                        }
                                                        iOvflSpace += sz_1;
                                                        k = 0 as ::core::ffi::c_int;
                                                        while b.ixNx[k as usize] <= j {
                                                            k += 1;
                                                        }
                                                        let pSrcEnd: *mut crate::src::ext::rtree::rtree::U8_0 = b.apEnd[k as usize];
                                                        if (pCell_0 as crate::src::headers::sqliteInt_h::Uptr) < pSrcEnd as crate::src::headers::sqliteInt_h::Uptr
                                                            && pCell_0.offset(sz_1 as isize)
                                                                as crate::src::headers::sqliteInt_h::Uptr
                                                                > pSrcEnd as crate::src::headers::sqliteInt_h::Uptr
                                                        {
                                                            rc = crate::src::src::main::sqlite3CorruptError(
                                                                8843 as ::core::ffi::c_int,
                                                            );
                                                            current_block = 4198108429590484834;
                                                            break;
                                                        } else {
                                                            rc = insertCell(
                                                                pParent,
                                                                nxDiv + i,
                                                                pCell_0,
                                                                sz_1,
                                                                pTemp_0,
                                                                (*pNew_1).pgno,
                                                            );
                                                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                                                current_block =
                                                                    4198108429590484834;
                                                                break;
                                                            }
                                                            i += 1;
                                                        }
                                                    }
                                                    match current_block {
                                                        4198108429590484834 => {}
                                                        _ => {
                                                            i = 1 as ::core::ffi::c_int - nNew;
                                                            loop {
                                                                if (i >= nNew) {
                                                                    current_block =
                                                                        18302639990213641465;
                                                                    break;
                                                                }
                                                                let iPg: ::core::ffi::c_int = if i
                                                                    < 0 as ::core::ffi::c_int
                                                                {
                                                                    -i
                                                                } else {
                                                                    i
                                                                };
                                                                if (abDone[iPg as usize] == 0)
                                                                    && (i >= 0 as ::core::ffi::c_int
                                                                        || cntOld[(iPg - 1 as ::core::ffi::c_int) as usize]
                                                                            >= cntNew[(iPg - 1 as ::core::ffi::c_int) as usize])
                                                                    {
                                                                        let iNew_0: ::core::ffi::c_int;
                                                                        let iOld_0: ::core::ffi::c_int;
                                                                        let nNewCell: ::core::ffi::c_int;
                                                                        if iPg == 0 as ::core::ffi::c_int {
                                                                            iOld_0 = 0 as ::core::ffi::c_int;
                                                                            iNew_0 = iOld_0;
                                                                            nNewCell = cntNew[0 as ::core::ffi::c_int as usize];
                                                                        } else {
                                                                            iOld_0 = if iPg < nOld {
                                                                                cntOld[(iPg - 1 as ::core::ffi::c_int) as usize]
                                                                                    + (leafData == 0) as ::core::ffi::c_int
                                                                            } else {
                                                                                b.nCell
                                                                            };
                                                                            iNew_0 = cntNew[(iPg - 1 as ::core::ffi::c_int) as usize]
                                                                                + (leafData == 0) as ::core::ffi::c_int;
                                                                            nNewCell = cntNew[iPg as usize] - iNew_0;
                                                                        }
                                                                        rc = editPage(
                                                                            apNew[iPg as usize],
                                                                            iOld_0,
                                                                            iNew_0,
                                                                            nNewCell,
                                                                            &raw mut b,
                                                                        );
                                                                        if rc != 0 {
                                                                            current_block = 4198108429590484834;
                                                                            break;
                                                                        }
                                                                        abDone[iPg as usize] = abDone[iPg as usize].wrapping_add(1);
                                                                        (*apNew[iPg as usize]).nFree = usableSpace
                                                                            - szNew[iPg as usize];
                                                                    }
                                                                i += 1;
                                                            }
                                                            match current_block {
                                                                4198108429590484834 => {}
                                                                _ => {
                                                                    if isRoot != 0
                                                                        && __pParent_ref.nCell as ::core::ffi::c_int
                                                                            == 0 as ::core::ffi::c_int
                                                                        && __pParent_ref.hdrOffset as ::core::ffi::c_int
                                                                            <= (*apNew[0 as ::core::ffi::c_int as usize]).nFree
                                                                    {
                                                                        rc = defragmentPage(
                                                                            apNew[0 as ::core::ffi::c_int as usize],
                                                                            -(1 as ::core::ffi::c_int),
                                                                        );
                                                                        copyNodeContent(
                                                                            apNew[0 as ::core::ffi::c_int as usize],
                                                                            pParent,
                                                                            &raw mut rc,
                                                                        );
                                                                        freePage(
                                                                            apNew[0 as ::core::ffi::c_int as usize],
                                                                            &raw mut rc,
                                                                        );
                                                                    } else if (*pBt).autoVacuum as ::core::ffi::c_int != 0
                                                                        && leafCorrection == 0
                                                                    {
                                                                        i = 0 as ::core::ffi::c_int;
                                                                        while i < nNew {
                                                                            let key: crate::src::ext::rtree::rtree::U32_0 = crate::src::src::util::sqlite3Get4byte(
                                                                                (**(&raw mut apNew as *mut *mut crate::src::headers::btreeInt_h::MemPage).offset(i as isize))
                                                                                    .aData
                                                                                    .offset(8_isize) as *mut crate::src::ext::rtree::rtree::U8_0,
                                                                            );
                                                                            ptrmapPut(
                                                                                pBt,
                                                                                key as crate::src::src::pager::Pgno,
                                                                                crate::src::headers::btreeInt_h::PTRMAP_BTREE as crate::src::ext::rtree::rtree::U8_0,
                                                                                (*apNew[i as usize]).pgno,
                                                                                &raw mut rc,
                                                                            );
                                                                            i += 1;
                                                                        }
                                                                    }
                                                                    i = nNew;
                                                                    while i < nOld {
                                                                        freePage(
                                                                            apOld[i as usize],
                                                                            &raw mut rc,
                                                                        );
                                                                        i += 1;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    crate::src::src::malloc::sqlite3DbFree(
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>()
            as *mut crate::src::headers::sqliteInt_h::sqlite3,
        b.apCell as *mut ::core::ffi::c_void,
    );
    i = 0 as ::core::ffi::c_int;
    while i < nOld {
        releasePage(apOld[i as usize]);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nNew {
        releasePage(apNew[i as usize]);
        i += 1;
    }
    rc
}

unsafe extern "C" fn balance_deeper(
    pRoot: *mut crate::src::headers::btreeInt_h::MemPage,
    ppChild: *mut *mut crate::src::headers::btreeInt_h::MemPage,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pChild: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let mut pgnoChild: crate::src::src::pager::Pgno = 0 as crate::src::src::pager::Pgno;
    let __pRoot_ref = unsafe { &mut *pRoot };
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __pRoot_ref.pBt;
    rc = crate::src::src::pager::sqlite3PagerWrite(
        __pRoot_ref.pDbPage as *mut crate::src::src::pcache::PgHdr,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = allocateBtreePage(
            pBt,
            &raw mut pChild,
            &raw mut pgnoChild,
            __pRoot_ref.pgno,
            0 as crate::src::ext::rtree::rtree::U8_0,
        );
        copyNodeContent(pRoot, pChild, &raw mut rc);
        if (*pBt).autoVacuum != 0 {
            ptrmapPut(
                pBt,
                pgnoChild,
                crate::src::headers::btreeInt_h::PTRMAP_BTREE
                    as crate::src::ext::rtree::rtree::U8_0,
                __pRoot_ref.pgno,
                &raw mut rc,
            );
        }
    }
    if rc != 0 {
        *ppChild = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
        releasePage(pChild);
        return rc;
    }
    let __pChild_ref = unsafe { &mut *pChild };
    ::core::ptr::copy_nonoverlapping(
        &raw mut __pRoot_ref.aiOvfl as *mut crate::src::fts5::U16_0 as *const u8,
        &raw mut __pChild_ref.aiOvfl as *mut crate::src::fts5::U16_0 as *mut u8,
        ((__pRoot_ref.nOverflow as crate::__stddef_size_t_h::SizeT)
            .wrapping_mul(::core::mem::size_of::<crate::src::fts5::U16_0>()
                as crate::__stddef_size_t_h::SizeT)) as usize,
    );
    ::core::ptr::copy_nonoverlapping(
        &raw mut __pRoot_ref.apOvfl as *mut *mut crate::src::ext::rtree::rtree::U8_0 as *const u8,
        &raw mut __pChild_ref.apOvfl as *mut *mut crate::src::ext::rtree::rtree::U8_0 as *mut u8,
        ((__pRoot_ref.nOverflow as crate::__stddef_size_t_h::SizeT).wrapping_mul(
            ::core::mem::size_of::<*mut crate::src::ext::rtree::rtree::U8_0>()
                as crate::__stddef_size_t_h::SizeT,
        )) as usize,
    );
    __pChild_ref.nOverflow = __pRoot_ref.nOverflow;
    zeroPage(
        pRoot,
        *__pChild_ref.aData.offset(0_isize) as ::core::ffi::c_int
            & !crate::src::headers::btreeInt_h::PTF_LEAF,
    );
    crate::src::src::util::sqlite3Put4byte(
        (*pRoot).aData.offset(
            (__pRoot_ref.hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::ext::rtree::rtree::U8_0,
        pgnoChild as crate::src::ext::rtree::rtree::U32_0,
    );
    *ppChild = pChild;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn anotherValidCursor(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let mut pOther: *mut crate::src::headers::btreeInt_h::BtCursor;
    pOther = (*(*pCur).pBt).pCursor;
    while !pOther.is_null() {
        if pOther != pCur
            && (*pOther).eState as ::core::ffi::c_int
                == crate::src::headers::btreeInt_h::CURSOR_VALID
            && (*pOther).pPage == (*pCur).pPage
        {
            return crate::src::src::main::sqlite3CorruptError(9075 as ::core::ffi::c_int);
        }
        pOther = (*pOther).pNext;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn balance(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut aBalanceQuickSpace: [crate::src::ext::rtree::rtree::U8_0; 13] = [0; 13];
    let mut pFree: *mut crate::src::ext::rtree::rtree::U8_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    loop {
        
        let __pCur_ref = unsafe { &mut *pCur };
        let pPage: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
        let __pPage_ref = unsafe { &mut *pPage };
        if __pPage_ref.nFree < 0 as ::core::ffi::c_int && btreeComputeFreeSpace(pPage) != 0 {
            break;
        }
        if __pPage_ref.nOverflow as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && __pPage_ref.nFree * 3 as ::core::ffi::c_int
                <= (*__pCur_ref.pBt).usableSize as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        {
            break;
        }
        let iPage: ::core::ffi::c_int = __pCur_ref.iPage as ::core::ffi::c_int;
        if iPage == 0 as ::core::ffi::c_int {
            if !(__pPage_ref.nOverflow as ::core::ffi::c_int != 0 && {
                rc = anotherValidCursor(pCur);
                rc == crate::src::headers::sqlite3_h::SQLITE_OK
            }) {
                break;
            }
            rc = balance_deeper(
                pPage,
                (&raw mut __pCur_ref.apPage as *mut *mut crate::src::headers::btreeInt_h::MemPage)
                    .offset(1_isize)
                    as *mut *mut crate::src::headers::btreeInt_h::MemPage,
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                __pCur_ref.iPage = 1 as crate::src::headers::sqliteInt_h::I8_0;
                __pCur_ref.ix = 0 as crate::src::fts5::U16_0;
                __pCur_ref.aiIdx[0 as ::core::ffi::c_int as usize] = 0 as crate::src::fts5::U16_0;
                __pCur_ref.apPage[0 as ::core::ffi::c_int as usize] = pPage;
                __pCur_ref.pPage = __pCur_ref.apPage[1 as ::core::ffi::c_int as usize];
            }
        } else if crate::src::src::pager::sqlite3PagerPageRefcount(
            __pPage_ref.pDbPage as *mut crate::src::src::pcache::PgHdr,
        ) > 1 as ::core::ffi::c_int
        {
            rc = crate::src::src::main::sqlite3CorruptError(9135 as ::core::ffi::c_int);
        } else {
            let pParent: *mut crate::src::headers::btreeInt_h::MemPage =
                __pCur_ref.apPage[(iPage - 1 as ::core::ffi::c_int) as usize];
            let iIdx: ::core::ffi::c_int =
                __pCur_ref.aiIdx[(iPage - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
            rc = crate::src::src::pager::sqlite3PagerWrite(
                (*pParent).pDbPage as *mut crate::src::src::pcache::PgHdr,
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                && (*pParent).nFree < 0 as ::core::ffi::c_int
            {
                rc = btreeComputeFreeSpace(pParent);
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                if __pPage_ref.intKeyLeaf as ::core::ffi::c_int != 0
                    && __pPage_ref.nOverflow as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                    && __pPage_ref.aiOvfl[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == __pPage_ref.nCell as ::core::ffi::c_int
                    && (*pParent).pgno != 1 as crate::src::src::pager::Pgno
                    && (*pParent).nCell as ::core::ffi::c_int == iIdx
                {
                    rc = balance_quick(
                        pParent,
                        pPage,
                        &raw mut aBalanceQuickSpace as *mut crate::src::ext::rtree::rtree::U8_0,
                    );
                } else {
                    let pSpace: *mut crate::src::ext::rtree::rtree::U8_0 =
                        crate::src::src::pcache1::sqlite3PageMalloc(
                            (*__pCur_ref.pBt).pageSize as ::core::ffi::c_int,
                        ) as *mut crate::src::ext::rtree::rtree::U8_0;
                    rc = balance_nonroot(
                        pParent,
                        iIdx,
                        pSpace,
                        (iPage == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        __pCur_ref.hints as ::core::ffi::c_int
                            & crate::src::src::btree::BTREE_BULKLOAD,
                    );
                    if !pFree.is_null() {
                        crate::src::src::pcache1::sqlite3PageFree(
                            pFree as *mut ::core::ffi::c_void,
                        );
                    }
                    pFree = pSpace;
                }
            }
            __pPage_ref.nOverflow = 0 as crate::src::ext::rtree::rtree::U8_0;
            releasePage(pPage);
            __pCur_ref.iPage -= 1;
            __pCur_ref.pPage = __pCur_ref.apPage[__pCur_ref.iPage as usize];
        }
        if (rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
            break;
        }
    }
    if !pFree.is_null() {
        crate::src::src::pcache1::sqlite3PageFree(pFree as *mut ::core::ffi::c_void);
    }
    rc
}

unsafe extern "C" fn btreeOverwriteContent(
    pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pDest: *mut crate::src::ext::rtree::rtree::U8_0,
    pX: *const crate::src::src::btree::BtreePayload,
    iOffset: ::core::ffi::c_int,
    mut iAmt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let nData: ::core::ffi::c_int = (*pX).nData - iOffset;
    if nData <= 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < iAmt && *pDest.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            i += 1;
        }
        if i < iAmt {
            let rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerWrite(
                (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
            );
            if rc != 0 {
                return rc;
            }
            ::libc::memset(
                pDest.offset(i as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (iAmt - i) as crate::__stddef_size_t_h::SizeT,
            );
        }
    } else {
        if nData < iAmt {
            let rc_0: ::core::ffi::c_int = btreeOverwriteContent(
                pPage,
                pDest.offset(nData as isize),
                pX,
                iOffset + nData,
                iAmt - nData,
            );
            if rc_0 != 0 {
                return rc_0;
            }
            iAmt = nData;
        }
        if ::libc::memcmp(
            pDest as *const ::core::ffi::c_void,
            ((*pX).pData as *mut crate::src::ext::rtree::rtree::U8_0).offset(iOffset as isize)
                as *const ::core::ffi::c_void,
            iAmt as crate::__stddef_size_t_h::SizeT,
        ) != 0 as ::core::ffi::c_int
        {
            let rc_1: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerWrite(
                (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
            );
            if rc_1 != 0 {
                return rc_1;
            }
            ::core::ptr::copy(
                ((*pX).pData as *mut crate::src::ext::rtree::rtree::U8_0).offset(iOffset as isize)
                    as *const u8,
                pDest as *mut u8,
                iAmt as usize,
            );
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[inline(never)]
unsafe extern "C" fn btreeOverwriteOverflowCell(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pX: *const crate::src::src::btree::BtreePayload,
) -> ::core::ffi::c_int {
    let mut iOffset: ::core::ffi::c_int;
    let nTotal: ::core::ffi::c_int = (*pX).nData + (*pX).nZero;
    let mut rc: ::core::ffi::c_int;
    let __pCur_ref = unsafe { &mut *pCur };
    let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
    
    let mut ovflPgno: crate::src::src::pager::Pgno;
    let mut ovflPageSize: crate::src::ext::rtree::rtree::U32_0;
    rc = btreeOverwriteContent(
        pPage,
        __pCur_ref.info.pPayload,
        pX,
        0 as ::core::ffi::c_int,
        __pCur_ref.info.nLocal as ::core::ffi::c_int,
    );
    if rc != 0 {
        return rc;
    }
    iOffset = __pCur_ref.info.nLocal as ::core::ffi::c_int;
    ovflPgno =
        crate::src::src::util::sqlite3Get4byte(__pCur_ref.info.pPayload.offset(iOffset as isize))
            as crate::src::src::pager::Pgno;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*pPage).pBt;
    ovflPageSize = (*pBt)
        .usableSize
        .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0);
    loop {
        rc = btreeGetPage(pBt, ovflPgno, &raw mut pPage, 0 as ::core::ffi::c_int);
        if rc != 0 {
            return rc;
        }
        let __pPage_ref = unsafe { &mut *pPage };
        if crate::src::src::pager::sqlite3PagerPageRefcount(
            __pPage_ref.pDbPage as *mut crate::src::src::pcache::PgHdr,
        ) != 1 as ::core::ffi::c_int
            || __pPage_ref.isInit as ::core::ffi::c_int != 0
        {
            rc = crate::src::src::main::sqlite3CorruptError(9299 as ::core::ffi::c_int);
        } else {
            if (iOffset as crate::src::ext::rtree::rtree::U32_0).wrapping_add(ovflPageSize)
                < nTotal as crate::src::ext::rtree::rtree::U32_0
            {
                ovflPgno = crate::src::src::util::sqlite3Get4byte(__pPage_ref.aData)
                    as crate::src::src::pager::Pgno;
            } else {
                ovflPageSize = (nTotal - iOffset) as crate::src::ext::rtree::rtree::U32_0;
            }
            rc = btreeOverwriteContent(
                pPage,
                __pPage_ref.aData.offset(4_isize),
                pX,
                iOffset,
                ovflPageSize as ::core::ffi::c_int,
            );
        }
        crate::src::src::pager::sqlite3PagerUnref(
            __pPage_ref.pDbPage as *mut crate::src::src::pcache::PgHdr,
        );
        if rc != 0 {
            return rc;
        }
        iOffset = (iOffset as crate::src::ext::rtree::rtree::U32_0).wrapping_add(ovflPageSize)
            as ::core::ffi::c_int as ::core::ffi::c_int;
        if (iOffset >= nTotal) {
            break;
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn btreeOverwriteCell(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pX: *const crate::src::src::btree::BtreePayload,
) -> ::core::ffi::c_int {
    let nTotal: ::core::ffi::c_int = (*pX).nData + (*pX).nZero;
    let __pCur_ref = unsafe { &mut *pCur };
    let pPage: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
    let __pPage_ref = unsafe { &mut *pPage };
    if (*pCur)
        .info
        .pPayload
        .offset(__pCur_ref.info.nLocal as ::core::ffi::c_int as isize)
        > __pPage_ref.aDataEnd
        || __pCur_ref.info.pPayload
            < (*pPage)
                .aData
                .offset(__pPage_ref.cellOffset as ::core::ffi::c_int as isize)
    {
        return crate::src::src::main::sqlite3CorruptError(9327 as ::core::ffi::c_int);
    }
    if __pCur_ref.info.nLocal as ::core::ffi::c_int == nTotal {
        btreeOverwriteContent(
            pPage,
            __pCur_ref.info.pPayload,
            pX,
            0 as ::core::ffi::c_int,
            __pCur_ref.info.nLocal as ::core::ffi::c_int,
        )
    } else {
        btreeOverwriteOverflowCell(pCur, pX)
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeInsert(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pX: *const crate::src::src::btree::BtreePayload,
    flags: ::core::ffi::c_int,
    seekResult: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int;
    let mut loc: ::core::ffi::c_int = seekResult;
    let mut szNew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idx: ::core::ffi::c_int;
    
    let __pCur_ref = unsafe { &mut *pCur };
    let p: *mut crate::src::headers::btreeInt_h::Btree = __pCur_ref.pBtree;
    let oldCell: *mut ::core::ffi::c_uchar;
    
    if __pCur_ref.curFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTCF_Multiple
        != 0
    {
        rc = saveAllCursors((*p).pBt, __pCur_ref.pgnoRoot, pCur);
        if rc != 0 {
            return rc;
        }
        if loc != 0 && (__pCur_ref.iPage as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            return crate::src::src::main::sqlite3CorruptError(9408 as ::core::ffi::c_int);
        }
    }
    if __pCur_ref.eState as ::core::ffi::c_int
        >= crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK
    {
        rc = moveToRoot(pCur);
        if rc != 0 && rc != crate::src::headers::sqlite3_h::SQLITE_EMPTY {
            return rc;
        }
    }
    if __pCur_ref.pKeyInfo.is_null() {
        if (*p).hasIncrblobCur != 0 {
            invalidateIncrblobCursors(
                p,
                __pCur_ref.pgnoRoot,
                (*pX).nKey as crate::src::ext::rtree::rtree::I64_0,
                0 as ::core::ffi::c_int,
            );
        }
        if __pCur_ref.curFlags as ::core::ffi::c_int
            & crate::src::headers::btreeInt_h::BTCF_ValidNKey
            != 0 as ::core::ffi::c_int
            && (*pX).nKey == __pCur_ref.info.nKey
        {
            if __pCur_ref.info.nSize as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && __pCur_ref.info.nPayload
                    == ((*pX).nData as crate::src::ext::rtree::rtree::U32_0)
                        .wrapping_add((*pX).nZero as crate::src::ext::rtree::rtree::U32_0)
            {
                return btreeOverwriteCell(pCur, pX);
            }
        } else if loc == 0 as ::core::ffi::c_int {
            rc = sqlite3BtreeTableMoveto(
                pCur,
                (*pX).nKey as crate::src::ext::rtree::rtree::I64_0,
                (flags & crate::src::src::btree::BTREE_APPEND != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int,
                &raw mut loc,
            );
            if rc != 0 {
                return rc;
            }
        }
    } else {
        if loc == 0 as ::core::ffi::c_int
            && flags & crate::src::src::btree::BTREE_SAVEPOSITION == 0 as ::core::ffi::c_int
        {
            if (*pX).nMem != 0 {
                let mut r: crate::src::headers::sqliteInt_h::UnpackedRecord =
                    unsafe { ::core::mem::zeroed() };
                r.pKeyInfo = __pCur_ref.pKeyInfo as *mut crate::src::headers::sqliteInt_h::KeyInfo;
                r.aMem = (*pX).aMem as *mut crate::src::src::vdbe::Mem;
                r.nField = (*pX).nMem;
                r.default_rc = 0 as crate::src::headers::sqliteInt_h::I8_0;
                r.eqSeen = 0 as crate::src::ext::rtree::rtree::U8_0;
                rc = sqlite3BtreeIndexMoveto(pCur, &raw mut r, &raw mut loc);
            } else {
                rc = btreeMoveto(
                    pCur,
                    (*pX).pKey,
                    (*pX).nKey as crate::src::ext::rtree::rtree::I64_0,
                    (flags & crate::src::src::btree::BTREE_APPEND != 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int,
                    &raw mut loc,
                );
            }
            if rc != 0 {
                return rc;
            }
        }
        if loc == 0 as ::core::ffi::c_int {
            getCellInfo(pCur);
            if __pCur_ref.info.nKey == (*pX).nKey {
                let mut x2: crate::src::src::btree::BtreePayload = unsafe { ::core::mem::zeroed() };
                x2.pData = (*pX).pKey;
                x2.nData = (*pX).nKey as ::core::ffi::c_int;
                x2.nZero = 0 as ::core::ffi::c_int;
                return btreeOverwriteCell(pCur, &raw mut x2);
            }
        }
    }
    let pPage: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
    if (*pPage).nFree < 0 as ::core::ffi::c_int {
        if __pCur_ref.eState as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            rc = crate::src::src::main::sqlite3CorruptError(9531 as ::core::ffi::c_int);
        } else {
            rc = btreeComputeFreeSpace(pPage);
        }
        if rc != 0 {
            return rc;
        }
    }
    let newCell: *mut ::core::ffi::c_uchar = (*(*p).pBt).pTmpSpace as *mut ::core::ffi::c_uchar;
    if flags & crate::src::src::btree::BTREE_PREFORMAT != 0 {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        let __pBt_ref = &*(*p).pBt;
        szNew = __pBt_ref.nPreformatSize;
        if szNew < 4 as ::core::ffi::c_int {
            szNew = 4 as ::core::ffi::c_int;
            *newCell.offset(3_isize) = 0 as ::core::ffi::c_uchar;
        }
        if __pBt_ref.autoVacuum as ::core::ffi::c_int != 0
            && szNew > (*pPage).maxLocal as ::core::ffi::c_int
        {
            let mut info: crate::src::headers::btreeInt_h::CellInfo =
                unsafe { ::core::mem::zeroed() };
            (*pPage).xParseCell.expect("non-null function pointer")(
                pPage,
                newCell as *mut crate::src::ext::rtree::rtree::U8_0,
                &raw mut info,
            );
            if info.nPayload != info.nLocal as crate::src::ext::rtree::rtree::U32_0 {
                let ovfl: crate::src::src::pager::Pgno = crate::src::src::util::sqlite3Get4byte(
                    newCell.offset((szNew - 4 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar,
                )
                    as crate::src::src::pager::Pgno;
                ptrmapPut(
                    (*p).pBt,
                    ovfl,
                    crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW1
                        as crate::src::ext::rtree::rtree::U8_0,
                    (*pPage).pgno,
                    &raw mut rc,
                );
                if rc != 0 {
                    current_block = 17030598806234454338;
                } else {
                    current_block = 479107131381816815;
                }
            } else {
                current_block = 479107131381816815;
            }
        } else {
            current_block = 479107131381816815;
        }
    } else {
        rc = fillInCell(pPage, newCell, pX, &raw mut szNew);
        if rc != 0 {
            current_block = 17030598806234454338;
        } else {
            current_block = 479107131381816815;
        }
    }
    if current_block == 479107131381816815 {
        idx = __pCur_ref.ix as ::core::ffi::c_int;
        __pCur_ref.info.nSize = 0 as crate::src::fts5::U16_0;
        if loc == 0 as ::core::ffi::c_int {
            let mut info_0: crate::src::headers::btreeInt_h::CellInfo =
                unsafe { ::core::mem::zeroed() };
            if idx >= (*pPage).nCell as ::core::ffi::c_int {
                return crate::src::src::main::sqlite3CorruptError(9573 as ::core::ffi::c_int);
            }
            rc = crate::src::src::pager::sqlite3PagerWrite(
                (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
            );
            if rc != 0 {
                current_block = 17030598806234454338;
            } else {
                let __pPage_ref = unsafe { &mut *pPage };
                oldCell = __pPage_ref.aData.offset(
                    (__pPage_ref.maskPage as ::core::ffi::c_int
                        & ((*((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * idx) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(0_isize)
                            as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * idx) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(1_isize)
                                as ::core::ffi::c_int)) as isize,
                ) as *mut ::core::ffi::c_uchar;
                if __pPage_ref.leaf == 0 {
                    ::core::ptr::copy_nonoverlapping(
                        oldCell as *const u8,
                        newCell as *mut u8,
                        4_usize,
                    );
                }
                __pPage_ref.xParseCell.expect("non-null function pointer")(
                    pPage,
                    oldCell as *mut crate::src::ext::rtree::rtree::U8_0,
                    &raw mut info_0,
                );
                if info_0.nLocal as crate::src::ext::rtree::rtree::U32_0 != info_0.nPayload {
                    rc = clearCellOverflow(pPage, oldCell, &raw mut info_0);
                } else {
                    rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                }
                __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
                    & !crate::src::headers::btreeInt_h::BTCF_ValidOvfl)
                    as crate::src::ext::rtree::rtree::U8_0;
                if info_0.nSize as ::core::ffi::c_int == szNew
                    && info_0.nLocal as crate::src::ext::rtree::rtree::U32_0 == info_0.nPayload
                    && ((*(*p).pBt).autoVacuum == 0
                        || szNew < __pPage_ref.minLocal as ::core::ffi::c_int)
                {
                    if oldCell
                        < (*pPage)
                            .aData
                            .offset(__pPage_ref.hdrOffset as ::core::ffi::c_int as isize)
                            .offset(10_isize)
                    {
                        return crate::src::src::main::sqlite3CorruptError(
                            9600 as ::core::ffi::c_int,
                        );
                    }
                    if oldCell.offset(szNew as isize) > __pPage_ref.aDataEnd {
                        return crate::src::src::main::sqlite3CorruptError(
                            9603 as ::core::ffi::c_int,
                        );
                    }
                    ::core::ptr::copy_nonoverlapping(
                        newCell as *const u8,
                        oldCell as *mut u8,
                        szNew as usize,
                    );
                    return crate::src::headers::sqlite3_h::SQLITE_OK;
                }
                dropCell(pPage, idx, info_0.nSize as ::core::ffi::c_int, &raw mut rc);
                if rc != 0 {
                    current_block = 17030598806234454338;
                } else {
                    current_block = 12065775993741208975;
                }
            }
        } else {
            if loc < 0 as ::core::ffi::c_int
                && (*pPage).nCell as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            {
                __pCur_ref.ix = __pCur_ref.ix.wrapping_add(1);
                idx = __pCur_ref.ix as ::core::ffi::c_int;
                __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
                    & !(crate::src::headers::btreeInt_h::BTCF_ValidNKey
                        | crate::src::headers::btreeInt_h::BTCF_ValidOvfl))
                    as crate::src::ext::rtree::rtree::U8_0;
            }
            current_block = 12065775993741208975;
        }
        match current_block {
            17030598806234454338 => {}
            _ => {
                rc = insertCellFast(
                    pPage,
                    idx,
                    newCell as *mut crate::src::ext::rtree::rtree::U8_0,
                    szNew,
                );
                if (*pPage).nOverflow != 0 {
                    __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
                        & !(crate::src::headers::btreeInt_h::BTCF_ValidNKey
                            | crate::src::headers::btreeInt_h::BTCF_ValidOvfl))
                        as crate::src::ext::rtree::rtree::U8_0;
                    rc = balance(pCur);
                    (*__pCur_ref.pPage).nOverflow = 0 as crate::src::ext::rtree::rtree::U8_0;
                    __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_INVALID
                        as crate::src::ext::rtree::rtree::U8_0;
                    if flags & crate::src::src::btree::BTREE_SAVEPOSITION != 0
                        && rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    {
                        btreeReleaseAllCursorPages(pCur);
                        if !__pCur_ref.pKeyInfo.is_null() {
                            __pCur_ref.pKey = crate::src::src::malloc::sqlite3Malloc(
                                (*pX).nKey as crate::src::ext::rtree::rtree::U64_0,
                            );
                            if __pCur_ref.pKey.is_null() {
                                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                            } else {
                                ::libc::memcpy(
                                    __pCur_ref.pKey,
                                    (*pX).pKey,
                                    (*pX).nKey as crate::__stddef_size_t_h::SizeT,
                                );
                            }
                        }
                        __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK
                            as crate::src::ext::rtree::rtree::U8_0;
                        __pCur_ref.nKey = (*pX).nKey as crate::src::ext::rtree::rtree::I64_0;
                    }
                }
            }
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeTransferRow(
    pDest: *mut crate::src::headers::btreeInt_h::BtCursor,
    pSrc: *mut crate::src::headers::btreeInt_h::BtCursor,
    iKey: crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    let __pDest_ref = unsafe { &mut *pDest };
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = __pDest_ref.pBt;
    let mut aOut: *mut crate::src::ext::rtree::rtree::U8_0 = (*pBt).pTmpSpace;
    let mut aIn: *const crate::src::ext::rtree::rtree::U8_0;
    let mut nIn: crate::src::ext::rtree::rtree::U32_0;
    let mut nRem: crate::src::ext::rtree::rtree::U32_0;
    getCellInfo(pSrc);
    let __pSrc_ref = unsafe { &mut *pSrc };
    if __pSrc_ref.info.nPayload < 0x80 as crate::src::ext::rtree::rtree::U32_0 {
        let fresh34 = aOut;
        aOut = aOut.offset(1);
        *fresh34 = __pSrc_ref.info.nPayload as crate::src::ext::rtree::rtree::U8_0;
    } else {
        aOut = aOut.offset(crate::src::src::util::sqlite3PutVarint(
            aOut as *mut ::core::ffi::c_uchar,
            __pSrc_ref.info.nPayload as crate::src::ext::rtree::rtree::U64_0,
        ) as isize);
    }
    if __pDest_ref.pKeyInfo.is_null() {
        aOut = aOut.offset(crate::src::src::util::sqlite3PutVarint(
            aOut as *mut ::core::ffi::c_uchar,
            iKey as crate::src::ext::rtree::rtree::U64_0,
        ) as isize);
    }
    nIn = __pSrc_ref.info.nLocal as crate::src::ext::rtree::rtree::U32_0;
    aIn = __pSrc_ref.info.pPayload;
    if aIn.offset(nIn as isize)
        > (*__pSrc_ref.pPage).aDataEnd as *const crate::src::ext::rtree::rtree::U8_0
    {
        return crate::src::src::main::sqlite3CorruptError(9705 as ::core::ffi::c_int);
    }
    nRem = __pSrc_ref.info.nPayload;
    if nIn == nRem && nIn < (*__pDest_ref.pPage).maxLocal as crate::src::ext::rtree::rtree::U32_0 {
        ::core::ptr::copy_nonoverlapping(aIn as *const u8, aOut as *mut u8, nIn as usize);
        (*pBt).nPreformatSize =
            nIn.wrapping_add(aOut.offset_from((*pBt).pTmpSpace) as ::core::ffi::c_long
                as ::core::ffi::c_int
                as crate::src::ext::rtree::rtree::U32_0) as ::core::ffi::c_int;
        crate::src::headers::sqlite3_h::SQLITE_OK
    } else {
        let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
        let pSrcPager: *mut crate::src::src::pager::Pager = (*__pSrc_ref.pBt).pPager;
        let mut pPgnoOut: *mut crate::src::ext::rtree::rtree::U8_0 =
            ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
        let mut ovflIn: crate::src::src::pager::Pgno = 0 as crate::src::src::pager::Pgno;
        let mut pPageIn: *mut crate::src::src::pager::DbPage =
            ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
        let mut pPageOut: *mut crate::src::headers::btreeInt_h::MemPage =
            ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
        let mut nOut: crate::src::ext::rtree::rtree::U32_0;
        nOut = btreePayloadToLocal(
            __pDest_ref.pPage,
            __pSrc_ref.info.nPayload as crate::src::ext::rtree::rtree::I64_0,
        ) as crate::src::ext::rtree::rtree::U32_0;
        (*pBt).nPreformatSize = nOut as ::core::ffi::c_int
            + aOut.offset_from((*pBt).pTmpSpace) as ::core::ffi::c_long as ::core::ffi::c_int;
        if nOut < __pSrc_ref.info.nPayload {
            pPgnoOut = aOut.offset(nOut as isize) as *mut crate::src::ext::rtree::rtree::U8_0;
            (*pBt).nPreformatSize += 4 as ::core::ffi::c_int;
        }
        if nRem > nIn {
            if aIn.offset(nIn as isize).offset(4_isize)
                > (*__pSrc_ref.pPage).aDataEnd as *const crate::src::ext::rtree::rtree::U8_0
            {
                return crate::src::src::main::sqlite3CorruptError(9730 as ::core::ffi::c_int);
            }
            ovflIn = crate::src::src::util::sqlite3Get4byte(
                __pSrc_ref.info.pPayload.offset(nIn as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0,
            ) as crate::src::src::pager::Pgno;
        }
        loop {
            nRem = nRem.wrapping_sub(nOut);
            loop {
                if nIn > 0 as crate::src::ext::rtree::rtree::U32_0 {
                    let nCopy: ::core::ffi::c_int =
                        (if nOut < nIn { nOut } else { nIn }) as ::core::ffi::c_int;
                    ::core::ptr::copy_nonoverlapping(
                        aIn as *const u8,
                        aOut as *mut u8,
                        nCopy as usize,
                    );
                    nOut = nOut.wrapping_sub(nCopy as crate::src::ext::rtree::rtree::U32_0);
                    nIn = nIn.wrapping_sub(nCopy as crate::src::ext::rtree::rtree::U32_0);
                    aOut = aOut.offset(nCopy as isize);
                    aIn = aIn.offset(nCopy as isize);
                }
                if nOut > 0 as crate::src::ext::rtree::rtree::U32_0 {
                    crate::src::src::pager::sqlite3PagerUnref(
                        pPageIn as *mut crate::src::src::pcache::PgHdr,
                    );
                    pPageIn = ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
                    rc = crate::src::src::pager::sqlite3PagerGet(
                        pSrcPager,
                        ovflIn,
                        &raw mut pPageIn as *mut _ as *mut *mut crate::src::src::pcache::PgHdr,
                        crate::src::src::pager::PAGER_GET_READONLY,
                    );
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        aIn = crate::src::src::pager::sqlite3PagerGetData(
                            pPageIn as *mut crate::src::src::pcache::PgHdr,
                        )
                            as *const crate::src::ext::rtree::rtree::U8_0;
                        ovflIn = crate::src::src::util::sqlite3Get4byte(aIn)
                            as crate::src::src::pager::Pgno;
                        aIn = aIn.offset(4_isize);
                        nIn = (*__pSrc_ref.pBt)
                            .usableSize
                            .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0);
                    }
                }
                if !(rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    && nOut > 0 as crate::src::ext::rtree::rtree::U32_0)
                {
                    break;
                }
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                && nRem > 0 as crate::src::ext::rtree::rtree::U32_0
                && !pPgnoOut.is_null()
            {
                let mut pgnoNew: crate::src::src::pager::Pgno = 0;
                let mut pNew: *mut crate::src::headers::btreeInt_h::MemPage =
                    ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
                rc = allocateBtreePage(
                    pBt,
                    &raw mut pNew,
                    &raw mut pgnoNew,
                    0 as crate::src::src::pager::Pgno,
                    0 as crate::src::ext::rtree::rtree::U8_0,
                );
                crate::src::src::util::sqlite3Put4byte(
                    pPgnoOut,
                    pgnoNew as crate::src::ext::rtree::rtree::U32_0,
                );
                if (*pBt).autoVacuum as ::core::ffi::c_int != 0 && !pPageOut.is_null() {
                    ptrmapPut(
                        pBt,
                        pgnoNew,
                        crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW2
                            as crate::src::ext::rtree::rtree::U8_0,
                        (*pPageOut).pgno,
                        &raw mut rc,
                    );
                }
                releasePage(pPageOut);
                pPageOut = pNew;
                if !pPageOut.is_null() {
                    pPgnoOut = (*pPageOut).aData;
                    crate::src::src::util::sqlite3Put4byte(
                        pPgnoOut,
                        0 as crate::src::ext::rtree::rtree::U32_0,
                    );
                    aOut = pPgnoOut.offset(4_isize) as *mut crate::src::ext::rtree::rtree::U8_0;
                    nOut = if (*pBt)
                        .usableSize
                        .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0)
                        < nRem
                    {
                        (*pBt)
                            .usableSize
                            .wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0)
                    } else {
                        nRem
                    };
                }
            }
            if !(nRem > 0 as crate::src::ext::rtree::rtree::U32_0
                && rc == crate::src::headers::sqlite3_h::SQLITE_OK)
            {
                break;
            }
        }
        releasePage(pPageOut);
        crate::src::src::pager::sqlite3PagerUnref(pPageIn as *mut crate::src::src::pcache::PgHdr);
        rc
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeDelete(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    flags: crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let __pCur_ref = unsafe { &mut *pCur };
    let p: *mut crate::src::headers::btreeInt_h::Btree = __pCur_ref.pBtree;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    let mut rc: ::core::ffi::c_int;
    
    let mut pCell: *mut ::core::ffi::c_uchar;
    
    
    let mut info: crate::src::headers::btreeInt_h::CellInfo = unsafe { ::core::mem::zeroed() };
    let mut bPreserve: crate::src::ext::rtree::rtree::U8_0;
    if __pCur_ref.eState as ::core::ffi::c_int != crate::src::headers::btreeInt_h::CURSOR_VALID {
        if __pCur_ref.eState as ::core::ffi::c_int
            >= crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK
        {
            rc = btreeRestoreCursorPosition(pCur);
            if rc != 0
                || __pCur_ref.eState as ::core::ffi::c_int
                    != crate::src::headers::btreeInt_h::CURSOR_VALID
            {
                return rc;
            }
        } else {
            return crate::src::src::main::sqlite3CorruptError(9826 as ::core::ffi::c_int);
        }
    }
    let iCellDepth: ::core::ffi::c_int = __pCur_ref.iPage as ::core::ffi::c_int;
    let iCellIdx: ::core::ffi::c_int = __pCur_ref.ix as ::core::ffi::c_int;
    let pPage: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
    if (*pPage).nCell as ::core::ffi::c_int <= iCellIdx {
        return crate::src::src::main::sqlite3CorruptError(9835 as ::core::ffi::c_int);
    }
    pCell = (*pPage).aData.offset(
        ((*pPage).maskPage as ::core::ffi::c_int
            & ((*((*pPage)
                .aCellIdx
                .offset((2 as ::core::ffi::c_int * iCellIdx) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0)
                .offset(0_isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *((*pPage)
                    .aCellIdx
                    .offset((2 as ::core::ffi::c_int * iCellIdx) as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(1_isize) as ::core::ffi::c_int)) as isize,
    ) as *mut ::core::ffi::c_uchar;
    if (*pPage).nFree < 0 as ::core::ffi::c_int && btreeComputeFreeSpace(pPage) != 0 {
        return crate::src::src::main::sqlite3CorruptError(9839 as ::core::ffi::c_int);
    }
    if pCell
        < (*pPage).aCellIdx.offset((*pPage).nCell as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0
    {
        return crate::src::src::main::sqlite3CorruptError(9842 as ::core::ffi::c_int);
    }
    bPreserve = (flags as ::core::ffi::c_int & crate::src::src::btree::BTREE_SAVEPOSITION
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as crate::src::ext::rtree::rtree::U8_0;
    if bPreserve != 0 {
        let __pPage_ref = unsafe { &mut *pPage };
        if __pPage_ref.leaf == 0
            || __pPage_ref.nFree
                + __pPage_ref.xCellSize.expect("non-null function pointer")(
                    pPage,
                    pCell as *mut crate::src::ext::rtree::rtree::U8_0,
                ) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                > (*pBt)
                    .usableSize
                    .wrapping_mul(2 as crate::src::ext::rtree::rtree::U32_0)
                    .wrapping_div(3 as crate::src::ext::rtree::rtree::U32_0)
                    as ::core::ffi::c_int
            || __pPage_ref.nCell as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        {
            rc = saveCursorKey(pCur);
            if rc != 0 {
                return rc;
            }
        } else {
            bPreserve = 2 as crate::src::ext::rtree::rtree::U8_0;
        }
    }
    if (*pPage).leaf == 0 {
        rc = sqlite3BtreePrevious(pCur, 0 as ::core::ffi::c_int);
        if rc != 0 {
            return rc;
        }
    }
    if __pCur_ref.curFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTCF_Multiple
        != 0
    {
        rc = saveAllCursors(pBt, __pCur_ref.pgnoRoot, pCur);
        if rc != 0 {
            return rc;
        }
    }
    if __pCur_ref.pKeyInfo.is_null() && (*p).hasIncrblobCur as ::core::ffi::c_int != 0 {
        invalidateIncrblobCursors(
            p,
            __pCur_ref.pgnoRoot,
            __pCur_ref.info.nKey,
            0 as ::core::ffi::c_int,
        );
    }
    rc = crate::src::src::pager::sqlite3PagerWrite(
        (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
    );
    if rc != 0 {
        return rc;
    }
    (*pPage).xParseCell.expect("non-null function pointer")(
        pPage,
        pCell as *mut crate::src::ext::rtree::rtree::U8_0,
        &raw mut info,
    );
    if info.nLocal as crate::src::ext::rtree::rtree::U32_0 != info.nPayload {
        rc = clearCellOverflow(pPage, pCell, &raw mut info);
    } else {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    dropCell(
        pPage,
        iCellIdx,
        info.nSize as ::core::ffi::c_int,
        &raw mut rc,
    );
    if rc != 0 {
        return rc;
    }
    if (*pPage).leaf == 0 {
        let pLeaf: *mut crate::src::headers::btreeInt_h::MemPage = __pCur_ref.pPage;
        
        let n: crate::src::src::pager::Pgno;
        
        let __pLeaf_ref = unsafe { &mut *pLeaf };
        if __pLeaf_ref.nFree < 0 as ::core::ffi::c_int {
            rc = btreeComputeFreeSpace(pLeaf);
            if rc != 0 {
                return rc;
            }
        }
        if iCellDepth < __pCur_ref.iPage as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
            n = (*__pCur_ref.apPage[(iCellDepth + 1 as ::core::ffi::c_int) as usize]).pgno;
        } else {
            n = (*__pCur_ref.pPage).pgno;
        }
        pCell = __pLeaf_ref.aData.offset(
            (__pLeaf_ref.maskPage as ::core::ffi::c_int
                & ((*(__pLeaf_ref.aCellIdx.offset(
                    (2 as ::core::ffi::c_int
                        * (__pLeaf_ref.nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                        as isize,
                ) as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset(0_isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(__pLeaf_ref.aCellIdx.offset(
                        (2 as ::core::ffi::c_int
                            * (__pLeaf_ref.nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                            as isize,
                    ) as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(1_isize) as ::core::ffi::c_int)) as isize,
        ) as *mut ::core::ffi::c_uchar;
        if pCell < __pLeaf_ref.aData.offset(4_isize) as *mut crate::src::ext::rtree::rtree::U8_0
        {
            return crate::src::src::main::sqlite3CorruptError(9933 as ::core::ffi::c_int);
        }
        let nCell: ::core::ffi::c_int = __pLeaf_ref.xCellSize.expect("non-null function pointer")(
            pLeaf,
            pCell as *mut crate::src::ext::rtree::rtree::U8_0,
        ) as ::core::ffi::c_int;
        let pTmp: *mut ::core::ffi::c_uchar = (*pBt).pTmpSpace as *mut ::core::ffi::c_uchar;
        rc = crate::src::src::pager::sqlite3PagerWrite(
            __pLeaf_ref.pDbPage as *mut crate::src::src::pcache::PgHdr,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = insertCell(
                pPage,
                iCellIdx,
                pCell.offset(-(4 as ::core::ffi::c_int as isize)),
                nCell + 4 as ::core::ffi::c_int,
                pTmp as *mut crate::src::ext::rtree::rtree::U8_0,
                n,
            );
        }
        dropCell(
            pLeaf,
            __pLeaf_ref.nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            nCell,
            &raw mut rc,
        );
        if rc != 0 {
            return rc;
        }
    }
    if (*__pCur_ref.pPage).nFree * 3 as ::core::ffi::c_int
        <= (*__pCur_ref.pBt).usableSize as ::core::ffi::c_int * 2 as ::core::ffi::c_int
    {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    } else {
        rc = balance(pCur);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && __pCur_ref.iPage as ::core::ffi::c_int > iCellDepth
    {
        releasePageNotNull(__pCur_ref.pPage);
        __pCur_ref.iPage -= 1;
        while __pCur_ref.iPage as ::core::ffi::c_int > iCellDepth {
            let fresh12 = __pCur_ref.iPage;
            __pCur_ref.iPage -= 1;
            releasePage(__pCur_ref.apPage[fresh12 as usize]);
        }
        __pCur_ref.pPage = __pCur_ref.apPage[__pCur_ref.iPage as usize];
        rc = balance(pCur);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if bPreserve as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_SKIPNEXT
                as crate::src::ext::rtree::rtree::U8_0;
            if iCellIdx >= (*pPage).nCell as ::core::ffi::c_int {
                __pCur_ref.skipNext = -(1 as ::core::ffi::c_int);
                __pCur_ref.ix = ((*pPage).nCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as crate::src::fts5::U16_0;
            } else {
                __pCur_ref.skipNext = 1 as ::core::ffi::c_int;
            }
        } else {
            rc = moveToRoot(pCur);
            if bPreserve != 0 {
                btreeReleaseAllCursorPages(pCur);
                __pCur_ref.eState = crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK
                    as crate::src::ext::rtree::rtree::U8_0;
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_EMPTY {
                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        }
    }
    rc
}

unsafe extern "C" fn btreeCreateTable(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    piTable: *mut crate::src::src::pager::Pgno,
    createTabFlags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    let mut pRoot: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let mut pgnoRoot: crate::src::src::pager::Pgno = 0;
    let mut rc: ::core::ffi::c_int;
    let ptfFlags: ::core::ffi::c_int;
    if (*pBt).autoVacuum != 0 {
        let mut pgnoMove: crate::src::src::pager::Pgno = 0;
        let mut pPageMove: *mut crate::src::headers::btreeInt_h::MemPage =
            ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
        invalidateAllOverflowCache(pBt);
        sqlite3BtreeGetMeta(
            p,
            crate::src::src::btree::BTREE_LARGEST_ROOT_PAGE,
            &raw mut pgnoRoot,
        );
        if pgnoRoot > btreePagecount(pBt) {
            return crate::src::src::main::sqlite3CorruptError(10049 as ::core::ffi::c_int);
        }
        pgnoRoot = pgnoRoot.wrapping_add(1);
        while pgnoRoot == ptrmapPageno(pBt, pgnoRoot)
            || pgnoRoot
                == (crate::src::src::global::sqlite3PendingByte
                    as crate::src::ext::rtree::rtree::U32_0)
                    .wrapping_div((*pBt).pageSize)
                    .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
        {
            pgnoRoot = pgnoRoot.wrapping_add(1);
        }
        rc = allocateBtreePage(
            pBt,
            &raw mut pPageMove,
            &raw mut pgnoMove,
            pgnoRoot,
            BTALLOC_EXACT as crate::src::ext::rtree::rtree::U8_0,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        if pgnoMove != pgnoRoot {
            let mut eType: crate::src::ext::rtree::rtree::U8_0 =
                0 as crate::src::ext::rtree::rtree::U8_0;
            let mut iPtrPage: crate::src::src::pager::Pgno = 0 as crate::src::src::pager::Pgno;
            rc = saveAllCursors(
                pBt,
                0 as crate::src::src::pager::Pgno,
                ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>(),
            );
            releasePage(pPageMove);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            rc = btreeGetPage(pBt, pgnoRoot, &raw mut pRoot, 0 as ::core::ffi::c_int);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            rc = ptrmapGet(pBt, pgnoRoot, &raw mut eType, &raw mut iPtrPage);
            if eType as ::core::ffi::c_int == crate::src::headers::btreeInt_h::PTRMAP_ROOTPAGE
                || eType as ::core::ffi::c_int == crate::src::headers::btreeInt_h::PTRMAP_FREEPAGE
            {
                rc = crate::src::src::main::sqlite3CorruptError(10097 as ::core::ffi::c_int);
            }
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                releasePage(pRoot);
                return rc;
            }
            rc = relocatePage(
                pBt,
                pRoot,
                eType,
                iPtrPage,
                pgnoMove,
                0 as ::core::ffi::c_int,
            );
            releasePage(pRoot);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            rc = btreeGetPage(pBt, pgnoRoot, &raw mut pRoot, 0 as ::core::ffi::c_int);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            rc = crate::src::src::pager::sqlite3PagerWrite(
                (*pRoot).pDbPage as *mut crate::src::src::pcache::PgHdr,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                releasePage(pRoot);
                return rc;
            }
        } else {
            pRoot = pPageMove;
        }
        ptrmapPut(
            pBt,
            pgnoRoot,
            crate::src::headers::btreeInt_h::PTRMAP_ROOTPAGE as crate::src::ext::rtree::rtree::U8_0,
            0 as crate::src::src::pager::Pgno,
            &raw mut rc,
        );
        if rc != 0 {
            releasePage(pRoot);
            return rc;
        }
        rc = sqlite3BtreeUpdateMeta(
            p,
            4 as ::core::ffi::c_int,
            pgnoRoot as crate::src::ext::rtree::rtree::U32_0,
        );
        if rc != 0 {
            releasePage(pRoot);
            return rc;
        }
    } else {
        rc = allocateBtreePage(
            pBt,
            &raw mut pRoot,
            &raw mut pgnoRoot,
            1 as crate::src::src::pager::Pgno,
            0 as crate::src::ext::rtree::rtree::U8_0,
        );
        if rc != 0 {
            return rc;
        }
    }
    if createTabFlags & crate::src::src::btree::BTREE_INTKEY != 0 {
        ptfFlags = crate::src::headers::btreeInt_h::PTF_INTKEY
            | crate::src::headers::btreeInt_h::PTF_LEAFDATA
            | crate::src::headers::btreeInt_h::PTF_LEAF;
    } else {
        ptfFlags = crate::src::headers::btreeInt_h::PTF_ZERODATA
            | crate::src::headers::btreeInt_h::PTF_LEAF;
    }
    zeroPage(pRoot, ptfFlags);
    crate::src::src::pager::sqlite3PagerUnref(
        (*pRoot).pDbPage as *mut crate::src::src::pcache::PgHdr,
    );
    *piTable = pgnoRoot;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCreateTable(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    piTable: *mut crate::src::src::pager::Pgno,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let rc: ::core::ffi::c_int = btreeCreateTable(p, piTable, flags);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}

unsafe extern "C" fn clearDatabasePage(
    pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pgno: crate::src::src::pager::Pgno,
    freePageFlag: ::core::ffi::c_int,
    mut pnChange: *mut crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let mut rc: ::core::ffi::c_int;
    let mut pCell: *mut ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int;
    let hdr: ::core::ffi::c_int;
    let mut info: crate::src::headers::btreeInt_h::CellInfo = unsafe { ::core::mem::zeroed() };
    if pgno > btreePagecount(pBt) {
        return crate::src::src::main::sqlite3CorruptError(10187 as ::core::ffi::c_int);
    }
    rc = getAndInitPage(pBt, pgno, &raw mut pPage, 0 as ::core::ffi::c_int);
    if rc != 0 {
        return rc;
    }
    if (*pBt).openFlags as ::core::ffi::c_int & crate::src::src::btree::BTREE_SINGLE
        == 0 as ::core::ffi::c_int
        && crate::src::src::pager::sqlite3PagerPageRefcount(
            (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
        ) != 1 as ::core::ffi::c_int
            + (pgno == 1 as crate::src::src::pager::Pgno) as ::core::ffi::c_int
    {
        rc = crate::src::src::main::sqlite3CorruptError(10194 as ::core::ffi::c_int);
    } else {
        hdr = (*pPage).hdrOffset as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        loop {
            let __pPage_ref = unsafe { &mut *pPage };
            if (i >= __pPage_ref.nCell as ::core::ffi::c_int) {
                current_block = 5783071609795492627;
                break;
            }
            pCell = __pPage_ref.aData.offset(
                (__pPage_ref.maskPage as ::core::ffi::c_int
                    & ((*((*pPage)
                        .aCellIdx
                        .offset((2 as ::core::ffi::c_int * i) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(0_isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *((*pPage)
                            .aCellIdx
                            .offset((2 as ::core::ffi::c_int * i) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(1_isize) as ::core::ffi::c_int)) as isize,
            ) as *mut ::core::ffi::c_uchar;
            if __pPage_ref.leaf == 0 {
                rc = clearDatabasePage(
                    pBt,
                    crate::src::src::util::sqlite3Get4byte(pCell) as crate::src::src::pager::Pgno,
                    1 as ::core::ffi::c_int,
                    pnChange,
                );
                if rc != 0 {
                    current_block = 8909377133154860393;
                    break;
                }
            }
            __pPage_ref.xParseCell.expect("non-null function pointer")(
                pPage,
                pCell as *mut crate::src::ext::rtree::rtree::U8_0,
                &raw mut info,
            );
            if info.nLocal as crate::src::ext::rtree::rtree::U32_0 != info.nPayload {
                rc = clearCellOverflow(pPage, pCell, &raw mut info);
            } else {
                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
            }
            if rc != 0 {
                current_block = 8909377133154860393;
                break;
            }
            i += 1;
        }
        match current_block {
            8909377133154860393 => {}
            _ => {
                if (*pPage).leaf == 0 {
                    rc = clearDatabasePage(
                        pBt,
                        crate::src::src::util::sqlite3Get4byte(
                            (*pPage)
                                .aData
                                .offset((hdr + 8 as ::core::ffi::c_int) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0,
                        ) as crate::src::src::pager::Pgno,
                        1 as ::core::ffi::c_int,
                        pnChange,
                    );
                    if rc != 0 {
                        current_block = 8909377133154860393;
                    } else {
                        if (*pPage).intKey != 0 {
                            pnChange =
                                ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::I64_0>();
                        }
                        current_block = 4068382217303356765;
                    }
                } else {
                    current_block = 4068382217303356765;
                }
                match current_block {
                    8909377133154860393 => {}
                    _ => {
                        if !pnChange.is_null() {
                            *pnChange += (*pPage).nCell as crate::src::ext::rtree::rtree::I64_0;
                        }
                        if freePageFlag != 0 {
                            freePage(pPage, &raw mut rc);
                        } else {
                            rc = crate::src::src::pager::sqlite3PagerWrite(
                                (*pPage).pDbPage as *mut crate::src::src::pcache::PgHdr,
                            );
                            if rc == 0 as ::core::ffi::c_int {
                                zeroPage(
                                    pPage,
                                    *(*pPage).aData.offset(hdr as isize) as ::core::ffi::c_int
                                        | crate::src::headers::btreeInt_h::PTF_LEAF,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    releasePage(pPage);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeClearTable(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    iTable: ::core::ffi::c_int,
    pnChange: *mut crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc = saveAllCursors(
        pBt,
        iTable as crate::src::src::pager::Pgno,
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>(),
    );
    if crate::src::headers::sqlite3_h::SQLITE_OK == rc {
        if (*p).hasIncrblobCur != 0 {
            invalidateIncrblobCursors(
                p,
                iTable as crate::src::src::pager::Pgno,
                0 as crate::src::ext::rtree::rtree::I64_0,
                1 as ::core::ffi::c_int,
            );
        }
        rc = clearDatabasePage(
            pBt,
            iTable as crate::src::src::pager::Pgno,
            0 as ::core::ffi::c_int,
            pnChange,
        );
    }
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeClearTableOfCursor(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) -> ::core::ffi::c_int {
    sqlite3BtreeClearTable(
        (*pCur).pBtree,
        (*pCur).pgnoRoot as ::core::ffi::c_int,
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::I64_0>(),
    )
}

unsafe extern "C" fn btreeDropTable(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    iTable: crate::src::src::pager::Pgno,
    piMoved: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    if iTable > btreePagecount(pBt) {
        return crate::src::src::main::sqlite3CorruptError(10298 as ::core::ffi::c_int);
    }
    rc = sqlite3BtreeClearTable(
        p,
        iTable as ::core::ffi::c_int,
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::I64_0>(),
    );
    if rc != 0 {
        return rc;
    }
    rc = btreeGetPage(pBt, iTable, &raw mut pPage, 0 as ::core::ffi::c_int);
    if rc != 0 {
        releasePage(pPage);
        return rc;
    }
    *piMoved = 0 as ::core::ffi::c_int;
    if (*pBt).autoVacuum != 0 {
        let mut maxRootPgno: crate::src::src::pager::Pgno = 0;
        sqlite3BtreeGetMeta(
            p,
            crate::src::src::btree::BTREE_LARGEST_ROOT_PAGE,
            &raw mut maxRootPgno,
        );
        if iTable == maxRootPgno {
            freePage(pPage, &raw mut rc);
            releasePage(pPage);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
        } else {
            let mut pMove: *mut crate::src::headers::btreeInt_h::MemPage =
                ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
            releasePage(pPage);
            rc = btreeGetPage(pBt, maxRootPgno, &raw mut pMove, 0 as ::core::ffi::c_int);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            rc = relocatePage(
                pBt,
                pMove,
                crate::src::headers::btreeInt_h::PTRMAP_ROOTPAGE
                    as crate::src::ext::rtree::rtree::U8_0,
                0 as crate::src::src::pager::Pgno,
                iTable,
                0 as ::core::ffi::c_int,
            );
            releasePage(pMove);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            pMove = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
            rc = btreeGetPage(pBt, maxRootPgno, &raw mut pMove, 0 as ::core::ffi::c_int);
            freePage(pMove, &raw mut rc);
            releasePage(pMove);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            *piMoved = maxRootPgno as ::core::ffi::c_int;
        }
        maxRootPgno = maxRootPgno.wrapping_sub(1);
        while maxRootPgno
            == (crate::src::src::global::sqlite3PendingByte as crate::src::ext::rtree::rtree::U32_0)
                .wrapping_div((*pBt).pageSize)
                .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0)
            || ptrmapPageno(pBt, maxRootPgno) == maxRootPgno
        {
            maxRootPgno = maxRootPgno.wrapping_sub(1);
        }
        rc = sqlite3BtreeUpdateMeta(
            p,
            4 as ::core::ffi::c_int,
            maxRootPgno as crate::src::ext::rtree::rtree::U32_0,
        );
    } else {
        freePage(pPage, &raw mut rc);
        releasePage(pPage);
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeDropTable(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    iTable: ::core::ffi::c_int,
    piMoved: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let rc: ::core::ffi::c_int = btreeDropTable(p, iTable as crate::src::src::pager::Pgno, piMoved);
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeGetMeta(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    idx: ::core::ffi::c_int,
    pMeta: *mut crate::src::ext::rtree::rtree::U32_0,
) {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    if idx == crate::src::src::btree::BTREE_DATA_VERSION {
        *pMeta = crate::src::src::pager::sqlite3PagerDataVersion((*pBt).pPager)
            .wrapping_add((*p).iBDataVersion);
    } else {
        *pMeta = crate::src::src::util::sqlite3Get4byte(
            (*(*pBt).pPage1)
                .aData
                .offset((36 as ::core::ffi::c_int + idx * 4 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0,
        );
    }
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeUpdateMeta(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    idx: ::core::ffi::c_int,
    iMeta: crate::src::ext::rtree::rtree::U32_0,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    
    
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let __pPage1_ref = &*(*pBt).pPage1;
    let pP1: *mut ::core::ffi::c_uchar = __pPage1_ref.aData as *mut ::core::ffi::c_uchar;
    let rc: ::core::ffi::c_int = crate::src::src::pager::sqlite3PagerWrite(
        __pPage1_ref.pDbPage as *mut crate::src::src::pcache::PgHdr,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::util::sqlite3Put4byte(
            pP1.offset((36 as ::core::ffi::c_int + idx * 4 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0,
            iMeta,
        );
        if idx == crate::src::src::btree::BTREE_INCR_VACUUM {
            (*pBt).incrVacuum = iMeta as crate::src::ext::rtree::rtree::U8_0;
        }
    }
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCount(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
    pnEntry: *mut crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    let mut nEntry: crate::src::ext::rtree::rtree::I64_0 =
        0 as crate::src::ext::rtree::rtree::I64_0;
    let mut rc: ::core::ffi::c_int;
    rc = moveToRoot(pCur);
    if rc == crate::src::headers::sqlite3_h::SQLITE_EMPTY {
        *pnEntry = 0 as crate::src::ext::rtree::rtree::I64_0;
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && (*((&raw mut (*db).u1.isInterrupted) as *mut std::sync::atomic::AtomicI32))
            .load(std::sync::atomic::Ordering::Relaxed)
            == 0
    {
        
        let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage;
        pPage = (*pCur).pPage;
        if (*pPage).leaf as ::core::ffi::c_int != 0 || (*pPage).intKey == 0 {
            nEntry += (*pPage).nCell as crate::src::ext::rtree::rtree::I64_0;
        }
        if (*pPage).leaf != 0 {
            let __pCur_ref = unsafe { &mut *pCur };
            loop {
                if __pCur_ref.iPage as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    *pnEntry = nEntry;
                    return moveToRoot(pCur);
                }
                moveToParent(pCur);
                if ((__pCur_ref.ix as ::core::ffi::c_int) < (*__pCur_ref.pPage).nCell as ::core::ffi::c_int)
                {
                    break;
                }
            }
            __pCur_ref.ix = __pCur_ref.ix.wrapping_add(1);
            pPage = __pCur_ref.pPage;
        }
        let iIdx: ::core::ffi::c_int = (*pCur).ix as ::core::ffi::c_int;
        if iIdx == (*pPage).nCell as ::core::ffi::c_int {
            rc = moveToChild(
                pCur,
                crate::src::src::util::sqlite3Get4byte((*pPage).aData.offset(
                    ((*pPage).hdrOffset as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                    as *mut crate::src::ext::rtree::rtree::U8_0),
            );
        } else {
            let __pPage_ref = unsafe { &mut *pPage };
            rc = moveToChild(
                pCur,
                crate::src::src::util::sqlite3Get4byte(
                    __pPage_ref.aData.offset(
                        (__pPage_ref.maskPage as ::core::ffi::c_int
                            & ((*((*pPage)
                                .aCellIdx
                                .offset((2 as ::core::ffi::c_int * iIdx) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(0_isize)
                                as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int
                                | *((*pPage)
                                    .aCellIdx
                                    .offset((2 as ::core::ffi::c_int * iIdx) as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0)
                                    .offset(1_isize)
                                    as ::core::ffi::c_int)) as isize,
                    ),
                ),
            );
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreePager(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> *mut crate::src::src::pager::Pager {
    (*(*p).pBt).pPager as *mut crate::src::src::pager::Pager
}

pub unsafe extern "C" fn checkOom(pCheck: *mut crate::src::headers::btreeInt_h::IntegrityCk) {
    let __pCheck_ref = unsafe { &mut *pCheck };
    __pCheck_ref.rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    __pCheck_ref.mxErr = 0 as ::core::ffi::c_int;
    if __pCheck_ref.nErr == 0 as ::core::ffi::c_int {
        __pCheck_ref.nErr += 1;
    }
}

pub unsafe extern "C" fn checkProgress(
    pCheck: *mut crate::src::headers::btreeInt_h::IntegrityCk,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pCheck).db;
    if (*((&raw mut (*db).u1.isInterrupted) as *mut std::sync::atomic::AtomicI32))
        .load(std::sync::atomic::Ordering::Relaxed)
        != 0
    {
        let __pCheck_ref = unsafe { &mut *pCheck };
        __pCheck_ref.rc = crate::src::headers::sqlite3_h::SQLITE_INTERRUPT;
        __pCheck_ref.nErr += 1;
        __pCheck_ref.mxErr = 0 as ::core::ffi::c_int;
    }
    if (*db).xProgress.is_some() {
        let __pCheck_ref = unsafe { &mut *pCheck };
        __pCheck_ref.nStep = __pCheck_ref.nStep.wrapping_add(1);
        let __db_ref = unsafe { &mut *db };
        if __pCheck_ref
            .nStep
            .wrapping_rem(__db_ref.nProgressOps as crate::src::ext::rtree::rtree::U32_0)
            == 0 as crate::src::ext::rtree::rtree::U32_0
            && __db_ref.xProgress.expect("non-null function pointer")(__db_ref.pProgressArg) != 0
        {
            __pCheck_ref.rc = crate::src::headers::sqlite3_h::SQLITE_INTERRUPT;
            __pCheck_ref.nErr += 1;
            __pCheck_ref.mxErr = 0 as ::core::ffi::c_int;
        }
    }
}

unsafe extern "C" fn getPageReferenced(
    pCheck: *mut crate::src::headers::btreeInt_h::IntegrityCk,
    iPg: crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    *(*pCheck)
        .aPgRef
        .offset(iPg.wrapping_div(8 as crate::src::src::pager::Pgno) as isize)
        as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << (iPg & 0x7 as crate::src::src::pager::Pgno)
}

unsafe extern "C" fn setPageReferenced(
    pCheck: *mut crate::src::headers::btreeInt_h::IntegrityCk,
    iPg: crate::src::src::pager::Pgno,
) {
    let fresh33 = &mut *(*pCheck)
        .aPgRef
        .offset(iPg.wrapping_div(8 as crate::src::src::pager::Pgno) as isize);
    *fresh33 = (*fresh33 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << (iPg & 0x7 as crate::src::src::pager::Pgno))
        as crate::src::ext::rtree::rtree::U8_0;
}

/// Non-variadic version of checkAppendMsg — takes pre-extracted PrintfArg slice.
/// Avoids c_variadic dependency for callers that can build args directly.
unsafe fn checkAppendMsg_args(
    pCheck: *mut crate::src::headers::btreeInt_h::IntegrityCk,
    zFormat: *const ::core::ffi::c_char,
    args: &[crate::src::src::printf::PrintfArg],
) {
    checkProgress(pCheck);
    let pck = unsafe { &mut *pCheck };
    if pck.mxErr == 0 {
        return;
    }
    pck.mxErr -= 1;
    pck.nErr += 1;
    if pck.errMsg.nChar != 0 {
        crate::src::src::printf::sqlite3_str_append(
            &raw mut pck.errMsg as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
    }
    if !pck.zPfx.is_null() {
        let pfx_args = vec![
            crate::src::src::printf::PrintfArg::from(pck.v0 as ::core::ffi::c_int),
            crate::src::src::printf::PrintfArg::from(pck.v1 as ::core::ffi::c_int),
            crate::src::src::printf::PrintfArg::from(pck.v2 as ::core::ffi::c_int),
        ];
        crate::src::src::printf::sqlite3_str_vappendf_args(
            &raw mut pck.errMsg as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
            pck.zPfx,
            &pfx_args,
        );
    }
    crate::src::src::printf::sqlite3_str_vappendf_args(&raw mut pck.errMsg, zFormat, args);
    if pck.errMsg.accError as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_NOMEM {
        checkOom(pCheck);
    }
}

unsafe extern "C" fn checkRef(
    pCheck: *mut crate::src::headers::btreeInt_h::IntegrityCk,
    iPage: crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    if iPage > (*pCheck).nCkPage || iPage == 0 as crate::src::src::pager::Pgno {
        checkAppendMsg_args(
            pCheck,
            b"invalid page number %u\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::from(iPage)],
        );
        return 1 as ::core::ffi::c_int;
    }
    if getPageReferenced(pCheck, iPage) != 0 {
        checkAppendMsg_args(
            pCheck,
            b"2nd reference to page %u\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::from(iPage)],
        );
        return 1 as ::core::ffi::c_int;
    }
    setPageReferenced(pCheck, iPage);
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn checkPtrmap(
    pCheck: *mut crate::src::headers::btreeInt_h::IntegrityCk,
    iChild: crate::src::src::pager::Pgno,
    eType: crate::src::ext::rtree::rtree::U8_0,
    iParent: crate::src::src::pager::Pgno,
) {
    
    let mut ePtrmapType: crate::src::ext::rtree::rtree::U8_0 = 0;
    let mut iPtrmapParent: crate::src::src::pager::Pgno = 0;
    let rc: ::core::ffi::c_int = ptrmapGet(
        (*pCheck).pBt,
        iChild,
        &raw mut ePtrmapType,
        &raw mut iPtrmapParent,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        if rc == crate::src::headers::sqlite3_h::SQLITE_NOMEM
            || rc == crate::src::headers::sqlite3_h::SQLITE_IOERR_NOMEM
        {
            checkOom(pCheck);
        }
        checkAppendMsg_args(
            pCheck,
            b"Failed to read ptrmap key=%u\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::from(iChild)],
        );
        return;
    }
    if ePtrmapType as ::core::ffi::c_int != eType as ::core::ffi::c_int || iPtrmapParent != iParent
    {
        checkAppendMsg_args(
            pCheck,
            b"Bad ptr map entry key=%u expected=(%u,%u) got=(%u,%u)\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::from(iChild),
                crate::src::src::printf::PrintfArg::from(eType as ::core::ffi::c_uint),
                crate::src::src::printf::PrintfArg::from(iParent),
                crate::src::src::printf::PrintfArg::from(ePtrmapType as ::core::ffi::c_uint),
                crate::src::src::printf::PrintfArg::from(iPtrmapParent),
            ],
        );
    }
}

unsafe extern "C" fn checkList(
    pCheck: *mut crate::src::headers::btreeInt_h::IntegrityCk,
    isFreeList: ::core::ffi::c_int,
    mut iPage: crate::src::src::pager::Pgno,
    mut N: crate::src::ext::rtree::rtree::U32_0,
) {
    let mut i: ::core::ffi::c_int;
    let expected: crate::src::ext::rtree::rtree::U32_0 = N;
    let __pCheck_ref = unsafe { &mut *pCheck };
    let nErrAtStart: ::core::ffi::c_int = __pCheck_ref.nErr;
    while iPage != 0 as crate::src::src::pager::Pgno && __pCheck_ref.mxErr != 0 {
        let mut pOvflPage: *mut crate::src::src::pager::DbPage =
            ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
        let pOvflData: *mut ::core::ffi::c_uchar;
        if checkRef(pCheck, iPage) != 0 {
            break;
        }
        N = N.wrapping_sub(1);
        if crate::src::src::pager::sqlite3PagerGet(
            __pCheck_ref.pPager,
            iPage,
            &raw mut pOvflPage as *mut _ as *mut *mut crate::src::src::pcache::PgHdr,
            0 as ::core::ffi::c_int,
        ) != 0
        {
            checkAppendMsg_args(
                pCheck,
                b"failed to get page %u\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::from(iPage)],
            );
            break;
        } else {
            pOvflData = crate::src::src::pager::sqlite3PagerGetData(
                pOvflPage as *mut crate::src::src::pcache::PgHdr,
            ) as *mut ::core::ffi::c_uchar;
            if isFreeList != 0 {
                let n: crate::src::ext::rtree::rtree::U32_0 =
                    crate::src::src::util::sqlite3Get4byte(
                        pOvflData.offset(4_isize) as *mut ::core::ffi::c_uchar
                    );
                if (*__pCheck_ref.pBt).autoVacuum != 0 {
                    checkPtrmap(
                        pCheck,
                        iPage,
                        crate::src::headers::btreeInt_h::PTRMAP_FREEPAGE
                            as crate::src::ext::rtree::rtree::U8_0,
                        0 as crate::src::src::pager::Pgno,
                    );
                }
                if n > (*__pCheck_ref.pBt)
                    .usableSize
                    .wrapping_div(4 as crate::src::ext::rtree::rtree::U32_0)
                    .wrapping_sub(2 as crate::src::ext::rtree::rtree::U32_0)
                {
                    checkAppendMsg_args(
                        pCheck,
                        b"freelist leaf count too big on page %u\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[crate::src::src::printf::PrintfArg::from(iPage)],
                    );
                    N = N.wrapping_sub(1);
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while i < n as ::core::ffi::c_int {
                        let iFreePage: crate::src::src::pager::Pgno =
                            crate::src::src::util::sqlite3Get4byte(pOvflData.offset(
                                (8 as ::core::ffi::c_int + i * 4 as ::core::ffi::c_int) as isize,
                            )
                                as *mut ::core::ffi::c_uchar)
                                as crate::src::src::pager::Pgno;
                        if (*__pCheck_ref.pBt).autoVacuum != 0 {
                            checkPtrmap(
                                pCheck,
                                iFreePage,
                                crate::src::headers::btreeInt_h::PTRMAP_FREEPAGE
                                    as crate::src::ext::rtree::rtree::U8_0,
                                0 as crate::src::src::pager::Pgno,
                            );
                        }
                        checkRef(pCheck, iFreePage);
                        i += 1;
                    }
                    N = N.wrapping_sub(n);
                }
            } else if (*__pCheck_ref.pBt).autoVacuum as ::core::ffi::c_int != 0
                && N > 0 as crate::src::ext::rtree::rtree::U32_0
            {
                i = crate::src::src::util::sqlite3Get4byte(pOvflData) as ::core::ffi::c_int;
                checkPtrmap(
                    pCheck,
                    i as crate::src::src::pager::Pgno,
                    crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW2
                        as crate::src::ext::rtree::rtree::U8_0,
                    iPage,
                );
            }
            iPage =
                crate::src::src::util::sqlite3Get4byte(pOvflData) as crate::src::src::pager::Pgno;
            crate::src::src::pager::sqlite3PagerUnref(
                pOvflPage as *mut crate::src::src::pcache::PgHdr,
            );
        }
    }
    if N != 0 && nErrAtStart == __pCheck_ref.nErr {
        checkAppendMsg_args(
            pCheck,
            b"%s is %u but should be %u\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::from(if isFreeList != 0 {
                    b"size\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"overflow list length\0" as *const u8 as *const ::core::ffi::c_char
                }),
                crate::src::src::printf::PrintfArg::from(expected.wrapping_sub(N)),
                crate::src::src::printf::PrintfArg::from(expected),
            ],
        );
    }
}

unsafe extern "C" fn btreeHeapInsert(
    aHeap: *mut crate::src::ext::rtree::rtree::U32_0,
    mut x: crate::src::ext::rtree::rtree::U32_0,
) {
    let mut j: crate::src::ext::rtree::rtree::U32_0;
    let mut i: crate::src::ext::rtree::rtree::U32_0;
    let fresh32 = &mut *aHeap.offset(0_isize);
    *fresh32 = (*fresh32).wrapping_add(1);
    i = *fresh32;
    *aHeap.offset(i as isize) = x;
    loop {
        j = i.wrapping_div(2 as crate::src::ext::rtree::rtree::U32_0);
        if !(j > 0 as crate::src::ext::rtree::rtree::U32_0
            && *aHeap.offset(j as isize) > *aHeap.offset(i as isize))
        {
            break;
        }
        x = *aHeap.offset(j as isize);
        *aHeap.offset(j as isize) = *aHeap.offset(i as isize);
        *aHeap.offset(i as isize) = x;
        i = j;
    }
}

unsafe extern "C" fn btreeHeapPull(
    aHeap: *mut crate::src::ext::rtree::rtree::U32_0,
    pOut: *mut crate::src::ext::rtree::rtree::U32_0,
) -> ::core::ffi::c_int {
    let mut j: crate::src::ext::rtree::rtree::U32_0;
    let mut i: crate::src::ext::rtree::rtree::U32_0;
    let mut x: crate::src::ext::rtree::rtree::U32_0;
    x = *aHeap.offset(0_isize);
    if x == 0 as crate::src::ext::rtree::rtree::U32_0 {
        return 0 as ::core::ffi::c_int;
    }
    *pOut = *aHeap.offset(1_isize);
    *aHeap.offset(1_isize) = *aHeap.offset(x as isize);
    *aHeap.offset(x as isize) =
        0xffffffff as ::core::ffi::c_uint as crate::src::ext::rtree::rtree::U32_0;
    let fresh31 = &mut *aHeap.offset(0_isize);
    *fresh31 = (*fresh31).wrapping_sub(1);
    i = 1 as crate::src::ext::rtree::rtree::U32_0;
    loop {
        j = i.wrapping_mul(2 as crate::src::ext::rtree::rtree::U32_0);
        if (j > *aHeap.offset(0_isize)) {
            break;
        }
        if *aHeap.offset(j as isize)
            > *aHeap.offset(j.wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0) as isize)
        {
            j = j.wrapping_add(1);
        }
        if *aHeap.offset(i as isize) < *aHeap.offset(j as isize) {
            break;
        }
        x = *aHeap.offset(i as isize);
        *aHeap.offset(i as isize) = *aHeap.offset(j as isize);
        *aHeap.offset(j as isize) = x;
        i = j;
    }
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn checkTreePage(
    pCheck: *mut crate::src::headers::btreeInt_h::IntegrityCk,
    iPage: crate::src::src::pager::Pgno,
    piMinKey: *mut crate::src::ext::rtree::rtree::I64_0,
    mut maxKey: crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    let mut pPage: *mut crate::src::headers::btreeInt_h::MemPage =
        ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::MemPage>();
    let mut i: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    let mut depth: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut d2: ::core::ffi::c_int;
    let mut pgno: ::core::ffi::c_int;
    let mut nFrag: ::core::ffi::c_int;
    let hdr: ::core::ffi::c_int;
    let cellStart: ::core::ffi::c_int;
    let nCell: ::core::ffi::c_int;
    let mut doCoverageCheck: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut keyCanBeEqual: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let data: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut pCell: *mut crate::src::ext::rtree::rtree::U8_0;
    let mut pCellIdx: *mut crate::src::ext::rtree::rtree::U8_0;
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared;
    let mut pc: crate::src::ext::rtree::rtree::U32_0;
    let usableSize: crate::src::ext::rtree::rtree::U32_0;
    let contentOffset: crate::src::ext::rtree::rtree::U32_0;
    let mut heap: *mut crate::src::ext::rtree::rtree::U32_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U32_0>();
    let mut x: crate::src::ext::rtree::rtree::U32_0 = 0;
    let mut prev: crate::src::ext::rtree::rtree::U32_0;
    let __pCheck_ref = unsafe { &mut *pCheck };
    let saved_zPfx: *const ::core::ffi::c_char = __pCheck_ref.zPfx;
    let saved_v1: ::core::ffi::c_int = __pCheck_ref.v1 as ::core::ffi::c_int;
    let saved_v2: ::core::ffi::c_int = __pCheck_ref.v2;
    let mut savedIsInit: crate::src::ext::rtree::rtree::U8_0 =
        0 as crate::src::ext::rtree::rtree::U8_0;
    checkProgress(pCheck);
    if (__pCheck_ref.mxErr != 0 as ::core::ffi::c_int) {
        pBt = __pCheck_ref.pBt;
        usableSize = (*pBt).usableSize;
        if iPage == 0 as crate::src::src::pager::Pgno {
            return 0 as ::core::ffi::c_int;
        }
        if checkRef(pCheck, iPage) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        __pCheck_ref.zPfx = b"Tree %u page %u: \0" as *const u8 as *const ::core::ffi::c_char;
        __pCheck_ref.v1 = iPage;
        rc = btreeGetPage(pBt, iPage, &raw mut pPage, 0 as ::core::ffi::c_int);
        if rc != 0 as ::core::ffi::c_int {
            checkAppendMsg_args(
                pCheck,
                b"unable to get the page. error code=%d\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::from(rc)],
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_IOERR_NOMEM {
                __pCheck_ref.rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
        } else {
            savedIsInit = (*pPage).isInit;
            (*pPage).isInit = 0 as crate::src::ext::rtree::rtree::U8_0;
            rc = btreeInitPage(pPage);
            if rc != 0 as ::core::ffi::c_int {
                checkAppendMsg_args(
                    pCheck,
                    b"btreeInitPage() returns error code %d\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::src::src::printf::PrintfArg::from(rc)],
                );
            } else {
                rc = btreeComputeFreeSpace(pPage);
                if rc != 0 as ::core::ffi::c_int {
                    checkAppendMsg_args(
                        pCheck,
                        b"free space corruption\0" as *const u8 as *const ::core::ffi::c_char,
                        &[],
                    );
                } else {
                    let __pPage_ref = unsafe { &mut *pPage };
                    data = __pPage_ref.aData;
                    hdr = __pPage_ref.hdrOffset as ::core::ffi::c_int;
                    __pCheck_ref.zPfx =
                        b"Tree %u page %u cell %u: \0" as *const u8 as *const ::core::ffi::c_char;
                    contentOffset = (((((*(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(0_isize)
                        as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(data.offset((hdr + 5 as ::core::ffi::c_int) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(1_isize) as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) & 0xffff as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U32_0;
                    nCell = (*(data.offset((hdr + 3 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(0_isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(data.offset((hdr + 3 as ::core::ffi::c_int) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(1_isize) as ::core::ffi::c_int;
                    if __pPage_ref.leaf as ::core::ffi::c_int != 0
                        || __pPage_ref.intKey as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        __pCheck_ref.nRow += nCell as crate::src::ext::rtree::rtree::I64_0;
                    }
                    cellStart = hdr + 12 as ::core::ffi::c_int
                        - 4 as ::core::ffi::c_int * __pPage_ref.leaf as ::core::ffi::c_int;
                    pCellIdx = data.offset(
                        (cellStart + 2 as ::core::ffi::c_int * (nCell - 1 as ::core::ffi::c_int))
                            as isize,
                    ) as *mut crate::src::ext::rtree::rtree::U8_0;
                    if __pPage_ref.leaf == 0 {
                        pgno = crate::src::src::util::sqlite3Get4byte(
                            data.offset((hdr + 8 as ::core::ffi::c_int) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0,
                        ) as ::core::ffi::c_int;
                        if (*pBt).autoVacuum != 0 {
                            __pCheck_ref.zPfx = b"Tree %u page %u right child: \0" as *const u8
                                as *const ::core::ffi::c_char;
                            checkPtrmap(
                                pCheck,
                                pgno as crate::src::src::pager::Pgno,
                                crate::src::headers::btreeInt_h::PTRMAP_BTREE
                                    as crate::src::ext::rtree::rtree::U8_0,
                                iPage,
                            );
                        }
                        depth = checkTreePage(
                            pCheck,
                            pgno as crate::src::src::pager::Pgno,
                            &raw mut maxKey,
                            maxKey,
                        );
                        keyCanBeEqual = 0 as ::core::ffi::c_int;
                    } else {
                        heap = __pCheck_ref.heap;
                        *heap.offset(0_isize) = 0 as crate::src::ext::rtree::rtree::U32_0;
                    }
                    i = nCell - 1 as ::core::ffi::c_int;
                    while i >= 0 as ::core::ffi::c_int && __pCheck_ref.mxErr != 0 {
                        let mut info: crate::src::headers::btreeInt_h::CellInfo =
                            unsafe { ::core::mem::zeroed() };
                        __pCheck_ref.v2 = i;
                        pc = ((*pCellIdx.offset(0_isize) as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *pCellIdx.offset(1_isize) as ::core::ffi::c_int)
                            as crate::src::ext::rtree::rtree::U32_0;
                        pCellIdx = pCellIdx.offset(-(2 as ::core::ffi::c_int as isize));
                        if pc < contentOffset
                            || pc
                                > usableSize.wrapping_sub(4 as crate::src::ext::rtree::rtree::U32_0)
                        {
                            checkAppendMsg_args(
                                pCheck,
                                b"Offset %u out of range %u..%u\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[
                                    crate::src::src::printf::PrintfArg::from(pc),
                                    crate::src::src::printf::PrintfArg::from(contentOffset),
                                    crate::src::src::printf::PrintfArg::from(
                                        usableSize.wrapping_sub(
                                            4 as crate::src::ext::rtree::rtree::U32_0,
                                        ),
                                    ),
                                ],
                            );
                            doCoverageCheck = 0 as ::core::ffi::c_int;
                        } else {
                            pCell = data.offset(pc as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0;
                            __pPage_ref.xParseCell.expect("non-null function pointer")(
                                pPage,
                                pCell,
                                &raw mut info,
                            );
                            if pc.wrapping_add(info.nSize as crate::src::ext::rtree::rtree::U32_0)
                                > usableSize
                            {
                                checkAppendMsg_args(
                                    pCheck,
                                    b"Extends off end of page\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[],
                                );
                                doCoverageCheck = 0 as ::core::ffi::c_int;
                            } else {
                                if __pPage_ref.intKey != 0 {
                                    if if keyCanBeEqual != 0 {
                                        (info.nKey > maxKey) as ::core::ffi::c_int
                                    } else {
                                        (info.nKey >= maxKey) as ::core::ffi::c_int
                                    } != 0
                                    {
                                        checkAppendMsg_args(
                                            pCheck,
                                            b"Rowid %lld out of order\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            &[crate::src::src::printf::PrintfArg::Int(info.nKey)],
                                        );
                                    }
                                    maxKey = info.nKey;
                                    keyCanBeEqual = 0 as ::core::ffi::c_int;
                                }
                                if info.nPayload
                                    > info.nLocal as crate::src::ext::rtree::rtree::U32_0
                                {
                                    
                                    
                                    let nPage: crate::src::ext::rtree::rtree::U32_0 = info
                                        .nPayload
                                        .wrapping_sub(
                                            info.nLocal as crate::src::ext::rtree::rtree::U32_0,
                                        )
                                        .wrapping_add(usableSize)
                                        .wrapping_sub(5 as crate::src::ext::rtree::rtree::U32_0)
                                        .wrapping_div(usableSize.wrapping_sub(
                                            4 as crate::src::ext::rtree::rtree::U32_0,
                                        ));
                                    let pgnoOvfl: crate::src::src::pager::Pgno = crate::src::src::util::sqlite3Get4byte(pCell.offset(
                                        (info.nSize as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                                            as isize,
                                    )
                                        as *mut crate::src::ext::rtree::rtree::U8_0)
                                        as crate::src::src::pager::Pgno;
                                    if (*pBt).autoVacuum != 0 {
                                        checkPtrmap(
                                            pCheck,
                                            pgnoOvfl,
                                            crate::src::headers::btreeInt_h::PTRMAP_OVERFLOW1
                                                as crate::src::ext::rtree::rtree::U8_0,
                                            iPage,
                                        );
                                    }
                                    checkList(pCheck, 0 as ::core::ffi::c_int, pgnoOvfl, nPage);
                                }
                                if __pPage_ref.leaf == 0 {
                                    pgno = crate::src::src::util::sqlite3Get4byte(pCell)
                                        as ::core::ffi::c_int;
                                    if (*pBt).autoVacuum != 0 {
                                        checkPtrmap(
                                            pCheck,
                                            pgno as crate::src::src::pager::Pgno,
                                            crate::src::headers::btreeInt_h::PTRMAP_BTREE
                                                as crate::src::ext::rtree::rtree::U8_0,
                                            iPage,
                                        );
                                    }
                                    d2 = checkTreePage(
                                        pCheck,
                                        pgno as crate::src::src::pager::Pgno,
                                        &raw mut maxKey,
                                        maxKey,
                                    );
                                    keyCanBeEqual = 0 as ::core::ffi::c_int;
                                    if d2 != depth {
                                        checkAppendMsg_args(
                                            pCheck,
                                            b"Child page depth differs\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            &[],
                                        );
                                        depth = d2;
                                    }
                                } else {
                                    btreeHeapInsert(
                                        heap,
                                        pc << 16 as ::core::ffi::c_int
                                            | pc.wrapping_add(
                                                info.nSize as crate::src::ext::rtree::rtree::U32_0,
                                            )
                                            .wrapping_sub(
                                                1 as crate::src::ext::rtree::rtree::U32_0,
                                            ),
                                    );
                                }
                            }
                        }
                        i -= 1;
                    }
                    *piMinKey = maxKey;
                    __pCheck_ref.zPfx = ::core::ptr::null::<::core::ffi::c_char>();
                    if doCoverageCheck != 0 && __pCheck_ref.mxErr > 0 as ::core::ffi::c_int {
                        if __pPage_ref.leaf == 0 {
                            heap = __pCheck_ref.heap;
                            *heap.offset(0_isize) = 0 as crate::src::ext::rtree::rtree::U32_0;
                            i = nCell - 1 as ::core::ffi::c_int;
                            while i >= 0 as ::core::ffi::c_int {
                                
                                pc = ((*(data
                                    .offset((cellStart + i * 2 as ::core::ffi::c_int) as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0)
                                    .offset(0_isize)
                                    as ::core::ffi::c_int)
                                    << 8 as ::core::ffi::c_int
                                    | *(data
                                        .offset((cellStart + i * 2 as ::core::ffi::c_int) as isize)
                                        as *mut crate::src::ext::rtree::rtree::U8_0)
                                        .offset(1_isize)
                                        as ::core::ffi::c_int)
                                    as crate::src::ext::rtree::rtree::U32_0;
                                let size: crate::src::ext::rtree::rtree::U32_0 = __pPage_ref.xCellSize.expect("non-null function pointer")(
                                    pPage,
                                    data.offset(pc as isize)
                                        as *mut crate::src::ext::rtree::rtree::U8_0,
                                )
                                    as crate::src::ext::rtree::rtree::U32_0;
                                btreeHeapInsert(
                                    heap,
                                    pc << 16 as ::core::ffi::c_int
                                        | pc.wrapping_add(size).wrapping_sub(
                                            1 as crate::src::ext::rtree::rtree::U32_0,
                                        ),
                                );
                                i -= 1;
                            }
                        }
                        i = (*(data.offset((hdr + 1 as ::core::ffi::c_int) as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0)
                            .offset(0_isize) as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *(data.offset((hdr + 1 as ::core::ffi::c_int) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(1_isize)
                                as ::core::ffi::c_int;
                        while i > 0 as ::core::ffi::c_int {
                            
                            
                            let size_0: ::core::ffi::c_int = (*(data.offset((i + 2 as ::core::ffi::c_int) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(0_isize)
                                as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int
                                | *(data.offset((i + 2 as ::core::ffi::c_int) as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0)
                                    .offset(1_isize)
                                    as ::core::ffi::c_int;
                            btreeHeapInsert(
                                heap,
                                (i as crate::src::ext::rtree::rtree::U32_0)
                                    << 16 as ::core::ffi::c_int
                                    | (i + size_0 - 1 as ::core::ffi::c_int)
                                        as crate::src::ext::rtree::rtree::U32_0,
                            );
                            let j: ::core::ffi::c_int = (*(data.offset(i as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(0_isize)
                                as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int
                                | *(data.offset(i as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0)
                                    .offset(1_isize)
                                    as ::core::ffi::c_int;
                            i = j;
                        }
                        nFrag = 0 as ::core::ffi::c_int;
                        prev =
                            contentOffset.wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0);
                        while btreeHeapPull(heap, &raw mut x) != 0 {
                            if prev & 0xffff as crate::src::ext::rtree::rtree::U32_0
                                >= x >> 16 as ::core::ffi::c_int
                            {
                                checkAppendMsg_args(
                                    pCheck,
                                    b"Multiple uses for byte %u of page %u\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[
                                        crate::src::src::printf::PrintfArg::from(
                                            x >> 16 as ::core::ffi::c_int,
                                        ),
                                        crate::src::src::printf::PrintfArg::from(iPage),
                                    ],
                                );
                                break;
                            } else {
                                nFrag = (nFrag as crate::src::ext::rtree::rtree::U32_0)
                                    .wrapping_add(
                                        (x >> 16 as ::core::ffi::c_int)
                                            .wrapping_sub(
                                                prev & 0xffff
                                                    as crate::src::ext::rtree::rtree::U32_0,
                                            )
                                            .wrapping_sub(
                                                1 as crate::src::ext::rtree::rtree::U32_0,
                                            ),
                                    ) as ::core::ffi::c_int
                                    as ::core::ffi::c_int;
                                prev = x;
                            }
                        }
                        nFrag = (nFrag as crate::src::ext::rtree::rtree::U32_0).wrapping_add(
                            usableSize
                                .wrapping_sub(prev & 0xffff as crate::src::ext::rtree::rtree::U32_0)
                                .wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0),
                        ) as ::core::ffi::c_int
                            as ::core::ffi::c_int;
                        if *heap.offset(0_isize) == 0 as crate::src::ext::rtree::rtree::U32_0
                            && nFrag
                                != *data.offset((hdr + 7 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                        {
                            checkAppendMsg_args(
                                pCheck,
                                b"Fragmentation of %u bytes reported as %u on page %u\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                &[
                                    crate::src::src::printf::PrintfArg::from(
                                        nFrag as ::core::ffi::c_uint,
                                    ),
                                    crate::src::src::printf::PrintfArg::from(
                                        *data.offset((hdr + 7 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_uint,
                                    ),
                                    crate::src::src::printf::PrintfArg::from(iPage),
                                ],
                            );
                        }
                    }
                }
            }
        }
    }
    if doCoverageCheck == 0 {
        (*pPage).isInit = savedIsInit;
    }
    releasePage(pPage);
    __pCheck_ref.zPfx = saved_zPfx;
    __pCheck_ref.v1 = saved_v1 as crate::src::src::pager::Pgno;
    __pCheck_ref.v2 = saved_v2;
    depth + 1 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeIntegrityCheck(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut crate::src::headers::btreeInt_h::Btree,
    aRoot: *mut crate::src::src::pager::Pgno,
    aCnt: *mut crate::src::src::vdbe::Mem,
    nRoot: ::core::ffi::c_int,
    mxErr: ::core::ffi::c_int,
    pnErr: *mut ::core::ffi::c_int,
    pzOut: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: crate::src::src::pager::Pgno;
    let mut sCheck: crate::src::headers::btreeInt_h::IntegrityCk = unsafe { ::core::mem::zeroed() };
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    let savedDbFlags: crate::src::ext::rtree::rtree::U64_0 = (*(*pBt).db).flags;
    let mut zErr: [::core::ffi::c_char; 100] = [0; 100];
    let mut bPartial: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bCkFreelist: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if *aRoot.offset(0_isize) == 0 as crate::src::src::pager::Pgno {
        bPartial = 1 as ::core::ffi::c_int;
        if *aRoot.offset(1_isize) != 1 as crate::src::src::pager::Pgno {
            bCkFreelist = 0 as ::core::ffi::c_int;
        }
    }
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    sCheck.db = db;
    sCheck.pBt = pBt;
    sCheck.pPager = (*pBt).pPager;
    sCheck.nCkPage = btreePagecount(sCheck.pBt);
    sCheck.mxErr = mxErr;
    crate::src::src::printf::sqlite3StrAccumInit(
        &raw mut sCheck.errMsg as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>()
            as *mut crate::src::headers::sqliteInt_h::sqlite3,
        &raw mut zErr as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
        crate::sqliteLimit_h::SQLITE_MAX_LENGTH,
    );
    sCheck.errMsg.printfFlags = crate::src::headers::sqliteInt_h::SQLITE_PRINTF_INTERNAL
        as crate::src::ext::rtree::rtree::U8_0;
    if (sCheck.nCkPage != 0 as crate::src::src::pager::Pgno) {
        sCheck.aPgRef = crate::src::src::malloc::sqlite3MallocZero(
            sCheck
                .nCkPage
                .wrapping_div(8 as crate::src::src::pager::Pgno)
                .wrapping_add(1 as crate::src::src::pager::Pgno)
                as crate::src::ext::rtree::rtree::U64_0,
        ) as *mut crate::src::ext::rtree::rtree::U8_0;
        if sCheck.aPgRef.is_null() {
            checkOom(&raw mut sCheck);
        } else {
            sCheck.heap =
                crate::src::src::pcache1::sqlite3PageMalloc((*pBt).pageSize as ::core::ffi::c_int)
                    as *mut crate::src::ext::rtree::rtree::U32_0;
            if sCheck.heap.is_null() {
                checkOom(&raw mut sCheck);
            } else {
                let __pBt_ref = unsafe { &mut *pBt };
                i = (crate::src::src::global::sqlite3PendingByte
                    as crate::src::ext::rtree::rtree::U32_0)
                    .wrapping_div(__pBt_ref.pageSize)
                    .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0);
                if i <= sCheck.nCkPage {
                    setPageReferenced(&raw mut sCheck, i);
                }
                if bCkFreelist != 0 {
                    sCheck.zPfx = b"Freelist: \0" as *const u8 as *const ::core::ffi::c_char;
                    checkList(
                        &raw mut sCheck,
                        1 as ::core::ffi::c_int,
                        crate::src::src::util::sqlite3Get4byte(
                            (*__pBt_ref.pPage1).aData.offset(32_isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0,
                        ) as crate::src::src::pager::Pgno,
                        crate::src::src::util::sqlite3Get4byte(
                            (*__pBt_ref.pPage1).aData.offset(36_isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0,
                        ),
                    );
                    sCheck.zPfx = ::core::ptr::null::<::core::ffi::c_char>();
                }
                if bPartial == 0 {
                    if __pBt_ref.autoVacuum != 0 {
                        let mut mx: crate::src::src::pager::Pgno =
                            0 as crate::src::src::pager::Pgno;
                        
                        i = 0 as crate::src::src::pager::Pgno;
                        while (i as ::core::ffi::c_int) < nRoot {
                            if mx < *aRoot.offset(i as isize) {
                                mx = *aRoot.offset(i as isize);
                            }
                            i = i.wrapping_add(1);
                        }
                        let mxInHdr: crate::src::src::pager::Pgno = crate::src::src::util::sqlite3Get4byte(
                            (*__pBt_ref.pPage1).aData.offset(52_isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0,
                        ) as crate::src::src::pager::Pgno;
                        if mx != mxInHdr {
                            checkAppendMsg_args(
                                &raw mut sCheck,
                                b"max rootpage (%u) disagrees with header (%u)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[
                                    crate::src::src::printf::PrintfArg::from(mx),
                                    crate::src::src::printf::PrintfArg::from(mxInHdr),
                                ],
                            );
                        }
                    } else if crate::src::src::util::sqlite3Get4byte(
                        (*__pBt_ref.pPage1).aData.offset(64_isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0,
                    ) != 0 as crate::src::ext::rtree::rtree::U32_0
                    {
                        checkAppendMsg_args(
                            &raw mut sCheck,
                            b"incremental_vacuum enabled with a max rootpage of zero\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    }
                }
                (*__pBt_ref.db).flags &= !(crate::src::headers::sqliteInt_h::SQLITE_CellSizeCk
                    as crate::src::ext::rtree::rtree::U64_0);
                i = 0 as crate::src::src::pager::Pgno;
                while (i as ::core::ffi::c_int) < nRoot && sCheck.mxErr != 0 {
                    sCheck.nRow = 0 as crate::src::ext::rtree::rtree::I64_0;
                    if *aRoot.offset(i as isize) != 0 {
                        let mut notUsed: crate::src::ext::rtree::rtree::I64_0 = 0;
                        if __pBt_ref.autoVacuum as ::core::ffi::c_int != 0
                            && *aRoot.offset(i as isize) > 1 as crate::src::src::pager::Pgno
                            && bPartial == 0
                        {
                            checkPtrmap(
                                &raw mut sCheck,
                                *aRoot.offset(i as isize),
                                crate::src::headers::btreeInt_h::PTRMAP_ROOTPAGE
                                    as crate::src::ext::rtree::rtree::U8_0,
                                0 as crate::src::src::pager::Pgno,
                            );
                        }
                        sCheck.v0 = *aRoot.offset(i as isize);
                        checkTreePage(
                            &raw mut sCheck,
                            *aRoot.offset(i as isize),
                            &raw mut notUsed,
                            crate::fts3Int_h::LARGEST_INT64,
                        );
                    }
                    crate::src::src::vdbemem::sqlite3MemSetArrayInt64(
                        aCnt as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        i as ::core::ffi::c_int,
                        sCheck.nRow,
                    );
                    i = i.wrapping_add(1);
                }
                (*__pBt_ref.db).flags = savedDbFlags;
                if bPartial == 0 {
                    i = 1 as crate::src::src::pager::Pgno;
                    while i <= sCheck.nCkPage && sCheck.mxErr != 0 {
                        if getPageReferenced(&raw mut sCheck, i) == 0 as ::core::ffi::c_int
                            && (ptrmapPageno(pBt, i) != i || __pBt_ref.autoVacuum == 0)
                        {
                            checkAppendMsg_args(
                                &raw mut sCheck,
                                b"Page %u: never used\0" as *const u8 as *const ::core::ffi::c_char,
                                &[crate::src::src::printf::PrintfArg::from(i)],
                            );
                        }
                        if getPageReferenced(&raw mut sCheck, i) != 0 as ::core::ffi::c_int
                            && (ptrmapPageno(pBt, i) == i
                                && __pBt_ref.autoVacuum as ::core::ffi::c_int != 0)
                        {
                            checkAppendMsg_args(
                                &raw mut sCheck,
                                b"Page %u: pointer map referenced\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[crate::src::src::printf::PrintfArg::from(i)],
                            );
                        }
                        i = i.wrapping_add(1);
                    }
                }
            }
        }
    }
    crate::src::src::pcache1::sqlite3PageFree(sCheck.heap as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(sCheck.aPgRef as *mut ::core::ffi::c_void);
    *pnErr = sCheck.nErr;
    if sCheck.nErr == 0 as ::core::ffi::c_int {
        crate::src::src::printf::sqlite3_str_reset(
            &raw mut sCheck.errMsg as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        );
        *pzOut = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        *pzOut = crate::src::src::printf::sqlite3StrAccumFinish(
            &raw mut sCheck.errMsg as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        );
    }
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    sCheck.rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeGetFilename(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> *const ::core::ffi::c_char {
    crate::src::src::pager::sqlite3PagerFilename((*(*p).pBt).pPager, 1 as ::core::ffi::c_int)
}
pub unsafe extern "C" fn sqlite3BtreeGetJournalname(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> *const ::core::ffi::c_char {
    crate::src::src::pager::sqlite3PagerJournalname((*(*p).pBt).pPager)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeTxnState(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    if !p.is_null() {
        (*p).inTrans as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCheckpoint(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    eMode: ::core::ffi::c_int,
    pnLog: *mut ::core::ffi::c_int,
    pnCkpt: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !p.is_null() {
        let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
        crate::src::src::btmutex::sqlite3BtreeEnter(
            p as *mut crate::src::headers::btreeInt_h::Btree,
        );
        if (*pBt).inTransaction as ::core::ffi::c_int != crate::src::headers::btreeInt_h::TRANS_NONE
        {
            rc = crate::src::headers::sqlite3_h::SQLITE_LOCKED;
        } else {
            rc = crate::src::src::pager::sqlite3PagerCheckpoint(
                (*pBt).pPager,
                (*p).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                eMode,
                pnLog,
                pnCkpt,
            );
        }
        crate::src::src::btmutex::sqlite3BtreeLeave(
            p as *mut crate::src::headers::btreeInt_h::Btree,
        );
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeIsInBackup(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    ((*p).nBackup != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}
pub unsafe extern "C" fn sqlite3BtreeSchema(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    nBytes: ::core::ffi::c_int,
    xFree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> *mut ::core::ffi::c_void {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    if (*pBt).pSchema.is_null() && nBytes != 0 {
        (*pBt).pSchema = crate::src::src::malloc::sqlite3DbMallocZero(
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>()
                as *mut crate::src::headers::sqliteInt_h::sqlite3,
            nBytes as crate::src::ext::rtree::rtree::U64_0,
        );
        (*pBt).xFreeSchema = xFree;
    }
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    (*pBt).pSchema
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSchemaLocked(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    
    crate::src::src::btmutex::sqlite3BtreeEnter(p as *mut crate::src::headers::btreeInt_h::Btree);
    let rc: ::core::ffi::c_int = querySharedCacheTableLock(
        p,
        crate::src::headers::sqliteInt_h::SCHEMA_ROOT as crate::src::src::pager::Pgno,
        crate::src::headers::btreeInt_h::READ_LOCK as crate::src::ext::rtree::rtree::U8_0,
    );
    crate::src::src::btmutex::sqlite3BtreeLeave(p as *mut crate::src::headers::btreeInt_h::Btree);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeLockTable(
    p: *mut crate::src::headers::btreeInt_h::Btree,
    iTab: ::core::ffi::c_int,
    isWriteLock: crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*p).sharable != 0 {
        let lockType: crate::src::ext::rtree::rtree::U8_0 =
            (crate::src::headers::btreeInt_h::READ_LOCK + isWriteLock as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U8_0;
        crate::src::src::btmutex::sqlite3BtreeEnter(
            p as *mut crate::src::headers::btreeInt_h::Btree,
        );
        rc = querySharedCacheTableLock(p, iTab as crate::src::src::pager::Pgno, lockType);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = setSharedCacheTableLock(p, iTab as crate::src::src::pager::Pgno, lockType);
        }
        crate::src::src::btmutex::sqlite3BtreeLeave(
            p as *mut crate::src::headers::btreeInt_h::Btree,
        );
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreePutData(
    pCsr: *mut crate::src::headers::btreeInt_h::BtCursor,
    offset: crate::src::ext::rtree::rtree::U32_0,
    amt: crate::src::ext::rtree::rtree::U32_0,
    z: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    
    let __pCsr_ref = unsafe { &*pCsr };
    let rc: ::core::ffi::c_int = if __pCsr_ref.eState as ::core::ffi::c_int
        >= crate::src::headers::btreeInt_h::CURSOR_REQUIRESEEK
    {
        btreeRestoreCursorPosition(pCsr)
    } else {
        crate::src::headers::sqlite3_h::SQLITE_OK
    };
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    if __pCsr_ref.eState as ::core::ffi::c_int != crate::src::headers::btreeInt_h::CURSOR_VALID {
        return crate::src::headers::sqlite3_h::SQLITE_ABORT;
    }
    saveAllCursors(__pCsr_ref.pBt, __pCsr_ref.pgnoRoot, pCsr);
    if __pCsr_ref.curFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTCF_WriteFlag
        == 0 as ::core::ffi::c_int
    {
        return crate::src::headers::sqlite3_h::SQLITE_READONLY;
    }
    accessPayload(
        pCsr,
        offset,
        amt,
        z as *mut ::core::ffi::c_uchar,
        1 as ::core::ffi::c_int,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeIncrblobCursor(
    pCur: *mut crate::src::headers::btreeInt_h::BtCursor,
) {
    let __pCur_ref = unsafe { &mut *pCur };
    __pCur_ref.curFlags = (__pCur_ref.curFlags as ::core::ffi::c_int
        | crate::src::headers::btreeInt_h::BTCF_Incrblob)
        as crate::src::ext::rtree::rtree::U8_0;
    (*__pCur_ref.pBtree).hasIncrblobCur = 1 as crate::src::ext::rtree::rtree::U8_0;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSetVersion(
    pBtree: *mut crate::src::headers::btreeInt_h::Btree,
    iVersion: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*pBtree).pBt;
    let mut rc: ::core::ffi::c_int;
    let __pBt_ref = unsafe { &mut *pBt };
    __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
        & !crate::src::headers::btreeInt_h::BTS_NO_WAL)
        as crate::src::fts5::U16_0;
    if iVersion == 1 as ::core::ffi::c_int {
        __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
            | crate::src::headers::btreeInt_h::BTS_NO_WAL)
            as crate::src::fts5::U16_0;
    }
    rc = sqlite3BtreeBeginTrans(
        pBtree,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let aData: *mut crate::src::ext::rtree::rtree::U8_0 = (*__pBt_ref.pPage1).aData;
        if *aData.offset(18_isize) as ::core::ffi::c_int
            != iVersion as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int
            || *aData.offset(19_isize) as ::core::ffi::c_int
                != iVersion as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int
        {
            rc = sqlite3BtreeBeginTrans(
                pBtree,
                2 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::src::src::pager::sqlite3PagerWrite(
                    (*__pBt_ref.pPage1).pDbPage as *mut crate::src::src::pcache::PgHdr,
                );
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    *aData.offset(18_isize) = iVersion as crate::src::ext::rtree::rtree::U8_0;
                    *aData.offset(19_isize) = iVersion as crate::src::ext::rtree::rtree::U8_0;
                }
            }
        }
    }
    __pBt_ref.btsFlags = (__pBt_ref.btsFlags as ::core::ffi::c_int
        & !crate::src::headers::btreeInt_h::BTS_NO_WAL)
        as crate::src::fts5::U16_0;
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeCursorHasHint(
    pCsr: *mut crate::src::headers::btreeInt_h::BtCursor,
    mask: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    ((*pCsr).hints as ::core::ffi::c_uint & mask != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeIsReadonly(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    ((*(*p).pBt).btsFlags as ::core::ffi::c_int & crate::src::headers::btreeInt_h::BTS_READ_ONLY
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}
pub unsafe extern "C" fn sqlite3HeaderSizeBtree() -> ::core::ffi::c_int {
    ((::core::mem::size_of::<crate::src::headers::btreeInt_h::MemPage>() as usize)
        .wrapping_add(7_usize)
        & !(7 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeClearCache(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) {
    let pBt: *mut crate::src::headers::btreeInt_h::BtShared = (*p).pBt;
    if (*pBt).inTransaction as ::core::ffi::c_int == crate::src::headers::btreeInt_h::TRANS_NONE {
        crate::src::src::pager::sqlite3PagerClearCache((*pBt).pPager);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BtreeSharable(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    (*p).sharable as ::core::ffi::c_int
}
pub unsafe extern "C" fn sqlite3BtreeConnectionCount(
    p: *mut crate::src::headers::btreeInt_h::Btree,
) -> ::core::ffi::c_int {
    (*(*p).pBt).nRef
}
