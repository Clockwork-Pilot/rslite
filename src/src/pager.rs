

















// =============== BEGIN pager_h ================
pub const SQLITE_DEFAULT_JOURNAL_SIZE_LIMIT:  ::core::ffi::c_int =
     -(1 as ::core::ffi::c_int);
    
    pub type Pgno = crate::src::ext::rtree::rtree::u32_0;
    
    pub type DbPage = crate::src::src::pcache::PgHdr;
    
    pub const PAGER_OMIT_JOURNAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    pub const PAGER_MEMORY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    pub const PAGER_LOCKINGMODE_QUERY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    
    pub const PAGER_LOCKINGMODE_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const PAGER_LOCKINGMODE_EXCLUSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const PAGER_JOURNALMODE_QUERY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    
    pub const PAGER_JOURNALMODE_DELETE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const PAGER_JOURNALMODE_PERSIST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const PAGER_JOURNALMODE_OFF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    pub const PAGER_JOURNALMODE_TRUNCATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    pub const PAGER_JOURNALMODE_MEMORY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    pub const PAGER_JOURNALMODE_WAL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    pub const PAGER_GET_NOCONTENT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    pub const PAGER_GET_READONLY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    pub const PAGER_SYNCHRONOUS_OFF: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    pub const PAGER_SYNCHRONOUS_FULL: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
    
    pub const PAGER_SYNCHRONOUS_EXTRA: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    pub const PAGER_SYNCHRONOUS_MASK: ::core::ffi::c_int = 0x7 as ::core::ffi::c_int;
    
    pub const PAGER_FULLFSYNC: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    pub const PAGER_CKPT_FULLFSYNC: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    pub const PAGER_CACHESPILL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    pub const PAGER_FLAGS_MASK: ::core::ffi::c_int = 0x38 as ::core::ffi::c_int;
pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;








pub use crate::src::src::hash::Hash;pub use crate::src::src::hash::HashElem;pub use crate::src::src::hash::_ht;pub use crate::internal::SQLITE_DEFAULT_PAGE_SIZE;pub use crate::internal::__SIZEOF_POINTER__;pub use crate::src::src::os::sqlite3OsAccess;pub use crate::src::src::os::sqlite3OsCheckReservedLock;pub use crate::src::src::os::sqlite3OsClose;pub use crate::src::src::os::sqlite3OsDelete;pub use crate::src::src::os::sqlite3OsDeviceCharacteristics;pub use crate::src::src::os::sqlite3OsFetch;pub use crate::src::src::os::sqlite3OsFileControl;pub use crate::src::src::os::sqlite3OsFileControlHint;pub use crate::src::src::os::sqlite3OsFileSize;pub use crate::src::src::os::sqlite3OsFullPathname;pub use crate::src::src::os::sqlite3OsLock;pub use crate::src::src::os::sqlite3OsOpen;pub use crate::src::src::os::sqlite3OsRead;pub use crate::src::src::os::sqlite3OsSectorSize;pub use crate::src::src::os::sqlite3OsSync;pub use crate::src::src::os::sqlite3OsTruncate;pub use crate::src::src::os::sqlite3OsUnfetch;pub use crate::src::src::os::sqlite3OsUnlock;pub use crate::src::src::os::sqlite3OsWrite;pub use crate::src::src::os::EXCLUSIVE_LOCK_1;pub use crate::src::src::os::NO_LOCK;pub use crate::src::src::os::RESERVED_LOCK_1;pub use crate::src::src::os::SHARED_LOCK;pub use crate::src::src::pcache::sqlite3PCacheIsDirty;pub use crate::src::src::pcache::sqlite3PCachePercentDirty;pub use crate::src::src::pcache::sqlite3PcacheCleanAll;pub use crate::src::src::pcache::sqlite3PcacheClear;pub use crate::src::src::pcache::sqlite3PcacheClearSyncFlags;pub use crate::src::src::pcache::sqlite3PcacheClearWritable;pub use crate::src::src::pcache::sqlite3PcacheClose;pub use crate::src::src::pcache::sqlite3PcacheDirtyList;pub use crate::src::src::pcache::sqlite3PcacheDrop;pub use crate::src::src::pcache::sqlite3PcacheFetch;pub use crate::src::src::pcache::sqlite3PcacheFetchFinish;pub use crate::src::src::pcache::sqlite3PcacheFetchStress;pub use crate::src::src::pcache::sqlite3PcacheGetCachesize;pub use crate::src::src::pcache::sqlite3PcacheMakeClean;pub use crate::src::src::pcache::sqlite3PcacheMakeDirty;pub use crate::src::src::pcache::sqlite3PcacheMove;pub use crate::src::src::pcache::sqlite3PcacheOpen;pub use crate::src::src::pcache::sqlite3PcachePageRefcount;pub use crate::src::src::pcache::sqlite3PcachePagecount;pub use crate::src::src::pcache::sqlite3PcacheRef;pub use crate::src::src::pcache::sqlite3PcacheRefCount;pub use crate::src::src::pcache::sqlite3PcacheRelease;pub use crate::src::src::pcache::sqlite3PcacheSetCachesize;pub use crate::src::src::pcache::sqlite3PcacheSetPageSize;pub use crate::src::src::pcache::sqlite3PcacheSetSpillsize;pub use crate::src::src::pcache::sqlite3PcacheShrink;pub use crate::src::src::pcache::sqlite3PcacheSize;pub use crate::src::src::pcache::sqlite3PcacheTruncate;pub use crate::pcache_h::PCache;pub use crate::src::src::pcache::PgHdr;pub use crate::src::src::pcache::PGHDR_DIRTY;pub use crate::src::src::pcache::PGHDR_DONT_WRITE;pub use crate::src::src::pcache::PGHDR_MMAP;pub use crate::src::src::pcache::PGHDR_NEED_SYNC;pub use crate::src::src::pcache::PGHDR_WRITEABLE;pub use crate::src::src::backup::sqlite3_backup;pub use crate::src::headers::vdbeInt_h::sqlite3_context;pub use crate::src::src::legacy::sqlite3_exec;pub use crate::src::headers::sqlite3_h::sqlite3_file;pub use crate::src::headers::sqlite3_h::sqlite3_filename;pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::src::headers::sqlite3_h::sqlite3_index_info;pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;pub use crate::src::src::printf::sqlite3_log;pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;pub use crate::src::headers::sqlite3_h::sqlite3_module;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;pub use crate::src::headers::sqlite3_h::sqlite3_pcache;pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;pub use crate::src::src::random::sqlite3_randomness;pub use crate::src::headers::sqlite3_h::sqlite3_syscall_ptr;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::src::main::sqlite3_uri_boolean;pub use crate::src::headers::vdbeInt_h::sqlite3_value;pub use crate::src::headers::sqlite3_h::sqlite3_vfs;pub use crate::src::headers::sqlite3_h::sqlite3_vtab;pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_ABORT;pub use crate::src::headers::sqlite3_h::SQLITE_ACCESS_EXISTS;pub use crate::src::headers::sqlite3_h::SQLITE_BUSY;pub use crate::src::headers::sqlite3_h::SQLITE_CANTOPEN;pub use crate::src::headers::sqlite3_h::SQLITE_CANTOPEN_SYMLINK_1;pub use crate::src::headers::sqlite3_h::SQLITE_CHECKPOINT_PASSIVE;pub use crate::src::headers::sqlite3_h::SQLITE_DBSTATUS_CACHE_HIT;pub use crate::src::headers::sqlite3_h::SQLITE_DONE;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_BUSYHANDLER;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_COMMIT_PHASETWO;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_HAS_MOVED_1;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_MMAP_SIZE_1;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_SIZE_HINT_1;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_SYNC;pub use crate::src::headers::sqlite3_h::SQLITE_FULL;pub use crate::src::headers::sqlite3_h::SQLITE_IOCAP_IMMUTABLE;pub use crate::src::headers::sqlite3_h::SQLITE_IOCAP_POWERSAFE_OVERWRITE;pub use crate::src::headers::sqlite3_h::SQLITE_IOCAP_SAFE_APPEND;pub use crate::src::headers::sqlite3_h::SQLITE_IOCAP_SEQUENTIAL;pub use crate::src::headers::sqlite3_h::SQLITE_IOCAP_SUBPAGE_READ;pub use crate::src::headers::sqlite3_h::SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_NOTFOUND;pub use crate::src::headers::sqlite3_h::SQLITE_NOTICE;pub use crate::src::headers::sqlite3_h::SQLITE_NOTICE_RECOVER_ROLLBACK_1;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::headers::sqlite3_h::SQLITE_OK_SYMLINK;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_DELETEONCLOSE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_EXCLUSIVE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_JOURNAL;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_MEMORY;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_NOFOLLOW;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_SUBJOURNAL;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_SUPER_JOURNAL;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_TEMP_JOURNAL;pub use crate::src::headers::sqlite3_h::SQLITE_READONLY;pub use crate::src::headers::sqlite3_h::SQLITE_READONLY_DBMOVED_1;pub use crate::src::headers::sqlite3_h::SQLITE_READONLY_ROLLBACK_1;pub use crate::src::headers::sqlite3_h::SQLITE_SYNC_DATAONLY;pub use crate::src::headers::sqlite3_h::SQLITE_SYNC_FULL;pub use crate::src::headers::sqlite3_h::SQLITE_SYNC_NORMAL;pub use crate::src::headers::sqliteInt_h::__anon_struct_0;pub use crate::src::headers::sqliteInt_h::__anon_struct_1;pub use crate::src::headers::sqliteInt_h::__anon_struct_2;pub use crate::src::headers::sqliteInt_h::__anon_struct_3;pub use crate::src::headers::sqliteInt_h::__anon_struct_4;pub use crate::src::headers::sqliteInt_h::__anon_struct_5;pub use crate::src::headers::sqliteInt_h::__anon_struct_6;pub use crate::src::headers::sqliteInt_h::__anon_struct_7;pub use crate::src::headers::sqliteInt_h::__anon_struct_8;pub use crate::src::headers::sqliteInt_h::__anon_union_0;pub use crate::src::headers::sqliteInt_h::__anon_union_1;pub use crate::src::headers::sqliteInt_h::__anon_union_10;pub use crate::src::headers::sqliteInt_h::__anon_union_11;pub use crate::src::headers::sqliteInt_h::__anon_union_12;pub use crate::src::headers::sqliteInt_h::__anon_union_13;pub use crate::src::headers::sqliteInt_h::__anon_union_15;pub use crate::src::headers::sqliteInt_h::__anon_union_2;pub use crate::src::headers::sqliteInt_h::__anon_union_3;pub use crate::src::headers::sqliteInt_h::__anon_union_5;pub use crate::src::headers::sqliteInt_h::__anon_union_6;pub use crate::src::headers::sqliteInt_h::__anon_union_7;pub use crate::src::headers::sqliteInt_h::__anon_union_8;pub use crate::src::headers::sqliteInt_h::__anon_union_9;pub use crate::src::headers::sqliteInt_h::bft;pub use crate::src::fts5::i16_0;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::headers::sqliteInt_h::sColMap;pub use crate::src::headers::sqliteInt_h::sqlite3;pub use crate::src::src::backup::sqlite3BackupRestart;pub use crate::src::src::backup::sqlite3BackupUpdate;pub use crate::src::src::fault::sqlite3BeginBenignMalloc;pub use crate::src::src::bitvec::sqlite3BitvecClear;pub use crate::src::src::bitvec::sqlite3BitvecCreate;pub use crate::src::src::bitvec::sqlite3BitvecDestroy;pub use crate::src::src::bitvec::sqlite3BitvecSet;pub use crate::src::src::bitvec::sqlite3BitvecTest;pub use crate::src::src::bitvec::sqlite3BitvecTestNotNull;pub use crate::src::src::main::sqlite3CantopenError;pub use crate::src::src::global::sqlite3Config;pub use crate::src::src::main::sqlite3CorruptError;pub use crate::src::src::malloc::sqlite3DbFree;pub use crate::src::src::malloc::sqlite3DbMallocRaw;pub use crate::src::src::malloc::sqlite3DbStrDup;pub use crate::src::src::fault::sqlite3EndBenignMalloc;pub use crate::src::src::util::sqlite3FaultSim;pub use crate::src::src::util::sqlite3Get4byte;pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;pub use crate::src::src::memdb::sqlite3IsMemdb;pub use crate::src::src::memjournal::sqlite3JournalIsInMemory;pub use crate::src::src::memjournal::sqlite3JournalOpen;pub use crate::src::src::memjournal::sqlite3JournalSize;pub use crate::src::src::malloc::sqlite3Malloc;pub use crate::src::src::malloc::sqlite3MallocSize;pub use crate::src::src::malloc::sqlite3MallocZero;pub use crate::src::src::memjournal::sqlite3MemJournalOpen;pub use crate::src::src::pcache1::sqlite3PageFree;pub use crate::src::src::pcache1::sqlite3PageMalloc;pub use crate::src::src::global::sqlite3PendingByte;pub use crate::src::src::util::sqlite3Put4byte;pub use crate::src::src::malloc::sqlite3Realloc;pub use crate::src::src::util::sqlite3Strlen30;pub use crate::src::headers::sqliteInt_h::sqlite3_xauth;pub use crate::src::fts5::u16_0;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::src::headers::sqliteInt_h::yDbMask;pub use crate::src::headers::sqliteInt_h::ynVar;pub use crate::src::headers::sqliteInt_h::AggInfo;pub use crate::src::headers::sqliteInt_h::AggInfo_col;pub use crate::src::headers::sqliteInt_h::AggInfo_func;pub use crate::src::headers::sqliteInt_h::AutoincInfo;pub use crate::src::headers::sqliteInt_h::Bitmask;pub use crate::src::src::bitvec::Bitvec;pub use crate::src::headers::sqliteInt_h::BusyHandler;pub use crate::src::headers::sqliteInt_h::CollSeq;pub use crate::src::headers::sqliteInt_h::Column;pub use crate::src::headers::sqliteInt_h::Cte;pub use crate::src::headers::sqliteInt_h::CteUse;pub use crate::src::headers::sqliteInt_h::Db;pub use crate::src::headers::sqliteInt_h::DbClientData;pub use crate::src::headers::sqliteInt_h::Expr;pub use crate::src::headers::sqliteInt_h::ExprList;pub use crate::src::headers::sqliteInt_h::ExprList_item;pub use crate::src::headers::sqliteInt_h::FKey;pub use crate::src::headers::sqliteInt_h::FuncDef;pub use crate::src::headers::sqliteInt_h::FuncDestructor;pub use crate::src::headers::sqliteInt_h::IdList;pub use crate::src::headers::sqliteInt_h::IdList_item;pub use crate::src::headers::sqliteInt_h::Index;pub use crate::src::headers::sqliteInt_h::IndexedExpr;pub use crate::src::headers::sqliteInt_h::KeyInfo;pub use crate::src::headers::sqliteInt_h::LogEst;pub use crate::src::headers::sqliteInt_h::Lookaside;pub use crate::src::headers::sqliteInt_h::LookasideSlot;pub use crate::src::headers::sqliteInt_h::Module;pub use crate::src::headers::sqliteInt_h::Parse;pub use crate::src::headers::sqliteInt_h::ParseCleanup;pub use crate::src::headers::vdbeInt_h::PreUpdate;pub use crate::src::headers::sqliteInt_h::RenameToken;pub use crate::src::headers::sqliteInt_h::Returning;pub use crate::src::headers::sqliteInt_h::SQLITE_NoCkptOnClose;pub use crate::src::headers::sqliteInt_h::Savepoint;pub use crate::src::headers::sqliteInt_h::Schema;pub use crate::src::headers::sqliteInt_h::Select;pub use crate::src::headers::sqliteInt_h::Sqlite3Config;pub use crate::src::headers::sqliteInt_h::SrcItem;pub use crate::src::headers::sqliteInt_h::SrcList;pub use crate::src::headers::sqliteInt_h::Subquery;pub use crate::src::headers::sqliteInt_h::Table;pub use crate::src::headers::sqliteInt_h::TableLock;pub use crate::src::headers::sqliteInt_h::Token;pub use crate::src::headers::sqliteInt_h::Trigger;pub use crate::src::headers::sqliteInt_h::TriggerPrg;pub use crate::src::headers::sqliteInt_h::TriggerStep;pub use crate::src::headers::sqliteInt_h::Upsert;pub use crate::src::headers::sqliteInt_h::VList;pub use crate::src::headers::sqliteInt_h::VTable;pub use crate::src::headers::sqliteInt_h::VtabCtx;pub use crate::src::headers::sqliteInt_h::Window;pub use crate::src::headers::sqliteInt_h::With;pub use crate::src::headers::sqliteInt_h::SAVEPOINT_RELEASE;pub use crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK;pub use crate::src::headers::sqliteInt_h::SQLITE_DEFAULT_SYNCHRONOUS;pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;pub use crate::src::headers::sqliteInt_h::SQLITE_PTRSIZE;pub use crate::sqliteLimit_h::SQLITE_MAX_DEFAULT_PAGE_SIZE;pub use crate::sqliteLimit_h::SQLITE_MAX_PAGE_COUNT;pub use crate::sqliteLimit_h::SQLITE_MAX_PAGE_SIZE;
pub use crate::stdlib::int16_t;




pub use crate::stdlib::uint16_t;pub use crate::stdlib::uint32_t;pub use crate::stdlib::uint8_t;pub use crate::stdlib::__int16_t;pub use crate::stdlib::__uint16_t;pub use crate::stdlib::__uint32_t;pub use crate::stdlib::__uint8_t;pub use crate::src::src::vdbe::p4union;pub use crate::src::src::vdbe::Mem;pub use crate::src::src::vdbe::SubProgram;pub use crate::src::src::vdbe::SubrtnSig;pub use crate::src::headers::vdbeInt_h::Vdbe;pub use crate::src::src::vdbe::VdbeOp;
extern "C" {
    
    pub static mut sqlite3_io_error_pending: ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct Pager {
    pub pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    pub exclusiveMode: crate::src::ext::rtree::rtree::u8_0,
    pub journalMode: crate::src::ext::rtree::rtree::u8_0,
    pub useJournal: crate::src::ext::rtree::rtree::u8_0,
    pub noSync: crate::src::ext::rtree::rtree::u8_0,
    pub fullSync: crate::src::ext::rtree::rtree::u8_0,
    pub extraSync: crate::src::ext::rtree::rtree::u8_0,
    pub syncFlags: crate::src::ext::rtree::rtree::u8_0,
    pub walSyncFlags: crate::src::ext::rtree::rtree::u8_0,
    pub tempFile: crate::src::ext::rtree::rtree::u8_0,
    pub noLock: crate::src::ext::rtree::rtree::u8_0,
    pub readOnly: crate::src::ext::rtree::rtree::u8_0,
    pub memDb: crate::src::ext::rtree::rtree::u8_0,
    pub memVfs: crate::src::ext::rtree::rtree::u8_0,
    pub eState: crate::src::ext::rtree::rtree::u8_0,
    pub eLock: crate::src::ext::rtree::rtree::u8_0,
    pub changeCountDone: crate::src::ext::rtree::rtree::u8_0,
    pub setSuper: crate::src::ext::rtree::rtree::u8_0,
    pub doNotSpill: crate::src::ext::rtree::rtree::u8_0,
    pub subjInMemory: crate::src::ext::rtree::rtree::u8_0,
    pub bUseFetch: crate::src::ext::rtree::rtree::u8_0,
    pub hasHeldSharedLock: crate::src::ext::rtree::rtree::u8_0,
    pub dbSize: crate::src::src::pager::Pgno,
    pub dbOrigSize: crate::src::src::pager::Pgno,
    pub dbFileSize: crate::src::src::pager::Pgno,
    pub dbHintSize: crate::src::src::pager::Pgno,
    pub errCode: ::core::ffi::c_int,
    pub nRec: ::core::ffi::c_int,
    pub cksumInit: crate::src::ext::rtree::rtree::u32_0,
    pub nSubRec: crate::src::ext::rtree::rtree::u32_0,
    pub pInJournal: *mut crate::src::src::bitvec::Bitvec,
    pub fd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    pub jfd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    pub sjfd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    pub journalOff: crate::src::ext::rtree::rtree::i64_0,
    pub journalHdr: crate::src::ext::rtree::rtree::i64_0,
    pub pBackup: *mut crate::src::src::backup::sqlite3_backup,
    pub aSavepoint: *mut PagerSavepoint,
    pub nSavepoint: ::core::ffi::c_int,
    pub iDataVersion: crate::src::ext::rtree::rtree::u32_0,
    pub dbFileVers: [::core::ffi::c_char; 16],
    pub nMmapOut: ::core::ffi::c_int,
    pub szMmap: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub pMmapFreelist: *mut crate::src::src::pcache::PgHdr,
    pub nExtra: crate::src::fts5::u16_0,
    pub nReserve: crate::src::fts5::i16_0,
    pub vfsFlags: crate::src::ext::rtree::rtree::u32_0,
    pub sectorSize: crate::src::ext::rtree::rtree::u32_0,
    pub mxPgno: crate::src::src::pager::Pgno,
    pub lckPgno: crate::src::src::pager::Pgno,
    pub pageSize: crate::src::ext::rtree::rtree::i64_0,
    pub journalSizeLimit: crate::src::ext::rtree::rtree::i64_0,
    pub zFilename: *mut ::core::ffi::c_char,
    pub zJournal: *mut ::core::ffi::c_char,
    pub xBusyHandler: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub pBusyHandlerArg: *mut ::core::ffi::c_void,
    pub aStat: [crate::src::ext::rtree::rtree::u32_0; 4],
    pub nRead: ::core::ffi::c_int,
    pub xReiniter: Option<unsafe extern "C" fn(*mut crate::src::src::pager::DbPage) -> ()>,
    pub xGet: Option<
        unsafe extern "C" fn(
            *mut Pager,
            crate::src::src::pager::Pgno,
            *mut *mut crate::src::src::pager::DbPage,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pTmpSpace: *mut ::core::ffi::c_char,
    pub pPCache: *mut crate::pcache_h::PCache,
    pub pWal: *mut crate::src::src::wal::Wal,
    pub zWal: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct PagerSavepoint {
    pub iOffset: crate::src::ext::rtree::rtree::i64_0,
    pub iHdrOffset: crate::src::ext::rtree::rtree::i64_0,
    pub pInSavepoint: *mut crate::src::src::bitvec::Bitvec,
    pub nOrig: crate::src::src::pager::Pgno,
    pub iSubRec: crate::src::src::pager::Pgno,
    pub bTruncateOnRelease: ::core::ffi::c_int,
    pub aWalData: [crate::src::ext::rtree::rtree::u32_0; 4],
}

pub const PAGER_OPEN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const PAGER_READER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const PAGER_WRITER_LOCKED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const PAGER_WRITER_CACHEMOD: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const PAGER_WRITER_DBMOD: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const PAGER_WRITER_FINISHED: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const PAGER_ERROR: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const UNKNOWN_LOCK: ::core::ffi::c_int = crate::src::src::os::EXCLUSIVE_LOCK_1 + 1 as ::core::ffi::c_int;

pub const MAX_SECTOR_SIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

pub const SPILLFLAG_OFF: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SPILLFLAG_ROLLBACK: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SPILLFLAG_NOSYNC: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const PAGER_STAT_HIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const PAGER_STAT_MISS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const PAGER_STAT_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const PAGER_STAT_SPILL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[no_mangle]

pub static mut sqlite3_pager_readdb_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]

pub static mut sqlite3_pager_writedb_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]

pub static mut sqlite3_pager_writej_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

static mut aJournalMagic: [::core::ffi::c_uchar; 8] = [
    0xd9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerDirectReadOk(
    mut pPager: *mut Pager,
    mut pgno: crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    let __pPager_ref = unsafe { &mut *pPager };
    if (*__pPager_ref.fd).pMethods.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if crate::src::src::pcache::sqlite3PCacheIsDirty(__pPager_ref.pPCache) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !__pPager_ref.pWal.is_null() {
        let mut iRead: crate::src::ext::rtree::rtree::u32_0 = 0 as crate::src::ext::rtree::rtree::u32_0;
        crate::src::src::wal::sqlite3WalFindFrame(__pPager_ref.pWal, pgno, &raw mut iRead);
        if iRead != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*(*__pPager_ref.fd).pMethods)
        .xDeviceCharacteristics
        .expect("non-null function pointer")(__pPager_ref.fd)
        & crate::src::headers::sqlite3_h::SQLITE_IOCAP_SUBPAGE_READ
        == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn setGetterMethod(mut pPager: *mut Pager) {
    if (*pPager).errCode != 0 {
        (*pPager).xGet = Some(
            getPageError
                as unsafe extern "C" fn(
                    *mut Pager,
                    crate::src::src::pager::Pgno,
                    *mut *mut crate::src::src::pager::DbPage,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut Pager,
                    crate::src::src::pager::Pgno,
                    *mut *mut crate::src::src::pager::DbPage,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
    } else if (*pPager).bUseFetch != 0 {
        (*pPager).xGet = Some(
            getPageMMap
                as unsafe extern "C" fn(
                    *mut Pager,
                    crate::src::src::pager::Pgno,
                    *mut *mut crate::src::src::pager::DbPage,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut Pager,
                    crate::src::src::pager::Pgno,
                    *mut *mut crate::src::src::pager::DbPage,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
    } else {
        (*pPager).xGet = Some(
            getPageNormal
                as unsafe extern "C" fn(
                    *mut Pager,
                    crate::src::src::pager::Pgno,
                    *mut *mut crate::src::src::pager::DbPage,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut Pager,
                    crate::src::src::pager::Pgno,
                    *mut *mut crate::src::src::pager::DbPage,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
    };
}

unsafe extern "C" fn subjRequiresPage(mut pPg: *mut crate::src::src::pcache::PgHdr) -> ::core::ffi::c_int {
    let mut pPager: *mut Pager = (*pPg).pPager;
    let mut p: *mut PagerSavepoint = ::core::ptr::null_mut::<PagerSavepoint>();
    let mut pgno: crate::src::src::pager::Pgno = (*pPg).pgno;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pPager).nSavepoint {
        p = (*pPager).aSavepoint.offset(i as isize) as *mut PagerSavepoint;
        if (*p).nOrig >= pgno
            && 0 as ::core::ffi::c_int == crate::src::src::bitvec::sqlite3BitvecTestNotNull((*p).pInSavepoint, pgno as crate::src::ext::rtree::rtree::u32_0)
        {
            i += 1 as ::core::ffi::c_int;
            while i < (*pPager).nSavepoint {
                (*(*pPager).aSavepoint.offset(i as isize)).bTruncateOnRelease =
                    0 as ::core::ffi::c_int;
                i += 1;
            }
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn read32bits(
    mut fd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut offset: crate::src::ext::rtree::rtree::i64_0,
    mut pRes: *mut crate::src::ext::rtree::rtree::u32_0,
) -> ::core::ffi::c_int {
    let mut ac: [::core::ffi::c_uchar; 4] = [0; 4];
    let mut rc: ::core::ffi::c_int = crate::src::src::os::sqlite3OsRead(
        
        fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
        &raw mut ac as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 4]>() as ::core::ffi::c_int,
        offset,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        *pRes = crate::src::src::util::sqlite3Get4byte(&raw mut ac as *mut ::core::ffi::c_uchar);
    }
    rc
}

unsafe extern "C" fn write32bits(
    mut fd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut offset: crate::src::ext::rtree::rtree::i64_0,
    mut val: crate::src::ext::rtree::rtree::u32_0,
) -> ::core::ffi::c_int {
    let mut ac: [::core::ffi::c_char; 4] = [0; 4];
    crate::src::src::util::sqlite3Put4byte(&raw mut ac as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0, val);
    crate::src::src::os::sqlite3OsWrite(
        
        fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
        &raw mut ac as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as ::core::ffi::c_int,
        offset,
    )
}

unsafe extern "C" fn pagerUnlockDb(
    mut pPager: *mut Pager,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if !(*__pPager_ref.fd).pMethods.is_null() {
        rc = if __pPager_ref.noLock as ::core::ffi::c_int != 0 {
            crate::src::headers::sqlite3_h::SQLITE_OK
        } else {
            crate::src::src::os::sqlite3OsUnlock(__pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file, eLock)
        };
        if __pPager_ref.eLock as ::core::ffi::c_int != UNKNOWN_LOCK {
            __pPager_ref.eLock = eLock as crate::src::ext::rtree::rtree::u8_0;
        }
    }
    __pPager_ref.changeCountDone = __pPager_ref.tempFile;
    rc
}

unsafe extern "C" fn pagerLockDb(
    mut pPager: *mut Pager,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if ((*pPager).eLock as ::core::ffi::c_int) < eLock
        || (*pPager).eLock as ::core::ffi::c_int == UNKNOWN_LOCK
    {
        rc = if (*pPager).noLock as ::core::ffi::c_int != 0 {
            crate::src::headers::sqlite3_h::SQLITE_OK
        } else {
            crate::src::src::os::sqlite3OsLock((*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file, eLock)
        };
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && ((*pPager).eLock as ::core::ffi::c_int != UNKNOWN_LOCK || eLock == crate::src::src::os::EXCLUSIVE_LOCK_1)
        {
            (*pPager).eLock = eLock as crate::src::ext::rtree::rtree::u8_0;
        }
    }
    rc
}

unsafe extern "C" fn jrnlBufferSize(mut _pPager: *mut Pager) -> ::core::ffi::c_int {
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn readSuperJournal(
    mut pJrnl: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut zSuper: *mut ::core::ffi::c_char,
    mut nSuper: crate::src::ext::rtree::rtree::u64_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut len: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut szJ: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut cksum: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut u: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut aMagic: [::core::ffi::c_uchar; 8] = [0; 8];
    *zSuper.offset(0 as isize) = '\0' as i32 as ::core::ffi::c_char;
    rc = crate::src::src::os::sqlite3OsFileSize(pJrnl as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut szJ);
    if crate::src::headers::sqlite3_h::SQLITE_OK != rc
        || szJ < 16 as crate::src::ext::rtree::rtree::i64_0
        || {
            rc = read32bits(pJrnl, szJ - 16 as crate::src::ext::rtree::rtree::i64_0, &raw mut len);
            crate::src::headers::sqlite3_h::SQLITE_OK != rc
        }
        || len as crate::src::ext::rtree::rtree::u64_0 >= nSuper
        || len as crate::src::ext::rtree::rtree::i64_0 > szJ - 16 as crate::src::ext::rtree::rtree::i64_0
        || len == 0 as crate::src::ext::rtree::rtree::u32_0
        || {
            rc = read32bits(pJrnl, szJ - 12 as crate::src::ext::rtree::rtree::i64_0, &raw mut cksum);
            crate::src::headers::sqlite3_h::SQLITE_OK != rc
        }
        || {
            rc = crate::src::src::os::sqlite3OsRead(
                
                pJrnl as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                &raw mut aMagic as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                8 as ::core::ffi::c_int,
                szJ - 8 as crate::src::ext::rtree::rtree::i64_0,
            );
            crate::src::headers::sqlite3_h::SQLITE_OK != rc
        }
        || ::libc::memcmp(
            &raw mut aMagic as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            &raw const aJournalMagic as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            8 as crate::__stddef_size_t_h::size_t,
        ) != 0
        || {
            rc = crate::src::src::os::sqlite3OsRead(
                
                pJrnl as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                zSuper as *mut ::core::ffi::c_void,
                len as ::core::ffi::c_int,
                szJ - 16 as crate::src::ext::rtree::rtree::i64_0 - len as crate::src::ext::rtree::rtree::i64_0,
            );
            crate::src::headers::sqlite3_h::SQLITE_OK != rc
        }
    {
        return rc;
    }
    u = 0 as crate::src::ext::rtree::rtree::u32_0;
    while u < len {
        cksum = cksum.wrapping_sub(*zSuper.offset(u as isize) as crate::src::ext::rtree::rtree::u32_0);
        u = u.wrapping_add(1);
    }
    if cksum != 0 {
        len = 0 as crate::src::ext::rtree::rtree::u32_0;
    }
    *zSuper.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    *zSuper.offset(len.wrapping_add(1 as crate::src::ext::rtree::rtree::u32_0) as isize) = '\0' as i32 as ::core::ffi::c_char;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn journalHdrOffset(mut pPager: *mut Pager) -> crate::src::ext::rtree::rtree::i64_0 {
    let mut offset: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
    let mut c: crate::src::ext::rtree::rtree::i64_0 = (*pPager).journalOff;
    if c != 0 {
        offset = ((c - 1 as crate::src::ext::rtree::rtree::i64_0) / (*pPager).sectorSize as crate::src::ext::rtree::rtree::i64_0 + 1 as crate::src::ext::rtree::rtree::i64_0)
            * (*pPager).sectorSize as crate::src::ext::rtree::rtree::i64_0;
    }
    offset
}

unsafe extern "C" fn zeroJournalHdr(
    mut pPager: *mut Pager,
    mut doTruncate: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pPager).journalOff != 0 {
        let iLimit: crate::src::ext::rtree::rtree::i64_0 = (*pPager).journalSizeLimit;
        if doTruncate != 0 || iLimit == 0 as crate::src::ext::rtree::rtree::i64_0 {
            rc = crate::src::src::os::sqlite3OsTruncate((*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, 0 as crate::src::ext::rtree::rtree::i64_0);
        } else {
            static mut zeroHdr: [::core::ffi::c_char; 28] = [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ];
            rc = crate::src::src::os::sqlite3OsWrite(
                
                (*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                &raw const zeroHdr as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 28]>() as ::core::ffi::c_int,
                0 as crate::src::ext::rtree::rtree::i64_0,
            );
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && (*pPager).noSync == 0 {
            rc = crate::src::src::os::sqlite3OsSync(
                
                (*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                crate::src::headers::sqlite3_h::SQLITE_SYNC_DATAONLY | (*pPager).syncFlags as ::core::ffi::c_int,
            );
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && iLimit > 0 as crate::src::ext::rtree::rtree::i64_0 {
            let mut sz: crate::src::ext::rtree::rtree::i64_0 = 0;
            rc = crate::src::src::os::sqlite3OsFileSize((*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut sz);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK && sz > iLimit {
                rc = crate::src::src::os::sqlite3OsTruncate((*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, iLimit);
            }
        }
    }
    rc
}

unsafe extern "C" fn writeJournalHdr(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    let mut zHeader: *mut ::core::ffi::c_char = __pPager_ref.pTmpSpace;
    let mut nHeader: crate::src::ext::rtree::rtree::u32_0 = __pPager_ref.pageSize as crate::src::ext::rtree::rtree::u32_0;
    let mut nWrite: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut ii: ::core::ffi::c_int = 0;
    if nHeader > __pPager_ref.sectorSize {
        nHeader = __pPager_ref.sectorSize;
    }
    ii = 0 as ::core::ffi::c_int;
    while ii < __pPager_ref.nSavepoint {
        if (*__pPager_ref.aSavepoint.offset(ii as isize)).iHdrOffset == 0 as crate::src::ext::rtree::rtree::i64_0 {
            (*__pPager_ref.aSavepoint.offset(ii as isize)).iHdrOffset = __pPager_ref.journalOff;
        }
        ii += 1;
    }
    __pPager_ref.journalOff = journalHdrOffset(pPager);
    __pPager_ref.journalHdr = __pPager_ref.journalOff;
    if __pPager_ref.noSync as ::core::ffi::c_int != 0
        || __pPager_ref.journalMode as ::core::ffi::c_int == crate::src::src::pager::PAGER_JOURNALMODE_MEMORY
        || crate::src::src::os::sqlite3OsDeviceCharacteristics(__pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file) & crate::src::headers::sqlite3_h::SQLITE_IOCAP_SAFE_APPEND != 0
    {
        ::core::ptr::copy_nonoverlapping(
                    &raw const aJournalMagic as *const ::core::ffi::c_uchar as *const u8,
                    zHeader as *mut u8,
                    ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize,
                );
        crate::src::src::util::sqlite3Put4byte(
            zHeader.offset(::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as isize)
                as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0,
            0xffffffff as crate::src::ext::rtree::rtree::u32_0,
        );
    } else {
        ::libc::memset(
            zHeader as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as crate::__stddef_size_t_h::size_t)
                .wrapping_add(4 as crate::__stddef_size_t_h::size_t),
        );
    }
    if __pPager_ref.journalMode as ::core::ffi::c_int != crate::src::src::pager::PAGER_JOURNALMODE_MEMORY {
        crate::src::src::random::sqlite3_randomness(
            ::core::mem::size_of::<crate::src::ext::rtree::rtree::u32_0>() as ::core::ffi::c_int,
            &raw mut __pPager_ref.cksumInit as *mut ::core::ffi::c_void,
        );
    }
    crate::src::src::util::sqlite3Put4byte(
        zHeader.offset(
            (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize).wrapping_add(4 as usize)
                as isize,
        ) as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0,
        __pPager_ref.cksumInit,
    );
    crate::src::src::util::sqlite3Put4byte(
        zHeader.offset(
            (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize).wrapping_add(8 as usize)
                as isize,
        ) as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0,
        __pPager_ref.dbOrigSize as crate::src::ext::rtree::rtree::u32_0,
    );
    crate::src::src::util::sqlite3Put4byte(
        zHeader.offset(
            (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize).wrapping_add(12 as usize)
                as isize,
        ) as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0,
        __pPager_ref.sectorSize,
    );
    crate::src::src::util::sqlite3Put4byte(
        zHeader.offset(
            (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize).wrapping_add(16 as usize)
                as isize,
        ) as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0,
        __pPager_ref.pageSize as crate::src::ext::rtree::rtree::u32_0,
    );
    ::libc::memset(
        zHeader.offset(
            (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize).wrapping_add(20 as usize)
                as isize,
        ) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (nHeader as crate::__stddef_size_t_h::size_t).wrapping_sub(
            (::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as crate::__stddef_size_t_h::size_t)
                .wrapping_add(20 as crate::__stddef_size_t_h::size_t),
        ),
    );
    nWrite = 0 as crate::src::ext::rtree::rtree::u32_0;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && nWrite < __pPager_ref.sectorSize {
        rc = crate::src::src::os::sqlite3OsWrite(
            
            __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            zHeader as *const ::core::ffi::c_void,
            nHeader as ::core::ffi::c_int,
            __pPager_ref.journalOff,
        );
        __pPager_ref.journalOff += nHeader as crate::src::ext::rtree::rtree::i64_0;
        nWrite = nWrite.wrapping_add(nHeader);
    }
    rc
}

unsafe extern "C" fn readJournalHdr(
    mut pPager: *mut Pager,
    mut isHot: ::core::ffi::c_int,
    mut journalSize: crate::src::ext::rtree::rtree::i64_0,
    mut pNRec: *mut crate::src::ext::rtree::rtree::u32_0,
    mut pDbSize: *mut crate::src::ext::rtree::rtree::u32_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut aMagic: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut iHdrOff: crate::src::ext::rtree::rtree::i64_0 = 0;
    let __pPager_ref = unsafe { &mut *pPager };
    __pPager_ref.journalOff = journalHdrOffset(pPager);
    if __pPager_ref.journalOff + __pPager_ref.sectorSize as crate::src::ext::rtree::rtree::i64_0 > journalSize {
        return crate::src::headers::sqlite3_h::SQLITE_DONE;
    }
    iHdrOff = __pPager_ref.journalOff;
    if isHot != 0 || iHdrOff != __pPager_ref.journalHdr {
        rc = crate::src::src::os::sqlite3OsRead(
            
            __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            &raw mut aMagic as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as ::core::ffi::c_int,
            iHdrOff,
        );
        if rc != 0 {
            return rc;
        }
        if ::libc::memcmp(
            &raw mut aMagic as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            &raw const aJournalMagic as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as crate::__stddef_size_t_h::size_t,
        ) != 0 as ::core::ffi::c_int
        {
            return crate::src::headers::sqlite3_h::SQLITE_DONE;
        }
    }
    rc = read32bits(__pPager_ref.jfd, iHdrOff + 8 as crate::src::ext::rtree::rtree::i64_0, pNRec);
    if crate::src::headers::sqlite3_h::SQLITE_OK != rc
        || {
            rc = read32bits(
                __pPager_ref.jfd,
                iHdrOff + 12 as crate::src::ext::rtree::rtree::i64_0,
                &raw mut __pPager_ref.cksumInit,
            );
            crate::src::headers::sqlite3_h::SQLITE_OK != rc
        }
        || {
            rc = read32bits(__pPager_ref.jfd, iHdrOff + 16 as crate::src::ext::rtree::rtree::i64_0, pDbSize);
            crate::src::headers::sqlite3_h::SQLITE_OK != rc
        }
    {
        return rc;
    }
    if __pPager_ref.journalOff == 0 as crate::src::ext::rtree::rtree::i64_0 {
        let mut iPageSize: crate::src::ext::rtree::rtree::u32_0 = 0;
        let mut iSectorSize: crate::src::ext::rtree::rtree::u32_0 = 0;
        rc = read32bits(__pPager_ref.jfd, iHdrOff + 20 as crate::src::ext::rtree::rtree::i64_0, &raw mut iSectorSize);
        if crate::src::headers::sqlite3_h::SQLITE_OK != rc || {
            rc = read32bits(__pPager_ref.jfd, iHdrOff + 24 as crate::src::ext::rtree::rtree::i64_0, &raw mut iPageSize);
            crate::src::headers::sqlite3_h::SQLITE_OK != rc
        } {
            return rc;
        }
        if iPageSize == 0 as crate::src::ext::rtree::rtree::u32_0 {
            iPageSize = __pPager_ref.pageSize as crate::src::ext::rtree::rtree::u32_0;
        }
        if iPageSize < 512 as crate::src::ext::rtree::rtree::u32_0
            || iSectorSize < 32 as crate::src::ext::rtree::rtree::u32_0
            || iPageSize > crate::sqliteLimit_h::SQLITE_MAX_PAGE_SIZE as crate::src::ext::rtree::rtree::u32_0
            || iSectorSize > MAX_SECTOR_SIZE as crate::src::ext::rtree::rtree::u32_0
            || iPageSize.wrapping_sub(1 as crate::src::ext::rtree::rtree::u32_0) & iPageSize != 0 as crate::src::ext::rtree::rtree::u32_0
            || iSectorSize.wrapping_sub(1 as crate::src::ext::rtree::rtree::u32_0) & iSectorSize != 0 as crate::src::ext::rtree::rtree::u32_0
        {
            return crate::src::headers::sqlite3_h::SQLITE_DONE;
        }
        rc = sqlite3PagerSetPagesize(pPager, &raw mut iPageSize, -(1 as ::core::ffi::c_int));
        __pPager_ref.sectorSize = iSectorSize;
    }
    __pPager_ref.journalOff += __pPager_ref.sectorSize as crate::src::ext::rtree::rtree::i64_0;
    rc
}

unsafe extern "C" fn writeSuperJournal(
    mut pPager: *mut Pager,
    mut zSuper: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut nSuper: ::core::ffi::c_int = 0;
    let mut iHdrOff: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut jrnlSize: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut cksum: crate::src::ext::rtree::rtree::u32_0 = 0 as crate::src::ext::rtree::rtree::u32_0;
    let __pPager_ref = unsafe { &mut *pPager };
    if zSuper.is_null()
        || __pPager_ref.journalMode as ::core::ffi::c_int == crate::src::src::pager::PAGER_JOURNALMODE_MEMORY
        || (*__pPager_ref.jfd).pMethods.is_null()
    {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    __pPager_ref.setSuper = 1 as crate::src::ext::rtree::rtree::u8_0;
    nSuper = 0 as ::core::ffi::c_int;
    while *zSuper.offset(nSuper as isize) != 0 {
        cksum = cksum.wrapping_add(*zSuper.offset(nSuper as isize) as crate::src::ext::rtree::rtree::u32_0);
        nSuper += 1;
    }
    if __pPager_ref.fullSync != 0 {
        __pPager_ref.journalOff = journalHdrOffset(pPager);
    }
    iHdrOff = __pPager_ref.journalOff;
    rc = write32bits(__pPager_ref.jfd, iHdrOff, __pPager_ref.lckPgno as crate::src::ext::rtree::rtree::u32_0);
    if 0 as ::core::ffi::c_int != rc
        || {
            rc = crate::src::src::os::sqlite3OsWrite(
                
                __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                zSuper as *const ::core::ffi::c_void,
                nSuper,
                iHdrOff + 4 as crate::src::ext::rtree::rtree::i64_0,
            );
            0 as ::core::ffi::c_int != rc
        }
        || {
            rc = write32bits(
                __pPager_ref.jfd,
                iHdrOff + 4 as crate::src::ext::rtree::rtree::i64_0 + nSuper as crate::src::ext::rtree::rtree::i64_0,
                nSuper as crate::src::ext::rtree::rtree::u32_0,
            );
            0 as ::core::ffi::c_int != rc
        }
        || {
            rc = write32bits(
                __pPager_ref.jfd,
                iHdrOff + 4 as crate::src::ext::rtree::rtree::i64_0 + nSuper as crate::src::ext::rtree::rtree::i64_0 + 4 as crate::src::ext::rtree::rtree::i64_0,
                cksum,
            );
            0 as ::core::ffi::c_int != rc
        }
        || {
            rc = crate::src::src::os::sqlite3OsWrite(
                
                __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                &raw const aJournalMagic as *const ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                8 as ::core::ffi::c_int,
                iHdrOff + 4 as crate::src::ext::rtree::rtree::i64_0 + nSuper as crate::src::ext::rtree::rtree::i64_0 + 8 as crate::src::ext::rtree::rtree::i64_0,
            );
            0 as ::core::ffi::c_int != rc
        }
    {
        return rc;
    }
    __pPager_ref.journalOff += (nSuper + 20 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
    rc = crate::src::src::os::sqlite3OsFileSize(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut jrnlSize);
    if crate::src::headers::sqlite3_h::SQLITE_OK == rc && jrnlSize > __pPager_ref.journalOff {
        rc = crate::src::src::os::sqlite3OsTruncate(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, __pPager_ref.journalOff);
    }
    rc
}

unsafe extern "C" fn pager_reset(mut pPager: *mut Pager) {
    let __pPager_ref = unsafe { &mut *pPager };
    __pPager_ref.iDataVersion = __pPager_ref.iDataVersion.wrapping_add(1);
    crate::src::src::backup::sqlite3BackupRestart(__pPager_ref.pBackup);
    crate::src::src::pcache::sqlite3PcacheClear(__pPager_ref.pPCache);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerDataVersion(mut pPager: *mut Pager) -> crate::src::ext::rtree::rtree::u32_0 {
    (*pPager).iDataVersion
}

unsafe extern "C" fn releaseAllSavepoints(mut pPager: *mut Pager) {
    let mut ii: ::core::ffi::c_int = 0;
    ii = 0 as ::core::ffi::c_int;
    let __pPager_ref = unsafe { &mut *pPager };
    while ii < __pPager_ref.nSavepoint {
        crate::src::src::bitvec::sqlite3BitvecDestroy((*__pPager_ref.aSavepoint.offset(ii as isize)).pInSavepoint);
        ii += 1;
    }
    if __pPager_ref.exclusiveMode == 0 || crate::src::src::memjournal::sqlite3JournalIsInMemory(__pPager_ref.sjfd as *mut crate::src::headers::sqlite3_h::sqlite3_file) != 0 {
        crate::src::src::os::sqlite3OsClose(__pPager_ref.sjfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
    }
    crate::src::src::malloc::sqlite3_free(__pPager_ref.aSavepoint as *mut ::core::ffi::c_void);
    __pPager_ref.aSavepoint = ::core::ptr::null_mut::<PagerSavepoint>();
    __pPager_ref.nSavepoint = 0 as ::core::ffi::c_int;
    __pPager_ref.nSubRec = 0 as crate::src::ext::rtree::rtree::u32_0;
}

unsafe extern "C" fn addToSavepointBitvecs(
    mut pPager: *mut Pager,
    mut pgno: crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    let mut ii: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    ii = 0 as ::core::ffi::c_int;
    while ii < (*pPager).nSavepoint {
        let mut p: *mut PagerSavepoint =
            (*pPager).aSavepoint.offset(ii as isize) as *mut PagerSavepoint;
        if pgno <= (*p).nOrig {
            rc |= crate::src::src::bitvec::sqlite3BitvecSet((*p).pInSavepoint, pgno as crate::src::ext::rtree::rtree::u32_0);
        }
        ii += 1;
    }
    rc
}

unsafe extern "C" fn pager_unlock(mut pPager: *mut Pager) {
    let __pPager_ref = unsafe { &mut *pPager };
    crate::src::src::bitvec::sqlite3BitvecDestroy(__pPager_ref.pInJournal);
    __pPager_ref.pInJournal = ::core::ptr::null_mut::<crate::src::src::bitvec::Bitvec>();
    releaseAllSavepoints(pPager);
    if !__pPager_ref.pWal.is_null() {
        if __pPager_ref.eState as ::core::ffi::c_int == PAGER_ERROR {
            crate::src::src::wal::sqlite3WalEndWriteTransaction(__pPager_ref.pWal);
        }
        crate::src::src::wal::sqlite3WalEndReadTransaction(__pPager_ref.pWal);
        __pPager_ref.eState = PAGER_OPEN as crate::src::ext::rtree::rtree::u8_0;
    } else if __pPager_ref.exclusiveMode == 0 {
        let mut rc: ::core::ffi::c_int = 0;
        let mut iDc: ::core::ffi::c_int = if !(*__pPager_ref.fd).pMethods.is_null() {
            crate::src::src::os::sqlite3OsDeviceCharacteristics(__pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file)
        } else {
            0 as ::core::ffi::c_int
        };
        if 0 as ::core::ffi::c_int == iDc & crate::src::headers::sqlite3_h::SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN
            || 1 as ::core::ffi::c_int
                != __pPager_ref.journalMode as ::core::ffi::c_int & 5 as ::core::ffi::c_int
        {
            crate::src::src::os::sqlite3OsClose(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
        }
        rc = pagerUnlockDb(pPager, crate::src::src::os::NO_LOCK);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK && __pPager_ref.eState as ::core::ffi::c_int == PAGER_ERROR {
            __pPager_ref.eLock = UNKNOWN_LOCK as crate::src::ext::rtree::rtree::u8_0;
        }
        __pPager_ref.eState = PAGER_OPEN as crate::src::ext::rtree::rtree::u8_0;
    }
    if __pPager_ref.errCode != 0 {
        if __pPager_ref.tempFile as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            pager_reset(pPager);
            __pPager_ref.changeCountDone = 0 as crate::src::ext::rtree::rtree::u8_0;
            __pPager_ref.eState = PAGER_OPEN as crate::src::ext::rtree::rtree::u8_0;
        } else {
            __pPager_ref.eState = (if !(*__pPager_ref.jfd).pMethods.is_null() {
                PAGER_OPEN
            } else {
                PAGER_READER
            }) as crate::src::ext::rtree::rtree::u8_0;
        }
        if __pPager_ref.bUseFetch != 0 {
            crate::src::src::os::sqlite3OsUnfetch(
                
                __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                0 as crate::src::ext::rtree::rtree::i64_0,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        __pPager_ref.errCode = crate::src::headers::sqlite3_h::SQLITE_OK;
        setGetterMethod(pPager);
    }
    __pPager_ref.journalOff = 0 as crate::src::ext::rtree::rtree::i64_0;
    __pPager_ref.journalHdr = 0 as crate::src::ext::rtree::rtree::i64_0;
    __pPager_ref.setSuper = 0 as crate::src::ext::rtree::rtree::u8_0;
}

unsafe extern "C" fn pager_error(
    mut pPager: *mut Pager,
    mut rc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc2: ::core::ffi::c_int = rc & 0xff as ::core::ffi::c_int;
    if rc2 == crate::src::headers::sqlite3_h::SQLITE_FULL || rc2 == crate::src::headers::sqlite3_h::SQLITE_IOERR {
        (*pPager).errCode = rc;
        (*pPager).eState = PAGER_ERROR as crate::src::ext::rtree::rtree::u8_0;
        setGetterMethod(pPager);
    }
    rc
}

unsafe extern "C" fn pagerFlushOnCommit(
    mut pPager: *mut Pager,
    mut bCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.tempFile as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if bCommit == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*__pPager_ref.fd).pMethods.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (crate::src::src::pcache::sqlite3PCachePercentDirty(__pPager_ref.pPCache) >= 25 as ::core::ffi::c_int)
        as ::core::ffi::c_int
}

unsafe extern "C" fn pager_end_transaction(
    mut pPager: *mut Pager,
    mut hasSuper: ::core::ffi::c_int,
    mut bCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut rc2: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if (__pPager_ref.eState as ::core::ffi::c_int) < PAGER_WRITER_LOCKED
        && (__pPager_ref.eLock as ::core::ffi::c_int) < crate::src::src::os::RESERVED_LOCK_1
    {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    releaseAllSavepoints(pPager);
    if !(*__pPager_ref.jfd).pMethods.is_null() {
        if crate::src::src::memjournal::sqlite3JournalIsInMemory(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file) != 0 {
            crate::src::src::os::sqlite3OsClose(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
        } else if __pPager_ref.journalMode as ::core::ffi::c_int == crate::src::src::pager::PAGER_JOURNALMODE_TRUNCATE {
            if __pPager_ref.journalOff == 0 as crate::src::ext::rtree::rtree::i64_0 {
                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
            } else {
                rc = crate::src::src::os::sqlite3OsTruncate(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, 0 as crate::src::ext::rtree::rtree::i64_0);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK && __pPager_ref.fullSync as ::core::ffi::c_int != 0 {
                    rc = crate::src::src::os::sqlite3OsSync(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, __pPager_ref.syncFlags as ::core::ffi::c_int);
                }
            }
            __pPager_ref.journalOff = 0 as crate::src::ext::rtree::rtree::i64_0;
        } else if __pPager_ref.journalMode as ::core::ffi::c_int == crate::src::src::pager::PAGER_JOURNALMODE_PERSIST
            || __pPager_ref.exclusiveMode as ::core::ffi::c_int != 0
                && (__pPager_ref.journalMode as ::core::ffi::c_int) < crate::src::src::pager::PAGER_JOURNALMODE_WAL
        {
            rc = zeroJournalHdr(
                pPager,
                (hasSuper != 0 || __pPager_ref.tempFile as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int,
            );
            __pPager_ref.journalOff = 0 as crate::src::ext::rtree::rtree::i64_0;
        } else {
            let mut bDelete: ::core::ffi::c_int = (__pPager_ref.tempFile == 0) as ::core::ffi::c_int;
            crate::src::src::os::sqlite3OsClose(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
            if bDelete != 0 {
                rc = crate::src::src::os::sqlite3OsDelete(
                    
                    __pPager_ref.pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                    __pPager_ref.zJournal,
                    __pPager_ref.extraSync as ::core::ffi::c_int,
                );
            }
        }
    }
    crate::src::src::bitvec::sqlite3BitvecDestroy(__pPager_ref.pInJournal);
    __pPager_ref.pInJournal = ::core::ptr::null_mut::<crate::src::src::bitvec::Bitvec>();
    __pPager_ref.nRec = 0 as ::core::ffi::c_int;
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if __pPager_ref.memDb as ::core::ffi::c_int != 0 || pagerFlushOnCommit(pPager, bCommit) != 0 {
            crate::src::src::pcache::sqlite3PcacheCleanAll(__pPager_ref.pPCache);
        } else {
            crate::src::src::pcache::sqlite3PcacheClearWritable(__pPager_ref.pPCache);
        }
        crate::src::src::pcache::sqlite3PcacheTruncate(__pPager_ref.pPCache, __pPager_ref.dbSize);
    }
    if !__pPager_ref.pWal.is_null() {
        rc2 = crate::src::src::wal::sqlite3WalEndWriteTransaction(__pPager_ref.pWal);
    } else if rc == crate::src::headers::sqlite3_h::SQLITE_OK && bCommit != 0 && __pPager_ref.dbFileSize > __pPager_ref.dbSize {
        rc = pager_truncate(pPager, __pPager_ref.dbSize);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && bCommit != 0 {
        rc = crate::src::src::os::sqlite3OsFileControl(
            
            __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            crate::src::headers::sqlite3_h::SQLITE_FCNTL_COMMIT_PHASETWO,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_NOTFOUND {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    if __pPager_ref.exclusiveMode == 0
        && (__pPager_ref.pWal.is_null()
            || crate::src::src::wal::sqlite3WalExclusiveMode(__pPager_ref.pWal, 0 as ::core::ffi::c_int) != 0)
    {
        rc2 = pagerUnlockDb(pPager, crate::src::src::os::SHARED_LOCK);
    }
    __pPager_ref.eState = PAGER_READER as crate::src::ext::rtree::rtree::u8_0;
    __pPager_ref.setSuper = 0 as crate::src::ext::rtree::rtree::u8_0;
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK { rc2 } else { rc }
}

unsafe extern "C" fn pagerUnlockAndRollback(mut pPager: *mut Pager) {
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.eState as ::core::ffi::c_int != PAGER_ERROR
        && __pPager_ref.eState as ::core::ffi::c_int != PAGER_OPEN
    {
        if __pPager_ref.eState as ::core::ffi::c_int >= PAGER_WRITER_LOCKED {
            crate::src::src::fault::sqlite3BeginBenignMalloc();
            sqlite3PagerRollback(pPager);
            crate::src::src::fault::sqlite3EndBenignMalloc();
        } else if __pPager_ref.exclusiveMode == 0 {
            pager_end_transaction(pPager, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        }
    } else if __pPager_ref.eState as ::core::ffi::c_int == PAGER_ERROR
        && __pPager_ref.journalMode as ::core::ffi::c_int == crate::src::src::pager::PAGER_JOURNALMODE_MEMORY
        && !(*__pPager_ref.jfd).pMethods.is_null()
    {
        let mut errCode: ::core::ffi::c_int = __pPager_ref.errCode;
        let mut eLock: crate::src::ext::rtree::rtree::u8_0 = __pPager_ref.eLock;
        __pPager_ref.eState = PAGER_OPEN as crate::src::ext::rtree::rtree::u8_0;
        __pPager_ref.errCode = crate::src::headers::sqlite3_h::SQLITE_OK;
        __pPager_ref.eLock = crate::src::src::os::EXCLUSIVE_LOCK_1 as crate::src::ext::rtree::rtree::u8_0;
        pager_playback(pPager, 1 as ::core::ffi::c_int);
        __pPager_ref.errCode = errCode;
        __pPager_ref.eLock = eLock;
    }
    pager_unlock(pPager);
}

unsafe extern "C" fn pager_cksum(mut pPager: *mut Pager, mut aData: *const crate::src::ext::rtree::rtree::u8_0) -> crate::src::ext::rtree::rtree::u32_0 {
    let mut cksum: crate::src::ext::rtree::rtree::u32_0 = (*pPager).cksumInit;
    let mut i: ::core::ffi::c_int = ((*pPager).pageSize - 200 as crate::src::ext::rtree::rtree::i64_0) as ::core::ffi::c_int;
    while i > 0 as ::core::ffi::c_int {
        cksum = cksum.wrapping_add(*aData.offset(i as isize) as crate::src::ext::rtree::rtree::u32_0);
        i -= 200 as ::core::ffi::c_int;
    }
    cksum
}

unsafe extern "C" fn pager_playback_one_page(
    mut pPager: *mut Pager,
    mut pOffset: *mut crate::src::ext::rtree::rtree::i64_0,
    mut pDone: *mut crate::src::src::bitvec::Bitvec,
    mut isMainJrnl: ::core::ffi::c_int,
    mut isSavepnt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pPg: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let mut pgno: crate::src::src::pager::Pgno = 0;
    let mut cksum: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut aData: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut jfd: *mut crate::src::headers::sqlite3_h::sqlite3_file = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_file>();
    let mut isSynced: ::core::ffi::c_int = 0;
    let __pPager_ref = unsafe { &mut *pPager };
    aData = __pPager_ref.pTmpSpace;
    jfd = if isMainJrnl != 0 {
        __pPager_ref.jfd
    } else {
        __pPager_ref.sjfd
    };
    rc = read32bits(jfd, *pOffset, &raw mut pgno);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    rc = crate::src::src::os::sqlite3OsRead(
        
        jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
        aData as *mut crate::src::ext::rtree::rtree::u8_0 as *mut ::core::ffi::c_void,
        __pPager_ref.pageSize as ::core::ffi::c_int,
        *pOffset + 4 as crate::src::ext::rtree::rtree::i64_0,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    *pOffset += __pPager_ref.pageSize + 4 as crate::src::ext::rtree::rtree::i64_0 + (isMainJrnl * 4 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
    if pgno == 0 as crate::src::src::pager::Pgno || pgno == __pPager_ref.lckPgno {
        return crate::src::headers::sqlite3_h::SQLITE_DONE;
    }
    if pgno > __pPager_ref.dbSize || crate::src::src::bitvec::sqlite3BitvecTest(pDone, pgno as crate::src::ext::rtree::rtree::u32_0) != 0 {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if isMainJrnl != 0 {
        rc = read32bits(jfd, *pOffset - 4 as crate::src::ext::rtree::rtree::i64_0, &raw mut cksum);
        if rc != 0 {
            return rc;
        }
        if isSavepnt == 0 && pager_cksum(pPager, aData as *mut crate::src::ext::rtree::rtree::u8_0) != cksum {
            return crate::src::headers::sqlite3_h::SQLITE_DONE;
        }
    }
    if !pDone.is_null() && {
        rc = crate::src::src::bitvec::sqlite3BitvecSet(pDone, pgno as crate::src::ext::rtree::rtree::u32_0);
        rc != crate::src::headers::sqlite3_h::SQLITE_OK
    } {
        return rc;
    }
    if pgno == 1 as crate::src::src::pager::Pgno
        && __pPager_ref.nReserve as ::core::ffi::c_int
            != *(aData as *mut crate::src::ext::rtree::rtree::u8_0).offset(20 as isize) as ::core::ffi::c_int
    {
        __pPager_ref.nReserve =
            *(aData as *mut crate::src::ext::rtree::rtree::u8_0).offset(20 as isize) as crate::src::fts5::i16_0;
    }
    if !__pPager_ref.pWal.is_null() {
        pPg = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    } else {
        pPg = sqlite3PagerLookup(pPager, pgno) as *mut crate::src::src::pcache::PgHdr;
    }
    if isMainJrnl != 0 {
        isSynced = (__pPager_ref.noSync as ::core::ffi::c_int != 0 || *pOffset <= __pPager_ref.journalHdr)
            as ::core::ffi::c_int;
    } else {
        isSynced = (pPg.is_null()
            || 0 as ::core::ffi::c_int == (*pPg).flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_NEED_SYNC)
            as ::core::ffi::c_int;
    }
    if !(*__pPager_ref.fd).pMethods.is_null()
        && (__pPager_ref.eState as ::core::ffi::c_int >= PAGER_WRITER_DBMOD
            || __pPager_ref.eState as ::core::ffi::c_int == PAGER_OPEN)
        && isSynced != 0
    {
        let mut ofst: crate::src::ext::rtree::rtree::i64_0 = pgno.wrapping_sub(1 as crate::src::src::pager::Pgno) as crate::src::ext::rtree::rtree::i64_0 * __pPager_ref.pageSize;
        rc = crate::src::src::os::sqlite3OsWrite(
            
            __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            aData as *mut crate::src::ext::rtree::rtree::u8_0 as *const ::core::ffi::c_void,
            __pPager_ref.pageSize as ::core::ffi::c_int,
            ofst,
        );
        if pgno > __pPager_ref.dbFileSize {
            __pPager_ref.dbFileSize = pgno;
        }
        if !__pPager_ref.pBackup.is_null() {
            crate::src::src::backup::sqlite3BackupUpdate(__pPager_ref.pBackup, pgno, aData as *mut crate::src::ext::rtree::rtree::u8_0);
        }
    } else if isMainJrnl == 0 && pPg.is_null() {
        __pPager_ref.doNotSpill =
            (__pPager_ref.doNotSpill as ::core::ffi::c_int | SPILLFLAG_ROLLBACK) as crate::src::ext::rtree::rtree::u8_0;
        rc = sqlite3PagerGet(pPager, pgno, &raw mut pPg, 1 as ::core::ffi::c_int);
        __pPager_ref.doNotSpill =
            (__pPager_ref.doNotSpill as ::core::ffi::c_int & !SPILLFLAG_ROLLBACK) as crate::src::ext::rtree::rtree::u8_0;
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        crate::src::src::pcache::sqlite3PcacheMakeDirty(pPg as *mut crate::src::src::pcache::PgHdr);
    }
    if !pPg.is_null() {
        let mut pData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        pData = (*pPg).pData;
        ::libc::memcpy(
            pData,
            aData as *mut crate::src::ext::rtree::rtree::u8_0 as *const ::core::ffi::c_void,
            __pPager_ref.pageSize as crate::__stddef_size_t_h::size_t,
        );
        __pPager_ref.xReiniter.expect("non-null function pointer")(pPg as *mut crate::src::src::pager::DbPage);
        if pgno == 1 as crate::src::src::pager::Pgno {
            ::core::ptr::copy_nonoverlapping(
                    (pData as *mut crate::src::ext::rtree::rtree::u8_0).offset(24 as isize) as *mut crate::src::ext::rtree::rtree::u8_0 as *const u8,
                    &raw mut __pPager_ref.dbFileVers as *mut u8,
                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as usize,
                );
        }
        crate::src::src::pcache::sqlite3PcacheRelease(pPg as *mut crate::src::src::pcache::PgHdr);
    }
    rc
}

unsafe extern "C" fn pager_delsuper(
    mut pPager: *mut Pager,
    mut zSuper: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs = (*pPager).pVfs;
    let mut rc: ::core::ffi::c_int = 0;
    let mut pSuper: *mut crate::src::headers::sqlite3_h::sqlite3_file = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_file>();
    let mut pJournal: *mut crate::src::headers::sqlite3_h::sqlite3_file = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_file>();
    let mut zSuperJournal: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nSuperJournal: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut zJournal: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zSuperPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nSuperPtr: crate::src::ext::rtree::rtree::i64_0 = 0;
    pSuper =
        crate::src::src::malloc::sqlite3MallocZero((2 as crate::src::ext::rtree::rtree::i64_0 * (*pVfs).szOsFile as crate::src::ext::rtree::rtree::i64_0) as crate::src::ext::rtree::rtree::u64_0) as *mut crate::src::headers::sqlite3_h::sqlite3_file;
    if pSuper.is_null() {
        rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        pJournal = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_file>();
    } else {
        let flags: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY | crate::src::headers::sqlite3_h::SQLITE_OPEN_SUPER_JOURNAL;
        rc = crate::src::src::os::sqlite3OsOpen(
            
            pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            zSuper,
            
            pSuper as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            flags,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        pJournal = (pSuper as *mut crate::src::ext::rtree::rtree::u8_0).offset((*pVfs).szOsFile as isize) as *mut crate::src::headers::sqlite3_h::sqlite3_file;
    }
    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
        rc = crate::src::src::os::sqlite3OsFileSize(pSuper as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut nSuperJournal);
        if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
            nSuperPtr = 1 as crate::src::ext::rtree::rtree::i64_0 + (*pVfs).mxPathname as crate::src::ext::rtree::rtree::i64_0;
            zFree = crate::src::src::malloc::sqlite3Malloc((4 as crate::src::ext::rtree::rtree::i64_0 + nSuperJournal + nSuperPtr + 2 as crate::src::ext::rtree::rtree::i64_0) as crate::src::ext::rtree::rtree::u64_0)
                as *mut ::core::ffi::c_char;
            if zFree.is_null() {
                rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
            } else {
                let ref mut fresh0 = *zFree.offset(3 as isize);
                *fresh0 = 0 as ::core::ffi::c_char;
                let ref mut fresh1 = *zFree.offset(2 as isize);
                *fresh1 = *fresh0;
                let ref mut fresh2 = *zFree.offset(1 as isize);
                *fresh2 = *fresh1;
                *zFree.offset(0 as isize) = *fresh2;
                zSuperJournal =
                    zFree.offset(4 as isize) as *mut ::core::ffi::c_char;
                zSuperPtr = zSuperJournal.offset((nSuperJournal + 2 as crate::src::ext::rtree::rtree::i64_0) as isize)
                    as *mut ::core::ffi::c_char;
                rc = crate::src::src::os::sqlite3OsRead(
                    
                    pSuper as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    zSuperJournal as *mut ::core::ffi::c_void,
                    nSuperJournal as ::core::ffi::c_int,
                    0 as crate::src::ext::rtree::rtree::i64_0,
                );
                if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                    *zSuperJournal.offset(nSuperJournal as isize) = 0 as ::core::ffi::c_char;
                    *zSuperJournal.offset((nSuperJournal + 1 as crate::src::ext::rtree::rtree::i64_0) as isize) =
                        0 as ::core::ffi::c_char;
                    zJournal = zSuperJournal;
                    loop {
                        if !((zJournal.offset_from(zSuperJournal) as ::core::ffi::c_long as crate::src::ext::rtree::rtree::i64_0)
                            < nSuperJournal)
                        {
                            current_block = 6450636197030046351;
                            break;
                        }
                        let mut exists: ::core::ffi::c_int = 0;
                        rc = crate::src::src::os::sqlite3OsAccess(pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs, zJournal, crate::src::headers::sqlite3_h::SQLITE_ACCESS_EXISTS, &raw mut exists);
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            current_block = 15518410441251332669;
                            break;
                        }
                        if exists != 0 {
                            let mut c: ::core::ffi::c_int = 0;
                            let mut flags_0: ::core::ffi::c_int =
                                crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY | crate::src::headers::sqlite3_h::SQLITE_OPEN_SUPER_JOURNAL;
                            rc = crate::src::src::os::sqlite3OsOpen(
                                
                                pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                                zJournal,
                                
                                pJournal as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                                flags_0,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            );
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                current_block = 15518410441251332669;
                                break;
                            }
                            rc = readSuperJournal(pJournal, zSuperPtr, nSuperPtr as crate::src::ext::rtree::rtree::u64_0);
                            crate::src::src::os::sqlite3OsClose(pJournal as *mut crate::src::headers::sqlite3_h::sqlite3_file);
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                current_block = 15518410441251332669;
                                break;
                            }
                            c = (*zSuperPtr.offset(0 as isize)
                                as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int
                                && ::libc::strcmp(zSuperPtr, zSuper) == 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int;
                            if c != 0 {
                                current_block = 15518410441251332669;
                                break;
                            }
                        }
                        zJournal = zJournal
                            .offset((crate::src::src::util::sqlite3Strlen30(zJournal) + 1 as ::core::ffi::c_int) as isize);
                    }
                    match current_block {
                        15518410441251332669 => {}
                        _ => {
                            crate::src::src::os::sqlite3OsClose(pSuper as *mut crate::src::headers::sqlite3_h::sqlite3_file);
                            rc = crate::src::src::os::sqlite3OsDelete(pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs, zSuper, 0 as ::core::ffi::c_int);
                        }
                    }
                }
            }
        }
    }
    crate::src::src::malloc::sqlite3_free(zFree as *mut ::core::ffi::c_void);
    if !pSuper.is_null() {
        crate::src::src::os::sqlite3OsClose(pSuper as *mut crate::src::headers::sqlite3_h::sqlite3_file);
        crate::src::src::malloc::sqlite3_free(pSuper as *mut ::core::ffi::c_void);
    }
    rc
}

unsafe extern "C" fn pager_truncate(mut pPager: *mut Pager, mut nPage: crate::src::src::pager::Pgno) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if !(*__pPager_ref.fd).pMethods.is_null()
        && (__pPager_ref.eState as ::core::ffi::c_int >= PAGER_WRITER_DBMOD
            || __pPager_ref.eState as ::core::ffi::c_int == PAGER_OPEN)
    {
        let mut currentSize: crate::src::ext::rtree::rtree::i64_0 = 0;
        let mut newSize: crate::src::ext::rtree::rtree::i64_0 = 0;
        let mut szPage: ::core::ffi::c_int = __pPager_ref.pageSize as ::core::ffi::c_int;
        rc = crate::src::src::os::sqlite3OsFileSize(__pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut currentSize);
        newSize = szPage as crate::src::ext::rtree::rtree::i64_0 * nPage as crate::src::ext::rtree::rtree::i64_0;
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && currentSize != newSize {
            if currentSize > newSize {
                rc = crate::src::src::os::sqlite3OsTruncate(__pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file, newSize);
            } else if currentSize + szPage as crate::src::ext::rtree::rtree::i64_0 <= newSize {
                let mut pTmp: *mut ::core::ffi::c_char = __pPager_ref.pTmpSpace;
                ::libc::memset(
                    pTmp as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    szPage as crate::__stddef_size_t_h::size_t,
                );
                crate::src::src::os::sqlite3OsFileControlHint(
                    
                    __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    crate::src::headers::sqlite3_h::SQLITE_FCNTL_SIZE_HINT_1,
                    &raw mut newSize as *mut ::core::ffi::c_void,
                );
                rc = crate::src::src::os::sqlite3OsWrite(
                    
                    __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    pTmp as *const ::core::ffi::c_void,
                    szPage,
                    newSize - szPage as crate::src::ext::rtree::rtree::i64_0,
                );
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                __pPager_ref.dbFileSize = nPage;
            }
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3SectorSize(mut pFile: *mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int {
    let mut iRet: ::core::ffi::c_int = crate::src::src::os::sqlite3OsSectorSize(pFile as *mut crate::src::headers::sqlite3_h::sqlite3_file);
    if iRet < 32 as ::core::ffi::c_int {
        iRet = 512 as ::core::ffi::c_int;
    } else if iRet > MAX_SECTOR_SIZE {
        iRet = MAX_SECTOR_SIZE;
    }
    iRet
}

unsafe extern "C" fn setSectorSize(mut pPager: *mut Pager) {
    if (*pPager).tempFile as ::core::ffi::c_int != 0
        || crate::src::src::os::sqlite3OsDeviceCharacteristics((*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file) & crate::src::headers::sqlite3_h::SQLITE_IOCAP_POWERSAFE_OVERWRITE
            != 0 as ::core::ffi::c_int
    {
        (*pPager).sectorSize = 512 as crate::src::ext::rtree::rtree::u32_0;
    } else {
        (*pPager).sectorSize = sqlite3SectorSize((*pPager).fd) as crate::src::ext::rtree::rtree::u32_0;
    };
}

unsafe extern "C" fn pager_playback(
    mut pPager: *mut Pager,
    mut isHot: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pPager_ref = unsafe { &mut *pPager };
    let mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs = __pPager_ref.pVfs;
    let mut szJ: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut nRec: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut u: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut mxPg: crate::src::src::pager::Pgno = 0 as crate::src::src::pager::Pgno;
    let mut rc: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut zSuper: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut needPagerReset: ::core::ffi::c_int = 0;
    let mut nPlayback: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut savedPageSize: crate::src::ext::rtree::rtree::u32_0 = __pPager_ref.pageSize as crate::src::ext::rtree::rtree::u32_0;
    rc = crate::src::src::os::sqlite3OsFileSize(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut szJ);
    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
        zSuper = __pPager_ref.pTmpSpace;
        rc = readSuperJournal(
            __pPager_ref.jfd,
            zSuper,
            (1 as crate::src::ext::rtree::rtree::i64_0 + (*__pPager_ref.pVfs).mxPathname as crate::src::ext::rtree::rtree::i64_0) as crate::src::ext::rtree::rtree::u64_0,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && *zSuper.offset(0 as isize) as ::core::ffi::c_int != 0
        {
            rc = crate::src::src::os::sqlite3OsAccess(pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs, zSuper, crate::src::headers::sqlite3_h::SQLITE_ACCESS_EXISTS, &raw mut res);
        }
        zSuper = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK || res == 0) {
            __pPager_ref.journalOff = 0 as crate::src::ext::rtree::rtree::i64_0;
            needPagerReset = isHot;
            's_73: loop {
                rc = readJournalHdr(pPager, isHot, szJ, &raw mut nRec, &raw mut mxPg);
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                    }
                    break;
                } else {
                    if nRec == 0xffffffff as crate::src::ext::rtree::rtree::u32_0 {
                        nRec = ((szJ - __pPager_ref.sectorSize as crate::src::ext::rtree::rtree::i64_0)
                            / (__pPager_ref.pageSize + 8 as crate::src::ext::rtree::rtree::i64_0))
                            as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u32_0;
                    }
                    if nRec == 0 as crate::src::ext::rtree::rtree::u32_0
                        && isHot == 0
                        && __pPager_ref.journalHdr + __pPager_ref.sectorSize as crate::src::ext::rtree::rtree::i64_0
                            == __pPager_ref.journalOff
                    {
                        nRec = ((szJ - __pPager_ref.journalOff) / (__pPager_ref.pageSize + 8 as crate::src::ext::rtree::rtree::i64_0))
                            as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u32_0;
                    }
                    if __pPager_ref.journalOff == __pPager_ref.sectorSize as crate::src::ext::rtree::rtree::i64_0 {
                        rc = pager_truncate(pPager, mxPg);
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            break;
                        }
                        __pPager_ref.dbSize = mxPg;
                        if __pPager_ref.mxPgno < mxPg {
                            __pPager_ref.mxPgno = mxPg;
                        }
                    }
                    u = 0 as crate::src::ext::rtree::rtree::u32_0;
                    while u < nRec {
                        if needPagerReset != 0 {
                            pager_reset(pPager);
                            needPagerReset = 0 as ::core::ffi::c_int;
                        }
                        rc = pager_playback_one_page(
                            pPager,
                            &raw mut __pPager_ref.journalOff,
                            ::core::ptr::null_mut::<crate::src::src::bitvec::Bitvec>(),
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            nPlayback += 1;
                            u = u.wrapping_add(1);
                        } else if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                            __pPager_ref.journalOff = szJ;
                            break;
                        } else {
                            if !(rc == crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1) {
                                break 's_73;
                            }
                            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                            break 's_73;
                        }
                    }
                }
            }
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3PagerSetPagesize(pPager, &raw mut savedPageSize, -(1 as ::core::ffi::c_int));
    }
    __pPager_ref.changeCountDone = __pPager_ref.tempFile;
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        zSuper = __pPager_ref.pTmpSpace.offset(4 as isize)
            as *mut ::core::ffi::c_char;
        rc = readSuperJournal(
            __pPager_ref.jfd,
            zSuper,
            (1 as crate::src::ext::rtree::rtree::i64_0 + (*__pPager_ref.pVfs).mxPathname as crate::src::ext::rtree::rtree::i64_0) as crate::src::ext::rtree::rtree::u64_0,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && (__pPager_ref.eState as ::core::ffi::c_int >= PAGER_WRITER_DBMOD
            || __pPager_ref.eState as ::core::ffi::c_int == PAGER_OPEN)
    {
        rc = sqlite3PagerSync(pPager, ::core::ptr::null::<::core::ffi::c_char>());
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = pager_end_transaction(
            pPager,
            (*zSuper.offset(0 as isize) as ::core::ffi::c_int != '\0' as i32)
                as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && *zSuper.offset(0 as isize) as ::core::ffi::c_int != 0
        && res != 0
    {
        ::libc::memset(
            __pPager_ref.pTmpSpace as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            4 as crate::__stddef_size_t_h::size_t,
        );
        rc = pager_delsuper(pPager, zSuper);
    }
    if isHot != 0 && nPlayback != 0 {
        crate::src::src::printf::sqlite3_log(
            crate::src::headers::sqlite3_h::SQLITE_NOTICE_RECOVER_ROLLBACK_1,
            b"recovered %d pages from %s\0" as *const u8 as *const ::core::ffi::c_char,
            nPlayback,
            __pPager_ref.zJournal,
        );
    }
    setSectorSize(pPager);
    rc
}

unsafe extern "C" fn readDbPage(mut pPg: *mut crate::src::src::pcache::PgHdr) -> ::core::ffi::c_int {
    let mut pPager: *mut Pager = (*pPg).pPager;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut iFrame: crate::src::ext::rtree::rtree::u32_0 = 0 as crate::src::ext::rtree::rtree::u32_0;
    if !(*pPager).pWal.is_null() {
        rc = crate::src::src::wal::sqlite3WalFindFrame((*pPager).pWal, (*pPg).pgno, &raw mut iFrame);
        if rc != 0 {
            return rc;
        }
    }
    if iFrame != 0 {
        rc = crate::src::src::wal::sqlite3WalReadFrame(
            (*pPager).pWal,
            iFrame,
            (*pPager).pageSize as ::core::ffi::c_int,
            (*pPg).pData as *mut crate::src::ext::rtree::rtree::u8_0,
        );
    } else {
        let __pPager_ref = unsafe { &*pPager };
        let mut iOffset: crate::src::ext::rtree::rtree::i64_0 = (*pPg).pgno.wrapping_sub(1 as crate::src::src::pager::Pgno) as crate::src::ext::rtree::rtree::i64_0 * __pPager_ref.pageSize;
        rc = crate::src::src::os::sqlite3OsRead(
            
            __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            (*pPg).pData,
            __pPager_ref.pageSize as ::core::ffi::c_int,
            iOffset,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1 {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    if (*pPg).pgno == 1 as crate::src::src::pager::Pgno {
        if rc != 0 {
            ::libc::memset(
                &raw mut (*pPager).dbFileVers as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                0xff as ::core::ffi::c_int,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as crate::__stddef_size_t_h::size_t,
            );
        } else {
            let mut dbFileVers: *mut crate::src::ext::rtree::rtree::u8_0 =
                ((*pPg).pData as *mut crate::src::ext::rtree::rtree::u8_0).offset(24 as isize) as *mut crate::src::ext::rtree::rtree::u8_0;
            ::core::ptr::copy_nonoverlapping(
                    dbFileVers as *const u8,
                    &raw mut (*pPager).dbFileVers as *mut u8,
                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as usize,
                );
        }
    }
    sqlite3_pager_readdb_count += 1;
    (*pPager).nRead += 1;
    rc
}

unsafe extern "C" fn pager_write_changecounter(mut pPg: *mut crate::src::src::pcache::PgHdr) {
    let mut change_counter: crate::src::ext::rtree::rtree::u32_0 = 0;
    if pPg.is_null() {
        return;
    }
    let __pPg_ref = unsafe { &mut *pPg };
    change_counter = crate::src::src::util::sqlite3Get4byte(
        &raw mut (*__pPg_ref.pPager).dbFileVers as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0,
    )
    .wrapping_add(1 as crate::src::ext::rtree::rtree::u32_0);
    crate::src::src::util::sqlite3Put4byte(
        (__pPg_ref.pData as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0)
            .offset(24 as isize),
        change_counter,
    );
    crate::src::src::util::sqlite3Put4byte(
        (__pPg_ref.pData as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0)
            .offset(92 as isize),
        change_counter,
    );
    crate::src::src::util::sqlite3Put4byte(
        (__pPg_ref.pData as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0)
            .offset(96 as isize),
        3051002 as crate::src::ext::rtree::rtree::u32_0,
    );
}

unsafe extern "C" fn pagerUndoCallback(
    mut pCtx: *mut ::core::ffi::c_void,
    mut iPg: crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pPager: *mut Pager = pCtx as *mut Pager;
    let mut pPg: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    pPg = sqlite3PagerLookup(pPager, iPg) as *mut crate::src::src::pcache::PgHdr;
    if !pPg.is_null() {
        if crate::src::src::pcache::sqlite3PcachePageRefcount(pPg as *mut crate::src::src::pcache::PgHdr) == 1 as crate::src::ext::rtree::rtree::i64_0 {
            crate::src::src::pcache::sqlite3PcacheDrop(pPg as *mut crate::src::src::pcache::PgHdr);
        } else {
            rc = readDbPage(pPg);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                (*pPager).xReiniter.expect("non-null function pointer")(pPg as *mut crate::src::src::pager::DbPage);
            }
            sqlite3PagerUnrefNotNull(pPg as *mut crate::src::src::pager::DbPage);
        }
    }
    crate::src::src::backup::sqlite3BackupRestart((*pPager).pBackup);
    rc
}

unsafe extern "C" fn pagerRollbackWal(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pList: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let __pPager_ref = unsafe { &mut *pPager };
    __pPager_ref.dbSize = __pPager_ref.dbOrigSize;
    rc = crate::src::src::wal::sqlite3WalUndo(
        __pPager_ref.pWal,
        Some(
            pagerUndoCallback
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::src::src::pager::Pgno) -> ::core::ffi::c_int,
        ),
        pPager as *mut ::core::ffi::c_void,
    );
    pList =  crate::src::src::pcache::sqlite3PcacheDirtyList(__pPager_ref.pPCache) as
    *mut crate::src::src::pcache::PgHdr;
    while !pList.is_null() && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut pNext: *mut crate::src::src::pcache::PgHdr = (*pList).pDirty;
        rc = pagerUndoCallback(pPager as *mut ::core::ffi::c_void, (*pList).pgno);
        pList = pNext;
    }
    rc
}

unsafe extern "C" fn pagerWalFrames(
    mut pPager: *mut Pager,
    mut pList: *mut crate::src::src::pcache::PgHdr,
    mut nTruncate: crate::src::src::pager::Pgno,
    mut isCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut nList: ::core::ffi::c_int = 0;
    let mut p: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    if isCommit != 0 {
        let mut ppNext: *mut *mut crate::src::src::pcache::PgHdr = &raw mut pList;
        nList = 0 as ::core::ffi::c_int;
        p = pList;
        loop {
            *ppNext = p;
            if (*ppNext).is_null() {
                break;
            }
            if (*p).pgno <= nTruncate {
                ppNext = &raw mut (*p).pDirty;
                nList += 1;
            }
            p = (*p).pDirty;
        }
    } else {
        nList = 1 as ::core::ffi::c_int;
    }
    let __pPager_ref = unsafe { &mut *pPager };
    __pPager_ref.aStat[PAGER_STAT_WRITE as usize] =
        __pPager_ref.aStat[PAGER_STAT_WRITE as usize].wrapping_add(nList as crate::src::ext::rtree::rtree::u32_0);
    if (*pList).pgno == 1 as crate::src::src::pager::Pgno {
        pager_write_changecounter(pList);
    }
    rc = crate::src::src::wal::sqlite3WalFrames(
        __pPager_ref.pWal,
        __pPager_ref.pageSize as ::core::ffi::c_int,
        
        pList as *mut crate::src::src::pcache::PgHdr,
        nTruncate,
        isCommit,
        __pPager_ref.walSyncFlags as ::core::ffi::c_int,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !__pPager_ref.pBackup.is_null() {
        p = pList;
        while !p.is_null() {
            crate::src::src::backup::sqlite3BackupUpdate(__pPager_ref.pBackup, (*p).pgno, (*p).pData as *mut crate::src::ext::rtree::rtree::u8_0);
            p = (*p).pDirty;
        }
    }
    rc
}

unsafe extern "C" fn pagerBeginReadTransaction(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut changed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    crate::src::src::wal::sqlite3WalEndReadTransaction((*pPager).pWal);
    rc = crate::src::src::wal::sqlite3WalBeginReadTransaction((*pPager).pWal, &raw mut changed);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK || changed != 0 {
        pager_reset(pPager);
        if (*pPager).bUseFetch != 0 {
            crate::src::src::os::sqlite3OsUnfetch(
                
                (*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                0 as crate::src::ext::rtree::rtree::i64_0,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
    }
    rc
}

unsafe extern "C" fn pagerPagecount(
    mut pPager: *mut Pager,
    mut pnPage: *mut crate::src::src::pager::Pgno,
) -> ::core::ffi::c_int {
    let mut nPage: crate::src::src::pager::Pgno = 0;
    let __pPager_ref = unsafe { &mut *pPager };
    nPage = crate::src::src::wal::sqlite3WalDbsize(__pPager_ref.pWal);
    if nPage == 0 as crate::src::src::pager::Pgno && !(*__pPager_ref.fd).pMethods.is_null() {
        let mut n: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
        let mut rc: ::core::ffi::c_int = crate::src::src::os::sqlite3OsFileSize(__pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut n);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        nPage = ((n + __pPager_ref.pageSize - 1 as crate::src::ext::rtree::rtree::i64_0) / __pPager_ref.pageSize) as crate::src::src::pager::Pgno;
    }
    if nPage > __pPager_ref.mxPgno {
        __pPager_ref.mxPgno = nPage;
    }
    *pnPage = nPage;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn pagerOpenWalIfPresent(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pPager).tempFile == 0 {
        let mut isWal: ::core::ffi::c_int = 0;
        rc = crate::src::src::os::sqlite3OsAccess(
            
            (*pPager).pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            (*pPager).zWal,
            crate::src::headers::sqlite3_h::SQLITE_ACCESS_EXISTS,
            &raw mut isWal,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if isWal != 0 {
                let mut nPage: crate::src::src::pager::Pgno = 0;
                rc = pagerPagecount(pPager, &raw mut nPage);
                if rc != 0 {
                    return rc;
                }
                if nPage == 0 as crate::src::src::pager::Pgno {
                    rc = crate::src::src::os::sqlite3OsDelete((*pPager).pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs, (*pPager).zWal, 0 as ::core::ffi::c_int);
                } else {
                    rc = sqlite3PagerOpenWal(pPager, ::core::ptr::null_mut::<::core::ffi::c_int>());
                }
            } else if (*pPager).journalMode as ::core::ffi::c_int == crate::src::src::pager::PAGER_JOURNALMODE_WAL {
                (*pPager).journalMode = crate::src::src::pager::PAGER_JOURNALMODE_DELETE as crate::src::ext::rtree::rtree::u8_0;
            }
        }
    }
    rc
}

unsafe extern "C" fn pagerPlaybackSavepoint(
    mut pPager: *mut Pager,
    mut pSavepoint: *mut PagerSavepoint,
) -> ::core::ffi::c_int {
    let mut szJ: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut iHdrOff: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pDone: *mut crate::src::src::bitvec::Bitvec = ::core::ptr::null_mut::<crate::src::src::bitvec::Bitvec>();
    if !pSavepoint.is_null() {
        pDone = crate::src::src::bitvec::sqlite3BitvecCreate((*pSavepoint).nOrig as crate::src::ext::rtree::rtree::u32_0);
        if pDone.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
    }
    let __pPager_ref = unsafe { &mut *pPager };
    __pPager_ref.dbSize = if !pSavepoint.is_null() {
        (*pSavepoint).nOrig
    } else {
        __pPager_ref.dbOrigSize
    };
    __pPager_ref.changeCountDone = __pPager_ref.tempFile;
    if pSavepoint.is_null() && !__pPager_ref.pWal.is_null() {
        return pagerRollbackWal(pPager);
    }
    szJ = __pPager_ref.journalOff;
    if !pSavepoint.is_null() && __pPager_ref.pWal.is_null() {
        iHdrOff = if (*pSavepoint).iHdrOffset != 0 {
            (*pSavepoint).iHdrOffset
        } else {
            szJ
        };
        __pPager_ref.journalOff = (*pSavepoint).iOffset;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && __pPager_ref.journalOff < iHdrOff {
            rc = pager_playback_one_page(
                pPager,
                &raw mut __pPager_ref.journalOff,
                pDone,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
        }
    } else {
        __pPager_ref.journalOff = 0 as crate::src::ext::rtree::rtree::i64_0;
    }
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && __pPager_ref.journalOff < szJ {
        let mut ii: crate::src::ext::rtree::rtree::u32_0 = 0;
        let mut nJRec: crate::src::ext::rtree::rtree::u32_0 = 0 as crate::src::ext::rtree::rtree::u32_0;
        let mut dummy: crate::src::ext::rtree::rtree::u32_0 = 0;
        rc = readJournalHdr(
            pPager,
            0 as ::core::ffi::c_int,
            szJ,
            &raw mut nJRec,
            &raw mut dummy,
        );
        if nJRec == 0 as crate::src::ext::rtree::rtree::u32_0
            && __pPager_ref.journalHdr + __pPager_ref.sectorSize as crate::src::ext::rtree::rtree::i64_0 == __pPager_ref.journalOff
        {
            nJRec = ((szJ - __pPager_ref.journalOff) / (__pPager_ref.pageSize + 8 as crate::src::ext::rtree::rtree::i64_0)) as crate::src::ext::rtree::rtree::u32_0;
        }
        ii = 0 as crate::src::ext::rtree::rtree::u32_0;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && ii < nJRec && __pPager_ref.journalOff < szJ {
            rc = pager_playback_one_page(
                pPager,
                &raw mut __pPager_ref.journalOff,
                pDone,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            ii = ii.wrapping_add(1);
        }
    }
    if !pSavepoint.is_null() {
        let mut ii_0: crate::src::ext::rtree::rtree::u32_0 = 0;
        let mut offset: crate::src::ext::rtree::rtree::i64_0 = (*pSavepoint).iSubRec as crate::src::ext::rtree::rtree::i64_0 * (4 as crate::src::ext::rtree::rtree::i64_0 + __pPager_ref.pageSize);
        if !__pPager_ref.pWal.is_null() {
            rc = crate::src::src::wal::sqlite3WalSavepointUndo(
                __pPager_ref.pWal,
                &raw mut (*pSavepoint).aWalData as *mut crate::src::ext::rtree::rtree::u32_0,
            );
        }
        ii_0 = (*pSavepoint).iSubRec as crate::src::ext::rtree::rtree::u32_0;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && ii_0 < __pPager_ref.nSubRec {
            rc = pager_playback_one_page(
                pPager,
                &raw mut offset,
                pDone,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            ii_0 = ii_0.wrapping_add(1);
        }
    }
    crate::src::src::bitvec::sqlite3BitvecDestroy(pDone);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        __pPager_ref.journalOff = szJ;
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerSetCachesize(
    mut pPager: *mut Pager,
    mut mxPage: ::core::ffi::c_int,
) {
    crate::src::src::pcache::sqlite3PcacheSetCachesize((*pPager).pPCache, mxPage);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerSetSpillsize(
    mut pPager: *mut Pager,
    mut mxPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    crate::src::src::pcache::sqlite3PcacheSetSpillsize((*pPager).pPCache, mxPage)
}

unsafe extern "C" fn pagerFixMaplimit(mut pPager: *mut Pager) {
    let mut fd: *mut crate::src::headers::sqlite3_h::sqlite3_file = (*pPager).fd;
    if !(*fd).pMethods.is_null() && (*(*fd).pMethods).iVersion >= 3 as ::core::ffi::c_int {
        let mut sz: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
        let __pPager_ref = unsafe { &mut *pPager };
        sz = __pPager_ref.szMmap;
        __pPager_ref.bUseFetch = (sz > 0 as crate::src::headers::sqlite3_h::sqlite3_int64) as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0;
        setGetterMethod(pPager);
        crate::src::src::os::sqlite3OsFileControlHint(
            
            __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            crate::src::headers::sqlite3_h::SQLITE_FCNTL_MMAP_SIZE_1,
            &raw mut sz as *mut ::core::ffi::c_void,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerSetMmapLimit(
    mut pPager: *mut Pager,
    mut szMmap: crate::src::headers::sqlite3_h::sqlite3_int64,
) {
    (*pPager).szMmap = szMmap;
    pagerFixMaplimit(pPager);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerShrink(mut pPager: *mut Pager) {
    crate::src::src::pcache::sqlite3PcacheShrink((*pPager).pPCache);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerSetFlags(
    mut pPager: *mut Pager,
    mut pgFlags: ::core::ffi::c_uint,
) {
    let mut level: ::core::ffi::c_uint = pgFlags & crate::src::src::pager::PAGER_SYNCHRONOUS_MASK as ::core::ffi::c_uint;
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.tempFile as ::core::ffi::c_int != 0
        || level == crate::src::src::pager::PAGER_SYNCHRONOUS_OFF as ::core::ffi::c_uint
    {
        __pPager_ref.noSync = 1 as crate::src::ext::rtree::rtree::u8_0;
        __pPager_ref.fullSync = 0 as crate::src::ext::rtree::rtree::u8_0;
        __pPager_ref.extraSync = 0 as crate::src::ext::rtree::rtree::u8_0;
    } else {
        __pPager_ref.noSync = 0 as crate::src::ext::rtree::rtree::u8_0;
        __pPager_ref.fullSync = (if level >= crate::src::src::pager::PAGER_SYNCHRONOUS_FULL as ::core::ffi::c_uint {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as crate::src::ext::rtree::rtree::u8_0;
        if level == crate::src::src::pager::PAGER_SYNCHRONOUS_EXTRA as ::core::ffi::c_uint {
            __pPager_ref.extraSync = 1 as crate::src::ext::rtree::rtree::u8_0;
        } else {
            __pPager_ref.extraSync = 0 as crate::src::ext::rtree::rtree::u8_0;
        }
    }
    if __pPager_ref.noSync != 0 {
        __pPager_ref.syncFlags = 0 as crate::src::ext::rtree::rtree::u8_0;
    } else if pgFlags & crate::src::src::pager::PAGER_FULLFSYNC as ::core::ffi::c_uint != 0 {
        __pPager_ref.syncFlags = crate::src::headers::sqlite3_h::SQLITE_SYNC_FULL as crate::src::ext::rtree::rtree::u8_0;
    } else {
        __pPager_ref.syncFlags = crate::src::headers::sqlite3_h::SQLITE_SYNC_NORMAL as crate::src::ext::rtree::rtree::u8_0;
    }
    __pPager_ref.walSyncFlags =
        ((__pPager_ref.syncFlags as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::u8_0;
    if __pPager_ref.fullSync != 0 {
        __pPager_ref.walSyncFlags = (__pPager_ref.walSyncFlags as ::core::ffi::c_int
            | __pPager_ref.syncFlags as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::u8_0;
    }
    if pgFlags & crate::src::src::pager::PAGER_CKPT_FULLFSYNC as ::core::ffi::c_uint != 0 && __pPager_ref.noSync == 0 {
        __pPager_ref.walSyncFlags = (__pPager_ref.walSyncFlags as ::core::ffi::c_int
            | crate::src::headers::sqlite3_h::SQLITE_SYNC_FULL << 2 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::u8_0;
    }
    if pgFlags & crate::src::src::pager::PAGER_CACHESPILL as ::core::ffi::c_uint != 0 {
        __pPager_ref.doNotSpill =
            (__pPager_ref.doNotSpill as ::core::ffi::c_int & !SPILLFLAG_OFF) as crate::src::ext::rtree::rtree::u8_0;
    } else {
        __pPager_ref.doNotSpill = (__pPager_ref.doNotSpill as ::core::ffi::c_int | SPILLFLAG_OFF) as crate::src::ext::rtree::rtree::u8_0;
    };
}
#[no_mangle]

pub static mut sqlite3_opentemp_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

unsafe extern "C" fn pagerOpentemp(
    mut pPager: *mut Pager,
    mut pFile: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut vfsFlags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    sqlite3_opentemp_count += 1;
    vfsFlags |= crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE
        | crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE
        | crate::src::headers::sqlite3_h::SQLITE_OPEN_EXCLUSIVE
        | crate::src::headers::sqlite3_h::SQLITE_OPEN_DELETEONCLOSE;
    rc = crate::src::src::os::sqlite3OsOpen(
        
        (*pPager).pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
        ::core::ptr::null::<::core::ffi::c_char>(),
        
        pFile as *mut crate::src::headers::sqlite3_h::sqlite3_file,
        vfsFlags,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerSetBusyHandler(
    mut pPager: *mut Pager,
    mut xBusyHandler: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    mut pBusyHandlerArg: *mut ::core::ffi::c_void,
) {
    let mut ap: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let __pPager_ref = unsafe { &mut *pPager };
    __pPager_ref.xBusyHandler = xBusyHandler;
    __pPager_ref.pBusyHandlerArg = pBusyHandlerArg;
    ap = &raw mut __pPager_ref.xBusyHandler as *mut *mut ::core::ffi::c_void;
    crate::src::src::os::sqlite3OsFileControlHint(
        
        __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
        crate::src::headers::sqlite3_h::SQLITE_FCNTL_BUSYHANDLER,
        ap as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerSetPagesize(
    mut pPager: *mut Pager,
    mut pPageSize: *mut crate::src::ext::rtree::rtree::u32_0,
    mut nReserve: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pageSize: crate::src::ext::rtree::rtree::u32_0 = *pPageSize;
    let __pPager_ref = unsafe { &mut *pPager };
    if (__pPager_ref.memDb as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || __pPager_ref.dbSize == 0 as crate::src::src::pager::Pgno)
        && crate::src::src::pcache::sqlite3PcacheRefCount(__pPager_ref.pPCache) == 0 as crate::src::ext::rtree::rtree::i64_0
        && pageSize != 0
        && pageSize != __pPager_ref.pageSize as crate::src::ext::rtree::rtree::u32_0
    {
        let mut pNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nByte: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
        if __pPager_ref.eState as ::core::ffi::c_int > PAGER_OPEN
            && !(*__pPager_ref.fd).pMethods.is_null()
        {
            rc = crate::src::src::os::sqlite3OsFileSize(__pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut nByte);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            pNew = crate::src::src::pcache1::sqlite3PageMalloc(pageSize.wrapping_add(8 as crate::src::ext::rtree::rtree::u32_0) as ::core::ffi::c_int)
                as *mut ::core::ffi::c_char;
            if pNew.is_null() {
                rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
            } else {
                ::libc::memset(
                    pNew.offset(pageSize as isize) as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    8 as crate::__stddef_size_t_h::size_t,
                );
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            pager_reset(pPager);
            rc = crate::src::src::pcache::sqlite3PcacheSetPageSize(__pPager_ref.pPCache, pageSize as ::core::ffi::c_int);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::pcache1::sqlite3PageFree(__pPager_ref.pTmpSpace as *mut ::core::ffi::c_void);
            __pPager_ref.pTmpSpace = pNew;
            __pPager_ref.dbSize =
                ((nByte + pageSize as crate::src::ext::rtree::rtree::i64_0 - 1 as crate::src::ext::rtree::rtree::i64_0) / pageSize as crate::src::ext::rtree::rtree::i64_0) as crate::src::src::pager::Pgno;
            __pPager_ref.pageSize = pageSize as crate::src::ext::rtree::rtree::i64_0;
            __pPager_ref.lckPgno = (crate::src::src::global::sqlite3PendingByte as crate::src::ext::rtree::rtree::u32_0)
                .wrapping_div(pageSize)
                .wrapping_add(1 as crate::src::src::pager::Pgno);
        } else {
            crate::src::src::pcache1::sqlite3PageFree(pNew as *mut ::core::ffi::c_void);
        }
    }
    *pPageSize = __pPager_ref.pageSize as crate::src::ext::rtree::rtree::u32_0;
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if nReserve < 0 as ::core::ffi::c_int {
            nReserve = __pPager_ref.nReserve as ::core::ffi::c_int;
        }
        __pPager_ref.nReserve = nReserve as crate::src::fts5::i16_0;
        pagerFixMaplimit(pPager);
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerTempSpace(mut pPager: *mut Pager) -> *mut ::core::ffi::c_void {
    (*pPager).pTmpSpace as *mut ::core::ffi::c_void
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerMaxPageCount(
    mut pPager: *mut Pager,
    mut mxPage: crate::src::src::pager::Pgno,
) -> crate::src::src::pager::Pgno {
    if mxPage > 0 as crate::src::src::pager::Pgno {
        (*pPager).mxPgno = mxPage;
    }
    (*pPager).mxPgno
}

static mut saved_cnt: ::core::ffi::c_int = 0;
#[no_mangle]

pub unsafe extern "C" fn disable_simulated_io_errors() {
    saved_cnt = sqlite3_io_error_pending;
    sqlite3_io_error_pending = -(1 as ::core::ffi::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn enable_simulated_io_errors() {
    sqlite3_io_error_pending = saved_cnt;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerReadFileheader(
    mut pPager: *mut Pager,
    mut N: ::core::ffi::c_int,
    mut pDest: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    ::libc::memset(
        pDest as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        N as crate::__stddef_size_t_h::size_t,
    );
    if !(*(*pPager).fd).pMethods.is_null() {
        rc = crate::src::src::os::sqlite3OsRead(
            
            (*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            pDest as *mut ::core::ffi::c_void,
            N,
            0 as crate::src::ext::rtree::rtree::i64_0,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1 {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerPagecount(
    mut pPager: *mut Pager,
    mut pnPage: *mut ::core::ffi::c_int,
) {
    *pnPage = (*pPager).dbSize as ::core::ffi::c_int;
}

unsafe extern "C" fn pager_wait_on_lock(
    mut pPager: *mut Pager,
    mut locktype: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    loop {
        rc = pagerLockDb(pPager, locktype);
        if !(rc == crate::src::headers::sqlite3_h::SQLITE_BUSY
            && (*pPager).xBusyHandler.expect("non-null function pointer")(
                (*pPager).pBusyHandlerArg,
            ) != 0)
        {
            break;
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerTruncateImage(mut pPager: *mut Pager, mut nPage: crate::src::src::pager::Pgno) {
    (*pPager).dbSize = nPage;
}

unsafe extern "C" fn pagerSyncHotJournal(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pPager).noSync == 0 {
        rc = crate::src::src::os::sqlite3OsSync((*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, crate::src::headers::sqlite3_h::SQLITE_SYNC_NORMAL);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::src::os::sqlite3OsFileSize((*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut (*pPager).journalHdr);
    }
    rc
}

unsafe extern "C" fn pagerAcquireMapPage(
    mut pPager: *mut Pager,
    mut pgno: crate::src::src::pager::Pgno,
    mut pData: *mut ::core::ffi::c_void,
    mut ppPage: *mut *mut crate::src::src::pcache::PgHdr,
) -> ::core::ffi::c_int {
    let mut p: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    if !(*pPager).pMmapFreelist.is_null() {
        p = (*pPager).pMmapFreelist;
        *ppPage = p;
        (*pPager).pMmapFreelist = (*p).pDirty;
        (*p).pDirty = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
        ::libc::memset((*p).pExtra, 0 as ::core::ffi::c_int, 8 as crate::__stddef_size_t_h::size_t);
    } else {
        p = crate::src::src::malloc::sqlite3MallocZero(
            (::core::mem::size_of::<crate::src::src::pcache::PgHdr>() as usize).wrapping_add((*pPager).nExtra as usize)
                as crate::src::ext::rtree::rtree::u64_0,
        ) as *mut crate::src::src::pcache::PgHdr;
        *ppPage = p;
        if p.is_null() {
            crate::src::src::os::sqlite3OsUnfetch(
                
                (*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                pgno.wrapping_sub(1 as crate::src::src::pager::Pgno) as crate::src::ext::rtree::rtree::i64_0 * (*pPager).pageSize,
                pData,
            );
            return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
        (*p).pExtra =
            p.offset(1 as isize) as *mut crate::src::src::pcache::PgHdr as *mut ::core::ffi::c_void;
        (*p).flags = crate::src::src::pcache::PGHDR_MMAP as crate::src::fts5::u16_0;
        (*p).nRef = 1 as crate::src::ext::rtree::rtree::i64_0;
        (*p).pPager = pPager;
    }
    (*p).pgno = pgno;
    (*p).pData = pData;
    (*pPager).nMmapOut += 1;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn pagerReleaseMapPage(mut pPg: *mut crate::src::src::pcache::PgHdr) {
    let __pPg_ref = unsafe { &mut *pPg };
    let mut pPager: *mut Pager = __pPg_ref.pPager;
    let __pPager_ref = unsafe { &mut *pPager };
    __pPager_ref.nMmapOut -= 1;
    __pPg_ref.pDirty = __pPager_ref.pMmapFreelist;
    __pPager_ref.pMmapFreelist = pPg;
    crate::src::src::os::sqlite3OsUnfetch(
        
        __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
        __pPg_ref.pgno.wrapping_sub(1 as crate::src::src::pager::Pgno) as crate::src::ext::rtree::rtree::i64_0 * __pPager_ref.pageSize,
        __pPg_ref.pData,
    );
}

unsafe extern "C" fn pagerFreeMapHdrs(mut pPager: *mut Pager) {
    let mut p: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let mut pNext: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    p = (*pPager).pMmapFreelist;
    while !p.is_null() {
        pNext = (*p).pDirty;
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        p = pNext;
    }
}

unsafe extern "C" fn databaseIsUnmoved(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut bHasMoved: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = 0;
    let __pPager_ref = unsafe { &*pPager };
    if __pPager_ref.tempFile != 0 {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if __pPager_ref.dbSize == 0 as crate::src::src::pager::Pgno {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc = crate::src::src::os::sqlite3OsFileControl(
        
        __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
        crate::src::headers::sqlite3_h::SQLITE_FCNTL_HAS_MOVED_1,
        &raw mut bHasMoved as *mut ::core::ffi::c_void,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_NOTFOUND {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    } else if rc == crate::src::headers::sqlite3_h::SQLITE_OK && bHasMoved != 0 {
        rc = crate::src::headers::sqlite3_h::SQLITE_READONLY_DBMOVED_1;
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerClose(
    mut pPager: *mut Pager,
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) -> ::core::ffi::c_int {
    let __pPager_ref = unsafe { &mut *pPager };
    let mut pTmp: *mut crate::src::ext::rtree::rtree::u8_0 = __pPager_ref.pTmpSpace as *mut crate::src::ext::rtree::rtree::u8_0;
    disable_simulated_io_errors();
    crate::src::src::fault::sqlite3BeginBenignMalloc();
    pagerFreeMapHdrs(pPager);
    __pPager_ref.exclusiveMode = 0 as crate::src::ext::rtree::rtree::u8_0;
    let mut a: *mut crate::src::ext::rtree::rtree::u8_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u8_0>();
    if !db.is_null()
        && 0 as crate::src::ext::rtree::rtree::u64_0 == (*db).flags & crate::src::headers::sqliteInt_h::SQLITE_NoCkptOnClose as crate::src::ext::rtree::rtree::u64_0
        && crate::src::headers::sqlite3_h::SQLITE_OK == databaseIsUnmoved(pPager)
    {
        a = pTmp;
    }
    crate::src::src::wal::sqlite3WalClose(
        __pPager_ref.pWal,
        
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pPager_ref.walSyncFlags as ::core::ffi::c_int,
        __pPager_ref.pageSize as ::core::ffi::c_int,
        a,
    );
    __pPager_ref.pWal = ::core::ptr::null_mut::<crate::src::src::wal::Wal>();
    pager_reset(pPager);
    if __pPager_ref.memDb != 0 {
        pager_unlock(pPager);
    } else {
        if !(*__pPager_ref.jfd).pMethods.is_null() {
            pager_error(pPager, pagerSyncHotJournal(pPager));
        }
        pagerUnlockAndRollback(pPager);
    }
    crate::src::src::fault::sqlite3EndBenignMalloc();
    enable_simulated_io_errors();
    crate::src::src::os::sqlite3OsClose(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
    crate::src::src::os::sqlite3OsClose(__pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
    crate::src::src::pcache1::sqlite3PageFree(pTmp as *mut ::core::ffi::c_void);
    crate::src::src::pcache::sqlite3PcacheClose(__pPager_ref.pPCache);
    crate::src::src::malloc::sqlite3_free(pPager as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerPagenumber(mut pPg: *mut crate::src::src::pager::DbPage) -> crate::src::src::pager::Pgno {
    (*pPg).pgno
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerRef(mut pPg: *mut crate::src::src::pager::DbPage) {
    crate::src::src::pcache::sqlite3PcacheRef(pPg as *mut crate::src::src::pcache::PgHdr as *mut crate::src::src::pcache::PgHdr);
}

unsafe extern "C" fn syncJournal(
    mut pPager: *mut Pager,
    mut newHdr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    rc = sqlite3PagerExclusiveLock(pPager);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.noSync == 0 {
        if !(*__pPager_ref.jfd).pMethods.is_null()
            && __pPager_ref.journalMode as ::core::ffi::c_int != crate::src::src::pager::PAGER_JOURNALMODE_MEMORY
        {
            let iDc: ::core::ffi::c_int =
                crate::src::src::os::sqlite3OsDeviceCharacteristics(__pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file) as ::core::ffi::c_int;
            if 0 as ::core::ffi::c_int == iDc & crate::src::headers::sqlite3_h::SQLITE_IOCAP_SAFE_APPEND {
                let mut iNextHdrOffset: crate::src::ext::rtree::rtree::i64_0 = 0;
                let mut aMagic: [crate::src::ext::rtree::rtree::u8_0; 8] = [0; 8];
                let mut zHeader: [crate::src::ext::rtree::rtree::u8_0; 12] = [0; 12];
                ::core::ptr::copy_nonoverlapping(
                    &raw const aJournalMagic as *const ::core::ffi::c_uchar as *const u8,
                    &raw mut zHeader as *mut crate::src::ext::rtree::rtree::u8_0 as *mut u8,
                    ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize,
                );
                crate::src::src::util::sqlite3Put4byte(
                    (&raw mut zHeader as *mut crate::src::ext::rtree::rtree::u8_0)
                        .offset(::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as isize)
                        as *mut crate::src::ext::rtree::rtree::u8_0,
                    __pPager_ref.nRec as crate::src::ext::rtree::rtree::u32_0,
                );
                iNextHdrOffset = journalHdrOffset(pPager);
                rc = crate::src::src::os::sqlite3OsRead(
                    
                    __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    &raw mut aMagic as *mut crate::src::ext::rtree::rtree::u8_0 as *mut ::core::ffi::c_void,
                    8 as ::core::ffi::c_int,
                    iNextHdrOffset,
                );
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    && 0 as ::core::ffi::c_int
                        == ::libc::memcmp(
                            &raw mut aMagic as *mut crate::src::ext::rtree::rtree::u8_0 as *const ::core::ffi::c_void,
                            &raw const aJournalMagic as *const ::core::ffi::c_uchar
                                as *const ::core::ffi::c_void,
                            8 as crate::__stddef_size_t_h::size_t,
                        )
                {
                    static mut zerobyte: crate::src::ext::rtree::rtree::u8_0 = 0 as crate::src::ext::rtree::rtree::u8_0;
                    rc = crate::src::src::os::sqlite3OsWrite(
                        
                        __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                        &raw const zerobyte as *const ::core::ffi::c_void,
                        1 as ::core::ffi::c_int,
                        iNextHdrOffset,
                    );
                }
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK && rc != crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1 {
                    return rc;
                }
                if __pPager_ref.fullSync as ::core::ffi::c_int != 0
                    && 0 as ::core::ffi::c_int == iDc & crate::src::headers::sqlite3_h::SQLITE_IOCAP_SEQUENTIAL
                {
                    rc = crate::src::src::os::sqlite3OsSync(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, __pPager_ref.syncFlags as ::core::ffi::c_int);
                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                        return rc;
                    }
                }
                rc = crate::src::src::os::sqlite3OsWrite(
                    
                    __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    &raw mut zHeader as *mut crate::src::ext::rtree::rtree::u8_0 as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[crate::src::ext::rtree::rtree::u8_0; 12]>() as ::core::ffi::c_int,
                    __pPager_ref.journalHdr,
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    return rc;
                }
            }
            if 0 as ::core::ffi::c_int == iDc & crate::src::headers::sqlite3_h::SQLITE_IOCAP_SEQUENTIAL {
                rc = crate::src::src::os::sqlite3OsSync(
                    
                    __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    __pPager_ref.syncFlags as ::core::ffi::c_int
                        | (if __pPager_ref.syncFlags as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_SYNC_FULL {
                            crate::src::headers::sqlite3_h::SQLITE_SYNC_DATAONLY
                        } else {
                            0 as ::core::ffi::c_int
                        }),
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    return rc;
                }
            }
            __pPager_ref.journalHdr = __pPager_ref.journalOff;
            if newHdr != 0 && 0 as ::core::ffi::c_int == iDc & crate::src::headers::sqlite3_h::SQLITE_IOCAP_SAFE_APPEND {
                __pPager_ref.nRec = 0 as ::core::ffi::c_int;
                rc = writeJournalHdr(pPager);
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    return rc;
                }
            }
        } else {
            __pPager_ref.journalHdr = __pPager_ref.journalOff;
        }
    }
    crate::src::src::pcache::sqlite3PcacheClearSyncFlags(__pPager_ref.pPCache);
    __pPager_ref.eState = PAGER_WRITER_DBMOD as crate::src::ext::rtree::rtree::u8_0;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn pager_write_pagelist(
    mut pPager: *mut Pager,
    mut pList: *mut crate::src::src::pcache::PgHdr,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if (*__pPager_ref.fd).pMethods.is_null() {
        rc = pagerOpentemp(
            pPager,
            __pPager_ref.fd,
            __pPager_ref.vfsFlags as ::core::ffi::c_int,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && __pPager_ref.dbHintSize < __pPager_ref.dbSize
        && (!(*pList).pDirty.is_null() || (*pList).pgno > __pPager_ref.dbHintSize)
    {
        let mut szFile: crate::src::headers::sqlite3_h::sqlite3_int64 =
            __pPager_ref.pageSize as crate::src::headers::sqlite3_h::sqlite3_int64 * __pPager_ref.dbSize as crate::src::headers::sqlite3_h::sqlite3_int64;
        crate::src::src::os::sqlite3OsFileControlHint(
            
            __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            crate::src::headers::sqlite3_h::SQLITE_FCNTL_SIZE_HINT_1,
            &raw mut szFile as *mut ::core::ffi::c_void,
        );
        __pPager_ref.dbHintSize = __pPager_ref.dbSize;
    }
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pList.is_null() {
        let mut pgno: crate::src::src::pager::Pgno = (*pList).pgno;
        if pgno <= __pPager_ref.dbSize
            && 0 as ::core::ffi::c_int == (*pList).flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_DONT_WRITE
        {
            let mut offset: crate::src::ext::rtree::rtree::i64_0 = pgno.wrapping_sub(1 as crate::src::src::pager::Pgno) as crate::src::ext::rtree::rtree::i64_0 * __pPager_ref.pageSize;
            let mut pData: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let __pList_ref = unsafe { &*pList };
            if __pList_ref.pgno == 1 as crate::src::src::pager::Pgno {
                pager_write_changecounter(pList);
            }
            pData = __pList_ref.pData as *mut ::core::ffi::c_char;
            rc = crate::src::src::os::sqlite3OsWrite(
                
                __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                pData as *const ::core::ffi::c_void,
                __pPager_ref.pageSize as ::core::ffi::c_int,
                offset,
            );
            if pgno == 1 as crate::src::src::pager::Pgno {
                ::core::ptr::copy_nonoverlapping(
                    pData.offset(24 as isize) as *mut ::core::ffi::c_char as *const u8,
                    &raw mut __pPager_ref.dbFileVers as *mut u8,
                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as usize,
                );
            }
            if pgno > __pPager_ref.dbFileSize {
                __pPager_ref.dbFileSize = pgno;
            }
            __pPager_ref.aStat[PAGER_STAT_WRITE as usize] =
                __pPager_ref.aStat[PAGER_STAT_WRITE as usize].wrapping_add(1);
            crate::src::src::backup::sqlite3BackupUpdate(__pPager_ref.pBackup, pgno, __pList_ref.pData as *mut crate::src::ext::rtree::rtree::u8_0);
            sqlite3_pager_writedb_count += 1;
        }
        pList = (*pList).pDirty;
    }
    rc
}

unsafe extern "C" fn openSubJournal(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*(*pPager).sjfd).pMethods.is_null() {
        let flags: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OPEN_SUBJOURNAL
            | crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE
            | crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE
            | crate::src::headers::sqlite3_h::SQLITE_OPEN_EXCLUSIVE
            | crate::src::headers::sqlite3_h::SQLITE_OPEN_DELETEONCLOSE;
        let mut nStmtSpill: ::core::ffi::c_int = crate::src::src::global::sqlite3Config.nStmtSpill;
        let __pPager_ref = unsafe { &*pPager };
        if __pPager_ref.journalMode as ::core::ffi::c_int == crate::src::src::pager::PAGER_JOURNALMODE_MEMORY
            || __pPager_ref.subjInMemory as ::core::ffi::c_int != 0
        {
            nStmtSpill = -(1 as ::core::ffi::c_int);
        }
        rc = crate::src::src::memjournal::sqlite3JournalOpen(
            
            __pPager_ref.pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            ::core::ptr::null::<::core::ffi::c_char>(),
            
            __pPager_ref.sjfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            flags,
            nStmtSpill,
        );
    }
    rc
}

unsafe extern "C" fn subjournalPage(mut pPg: *mut crate::src::src::pcache::PgHdr) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pPager: *mut Pager = (*pPg).pPager;
    if (*pPager).journalMode as ::core::ffi::c_int != crate::src::src::pager::PAGER_JOURNALMODE_OFF {
        rc = openSubJournal(pPager);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let mut pData: *mut ::core::ffi::c_void = (*pPg).pData;
            let __pPager_ref = unsafe { &*pPager };
            let mut offset: crate::src::ext::rtree::rtree::i64_0 = __pPager_ref.nSubRec as crate::src::ext::rtree::rtree::i64_0 * (4 as crate::src::ext::rtree::rtree::i64_0 + __pPager_ref.pageSize);
            let mut pData2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            pData2 = pData as *mut ::core::ffi::c_char;
            rc = write32bits(__pPager_ref.sjfd, offset, (*pPg).pgno as crate::src::ext::rtree::rtree::u32_0);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::src::src::os::sqlite3OsWrite(
                    
                    __pPager_ref.sjfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    pData2 as *const ::core::ffi::c_void,
                    __pPager_ref.pageSize as ::core::ffi::c_int,
                    offset + 4 as crate::src::ext::rtree::rtree::i64_0,
                );
            }
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        (*pPager).nSubRec = (*pPager).nSubRec.wrapping_add(1);
        rc = addToSavepointBitvecs(pPager, (*pPg).pgno);
    }
    rc
}

unsafe extern "C" fn subjournalPageIfRequired(mut pPg: *mut crate::src::src::pcache::PgHdr) -> ::core::ffi::c_int {
    if subjRequiresPage(pPg) != 0 {
        return subjournalPage(pPg);
    } else {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    };
}

unsafe extern "C" fn pagerStress(
    mut p: *mut ::core::ffi::c_void,
    mut pPg: *mut crate::src::src::pcache::PgHdr,
) -> ::core::ffi::c_int {
    let mut pPager: *mut Pager = p as *mut Pager;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.errCode != 0 {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if __pPager_ref.doNotSpill as ::core::ffi::c_int != 0
        && (__pPager_ref.doNotSpill as ::core::ffi::c_int & (SPILLFLAG_ROLLBACK | SPILLFLAG_OFF)
            != 0 as ::core::ffi::c_int
            || (*pPg).flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_NEED_SYNC != 0 as ::core::ffi::c_int)
    {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    __pPager_ref.aStat[PAGER_STAT_SPILL as usize] =
        __pPager_ref.aStat[PAGER_STAT_SPILL as usize].wrapping_add(1);
    (*pPg).pDirty = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    if !__pPager_ref.pWal.is_null() {
        rc = subjournalPageIfRequired(pPg);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = pagerWalFrames(pPager, pPg, 0 as crate::src::src::pager::Pgno, 0 as ::core::ffi::c_int);
        }
    } else {
        if (*pPg).flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_NEED_SYNC != 0
            || __pPager_ref.eState as ::core::ffi::c_int == PAGER_WRITER_CACHEMOD
        {
            rc = syncJournal(pPager, 1 as ::core::ffi::c_int);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = pager_write_pagelist(pPager, pPg);
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::pcache::sqlite3PcacheMakeClean(pPg as *mut crate::src::src::pcache::PgHdr);
    }
    pager_error(pPager, rc)
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerFlush(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = (*pPager).errCode;
    if (*pPager).memDb == 0 {
        let mut pList: *mut crate::src::src::pcache::PgHdr =  crate::src::src::pcache::sqlite3PcacheDirtyList((*pPager).pPCache) as
    *mut crate::src::src::pcache::PgHdr;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pList.is_null() {
            let mut pNext: *mut crate::src::src::pcache::PgHdr = (*pList).pDirty;
            if (*pList).nRef == 0 as crate::src::ext::rtree::rtree::i64_0 {
                rc = pagerStress(pPager as *mut ::core::ffi::c_void, pList);
            }
            pList = pNext;
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerOpen(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut ppPager: *mut *mut Pager,
    mut zFilename: *const ::core::ffi::c_char,
    mut nExtra: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut vfsFlags: ::core::ffi::c_int,
    mut xReinit: Option<unsafe extern "C" fn(*mut crate::src::src::pager::DbPage) -> ()>,
) -> ::core::ffi::c_int {
    let mut pPtr: *mut crate::src::ext::rtree::rtree::u8_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u8_0>();
    let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut tempFile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut memDb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut memJM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut readOnly: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut journalFileSize: ::core::ffi::c_int = 0;
    let mut zPathname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nPathname: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut useJournal: ::core::ffi::c_int =
        (flags & crate::src::src::pager::PAGER_OMIT_JOURNAL == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut pcacheSize: ::core::ffi::c_int = crate::src::src::pcache::sqlite3PcacheSize();
    let mut szPageDflt: crate::src::ext::rtree::rtree::u32_0 = crate::internal::SQLITE_DEFAULT_PAGE_SIZE as crate::src::ext::rtree::rtree::u32_0;
    let mut zUri: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nUriByte: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    journalFileSize =
        crate::src::src::memjournal::sqlite3JournalSize(pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs) + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int);
    *ppPager = ::core::ptr::null_mut::<Pager>();
    if flags & crate::src::src::pager::PAGER_MEMORY != 0 {
        memDb = 1 as ::core::ffi::c_int;
        if !zFilename.is_null()
            && *zFilename.offset(0 as isize) as ::core::ffi::c_int != 0
        {
            zPathname = crate::src::src::malloc::sqlite3DbStrDup(::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>() as
    *mut crate::src::headers::sqliteInt_h::sqlite3, zFilename);
            if zPathname.is_null() {
                return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
            }
            nPathname = crate::src::src::util::sqlite3Strlen30(zPathname);
            zFilename = ::core::ptr::null::<::core::ffi::c_char>();
        }
    }
    if !zFilename.is_null()
        && *zFilename.offset(0 as isize) as ::core::ffi::c_int != 0
    {
        let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        nPathname = (*pVfs).mxPathname + 1 as ::core::ffi::c_int;
        zPathname = crate::src::src::malloc::sqlite3DbMallocRaw(
            
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>() as
    *mut crate::src::headers::sqliteInt_h::sqlite3,
            (2 as crate::src::ext::rtree::rtree::i64_0 * nPathname as crate::src::ext::rtree::rtree::i64_0) as crate::src::ext::rtree::rtree::u64_0,
        ) as *mut ::core::ffi::c_char;
        if zPathname.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
        *zPathname.offset(0 as isize) = 0 as ::core::ffi::c_char;
        rc = crate::src::src::os::sqlite3OsFullPathname(pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs, zFilename, nPathname, zPathname);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK_SYMLINK {
                if vfsFlags & crate::src::headers::sqlite3_h::SQLITE_OPEN_NOFOLLOW != 0 {
                    rc = crate::src::headers::sqlite3_h::SQLITE_CANTOPEN_SYMLINK_1;
                } else {
                    rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                }
            }
        }
        nPathname = crate::src::src::util::sqlite3Strlen30(zPathname);
        zUri = zFilename.offset(
            ((crate::src::src::util::sqlite3Strlen30
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int)(
                zFilename,
            ) + 1 as ::core::ffi::c_int) as isize,
        ) as *const ::core::ffi::c_char;
        z = zUri;
        while *z != 0 {
            z = z.offset(::libc::strlen(z).wrapping_add(1 as crate::__stddef_size_t_h::size_t) as isize);
            z = z.offset(::libc::strlen(z).wrapping_add(1 as crate::__stddef_size_t_h::size_t) as isize);
        }
        nUriByte = (z.offset(1 as isize) as *const ::core::ffi::c_char)
            .offset_from(zUri) as ::core::ffi::c_long as ::core::ffi::c_int;
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && nPathname + 8 as ::core::ffi::c_int > (*pVfs).mxPathname {
            rc = crate::src::src::main::sqlite3CantopenError(4813 as ::core::ffi::c_int);
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::malloc::sqlite3DbFree(
                
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>() as
    *mut crate::src::headers::sqliteInt_h::sqlite3,
                zPathname as *mut ::core::ffi::c_void,
            );
            return rc;
        }
    }
    pPtr = crate::src::src::malloc::sqlite3MallocZero(
        (((::core::mem::size_of::<Pager>() as usize).wrapping_add(7 as usize)
            & !(7 as ::core::ffi::c_int) as usize)
            .wrapping_add(
                (pcacheSize + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int)) as usize,
            )
            .wrapping_add(
                ((*pVfs).szOsFile + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int)) as usize,
            ) as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add((journalFileSize as crate::src::ext::rtree::rtree::u64_0).wrapping_mul(2 as crate::src::ext::rtree::rtree::u64_0))
            .wrapping_add(crate::src::headers::sqliteInt_h::SQLITE_PTRSIZE as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(4 as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(nPathname as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(1 as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(nUriByte as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(nPathname as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(8 as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(1 as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(nPathname as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(4 as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(1 as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(3 as crate::src::ext::rtree::rtree::u64_0),
    ) as *mut crate::src::ext::rtree::rtree::u8_0;
    if pPtr.is_null() {
        crate::src::src::malloc::sqlite3DbFree(
            
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>() as
    *mut crate::src::headers::sqliteInt_h::sqlite3,
            zPathname as *mut ::core::ffi::c_void,
        );
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    pPager = pPtr as *mut Pager;
    pPtr = pPtr.offset(
        ((::core::mem::size_of::<Pager>() as usize).wrapping_add(7 as usize)
            & !(7 as ::core::ffi::c_int) as usize) as isize,
    );
    (*pPager).pPCache = pPtr as *mut crate::pcache_h::PCache;
    pPtr =
        pPtr.offset((pcacheSize + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int)) as isize);
    (*pPager).fd = pPtr as *mut crate::src::headers::sqlite3_h::sqlite3_file;
    pPtr = pPtr
        .offset(((*pVfs).szOsFile + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int)) as isize);
    (*pPager).sjfd = pPtr as *mut crate::src::headers::sqlite3_h::sqlite3_file;
    pPtr = pPtr.offset(journalFileSize as isize);
    (*pPager).jfd = pPtr as *mut crate::src::headers::sqlite3_h::sqlite3_file;
    pPtr = pPtr.offset(journalFileSize as isize);
    ::core::ptr::copy_nonoverlapping(
                    &raw mut pPager as *const u8,
                    pPtr as *mut u8,
                    crate::src::headers::sqliteInt_h::SQLITE_PTRSIZE as usize,
                );
    pPtr = pPtr.offset(crate::src::headers::sqliteInt_h::SQLITE_PTRSIZE as isize);
    pPtr = pPtr.offset(4 as isize);
    (*pPager).zFilename = pPtr as *mut ::core::ffi::c_char;
    if nPathname > 0 as ::core::ffi::c_int {
        ::core::ptr::copy_nonoverlapping(
                    zPathname as *const u8,
                    pPtr as *mut u8,
                    nPathname as usize,
                );
        pPtr = pPtr.offset((nPathname + 1 as ::core::ffi::c_int) as isize);
        if !zUri.is_null() {
            ::core::ptr::copy_nonoverlapping(
                    zUri as *const u8,
                    pPtr as *mut u8,
                    nUriByte as usize,
                );
            pPtr = pPtr.offset(nUriByte as isize);
        } else {
            pPtr = pPtr.offset(1);
        }
    }
    if nPathname > 0 as ::core::ffi::c_int {
        (*pPager).zJournal = pPtr as *mut ::core::ffi::c_char;
        ::core::ptr::copy_nonoverlapping(
                    zPathname as *const u8,
                    pPtr as *mut u8,
                    nPathname as usize,
                );
        pPtr = pPtr.offset(nPathname as isize);
        ::core::ptr::copy_nonoverlapping(
                    b"-journal\0" as *const u8 as *const ::core::ffi::c_char as *const u8,
                    pPtr as *mut u8,
                    8 as usize,
                );
        pPtr = pPtr.offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
    } else {
        (*pPager).zJournal = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if nPathname > 0 as ::core::ffi::c_int {
        (*pPager).zWal = pPtr as *mut ::core::ffi::c_char;
        ::core::ptr::copy_nonoverlapping(
                    zPathname as *const u8,
                    pPtr as *mut u8,
                    nPathname as usize,
                );
        pPtr = pPtr.offset(nPathname as isize);
        ::core::ptr::copy_nonoverlapping(
                    b"-wal\0" as *const u8 as *const ::core::ffi::c_char as *const u8,
                    pPtr as *mut u8,
                    4 as usize,
                );
        pPtr = pPtr.offset((4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
    } else {
        (*pPager).zWal = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if nPathname != 0 {
        crate::src::src::malloc::sqlite3DbFree(
            
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>() as
    *mut crate::src::headers::sqliteInt_h::sqlite3,
            zPathname as *mut ::core::ffi::c_void,
        );
    }
    (*pPager).pVfs = pVfs;
    (*pPager).vfsFlags = vfsFlags as crate::src::ext::rtree::rtree::u32_0;
    let mut current_block_121: u64;
    if !zFilename.is_null()
        && *zFilename.offset(0 as isize) as ::core::ffi::c_int != 0
    {
        let mut fout: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let __pPager_ref = unsafe { &mut *pPager };
        rc = crate::src::src::os::sqlite3OsOpen(
            
            pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            __pPager_ref.zFilename,
            
            __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            vfsFlags,
            &raw mut fout,
        );
        memJM = (fout & crate::src::headers::sqlite3_h::SQLITE_OPEN_MEMORY != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        __pPager_ref.memVfs = memJM as crate::src::ext::rtree::rtree::u8_0;
        readOnly = (fout & crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let mut iDc: ::core::ffi::c_int = crate::src::src::os::sqlite3OsDeviceCharacteristics(__pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
            if readOnly == 0 {
                setSectorSize(pPager);
                if szPageDflt < __pPager_ref.sectorSize {
                    if __pPager_ref.sectorSize > crate::sqliteLimit_h::SQLITE_MAX_DEFAULT_PAGE_SIZE as crate::src::ext::rtree::rtree::u32_0 {
                        szPageDflt = crate::sqliteLimit_h::SQLITE_MAX_DEFAULT_PAGE_SIZE as crate::src::ext::rtree::rtree::u32_0;
                    } else {
                        szPageDflt = __pPager_ref.sectorSize;
                    }
                }
            }
            __pPager_ref.noLock = crate::src::src::main::sqlite3_uri_boolean(
                __pPager_ref.zFilename as crate::src::headers::sqlite3_h::sqlite3_filename,
                b"nolock\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            ) as crate::src::ext::rtree::rtree::u8_0;
            if iDc & crate::src::headers::sqlite3_h::SQLITE_IOCAP_IMMUTABLE != 0 as ::core::ffi::c_int
                || crate::src::src::main::sqlite3_uri_boolean(
                    __pPager_ref.zFilename as crate::src::headers::sqlite3_h::sqlite3_filename,
                    b"immutable\0" as *const u8 as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                ) != 0
            {
                vfsFlags |= crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY;
                current_block_121 = 13644597164098861495;
            } else {
                current_block_121 = 6712462580143783635;
            }
        } else {
            current_block_121 = 6712462580143783635;
        }
    } else {
        current_block_121 = 13644597164098861495;
    }
    match current_block_121 {
        13644597164098861495 => {
            tempFile = 1 as ::core::ffi::c_int;
            let __pPager_ref = unsafe { &mut *pPager };
            __pPager_ref.eState = PAGER_READER as crate::src::ext::rtree::rtree::u8_0;
            __pPager_ref.eLock = crate::src::src::os::EXCLUSIVE_LOCK_1 as crate::src::ext::rtree::rtree::u8_0;
            __pPager_ref.noLock = 1 as crate::src::ext::rtree::rtree::u8_0;
            readOnly = vfsFlags & crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY;
        }
        _ => {}
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3PagerSetPagesize(pPager, &raw mut szPageDflt, -(1 as ::core::ffi::c_int));
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        nExtra = nExtra + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int);
        rc = crate::src::src::pcache::sqlite3PcacheOpen(
            szPageDflt as ::core::ffi::c_int,
            nExtra,
            (memDb == 0) as ::core::ffi::c_int,
            
            if memDb == 0 {
                Some(
                    pagerStress
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut crate::src::src::pcache::PgHdr,
                        ) -> ::core::ffi::c_int,
                )
            } else {
                None
            } as
    ::std::option::Option<unsafe extern "C" fn(_: *mut ::libc::c_void,
        _: *mut crate::src::src::pcache::PgHdr) -> i32>,
            pPager as *mut ::core::ffi::c_void,
            (*pPager).pPCache,
        );
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::os::sqlite3OsClose((*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
        crate::src::src::pcache1::sqlite3PageFree((*pPager).pTmpSpace as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(pPager as *mut ::core::ffi::c_void);
        return rc;
    }
    (*pPager).useJournal = useJournal as crate::src::ext::rtree::rtree::u8_0;
    (*pPager).mxPgno = crate::sqliteLimit_h::SQLITE_MAX_PAGE_COUNT as crate::src::src::pager::Pgno;
    (*pPager).tempFile = tempFile as crate::src::ext::rtree::rtree::u8_0;
    (*pPager).exclusiveMode = tempFile as crate::src::ext::rtree::rtree::u8_0;
    (*pPager).changeCountDone = (*pPager).tempFile;
    (*pPager).memDb = memDb as crate::src::ext::rtree::rtree::u8_0;
    (*pPager).readOnly = readOnly as crate::src::ext::rtree::rtree::u8_0;
    sqlite3PagerSetFlags(
        pPager,
        (crate::src::headers::sqliteInt_h::SQLITE_DEFAULT_SYNCHRONOUS + 1 as ::core::ffi::c_int | crate::src::src::pager::PAGER_CACHESPILL)
            as ::core::ffi::c_uint,
    );
    (*pPager).nExtra = nExtra as crate::src::fts5::u16_0;
    (*pPager).journalSizeLimit = crate::src::src::pager::SQLITE_DEFAULT_JOURNAL_SIZE_LIMIT as crate::src::ext::rtree::rtree::i64_0;
    setSectorSize(pPager);
    if useJournal == 0 {
        (*pPager).journalMode = crate::src::src::pager::PAGER_JOURNALMODE_OFF as crate::src::ext::rtree::rtree::u8_0;
    } else if memDb != 0 || memJM != 0 {
        (*pPager).journalMode = crate::src::src::pager::PAGER_JOURNALMODE_MEMORY as crate::src::ext::rtree::rtree::u8_0;
    }
    (*pPager).xReiniter = xReinit;
    setGetterMethod(pPager);
    *ppPager = pPager;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3_database_file_object(
    mut zName: *const ::core::ffi::c_char,
) -> *mut crate::src::headers::sqlite3_h::sqlite3_file {
    let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    while *zName.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        || *zName.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        || *zName.offset(-(3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        || *zName.offset(-(4 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        zName = zName.offset(-1);
    }
    p = zName
        .offset(-(4 as ::core::ffi::c_int as isize))
        .offset(-(::core::mem::size_of::<*mut Pager>() as usize as isize));
    pPager = *(p as *mut *mut Pager);
    (*pPager).fd
}

unsafe extern "C" fn hasHotJournal(
    mut pPager: *mut Pager,
    mut pExists: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs = (*pPager).pVfs;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut exists: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut jrnlOpen: ::core::ffi::c_int =
        !(*(*pPager).jfd).pMethods.is_null() as ::core::ffi::c_int;
    *pExists = 0 as ::core::ffi::c_int;
    if jrnlOpen == 0 {
        rc = crate::src::src::os::sqlite3OsAccess(
            
            pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            (*pPager).zJournal,
            crate::src::headers::sqlite3_h::SQLITE_ACCESS_EXISTS,
            &raw mut exists,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && exists != 0 {
        let mut locked: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = crate::src::src::os::sqlite3OsCheckReservedLock((*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut locked);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && locked == 0 {
            let mut nPage: crate::src::src::pager::Pgno = 0;
            rc = pagerPagecount(pPager, &raw mut nPage);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                if nPage == 0 as crate::src::src::pager::Pgno && jrnlOpen == 0 {
                    crate::src::src::fault::sqlite3BeginBenignMalloc();
                    if pagerLockDb(pPager, crate::src::src::os::RESERVED_LOCK_1) == crate::src::headers::sqlite3_h::SQLITE_OK {
                        crate::src::src::os::sqlite3OsDelete(pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs, (*pPager).zJournal, 0 as ::core::ffi::c_int);
                        if (*pPager).exclusiveMode == 0 {
                            pagerUnlockDb(pPager, crate::src::src::os::SHARED_LOCK);
                        }
                    }
                    crate::src::src::fault::sqlite3EndBenignMalloc();
                } else {
                    if jrnlOpen == 0 {
                        let mut f: ::core::ffi::c_int =
                            crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY | crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_JOURNAL;
                        rc = crate::src::src::os::sqlite3OsOpen(pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs, (*pPager).zJournal,  (*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, f, &raw mut f);
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        let mut first: crate::src::ext::rtree::rtree::u8_0 = 0 as crate::src::ext::rtree::rtree::u8_0;
                        rc = crate::src::src::os::sqlite3OsRead(
                            
                            (*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                            &raw mut first as *mut ::core::ffi::c_void,
                            1 as ::core::ffi::c_int,
                            0 as crate::src::ext::rtree::rtree::i64_0,
                        );
                        if rc == crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1 {
                            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                        }
                        if jrnlOpen == 0 {
                            crate::src::src::os::sqlite3OsClose((*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
                        }
                        *pExists = (first as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                    } else if rc == crate::src::headers::sqlite3_h::SQLITE_CANTOPEN {
                        *pExists = 1 as ::core::ffi::c_int;
                        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                    }
                }
            }
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerSharedLock(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pPager).pWal.is_null() && (*pPager).eState as ::core::ffi::c_int == PAGER_OPEN {
        let mut bHotJournal: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        rc = pager_wait_on_lock(pPager, crate::src::src::os::SHARED_LOCK);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            current_block = 17065001908276206241;
        } else {
            if (*pPager).eLock as ::core::ffi::c_int <= crate::src::src::os::SHARED_LOCK {
                rc = hasHotJournal(pPager, &raw mut bHotJournal);
            }
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                current_block = 17065001908276206241;
            } else {
                if bHotJournal != 0 {
                    if (*pPager).readOnly != 0 {
                        rc = crate::src::headers::sqlite3_h::SQLITE_READONLY_ROLLBACK_1;
                        current_block = 17065001908276206241;
                    } else {
                        rc = pagerLockDb(pPager, crate::src::src::os::EXCLUSIVE_LOCK_1);
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            current_block = 17065001908276206241;
                        } else {
                            let __pPager_ref = unsafe { &mut *pPager };
                            if (*__pPager_ref.jfd).pMethods.is_null()
                                && __pPager_ref.journalMode as ::core::ffi::c_int
                                    != crate::src::src::pager::PAGER_JOURNALMODE_OFF
                            {
                                let pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs = __pPager_ref.pVfs;
                                let mut bExists: ::core::ffi::c_int = 0;
                                rc = crate::src::src::os::sqlite3OsAccess(
                                    
                                    pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                                    __pPager_ref.zJournal,
                                    crate::src::headers::sqlite3_h::SQLITE_ACCESS_EXISTS,
                                    &raw mut bExists,
                                );
                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK && bExists != 0 {
                                    let mut fout: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    let mut f: ::core::ffi::c_int =
                                        crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE | crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_JOURNAL;
                                    rc = crate::src::src::os::sqlite3OsOpen(
                                        
                                        pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                                        __pPager_ref.zJournal,
                                        
                                        __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                                        f,
                                        &raw mut fout,
                                    );
                                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && fout & crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY != 0 {
                                        rc = crate::src::src::main::sqlite3CantopenError(5334 as ::core::ffi::c_int);
                                        crate::src::src::os::sqlite3OsClose(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
                                    }
                                }
                            }
                            if !(*__pPager_ref.jfd).pMethods.is_null() {
                                rc = pagerSyncHotJournal(pPager);
                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                    rc = pager_playback(
                                        pPager,
                                        (__pPager_ref.tempFile == 0) as ::core::ffi::c_int,
                                    );
                                    __pPager_ref.eState = PAGER_OPEN as crate::src::ext::rtree::rtree::u8_0;
                                }
                            } else if __pPager_ref.exclusiveMode == 0 {
                                pagerUnlockDb(pPager, crate::src::src::os::SHARED_LOCK);
                            }
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                pager_error(pPager, rc);
                                current_block = 17065001908276206241;
                            } else {
                                current_block = 14072441030219150333;
                            }
                        }
                    }
                } else {
                    current_block = 14072441030219150333;
                }
                match current_block {
                    17065001908276206241 => {}
                    _ => {
                        if (*pPager).tempFile == 0
                            && (*pPager).hasHeldSharedLock as ::core::ffi::c_int != 0
                        {
                            let mut dbFileVers: [::core::ffi::c_char; 16] = unsafe { ::core::mem::zeroed() };
                            rc = crate::src::src::os::sqlite3OsRead(
                                
                                (*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                                &raw mut dbFileVers as *mut ::core::ffi::c_void,
                                ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                                    as ::core::ffi::c_int,
                                24 as crate::src::ext::rtree::rtree::i64_0,
                            );
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                if rc != crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1 {
                                    current_block = 17065001908276206241;
                                } else {
                                    current_block = 1622411330066726685;
                                }
                            } else {
                                current_block = 1622411330066726685;
                            }
                            match current_block {
                                17065001908276206241 => {}
                                _ => {
                                    if ::libc::memcmp(
                                        &raw mut (*pPager).dbFileVers as *mut ::core::ffi::c_char
                                            as *const ::core::ffi::c_void,
                                        &raw mut dbFileVers as *mut ::core::ffi::c_char
                                            as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                                            as crate::__stddef_size_t_h::size_t,
                                    ) != 0 as ::core::ffi::c_int
                                    {
                                        pager_reset(pPager);
                                        if (*pPager).bUseFetch != 0 {
                                            crate::src::src::os::sqlite3OsUnfetch(
                                                
                                                (*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                                                0 as crate::src::ext::rtree::rtree::i64_0,
                                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                            );
                                        }
                                    }
                                    current_block = 1423531122933789233;
                                }
                            }
                        } else {
                            current_block = 1423531122933789233;
                        }
                        match current_block {
                            17065001908276206241 => {}
                            _ => {
                                rc = pagerOpenWalIfPresent(pPager);
                                current_block = 7420279277351916581;
                            }
                        }
                    }
                }
            }
        }
    } else {
        current_block = 7420279277351916581;
    }
    match current_block {
        7420279277351916581 => {
            let __pPager_ref = unsafe { &mut *pPager };
            if !__pPager_ref.pWal.is_null() {
                rc = pagerBeginReadTransaction(pPager);
            }
            if __pPager_ref.tempFile as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && __pPager_ref.eState as ::core::ffi::c_int == PAGER_OPEN
                && rc == crate::src::headers::sqlite3_h::SQLITE_OK
            {
                rc = pagerPagecount(pPager, &raw mut __pPager_ref.dbSize);
            }
        }
        _ => {}
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        pager_unlock(pPager);
    } else {
        (*pPager).eState = PAGER_READER as crate::src::ext::rtree::rtree::u8_0;
        (*pPager).hasHeldSharedLock = 1 as crate::src::ext::rtree::rtree::u8_0;
    }
    rc
}

unsafe extern "C" fn pagerUnlockIfUnused(mut pPager: *mut Pager) {
    if crate::src::src::pcache::sqlite3PcacheRefCount((*pPager).pPCache) == 0 as crate::src::ext::rtree::rtree::i64_0 {
        pagerUnlockAndRollback(pPager);
    }
}

unsafe extern "C" fn getPageNormal(
    mut pPager: *mut Pager,
    mut pgno: crate::src::src::pager::Pgno,
    mut ppPage: *mut *mut crate::src::src::pager::DbPage,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pPg: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let mut noContent: crate::src::ext::rtree::rtree::u8_0 = 0;
    let mut pBase: *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_pcache_page>();
    if pgno == 0 as crate::src::src::pager::Pgno {
        return crate::src::src::main::sqlite3CorruptError(5547 as ::core::ffi::c_int);
    }
    pBase =  crate::src::src::pcache::sqlite3PcacheFetch((*pPager).pPCache, pgno, 3 as ::core::ffi::c_int) as *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page;
    if pBase.is_null() {
        pPg = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
        rc = crate::src::src::pcache::sqlite3PcacheFetchStress((*pPager).pPCache, pgno,  &raw mut pBase as *mut _ as *mut *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            current_block = 3222590281903869779;
        } else if pBase.is_null() {
            rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
            current_block = 3222590281903869779;
        } else {
            current_block = 7746791466490516765;
        }
    } else {
        current_block = 7746791466490516765;
    }
    match current_block {
        7746791466490516765 => {
            *ppPage =  crate::src::src::pcache::sqlite3PcacheFetchFinish((*pPager).pPCache, pgno,  pBase as *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page) as
    *mut crate::src::src::pcache::PgHdr as *mut crate::src::src::pager::DbPage;
            pPg = *ppPage as *mut crate::src::src::pcache::PgHdr;
            noContent = (flags & crate::src::src::pager::PAGER_GET_NOCONTENT != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0;
            if !(*pPg).pPager.is_null() && noContent == 0 {
                (*pPager).aStat[PAGER_STAT_HIT as usize] =
                    (*pPager).aStat[PAGER_STAT_HIT as usize].wrapping_add(1);
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            } else if pgno == (*pPager).lckPgno {
                rc = crate::src::src::main::sqlite3CorruptError(5579 as ::core::ffi::c_int);
            } else {
                (*pPg).pPager = pPager;
                if (*(*pPager).fd).pMethods.is_null()
                    || (*pPager).dbSize < pgno
                    || noContent as ::core::ffi::c_int != 0
                {
                    if pgno > (*pPager).mxPgno {
                        rc = crate::src::headers::sqlite3_h::SQLITE_FULL;
                        if pgno <= (*pPager).dbSize {
                            crate::src::src::pcache::sqlite3PcacheRelease(pPg as *mut crate::src::src::pcache::PgHdr);
                            pPg = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
                        }
                        current_block = 3222590281903869779;
                    } else {
                        if noContent != 0 {
                            crate::src::src::fault::sqlite3BeginBenignMalloc();
                            if pgno <= (*pPager).dbOrigSize {
                                crate::src::src::bitvec::sqlite3BitvecSet((*pPager).pInJournal, pgno as crate::src::ext::rtree::rtree::u32_0);
                            }
                            addToSavepointBitvecs(pPager, pgno);
                            crate::src::src::fault::sqlite3EndBenignMalloc();
                        }
                        ::libc::memset(
                            (*pPg).pData,
                            0 as ::core::ffi::c_int,
                            (*pPager).pageSize as crate::__stddef_size_t_h::size_t,
                        );
                        current_block = 7427571413727699167;
                    }
                } else {
                    (*pPager).aStat[PAGER_STAT_MISS as usize] =
                        (*pPager).aStat[PAGER_STAT_MISS as usize].wrapping_add(1);
                    rc = readDbPage(pPg);
                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                        current_block = 3222590281903869779;
                    } else {
                        current_block = 7427571413727699167;
                    }
                }
                match current_block {
                    3222590281903869779 => {}
                    _ => return crate::src::headers::sqlite3_h::SQLITE_OK,
                }
            }
        }
        _ => {}
    }
    if !pPg.is_null() {
        crate::src::src::pcache::sqlite3PcacheDrop(pPg as *mut crate::src::src::pcache::PgHdr);
    }
    pagerUnlockIfUnused(pPager);
    *ppPage = ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
    rc
}

unsafe extern "C" fn getPageMMap(
    mut pPager: *mut Pager,
    mut pgno: crate::src::src::pager::Pgno,
    mut ppPage: *mut *mut crate::src::src::pager::DbPage,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pPg: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let mut iFrame: crate::src::ext::rtree::rtree::u32_0 = 0 as crate::src::ext::rtree::rtree::u32_0;
    let bMmapOk: ::core::ffi::c_int = (pgno > 1 as crate::src::src::pager::Pgno
        && ((*pPager).eState as ::core::ffi::c_int == PAGER_READER
            || flags & crate::src::src::pager::PAGER_GET_READONLY != 0))
        as ::core::ffi::c_int;
    if pgno <= 1 as crate::src::src::pager::Pgno && pgno == 0 as crate::src::src::pager::Pgno {
        return crate::src::src::main::sqlite3CorruptError(5662 as ::core::ffi::c_int);
    }
    if bMmapOk != 0 && !(*pPager).pWal.is_null() {
        rc = crate::src::src::wal::sqlite3WalFindFrame((*pPager).pWal, pgno, &raw mut iFrame);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            *ppPage = ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
            return rc;
        }
    }
    if bMmapOk != 0 && iFrame == 0 as crate::src::ext::rtree::rtree::u32_0 {
        let mut pData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let __pPager_ref = unsafe { &*pPager };
        rc = crate::src::src::os::sqlite3OsFetch(
            
            __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            pgno.wrapping_sub(1 as crate::src::src::pager::Pgno) as crate::src::ext::rtree::rtree::i64_0 * __pPager_ref.pageSize,
            __pPager_ref.pageSize as ::core::ffi::c_int,
            &raw mut pData,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pData.is_null() {
            if __pPager_ref.eState as ::core::ffi::c_int > PAGER_READER
                || __pPager_ref.tempFile as ::core::ffi::c_int != 0
            {
                pPg = sqlite3PagerLookup(pPager, pgno) as *mut crate::src::src::pcache::PgHdr;
            }
            if pPg.is_null() {
                rc = pagerAcquireMapPage(pPager, pgno, pData, &raw mut pPg);
            } else {
                crate::src::src::os::sqlite3OsUnfetch(
                    
                    __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    pgno.wrapping_sub(1 as crate::src::src::pager::Pgno) as crate::src::ext::rtree::rtree::i64_0 * __pPager_ref.pageSize,
                    pData,
                );
            }
            if !pPg.is_null() {
                *ppPage = pPg as *mut crate::src::src::pager::DbPage;
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            *ppPage = ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
            return rc;
        }
    }
    getPageNormal(pPager, pgno, ppPage, flags)
}

unsafe extern "C" fn getPageError(
    mut pPager: *mut Pager,
    mut _pgno: crate::src::src::pager::Pgno,
    mut ppPage: *mut *mut crate::src::src::pager::DbPage,
    mut _flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *ppPage = ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
    (*pPager).errCode
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerGet(
    mut pPager: *mut Pager,
    mut pgno: crate::src::src::pager::Pgno,
    mut ppPage: *mut *mut crate::src::src::pager::DbPage,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*pPager).xGet.expect("non-null function pointer")(pPager, pgno, ppPage, flags)
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerLookup(mut pPager: *mut Pager, mut pgno: crate::src::src::pager::Pgno) -> *mut crate::src::src::pager::DbPage {
    let mut pPage: *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_pcache_page>();
    pPage =  crate::src::src::pcache::sqlite3PcacheFetch((*pPager).pPCache, pgno, 0 as ::core::ffi::c_int) as *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page;
    if pPage.is_null() {
        return ::core::ptr::null_mut::<crate::src::src::pager::DbPage>();
    }
    crate::src::src::pcache::sqlite3PcacheFetchFinish((*pPager).pPCache, pgno,  pPage as *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page) as
    *mut crate::src::src::pcache::PgHdr as *mut crate::src::src::pager::DbPage
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerUnrefNotNull(mut pPg: *mut crate::src::src::pager::DbPage) {
    if (*pPg).flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_MMAP != 0 {
        pagerReleaseMapPage(pPg as *mut crate::src::src::pcache::PgHdr);
    } else {
        crate::src::src::pcache::sqlite3PcacheRelease(pPg as *mut crate::src::src::pcache::PgHdr as *mut crate::src::src::pcache::PgHdr);
    };
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerUnref(mut pPg: *mut crate::src::src::pager::DbPage) {
    if !pPg.is_null() {
        sqlite3PagerUnrefNotNull(pPg);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerUnrefPageOne(mut pPg: *mut crate::src::src::pager::DbPage) {
    let mut pPager: *mut Pager = ::core::ptr::null_mut::<Pager>();
    pPager = (*pPg).pPager;
    crate::src::src::pcache::sqlite3PcacheRelease(pPg as *mut crate::src::src::pcache::PgHdr as *mut crate::src::src::pcache::PgHdr);
    pagerUnlockIfUnused(pPager);
}

unsafe extern "C" fn pager_open_journal(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    let pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs = __pPager_ref.pVfs;
    if __pPager_ref.errCode != 0 {
        return __pPager_ref.errCode;
    }
    if __pPager_ref.pWal.is_null()
        && __pPager_ref.journalMode as ::core::ffi::c_int != crate::src::src::pager::PAGER_JOURNALMODE_OFF
    {
        __pPager_ref.pInJournal = crate::src::src::bitvec::sqlite3BitvecCreate(__pPager_ref.dbSize as crate::src::ext::rtree::rtree::u32_0);
        if __pPager_ref.pInJournal.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
        if (*__pPager_ref.jfd).pMethods.is_null() {
            if __pPager_ref.journalMode as ::core::ffi::c_int == crate::src::src::pager::PAGER_JOURNALMODE_MEMORY {
                crate::src::src::memjournal::sqlite3MemJournalOpen(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
            } else {
                let mut flags: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE | crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE;
                let mut nSpill: ::core::ffi::c_int = 0;
                if __pPager_ref.tempFile != 0 {
                    flags |= crate::src::headers::sqlite3_h::SQLITE_OPEN_DELETEONCLOSE | crate::src::headers::sqlite3_h::SQLITE_OPEN_TEMP_JOURNAL;
                    flags |= crate::src::headers::sqlite3_h::SQLITE_OPEN_EXCLUSIVE;
                    nSpill = crate::src::src::global::sqlite3Config.nStmtSpill;
                } else {
                    flags |= crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_JOURNAL;
                    nSpill = jrnlBufferSize(pPager);
                }
                rc = databaseIsUnmoved(pPager);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    rc = crate::src::src::memjournal::sqlite3JournalOpen(pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs, __pPager_ref.zJournal,  __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, flags, nSpill);
                }
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            __pPager_ref.nRec = 0 as ::core::ffi::c_int;
            __pPager_ref.journalOff = 0 as crate::src::ext::rtree::rtree::i64_0;
            __pPager_ref.setSuper = 0 as crate::src::ext::rtree::rtree::u8_0;
            __pPager_ref.journalHdr = 0 as crate::src::ext::rtree::rtree::i64_0;
            rc = writeJournalHdr(pPager);
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::bitvec::sqlite3BitvecDestroy(__pPager_ref.pInJournal);
        __pPager_ref.pInJournal = ::core::ptr::null_mut::<crate::src::src::bitvec::Bitvec>();
        __pPager_ref.journalOff = 0 as crate::src::ext::rtree::rtree::i64_0;
    } else {
        __pPager_ref.eState = PAGER_WRITER_CACHEMOD as crate::src::ext::rtree::rtree::u8_0;
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerBegin(
    mut pPager: *mut Pager,
    mut exFlag: ::core::ffi::c_int,
    mut subjInMemory: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.errCode != 0 {
        return __pPager_ref.errCode;
    }
    __pPager_ref.subjInMemory = subjInMemory as crate::src::ext::rtree::rtree::u8_0;
    if __pPager_ref.eState as ::core::ffi::c_int == PAGER_READER {
        if !__pPager_ref.pWal.is_null() {
            if __pPager_ref.exclusiveMode as ::core::ffi::c_int != 0
                && crate::src::src::wal::sqlite3WalExclusiveMode(__pPager_ref.pWal, -(1 as ::core::ffi::c_int)) != 0
            {
                rc = pagerLockDb(pPager, crate::src::src::os::EXCLUSIVE_LOCK_1);
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    return rc;
                }
                crate::src::src::wal::sqlite3WalExclusiveMode(__pPager_ref.pWal, 1 as ::core::ffi::c_int);
            }
            rc = crate::src::src::wal::sqlite3WalBeginWriteTransaction(__pPager_ref.pWal);
        } else {
            rc = pagerLockDb(pPager, crate::src::src::os::RESERVED_LOCK_1);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK && exFlag != 0 {
                rc = pager_wait_on_lock(pPager, crate::src::src::os::EXCLUSIVE_LOCK_1);
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            __pPager_ref.eState = PAGER_WRITER_LOCKED as crate::src::ext::rtree::rtree::u8_0;
            __pPager_ref.dbHintSize = __pPager_ref.dbSize;
            __pPager_ref.dbFileSize = __pPager_ref.dbSize;
            __pPager_ref.dbOrigSize = __pPager_ref.dbSize;
            __pPager_ref.journalOff = 0 as crate::src::ext::rtree::rtree::i64_0;
        }
    }
    rc
}
#[inline(never)]

unsafe extern "C" fn pagerAddPageToRollbackJournal(mut pPg: *mut crate::src::src::pcache::PgHdr) -> ::core::ffi::c_int {
    let __pPg_ref = unsafe { &mut *pPg };
    let mut pPager: *mut Pager = __pPg_ref.pPager;
    let mut rc: ::core::ffi::c_int = 0;
    let mut cksum: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut pData2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let __pPager_ref = unsafe { &mut *pPager };
    let mut iOff: crate::src::ext::rtree::rtree::i64_0 = __pPager_ref.journalOff;
    pData2 = __pPg_ref.pData as *mut ::core::ffi::c_char;
    cksum = pager_cksum(pPager, pData2 as *mut crate::src::ext::rtree::rtree::u8_0);
    __pPg_ref.flags = (__pPg_ref.flags as ::core::ffi::c_int | crate::src::src::pcache::PGHDR_NEED_SYNC) as crate::src::fts5::u16_0;
    rc = write32bits(__pPager_ref.jfd, iOff, __pPg_ref.pgno as crate::src::ext::rtree::rtree::u32_0);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    rc = crate::src::src::os::sqlite3OsWrite(
        
        __pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
        pData2 as *const ::core::ffi::c_void,
        __pPager_ref.pageSize as ::core::ffi::c_int,
        iOff + 4 as crate::src::ext::rtree::rtree::i64_0,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    rc = write32bits(__pPager_ref.jfd, iOff + __pPager_ref.pageSize + 4 as crate::src::ext::rtree::rtree::i64_0, cksum);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    sqlite3_pager_writej_count += 1;
    __pPager_ref.journalOff += 8 as crate::src::ext::rtree::rtree::i64_0 + __pPager_ref.pageSize;
    __pPager_ref.nRec += 1;
    rc = crate::src::src::bitvec::sqlite3BitvecSet(__pPager_ref.pInJournal, __pPg_ref.pgno as crate::src::ext::rtree::rtree::u32_0);
    rc |= addToSavepointBitvecs(pPager, __pPg_ref.pgno);
    rc
}

unsafe extern "C" fn pager_write(mut pPg: *mut crate::src::src::pcache::PgHdr) -> ::core::ffi::c_int {
    let __pPg_ref = unsafe { &mut *pPg };
    let mut pPager: *mut Pager = __pPg_ref.pPager;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.eState as ::core::ffi::c_int == PAGER_WRITER_LOCKED {
        rc = pager_open_journal(pPager);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
    }
    crate::src::src::pcache::sqlite3PcacheMakeDirty(pPg as *mut crate::src::src::pcache::PgHdr);
    if !__pPager_ref.pInJournal.is_null()
        && crate::src::src::bitvec::sqlite3BitvecTestNotNull(__pPager_ref.pInJournal, __pPg_ref.pgno as crate::src::ext::rtree::rtree::u32_0)
            == 0 as ::core::ffi::c_int
    {
        if __pPg_ref.pgno <= __pPager_ref.dbOrigSize {
            rc = pagerAddPageToRollbackJournal(pPg);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
        } else if __pPager_ref.eState as ::core::ffi::c_int != PAGER_WRITER_DBMOD {
            __pPg_ref.flags = (__pPg_ref.flags as ::core::ffi::c_int | crate::src::src::pcache::PGHDR_NEED_SYNC) as crate::src::fts5::u16_0;
        }
    }
    __pPg_ref.flags = (__pPg_ref.flags as ::core::ffi::c_int | crate::src::src::pcache::PGHDR_WRITEABLE) as crate::src::fts5::u16_0;
    if __pPager_ref.nSavepoint > 0 as ::core::ffi::c_int {
        rc = subjournalPageIfRequired(pPg);
    }
    if __pPager_ref.dbSize < __pPg_ref.pgno {
        __pPager_ref.dbSize = __pPg_ref.pgno;
    }
    rc
}
#[inline(never)]

unsafe extern "C" fn pagerWriteLargeSector(mut pPg: *mut crate::src::src::pcache::PgHdr) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut nPageCount: crate::src::src::pager::Pgno = 0;
    let mut pg1: crate::src::src::pager::Pgno = 0;
    let mut nPage: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ii: ::core::ffi::c_int = 0;
    let mut needSync: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let __pPg_ref = unsafe { &mut *pPg };
    let mut pPager: *mut Pager = __pPg_ref.pPager;
    let __pPager_ref = unsafe { &mut *pPager };
    let mut nPagePerSector: crate::src::src::pager::Pgno = (__pPager_ref.sectorSize as crate::src::ext::rtree::rtree::i64_0 / __pPager_ref.pageSize) as crate::src::src::pager::Pgno;
    __pPager_ref.doNotSpill = (__pPager_ref.doNotSpill as ::core::ffi::c_int | SPILLFLAG_NOSYNC) as crate::src::ext::rtree::rtree::u8_0;
    pg1 = (__pPg_ref.pgno.wrapping_sub(1 as crate::src::src::pager::Pgno) & !nPagePerSector.wrapping_sub(1 as crate::src::src::pager::Pgno))
        .wrapping_add(1 as crate::src::src::pager::Pgno);
    nPageCount = __pPager_ref.dbSize;
    if __pPg_ref.pgno > nPageCount {
        nPage = __pPg_ref.pgno.wrapping_sub(pg1).wrapping_add(1 as crate::src::src::pager::Pgno) as ::core::ffi::c_int;
    } else if pg1.wrapping_add(nPagePerSector).wrapping_sub(1 as crate::src::src::pager::Pgno) > nPageCount {
        nPage = nPageCount.wrapping_add(1 as crate::src::src::pager::Pgno).wrapping_sub(pg1) as ::core::ffi::c_int;
    } else {
        nPage = nPagePerSector as ::core::ffi::c_int;
    }
    ii = 0 as ::core::ffi::c_int;
    while ii < nPage && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut pg: crate::src::src::pager::Pgno = pg1.wrapping_add(ii as crate::src::src::pager::Pgno);
        let mut pPage: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
        if pg == __pPg_ref.pgno || crate::src::src::bitvec::sqlite3BitvecTest(__pPager_ref.pInJournal, pg as crate::src::ext::rtree::rtree::u32_0) == 0 {
            if pg != __pPager_ref.lckPgno {
                rc = sqlite3PagerGet(pPager, pg, &raw mut pPage, 0 as ::core::ffi::c_int);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    rc = pager_write(pPage);
                    if (*pPage).flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_NEED_SYNC != 0 {
                        needSync = 1 as ::core::ffi::c_int;
                    }
                    sqlite3PagerUnrefNotNull(pPage as *mut crate::src::src::pager::DbPage);
                }
            }
        } else {
            pPage = sqlite3PagerLookup(pPager, pg) as *mut crate::src::src::pcache::PgHdr;
            if !pPage.is_null() {
                if (*pPage).flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_NEED_SYNC != 0 {
                    needSync = 1 as ::core::ffi::c_int;
                }
                sqlite3PagerUnrefNotNull(pPage as *mut crate::src::src::pager::DbPage);
            }
        }
        ii += 1;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && needSync != 0 {
        ii = 0 as ::core::ffi::c_int;
        while ii < nPage {
            let mut pPage_0: *mut crate::src::src::pcache::PgHdr =
                sqlite3PagerLookup(pPager, pg1.wrapping_add(ii as crate::src::src::pager::Pgno)) as *mut crate::src::src::pcache::PgHdr;
            if !pPage_0.is_null() {
                (*pPage_0).flags =
                    ((*pPage_0).flags as ::core::ffi::c_int | crate::src::src::pcache::PGHDR_NEED_SYNC) as crate::src::fts5::u16_0;
                sqlite3PagerUnrefNotNull(pPage_0 as *mut crate::src::src::pager::DbPage);
            }
            ii += 1;
        }
    }
    __pPager_ref.doNotSpill = (__pPager_ref.doNotSpill as ::core::ffi::c_int & !SPILLFLAG_NOSYNC) as crate::src::ext::rtree::rtree::u8_0;
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerWrite(mut pPg: *mut crate::src::src::pcache::PgHdr) -> ::core::ffi::c_int {
    let __pPg_ref = unsafe { &*pPg };
    let mut pPager: *mut Pager = __pPg_ref.pPager;
    let __pPager_ref = unsafe { &*pPager };
    if __pPg_ref.flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_WRITEABLE != 0 as ::core::ffi::c_int
        && __pPager_ref.dbSize >= __pPg_ref.pgno
    {
        if __pPager_ref.nSavepoint != 0 {
            return subjournalPageIfRequired(pPg);
        }
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    } else if __pPager_ref.errCode != 0 {
        return __pPager_ref.errCode;
    } else if __pPager_ref.sectorSize > __pPager_ref.pageSize as crate::src::ext::rtree::rtree::u32_0 {
        return pagerWriteLargeSector(pPg);
    } else {
        return pager_write(pPg);
    };
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerDontWrite(mut pPg: *mut crate::src::src::pcache::PgHdr) {
    let mut pPager: *mut Pager = (*pPg).pPager;
    if (*pPager).tempFile == 0
        && (*pPg).flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_DIRTY != 0
        && (*pPager).nSavepoint == 0 as ::core::ffi::c_int
    {
        let __pPg_ref = unsafe { &mut *pPg };
        __pPg_ref.flags = (__pPg_ref.flags as ::core::ffi::c_int | crate::src::src::pcache::PGHDR_DONT_WRITE) as crate::src::fts5::u16_0;
        __pPg_ref.flags = (__pPg_ref.flags as ::core::ffi::c_int & !crate::src::src::pcache::PGHDR_WRITEABLE) as crate::src::fts5::u16_0;
    }
}

unsafe extern "C" fn pager_incr_changecounter(
    mut pPager: *mut Pager,
    mut _isDirectMode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pPager).changeCountDone == 0 && (*pPager).dbSize > 0 as crate::src::src::pager::Pgno {
        let mut pPgHdr: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
        rc = sqlite3PagerGet(pPager, 1 as crate::src::src::pager::Pgno, &raw mut pPgHdr, 0 as ::core::ffi::c_int);
        if DIRECT_MODE == 0 && rc == 0 as ::core::ffi::c_int {
            rc = sqlite3PagerWrite(pPgHdr);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            pager_write_changecounter(pPgHdr);
            (*pPager).changeCountDone = 1 as crate::src::ext::rtree::rtree::u8_0;
        }
        sqlite3PagerUnref(pPgHdr as *mut crate::src::src::pager::DbPage);
    }
    rc
}

pub const DIRECT_MODE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerSync(
    mut pPager: *mut Pager,
    mut zSuper: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pArg: *mut ::core::ffi::c_void = zSuper as *mut ::core::ffi::c_void;
    rc = crate::src::src::os::sqlite3OsFileControl((*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file, crate::src::headers::sqlite3_h::SQLITE_FCNTL_SYNC, pArg);
    if rc == crate::src::headers::sqlite3_h::SQLITE_NOTFOUND {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && (*pPager).noSync == 0 {
        rc = crate::src::src::os::sqlite3OsSync((*pPager).fd as *mut crate::src::headers::sqlite3_h::sqlite3_file, (*pPager).syncFlags as ::core::ffi::c_int);
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerExclusiveLock(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = (*pPager).errCode;
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if 0 as ::core::ffi::c_int
            == ((*pPager).pWal != ::core::ptr::null_mut::<crate::src::src::wal::Wal>()) as ::core::ffi::c_int
        {
            rc = pager_wait_on_lock(pPager, crate::src::src::os::EXCLUSIVE_LOCK_1);
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerCommitPhaseOne(
    mut pPager: *mut Pager,
    mut zSuper: *const ::core::ffi::c_char,
    mut noSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.errCode != 0 {
        return __pPager_ref.errCode;
    }
    if crate::src::src::util::sqlite3FaultSim(400 as ::core::ffi::c_int) != 0 {
        return crate::src::headers::sqlite3_h::SQLITE_IOERR;
    }
    if (__pPager_ref.eState as ::core::ffi::c_int) < PAGER_WRITER_CACHEMOD {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if 0 as ::core::ffi::c_int == pagerFlushOnCommit(pPager, 1 as ::core::ffi::c_int) {
        crate::src::src::backup::sqlite3BackupRestart(__pPager_ref.pBackup);
    } else {
        let mut pList: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
        if !__pPager_ref.pWal.is_null() {
            let mut pPageOne: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
            pList =  crate::src::src::pcache::sqlite3PcacheDirtyList(__pPager_ref.pPCache) as
    *mut crate::src::src::pcache::PgHdr;
            if pList.is_null() {
                rc = sqlite3PagerGet(
                    pPager,
                    1 as crate::src::src::pager::Pgno,
                    &raw mut pPageOne,
                    0 as ::core::ffi::c_int,
                );
                pList = pPageOne;
                (*pList).pDirty = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
            }
            if !pList.is_null() {
                rc = pagerWalFrames(pPager, pList, __pPager_ref.dbSize, 1 as ::core::ffi::c_int);
            }
            sqlite3PagerUnref(pPageOne as *mut crate::src::src::pager::DbPage);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::pcache::sqlite3PcacheCleanAll(__pPager_ref.pPCache);
            }
        } else {
            rc = pager_incr_changecounter(pPager, 0 as ::core::ffi::c_int);
            if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                rc = writeSuperJournal(pPager, zSuper);
                if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                    rc = syncJournal(pPager, 0 as ::core::ffi::c_int);
                    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                        pList =  crate::src::src::pcache::sqlite3PcacheDirtyList(__pPager_ref.pPCache) as
    *mut crate::src::src::pcache::PgHdr;
                        if bBatch == 0 as ::core::ffi::c_int {
                            rc = pager_write_pagelist(pPager, pList);
                        }
                        if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                            crate::src::src::pcache::sqlite3PcacheCleanAll(__pPager_ref.pPCache);
                            if __pPager_ref.dbSize > __pPager_ref.dbFileSize {
                                let mut nNew: crate::src::src::pager::Pgno = __pPager_ref.dbSize.wrapping_sub(
                                    (__pPager_ref.dbSize == __pPager_ref.lckPgno) as ::core::ffi::c_int
                                        as crate::src::src::pager::Pgno,
                                );
                                rc = pager_truncate(pPager, nNew);
                                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                    current_block = 14801210418546061600;
                                } else {
                                    current_block = 6450636197030046351;
                                }
                            } else {
                                current_block = 6450636197030046351;
                            }
                            match current_block {
                                14801210418546061600 => {}
                                _ => {
                                    if noSync == 0 {
                                        rc = sqlite3PagerSync(pPager, zSuper);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && __pPager_ref.pWal.is_null() {
        __pPager_ref.eState = PAGER_WRITER_FINISHED as crate::src::ext::rtree::rtree::u8_0;
    }
    rc
}

pub const bBatch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerCommitPhaseTwo(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.errCode != 0 {
        return __pPager_ref.errCode;
    }
    __pPager_ref.iDataVersion = __pPager_ref.iDataVersion.wrapping_add(1);
    if __pPager_ref.eState as ::core::ffi::c_int == PAGER_WRITER_LOCKED
        && __pPager_ref.exclusiveMode as ::core::ffi::c_int != 0
        && __pPager_ref.journalMode as ::core::ffi::c_int == crate::src::src::pager::PAGER_JOURNALMODE_PERSIST
    {
        __pPager_ref.eState = PAGER_READER as crate::src::ext::rtree::rtree::u8_0;
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc = pager_end_transaction(
        pPager,
        __pPager_ref.setSuper as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    pager_error(pPager, rc)
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerRollback(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.eState as ::core::ffi::c_int == PAGER_ERROR {
        return __pPager_ref.errCode;
    }
    if __pPager_ref.eState as ::core::ffi::c_int <= PAGER_READER {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if !__pPager_ref.pWal.is_null() {
        let mut rc2: ::core::ffi::c_int = 0;
        rc = sqlite3PagerSavepoint(pPager, crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK, -(1 as ::core::ffi::c_int));
        rc2 = pager_end_transaction(
            pPager,
            __pPager_ref.setSuper as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    } else if (*__pPager_ref.jfd).pMethods.is_null()
        || __pPager_ref.eState as ::core::ffi::c_int == PAGER_WRITER_LOCKED
    {
        let mut eState: ::core::ffi::c_int = __pPager_ref.eState as ::core::ffi::c_int;
        rc = pager_end_transaction(pPager, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        if __pPager_ref.memDb == 0 && eState > PAGER_WRITER_LOCKED {
            __pPager_ref.errCode = crate::src::headers::sqlite3_h::SQLITE_ABORT;
            __pPager_ref.eState = PAGER_ERROR as crate::src::ext::rtree::rtree::u8_0;
            setGetterMethod(pPager);
            return rc;
        }
    } else {
        rc = pager_playback(pPager, 0 as ::core::ffi::c_int);
    }
    pager_error(pPager, rc)
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerIsreadonly(mut pPager: *mut Pager) -> crate::src::ext::rtree::rtree::u8_0 {
    (*pPager).readOnly
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerMemUsed(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let __pPager_ref = unsafe { &*pPager };
    let mut perPageSize: ::core::ffi::c_int = (__pPager_ref.pageSize
        + __pPager_ref.nExtra as crate::src::ext::rtree::rtree::i64_0
        + (::core::mem::size_of::<crate::src::src::pcache::PgHdr>() as usize).wrapping_add(
            (5 as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize),
        ) as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0)
        as ::core::ffi::c_int;
    ((perPageSize * crate::src::src::pcache::sqlite3PcachePagecount(__pPager_ref.pPCache)
        + crate::src::src::malloc::sqlite3MallocSize(pPager as *const ::core::ffi::c_void)) as crate::src::ext::rtree::rtree::i64_0
        + __pPager_ref.pageSize) as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerPageRefcount(mut pPage: *mut crate::src::src::pager::DbPage) -> ::core::ffi::c_int {
    crate::src::src::pcache::sqlite3PcachePageRefcount(pPage as *mut crate::src::src::pcache::PgHdr as *mut crate::src::src::pcache::PgHdr) as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerStats(mut pPager: *mut Pager) -> *mut ::core::ffi::c_int {
    static mut a: [::core::ffi::c_int; 11] = [0; 11];
    let __pPager_ref = unsafe { &*pPager };
    a[0 as ::core::ffi::c_int as usize] =
        crate::src::src::pcache::sqlite3PcacheRefCount(__pPager_ref.pPCache) as ::core::ffi::c_int;
    a[1 as ::core::ffi::c_int as usize] = crate::src::src::pcache::sqlite3PcachePagecount(__pPager_ref.pPCache);
    a[2 as ::core::ffi::c_int as usize] = crate::src::src::pcache::sqlite3PcacheGetCachesize(__pPager_ref.pPCache);
    a[3 as ::core::ffi::c_int as usize] = if __pPager_ref.eState as ::core::ffi::c_int == PAGER_OPEN {
        -(1 as ::core::ffi::c_int)
    } else {
        __pPager_ref.dbSize as ::core::ffi::c_int
    };
    a[4 as ::core::ffi::c_int as usize] = __pPager_ref.eState as ::core::ffi::c_int;
    a[5 as ::core::ffi::c_int as usize] = __pPager_ref.errCode;
    a[6 as ::core::ffi::c_int as usize] = __pPager_ref.aStat[PAGER_STAT_HIT as usize]
        as ::core::ffi::c_int
        & 0x7fffffff as ::core::ffi::c_int;
    a[7 as ::core::ffi::c_int as usize] = __pPager_ref.aStat[PAGER_STAT_MISS as usize]
        as ::core::ffi::c_int
        & 0x7fffffff as ::core::ffi::c_int;
    a[8 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    a[9 as ::core::ffi::c_int as usize] = __pPager_ref.nRead;
    a[10 as ::core::ffi::c_int as usize] = __pPager_ref.aStat[PAGER_STAT_WRITE as usize]
        as ::core::ffi::c_int
        & 0x7fffffff as ::core::ffi::c_int;
    &raw mut a as *mut ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerCacheStat(
    mut pPager: *mut Pager,
    mut eStat: ::core::ffi::c_int,
    mut reset: ::core::ffi::c_int,
    mut pnVal: *mut crate::src::ext::rtree::rtree::u64_0,
) {
    eStat -= crate::src::headers::sqlite3_h::SQLITE_DBSTATUS_CACHE_HIT;
    *pnVal = (*pnVal).wrapping_add((*pPager).aStat[eStat as usize] as crate::src::ext::rtree::rtree::u64_0);
    if reset != 0 {
        (*pPager).aStat[eStat as usize] = 0 as crate::src::ext::rtree::rtree::u32_0;
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerIsMemdb(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    ((*pPager).tempFile as ::core::ffi::c_int != 0
        || (*pPager).memVfs as ::core::ffi::c_int != 0) as ::core::ffi::c_int
}
#[inline(never)]

unsafe extern "C" fn pagerOpenSavepoint(
    mut pPager: *mut Pager,
    mut nSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    let mut nCurrent: ::core::ffi::c_int = __pPager_ref.nSavepoint;
    let mut ii: ::core::ffi::c_int = 0;
    let mut aNew: *mut PagerSavepoint = ::core::ptr::null_mut::<PagerSavepoint>();
    aNew = crate::src::src::malloc::sqlite3Realloc(
        __pPager_ref.aSavepoint as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<PagerSavepoint>() as usize).wrapping_mul(nSavepoint as usize)
            as crate::src::ext::rtree::rtree::u64_0,
    ) as *mut PagerSavepoint;
    if aNew.is_null() {
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    ::libc::memset(
        aNew.offset(nCurrent as isize) as *mut PagerSavepoint as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ((nSavepoint - nCurrent) as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<PagerSavepoint>() as crate::__stddef_size_t_h::size_t),
    );
    __pPager_ref.aSavepoint = aNew;
    ii = nCurrent;
    while ii < nSavepoint {
        (*aNew.offset(ii as isize)).nOrig = __pPager_ref.dbSize;
        if !(*__pPager_ref.jfd).pMethods.is_null() && __pPager_ref.journalOff > 0 as crate::src::ext::rtree::rtree::i64_0 {
            (*aNew.offset(ii as isize)).iOffset = __pPager_ref.journalOff;
        } else {
            (*aNew.offset(ii as isize)).iOffset = __pPager_ref.sectorSize as crate::src::ext::rtree::rtree::i64_0;
        }
        (*aNew.offset(ii as isize)).iSubRec = __pPager_ref.nSubRec as crate::src::src::pager::Pgno;
        let ref mut fresh3 = (*aNew.offset(ii as isize)).pInSavepoint;
        *fresh3 = crate::src::src::bitvec::sqlite3BitvecCreate(__pPager_ref.dbSize as crate::src::ext::rtree::rtree::u32_0);
        (*aNew.offset(ii as isize)).bTruncateOnRelease = 1 as ::core::ffi::c_int;
        if (*aNew.offset(ii as isize)).pInSavepoint.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
        if !__pPager_ref.pWal.is_null() {
            crate::src::src::wal::sqlite3WalSavepoint(
                __pPager_ref.pWal,
                &raw mut (*aNew.offset(ii as isize)).aWalData as *mut crate::src::ext::rtree::rtree::u32_0,
            );
        }
        __pPager_ref.nSavepoint = ii + 1 as ::core::ffi::c_int;
        ii += 1;
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerOpenSavepoint(
    mut pPager: *mut Pager,
    mut nSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if nSavepoint > (*pPager).nSavepoint && (*pPager).useJournal as ::core::ffi::c_int != 0 {
        return pagerOpenSavepoint(pPager, nSavepoint);
    } else {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    };
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerSavepoint(
    mut pPager: *mut Pager,
    mut op: ::core::ffi::c_int,
    mut iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = (*pPager).errCode;
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && iSavepoint < (*pPager).nSavepoint {
        let mut ii: ::core::ffi::c_int = 0;
        let mut nNew: ::core::ffi::c_int = 0;
        nNew = iSavepoint
            + (if op == crate::src::headers::sqliteInt_h::SAVEPOINT_RELEASE {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            });
        ii = nNew;
        let __pPager_ref = unsafe { &mut *pPager };
        while ii < __pPager_ref.nSavepoint {
            crate::src::src::bitvec::sqlite3BitvecDestroy((*__pPager_ref.aSavepoint.offset(ii as isize)).pInSavepoint);
            ii += 1;
        }
        __pPager_ref.nSavepoint = nNew;
        if op == crate::src::headers::sqliteInt_h::SAVEPOINT_RELEASE {
            let pRel = &*(__pPager_ref.aSavepoint.offset(nNew as isize) as *mut PagerSavepoint);

            if pRel.bTruncateOnRelease != 0 && !(*__pPager_ref.sjfd).pMethods.is_null() {
                if crate::src::src::memjournal::sqlite3JournalIsInMemory(__pPager_ref.sjfd as *mut crate::src::headers::sqlite3_h::sqlite3_file) != 0 {
                    let mut sz: crate::src::ext::rtree::rtree::i64_0 =
                        (__pPager_ref.pageSize + 4 as crate::src::ext::rtree::rtree::i64_0) * pRel.iSubRec as crate::src::ext::rtree::rtree::i64_0;
                    rc = crate::src::src::os::sqlite3OsTruncate(__pPager_ref.sjfd as *mut crate::src::headers::sqlite3_h::sqlite3_file, sz);
                }
                __pPager_ref.nSubRec = pRel.iSubRec as crate::src::ext::rtree::rtree::u32_0;
            }
        } else if !__pPager_ref.pWal.is_null() || !(*__pPager_ref.jfd).pMethods.is_null() {
            let mut pSavepoint: *mut PagerSavepoint = if nNew == 0 as ::core::ffi::c_int {
                ::core::ptr::null_mut::<PagerSavepoint>()
            } else {
                (*pPager)
                    .aSavepoint
                    .offset((nNew - 1 as ::core::ffi::c_int) as isize)
                    as *mut PagerSavepoint
            };
            rc = pagerPlaybackSavepoint(pPager, pSavepoint);
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerFilename(
    mut pPager: *const Pager,
    mut nullIfMemDb: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    static mut zFake: [::core::ffi::c_char; 8] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
    ];
    if nullIfMemDb != 0
        && ((*pPager).memDb as ::core::ffi::c_int != 0 || crate::src::src::memdb::sqlite3IsMemdb((*pPager).pVfs as *const crate::src::headers::sqlite3_h::sqlite3_vfs) != 0)
    {
        return (&raw const zFake as *const ::core::ffi::c_char)
            .offset(4 as isize) as *const ::core::ffi::c_char;
    } else {
        return (*pPager).zFilename;
    };
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerVfs(mut pPager: *mut Pager) -> *mut crate::src::headers::sqlite3_h::sqlite3_vfs {
    (*pPager).pVfs
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerFile(mut pPager: *mut Pager) -> *mut crate::src::headers::sqlite3_h::sqlite3_file {
    (*pPager).fd
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerJrnlFile(mut pPager: *mut Pager) -> *mut crate::src::headers::sqlite3_h::sqlite3_file {
    if !(*pPager).pWal.is_null() {
        
        crate::src::src::wal::sqlite3WalFile((*pPager).pWal) as
    *mut crate::src::headers::sqlite3_h::sqlite3_file
    } else {
        (*pPager).jfd
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerJournalname(
    mut pPager: *mut Pager,
) -> *const ::core::ffi::c_char {
    (*pPager).zJournal
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerMovepage(
    mut pPager: *mut Pager,
    mut pPg: *mut crate::src::src::pager::DbPage,
    mut pgno: crate::src::src::pager::Pgno,
    mut isCommit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pPgOld: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let mut needSyncPgno: crate::src::src::pager::Pgno = 0 as crate::src::src::pager::Pgno;
    let mut rc: ::core::ffi::c_int = 0;
    let mut origPgno: crate::src::src::pager::Pgno = 0;
    if (*pPager).tempFile != 0 {
        rc = sqlite3PagerWrite(pPg as *mut crate::src::src::pcache::PgHdr);
        if rc != 0 {
            return rc;
        }
    }
    let __pPg_ref = unsafe { &mut *pPg };
    if __pPg_ref.flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_DIRTY != 0 as ::core::ffi::c_int && {
        rc = subjournalPageIfRequired(pPg as *mut crate::src::src::pcache::PgHdr);
        crate::src::headers::sqlite3_h::SQLITE_OK != rc
    } {
        return rc;
    }
    if __pPg_ref.flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_NEED_SYNC != 0 && isCommit == 0 {
        needSyncPgno = __pPg_ref.pgno;
    }
    __pPg_ref.flags = (__pPg_ref.flags as ::core::ffi::c_int & !crate::src::src::pcache::PGHDR_NEED_SYNC) as crate::src::fts5::u16_0;
    pPgOld = sqlite3PagerLookup(pPager, pgno) as *mut crate::src::src::pcache::PgHdr;
    if !pPgOld.is_null() {
        if (*pPgOld).nRef > 1 as crate::src::ext::rtree::rtree::i64_0 {
            sqlite3PagerUnrefNotNull(pPgOld as *mut crate::src::src::pager::DbPage);
            return crate::src::src::main::sqlite3CorruptError(7228 as ::core::ffi::c_int);
        }
        __pPg_ref.flags = (__pPg_ref.flags as ::core::ffi::c_int
            | (*pPgOld).flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_NEED_SYNC)
            as crate::src::fts5::u16_0;
        if (*pPager).tempFile != 0 {
            crate::src::src::pcache::sqlite3PcacheMove(pPgOld as *mut crate::src::src::pcache::PgHdr, (*pPager).dbSize.wrapping_add(1 as crate::src::src::pager::Pgno));
        } else {
            crate::src::src::pcache::sqlite3PcacheDrop(pPgOld as *mut crate::src::src::pcache::PgHdr);
        }
    }
    origPgno = __pPg_ref.pgno;
    crate::src::src::pcache::sqlite3PcacheMove(pPg as *mut crate::src::src::pcache::PgHdr as *mut crate::src::src::pcache::PgHdr, pgno);
    crate::src::src::pcache::sqlite3PcacheMakeDirty(pPg as *mut crate::src::src::pcache::PgHdr as *mut crate::src::src::pcache::PgHdr);
    if (*pPager).tempFile as ::core::ffi::c_int != 0 && !pPgOld.is_null() {
        crate::src::src::pcache::sqlite3PcacheMove(pPgOld as *mut crate::src::src::pcache::PgHdr, origPgno);
        sqlite3PagerUnrefNotNull(pPgOld as *mut crate::src::src::pager::DbPage);
    }
    if needSyncPgno != 0 {
        let mut pPgHdr: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
        rc = sqlite3PagerGet(
            pPager,
            needSyncPgno,
            &raw mut pPgHdr,
            0 as ::core::ffi::c_int,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            if needSyncPgno <= (*pPager).dbOrigSize {
                crate::src::src::bitvec::sqlite3BitvecClear(
                    (*pPager).pInJournal,
                    needSyncPgno as crate::src::ext::rtree::rtree::u32_0,
                    (*pPager).pTmpSpace as *mut ::core::ffi::c_void,
                );
            }
            return rc;
        }
        (*pPgHdr).flags = ((*pPgHdr).flags as ::core::ffi::c_int | crate::src::src::pcache::PGHDR_NEED_SYNC) as crate::src::fts5::u16_0;
        crate::src::src::pcache::sqlite3PcacheMakeDirty(pPgHdr as *mut crate::src::src::pcache::PgHdr);
        sqlite3PagerUnrefNotNull(pPgHdr as *mut crate::src::src::pager::DbPage);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerRekey(mut pPg: *mut crate::src::src::pager::DbPage, mut iNew: crate::src::src::pager::Pgno, mut flags: crate::src::fts5::u16_0) {
    (*pPg).flags = flags;
    crate::src::src::pcache::sqlite3PcacheMove(pPg as *mut crate::src::src::pcache::PgHdr as *mut crate::src::src::pcache::PgHdr, iNew);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerGetData(mut pPg: *mut crate::src::src::pager::DbPage) -> *mut ::core::ffi::c_void {
    (*pPg).pData
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerGetExtra(mut pPg: *mut crate::src::src::pager::DbPage) -> *mut ::core::ffi::c_void {
    (*pPg).pExtra
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerLockingMode(
    mut pPager: *mut Pager,
    mut eMode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pPager_ref = unsafe { &mut *pPager };
    if eMode >= 0 as ::core::ffi::c_int
        && __pPager_ref.tempFile == 0
        && crate::src::src::wal::sqlite3WalHeapMemory(__pPager_ref.pWal) == 0
    {
        __pPager_ref.exclusiveMode = eMode as crate::src::ext::rtree::rtree::u8_0;
    }
    __pPager_ref.exclusiveMode as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerSetJournalMode(
    mut pPager: *mut Pager,
    mut eMode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pPager_ref = unsafe { &mut *pPager };
    let mut eOld: crate::src::ext::rtree::rtree::u8_0 = __pPager_ref.journalMode;
    if __pPager_ref.memDb != 0 {
        if eMode != crate::src::src::pager::PAGER_JOURNALMODE_MEMORY && eMode != crate::src::src::pager::PAGER_JOURNALMODE_OFF {
            eMode = eOld as ::core::ffi::c_int;
        }
    }
    if eMode != eOld as ::core::ffi::c_int {
        __pPager_ref.journalMode = eMode as crate::src::ext::rtree::rtree::u8_0;
        if __pPager_ref.exclusiveMode == 0
            && eOld as ::core::ffi::c_int & 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && eMode & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            crate::src::src::os::sqlite3OsClose(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
            if __pPager_ref.eLock as ::core::ffi::c_int >= crate::src::src::os::RESERVED_LOCK_1 {
                crate::src::src::os::sqlite3OsDelete(__pPager_ref.pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs, __pPager_ref.zJournal, 0 as ::core::ffi::c_int);
            } else {
                let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
                let mut state: ::core::ffi::c_int = __pPager_ref.eState as ::core::ffi::c_int;
                if state == PAGER_OPEN {
                    rc = sqlite3PagerSharedLock(pPager);
                }
                if __pPager_ref.eState as ::core::ffi::c_int == PAGER_READER {
                    rc = pagerLockDb(pPager, crate::src::src::os::RESERVED_LOCK_1);
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    crate::src::src::os::sqlite3OsDelete(__pPager_ref.pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs, __pPager_ref.zJournal, 0 as ::core::ffi::c_int);
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK && state == PAGER_READER {
                    pagerUnlockDb(pPager, crate::src::src::os::SHARED_LOCK);
                } else if state == PAGER_OPEN {
                    pager_unlock(pPager);
                }
            }
        } else if eMode == crate::src::src::pager::PAGER_JOURNALMODE_OFF || eMode == crate::src::src::pager::PAGER_JOURNALMODE_MEMORY {
            crate::src::src::os::sqlite3OsClose(__pPager_ref.jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
        }
    }
    __pPager_ref.journalMode as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerGetJournalMode(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    (*pPager).journalMode as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerOkToChangeJournalMode(
    mut pPager: *mut Pager,
) -> ::core::ffi::c_int {
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.eState as ::core::ffi::c_int >= PAGER_WRITER_CACHEMOD {
        return 0 as ::core::ffi::c_int;
    }
    if !(*__pPager_ref.jfd).pMethods.is_null() && __pPager_ref.journalOff > 0 as crate::src::ext::rtree::rtree::i64_0 {
        return 0 as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerJournalSizeLimit(
    mut pPager: *mut Pager,
    mut iLimit: crate::src::ext::rtree::rtree::i64_0,
) -> crate::src::ext::rtree::rtree::i64_0 {
    if iLimit >= -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0 {
        (*pPager).journalSizeLimit = iLimit;
        crate::src::src::wal::sqlite3WalLimit((*pPager).pWal, iLimit);
    }
    (*pPager).journalSizeLimit
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerBackupPtr(mut pPager: *mut Pager) -> *mut *mut crate::src::src::backup::sqlite3_backup {
    &raw mut (*pPager).pBackup
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerClearCache(mut pPager: *mut Pager) {
    if (*pPager).tempFile as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        pager_reset(pPager);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerCheckpoint(
    mut pPager: *mut Pager,
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut eMode: ::core::ffi::c_int,
    mut pnLog: *mut ::core::ffi::c_int,
    mut pnCkpt: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPager_ref = unsafe { &mut *pPager };
    if __pPager_ref.pWal.is_null()
        && __pPager_ref.journalMode as ::core::ffi::c_int == crate::src::src::pager::PAGER_JOURNALMODE_WAL
    {
        crate::src::src::legacy::sqlite3_exec(
            
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            b"PRAGMA table_list\0" as *const u8 as *const ::core::ffi::c_char,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    if !__pPager_ref.pWal.is_null() {
        rc = crate::src::src::wal::sqlite3WalCheckpoint(
            __pPager_ref.pWal,
            
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            eMode,
            if eMode <= crate::src::headers::sqlite3_h::SQLITE_CHECKPOINT_PASSIVE {
                None
            } else {
                __pPager_ref.xBusyHandler
            },
            __pPager_ref.pBusyHandlerArg,
            __pPager_ref.walSyncFlags as ::core::ffi::c_int,
            __pPager_ref.pageSize as ::core::ffi::c_int,
            __pPager_ref.pTmpSpace as *mut crate::src::ext::rtree::rtree::u8_0,
            pnLog,
            pnCkpt,
        );
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerWalCallback(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    crate::src::src::wal::sqlite3WalCallback((*pPager).pWal)
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerWalSupported(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let __pPager_ref = unsafe { &*pPager };
    let pMethods = &*((*__pPager_ref.fd).pMethods as *const crate::src::headers::sqlite3_h::sqlite3_io_methods);

    if __pPager_ref.noLock != 0 {
        return 0 as ::core::ffi::c_int;
    }
    (__pPager_ref.exclusiveMode as ::core::ffi::c_int != 0
        || pMethods.iVersion >= 2 as ::core::ffi::c_int && pMethods.xShmMap.is_some())
        as ::core::ffi::c_int
}

unsafe extern "C" fn pagerExclusiveLock(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut eOrigLock: crate::src::ext::rtree::rtree::u8_0 = 0;
    eOrigLock = (*pPager).eLock;
    rc = pagerLockDb(pPager, crate::src::src::os::EXCLUSIVE_LOCK_1);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        pagerUnlockDb(pPager, eOrigLock as ::core::ffi::c_int);
    }
    rc
}

unsafe extern "C" fn pagerOpenWal(mut pPager: *mut Pager) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pPager).exclusiveMode != 0 {
        rc = pagerExclusiveLock(pPager);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pPager_ref = unsafe { &mut *pPager };
        rc = crate::src::src::wal::sqlite3WalOpen(
            
            __pPager_ref.pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            
            __pPager_ref.fd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            __pPager_ref.zWal,
            __pPager_ref.exclusiveMode as ::core::ffi::c_int,
            __pPager_ref.journalSizeLimit,
            &raw mut __pPager_ref.pWal,
        );
    }
    pagerFixMaplimit(pPager);
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerOpenWal(
    mut pPager: *mut Pager,
    mut pbOpen: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pPager).tempFile == 0 && (*pPager).pWal.is_null() {
        if sqlite3PagerWalSupported(pPager) == 0 {
            return crate::src::headers::sqlite3_h::SQLITE_CANTOPEN;
        }
        crate::src::src::os::sqlite3OsClose((*pPager).jfd as *mut crate::src::headers::sqlite3_h::sqlite3_file);
        rc = pagerOpenWal(pPager);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            (*pPager).journalMode = crate::src::src::pager::PAGER_JOURNALMODE_WAL as crate::src::ext::rtree::rtree::u8_0;
            (*pPager).eState = PAGER_OPEN as crate::src::ext::rtree::rtree::u8_0;
        }
    } else {
        *pbOpen = 1 as ::core::ffi::c_int;
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PagerCloseWal(
    mut pPager: *mut Pager,
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pPager).pWal.is_null() {
        let mut logexists: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = pagerLockDb(pPager, crate::src::src::os::SHARED_LOCK);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = crate::src::src::os::sqlite3OsAccess(
                
                (*pPager).pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                (*pPager).zWal,
                crate::src::headers::sqlite3_h::SQLITE_ACCESS_EXISTS,
                &raw mut logexists,
            );
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && logexists != 0 {
            rc = pagerOpenWal(pPager);
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !(*pPager).pWal.is_null() {
        rc = pagerExclusiveLock(pPager);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let __pPager_ref = unsafe { &mut *pPager };
            rc = crate::src::src::wal::sqlite3WalClose(
                __pPager_ref.pWal,
                
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                __pPager_ref.walSyncFlags as ::core::ffi::c_int,
                __pPager_ref.pageSize as ::core::ffi::c_int,
                __pPager_ref.pTmpSpace as *mut crate::src::ext::rtree::rtree::u8_0,
            );
            __pPager_ref.pWal = ::core::ptr::null_mut::<crate::src::src::wal::Wal>();
            pagerFixMaplimit(pPager);
            if rc != 0 && __pPager_ref.exclusiveMode == 0 {
                pagerUnlockDb(pPager, crate::src::src::os::SHARED_LOCK);
            }
        }
    }
    rc
}
