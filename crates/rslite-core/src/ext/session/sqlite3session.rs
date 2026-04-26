// =============== BEGIN sqlite3session_h ================
pub const SQLITE_SESSION_OBJCONFIG_SIZE: ::core::ffi::c_int = 1;

pub const SQLITE_SESSION_OBJCONFIG_ROWID: ::core::ffi::c_int = 2;

pub const SQLITE_CHANGESETSTART_INVERT: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SQLITE_CHANGESETAPPLY_NOSAVEPOINT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_CHANGESETAPPLY_INVERT: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SQLITE_CHANGESETAPPLY_IGNORENOOP: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const SQLITE_CHANGESETAPPLY_FKNOACTION: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const SQLITE_CHANGESET_DATA: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_CHANGESET_CONFLICT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_CHANGESET_FOREIGN_KEY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_CHANGESET_OMIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_CHANGESET_REPLACE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_CHANGESET_ABORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_SESSION_CONFIG_STRMSIZE: ::core::ffi::c_int = 1;
pub use crate::__stddef_size_t_h::SizeT;
pub use crate::src::headers::stdlib::VaList;
use sqlite_printf_macros::{sqlite_printf, sqlite_snprintf};

pub use crate::internal::BuiltinVaList;
pub use crate::internal::VaListTag;
pub use crate::src::src::hash::_ht;
pub use crate::src::src::hash::Hash;
pub use crate::src::src::hash::HashElem;
pub use crate::src::src::pager::Pgno;

pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::I16_0;
pub use crate::src::fts5::U16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_ABORT;
pub use crate::src::headers::sqlite3_h::SQLITE_BLOB;
pub use crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT;
pub use crate::src::headers::sqlite3_h::SQLITE_DBSTATUS_DEFERRED_FKS;
pub use crate::src::headers::sqlite3_h::SQLITE_DELETE;
pub use crate::src::headers::sqlite3_h::SQLITE_DONE;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_FLOAT_1;
pub use crate::src::headers::sqlite3_h::SQLITE_INSERT;
pub use crate::src::headers::sqlite3_h::SQLITE_INTEGER;
pub use crate::src::headers::sqlite3_h::SQLITE_MISUSE;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_NULL_1;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_RANGE;
pub use crate::src::headers::sqlite3_h::SQLITE_ROW;
pub use crate::src::headers::sqlite3_h::SQLITE_SCHEMA;
pub use crate::src::headers::sqlite3_h::SQLITE_STATIC;
pub use crate::src::headers::sqlite3_h::SQLITE_TEXT;
pub use crate::src::headers::sqlite3_h::SQLITE_UPDATE;
pub use crate::src::headers::sqlite3_h::SQLITE_UTF8;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::Sqlite3DestructorType;
pub use crate::src::headers::sqlite3_h::sqlite3_file;
pub use crate::src::headers::sqlite3_h::Sqlite3Filename;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use crate::src::headers::sqlite3_h::sqlite3_index_info;
pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_module;
pub use crate::src::headers::sqlite3_h::Sqlite3Stmt;
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
pub use crate::src::headers::sqliteInt_h::SQLITE_FkNoAction;
pub use crate::src::headers::sqliteInt_h::Savepoint;
pub use crate::src::headers::sqliteInt_h::Schema;
pub use crate::src::headers::sqliteInt_h::Select;
pub use crate::src::headers::sqliteInt_h::SrcItem;
pub use crate::src::headers::sqliteInt_h::SrcList;
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
pub use crate::src::headers::sqliteInt_h::Sqlite3Xauth;
pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;
pub use crate::src::headers::sqliteInt_h::YDbMask;
pub use crate::src::headers::sqliteInt_h::YnVar;
pub use crate::src::headers::stdlib::Int8T;
pub use crate::src::headers::stdlib::Int16T;
pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint16T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::headers::vdbeInt_h::__anon_struct_10;
pub use crate::src::headers::vdbeInt_h::__anon_union_17;
pub use crate::src::headers::vdbeInt_h::__anon_union_18;
pub use crate::src::headers::vdbeInt_h::AuxData;
pub use crate::src::headers::vdbeInt_h::Bool;
pub use crate::src::headers::vdbeInt_h::MemValue;
pub use crate::src::headers::vdbeInt_h::Op;
pub use crate::src::headers::vdbeInt_h::PreUpdate;
pub use crate::src::headers::vdbeInt_h::Vdbe;
pub use crate::src::headers::vdbeInt_h::VdbeCursor;
pub use crate::src::headers::vdbeInt_h::VdbeFrame;
pub use crate::src::headers::vdbeInt_h::VdbeSorter;
pub use crate::src::headers::vdbeInt_h::VdbeTxtBlbCache;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::src::legacy::sqlite3_exec;
pub use crate::src::src::main::sqlite3_changes;
pub use crate::src::src::main::sqlite3_db_mutex;
pub use crate::src::src::main::sqlite3_errcode;
pub use crate::src::src::main::sqlite3_errmsg;
pub use crate::src::src::main::sqlite3_preupdate_hook;
pub use crate::src::src::main::sqlite3_set_errmsg;
pub use crate::src::src::main::sqlite3_table_column_metadata;
pub use crate::src::src::main::sqlite3CorruptError;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_malloc;
pub use crate::src::src::malloc::sqlite3_malloc64;
pub use crate::src::src::malloc::sqlite3_msize;
pub use crate::src::src::malloc::sqlite3_realloc64;
pub use crate::src::src::mutex::sqlite3_mutex_enter;
pub use crate::src::src::mutex::sqlite3_mutex_leave;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::prepare::sqlite3_prepare;
pub use crate::src::src::prepare::sqlite3_prepare_v2;
pub use crate::src::src::status::sqlite3_db_status;
pub use crate::src::src::util::sqlite3_stricmp;
pub use crate::src::src::util::sqlite3_strnicmp;
pub use crate::src::src::util::sqlite3GetVarint32;
pub use crate::src::src::util::sqlite3PutVarint;
pub use crate::src::src::util::sqlite3Strlen30;
pub use crate::src::src::util::sqlite3VarintLen;
pub use crate::src::src::vdbe::Mem;
pub use crate::src::src::vdbe::SubProgram;
pub use crate::src::src::vdbe::SubrtnSig;
pub use crate::src::src::vdbe::VdbeOp;
pub use crate::src::src::vdbe::p4union;
pub use crate::src::src::vdbeapi::sqlite3_bind_blob;
pub use crate::src::src::vdbeapi::sqlite3_bind_double;
pub use crate::src::src::vdbeapi::sqlite3_bind_int;
pub use crate::src::src::vdbeapi::sqlite3_bind_int64;
pub use crate::src::src::vdbeapi::sqlite3_bind_parameter_count;
pub use crate::src::src::vdbeapi::sqlite3_bind_text;
pub use crate::src::src::vdbeapi::sqlite3_bind_value;
pub use crate::src::src::vdbeapi::sqlite3_clear_bindings;
pub use crate::src::src::vdbeapi::sqlite3_column_blob;
pub use crate::src::src::vdbeapi::sqlite3_column_bytes;
pub use crate::src::src::vdbeapi::sqlite3_column_count;
pub use crate::src::src::vdbeapi::sqlite3_column_double;
pub use crate::src::src::vdbeapi::sqlite3_column_int;
pub use crate::src::src::vdbeapi::sqlite3_column_int64;
pub use crate::src::src::vdbeapi::sqlite3_column_text;
pub use crate::src::src::vdbeapi::sqlite3_column_type;
pub use crate::src::src::vdbeapi::sqlite3_column_value;
pub use crate::src::src::vdbeapi::sqlite3_finalize;
pub use crate::src::src::vdbeapi::sqlite3_preupdate_blobwrite;
pub use crate::src::src::vdbeapi::sqlite3_preupdate_count;
pub use crate::src::src::vdbeapi::sqlite3_preupdate_depth;
pub use crate::src::src::vdbeapi::sqlite3_preupdate_new;
pub use crate::src::src::vdbeapi::sqlite3_preupdate_old;
pub use crate::src::src::vdbeapi::sqlite3_reset;
pub use crate::src::src::vdbeapi::sqlite3_step;
pub use crate::src::src::vdbeapi::sqlite3_value_blob;
pub use crate::src::src::vdbeapi::sqlite3_value_bytes;
pub use crate::src::src::vdbeapi::sqlite3_value_double;
pub use crate::src::src::vdbeapi::sqlite3_value_int64;
pub use crate::src::src::vdbeapi::sqlite3_value_text;
pub use crate::src::src::vdbeapi::sqlite3_value_type;
pub use crate::src::src::vdbemem::sqlite3ValueFree;
pub use crate::src::src::vdbemem::sqlite3ValueNew;
pub use crate::src::src::vdbemem::sqlite3ValueSetStr;
pub use crate::src::src::vdbemem::sqlite3VdbeMemSetDouble;
pub use crate::src::src::vdbemem::sqlite3VdbeMemSetInt64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_session {
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub zDb: *mut ::core::ffi::c_char,
    pub bEnableSize: ::core::ffi::c_int,
    pub bEnable: ::core::ffi::c_int,
    pub bIndirect: ::core::ffi::c_int,
    pub bAutoAttach: ::core::ffi::c_int,
    pub bImplicitPK: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub pFilterCtx: *mut ::core::ffi::c_void,
    pub xTableFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub nMalloc: crate::src::ext::rtree::rtree::I64_0,
    pub nMaxChangesetSize: crate::src::ext::rtree::rtree::I64_0,
    pub pZeroBlob: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pub pNext: *mut sqlite3_session,
    pub pTable: *mut SessionTable,
    pub hook: SessionHook,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SessionHook {
    pub pCtx: *mut ::core::ffi::c_void,
    pub xOld: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNew: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xCount: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub xDepth: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SessionTable {
    pub pNext: *mut SessionTable,
    pub zName: *mut ::core::ffi::c_char,
    pub nCol: ::core::ffi::c_int,
    pub nTotalCol: ::core::ffi::c_int,
    pub bStat1: ::core::ffi::c_int,
    pub bRowid: ::core::ffi::c_int,
    pub azCol: *mut *const ::core::ffi::c_char,
    pub azDflt: *mut *const ::core::ffi::c_char,
    pub aiIdx: *mut ::core::ffi::c_int,
    pub abPK: *mut crate::src::ext::rtree::rtree::U8_0,
    pub nEntry: ::core::ffi::c_int,
    pub nChange: ::core::ffi::c_int,
    pub apChange: *mut *mut SessionChange,
    pub pDfltStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SessionChange {
    pub op: crate::src::ext::rtree::rtree::U8_0,
    pub bIndirect: crate::src::ext::rtree::rtree::U8_0,
    pub nRecordField: crate::src::fts5::U16_0,
    pub nMaxSize: ::core::ffi::c_int,
    pub nRecord: ::core::ffi::c_int,
    pub aRecord: *mut crate::src::ext::rtree::rtree::U8_0,
    pub pNext: *mut SessionChange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_changeset_iter {
    pub in_0: SessionInput,
    pub tblhdr: SessionBuffer,
    pub bPatchset: ::core::ffi::c_int,
    pub bInvert: ::core::ffi::c_int,
    pub bSkipEmpty: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub pConflict: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub zTab: *mut ::core::ffi::c_char,
    pub nCol: ::core::ffi::c_int,
    pub op: ::core::ffi::c_int,
    pub bIndirect: ::core::ffi::c_int,
    pub abPK: *mut crate::src::ext::rtree::rtree::U8_0,
    pub apValue: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SessionBuffer {
    pub aBuf: *mut crate::src::ext::rtree::rtree::U8_0,
    pub nBuf: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SessionInput {
    pub bNoDiscard: ::core::ffi::c_int,
    pub iCurrent: ::core::ffi::c_int,
    pub iNext: ::core::ffi::c_int,
    pub aData: *mut crate::src::ext::rtree::rtree::U8_0,
    pub nData: ::core::ffi::c_int,
    pub buf: SessionBuffer,
    pub xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pIn: *mut ::core::ffi::c_void,
    pub bEof: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SessionStat1Ctx {
    pub hook: SessionHook,
    pub pSession: *mut sqlite3_session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SessionDiffCtx {
    pub pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub bRowid: ::core::ffi::c_int,
    pub nOldOff: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_changegroup {
    pub rc: ::core::ffi::c_int,
    pub bPatch: ::core::ffi::c_int,
    pub pList: *mut SessionTable,
    pub rec: SessionBuffer,
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub zDb: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SessionApplyCtx {
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub pDelete: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub pInsert: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub pSelect: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub nCol: ::core::ffi::c_int,
    pub azCol: *mut *const ::core::ffi::c_char,
    pub abPK: *mut crate::src::ext::rtree::rtree::U8_0,
    pub aUpdateMask: *mut crate::src::ext::rtree::rtree::U32_0,
    pub pUp: *mut SessionUpdate,
    pub bStat1: ::core::ffi::c_int,
    pub bDeferConstraints: ::core::ffi::c_int,
    pub bInvertConstraints: ::core::ffi::c_int,
    pub constraints: SessionBuffer,
    pub rebase: SessionBuffer,
    pub bRebaseStarted: crate::src::ext::rtree::rtree::U8_0,
    pub bRebase: crate::src::ext::rtree::rtree::U8_0,
    pub bIgnoreNoop: crate::src::ext::rtree::rtree::U8_0,
    pub bRowid: ::core::ffi::c_int,
    pub zErr: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SessionUpdate {
    pub pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub aMask: *mut crate::src::ext::rtree::rtree::U32_0,
    pub pNext: *mut SessionUpdate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_rebaser {
    pub grp: sqlite3_changegroup,
}

pub const SESSIONS_STRM_CHUNK_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;

pub const SESSIONS_ROWID: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"_rowid_\0") };

static mut sessions_strm_chunk_size: ::core::ffi::c_int = SESSIONS_STRM_CHUNK_SIZE;

unsafe extern "C" fn sessionVarintPut(
    aBuf: *mut crate::src::ext::rtree::rtree::U8_0,
    iVal: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (if (iVal as crate::src::ext::rtree::rtree::U32_0)
        < 0x80 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
    {
        *aBuf = iVal as ::core::ffi::c_uchar as crate::src::ext::rtree::rtree::U8_0;
        1 as ::core::ffi::c_int
    } else {
        crate::src::src::util::sqlite3PutVarint(
            aBuf as *mut ::core::ffi::c_uchar,
            iVal as crate::src::ext::rtree::rtree::U64_0,
        )
    }) as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int
}

unsafe extern "C" fn sessionVarintLen(iVal: ::core::ffi::c_int) -> ::core::ffi::c_int {
    crate::src::src::util::sqlite3VarintLen(iVal as crate::src::ext::rtree::rtree::U64_0)
}

unsafe extern "C" fn sessionVarintGet(
    aBuf: *const crate::src::ext::rtree::rtree::U8_0,
    piVal: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (if (*aBuf as ::core::ffi::c_int)
        < 0x80 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int
    {
        *piVal = *aBuf as crate::src::ext::rtree::rtree::U32_0 as ::core::ffi::c_int;
        1 as ::core::ffi::c_int
    } else {
        crate::src::src::util::sqlite3GetVarint32(
            aBuf as *const ::core::ffi::c_uchar,
            piVal as *mut crate::src::ext::rtree::rtree::U32_0,
        ) as ::core::ffi::c_int
    }) as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int
}

unsafe extern "C" fn sessionGetI64(
    aRec: *mut crate::src::ext::rtree::rtree::U8_0,
) -> crate::src::headers::sqlite3_h::Sqlite3Int64 {
    let mut x: crate::src::ext::rtree::rtree::U64_0 = ((*aRec.offset(0 as isize)
        as crate::src::ext::rtree::rtree::U32_0)
        << 24 as ::core::ffi::c_int
        | ((*aRec.offset(1 as isize) as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0
        | ((*aRec.offset(2 as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0
        | *aRec.offset(3 as isize) as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::ext::rtree::rtree::U64_0;
    let y: crate::src::ext::rtree::rtree::U32_0 = (*aRec.offset(4 as isize).offset(0 as isize)
        as crate::src::ext::rtree::rtree::U32_0)
        << 24 as ::core::ffi::c_int
        | ((*aRec.offset(4 as isize).offset(1 as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U32_0
        | ((*aRec.offset(4 as isize).offset(2 as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U32_0
        | *aRec.offset(4 as isize).offset(3 as isize) as crate::src::ext::rtree::rtree::U32_0;
    x = (x << 32 as ::core::ffi::c_int).wrapping_add(y as crate::src::ext::rtree::rtree::U64_0);
    x as crate::src::headers::sqlite3_h::Sqlite3Int64
}

unsafe extern "C" fn sessionPutI64(
    aBuf: *mut crate::src::ext::rtree::rtree::U8_0,
    i: crate::src::headers::sqlite3_h::Sqlite3Int64,
) {
    *aBuf.offset(0 as isize) = (i >> 56 as ::core::ffi::c_int
        & 0xff as crate::src::headers::sqlite3_h::Sqlite3Int64)
        as crate::src::ext::rtree::rtree::U8_0;
    *aBuf.offset(1 as isize) = (i >> 48 as ::core::ffi::c_int
        & 0xff as crate::src::headers::sqlite3_h::Sqlite3Int64)
        as crate::src::ext::rtree::rtree::U8_0;
    *aBuf.offset(2 as isize) = (i >> 40 as ::core::ffi::c_int
        & 0xff as crate::src::headers::sqlite3_h::Sqlite3Int64)
        as crate::src::ext::rtree::rtree::U8_0;
    *aBuf.offset(3 as isize) = (i >> 32 as ::core::ffi::c_int
        & 0xff as crate::src::headers::sqlite3_h::Sqlite3Int64)
        as crate::src::ext::rtree::rtree::U8_0;
    *aBuf.offset(4 as isize) = (i >> 24 as ::core::ffi::c_int
        & 0xff as crate::src::headers::sqlite3_h::Sqlite3Int64)
        as crate::src::ext::rtree::rtree::U8_0;
    *aBuf.offset(5 as isize) = (i >> 16 as ::core::ffi::c_int
        & 0xff as crate::src::headers::sqlite3_h::Sqlite3Int64)
        as crate::src::ext::rtree::rtree::U8_0;
    *aBuf.offset(6 as isize) = (i >> 8 as ::core::ffi::c_int
        & 0xff as crate::src::headers::sqlite3_h::Sqlite3Int64)
        as crate::src::ext::rtree::rtree::U8_0;
    *aBuf.offset(7 as isize) = (i >> 0 as ::core::ffi::c_int
        & 0xff as crate::src::headers::sqlite3_h::Sqlite3Int64)
        as crate::src::ext::rtree::rtree::U8_0;
}

unsafe extern "C" fn sessionSerializeValue(
    aBuf: *mut crate::src::ext::rtree::rtree::U8_0,
    pValue: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pnWrite: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let nByte: ::core::ffi::c_int;
    if !pValue.is_null() {
        let eType: ::core::ffi::c_int;
        eType = crate::src::src::vdbeapi::sqlite3_value_type(
            pValue as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        );
        if !aBuf.is_null() {
            *aBuf.offset(0 as isize) = eType as crate::src::ext::rtree::rtree::U8_0;
        }
        match eType {
            crate::src::headers::sqlite3_h::SQLITE_NULL_1 => {
                nByte = 1 as ::core::ffi::c_int;
            }
            crate::src::headers::sqlite3_h::SQLITE_INTEGER
            | crate::src::headers::sqlite3_h::SQLITE_FLOAT_1 => {
                if !aBuf.is_null() {
                    let mut i: crate::src::ext::rtree::rtree::U64_0 = 0;
                    if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
                        i = crate::src::src::vdbeapi::sqlite3_value_int64(
                            pValue as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) as crate::src::ext::rtree::rtree::U64_0;
                    } else {
                        let mut r: ::core::ffi::c_double;
                        r = crate::src::src::vdbeapi::sqlite3_value_double(
                            pValue as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        );
                        ::core::ptr::copy_nonoverlapping(
                            &raw mut r as *const u8,
                            &raw mut i as *mut u8,
                            8 as usize,
                        );
                    }
                    sessionPutI64(
                        aBuf.offset(1 as isize) as *mut crate::src::ext::rtree::rtree::U8_0,
                        i as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    );
                }
                nByte = 9 as ::core::ffi::c_int;
            }
            _ => {
                let z: *mut crate::src::ext::rtree::rtree::U8_0;
                let n: ::core::ffi::c_int;
                let nVarint: ::core::ffi::c_int;
                if eType == crate::src::headers::sqlite3_h::SQLITE_TEXT {
                    z = crate::src::src::vdbeapi::sqlite3_value_text(
                        pValue as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) as *mut crate::src::ext::rtree::rtree::U8_0;
                } else {
                    z = crate::src::src::vdbeapi::sqlite3_value_blob(
                        pValue as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) as *mut crate::src::ext::rtree::rtree::U8_0;
                }
                n = crate::src::src::vdbeapi::sqlite3_value_bytes(
                    pValue as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                );
                if z.is_null()
                    && (eType != crate::src::headers::sqlite3_h::SQLITE_BLOB
                        || n > 0 as ::core::ffi::c_int)
                {
                    return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                }
                nVarint = sessionVarintLen(n);
                if !aBuf.is_null() {
                    sessionVarintPut(
                        aBuf.offset(1 as isize) as *mut crate::src::ext::rtree::rtree::U8_0,
                        n,
                    );
                    if n > 0 as ::core::ffi::c_int {
                        ::core::ptr::copy_nonoverlapping(
                            z as *const u8,
                            aBuf.offset((nVarint + 1 as ::core::ffi::c_int) as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0
                                as *mut u8,
                            n as usize,
                        );
                    }
                }
                nByte = 1 as ::core::ffi::c_int + nVarint + n;
            }
        }
    } else {
        nByte = 1 as ::core::ffi::c_int;
        if !aBuf.is_null() {
            *aBuf.offset(0 as isize) = '\0' as i32 as crate::src::ext::rtree::rtree::U8_0;
        }
    }
    if !pnWrite.is_null() {
        *pnWrite += nByte as crate::src::headers::sqlite3_h::Sqlite3Int64;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn sessionMalloc64(
    pSession: *mut sqlite3_session,
    nByte: crate::src::ext::rtree::rtree::I64_0,
) -> *mut ::core::ffi::c_void {
    let pRet: *mut ::core::ffi::c_void = crate::src::src::malloc::sqlite3_malloc64(
        nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    );
    if !pSession.is_null() {
        (*pSession).nMalloc = ((*pSession).nMalloc
            as crate::src::headers::sqlite3_h::Sqlite3Uint64)
            .wrapping_add(crate::src::src::malloc::sqlite3_msize(pRet))
            as crate::src::ext::rtree::rtree::I64_0
            as crate::src::ext::rtree::rtree::I64_0;
    }
    pRet
}

unsafe extern "C" fn sessionFree(
    pSession: *mut sqlite3_session,
    pFree: *mut ::core::ffi::c_void,
) {
    if !pSession.is_null() {
        (*pSession).nMalloc = ((*pSession).nMalloc
            as crate::src::headers::sqlite3_h::Sqlite3Uint64)
            .wrapping_sub(crate::src::src::malloc::sqlite3_msize(pFree))
            as crate::src::ext::rtree::rtree::I64_0
            as crate::src::ext::rtree::rtree::I64_0;
    }
    crate::src::src::malloc::sqlite3_free(pFree);
}

unsafe extern "C" fn sessionHashAppendI64(
    mut h: ::core::ffi::c_uint,
    i: crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_uint {
    h = h << 3 as ::core::ffi::c_int
        ^ h
        ^ (i & 0xffffffff as crate::src::ext::rtree::rtree::I64_0) as ::core::ffi::c_uint;
    h << 3 as ::core::ffi::c_int
        ^ h
        ^ (i >> 32 as ::core::ffi::c_int & 0xffffffff as crate::src::ext::rtree::rtree::I64_0)
            as ::core::ffi::c_uint
}

unsafe extern "C" fn sessionHashAppendBlob(
    mut h: ::core::ffi::c_uint,
    n: ::core::ffi::c_int,
    z: *const crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        h = h << 3 as ::core::ffi::c_int ^ h ^ *z.offset(i as isize) as ::core::ffi::c_uint;
        i += 1;
    }
    h
}

unsafe extern "C" fn sessionHashAppendType(
    h: ::core::ffi::c_uint,
    eType: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    h << 3 as ::core::ffi::c_int ^ h ^ eType as ::core::ffi::c_uint
}

unsafe extern "C" fn sessionPreupdateHash(
    pSession: *mut sqlite3_session,
    iRowid: crate::src::ext::rtree::rtree::I64_0,
    pTab: *mut SessionTable,
    bNew: ::core::ffi::c_int,
    piHash: *mut ::core::ffi::c_int,
    pbNullPK: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_int;
    if (*pTab).bRowid != 0 {
        h = sessionHashAppendI64(h, iRowid);
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < (*pTab).nCol {
            if *(*pTab).abPK.offset(i as isize) != 0 {
                let rc: ::core::ffi::c_int;
                let eType: ::core::ffi::c_int;
                let mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                    ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
                let iIdx: ::core::ffi::c_int = *(*pTab).aiIdx.offset(i as isize);
                if bNew != 0 {
                    rc = (*pSession).hook.xNew.expect("non-null function pointer")(
                        (*pSession).hook.pCtx,
                        iIdx,
                        &raw mut pVal,
                    );
                } else {
                    rc = (*pSession).hook.xOld.expect("non-null function pointer")(
                        (*pSession).hook.pCtx,
                        iIdx,
                        &raw mut pVal,
                    );
                }
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    return rc;
                }
                eType = crate::src::src::vdbeapi::sqlite3_value_type(
                    pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                );
                h = sessionHashAppendType(h, eType);
                if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER
                    || eType == crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
                {
                    let mut iVal: crate::src::ext::rtree::rtree::I64_0 = 0;
                    if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
                        iVal = crate::src::src::vdbeapi::sqlite3_value_int64(
                            pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) as crate::src::ext::rtree::rtree::I64_0;
                    } else {
                        let mut rVal: ::core::ffi::c_double =
                            crate::src::src::vdbeapi::sqlite3_value_double(
                                pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            );
                        ::core::ptr::copy_nonoverlapping(
                            &raw mut rVal as *const u8,
                            &raw mut iVal as *mut u8,
                            8 as usize,
                        );
                    }
                    h = sessionHashAppendI64(h, iVal);
                } else if eType == crate::src::headers::sqlite3_h::SQLITE_TEXT
                    || eType == crate::src::headers::sqlite3_h::SQLITE_BLOB
                {
                    let z: *const crate::src::ext::rtree::rtree::U8_0;
                    let n: ::core::ffi::c_int;
                    if eType == crate::src::headers::sqlite3_h::SQLITE_TEXT {
                        z = crate::src::src::vdbeapi::sqlite3_value_text(
                            pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) as *const crate::src::ext::rtree::rtree::U8_0;
                    } else {
                        z = crate::src::src::vdbeapi::sqlite3_value_blob(
                            pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) as *const crate::src::ext::rtree::rtree::U8_0;
                    }
                    n = crate::src::src::vdbeapi::sqlite3_value_bytes(
                        pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    );
                    if z.is_null()
                        && (eType != crate::src::headers::sqlite3_h::SQLITE_BLOB
                            || n > 0 as ::core::ffi::c_int)
                    {
                        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    }
                    h = sessionHashAppendBlob(h, n, z);
                } else {
                    *pbNullPK = 1 as ::core::ffi::c_int;
                }
            }
            i += 1;
        }
    }
    *piHash = h.wrapping_rem((*pTab).nChange as ::core::ffi::c_uint) as ::core::ffi::c_int;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn sessionSerialLen(
    a: *const crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let e: ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    e = *a as ::core::ffi::c_int;
    if e == 0 as ::core::ffi::c_int || e == 0xff as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if e == crate::src::headers::sqlite3_h::SQLITE_NULL_1 {
        return 1 as ::core::ffi::c_int;
    }
    if e == crate::src::headers::sqlite3_h::SQLITE_INTEGER
        || e == crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
    {
        return 9 as ::core::ffi::c_int;
    }
    sessionVarintGet(
        a.offset(1 as isize) as *const crate::src::ext::rtree::rtree::U8_0,
        &raw mut n,
    ) + 1 as ::core::ffi::c_int
        + n
}

unsafe extern "C" fn sessionChangeHash(
    pTab: *mut SessionTable,
    bPkOnly: ::core::ffi::c_int,
    aRecord: *mut crate::src::ext::rtree::rtree::U8_0,
    nBucket: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_int;
    let mut a: *mut crate::src::ext::rtree::rtree::U8_0 = aRecord;
    i = 0 as ::core::ffi::c_int;
    while i < (*pTab).nCol {
        let eType: ::core::ffi::c_int = *a as ::core::ffi::c_int;
        let isPK: ::core::ffi::c_int = *(*pTab).abPK.offset(i as isize) as ::core::ffi::c_int;
        if !(bPkOnly != 0 && isPK == 0 as ::core::ffi::c_int) {
            if isPK != 0 {
                a = a.offset(1);
                h = sessionHashAppendType(h, eType);
                if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER
                    || eType == crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
                {
                    h = sessionHashAppendI64(
                        h,
                        sessionGetI64(a) as crate::src::ext::rtree::rtree::I64_0,
                    );
                    a = a.offset(8 as isize);
                } else {
                    let mut n: ::core::ffi::c_int = 0;
                    a = a.offset(sessionVarintGet(a, &raw mut n) as isize);
                    h = sessionHashAppendBlob(h, n, a);
                    a = a.offset(n as isize);
                }
            } else {
                a = a.offset(sessionSerialLen(a) as isize);
            }
        }
        i += 1;
    }
    h.wrapping_rem(nBucket as ::core::ffi::c_uint)
}

unsafe extern "C" fn sessionChangeEqual(
    pTab: *mut SessionTable,
    bLeftPkOnly: ::core::ffi::c_int,
    aLeft: *mut crate::src::ext::rtree::rtree::U8_0,
    bRightPkOnly: ::core::ffi::c_int,
    aRight: *mut crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let mut a1: *mut crate::src::ext::rtree::rtree::U8_0 = aLeft;
    let mut a2: *mut crate::src::ext::rtree::rtree::U8_0 = aRight;
    let mut iCol: ::core::ffi::c_int;
    iCol = 0 as ::core::ffi::c_int;
    while iCol < (*pTab).nCol {
        if *(*pTab).abPK.offset(iCol as isize) != 0 {
            let n1: ::core::ffi::c_int = sessionSerialLen(a1);
            let n2: ::core::ffi::c_int = sessionSerialLen(a2);
            if n1 != n2
                || ::libc::memcmp(
                    a1 as *const ::core::ffi::c_void,
                    a2 as *const ::core::ffi::c_void,
                    n1 as crate::__stddef_size_t_h::SizeT,
                ) != 0
            {
                return 0 as ::core::ffi::c_int;
            }
            a1 = a1.offset(n1 as isize);
            a2 = a2.offset(n2 as isize);
        } else {
            if bLeftPkOnly == 0 as ::core::ffi::c_int {
                a1 = a1.offset(sessionSerialLen(a1) as isize);
            }
            if bRightPkOnly == 0 as ::core::ffi::c_int {
                a2 = a2.offset(sessionSerialLen(a2) as isize);
            }
        }
        iCol += 1;
    }
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn sessionMergeRecord(
    paOut: *mut *mut crate::src::ext::rtree::rtree::U8_0,
    nCol: ::core::ffi::c_int,
    aLeft: *mut crate::src::ext::rtree::rtree::U8_0,
    aRight: *mut crate::src::ext::rtree::rtree::U8_0,
) {
    let mut a1: *mut crate::src::ext::rtree::rtree::U8_0 = aLeft;
    let mut a2: *mut crate::src::ext::rtree::rtree::U8_0 = aRight;
    let mut aOut: *mut crate::src::ext::rtree::rtree::U8_0 = *paOut;
    let mut iCol: ::core::ffi::c_int;
    iCol = 0 as ::core::ffi::c_int;
    while iCol < nCol {
        let n1: ::core::ffi::c_int = sessionSerialLen(a1);
        let n2: ::core::ffi::c_int = sessionSerialLen(a2);
        if *a2 != 0 {
            ::core::ptr::copy_nonoverlapping(a2 as *const u8, aOut as *mut u8, n2 as usize);
            aOut = aOut.offset(n2 as isize);
        } else {
            ::core::ptr::copy_nonoverlapping(a1 as *const u8, aOut as *mut u8, n1 as usize);
            aOut = aOut.offset(n1 as isize);
        }
        a1 = a1.offset(n1 as isize);
        a2 = a2.offset(n2 as isize);
        iCol += 1;
    }
    *paOut = aOut;
}

unsafe extern "C" fn sessionMergeValue(
    paOne: *mut *mut crate::src::ext::rtree::rtree::U8_0,
    paTwo: *mut *mut crate::src::ext::rtree::rtree::U8_0,
    pnVal: *mut ::core::ffi::c_int,
) -> *mut crate::src::ext::rtree::rtree::U8_0 {
    let a1: *mut crate::src::ext::rtree::rtree::U8_0 = *paOne;
    let a2: *mut crate::src::ext::rtree::rtree::U8_0 = *paTwo;
    let mut pRet: *mut crate::src::ext::rtree::rtree::U8_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    let n1: ::core::ffi::c_int;
    if !a2.is_null() {
        let n2: ::core::ffi::c_int = sessionSerialLen(a2);
        if *a2 != 0 {
            *pnVal = n2;
            pRet = a2;
        }
        *paTwo = a2.offset(n2 as isize) as *mut crate::src::ext::rtree::rtree::U8_0;
    }
    n1 = sessionSerialLen(a1);
    if pRet.is_null() {
        *pnVal = n1;
        pRet = a1;
    }
    *paOne = a1.offset(n1 as isize) as *mut crate::src::ext::rtree::rtree::U8_0;
    pRet
}

unsafe extern "C" fn sessionMergeUpdate(
    paOut: *mut *mut crate::src::ext::rtree::rtree::U8_0,
    pTab: *mut SessionTable,
    bPatchset: ::core::ffi::c_int,
    aOldRecord1: *mut crate::src::ext::rtree::rtree::U8_0,
    aOldRecord2: *mut crate::src::ext::rtree::rtree::U8_0,
    aNewRecord1: *mut crate::src::ext::rtree::rtree::U8_0,
    aNewRecord2: *mut crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let mut aOld1: *mut crate::src::ext::rtree::rtree::U8_0 = aOldRecord1;
    let mut aOld2: *mut crate::src::ext::rtree::rtree::U8_0 = aOldRecord2;
    let mut aNew1: *mut crate::src::ext::rtree::rtree::U8_0 = aNewRecord1;
    let mut aNew2: *mut crate::src::ext::rtree::rtree::U8_0 = aNewRecord2;
    let mut aOut: *mut crate::src::ext::rtree::rtree::U8_0 = *paOut;
    let mut i: ::core::ffi::c_int;
    if bPatchset == 0 as ::core::ffi::c_int {
        let mut bRequired: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pTab).nCol {
            let mut nOld: ::core::ffi::c_int = 0;
            let aOld: *mut crate::src::ext::rtree::rtree::U8_0;
            let mut nNew: ::core::ffi::c_int = 0;
            let aNew: *mut crate::src::ext::rtree::rtree::U8_0;
            aOld = sessionMergeValue(&raw mut aOld1, &raw mut aOld2, &raw mut nOld);
            aNew = sessionMergeValue(&raw mut aNew1, &raw mut aNew2, &raw mut nNew);
            if *(*pTab).abPK.offset(i as isize) as ::core::ffi::c_int != 0
                || nOld != nNew
                || ::libc::memcmp(
                    aOld as *const ::core::ffi::c_void,
                    aNew as *const ::core::ffi::c_void,
                    nNew as crate::__stddef_size_t_h::SizeT,
                ) != 0
            {
                if *(*pTab).abPK.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    bRequired = 1 as ::core::ffi::c_int;
                }
                ::core::ptr::copy_nonoverlapping(aOld as *const u8, aOut as *mut u8, nOld as usize);
                aOut = aOut.offset(nOld as isize);
            } else {
                let fresh30 = aOut;
                aOut = aOut.offset(1);
                *fresh30 = '\0' as i32 as crate::src::ext::rtree::rtree::U8_0;
            }
            i += 1;
        }
        if bRequired == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    aOld1 = aOldRecord1;
    aOld2 = aOldRecord2;
    aNew1 = aNewRecord1;
    aNew2 = aNewRecord2;
    i = 0 as ::core::ffi::c_int;
    while i < (*pTab).nCol {
        let mut nOld_0: ::core::ffi::c_int = 0;
        let aOld_0: *mut crate::src::ext::rtree::rtree::U8_0;
        let mut nNew_0: ::core::ffi::c_int = 0;
        let aNew_0: *mut crate::src::ext::rtree::rtree::U8_0;
        aOld_0 = sessionMergeValue(&raw mut aOld1, &raw mut aOld2, &raw mut nOld_0);
        aNew_0 = sessionMergeValue(&raw mut aNew1, &raw mut aNew2, &raw mut nNew_0);
        if bPatchset == 0 as ::core::ffi::c_int
            && (*(*pTab).abPK.offset(i as isize) as ::core::ffi::c_int != 0
                || nOld_0 == nNew_0
                    && 0 as ::core::ffi::c_int
                        == ::libc::memcmp(
                            aOld_0 as *const ::core::ffi::c_void,
                            aNew_0 as *const ::core::ffi::c_void,
                            nNew_0 as crate::__stddef_size_t_h::SizeT,
                        ))
        {
            let fresh31 = aOut;
            aOut = aOut.offset(1);
            *fresh31 = '\0' as i32 as crate::src::ext::rtree::rtree::U8_0;
        } else {
            ::core::ptr::copy_nonoverlapping(aNew_0 as *const u8, aOut as *mut u8, nNew_0 as usize);
            aOut = aOut.offset(nNew_0 as isize);
        }
        i += 1;
    }
    *paOut = aOut;
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn sessionPreupdateEqual(
    pSession: *mut sqlite3_session,
    iRowid: crate::src::ext::rtree::rtree::I64_0,
    pTab: *mut SessionTable,
    pChange: *mut SessionChange,
    op: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut iCol: ::core::ffi::c_int;
    let mut a: *mut crate::src::ext::rtree::rtree::U8_0 = (*pChange).aRecord;
    if (*pTab).bRowid != 0 {
        if *a.offset(0 as isize) as ::core::ffi::c_int
            != crate::src::headers::sqlite3_h::SQLITE_INTEGER
        {
            return 0 as ::core::ffi::c_int;
        }
        return (sessionGetI64(a.offset(1 as isize) as *mut crate::src::ext::rtree::rtree::U8_0)
            == iRowid) as ::core::ffi::c_int;
    }
    iCol = 0 as ::core::ffi::c_int;
    while iCol < (*pTab).nCol {
        if *(*pTab).abPK.offset(iCol as isize) == 0 {
            a = a.offset(sessionSerialLen(a) as isize);
        } else {
            let mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
            let rc: ::core::ffi::c_int = 0;
            let fresh2 = a;
            a = a.offset(1);
            let eType: ::core::ffi::c_int = *fresh2 as ::core::ffi::c_int;
            let iIdx: ::core::ffi::c_int = *(*pTab).aiIdx.offset(iCol as isize);
            if op == crate::src::headers::sqlite3_h::SQLITE_INSERT {
                (*pSession).hook.xNew.expect("non-null function pointer")(
                    (*pSession).hook.pCtx,
                    iIdx,
                    &raw mut pVal,
                );
            } else {
                (*pSession).hook.xOld.expect("non-null function pointer")(
                    (*pSession).hook.pCtx,
                    iIdx,
                    &raw mut pVal,
                );
            }
            if crate::src::src::vdbeapi::sqlite3_value_type(
                pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) != eType
            {
                return 0 as ::core::ffi::c_int;
            }
            if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER
                || eType == crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
            {
                let mut iVal: crate::src::ext::rtree::rtree::I64_0 =
                    sessionGetI64(a) as crate::src::ext::rtree::rtree::I64_0;
                a = a.offset(8 as isize);
                if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
                    if crate::src::src::vdbeapi::sqlite3_value_int64(
                        pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) != iVal
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                } else {
                    let mut rVal: ::core::ffi::c_double = 0.;
                    ::core::ptr::copy_nonoverlapping(
                        &raw mut iVal as *const u8,
                        &raw mut rVal as *mut u8,
                        8 as usize,
                    );
                    if crate::src::src::vdbeapi::sqlite3_value_double(
                        pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) != rVal
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            } else {
                let mut n: ::core::ffi::c_int = 0;
                let z: *const crate::src::ext::rtree::rtree::U8_0;
                a = a.offset(sessionVarintGet(a, &raw mut n) as isize);
                if crate::src::src::vdbeapi::sqlite3_value_bytes(
                    pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) != n
                {
                    return 0 as ::core::ffi::c_int;
                }
                if eType == crate::src::headers::sqlite3_h::SQLITE_TEXT {
                    z = crate::src::src::vdbeapi::sqlite3_value_text(
                        pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) as *const crate::src::ext::rtree::rtree::U8_0;
                } else {
                    z = crate::src::src::vdbeapi::sqlite3_value_blob(
                        pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) as *const crate::src::ext::rtree::rtree::U8_0;
                }
                if n > 0 as ::core::ffi::c_int
                    && ::libc::memcmp(
                        a as *const ::core::ffi::c_void,
                        z as *const ::core::ffi::c_void,
                        n as crate::__stddef_size_t_h::SizeT,
                    ) != 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                a = a.offset(n as isize);
            }
        }
        iCol += 1;
    }
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn sessionGrowHash(
    pSession: *mut sqlite3_session,
    bPatchset: ::core::ffi::c_int,
    pTab: *mut SessionTable,
) -> ::core::ffi::c_int {
    let __pTab_ref = unsafe { &mut *pTab };
    if __pTab_ref.nChange == 0 as ::core::ffi::c_int
        || __pTab_ref.nEntry >= __pTab_ref.nChange / 2 as ::core::ffi::c_int
    {
        let mut i: ::core::ffi::c_int;
        let apNew: *mut *mut SessionChange;
        let nNew: crate::src::headers::sqlite3_h::Sqlite3Int64 = 2
            as crate::src::headers::sqlite3_h::Sqlite3Int64
            * (if __pTab_ref.nChange != 0 {
                __pTab_ref.nChange
            } else {
                128 as ::core::ffi::c_int
            }) as crate::src::headers::sqlite3_h::Sqlite3Int64;
        apNew = sessionMalloc64(
            pSession,
            (::core::mem::size_of::<*mut SessionChange>() as ::core::ffi::c_ulonglong)
                .wrapping_mul(nNew as ::core::ffi::c_ulonglong)
                as crate::src::ext::rtree::rtree::I64_0,
        ) as *mut *mut SessionChange;
        if apNew.is_null() {
            if __pTab_ref.nChange == 0 as ::core::ffi::c_int {
                return crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        ::libc::memset(
            apNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<*mut SessionChange>() as ::core::ffi::c_ulonglong)
                .wrapping_mul(nNew as ::core::ffi::c_ulonglong)
                as crate::__stddef_size_t_h::SizeT,
        );
        i = 0 as ::core::ffi::c_int;
        while i < __pTab_ref.nChange {
            let mut p: *mut SessionChange;
            let mut pNext: *mut SessionChange;
            p = *__pTab_ref.apChange.offset(i as isize);
            while !p.is_null() {
                let bPkOnly: ::core::ffi::c_int =
                    ((*p).op as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_DELETE
                        && bPatchset != 0) as ::core::ffi::c_int;
                let iHash: ::core::ffi::c_int =
                    sessionChangeHash(pTab, bPkOnly, (*p).aRecord, nNew as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                pNext = (*p).pNext;
                (*p).pNext = *apNew.offset(iHash as isize);
                let ref mut fresh3 = *apNew.offset(iHash as isize);
                *fresh3 = p;
                p = pNext;
            }
            i += 1;
        }
        sessionFree(pSession, __pTab_ref.apChange as *mut ::core::ffi::c_void);
        __pTab_ref.nChange = nNew as ::core::ffi::c_int;
        __pTab_ref.apChange = apNew;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn sessionTableInfo(
    pSession: *mut sqlite3_session,
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zDb: *const ::core::ffi::c_char,
    zThis: *const ::core::ffi::c_char,
    pnCol: *mut ::core::ffi::c_int,
    pnTotalCol: *mut ::core::ffi::c_int,
    pzTab: *mut *const ::core::ffi::c_char,
    pazCol: *mut *mut *const ::core::ffi::c_char,
    pazDflt: *mut *mut *const ::core::ffi::c_char,
    paiIdx: *mut *mut ::core::ffi::c_int,
    pabPK: *mut *mut crate::src::ext::rtree::rtree::U8_0,
    pbRowid: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let zPragma: *mut ::core::ffi::c_char;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int;
    let mut nByte: crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut nDbCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let nThis: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut pAlloc: *mut crate::src::ext::rtree::rtree::U8_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    let mut azCol: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut azDflt: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut abPK: *mut crate::src::ext::rtree::rtree::U8_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    let mut aiIdx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut bRowid: ::core::ffi::c_int;
    *pazCol = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    *pabPK = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    *pnCol = 0 as ::core::ffi::c_int;
    if !pnTotalCol.is_null() {
        *pnTotalCol = 0 as ::core::ffi::c_int;
    }
    if !paiIdx.is_null() {
        *paiIdx = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    if !pzTab.is_null() {
        *pzTab = ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !pazDflt.is_null() {
        *pazDflt = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    }
    nThis = crate::src::src::util::sqlite3Strlen30(zThis);
    if nThis == 12 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3_stricmp(
                b"sqlite_stat1\0" as *const u8 as *const ::core::ffi::c_char,
                zThis,
            )
    {
        rc = crate::src::src::main::sqlite3_table_column_metadata(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zDb,
            zThis,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            zPragma = sqlite_printf!(
                "SELECT 0, 'tbl',  '', 0, '', 1, 0     UNION ALL SELECT 1, 'idx',  '', 0, '', 2, 0     UNION ALL SELECT 2, 'stat', '', 0, '', 0, 0"
            );
        } else if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR {
            zPragma = sqlite_printf!("");
        } else {
            return rc;
        }
    } else {
        zPragma = sqlite_printf!("PRAGMA '%q'.table_xinfo('%q')", zDb, zThis);
    }
    if zPragma.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    rc = crate::src::src::prepare::sqlite3_prepare_v2(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zPragma,
        -(1 as ::core::ffi::c_int),
        &raw mut pStmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    crate::src::src::malloc::sqlite3_free(zPragma as *mut ::core::ffi::c_void);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    nByte = (nThis + 1 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::Sqlite3Int64;
    bRowid = (pbRowid != ::core::ptr::null_mut::<::core::ffi::c_int>()) as ::core::ffi::c_int;
    while crate::src::headers::sqlite3_h::SQLITE_ROW
        == crate::src::src::vdbeapi::sqlite3_step(pStmt)
    {
        nByte += crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 1 as ::core::ffi::c_int)
            as crate::src::headers::sqlite3_h::Sqlite3Int64;
        nByte += crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 4 as ::core::ffi::c_int)
            as crate::src::headers::sqlite3_h::Sqlite3Int64;
        if crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 6 as ::core::ffi::c_int)
            == 0 as ::core::ffi::c_int
        {
            nDbCol += 1;
        }
        if crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 5 as ::core::ffi::c_int) != 0 {
            bRowid = 0 as ::core::ffi::c_int;
        }
    }
    if nDbCol == 0 as ::core::ffi::c_int {
        bRowid = 0 as ::core::ffi::c_int;
    }
    nDbCol += bRowid;
    nByte = (nByte as ::core::ffi::c_ulonglong)
        .wrapping_add(::libc::strlen(SESSIONS_ROWID.as_ptr()) as ::core::ffi::c_ulonglong)
        as crate::src::headers::sqlite3_h::Sqlite3Int64
        as crate::src::headers::sqlite3_h::Sqlite3Int64;
    rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        nByte = (nByte as ::core::ffi::c_ulonglong).wrapping_add(
            (nDbCol as usize).wrapping_mul(
                (::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
                    .wrapping_mul(2 as usize)
                    .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    .wrapping_add(
                        ::core::mem::size_of::<crate::src::ext::rtree::rtree::U8_0>() as usize,
                    )
                    .wrapping_add(1 as usize)
                    .wrapping_add(1 as usize),
            ) as ::core::ffi::c_ulonglong,
        ) as crate::src::headers::sqlite3_h::Sqlite3Int64
            as crate::src::headers::sqlite3_h::Sqlite3Int64;
        pAlloc = sessionMalloc64(pSession, nByte as crate::src::ext::rtree::rtree::I64_0)
            as *mut crate::src::ext::rtree::rtree::U8_0;
        if pAlloc.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            ::libc::memset(
                pAlloc as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nByte as crate::__stddef_size_t_h::SizeT,
            );
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        azCol = pAlloc as *mut *mut ::core::ffi::c_char;
        azDflt = azCol.offset(nDbCol as isize) as *mut *mut ::core::ffi::c_char;
        aiIdx = azDflt.offset(nDbCol as isize) as *mut *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_int;
        abPK = aiIdx.offset(nDbCol as isize) as *mut ::core::ffi::c_int
            as *mut crate::src::ext::rtree::rtree::U8_0;
        pAlloc = abPK.offset(nDbCol as isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        if !pzTab.is_null() {
            ::core::ptr::copy_nonoverlapping(
                zThis as *const u8,
                pAlloc as *mut u8,
                (nThis + 1 as ::core::ffi::c_int) as usize,
            );
            *pzTab = pAlloc as *mut ::core::ffi::c_char;
            pAlloc = pAlloc.offset((nThis + 1 as ::core::ffi::c_int) as isize);
        }
        i = 0 as ::core::ffi::c_int;
        if bRowid != 0 {
            let nName: crate::__stddef_size_t_h::SizeT =
                ::libc::strlen(SESSIONS_ROWID.as_ptr());
            ::core::ptr::copy_nonoverlapping(
                SESSIONS_ROWID.as_ptr() as *const u8,
                pAlloc as *mut u8,
                (nName.wrapping_add(1 as crate::__stddef_size_t_h::SizeT)) as usize,
            );
            let ref mut fresh5 = *azCol.offset(i as isize);
            *fresh5 = pAlloc as *mut ::core::ffi::c_char;
            pAlloc =
                pAlloc.offset(nName.wrapping_add(1 as crate::__stddef_size_t_h::SizeT) as isize);
            *abPK.offset(i as isize) = 1 as crate::src::ext::rtree::rtree::U8_0;
            *aiIdx.offset(i as isize) = -(1 as ::core::ffi::c_int);
            i += 1;
        }
        while crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pStmt)
        {
            if crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 6 as ::core::ffi::c_int)
                == 0 as ::core::ffi::c_int
            {
                let nName_0: ::core::ffi::c_int =
                    crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 1 as ::core::ffi::c_int);
                let nDflt: ::core::ffi::c_int =
                    crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 4 as ::core::ffi::c_int);
                let zName: *const ::core::ffi::c_uchar =
                    crate::src::src::vdbeapi::sqlite3_column_text(pStmt, 1 as ::core::ffi::c_int);
                let zDflt: *const ::core::ffi::c_uchar =
                    crate::src::src::vdbeapi::sqlite3_column_text(pStmt, 4 as ::core::ffi::c_int);
                if zName.is_null() {
                    break;
                }
                ::core::ptr::copy_nonoverlapping(
                    zName as *const u8,
                    pAlloc as *mut u8,
                    (nName_0 + 1 as ::core::ffi::c_int) as usize,
                );
                let ref mut fresh6 = *azCol.offset(i as isize);
                *fresh6 = pAlloc as *mut ::core::ffi::c_char;
                pAlloc = pAlloc.offset((nName_0 + 1 as ::core::ffi::c_int) as isize);
                if !zDflt.is_null() {
                    ::core::ptr::copy_nonoverlapping(
                        zDflt as *const u8,
                        pAlloc as *mut u8,
                        (nDflt + 1 as ::core::ffi::c_int) as usize,
                    );
                    let ref mut fresh7 = *azDflt.offset(i as isize);
                    *fresh7 = pAlloc as *mut ::core::ffi::c_char;
                    pAlloc = pAlloc.offset((nDflt + 1 as ::core::ffi::c_int) as isize);
                } else {
                    let ref mut fresh8 = *azDflt.offset(i as isize);
                    *fresh8 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                *abPK.offset(i as isize) =
                    crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 5 as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U8_0;
                *aiIdx.offset(i as isize) =
                    crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
                i += 1;
            }
            if !pnTotalCol.is_null() {
                *pnTotalCol += 1;
            }
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        *pazCol = azCol as *mut *const ::core::ffi::c_char;
        if !pazDflt.is_null() {
            *pazDflt = azDflt as *mut *const ::core::ffi::c_char;
        }
        *pabPK = abPK;
        *pnCol = nDbCol;
        if !paiIdx.is_null() {
            *paiIdx = aiIdx;
        }
    } else {
        sessionFree(pSession, azCol as *mut ::core::ffi::c_void);
    }
    if !pbRowid.is_null() {
        *pbRowid = bRowid;
    }
    crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
    rc
}

unsafe extern "C" fn sessionInitTable(
    pSession: *mut sqlite3_session,
    pTab: *mut SessionTable,
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zDb: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pTab).nCol == 0 as ::core::ffi::c_int {
        let mut abPK: *mut crate::src::ext::rtree::rtree::U8_0 =
            ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
        let __pTab_ref = unsafe { &mut *pTab };
        crate::src::src::malloc::sqlite3_free(__pTab_ref.azCol as *mut ::core::ffi::c_void);
        __pTab_ref.abPK = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
        rc = sessionTableInfo(
            pSession,
            db,
            zDb,
            __pTab_ref.zName,
            &raw mut __pTab_ref.nCol,
            &raw mut __pTab_ref.nTotalCol,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            &raw mut __pTab_ref.azCol,
            &raw mut __pTab_ref.azDflt,
            &raw mut __pTab_ref.aiIdx,
            &raw mut abPK,
            if pSession.is_null() || (*pSession).bImplicitPK != 0 {
                &raw mut __pTab_ref.bRowid
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_int>()
            },
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let mut i: ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < __pTab_ref.nCol {
                if *abPK.offset(i as isize) != 0 {
                    __pTab_ref.abPK = abPK;
                    break;
                } else {
                    i += 1;
                }
            }
            if 0 as ::core::ffi::c_int
                == crate::src::src::util::sqlite3_stricmp(
                    b"sqlite_stat1\0" as *const u8 as *const ::core::ffi::c_char,
                    __pTab_ref.zName,
                )
            {
                __pTab_ref.bStat1 = 1 as ::core::ffi::c_int;
            }
            if !pSession.is_null() && (*pSession).bEnableSize != 0 {
                (*pSession).nMaxChangesetSize =
                    ((*pSession).nMaxChangesetSize as ::core::ffi::c_ulonglong).wrapping_add(
                        ((1 as ::core::ffi::c_int
                            + sessionVarintLen(__pTab_ref.nCol)
                            + __pTab_ref.nCol)
                            as crate::__stddef_size_t_h::SizeT)
                            .wrapping_add(::libc::strlen(__pTab_ref.zName))
                            .wrapping_add(1 as crate::__stddef_size_t_h::SizeT)
                            as ::core::ffi::c_ulonglong,
                    ) as crate::src::ext::rtree::rtree::I64_0
                        as crate::src::ext::rtree::rtree::I64_0;
            }
        }
    }
    if !pSession.is_null() {
        (*pSession).rc = rc;
        return (rc != 0 || (*pTab).abPK.is_null()) as ::core::ffi::c_int;
    }
    rc
}

unsafe extern "C" fn sessionReinitTable(
    pSession: *mut sqlite3_session,
    pTab: *mut SessionTable,
) -> ::core::ffi::c_int {
    let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nTotalCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut azCol: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut azDflt: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut aiIdx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut abPK: *mut crate::src::ext::rtree::rtree::U8_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    let mut bRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let __pSession_ref = unsafe { &mut *pSession };
    __pSession_ref.rc = sessionTableInfo(
        pSession,
        __pSession_ref.db,
        __pSession_ref.zDb,
        (*pTab).zName,
        &raw mut nCol,
        &raw mut nTotalCol,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        &raw mut azCol,
        &raw mut azDflt,
        &raw mut aiIdx,
        &raw mut abPK,
        if __pSession_ref.bImplicitPK != 0 {
            &raw mut bRowid
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_int>()
        },
    );
    if __pSession_ref.rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if (*pTab).nCol > nCol || (*pTab).bRowid != bRowid {
            __pSession_ref.rc = crate::src::headers::sqlite3_h::SQLITE_SCHEMA;
        } else {
            let mut ii: ::core::ffi::c_int;
            let nOldCol: ::core::ffi::c_int = (*pTab).nCol;
            ii = 0 as ::core::ffi::c_int;
            while ii < nCol {
                if ii < (*pTab).nCol {
                    if *(*pTab).abPK.offset(ii as isize) as ::core::ffi::c_int
                        != *abPK.offset(ii as isize) as ::core::ffi::c_int
                    {
                        __pSession_ref.rc = crate::src::headers::sqlite3_h::SQLITE_SCHEMA;
                    }
                } else if *abPK.offset(ii as isize) != 0 {
                    __pSession_ref.rc = crate::src::headers::sqlite3_h::SQLITE_SCHEMA;
                }
                ii += 1;
            }
            if __pSession_ref.rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let __pTab_ref = unsafe { &mut *pTab };
                let a: *mut *const ::core::ffi::c_char = __pTab_ref.azCol;
                __pTab_ref.azCol = azCol;
                __pTab_ref.nCol = nCol;
                __pTab_ref.nTotalCol = nTotalCol;
                __pTab_ref.azDflt = azDflt;
                __pTab_ref.abPK = abPK;
                __pTab_ref.aiIdx = aiIdx;
                azCol = a;
            }
            if __pSession_ref.bEnableSize != 0 {
                __pSession_ref.nMaxChangesetSize +=
                    (nCol - nOldCol) as crate::src::ext::rtree::rtree::I64_0;
                __pSession_ref.nMaxChangesetSize +=
                    sessionVarintLen(nCol) as crate::src::ext::rtree::rtree::I64_0;
                __pSession_ref.nMaxChangesetSize -=
                    sessionVarintLen(nOldCol) as crate::src::ext::rtree::rtree::I64_0;
            }
        }
    }
    crate::src::src::malloc::sqlite3_free(
        azCol as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
    );
    __pSession_ref.rc
}

unsafe extern "C" fn sessionUpdateOneChange(
    pSession: *mut sqlite3_session,
    pRc: *mut ::core::ffi::c_int,
    pp: *mut *mut SessionChange,
    nCol: ::core::ffi::c_int,
    pDflt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
) {
    let mut pOld: *mut SessionChange = *pp;
    while ((*pOld).nRecordField as ::core::ffi::c_int) < nCol {
        let pNew: *mut SessionChange;
        let nByte: ::core::ffi::c_int;
        let nIncr: ::core::ffi::c_int;
        let iField: ::core::ffi::c_int = (*pOld).nRecordField as ::core::ffi::c_int;
        let eType: ::core::ffi::c_int =
            crate::src::src::vdbeapi::sqlite3_column_type(pDflt, iField);
        match eType {
            crate::src::headers::sqlite3_h::SQLITE_NULL_1 => {
                nIncr = 1 as ::core::ffi::c_int;
            }
            crate::src::headers::sqlite3_h::SQLITE_INTEGER
            | crate::src::headers::sqlite3_h::SQLITE_FLOAT_1 => {
                nIncr = 9 as ::core::ffi::c_int;
            }
            _ => {
                let n: ::core::ffi::c_int =
                    crate::src::src::vdbeapi::sqlite3_column_bytes(pDflt, iField);
                nIncr = 1 as ::core::ffi::c_int + sessionVarintLen(n) + n;
            }
        }
        nByte = (nIncr as usize).wrapping_add(
            (::core::mem::size_of::<SessionChange>() as usize)
                .wrapping_add((*pOld).nRecord as usize),
        ) as ::core::ffi::c_int;
        pNew = sessionMalloc64(pSession, nByte as crate::src::ext::rtree::rtree::I64_0)
            as *mut SessionChange;
        if pNew.is_null() {
            *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            return;
        } else {
            ::core::ptr::copy_nonoverlapping(
                pOld as *const u8,
                pNew as *mut u8,
                ::core::mem::size_of::<SessionChange>() as usize,
            );
            let __pNew_ref = unsafe { &mut *pNew };
            __pNew_ref.aRecord = pNew.offset(1 as isize) as *mut SessionChange
                as *mut crate::src::ext::rtree::rtree::U8_0;
            ::core::ptr::copy_nonoverlapping(
                (*pOld).aRecord as *const u8,
                __pNew_ref.aRecord as *mut u8,
                (*pOld).nRecord as usize,
            );
            let fresh4 = __pNew_ref.nRecord;
            __pNew_ref.nRecord += 1;
            *__pNew_ref.aRecord.offset(fresh4 as isize) =
                eType as crate::src::ext::rtree::rtree::U8_0;
            match eType {
                crate::src::headers::sqlite3_h::SQLITE_INTEGER => {
                    let iVal: crate::src::ext::rtree::rtree::I64_0 =
                        crate::src::src::vdbeapi::sqlite3_column_int64(pDflt, iField)
                            as crate::src::ext::rtree::rtree::I64_0;
                    sessionPutI64(
                        __pNew_ref.aRecord.offset(__pNew_ref.nRecord as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0,
                        iVal as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    );
                    __pNew_ref.nRecord += 8 as ::core::ffi::c_int;
                }
                crate::src::headers::sqlite3_h::SQLITE_FLOAT_1 => {
                    let mut rVal: ::core::ffi::c_double =
                        crate::src::src::vdbeapi::sqlite3_column_double(pDflt, iField);
                    let mut iVal_0: crate::src::ext::rtree::rtree::I64_0 =
                        0 as crate::src::ext::rtree::rtree::I64_0;
                    ::core::ptr::copy_nonoverlapping(
                        &raw mut rVal as *const u8,
                        &raw mut iVal_0 as *mut u8,
                        ::core::mem::size_of::<::core::ffi::c_double>() as usize,
                    );
                    sessionPutI64(
                        __pNew_ref.aRecord.offset(__pNew_ref.nRecord as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0,
                        iVal_0 as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    );
                    __pNew_ref.nRecord += 8 as ::core::ffi::c_int;
                }
                crate::src::headers::sqlite3_h::SQLITE_TEXT => {
                    let n_0: ::core::ffi::c_int =
                        crate::src::src::vdbeapi::sqlite3_column_bytes(pDflt, iField);
                    let z: *const ::core::ffi::c_char =
                        crate::src::src::vdbeapi::sqlite3_column_text(pDflt, iField)
                            as *const ::core::ffi::c_char;
                    __pNew_ref.nRecord += sessionVarintPut(
                        __pNew_ref.aRecord.offset(__pNew_ref.nRecord as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0,
                        n_0,
                    );
                    ::core::ptr::copy_nonoverlapping(
                        z as *const u8,
                        __pNew_ref.aRecord.offset(__pNew_ref.nRecord as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0
                            as *mut u8,
                        n_0 as usize,
                    );
                    __pNew_ref.nRecord += n_0;
                }
                crate::src::headers::sqlite3_h::SQLITE_BLOB => {
                    let n_1: ::core::ffi::c_int =
                        crate::src::src::vdbeapi::sqlite3_column_bytes(pDflt, iField);
                    let z_0: *const crate::src::ext::rtree::rtree::U8_0 =
                        crate::src::src::vdbeapi::sqlite3_column_blob(pDflt, iField)
                            as *const crate::src::ext::rtree::rtree::U8_0;
                    __pNew_ref.nRecord += sessionVarintPut(
                        __pNew_ref.aRecord.offset(__pNew_ref.nRecord as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0,
                        n_1,
                    );
                    ::core::ptr::copy_nonoverlapping(
                        z_0 as *const u8,
                        __pNew_ref.aRecord.offset(__pNew_ref.nRecord as isize)
                            as *mut crate::src::ext::rtree::rtree::U8_0
                            as *mut u8,
                        n_1 as usize,
                    );
                    __pNew_ref.nRecord += n_1;
                }
                _ => {}
            }
            sessionFree(pSession, pOld as *mut ::core::ffi::c_void);
            pOld = pNew;
            *pp = pOld;
            __pNew_ref.nRecordField = __pNew_ref.nRecordField.wrapping_add(1);
            __pNew_ref.nMaxSize += nIncr;
            if !pSession.is_null() {
                (*pSession).nMaxChangesetSize += nIncr as crate::src::ext::rtree::rtree::I64_0;
            }
        }
    }
}

unsafe extern "C" fn sessionBufferGrow(
    p: *mut SessionBuffer,
    nByte: crate::src::ext::rtree::rtree::I64_0,
    pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let nReq: crate::src::ext::rtree::rtree::I64_0 =
        (*p).nBuf as crate::src::ext::rtree::rtree::I64_0 + nByte;
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK
        && nReq > (*p).nAlloc as crate::src::ext::rtree::rtree::I64_0
    {
        let aNew: *mut crate::src::ext::rtree::rtree::U8_0;
        let mut nNew: crate::src::ext::rtree::rtree::I64_0 = (if (*p).nAlloc != 0 {
            (*p).nAlloc
        } else {
            128 as ::core::ffi::c_int
        })
            as crate::src::ext::rtree::rtree::I64_0;
        loop {
            nNew *= 2 as crate::src::ext::rtree::rtree::I64_0;
            if !(nNew < nReq) {
                break;
            }
        }
        if nNew > SESSION_MAX_BUFFER_SZ as crate::src::ext::rtree::rtree::I64_0 {
            nNew = SESSION_MAX_BUFFER_SZ as crate::src::ext::rtree::rtree::I64_0;
            if nNew < nReq {
                *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                return 1 as ::core::ffi::c_int;
            }
        }
        aNew = crate::src::src::malloc::sqlite3_realloc64(
            (*p).aBuf as *mut ::core::ffi::c_void,
            nNew as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut crate::src::ext::rtree::rtree::U8_0;
        if aNew.is_null() {
            *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            (*p).aBuf = aNew;
            (*p).nAlloc = nNew as ::core::ffi::c_int;
        }
    }
    (*pRc != crate::src::headers::sqlite3_h::SQLITE_OK) as ::core::ffi::c_int
}

pub const SESSION_MAX_BUFFER_SZ: ::core::ffi::c_int =
    0x7fffff00 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;

pub unsafe extern "C" fn sessionAppendStr(
    p: *mut SessionBuffer,
    zStr: *const ::core::ffi::c_char,
    pRc: *mut ::core::ffi::c_int,
) {
    let nStr: ::core::ffi::c_int = crate::src::src::util::sqlite3Strlen30(zStr);
    if 0 as ::core::ffi::c_int
        == sessionBufferGrow(
            p,
            (nStr + 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::I64_0,
            pRc,
        )
    {
        let __p_ref = unsafe { &mut *p };
        ::core::ptr::copy_nonoverlapping(
            zStr as *const u8,
            __p_ref.aBuf.offset(__p_ref.nBuf as isize) as *mut crate::src::ext::rtree::rtree::U8_0
                as *mut u8,
            nStr as usize,
        );
        __p_ref.nBuf += nStr;
        *__p_ref.aBuf.offset(__p_ref.nBuf as isize) = 0 as crate::src::ext::rtree::rtree::U8_0;
    }
}

unsafe extern "C" fn sessionPrepareDfltStmt(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pTab: *mut SessionTable,
    ppStmt: *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
) -> ::core::ffi::c_int {
    let mut sql: SessionBuffer = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut zSep: *const ::core::ffi::c_char = b" \0" as *const u8 as *const ::core::ffi::c_char;
    let mut ii: ::core::ffi::c_int;
    *ppStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    sessionAppendStr(
        &raw mut sql,
        b"SELECT\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut rc,
    );
    ii = 0 as ::core::ffi::c_int;
    while ii < (*pTab).nCol {
        let zDflt: *const ::core::ffi::c_char =
            if !(*(*pTab).azDflt.offset(ii as isize)).is_null() {
                *(*pTab).azDflt.offset(ii as isize)
            } else {
                b"NULL\0" as *const u8 as *const ::core::ffi::c_char
            };
        sessionAppendStr(&raw mut sql, zSep, &raw mut rc);
        sessionAppendStr(&raw mut sql, zDflt, &raw mut rc);
        zSep = b", \0" as *const u8 as *const ::core::ffi::c_char;
        ii += 1;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::src::prepare::sqlite3_prepare_v2(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            sql.aBuf as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
            ppStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
    }
    crate::src::src::malloc::sqlite3_free(sql.aBuf as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn sessionUpdateChanges(
    pSession: *mut sqlite3_session,
    pTab: *mut SessionTable,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let __pSession_ref = unsafe { &mut *pSession };
    let mut rc: ::core::ffi::c_int;
    rc = sessionPrepareDfltStmt(__pSession_ref.db, pTab, &raw mut pStmt);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pStmt)
    {
        let mut ii: ::core::ffi::c_int;
        let mut pp: *mut *mut SessionChange;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pTab).nChange {
            pp = (*pTab).apChange.offset(ii as isize) as *mut *mut SessionChange;
            while !(*pp).is_null() {
                if (**pp).nRecordField as ::core::ffi::c_int != (*pTab).nCol {
                    sessionUpdateOneChange(pSession, &raw mut rc, pp, (*pTab).nCol, pStmt);
                }
                pp = &raw mut (**pp).pNext;
            }
            ii += 1;
        }
    }
    __pSession_ref.rc = rc;
    rc = crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
    if __pSession_ref.rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        __pSession_ref.rc = rc;
    }
    __pSession_ref.rc
}

unsafe extern "C" fn sessionStat1Old(
    pCtx: *mut ::core::ffi::c_void,
    iCol: ::core::ffi::c_int,
    ppVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let p: *mut SessionStat1Ctx = pCtx as *mut SessionStat1Ctx;
    let mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
        ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
    let rc: ::core::ffi::c_int =
        (*p).hook.xOld.expect("non-null function pointer")((*p).hook.pCtx, iCol, &raw mut pVal);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && iCol == 1 as ::core::ffi::c_int
        && crate::src::src::vdbeapi::sqlite3_value_type(
            pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) == crate::src::headers::sqlite3_h::SQLITE_NULL_1
    {
        pVal = (*(*p).pSession).pZeroBlob;
    }
    *ppVal = pVal;
    rc
}

unsafe extern "C" fn sessionStat1New(
    pCtx: *mut ::core::ffi::c_void,
    iCol: ::core::ffi::c_int,
    ppVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let p: *mut SessionStat1Ctx = pCtx as *mut SessionStat1Ctx;
    let mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
        ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
    let rc: ::core::ffi::c_int =
        (*p).hook.xNew.expect("non-null function pointer")((*p).hook.pCtx, iCol, &raw mut pVal);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && iCol == 1 as ::core::ffi::c_int
        && crate::src::src::vdbeapi::sqlite3_value_type(
            pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) == crate::src::headers::sqlite3_h::SQLITE_NULL_1
    {
        pVal = (*(*p).pSession).pZeroBlob;
    }
    *ppVal = pVal;
    rc
}

unsafe extern "C" fn sessionStat1Count(pCtx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let p: *mut SessionStat1Ctx = pCtx as *mut SessionStat1Ctx;
    (*p).hook.xCount.expect("non-null function pointer")((*p).hook.pCtx)
}

unsafe extern "C" fn sessionStat1Depth(pCtx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let p: *mut SessionStat1Ctx = pCtx as *mut SessionStat1Ctx;
    (*p).hook.xDepth.expect("non-null function pointer")((*p).hook.pCtx)
}

unsafe extern "C" fn sessionUpdateMaxSize(
    op: ::core::ffi::c_int,
    pSession: *mut sqlite3_session,
    pTab: *mut SessionTable,
    pC: *mut SessionChange,
) -> ::core::ffi::c_int {
    let mut nNew: crate::src::ext::rtree::rtree::I64_0 = 2 as crate::src::ext::rtree::rtree::I64_0;
    if (*pC).op as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_INSERT {
        if (*pTab).bRowid != 0 {
            nNew += 9 as crate::src::ext::rtree::rtree::I64_0;
        }
        if op != crate::src::headers::sqlite3_h::SQLITE_DELETE {
            let mut ii: ::core::ffi::c_int;
            ii = 0 as ::core::ffi::c_int;
            while ii < (*pTab).nCol {
                let mut p: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                    ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
                (*pSession).hook.xNew.expect("non-null function pointer")(
                    (*pSession).hook.pCtx,
                    *(*pTab).aiIdx.offset(ii as isize),
                    &raw mut p,
                );
                sessionSerializeValue(
                    ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
                    p,
                    &raw mut nNew,
                );
                ii += 1;
            }
        }
    } else if op == crate::src::headers::sqlite3_h::SQLITE_DELETE {
        nNew += (*pC).nRecord as crate::src::ext::rtree::rtree::I64_0;
        if crate::src::src::vdbeapi::sqlite3_preupdate_blobwrite(
            (*pSession).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) >= 0 as ::core::ffi::c_int
        {
            nNew += (*pC).nRecord as crate::src::ext::rtree::rtree::I64_0;
        }
    } else {
        let mut ii_0: ::core::ffi::c_int;
        let mut pCsr: *mut crate::src::ext::rtree::rtree::U8_0 = (*pC).aRecord;
        let __pTab_ref = unsafe { &mut *pTab };
        if __pTab_ref.bRowid != 0 {
            nNew += (9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::I64_0;
            pCsr = pCsr.offset(9 as isize);
        }
        ii_0 = __pTab_ref.bRowid;
        while ii_0 < __pTab_ref.nCol {
            let mut bChanged: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut nOld: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let eType: ::core::ffi::c_int;
            let iIdx: ::core::ffi::c_int = *__pTab_ref.aiIdx.offset(ii_0 as isize);
            let mut p_0: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
            (*pSession).hook.xNew.expect("non-null function pointer")(
                (*pSession).hook.pCtx,
                iIdx,
                &raw mut p_0,
            );
            if p_0.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            let fresh1 = pCsr;
            pCsr = pCsr.offset(1);
            eType = *fresh1 as ::core::ffi::c_int;
            match eType {
                crate::src::headers::sqlite3_h::SQLITE_NULL_1 => {
                    bChanged = (crate::src::src::vdbeapi::sqlite3_value_type(
                        p_0 as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) != crate::src::headers::sqlite3_h::SQLITE_NULL_1)
                        as ::core::ffi::c_int;
                }
                crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
                | crate::src::headers::sqlite3_h::SQLITE_INTEGER => {
                    if eType
                        == crate::src::src::vdbeapi::sqlite3_value_type(
                            p_0 as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        )
                    {
                        let mut iVal: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                            sessionGetI64(pCsr);
                        if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
                            bChanged = (iVal
                                != crate::src::src::vdbeapi::sqlite3_value_int64(
                                    p_0 as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                )) as ::core::ffi::c_int;
                        } else {
                            let mut dVal: ::core::ffi::c_double = 0.;
                            ::core::ptr::copy_nonoverlapping(
                                &raw mut iVal as *const u8,
                                &raw mut dVal as *mut u8,
                                8 as usize,
                            );
                            bChanged = (dVal
                                != crate::src::src::vdbeapi::sqlite3_value_double(
                                    p_0 as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                )) as ::core::ffi::c_int;
                        }
                    }
                    nOld = 8 as ::core::ffi::c_int;
                    pCsr = pCsr.offset(8 as isize);
                }
                _ => {
                    let mut nByte: ::core::ffi::c_int = 0;
                    nOld = sessionVarintGet(pCsr, &raw mut nByte);
                    pCsr = pCsr.offset(nOld as isize);
                    nOld += nByte;
                    if eType
                        == crate::src::src::vdbeapi::sqlite3_value_type(
                            p_0 as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        )
                        && nByte
                            == crate::src::src::vdbeapi::sqlite3_value_bytes(
                                p_0 as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            )
                        && (nByte == 0 as ::core::ffi::c_int
                            || 0 as ::core::ffi::c_int
                                == ::libc::memcmp(
                                    pCsr as *const ::core::ffi::c_void,
                                    crate::src::src::vdbeapi::sqlite3_value_blob(
                                        p_0 as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                    ),
                                    nByte as crate::__stddef_size_t_h::SizeT,
                                ))
                    {
                        bChanged = 0 as ::core::ffi::c_int;
                    }
                    pCsr = pCsr.offset(nByte as isize);
                }
            }
            if bChanged != 0 && *__pTab_ref.abPK.offset(ii_0 as isize) as ::core::ffi::c_int != 0 {
                nNew = ((*pC).nRecord + 2 as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::I64_0;
                break;
            } else {
                if bChanged != 0 {
                    nNew +=
                        (1 as ::core::ffi::c_int + nOld) as crate::src::ext::rtree::rtree::I64_0;
                    sessionSerializeValue(
                        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
                        p_0,
                        &raw mut nNew,
                    );
                } else if *__pTab_ref.abPK.offset(ii_0 as isize) != 0 {
                    nNew +=
                        (2 as ::core::ffi::c_int + nOld) as crate::src::ext::rtree::rtree::I64_0;
                } else {
                    nNew += 2 as crate::src::ext::rtree::rtree::I64_0;
                }
                ii_0 += 1;
            }
        }
    }
    if nNew > (*pC).nMaxSize as crate::src::ext::rtree::rtree::I64_0 {
        let nIncr: ::core::ffi::c_int =
            (nNew - (*pC).nMaxSize as crate::src::ext::rtree::rtree::I64_0) as ::core::ffi::c_int;
        (*pC).nMaxSize = nNew as ::core::ffi::c_int;
        (*pSession).nMaxChangesetSize += nIncr as crate::src::ext::rtree::rtree::I64_0;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn sessionPreupdateOneChange(
    op: ::core::ffi::c_int,
    iRowid: crate::src::ext::rtree::rtree::I64_0,
    pSession: *mut sqlite3_session,
    pTab: *mut SessionTable,
) {
    let mut current_block: u64;
    let mut iHash: ::core::ffi::c_int = 0;
    let mut bNull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let nExpect: ::core::ffi::c_int;
    let mut stat1: SessionStat1Ctx = SessionStat1Ctx {
        hook: SessionHook {
            pCtx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            xOld: None,
            xNew: None,
            xCount: None,
            xDepth: None,
        },
        pSession: ::core::ptr::null_mut::<sqlite3_session>(),
    };
    let __pSession_ref = unsafe { &mut *pSession };
    if __pSession_ref.rc != 0 {
        return;
    }
    if sessionInitTable(pSession, pTab, __pSession_ref.db, __pSession_ref.zDb) != 0 {
        return;
    }
    nExpect = __pSession_ref
        .hook
        .xCount
        .expect("non-null function pointer")(__pSession_ref.hook.pCtx);
    let __pTab_ref = unsafe { &mut *pTab };
    if __pTab_ref.nTotalCol < nExpect {
        if sessionReinitTable(pSession, pTab) != 0 {
            return;
        }
        if sessionUpdateChanges(pSession, pTab) != 0 {
            return;
        }
    }
    if __pTab_ref.nTotalCol != nExpect {
        __pSession_ref.rc = crate::src::headers::sqlite3_h::SQLITE_SCHEMA;
        return;
    }
    if sessionGrowHash(pSession, 0 as ::core::ffi::c_int, pTab) != 0 {
        __pSession_ref.rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        return;
    }
    if __pTab_ref.bStat1 != 0 {
        stat1.hook = __pSession_ref.hook;
        stat1.pSession = pSession;
        __pSession_ref.hook.pCtx = &raw mut stat1 as *mut ::core::ffi::c_void;
        __pSession_ref.hook.xNew = Some(
            sessionStat1New
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) -> ::core::ffi::c_int,
            >;
        __pSession_ref.hook.xOld = Some(
            sessionStat1Old
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) -> ::core::ffi::c_int,
            >;
        __pSession_ref.hook.xCount = Some(
            sessionStat1Count
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
        __pSession_ref.hook.xDepth = Some(
            sessionStat1Depth
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
        if __pSession_ref.pZeroBlob.is_null() {
            let p: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                crate::src::src::vdbemem::sqlite3ValueNew(::core::ptr::null_mut::<
                    crate::src::headers::sqliteInt_h::sqlite3,
                >()
                    as *mut crate::src::headers::sqliteInt_h::sqlite3)
                    as *mut crate::src::headers::vdbeInt_h::sqlite3_value;
            if p.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                current_block = 15922110044500416833;
            } else {
                crate::src::src::vdbemem::sqlite3ValueSetStr(
                    p as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    0 as ::core::ffi::c_int,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    0 as crate::src::ext::rtree::rtree::U8_0,
                    crate::src::headers::sqlite3_h::SQLITE_STATIC,
                );
                __pSession_ref.pZeroBlob = p;
                current_block = 7172762164747879670;
            }
        } else {
            current_block = 7172762164747879670;
        }
    } else {
        current_block = 7172762164747879670;
    }
    match current_block {
        7172762164747879670 => {
            rc = sessionPreupdateHash(
                pSession,
                iRowid,
                pTab,
                (op == crate::src::headers::sqlite3_h::SQLITE_INSERT) as ::core::ffi::c_int,
                &raw mut iHash,
                &raw mut bNull,
            );
            if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                if bNull == 0 as ::core::ffi::c_int {
                    let mut pC: *mut SessionChange;
                    pC = *__pTab_ref.apChange.offset(iHash as isize);
                    while !pC.is_null() {
                        if sessionPreupdateEqual(pSession, iRowid, pTab, pC, op) != 0 {
                            break;
                        }
                        pC = (*pC).pNext;
                    }
                    if pC.is_null() {
                        let mut nByte: crate::src::headers::sqlite3_h::Sqlite3Int64;
                        let mut i: ::core::ffi::c_int;
                        __pTab_ref.nEntry += 1;
                        nByte = ::core::mem::size_of::<SessionChange>()
                            as crate::src::headers::sqlite3_h::Sqlite3Int64;
                        i = __pTab_ref.bRowid;
                        loop {
                            if !(i < __pTab_ref.nCol) {
                                current_block = 313581471991351815;
                                break;
                            }
                            let iIdx: ::core::ffi::c_int = *__pTab_ref.aiIdx.offset(i as isize);
                            let mut p_0: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                                ::core::ptr::null_mut::<
                                    crate::src::headers::vdbeInt_h::sqlite3_value,
                                >();
                            if op != crate::src::headers::sqlite3_h::SQLITE_INSERT {
                                rc = __pSession_ref.hook.xOld.expect("non-null function pointer")(
                                    __pSession_ref.hook.pCtx,
                                    iIdx,
                                    &raw mut p_0,
                                );
                            } else if *__pTab_ref.abPK.offset(i as isize) != 0 {
                                __pSession_ref.hook.xNew.expect("non-null function pointer")(
                                    __pSession_ref.hook.pCtx,
                                    iIdx,
                                    &raw mut p_0,
                                );
                            }
                            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                rc = sessionSerializeValue(
                                    ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
                                    p_0,
                                    &raw mut nByte,
                                );
                            }
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                current_block = 15922110044500416833;
                                break;
                            }
                            i += 1;
                        }
                        match current_block {
                            15922110044500416833 => {}
                            _ => {
                                if __pTab_ref.bRowid != 0 {
                                    nByte += 9 as crate::src::headers::sqlite3_h::Sqlite3Int64;
                                }
                                pC = sessionMalloc64(
                                    pSession,
                                    nByte as crate::src::ext::rtree::rtree::I64_0,
                                ) as *mut SessionChange;
                                if pC.is_null() {
                                    rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                                    current_block = 15922110044500416833;
                                } else {
                                    ::libc::memset(
                                        pC as *mut ::core::ffi::c_void,
                                        0 as ::core::ffi::c_int,
                                        ::core::mem::size_of::<SessionChange>()
                                            as crate::__stddef_size_t_h::SizeT,
                                    );
                                    let __pC_ref = unsafe { &mut *pC };
                                    __pC_ref.aRecord = pC.offset(1 as isize) as *mut SessionChange
                                        as *mut crate::src::ext::rtree::rtree::U8_0;
                                    nByte = 0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
                                    if __pTab_ref.bRowid != 0 {
                                        *__pC_ref.aRecord.offset(0 as isize) =
                                            crate::src::headers::sqlite3_h::SQLITE_INTEGER
                                                as crate::src::ext::rtree::rtree::U8_0;
                                        sessionPutI64(
                                            __pC_ref.aRecord.offset(1 as isize)
                                                as *mut crate::src::ext::rtree::rtree::U8_0,
                                            iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
                                        );
                                        nByte = 9 as crate::src::headers::sqlite3_h::Sqlite3Int64;
                                    }
                                    i = __pTab_ref.bRowid;
                                    while i < __pTab_ref.nCol {
                                        let mut p_1: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                                            ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
                                        let iIdx_0: ::core::ffi::c_int =
                                            *__pTab_ref.aiIdx.offset(i as isize);
                                        if op != crate::src::headers::sqlite3_h::SQLITE_INSERT {
                                            (*pSession)
                                                .hook
                                                .xOld
                                                .expect("non-null function pointer")(
                                                __pSession_ref.hook.pCtx,
                                                iIdx_0,
                                                &raw mut p_1,
                                            );
                                        } else if *__pTab_ref.abPK.offset(i as isize) != 0 {
                                            (*pSession)
                                                .hook
                                                .xNew
                                                .expect("non-null function pointer")(
                                                __pSession_ref.hook.pCtx,
                                                iIdx_0,
                                                &raw mut p_1,
                                            );
                                        }
                                        sessionSerializeValue(
                                            __pC_ref.aRecord.offset(nByte as isize)
                                                as *mut crate::src::ext::rtree::rtree::U8_0,
                                            p_1,
                                            &raw mut nByte,
                                        );
                                        i += 1;
                                    }
                                    if __pSession_ref.bIndirect != 0
                                        || (*pSession)
                                            .hook
                                            .xDepth
                                            .expect("non-null function pointer")(
                                            __pSession_ref.hook.pCtx,
                                        ) != 0
                                    {
                                        __pC_ref.bIndirect =
                                            1 as crate::src::ext::rtree::rtree::U8_0;
                                    }
                                    __pC_ref.nRecordField =
                                        __pTab_ref.nCol as crate::src::fts5::U16_0;
                                    __pC_ref.nRecord = nByte as ::core::ffi::c_int;
                                    __pC_ref.op = op as crate::src::ext::rtree::rtree::U8_0;
                                    __pC_ref.pNext = *__pTab_ref.apChange.offset(iHash as isize);
                                    let ref mut fresh0 =
                                        *__pTab_ref.apChange.offset(iHash as isize);
                                    *fresh0 = pC;
                                    current_block = 16108440464692313034;
                                }
                            }
                        }
                    } else {
                        if (*pC).bIndirect != 0 {
                            if __pSession_ref
                                .hook
                                .xDepth
                                .expect("non-null function pointer")(
                                __pSession_ref.hook.pCtx
                            ) == 0 as ::core::ffi::c_int
                                && __pSession_ref.bIndirect == 0 as ::core::ffi::c_int
                            {
                                (*pC).bIndirect = 0 as crate::src::ext::rtree::rtree::U8_0;
                            }
                        }
                        current_block = 16108440464692313034;
                    }
                    match current_block {
                        15922110044500416833 => {}
                        _ => {
                            if __pSession_ref.bEnableSize != 0 {
                                rc = sessionUpdateMaxSize(op, pSession, pTab, pC);
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if __pTab_ref.bStat1 != 0 {
        __pSession_ref.hook = stat1.hook;
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        __pSession_ref.rc = rc;
    }
}

unsafe extern "C" fn sessionFindTable(
    pSession: *mut sqlite3_session,
    zName: *const ::core::ffi::c_char,
    ppTab: *mut *mut SessionTable,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let nName: ::core::ffi::c_int = crate::src::src::util::sqlite3Strlen30(zName);
    let mut pRet: *mut SessionTable;
    pRet = (*pSession).pTable;
    while !pRet.is_null() {
        if 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3_strnicmp(
                (*pRet).zName,
                zName,
                nName + 1 as ::core::ffi::c_int,
            )
        {
            break;
        }
        pRet = (*pRet).pNext;
    }
    if pRet.is_null() && (*pSession).bAutoAttach != 0 {
        let __pSession_ref = unsafe { &mut *pSession };
        if __pSession_ref.xTableFilter.is_none()
            || __pSession_ref
                .xTableFilter
                .expect("non-null function pointer")(__pSession_ref.pFilterCtx, zName)
                != 0
        {
            rc = sqlite3session_attach(pSession, zName);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                pRet = __pSession_ref.pTable;
                while !pRet.is_null() && !(*pRet).pNext.is_null() {
                    pRet = (*pRet).pNext;
                }
            }
        }
    }
    *ppTab = pRet;
    rc
}

unsafe extern "C" fn xPreUpdate(
    pCtx: *mut ::core::ffi::c_void,
    mut _db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    op: ::core::ffi::c_int,
    zDb: *const ::core::ffi::c_char,
    zName: *const ::core::ffi::c_char,
    iKey1: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iKey2: crate::src::headers::sqlite3_h::Sqlite3Int64,
) {
    let mut pSession: *mut sqlite3_session;
    let nDb: ::core::ffi::c_int = crate::src::src::util::sqlite3Strlen30(zDb);
    pSession = pCtx as *mut sqlite3_session;
    while !pSession.is_null() {
        let mut pTab: *mut SessionTable = ::core::ptr::null_mut::<SessionTable>();
        if !((*pSession).bEnable == 0 as ::core::ffi::c_int) {
            if !((*pSession).rc != 0) {
                if !(crate::src::src::util::sqlite3_strnicmp(
                    zDb,
                    (*pSession).zDb,
                    nDb + 1 as ::core::ffi::c_int,
                ) != 0)
                {
                    (*pSession).rc = sessionFindTable(pSession, zName, &raw mut pTab);
                    if !pTab.is_null() {
                        sessionPreupdateOneChange(
                            op,
                            iKey1 as crate::src::ext::rtree::rtree::I64_0,
                            pSession,
                            pTab,
                        );
                        if op == crate::src::headers::sqlite3_h::SQLITE_UPDATE {
                            sessionPreupdateOneChange(
                                crate::src::headers::sqlite3_h::SQLITE_INSERT,
                                iKey2 as crate::src::ext::rtree::rtree::I64_0,
                                pSession,
                                pTab,
                            );
                        }
                    }
                }
            }
        }
        pSession = (*pSession).pNext;
    }
}

unsafe extern "C" fn sessionPreupdateOld(
    pCtx: *mut ::core::ffi::c_void,
    iVal: ::core::ffi::c_int,
    ppVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    crate::src::src::vdbeapi::sqlite3_preupdate_old(
        pCtx as *mut crate::src::headers::sqliteInt_h::sqlite3
            as *mut crate::src::headers::sqliteInt_h::sqlite3,
        iVal,
        ppVal as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    )
}

unsafe extern "C" fn sessionPreupdateNew(
    pCtx: *mut ::core::ffi::c_void,
    iVal: ::core::ffi::c_int,
    ppVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    crate::src::src::vdbeapi::sqlite3_preupdate_new(
        pCtx as *mut crate::src::headers::sqliteInt_h::sqlite3
            as *mut crate::src::headers::sqliteInt_h::sqlite3,
        iVal,
        ppVal as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    )
}

unsafe extern "C" fn sessionPreupdateCount(
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    crate::src::src::vdbeapi::sqlite3_preupdate_count(
        pCtx as *mut crate::src::headers::sqliteInt_h::sqlite3
            as *mut crate::src::headers::sqliteInt_h::sqlite3,
    )
}

unsafe extern "C" fn sessionPreupdateDepth(
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    crate::src::src::vdbeapi::sqlite3_preupdate_depth(
        pCtx as *mut crate::src::headers::sqliteInt_h::sqlite3
            as *mut crate::src::headers::sqliteInt_h::sqlite3,
    )
}

unsafe extern "C" fn sessionPreupdateHooks(pSession: *mut sqlite3_session) {
    let __pSession_ref = unsafe { &mut *pSession };
    __pSession_ref.hook.pCtx = __pSession_ref.db as *mut ::core::ffi::c_void;
    __pSession_ref.hook.xOld = Some(
        sessionPreupdateOld
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
        >;
    __pSession_ref.hook.xNew = Some(
        sessionPreupdateNew
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
        >;
    __pSession_ref.hook.xCount = Some(
        sessionPreupdateCount
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    __pSession_ref.hook.xDepth = Some(
        sessionPreupdateDepth
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
}

unsafe extern "C" fn sessionDiffOld(
    pCtx: *mut ::core::ffi::c_void,
    iVal: ::core::ffi::c_int,
    ppVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let p: *mut SessionDiffCtx = pCtx as *mut SessionDiffCtx;
    let __p_ref = unsafe { &*p };
    *ppVal = crate::src::src::vdbeapi::sqlite3_column_value(
        __p_ref.pStmt,
        iVal + __p_ref.nOldOff + __p_ref.bRowid,
    ) as *mut crate::src::headers::vdbeInt_h::sqlite3_value;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn sessionDiffNew(
    pCtx: *mut ::core::ffi::c_void,
    iVal: ::core::ffi::c_int,
    ppVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let p: *mut SessionDiffCtx = pCtx as *mut SessionDiffCtx;
    *ppVal = crate::src::src::vdbeapi::sqlite3_column_value((*p).pStmt, iVal + (*p).bRowid)
        as *mut crate::src::headers::vdbeInt_h::sqlite3_value;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn sessionDiffCount(pCtx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let p: *mut SessionDiffCtx = pCtx as *mut SessionDiffCtx;
    (if (*p).nOldOff != 0 {
        (*p).nOldOff
    } else {
        crate::src::src::vdbeapi::sqlite3_column_count((*p).pStmt)
    }) - (*p).bRowid
}

unsafe extern "C" fn sessionDiffDepth(mut _pCtx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn sessionDiffHooks(
    pSession: *mut sqlite3_session,
    pDiffCtx: *mut SessionDiffCtx,
) {
    let __pSession_ref = unsafe { &mut *pSession };
    __pSession_ref.hook.pCtx = pDiffCtx as *mut ::core::ffi::c_void;
    __pSession_ref.hook.xOld = Some(
        sessionDiffOld
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
        >;
    __pSession_ref.hook.xNew = Some(
        sessionDiffNew
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> ::core::ffi::c_int,
        >;
    __pSession_ref.hook.xCount = Some(
        sessionDiffCount as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    __pSession_ref.hook.xDepth = Some(
        sessionDiffDepth as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
}

unsafe extern "C" fn sessionExprComparePK(
    nCol: ::core::ffi::c_int,
    zDb1: *const ::core::ffi::c_char,
    zDb2: *const ::core::ffi::c_char,
    zTab: *const ::core::ffi::c_char,
    azCol: *mut *const ::core::ffi::c_char,
    abPK: *mut crate::src::ext::rtree::rtree::U8_0,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int;
    let mut zSep: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    i = 0 as ::core::ffi::c_int;
    while i < nCol {
        if *abPK.offset(i as isize) != 0 {
            let zCol = *azCol.offset(i as isize);
            zRet = sqlite_printf!(
                "%z%s\"%w\".\"%w\".\"%w\"=\"%w\".\"%w\".\"%w\"",
                zRet,
                zSep,
                zDb1,
                zTab,
                zCol,
                zDb2,
                zTab,
                zCol
            );
            zSep = b" AND \0" as *const u8 as *const ::core::ffi::c_char;
            if zRet.is_null() {
                break;
            }
        }
        i += 1;
    }
    zRet
}

unsafe extern "C" fn sessionExprCompareOther(
    nCol: ::core::ffi::c_int,
    zDb1: *const ::core::ffi::c_char,
    zDb2: *const ::core::ffi::c_char,
    zTab: *const ::core::ffi::c_char,
    azCol: *mut *const ::core::ffi::c_char,
    abPK: *mut crate::src::ext::rtree::rtree::U8_0,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int;
    let mut zSep: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bHave: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nCol {
        if *abPK.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            bHave = 1 as ::core::ffi::c_int;
            let zCol = *azCol.offset(i as isize);
            zRet = sqlite_printf!(
                "%z%s\"%w\".\"%w\".\"%w\" IS NOT \"%w\".\"%w\".\"%w\"",
                zRet,
                zSep,
                zDb1,
                zTab,
                zCol,
                zDb2,
                zTab,
                zCol
            );
            zSep = b" OR \0" as *const u8 as *const ::core::ffi::c_char;
            if zRet.is_null() {
                break;
            }
        }
        i += 1;
    }
    if bHave == 0 as ::core::ffi::c_int {
        zRet = sqlite_printf!("0");
    }
    zRet
}

unsafe extern "C" fn sessionSelectFindNew(
    zDb1: *const ::core::ffi::c_char,
    zDb2: *const ::core::ffi::c_char,
    bRowid: ::core::ffi::c_int,
    zTbl: *const ::core::ffi::c_char,
    zExpr: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let zSel: *const ::core::ffi::c_char = if bRowid != 0 {
        b"_rowid_, *\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"*\0" as *const u8 as *const ::core::ffi::c_char
    };
    let zRet: *mut ::core::ffi::c_char = sqlite_printf!(
        "SELECT %s FROM \"%w\".\"%w\" WHERE NOT EXISTS (  SELECT 1 FROM \"%w\".\"%w\" WHERE %s)",
        zSel,
        zDb1,
        zTbl,
        zDb2,
        zTbl,
        zExpr,
    );
    zRet
}

unsafe extern "C" fn sessionDiffFindNew(
    op: ::core::ffi::c_int,
    pSession: *mut sqlite3_session,
    pTab: *mut SessionTable,
    zDb1: *const ::core::ffi::c_char,
    zDb2: *const ::core::ffi::c_char,
    zExpr: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let zStmt: *mut ::core::ffi::c_char =
        sessionSelectFindNew(zDb1, zDb2, (*pTab).bRowid, (*pTab).zName, zExpr);
    if zStmt.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        rc = crate::src::src::prepare::sqlite3_prepare(
            (*pSession).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zStmt,
            -(1 as ::core::ffi::c_int),
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let pDiffCtx: *mut SessionDiffCtx = (*pSession).hook.pCtx as *mut SessionDiffCtx;
            let __pDiffCtx_ref = unsafe { &mut *pDiffCtx };
            __pDiffCtx_ref.pStmt = pStmt;
            __pDiffCtx_ref.nOldOff = 0 as ::core::ffi::c_int;
            __pDiffCtx_ref.bRowid = (*pTab).bRowid;
            while crate::src::headers::sqlite3_h::SQLITE_ROW
                == crate::src::src::vdbeapi::sqlite3_step(pStmt)
            {
                let iRowid: crate::src::ext::rtree::rtree::I64_0 = if (*pTab).bRowid != 0 {
                    crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::I64_0
                } else {
                    0 as crate::src::ext::rtree::rtree::I64_0
                };
                sessionPreupdateOneChange(op, iRowid, pSession, pTab);
            }
            rc = crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
        }
        crate::src::src::malloc::sqlite3_free(zStmt as *mut ::core::ffi::c_void);
    }
    rc
}

unsafe extern "C" fn sessionAllCols(
    zDb: *const ::core::ffi::c_char,
    pTab: *mut SessionTable,
) -> *mut ::core::ffi::c_char {
    let mut ii: ::core::ffi::c_int;
    let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    ii = 0 as ::core::ffi::c_int;
    while ii < (*pTab).nCol {
        let zSep = if !zRet.is_null() {
            b", \0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        };
        let zCol = *(*pTab).azCol.offset(ii as isize);
        zRet = sqlite_printf!(
            "%z%s\"%w\".\"%w\".\"%w\"",
            zRet,
            zSep,
            zDb,
            (*pTab).zName,
            zCol
        );
        if zRet.is_null() {
            break;
        }
        ii += 1;
    }
    zRet
}

unsafe extern "C" fn sessionDiffFindModified(
    pSession: *mut sqlite3_session,
    pTab: *mut SessionTable,
    zFrom: *const ::core::ffi::c_char,
    zExpr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let __pTab_ref = unsafe { &*pTab };
    let zExpr2: *mut ::core::ffi::c_char = sessionExprCompareOther(
        __pTab_ref.nCol,
        (*pSession).zDb,
        zFrom,
        __pTab_ref.zName,
        __pTab_ref.azCol,
        __pTab_ref.abPK,
    );
    if zExpr2.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        let z1: *mut ::core::ffi::c_char = sessionAllCols((*pSession).zDb, pTab);
        let z2: *mut ::core::ffi::c_char = sessionAllCols(zFrom, pTab);
        let zStmt: *mut ::core::ffi::c_char = sqlite_printf!(
            "SELECT %s,%s FROM \"%w\".\"%w\", \"%w\".\"%w\" WHERE %s AND (%z)",
            z1,
            z2,
            (*pSession).zDb,
            __pTab_ref.zName,
            zFrom,
            __pTab_ref.zName,
            zExpr,
            zExpr2,
        );
        if zStmt.is_null() || z1.is_null() || z2.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
            rc = crate::src::src::prepare::sqlite3_prepare(
                (*pSession).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                zStmt,
                -(1 as ::core::ffi::c_int),
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let pDiffCtx: *mut SessionDiffCtx =
                    (*pSession).hook.pCtx as *mut SessionDiffCtx;
                (*pDiffCtx).pStmt = pStmt;
                (*pDiffCtx).nOldOff = __pTab_ref.nCol;
                while crate::src::headers::sqlite3_h::SQLITE_ROW
                    == crate::src::src::vdbeapi::sqlite3_step(pStmt)
                {
                    let iRowid: crate::src::ext::rtree::rtree::I64_0 = if __pTab_ref.bRowid != 0
                    {
                        crate::src::src::vdbeapi::sqlite3_column_int64(
                            pStmt,
                            0 as ::core::ffi::c_int,
                        ) as crate::src::ext::rtree::rtree::I64_0
                    } else {
                        0 as crate::src::ext::rtree::rtree::I64_0
                    };
                    sessionPreupdateOneChange(
                        crate::src::headers::sqlite3_h::SQLITE_UPDATE,
                        iRowid,
                        pSession,
                        pTab,
                    );
                }
                rc = crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
            }
        }
        crate::src::src::malloc::sqlite3_free(zStmt as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(z1 as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(z2 as *mut ::core::ffi::c_void);
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3session_diff(
    pSession: *mut sqlite3_session,
    zFrom: *const ::core::ffi::c_char,
    zTbl: *const ::core::ffi::c_char,
    pzErrMsg: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let current_block: u64;
    let __pSession_ref = unsafe { &mut *pSession };
    let zDb: *const ::core::ffi::c_char = __pSession_ref.zDb;
    let mut rc: ::core::ffi::c_int = __pSession_ref.rc;
    let mut d: SessionDiffCtx = unsafe { ::core::mem::zeroed() };
    sessionDiffHooks(pSession, &raw mut d);
    crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::main::sqlite3_db_mutex(
        __pSession_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    if !pzErrMsg.is_null() {
        *pzErrMsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut zExpr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pSession_ref.db;
        let mut pTo: *mut SessionTable = ::core::ptr::null_mut::<SessionTable>();
        __pSession_ref.bAutoAttach += 1;
        rc = sessionFindTable(pSession, zTbl, &raw mut pTo);
        __pSession_ref.bAutoAttach -= 1;
        if !pTo.is_null() {
            if sessionInitTable(pSession, pTo, __pSession_ref.db, __pSession_ref.zDb) != 0 {
                rc = __pSession_ref.rc;
            } else {
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    let mut bHasPk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut bMismatch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut bRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut abPK: *mut crate::src::ext::rtree::rtree::U8_0 =
                        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
                    let mut azCol: *mut *const ::core::ffi::c_char =
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
                    let zDbExists: *mut ::core::ffi::c_char;
                    zDbExists = sqlite_printf!("SELECT * FROM %Q.sqlite_schema", zFrom,);
                    if zDbExists.is_null() {
                        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    } else {
                        let mut pDbExists: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
                            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
                        rc = crate::src::src::prepare::sqlite3_prepare_v2(
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            zDbExists,
                            -(1 as ::core::ffi::c_int),
                            &raw mut pDbExists,
                            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                        );
                        if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR {
                            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                            nCol = -(1 as ::core::ffi::c_int);
                        }
                        crate::src::src::vdbeapi::sqlite3_finalize(pDbExists);
                        crate::src::src::malloc::sqlite3_free(
                            zDbExists as *mut ::core::ffi::c_void,
                        );
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                        && nCol == 0 as ::core::ffi::c_int
                    {
                        rc = sessionTableInfo(
                            ::core::ptr::null_mut::<sqlite3_session>(),
                            db,
                            zFrom,
                            zTbl,
                            &raw mut nCol,
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                            &raw mut azCol,
                            ::core::ptr::null_mut::<*mut *const ::core::ffi::c_char>(),
                            ::core::ptr::null_mut::<*mut ::core::ffi::c_int>(),
                            &raw mut abPK,
                            if __pSession_ref.bImplicitPK != 0 {
                                &raw mut bRowid
                            } else {
                                ::core::ptr::null_mut::<::core::ffi::c_int>()
                            },
                        );
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        if (*pTo).nCol != nCol {
                            if nCol <= 0 as ::core::ffi::c_int {
                                rc = crate::src::headers::sqlite3_h::SQLITE_SCHEMA;
                                if !pzErrMsg.is_null() {
                                    *pzErrMsg =
                                        sqlite_printf!("no such table: %s.%s", zFrom, zTbl,);
                                }
                            } else {
                                bMismatch = 1 as ::core::ffi::c_int;
                            }
                        } else {
                            let mut i: ::core::ffi::c_int;
                            i = 0 as ::core::ffi::c_int;
                            while i < nCol {
                                if *(*pTo).abPK.offset(i as isize) as ::core::ffi::c_int
                                    != *abPK.offset(i as isize) as ::core::ffi::c_int
                                {
                                    bMismatch = 1 as ::core::ffi::c_int;
                                }
                                if crate::src::src::util::sqlite3_stricmp(
                                    *azCol.offset(i as isize),
                                    *(*pTo).azCol.offset(i as isize),
                                ) != 0
                                {
                                    bMismatch = 1 as ::core::ffi::c_int;
                                }
                                if *abPK.offset(i as isize) != 0 {
                                    bHasPk = 1 as ::core::ffi::c_int;
                                }
                                i += 1;
                            }
                        }
                    }
                    crate::src::src::malloc::sqlite3_free(
                        azCol as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    );
                    if bMismatch != 0 {
                        if !pzErrMsg.is_null() {
                            *pzErrMsg = sqlite_printf!("table schemas do not match");
                        }
                        rc = crate::src::headers::sqlite3_h::SQLITE_SCHEMA;
                    }
                    if bHasPk == 0 as ::core::ffi::c_int {
                        current_block = 8342007778023407754;
                    } else {
                        current_block = 13321564401369230990;
                    }
                } else {
                    current_block = 13321564401369230990;
                }
                match current_block {
                    8342007778023407754 => {}
                    _ => {
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            let __pTo_ref = unsafe { &*pTo };
                            zExpr = sessionExprComparePK(
                                __pTo_ref.nCol,
                                zDb,
                                zFrom,
                                __pTo_ref.zName,
                                __pTo_ref.azCol,
                                __pTo_ref.abPK,
                            );
                        }
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            rc = sessionDiffFindNew(
                                crate::src::headers::sqlite3_h::SQLITE_INSERT,
                                pSession,
                                pTo,
                                zDb,
                                zFrom,
                                zExpr,
                            );
                        }
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            rc = sessionDiffFindNew(
                                crate::src::headers::sqlite3_h::SQLITE_DELETE,
                                pSession,
                                pTo,
                                zFrom,
                                zDb,
                                zExpr,
                            );
                        }
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            rc = sessionDiffFindModified(pSession, pTo, zFrom, zExpr);
                        }
                        crate::src::src::malloc::sqlite3_free(zExpr as *mut ::core::ffi::c_void);
                    }
                }
            }
        }
    }
    sessionPreupdateHooks(pSession);
    crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::main::sqlite3_db_mutex(
        __pSession_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    rc
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3session_create(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zDb: *const ::core::ffi::c_char,
    ppSession: *mut *mut sqlite3_session,
) -> ::core::ffi::c_int {
    let pNew: *mut sqlite3_session;
    let pOld: *mut sqlite3_session;
    let nDb: ::core::ffi::c_int = crate::src::src::util::sqlite3Strlen30(zDb);
    *ppSession = ::core::ptr::null_mut::<sqlite3_session>();
    pNew = crate::src::src::malloc::sqlite3_malloc64(
        (::core::mem::size_of::<sqlite3_session>() as usize)
            .wrapping_add(nDb as usize)
            .wrapping_add(1 as usize) as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut sqlite3_session;
    if pNew.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pNew as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sqlite3_session>() as crate::__stddef_size_t_h::SizeT,
    );
    (*pNew).db = db;
    (*pNew).zDb = pNew.offset(1 as isize) as *mut sqlite3_session as *mut ::core::ffi::c_char;
    (*pNew).bEnable = 1 as ::core::ffi::c_int;
    ::core::ptr::copy_nonoverlapping(
        zDb as *const u8,
        (*pNew).zDb as *mut u8,
        (nDb + 1 as ::core::ffi::c_int) as usize,
    );
    sessionPreupdateHooks(pNew);
    crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::main::sqlite3_db_mutex(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    pOld = crate::src::src::main::sqlite3_preupdate_hook(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::mem::transmute(Some(
            xPreUpdate
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut crate::src::headers::sqliteInt_h::sqlite3,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    crate::src::headers::sqlite3_h::Sqlite3Int64,
                    crate::src::headers::sqlite3_h::Sqlite3Int64,
                ) -> (),
        )),
        pNew as *mut ::core::ffi::c_void,
    ) as *mut sqlite3_session;
    (*pNew).pNext = pOld;
    crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::main::sqlite3_db_mutex(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    *ppSession = pNew;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn sessionDeleteTable(
    pSession: *mut sqlite3_session,
    pList: *mut SessionTable,
) {
    let mut pNext: *mut SessionTable;
    let mut pTab: *mut SessionTable;
    pTab = pList;
    while !pTab.is_null() {
        let mut i: ::core::ffi::c_int;
        pNext = (*pTab).pNext;
        i = 0 as ::core::ffi::c_int;
        while i < (*pTab).nChange {
            let mut p: *mut SessionChange;
            let mut pNextChange: *mut SessionChange;
            p = *(*pTab).apChange.offset(i as isize);
            while !p.is_null() {
                pNextChange = (*p).pNext;
                sessionFree(pSession, p as *mut ::core::ffi::c_void);
                p = pNextChange;
            }
            i += 1;
        }
        crate::src::src::vdbeapi::sqlite3_finalize((*pTab).pDfltStmt);
        sessionFree(
            pSession,
            (*pTab).azCol as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
        sessionFree(pSession, (*pTab).apChange as *mut ::core::ffi::c_void);
        sessionFree(pSession, pTab as *mut ::core::ffi::c_void);
        pTab = pNext;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3session_delete(pSession: *mut sqlite3_session) {
    let __pSession_ref = unsafe { &*pSession };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pSession_ref.db;
    let mut pHead: *mut sqlite3_session;
    let mut pp: *mut *mut sqlite3_session;
    crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::main::sqlite3_db_mutex(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    pHead = crate::src::src::main::sqlite3_preupdate_hook(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        None as ::std::option::Option<
            unsafe extern "C" fn(
                _: *mut ::libc::c_void,
                _: *mut crate::src::headers::sqliteInt_h::sqlite3,
                _: i32,
                _: *const i8,
                _: *const i8,
                _: i64,
                _: i64,
            ) -> (),
        >,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut sqlite3_session;
    pp = &raw mut pHead;
    while !(*pp).is_null() {
        if *pp == pSession {
            *pp = (**pp).pNext;
            if !pHead.is_null() {
                crate::src::src::main::sqlite3_preupdate_hook(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    ::core::mem::transmute(Some(
                        xPreUpdate
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut crate::src::headers::sqliteInt_h::sqlite3,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                crate::src::headers::sqlite3_h::Sqlite3Int64,
                                crate::src::headers::sqlite3_h::Sqlite3Int64,
                            ) -> (),
                    )),
                    pHead as *mut ::core::ffi::c_void,
                );
            }
            break;
        } else {
            pp = &raw mut (**pp).pNext;
        }
    }
    crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::main::sqlite3_db_mutex(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    crate::src::src::vdbemem::sqlite3ValueFree(
        __pSession_ref.pZeroBlob as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    );
    sessionDeleteTable(pSession, __pSession_ref.pTable);
    crate::src::src::malloc::sqlite3_free(pSession as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3session_table_filter(
    pSession: *mut sqlite3_session,
    xFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
) {
    let __pSession_ref = unsafe { &mut *pSession };
    __pSession_ref.bAutoAttach = 1 as ::core::ffi::c_int;
    __pSession_ref.pFilterCtx = pCtx;
    __pSession_ref.xTableFilter = xFilter;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3session_attach(
    pSession: *mut sqlite3_session,
    zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::main::sqlite3_db_mutex(
        (*pSession).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    if zName.is_null() {
        (*pSession).bAutoAttach = 1 as ::core::ffi::c_int;
    } else {
        let mut pTab: *mut SessionTable;
        let nName: ::core::ffi::c_int;
        nName = crate::src::src::util::sqlite3Strlen30(zName);
        pTab = (*pSession).pTable;
        while !pTab.is_null() {
            if 0 as ::core::ffi::c_int
                == crate::src::src::util::sqlite3_strnicmp(
                    (*pTab).zName,
                    zName,
                    nName + 1 as ::core::ffi::c_int,
                )
            {
                break;
            }
            pTab = (*pTab).pNext;
        }
        if pTab.is_null() {
            let nByte: ::core::ffi::c_int = (::core::mem::size_of::<SessionTable>() as usize)
                .wrapping_add(nName as usize)
                .wrapping_add(1 as usize)
                as ::core::ffi::c_int;
            pTab = sessionMalloc64(pSession, nByte as crate::src::ext::rtree::rtree::I64_0)
                as *mut SessionTable;
            if pTab.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                let mut ppTab: *mut *mut SessionTable;
                ::libc::memset(
                    pTab as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<SessionTable>() as crate::__stddef_size_t_h::SizeT,
                );
                (*pTab).zName =
                    pTab.offset(1 as isize) as *mut SessionTable as *mut ::core::ffi::c_char;
                ::core::ptr::copy_nonoverlapping(
                    zName as *const u8,
                    (*pTab).zName as *mut u8,
                    (nName + 1 as ::core::ffi::c_int) as usize,
                );
                ppTab = &raw mut (*pSession).pTable;
                while !(*ppTab).is_null() {
                    ppTab = &raw mut (**ppTab).pNext;
                }
                *ppTab = pTab;
            }
        }
    }
    crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::main::sqlite3_db_mutex(
        (*pSession).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    rc
}

unsafe extern "C" fn sessionAppendValue(
    p: *mut SessionBuffer,
    pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pRc: *mut ::core::ffi::c_int,
) {
    let mut rc: ::core::ffi::c_int = *pRc;
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        rc = sessionSerializeValue(
            ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
            pVal,
            &raw mut nByte,
        );
        sessionBufferGrow(
            p,
            nByte as crate::src::ext::rtree::rtree::I64_0,
            &raw mut rc,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let __p_ref = unsafe { &mut *p };
            sessionSerializeValue(
                __p_ref.aBuf.offset(__p_ref.nBuf as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0,
                pVal,
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Int64>(),
            );
            __p_ref.nBuf = (__p_ref.nBuf as crate::src::headers::sqlite3_h::Sqlite3Int64 + nByte)
                as ::core::ffi::c_int;
        } else {
            *pRc = rc;
        }
    }
}

unsafe extern "C" fn sessionAppendByte(
    p: *mut SessionBuffer,
    v: crate::src::ext::rtree::rtree::U8_0,
    pRc: *mut ::core::ffi::c_int,
) {
    if 0 as ::core::ffi::c_int
        == sessionBufferGrow(p, 1 as crate::src::ext::rtree::rtree::I64_0, pRc)
    {
        let __p_ref = unsafe { &mut *p };
        let fresh10 = __p_ref.nBuf;
        __p_ref.nBuf += 1;
        *__p_ref.aBuf.offset(fresh10 as isize) = v;
    }
}

unsafe extern "C" fn sessionAppendVarint(
    p: *mut SessionBuffer,
    v: ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) {
    if 0 as ::core::ffi::c_int
        == sessionBufferGrow(p, 9 as crate::src::ext::rtree::rtree::I64_0, pRc)
    {
        let __p_ref = unsafe { &mut *p };
        __p_ref.nBuf += sessionVarintPut(
            __p_ref.aBuf.offset(__p_ref.nBuf as isize) as *mut crate::src::ext::rtree::rtree::U8_0,
            v,
        );
    }
}

unsafe extern "C" fn sessionAppendBlob(
    p: *mut SessionBuffer,
    aBlob: *const crate::src::ext::rtree::rtree::U8_0,
    nBlob: ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) {
    if nBlob > 0 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == sessionBufferGrow(p, nBlob as crate::src::ext::rtree::rtree::I64_0, pRc)
    {
        let __p_ref = unsafe { &mut *p };
        ::core::ptr::copy_nonoverlapping(
            aBlob as *const u8,
            __p_ref.aBuf.offset(__p_ref.nBuf as isize) as *mut crate::src::ext::rtree::rtree::U8_0
                as *mut u8,
            nBlob as usize,
        );
        __p_ref.nBuf += nBlob;
    }
}

unsafe extern "C" fn sessionAppendInteger(
    p: *mut SessionBuffer,
    iVal: ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) {
    let mut aBuf: [::core::ffi::c_char; 24] = [0; 24];
    let size = (::core::mem::size_of::<[::core::ffi::c_char; 24]>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_int;
    sqlite_snprintf!(&raw mut aBuf as *mut ::core::ffi::c_char, size, "%d", iVal);
    sessionAppendStr(p, &raw mut aBuf as *mut ::core::ffi::c_char, pRc);
}

unsafe extern "C" fn sessionAppendIdent(
    p: *mut SessionBuffer,
    zStr: *const ::core::ffi::c_char,
    pRc: *mut ::core::ffi::c_int,
) {
    let nStr: ::core::ffi::c_int = crate::src::src::util::sqlite3Strlen30(zStr)
        * 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int
        == sessionBufferGrow(p, nStr as crate::src::ext::rtree::rtree::I64_0, pRc)
    {
        let __p_ref = unsafe { &mut *p };
        let mut zOut: *mut ::core::ffi::c_char = __p_ref.aBuf.offset(__p_ref.nBuf as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0
            as *mut ::core::ffi::c_char;
        let mut zIn: *const ::core::ffi::c_char = zStr;
        let fresh12 = zOut;
        zOut = zOut.offset(1);
        *fresh12 = '"' as i32 as ::core::ffi::c_char;
        if !zIn.is_null() {
            while *zIn != 0 {
                if *zIn as ::core::ffi::c_int == '"' as i32 {
                    let fresh13 = zOut;
                    zOut = zOut.offset(1);
                    *fresh13 = '"' as i32 as ::core::ffi::c_char;
                }
                let fresh14 = zIn;
                zIn = zIn.offset(1);
                let fresh15 = zOut;
                zOut = zOut.offset(1);
                *fresh15 = *fresh14;
            }
        }
        let fresh16 = zOut;
        zOut = zOut.offset(1);
        *fresh16 = '"' as i32 as ::core::ffi::c_char;
        __p_ref.nBuf = (zOut as *mut crate::src::ext::rtree::rtree::U8_0).offset_from(__p_ref.aBuf)
            as ::core::ffi::c_long as ::core::ffi::c_int;
        *__p_ref.aBuf.offset(__p_ref.nBuf as isize) = 0 as crate::src::ext::rtree::rtree::U8_0;
    }
}

unsafe extern "C" fn sessionAppendCol(
    p: *mut SessionBuffer,
    pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    iCol: ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) {
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let eType: ::core::ffi::c_int =
            crate::src::src::vdbeapi::sqlite3_column_type(pStmt, iCol);
        sessionAppendByte(p, eType as crate::src::ext::rtree::rtree::U8_0, pRc);
        if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER
            || eType == crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
        {
            let mut i: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
            let mut aBuf: [crate::src::ext::rtree::rtree::U8_0; 8] = [0; 8];
            if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
                i = crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, iCol);
            } else {
                let mut r: ::core::ffi::c_double =
                    crate::src::src::vdbeapi::sqlite3_column_double(pStmt, iCol);
                ::core::ptr::copy_nonoverlapping(
                    &raw mut r as *const u8,
                    &raw mut i as *mut u8,
                    8 as usize,
                );
            }
            sessionPutI64(&raw mut aBuf as *mut crate::src::ext::rtree::rtree::U8_0, i);
            sessionAppendBlob(
                p,
                &raw mut aBuf as *mut crate::src::ext::rtree::rtree::U8_0,
                8 as ::core::ffi::c_int,
                pRc,
            );
        }
        if eType == crate::src::headers::sqlite3_h::SQLITE_BLOB
            || eType == crate::src::headers::sqlite3_h::SQLITE_TEXT
        {
            let z: *mut crate::src::ext::rtree::rtree::U8_0;
            let nByte: ::core::ffi::c_int;
            if eType == crate::src::headers::sqlite3_h::SQLITE_BLOB {
                z = crate::src::src::vdbeapi::sqlite3_column_blob(pStmt, iCol)
                    as *mut crate::src::ext::rtree::rtree::U8_0;
            } else {
                z = crate::src::src::vdbeapi::sqlite3_column_text(pStmt, iCol)
                    as *mut crate::src::ext::rtree::rtree::U8_0;
            }
            nByte = crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, iCol);
            if !z.is_null()
                || eType == crate::src::headers::sqlite3_h::SQLITE_BLOB
                    && nByte == 0 as ::core::ffi::c_int
            {
                sessionAppendVarint(p, nByte, pRc);
                sessionAppendBlob(p, z, nByte, pRc);
            } else {
                *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
        }
    }
}

unsafe extern "C" fn sessionAppendUpdate(
    pBuf: *mut SessionBuffer,
    bPatchset: ::core::ffi::c_int,
    pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    p: *mut SessionChange,
    abPK: *mut crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut buf2: SessionBuffer = unsafe { ::core::mem::zeroed() };
    let mut bNoop: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let nRewind: ::core::ffi::c_int = (*pBuf).nBuf;
    let mut i: ::core::ffi::c_int;
    let mut pCsr: *mut crate::src::ext::rtree::rtree::U8_0 = (*p).aRecord;
    sessionAppendByte(
        pBuf,
        crate::src::headers::sqlite3_h::SQLITE_UPDATE as crate::src::ext::rtree::rtree::U8_0,
        &raw mut rc,
    );
    sessionAppendByte(pBuf, (*p).bIndirect, &raw mut rc);
    i = 0 as ::core::ffi::c_int;
    while i < crate::src::src::vdbeapi::sqlite3_column_count(pStmt) {
        let mut bChanged: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let nAdvance: ::core::ffi::c_int;
        let eType: ::core::ffi::c_int = *pCsr as ::core::ffi::c_int;
        let current_block_13: u64;
        match eType {
            crate::src::headers::sqlite3_h::SQLITE_NULL_1 => {
                nAdvance = 1 as ::core::ffi::c_int;
                if crate::src::src::vdbeapi::sqlite3_column_type(pStmt, i)
                    != crate::src::headers::sqlite3_h::SQLITE_NULL_1
                {
                    bChanged = 1 as ::core::ffi::c_int;
                }
            }
            crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
            | crate::src::headers::sqlite3_h::SQLITE_INTEGER => {
                nAdvance = 9 as ::core::ffi::c_int;
                if eType == crate::src::src::vdbeapi::sqlite3_column_type(pStmt, i) {
                    let mut iVal: crate::src::headers::sqlite3_h::Sqlite3Int64 = sessionGetI64(
                        pCsr.offset(1 as isize) as *mut crate::src::ext::rtree::rtree::U8_0,
                    );
                    if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
                        if iVal == crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, i) {
                            current_block_13 = 4068382217303356765;
                        } else {
                            current_block_13 = 13242334135786603907;
                        }
                    } else {
                        let mut dVal: ::core::ffi::c_double = 0.;
                        ::core::ptr::copy_nonoverlapping(
                            &raw mut iVal as *const u8,
                            &raw mut dVal as *mut u8,
                            8 as usize,
                        );
                        if dVal == crate::src::src::vdbeapi::sqlite3_column_double(pStmt, i) {
                            current_block_13 = 4068382217303356765;
                        } else {
                            current_block_13 = 13242334135786603907;
                        }
                    }
                } else {
                    current_block_13 = 13242334135786603907;
                }
                match current_block_13 {
                    4068382217303356765 => {}
                    _ => {
                        bChanged = 1 as ::core::ffi::c_int;
                    }
                }
            }
            _ => {
                let mut n: ::core::ffi::c_int = 0;
                let nHdr: ::core::ffi::c_int = 1 as ::core::ffi::c_int
                    + sessionVarintGet(
                        pCsr.offset(1 as isize) as *mut crate::src::ext::rtree::rtree::U8_0,
                        &raw mut n,
                    );
                nAdvance = nHdr + n;
                if !(eType == crate::src::src::vdbeapi::sqlite3_column_type(pStmt, i)
                    && n == crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, i)
                    && (n == 0 as ::core::ffi::c_int
                        || 0 as ::core::ffi::c_int
                            == ::libc::memcmp(
                                pCsr.offset(nHdr as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0
                                    as *const ::core::ffi::c_void,
                                crate::src::src::vdbeapi::sqlite3_column_blob(pStmt, i),
                                n as crate::__stddef_size_t_h::SizeT,
                            )))
                {
                    bChanged = 1 as ::core::ffi::c_int;
                }
            }
        }
        if bChanged != 0 {
            bNoop = 0 as ::core::ffi::c_int;
        }
        if bPatchset == 0 as ::core::ffi::c_int {
            if bChanged != 0 || *abPK.offset(i as isize) as ::core::ffi::c_int != 0 {
                sessionAppendBlob(pBuf, pCsr, nAdvance, &raw mut rc);
            } else {
                sessionAppendByte(pBuf, 0 as crate::src::ext::rtree::rtree::U8_0, &raw mut rc);
            }
        }
        if bChanged != 0 || bPatchset != 0 && *abPK.offset(i as isize) as ::core::ffi::c_int != 0 {
            sessionAppendCol(&raw mut buf2, pStmt, i, &raw mut rc);
        } else {
            sessionAppendByte(
                &raw mut buf2,
                0 as crate::src::ext::rtree::rtree::U8_0,
                &raw mut rc,
            );
        }
        pCsr = pCsr.offset(nAdvance as isize);
        i += 1;
    }
    if bNoop != 0 {
        (*pBuf).nBuf = nRewind;
    } else {
        sessionAppendBlob(pBuf, buf2.aBuf, buf2.nBuf, &raw mut rc);
    }
    crate::src::src::malloc::sqlite3_free(buf2.aBuf as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn sessionAppendDelete(
    pBuf: *mut SessionBuffer,
    bPatchset: ::core::ffi::c_int,
    p: *mut SessionChange,
    nCol: ::core::ffi::c_int,
    abPK: *mut crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    sessionAppendByte(
        pBuf,
        crate::src::headers::sqlite3_h::SQLITE_DELETE as crate::src::ext::rtree::rtree::U8_0,
        &raw mut rc,
    );
    sessionAppendByte(pBuf, (*p).bIndirect, &raw mut rc);
    if bPatchset == 0 as ::core::ffi::c_int {
        sessionAppendBlob(pBuf, (*p).aRecord, (*p).nRecord, &raw mut rc);
    } else {
        let mut i: ::core::ffi::c_int;
        let mut a: *mut crate::src::ext::rtree::rtree::U8_0 = (*p).aRecord;
        i = 0 as ::core::ffi::c_int;
        while i < nCol {
            let pStart: *mut crate::src::ext::rtree::rtree::U8_0 = a;
            let fresh9 = a;
            a = a.offset(1);
            let eType: ::core::ffi::c_int = *fresh9 as ::core::ffi::c_int;
            match eType {
                0 | crate::src::headers::sqlite3_h::SQLITE_NULL_1 => {}
                crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
                | crate::src::headers::sqlite3_h::SQLITE_INTEGER => {
                    a = a.offset(8 as isize);
                }
                _ => {
                    let mut n: ::core::ffi::c_int = 0;
                    a = a.offset(sessionVarintGet(a, &raw mut n) as isize);
                    a = a.offset(n as isize);
                }
            }
            if *abPK.offset(i as isize) != 0 {
                sessionAppendBlob(
                    pBuf,
                    pStart,
                    a.offset_from(pStart) as ::core::ffi::c_long as ::core::ffi::c_int,
                    &raw mut rc,
                );
            }
            i += 1;
        }
    }
    rc
}

unsafe extern "C" fn sessionPrepare(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pp: *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pzErrmsg: *mut *mut ::core::ffi::c_char,
    zSql: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int = crate::src::src::prepare::sqlite3_prepare_v2(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zSql,
        -(1 as ::core::ffi::c_int),
        pp,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if !pzErrmsg.is_null() && rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        *pzErrmsg = sqlite_printf!(
            "%s",
            crate::src::src::main::sqlite3_errmsg(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3
            ),
        );
    }
    rc
}

unsafe extern "C" fn sessionSelectStmt(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    bIgnoreNoop: ::core::ffi::c_int,
    zDb: *const ::core::ffi::c_char,
    zTab: *const ::core::ffi::c_char,
    mut _bRowid: ::core::ffi::c_int,
    nCol: ::core::ffi::c_int,
    azCol: *mut *const ::core::ffi::c_char,
    abPK: *mut crate::src::ext::rtree::rtree::U8_0,
    ppStmt: *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pzErrmsg: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zSep: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int;
    let mut cols: SessionBuffer = unsafe { ::core::mem::zeroed() };
    let mut nooptest: SessionBuffer = unsafe { ::core::mem::zeroed() };
    let mut pkfield: SessionBuffer = unsafe { ::core::mem::zeroed() };
    let mut pkvar: SessionBuffer = unsafe { ::core::mem::zeroed() };
    sessionAppendStr(
        &raw mut nooptest,
        b", 1\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut rc,
    );
    if 0 as ::core::ffi::c_int
        == crate::src::src::util::sqlite3_stricmp(
            b"sqlite_stat1\0" as *const u8 as *const ::core::ffi::c_char,
            zTab,
        )
    {
        sessionAppendStr(
            &raw mut nooptest,
            b" AND (?6 OR ?3 IS stat)\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut rc,
        );
        sessionAppendStr(
            &raw mut pkfield,
            b"tbl, idx\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut rc,
        );
        sessionAppendStr(
            &raw mut pkvar,
            b"?1, (CASE WHEN ?2=X'' THEN NULL ELSE ?2 END)\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut rc,
        );
        sessionAppendStr(
            &raw mut cols,
            b"tbl, ?2, stat\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut rc,
        );
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < nCol {
            if cols.nBuf != 0 {
                sessionAppendStr(
                    &raw mut cols,
                    b", \0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
            }
            sessionAppendIdent(&raw mut cols, *azCol.offset(i as isize), &raw mut rc);
            if *abPK.offset(i as isize) != 0 {
                sessionAppendStr(&raw mut pkfield, zSep, &raw mut rc);
                sessionAppendStr(&raw mut pkvar, zSep, &raw mut rc);
                zSep = b", \0" as *const u8 as *const ::core::ffi::c_char;
                sessionAppendIdent(&raw mut pkfield, *azCol.offset(i as isize), &raw mut rc);
                sessionAppendStr(
                    &raw mut pkvar,
                    b"?\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
                sessionAppendInteger(&raw mut pkvar, i + 1 as ::core::ffi::c_int, &raw mut rc);
            } else {
                sessionAppendStr(
                    &raw mut nooptest,
                    b" AND (?\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
                sessionAppendInteger(
                    &raw mut nooptest,
                    i + 1 as ::core::ffi::c_int + nCol,
                    &raw mut rc,
                );
                sessionAppendStr(
                    &raw mut nooptest,
                    b" OR ?\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
                sessionAppendInteger(&raw mut nooptest, i + 1 as ::core::ffi::c_int, &raw mut rc);
                sessionAppendStr(
                    &raw mut nooptest,
                    b" IS \0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
                sessionAppendIdent(&raw mut nooptest, zTab, &raw mut rc);
                sessionAppendStr(
                    &raw mut nooptest,
                    b".\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
                sessionAppendIdent(&raw mut nooptest, *azCol.offset(i as isize), &raw mut rc);
                sessionAppendStr(
                    &raw mut nooptest,
                    b")\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
            }
            i += 1;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        zSql = sqlite_printf!(
            "SELECT %s%s FROM %Q.%Q WHERE (%s) IS (%s)",
            (cols.aBuf) as *mut ::core::ffi::c_char,
            if bIgnoreNoop != 0 {
                (nooptest.aBuf) as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            zDb,
            zTab,
            (pkfield.aBuf) as *mut ::core::ffi::c_char,
            (pkvar.aBuf) as *mut ::core::ffi::c_char,
        );
        if zSql.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionPrepare(db, ppStmt, pzErrmsg, zSql);
    }
    crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(nooptest.aBuf as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(pkfield.aBuf as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(pkvar.aBuf as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(cols.aBuf as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn sessionSelectBind(
    pSelect: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    nCol: ::core::ffi::c_int,
    abPK: *mut crate::src::ext::rtree::rtree::U8_0,
    pChange: *mut SessionChange,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut a: *mut crate::src::ext::rtree::rtree::U8_0 = (*pChange).aRecord;
    i = 0 as ::core::ffi::c_int;
    while i < nCol && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let fresh11 = a;
        a = a.offset(1);
        let eType: ::core::ffi::c_int = *fresh11 as ::core::ffi::c_int;
        match eType {
            0 | crate::src::headers::sqlite3_h::SQLITE_NULL_1 => {}
            crate::src::headers::sqlite3_h::SQLITE_INTEGER => {
                if *abPK.offset(i as isize) != 0 {
                    let iVal: crate::src::ext::rtree::rtree::I64_0 =
                        sessionGetI64(a) as crate::src::ext::rtree::rtree::I64_0;
                    rc = crate::src::src::vdbeapi::sqlite3_bind_int64(
                        pSelect,
                        i + 1 as ::core::ffi::c_int,
                        iVal as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    );
                }
                a = a.offset(8 as isize);
            }
            crate::src::headers::sqlite3_h::SQLITE_FLOAT_1 => {
                if *abPK.offset(i as isize) != 0 {
                    let mut rVal: ::core::ffi::c_double = 0.;
                    let mut iVal_0: crate::src::ext::rtree::rtree::I64_0 =
                        sessionGetI64(a) as crate::src::ext::rtree::rtree::I64_0;
                    ::core::ptr::copy_nonoverlapping(
                        &raw mut iVal_0 as *const u8,
                        &raw mut rVal as *mut u8,
                        8 as usize,
                    );
                    rc = crate::src::src::vdbeapi::sqlite3_bind_double(
                        pSelect,
                        i + 1 as ::core::ffi::c_int,
                        rVal,
                    );
                }
                a = a.offset(8 as isize);
            }
            crate::src::headers::sqlite3_h::SQLITE_TEXT => {
                let mut n: ::core::ffi::c_int = 0;
                a = a.offset(sessionVarintGet(a, &raw mut n) as isize);
                if *abPK.offset(i as isize) != 0 {
                    rc = crate::src::src::vdbeapi::sqlite3_bind_text(
                        pSelect,
                        i + 1 as ::core::ffi::c_int,
                        a as *mut ::core::ffi::c_char,
                        n,
                        ::core::mem::transmute::<
                            crate::src::headers::stdlib::IntptrT,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
                    );
                }
                a = a.offset(n as isize);
            }
            _ => {
                let mut n_0: ::core::ffi::c_int = 0;
                a = a.offset(sessionVarintGet(a, &raw mut n_0) as isize);
                if *abPK.offset(i as isize) != 0 {
                    rc = crate::src::src::vdbeapi::sqlite3_bind_blob(
                        pSelect,
                        i + 1 as ::core::ffi::c_int,
                        a as *const ::core::ffi::c_void,
                        n_0,
                        ::core::mem::transmute::<
                            crate::src::headers::stdlib::IntptrT,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
                    );
                }
                a = a.offset(n_0 as isize);
            }
        }
        i += 1;
    }
    rc
}

unsafe extern "C" fn sessionAppendTableHdr(
    pBuf: *mut SessionBuffer,
    bPatchset: ::core::ffi::c_int,
    pTab: *mut SessionTable,
    pRc: *mut ::core::ffi::c_int,
) {
    sessionAppendByte(
        pBuf,
        (if bPatchset != 0 {
            'P' as i32
        } else {
            'T' as i32
        }) as crate::src::ext::rtree::rtree::U8_0,
        pRc,
    );
    let __pTab_ref = unsafe { &*pTab };
    sessionAppendVarint(pBuf, __pTab_ref.nCol, pRc);
    sessionAppendBlob(pBuf, __pTab_ref.abPK, __pTab_ref.nCol, pRc);
    sessionAppendBlob(
        pBuf,
        __pTab_ref.zName as *mut crate::src::ext::rtree::rtree::U8_0,
        ::libc::strlen(__pTab_ref.zName) as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        pRc,
    );
}

unsafe extern "C" fn sessionGenerateChangeset(
    pSession: *mut sqlite3_session,
    bPatchset: ::core::ffi::c_int,
    xOutput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pOut: *mut ::core::ffi::c_void,
    pnChangeset: *mut ::core::ffi::c_int,
    ppChangeset: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let __pSession_ref = unsafe { &*pSession };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pSession_ref.db;
    let mut pTab: *mut SessionTable;
    let mut buf: SessionBuffer = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int;
    if xOutput.is_none() {
        *pnChangeset = 0 as ::core::ffi::c_int;
        *ppChangeset = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if __pSession_ref.rc != 0 {
        return __pSession_ref.rc;
    }
    rc = crate::src::src::legacy::sqlite3_exec(
        __pSession_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        b"SAVEPOINT changeset\0" as *const u8 as *const ::core::ffi::c_char,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::main::sqlite3_db_mutex(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    pTab = __pSession_ref.pTable;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pTab.is_null() {
        if (*pTab).nEntry != 0 {
            let __pTab_ref = unsafe { &mut *pTab };
            let zName: *const ::core::ffi::c_char = __pTab_ref.zName;
            let mut i: ::core::ffi::c_int;
            let mut pSel: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
            let nRewind: ::core::ffi::c_int = buf.nBuf;
            let mut nNoop: ::core::ffi::c_int;
            let nOldCol: ::core::ffi::c_int = __pTab_ref.nCol;
            rc = sessionReinitTable(pSession, pTab);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK && __pTab_ref.nCol != nOldCol {
                rc = sessionUpdateChanges(pSession, pTab);
            }
            sessionAppendTableHdr(&raw mut buf, bPatchset, pTab, &raw mut rc);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = sessionSelectStmt(
                    db,
                    0 as ::core::ffi::c_int,
                    __pSession_ref.zDb,
                    zName,
                    __pTab_ref.bRowid,
                    __pTab_ref.nCol,
                    __pTab_ref.azCol,
                    __pTab_ref.abPK,
                    &raw mut pSel,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
            }
            nNoop = buf.nBuf;
            i = 0 as ::core::ffi::c_int;
            while i < __pTab_ref.nChange && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let mut p: *mut SessionChange;
                p = *__pTab_ref.apChange.offset(i as isize);
                while rc == crate::src::headers::sqlite3_h::SQLITE_OK && !p.is_null() {
                    rc = sessionSelectBind(pSel, __pTab_ref.nCol, __pTab_ref.abPK, p);
                    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                        if crate::src::src::vdbeapi::sqlite3_step(pSel)
                            == crate::src::headers::sqlite3_h::SQLITE_ROW
                        {
                            if (*p).op as ::core::ffi::c_int
                                == crate::src::headers::sqlite3_h::SQLITE_INSERT
                            {
                                let mut iCol: ::core::ffi::c_int;
                                sessionAppendByte(
                                    &raw mut buf,
                                    crate::src::headers::sqlite3_h::SQLITE_INSERT
                                        as crate::src::ext::rtree::rtree::U8_0,
                                    &raw mut rc,
                                );
                                sessionAppendByte(&raw mut buf, (*p).bIndirect, &raw mut rc);
                                iCol = 0 as ::core::ffi::c_int;
                                while iCol < __pTab_ref.nCol {
                                    sessionAppendCol(&raw mut buf, pSel, iCol, &raw mut rc);
                                    iCol += 1;
                                }
                            } else {
                                rc = sessionAppendUpdate(
                                    &raw mut buf,
                                    bPatchset,
                                    pSel,
                                    p,
                                    __pTab_ref.abPK,
                                );
                            }
                        } else if (*p).op as ::core::ffi::c_int
                            != crate::src::headers::sqlite3_h::SQLITE_INSERT
                        {
                            rc = sessionAppendDelete(
                                &raw mut buf,
                                bPatchset,
                                p,
                                __pTab_ref.nCol,
                                __pTab_ref.abPK,
                            );
                        }
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            rc = crate::src::src::vdbeapi::sqlite3_reset(pSel);
                        }
                        if xOutput.is_some()
                            && rc == crate::src::headers::sqlite3_h::SQLITE_OK
                            && buf.nBuf > nNoop
                            && buf.nBuf > sessions_strm_chunk_size
                        {
                            rc = xOutput.expect("non-null function pointer")(
                                pOut,
                                buf.aBuf as *mut ::core::ffi::c_void,
                                buf.nBuf,
                            );
                            nNoop = -(1 as ::core::ffi::c_int);
                            buf.nBuf = 0 as ::core::ffi::c_int;
                        }
                    }
                    p = (*p).pNext;
                }
                i += 1;
            }
            crate::src::src::vdbeapi::sqlite3_finalize(pSel);
            if buf.nBuf == nNoop {
                buf.nBuf = nRewind;
            }
        }
        pTab = (*pTab).pNext;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if xOutput.is_none() {
            *pnChangeset = buf.nBuf;
            *ppChangeset = buf.aBuf as *mut ::core::ffi::c_void;
            buf.aBuf = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
        } else if buf.nBuf > 0 as ::core::ffi::c_int {
            rc = xOutput.expect("non-null function pointer")(
                pOut,
                buf.aBuf as *mut ::core::ffi::c_void,
                buf.nBuf,
            );
        }
    }
    crate::src::src::malloc::sqlite3_free(buf.aBuf as *mut ::core::ffi::c_void);
    crate::src::src::legacy::sqlite3_exec(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        b"RELEASE changeset\0" as *const u8 as *const ::core::ffi::c_char,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::main::sqlite3_db_mutex(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    rc
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3session_changeset(
    pSession: *mut sqlite3_session,
    pnChangeset: *mut ::core::ffi::c_int,
    ppChangeset: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    if pnChangeset.is_null() || ppChangeset.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_MISUSE;
    }
    rc = sessionGenerateChangeset(
        pSession,
        0 as ::core::ffi::c_int,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pnChangeset,
        ppChangeset,
    );
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3session_changeset_strm(
    pSession: *mut sqlite3_session,
    xOutput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pOut: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if xOutput.is_none() {
        return crate::src::headers::sqlite3_h::SQLITE_MISUSE;
    }
    sessionGenerateChangeset(
        pSession,
        0 as ::core::ffi::c_int,
        xOutput,
        pOut,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3session_patchset_strm(
    pSession: *mut sqlite3_session,
    xOutput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pOut: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if xOutput.is_none() {
        return crate::src::headers::sqlite3_h::SQLITE_MISUSE;
    }
    sessionGenerateChangeset(
        pSession,
        1 as ::core::ffi::c_int,
        xOutput,
        pOut,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    )
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3session_patchset(
    pSession: *mut sqlite3_session,
    pnPatchset: *mut ::core::ffi::c_int,
    ppPatchset: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if pnPatchset.is_null() || ppPatchset.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_MISUSE;
    }
    sessionGenerateChangeset(
        pSession,
        1 as ::core::ffi::c_int,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pnPatchset,
        ppPatchset,
    )
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3session_enable(
    pSession: *mut sqlite3_session,
    bEnable: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let ret: ::core::ffi::c_int;
    let __pSession_ref = unsafe { &mut *pSession };
    crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::main::sqlite3_db_mutex(
        __pSession_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    if bEnable >= 0 as ::core::ffi::c_int {
        __pSession_ref.bEnable = bEnable;
    }
    ret = __pSession_ref.bEnable;
    crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::main::sqlite3_db_mutex(
        __pSession_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    ret
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3session_indirect(
    pSession: *mut sqlite3_session,
    bIndirect: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let ret: ::core::ffi::c_int;
    let __pSession_ref = unsafe { &mut *pSession };
    crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::main::sqlite3_db_mutex(
        __pSession_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    if bIndirect >= 0 as ::core::ffi::c_int {
        __pSession_ref.bIndirect = bIndirect;
    }
    ret = __pSession_ref.bIndirect;
    crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::main::sqlite3_db_mutex(
        __pSession_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    ret
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3session_isempty(
    pSession: *mut sqlite3_session,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pTab: *mut SessionTable;
    let __pSession_ref = unsafe { &*pSession };
    crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::main::sqlite3_db_mutex(
        __pSession_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    pTab = __pSession_ref.pTable;
    while !pTab.is_null() && ret == 0 as ::core::ffi::c_int {
        ret = ((*pTab).nEntry > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        pTab = (*pTab).pNext;
    }
    crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::main::sqlite3_db_mutex(
        __pSession_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    (ret == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3session_memory_used(
    pSession: *mut sqlite3_session,
) -> crate::src::headers::sqlite3_h::Sqlite3Int64 {
    (*pSession).nMalloc as crate::src::headers::sqlite3_h::Sqlite3Int64
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3session_object_config(
    pSession: *mut sqlite3_session,
    op: ::core::ffi::c_int,
    pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    match op {
        crate::src::ext::session::sqlite3session::SQLITE_SESSION_OBJCONFIG_SIZE => {
            let iArg: ::core::ffi::c_int = *(pArg as *mut ::core::ffi::c_int);
            if iArg >= 0 as ::core::ffi::c_int {
                if !(*pSession).pTable.is_null() {
                    rc = crate::src::headers::sqlite3_h::SQLITE_MISUSE;
                } else {
                    (*pSession).bEnableSize =
                        (iArg != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                }
            }
            *(pArg as *mut ::core::ffi::c_int) = (*pSession).bEnableSize;
        }
        crate::src::ext::session::sqlite3session::SQLITE_SESSION_OBJCONFIG_ROWID => {
            let iArg_0: ::core::ffi::c_int = *(pArg as *mut ::core::ffi::c_int);
            if iArg_0 >= 0 as ::core::ffi::c_int {
                if !(*pSession).pTable.is_null() {
                    rc = crate::src::headers::sqlite3_h::SQLITE_MISUSE;
                } else {
                    (*pSession).bImplicitPK =
                        (iArg_0 != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                }
            }
            *(pArg as *mut ::core::ffi::c_int) = (*pSession).bImplicitPK;
        }
        _ => {
            rc = crate::src::headers::sqlite3_h::SQLITE_MISUSE;
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3session_changeset_size(
    pSession: *mut sqlite3_session,
) -> crate::src::headers::sqlite3_h::Sqlite3Int64 {
    (*pSession).nMaxChangesetSize as crate::src::headers::sqlite3_h::Sqlite3Int64
}

unsafe extern "C" fn sessionChangesetStart(
    pp: *mut *mut sqlite3_changeset_iter,
    xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pIn: *mut ::core::ffi::c_void,
    nChangeset: ::core::ffi::c_int,
    pChangeset: *mut ::core::ffi::c_void,
    bInvert: ::core::ffi::c_int,
    bSkipEmpty: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pRet: *mut sqlite3_changeset_iter;
    let nByte: ::core::ffi::c_int;
    *pp = ::core::ptr::null_mut::<sqlite3_changeset_iter>();
    nByte = ::core::mem::size_of::<sqlite3_changeset_iter>() as ::core::ffi::c_int;
    pRet = crate::src::src::malloc::sqlite3_malloc(nByte) as *mut sqlite3_changeset_iter;
    if pRet.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pRet as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sqlite3_changeset_iter>() as crate::__stddef_size_t_h::SizeT,
    );
    (*pRet).in_0.aData = pChangeset as *mut crate::src::ext::rtree::rtree::U8_0;
    (*pRet).in_0.nData = nChangeset;
    (*pRet).in_0.xInput = xInput;
    (*pRet).in_0.pIn = pIn;
    (*pRet).in_0.bEof = if xInput.is_some() {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    (*pRet).bInvert = bInvert;
    (*pRet).bSkipEmpty = bSkipEmpty;
    *pp = pRet;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_start(
    pp: *mut *mut sqlite3_changeset_iter,
    nChangeset: ::core::ffi::c_int,
    pChangeset: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    sessionChangesetStart(
        pp,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        nChangeset,
        pChangeset,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_start_v2(
    pp: *mut *mut sqlite3_changeset_iter,
    nChangeset: ::core::ffi::c_int,
    pChangeset: *mut ::core::ffi::c_void,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let bInvert: ::core::ffi::c_int = (flags
        & crate::src::ext::session::sqlite3session::SQLITE_CHANGESETSTART_INVERT
        != 0) as ::core::ffi::c_int;
    sessionChangesetStart(
        pp,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        nChangeset,
        pChangeset,
        bInvert,
        0 as ::core::ffi::c_int,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_start_strm(
    pp: *mut *mut sqlite3_changeset_iter,
    xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pIn: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    sessionChangesetStart(
        pp,
        xInput,
        pIn,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_start_v2_strm(
    pp: *mut *mut sqlite3_changeset_iter,
    xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pIn: *mut ::core::ffi::c_void,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let bInvert: ::core::ffi::c_int = (flags
        & crate::src::ext::session::sqlite3session::SQLITE_CHANGESETSTART_INVERT
        != 0) as ::core::ffi::c_int;
    sessionChangesetStart(
        pp,
        xInput,
        pIn,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        bInvert,
        0 as ::core::ffi::c_int,
    )
}

unsafe extern "C" fn sessionDiscardData(pIn: *mut SessionInput) {
    if (*pIn).xInput.is_some() && (*pIn).iCurrent >= sessions_strm_chunk_size {
        let __pIn_ref = unsafe { &mut *pIn };
        let nMove: ::core::ffi::c_int = __pIn_ref.buf.nBuf - __pIn_ref.iCurrent;
        if nMove > 0 as ::core::ffi::c_int {
            ::core::ptr::copy(
                __pIn_ref.buf.aBuf.offset(__pIn_ref.iCurrent as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0 as *const u8,
                __pIn_ref.buf.aBuf as *mut u8,
                nMove as usize,
            );
        }
        __pIn_ref.buf.nBuf -= __pIn_ref.iCurrent;
        __pIn_ref.iNext -= __pIn_ref.iCurrent;
        __pIn_ref.iCurrent = 0 as ::core::ffi::c_int;
        __pIn_ref.nData = __pIn_ref.buf.nBuf;
    }
}

unsafe extern "C" fn sessionInputBuffer(
    pIn: *mut SessionInput,
    nByte: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pIn).xInput.is_some() {
        let __pIn_ref = unsafe { &mut *pIn };
        while __pIn_ref.bEof == 0
            && __pIn_ref.iNext + nByte >= __pIn_ref.nData
            && rc == crate::src::headers::sqlite3_h::SQLITE_OK
        {
            let mut nNew: ::core::ffi::c_int = sessions_strm_chunk_size;
            if __pIn_ref.bNoDiscard == 0 as ::core::ffi::c_int {
                sessionDiscardData(pIn);
            }
            if crate::src::headers::sqlite3_h::SQLITE_OK
                == sessionBufferGrow(
                    &raw mut __pIn_ref.buf,
                    nNew as crate::src::ext::rtree::rtree::I64_0,
                    &raw mut rc,
                )
            {
                rc = __pIn_ref.xInput.expect("non-null function pointer")(
                    __pIn_ref.pIn,
                    __pIn_ref.buf.aBuf.offset(__pIn_ref.buf.nBuf as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0
                        as *mut ::core::ffi::c_void,
                    &raw mut nNew,
                );
                if nNew == 0 as ::core::ffi::c_int {
                    __pIn_ref.bEof = 1 as ::core::ffi::c_int;
                } else {
                    __pIn_ref.buf.nBuf += nNew;
                }
            }
            __pIn_ref.aData = __pIn_ref.buf.aBuf;
            __pIn_ref.nData = __pIn_ref.buf.nBuf;
        }
    }
    rc
}

unsafe extern "C" fn sessionSkipRecord(
    ppRec: *mut *mut crate::src::ext::rtree::rtree::U8_0,
    nCol: ::core::ffi::c_int,
) {
    let mut aRec: *mut crate::src::ext::rtree::rtree::U8_0 = *ppRec;
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nCol {
        let fresh32 = aRec;
        aRec = aRec.offset(1);
        let eType: ::core::ffi::c_int = *fresh32 as ::core::ffi::c_int;
        if eType == crate::src::headers::sqlite3_h::SQLITE_TEXT
            || eType == crate::src::headers::sqlite3_h::SQLITE_BLOB
        {
            let mut nByte: ::core::ffi::c_int = 0;
            aRec = aRec.offset(sessionVarintGet(aRec, &raw mut nByte) as isize);
            aRec = aRec.offset(nByte as isize);
        } else if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER
            || eType == crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
        {
            aRec = aRec.offset(8 as isize);
        }
        i += 1;
    }
    *ppRec = aRec;
}

unsafe extern "C" fn sessionValueSetStr(
    pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    aData: *mut crate::src::ext::rtree::rtree::U8_0,
    nData: ::core::ffi::c_int,
    enc: crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let aCopy: *mut crate::src::ext::rtree::rtree::U8_0 =
        crate::src::src::malloc::sqlite3_malloc64(
            (nData as crate::src::headers::sqlite3_h::Sqlite3Int64
                + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64)
                as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut crate::src::ext::rtree::rtree::U8_0;
    if aCopy.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::core::ptr::copy_nonoverlapping(aData as *const u8, aCopy as *mut u8, nData as usize);
    crate::src::src::vdbemem::sqlite3ValueSetStr(
        pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        nData,
        aCopy as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        enc,
        Some(
            crate::src::src::malloc::sqlite3_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
    );
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn sessionReadRecord(
    pIn: *mut SessionInput,
    nCol: ::core::ffi::c_int,
    abPK: *mut crate::src::ext::rtree::rtree::U8_0,
    apOut: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pbEmpty: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !pbEmpty.is_null() {
        *pbEmpty = 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nCol && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut eType: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !(!abPK.is_null()
            && *abPK.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            rc = sessionInputBuffer(pIn, 9 as ::core::ffi::c_int);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                if (*pIn).iNext >= (*pIn).nData {
                    rc = crate::src::src::main::sqlite3CorruptError(3545 as ::core::ffi::c_int);
                } else {
                    let __pIn_ref = unsafe { &mut *pIn };
                    let fresh23 = __pIn_ref.iNext;
                    __pIn_ref.iNext += 1;
                    eType = *__pIn_ref.aData.offset(fresh23 as isize) as ::core::ffi::c_int;
                    if eType != 0 {
                        if !pbEmpty.is_null() {
                            *pbEmpty = 0 as ::core::ffi::c_int;
                        }
                        let ref mut fresh24 = *apOut.offset(i as isize);
                        *fresh24 =
                            crate::src::src::vdbemem::sqlite3ValueNew(::core::ptr::null_mut::<
                                crate::src::headers::sqliteInt_h::sqlite3,
                            >(
                            )
                                as *mut crate::src::headers::sqliteInt_h::sqlite3)
                                as *mut crate::src::headers::vdbeInt_h::sqlite3_value;
                        if (*apOut.offset(i as isize)).is_null() {
                            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                        }
                    }
                }
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let aVal: *mut crate::src::ext::rtree::rtree::U8_0 =
                    (*pIn).aData.offset((*pIn).iNext as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0;
                if eType == crate::src::headers::sqlite3_h::SQLITE_TEXT
                    || eType == crate::src::headers::sqlite3_h::SQLITE_BLOB
                {
                    let mut nByte: ::core::ffi::c_int = 0;
                    (*pIn).iNext += sessionVarintGet(aVal, &raw mut nByte);
                    rc = sessionInputBuffer(pIn, nByte);
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        if nByte < 0 as ::core::ffi::c_int || nByte > (*pIn).nData - (*pIn).iNext {
                            rc = crate::src::src::main::sqlite3CorruptError(
                                3565 as ::core::ffi::c_int,
                            );
                        } else {
                            let enc: crate::src::ext::rtree::rtree::U8_0 =
                                (if eType == crate::src::headers::sqlite3_h::SQLITE_TEXT {
                                    crate::src::headers::sqlite3_h::SQLITE_UTF8
                                } else {
                                    0 as ::core::ffi::c_int
                                })
                                    as crate::src::ext::rtree::rtree::U8_0;
                            let __pIn_ref = unsafe { &mut *pIn };
                            rc = sessionValueSetStr(
                                *apOut.offset(i as isize),
                                __pIn_ref.aData.offset(__pIn_ref.iNext as isize)
                                    as *mut crate::src::ext::rtree::rtree::U8_0,
                                nByte,
                                enc,
                            );
                            __pIn_ref.iNext += nByte;
                        }
                    }
                }
                if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER
                    || eType == crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
                {
                    if (*pIn).nData - (*pIn).iNext < 8 as ::core::ffi::c_int {
                        rc = crate::src::src::main::sqlite3CorruptError(3575 as ::core::ffi::c_int);
                    } else {
                        let mut v: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                            sessionGetI64(aVal);
                        if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(
                                *apOut.offset(i as isize) as *mut crate::src::src::vdbe::Mem
                                    as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                v as crate::src::ext::rtree::rtree::I64_0,
                            );
                        } else {
                            let mut d: ::core::ffi::c_double = 0.;
                            ::core::ptr::copy_nonoverlapping(
                                &raw mut v as *const u8,
                                &raw mut d as *mut u8,
                                8 as usize,
                            );
                            crate::src::src::vdbemem::sqlite3VdbeMemSetDouble(
                                *apOut.offset(i as isize) as *mut crate::src::src::vdbe::Mem
                                    as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                d,
                            );
                        }
                        (*pIn).iNext += 8 as ::core::ffi::c_int;
                    }
                }
            }
        }
        i += 1;
    }
    rc
}

unsafe extern "C" fn sessionChangesetBufferTblhdr(
    pIn: *mut SessionInput,
    pnByte: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nRead: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = sessionInputBuffer(pIn, 9 as ::core::ffi::c_int);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        nRead += sessionVarintGet(
            (*pIn).aData.offset(((*pIn).iNext + nRead) as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0,
            &raw mut nCol,
        );
        if nCol < 0 as ::core::ffi::c_int || nCol > 65536 as ::core::ffi::c_int {
            rc = crate::src::src::main::sqlite3CorruptError(3623 as ::core::ffi::c_int);
        } else {
            rc = sessionInputBuffer(pIn, nRead + nCol + 100 as ::core::ffi::c_int);
            nRead += nCol;
        }
    }
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pIn_ref = unsafe { &mut *pIn };
        while __pIn_ref.iNext + nRead < __pIn_ref.nData
            && *__pIn_ref.aData.offset((__pIn_ref.iNext + nRead) as isize) as ::core::ffi::c_int
                != 0
        {
            nRead += 1;
        }
        if __pIn_ref.iNext + nRead < __pIn_ref.nData {
            break;
        }
        rc = sessionInputBuffer(pIn, nRead + 100 as ::core::ffi::c_int);
    }
    *pnByte = nRead + 1 as ::core::ffi::c_int;
    rc
}

unsafe extern "C" fn sessionChangesetBufferRecord(
    pIn: *mut SessionInput,
    nCol: ::core::ffi::c_int,
    pnByte: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < nCol {
        let eType: ::core::ffi::c_int;
        rc = sessionInputBuffer(pIn, nByte + 10 as ::core::ffi::c_int);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let fresh25 = nByte;
            nByte += 1;
            eType = *(*pIn).aData.offset(((*pIn).iNext + fresh25) as isize) as ::core::ffi::c_int;
            if eType == crate::src::headers::sqlite3_h::SQLITE_TEXT
                || eType == crate::src::headers::sqlite3_h::SQLITE_BLOB
            {
                let mut n: ::core::ffi::c_int = 0;
                nByte += sessionVarintGet(
                    (*pIn).aData.offset(((*pIn).iNext + nByte) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0,
                    &raw mut n,
                );
                nByte += n;
                rc = sessionInputBuffer(pIn, nByte);
            } else if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER
                || eType == crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
            {
                nByte += 8 as ::core::ffi::c_int;
            }
        }
        i += 1;
    }
    *pnByte = nByte;
    rc
}

unsafe extern "C" fn sessionChangesetReadTblhdr(
    p: *mut sqlite3_changeset_iter,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut nCopy: ::core::ffi::c_int = 0;
    let __p_ref = unsafe { &mut *p };
    rc = sessionChangesetBufferTblhdr(&raw mut __p_ref.in_0, &raw mut nCopy);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let nByte: ::core::ffi::c_int;
        let nVarint: ::core::ffi::c_int;
        nVarint = sessionVarintGet(
            __p_ref.in_0.aData.offset(__p_ref.in_0.iNext as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0,
            &raw mut __p_ref.nCol,
        );
        if __p_ref.nCol > 0 as ::core::ffi::c_int {
            nCopy -= nVarint;
            __p_ref.in_0.iNext += nVarint;
            nByte = (__p_ref.nCol as usize)
                .wrapping_mul(::core::mem::size_of::<
                    *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                >() as usize)
                .wrapping_mul(2 as usize)
                .wrapping_add(nCopy as usize) as ::core::ffi::c_int;
            __p_ref.tblhdr.nBuf = 0 as ::core::ffi::c_int;
            sessionBufferGrow(
                &raw mut __p_ref.tblhdr,
                nByte as crate::src::ext::rtree::rtree::I64_0,
                &raw mut rc,
            );
        } else {
            rc = crate::src::src::main::sqlite3CorruptError(3711 as ::core::ffi::c_int);
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let iPK: crate::__stddef_size_t_h::SizeT =
            (::core::mem::size_of::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>()
                as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul(__p_ref.nCol as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul(2 as crate::__stddef_size_t_h::SizeT);
        ::libc::memset(
            __p_ref.tblhdr.aBuf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            iPK,
        );
        ::core::ptr::copy_nonoverlapping(
            __p_ref.in_0.aData.offset(__p_ref.in_0.iNext as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0 as *const u8,
            __p_ref.tblhdr.aBuf.offset(iPK as isize) as *mut crate::src::ext::rtree::rtree::U8_0
                as *mut u8,
            nCopy as usize,
        );
        __p_ref.in_0.iNext += nCopy;
    }
    __p_ref.apValue =
        __p_ref.tblhdr.aBuf as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value;
    if __p_ref.apValue.is_null() {
        __p_ref.abPK = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
        __p_ref.zTab = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        __p_ref.abPK = (*p)
            .apValue
            .offset((__p_ref.nCol * 2 as ::core::ffi::c_int) as isize)
            as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value
            as *mut crate::src::ext::rtree::rtree::U8_0;
        __p_ref.zTab = if !__p_ref.abPK.is_null() {
            __p_ref.abPK.offset(__p_ref.nCol as isize) as *mut crate::src::ext::rtree::rtree::U8_0
                as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    }
    __p_ref.rc = rc;
    __p_ref.rc
}

unsafe extern "C" fn sessionChangesetNextOne(
    p: *mut sqlite3_changeset_iter,
    paRec: *mut *mut crate::src::ext::rtree::rtree::U8_0,
    pnRec: *mut ::core::ffi::c_int,
    pbNew: *mut ::core::ffi::c_int,
    pbEmpty: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut op: crate::src::ext::rtree::rtree::U8_0;
    let __p_ref = unsafe { &mut *p };
    if __p_ref.rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return __p_ref.rc;
    }
    if !__p_ref.apValue.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < __p_ref.nCol * 2 as ::core::ffi::c_int {
            crate::src::src::vdbemem::sqlite3ValueFree(*__p_ref.apValue.offset(i as isize)
                as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
            i += 1;
        }
        ::libc::memset(
            __p_ref.apValue as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>()
                as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul(__p_ref.nCol as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul(2 as crate::__stddef_size_t_h::SizeT),
        );
    }
    __p_ref.rc = sessionInputBuffer(&raw mut __p_ref.in_0, 2 as ::core::ffi::c_int);
    if __p_ref.rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return __p_ref.rc;
    }
    __p_ref.in_0.iCurrent = __p_ref.in_0.iNext;
    sessionDiscardData(&raw mut __p_ref.in_0);
    if __p_ref.in_0.iNext >= __p_ref.in_0.nData {
        return crate::src::headers::sqlite3_h::SQLITE_DONE;
    }
    let fresh17 = __p_ref.in_0.iNext;
    __p_ref.in_0.iNext += 1;
    op = *__p_ref.in_0.aData.offset(fresh17 as isize);
    while op as ::core::ffi::c_int == 'T' as i32 || op as ::core::ffi::c_int == 'P' as i32 {
        if !pbNew.is_null() {
            *pbNew = 1 as ::core::ffi::c_int;
        }
        __p_ref.bPatchset = (op as ::core::ffi::c_int == 'P' as i32) as ::core::ffi::c_int;
        if sessionChangesetReadTblhdr(p) != 0 {
            return __p_ref.rc;
        }
        __p_ref.rc = sessionInputBuffer(&raw mut __p_ref.in_0, 2 as ::core::ffi::c_int);
        if __p_ref.rc != 0 {
            return __p_ref.rc;
        }
        __p_ref.in_0.iCurrent = __p_ref.in_0.iNext;
        if __p_ref.in_0.iNext >= __p_ref.in_0.nData {
            return crate::src::headers::sqlite3_h::SQLITE_DONE;
        }
        let fresh18 = __p_ref.in_0.iNext;
        __p_ref.in_0.iNext += 1;
        op = *__p_ref.in_0.aData.offset(fresh18 as isize);
    }
    if __p_ref.zTab.is_null() || __p_ref.bPatchset != 0 && __p_ref.bInvert != 0 {
        __p_ref.rc = crate::src::src::main::sqlite3CorruptError(3797 as ::core::ffi::c_int);
        return __p_ref.rc;
    }
    __p_ref.op = op as ::core::ffi::c_int;
    let fresh19 = __p_ref.in_0.iNext;
    __p_ref.in_0.iNext += 1;
    __p_ref.bIndirect = *__p_ref.in_0.aData.offset(fresh19 as isize) as ::core::ffi::c_int;
    if __p_ref.op != crate::src::headers::sqlite3_h::SQLITE_UPDATE
        && __p_ref.op != crate::src::headers::sqlite3_h::SQLITE_DELETE
        && __p_ref.op != crate::src::headers::sqlite3_h::SQLITE_INSERT
    {
        __p_ref.rc = crate::src::src::main::sqlite3CorruptError(3803 as ::core::ffi::c_int);
        return __p_ref.rc;
    }
    if !paRec.is_null() {
        let mut nVal: ::core::ffi::c_int;
        if __p_ref.bPatchset == 0 as ::core::ffi::c_int
            && op as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_UPDATE
        {
            nVal = __p_ref.nCol * 2 as ::core::ffi::c_int;
        } else if __p_ref.bPatchset != 0
            && op as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_DELETE
        {
            nVal = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < __p_ref.nCol {
                if *__p_ref.abPK.offset(i as isize) != 0 {
                    nVal += 1;
                }
                i += 1;
            }
        } else {
            nVal = __p_ref.nCol;
        }
        __p_ref.rc = sessionChangesetBufferRecord(&raw mut __p_ref.in_0, nVal, pnRec);
        if __p_ref.rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return __p_ref.rc;
        }
        *paRec = __p_ref.in_0.aData.offset(__p_ref.in_0.iNext as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0;
        __p_ref.in_0.iNext += *pnRec;
    } else {
        let apOld: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value =
            if __p_ref.bInvert != 0 {
                __p_ref.apValue.offset(__p_ref.nCol as isize)
                    as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value
            } else {
                __p_ref.apValue
            };
        let apNew: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value =
            if __p_ref.bInvert != 0 {
                __p_ref.apValue
            } else {
                __p_ref.apValue.offset(__p_ref.nCol as isize)
                    as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value
            };
        if __p_ref.op != crate::src::headers::sqlite3_h::SQLITE_INSERT
            && (__p_ref.bPatchset == 0 as ::core::ffi::c_int
                || __p_ref.op == crate::src::headers::sqlite3_h::SQLITE_DELETE)
        {
            let abPK: *mut crate::src::ext::rtree::rtree::U8_0 = if __p_ref.bPatchset != 0 {
                __p_ref.abPK
            } else {
                ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>()
            };
            __p_ref.rc = sessionReadRecord(
                &raw mut __p_ref.in_0,
                __p_ref.nCol,
                abPK,
                apOld,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if __p_ref.rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return __p_ref.rc;
            }
        }
        if __p_ref.op != crate::src::headers::sqlite3_h::SQLITE_DELETE {
            __p_ref.rc = sessionReadRecord(
                &raw mut __p_ref.in_0,
                __p_ref.nCol,
                ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
                apNew,
                pbEmpty,
            );
            if __p_ref.rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return __p_ref.rc;
            }
        }
        if (__p_ref.bPatchset != 0 || __p_ref.bInvert != 0)
            && __p_ref.op == crate::src::headers::sqlite3_h::SQLITE_UPDATE
        {
            i = 0 as ::core::ffi::c_int;
            while i < __p_ref.nCol {
                if *__p_ref.abPK.offset(i as isize) != 0 {
                    let ref mut fresh20 = *__p_ref.apValue.offset(i as isize);
                    *fresh20 = *__p_ref.apValue.offset((i + __p_ref.nCol) as isize);
                    if (*__p_ref.apValue.offset(i as isize)).is_null() {
                        __p_ref.rc =
                            crate::src::src::main::sqlite3CorruptError(3847 as ::core::ffi::c_int);
                        return __p_ref.rc;
                    }
                    let ref mut fresh21 = *__p_ref.apValue.offset((i + __p_ref.nCol) as isize);
                    *fresh21 =
                        ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
                }
                i += 1;
            }
        } else if __p_ref.bInvert != 0 {
            if __p_ref.op == crate::src::headers::sqlite3_h::SQLITE_INSERT {
                __p_ref.op = crate::src::headers::sqlite3_h::SQLITE_DELETE;
            } else if __p_ref.op == crate::src::headers::sqlite3_h::SQLITE_DELETE {
                __p_ref.op = crate::src::headers::sqlite3_h::SQLITE_INSERT;
            }
        }
        if __p_ref.bPatchset == 0 as ::core::ffi::c_int
            && __p_ref.op == crate::src::headers::sqlite3_h::SQLITE_UPDATE
        {
            i = 0 as ::core::ffi::c_int;
            while i < __p_ref.nCol {
                if *__p_ref.abPK.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*__p_ref.apValue.offset((i + __p_ref.nCol) as isize)).is_null()
                {
                    crate::src::src::vdbemem::sqlite3ValueFree(*__p_ref.apValue.offset(i as isize)
                        as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                    let ref mut fresh22 = *__p_ref.apValue.offset(i as isize);
                    *fresh22 =
                        ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
                }
                i += 1;
            }
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_ROW
}

unsafe extern "C" fn sessionChangesetNext(
    p: *mut sqlite3_changeset_iter,
    paRec: *mut *mut crate::src::ext::rtree::rtree::U8_0,
    pnRec: *mut ::core::ffi::c_int,
    pbNew: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bEmpty: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    loop {
        bEmpty = 0 as ::core::ffi::c_int;
        rc = sessionChangesetNextOne(p, paRec, pnRec, pbNew, &raw mut bEmpty);
        if !(rc == crate::src::headers::sqlite3_h::SQLITE_ROW
            && (*p).bSkipEmpty != 0
            && bEmpty != 0)
        {
            break;
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_next(
    p: *mut sqlite3_changeset_iter,
) -> ::core::ffi::c_int {
    sessionChangesetNext(
        p,
        ::core::ptr::null_mut::<*mut crate::src::ext::rtree::rtree::U8_0>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_op(
    pIter: *mut sqlite3_changeset_iter,
    pzTab: *mut *const ::core::ffi::c_char,
    pnCol: *mut ::core::ffi::c_int,
    pOp: *mut ::core::ffi::c_int,
    pbIndirect: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pIter_ref = unsafe { &*pIter };
    *pOp = __pIter_ref.op;
    *pnCol = __pIter_ref.nCol;
    *pzTab = __pIter_ref.zTab;
    if !pbIndirect.is_null() {
        *pbIndirect = __pIter_ref.bIndirect;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_pk(
    pIter: *mut sqlite3_changeset_iter,
    pabPK: *mut *mut ::core::ffi::c_uchar,
    pnCol: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *pabPK = (*pIter).abPK as *mut ::core::ffi::c_uchar;
    if !pnCol.is_null() {
        *pnCol = (*pIter).nCol;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_old(
    pIter: *mut sqlite3_changeset_iter,
    iVal: ::core::ffi::c_int,
    ppValue: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let __pIter_ref = unsafe { &mut *pIter };
    if __pIter_ref.op != crate::src::headers::sqlite3_h::SQLITE_UPDATE
        && __pIter_ref.op != crate::src::headers::sqlite3_h::SQLITE_DELETE
    {
        return crate::src::headers::sqlite3_h::SQLITE_MISUSE;
    }
    if iVal < 0 as ::core::ffi::c_int || iVal >= __pIter_ref.nCol {
        return crate::src::headers::sqlite3_h::SQLITE_RANGE;
    }
    *ppValue = *__pIter_ref.apValue.offset(iVal as isize);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_new(
    pIter: *mut sqlite3_changeset_iter,
    iVal: ::core::ffi::c_int,
    ppValue: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let __pIter_ref = unsafe { &mut *pIter };
    if __pIter_ref.op != crate::src::headers::sqlite3_h::SQLITE_UPDATE
        && __pIter_ref.op != crate::src::headers::sqlite3_h::SQLITE_INSERT
    {
        return crate::src::headers::sqlite3_h::SQLITE_MISUSE;
    }
    if iVal < 0 as ::core::ffi::c_int || iVal >= __pIter_ref.nCol {
        return crate::src::headers::sqlite3_h::SQLITE_RANGE;
    }
    *ppValue = *__pIter_ref
        .apValue
        .offset((__pIter_ref.nCol + iVal) as isize);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_conflict(
    pIter: *mut sqlite3_changeset_iter,
    iVal: ::core::ffi::c_int,
    ppValue: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let __pIter_ref = unsafe { &mut *pIter };
    if __pIter_ref.pConflict.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_MISUSE;
    }
    if iVal < 0 as ::core::ffi::c_int || iVal >= __pIter_ref.nCol {
        return crate::src::headers::sqlite3_h::SQLITE_RANGE;
    }
    *ppValue = crate::src::src::vdbeapi::sqlite3_column_value(__pIter_ref.pConflict, iVal)
        as *mut crate::src::headers::vdbeInt_h::sqlite3_value;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_fk_conflicts(
    pIter: *mut sqlite3_changeset_iter,
    pnOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pIter_ref = unsafe { &mut *pIter };
    if !__pIter_ref.pConflict.is_null() || !__pIter_ref.apValue.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_MISUSE;
    }
    *pnOut = __pIter_ref.nCol;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_finalize(
    p: *mut sqlite3_changeset_iter,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !p.is_null() {
        let mut i: ::core::ffi::c_int;
        let __p_ref = unsafe { &mut *p };
        rc = __p_ref.rc;
        if !__p_ref.apValue.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < __p_ref.nCol * 2 as ::core::ffi::c_int {
                crate::src::src::vdbemem::sqlite3ValueFree(*__p_ref.apValue.offset(i as isize)
                    as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                i += 1;
            }
        }
        crate::src::src::malloc::sqlite3_free(__p_ref.tblhdr.aBuf as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(__p_ref.in_0.buf.aBuf as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    }
    rc
}

unsafe extern "C" fn sessionChangesetInvert(
    pInput: *mut SessionInput,
    xOutput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pOut: *mut ::core::ffi::c_void,
    pnInverted: *mut ::core::ffi::c_int,
    ppInverted: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let current_block: u64;
    let mut rc: ::core::ffi::c_int;
    let mut sOut: SessionBuffer = unsafe { ::core::mem::zeroed() };
    let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut abPK: *mut crate::src::ext::rtree::rtree::U8_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    let mut apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value =
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>();
    let mut sPK: SessionBuffer = unsafe { ::core::mem::zeroed() };
    if !ppInverted.is_null() {
        *ppInverted = ::core::ptr::null_mut::<::core::ffi::c_void>();
        *pnInverted = 0 as ::core::ffi::c_int;
    }
    loop {
        let eType: crate::src::ext::rtree::rtree::U8_0;
        rc = sessionInputBuffer(pInput, 2 as ::core::ffi::c_int);
        if rc != 0 {
            current_block = 7699217338199643514;
            break;
        }
        let __pInput_ref = unsafe { &mut *pInput };
        if __pInput_ref.iNext >= __pInput_ref.nData {
            current_block = 11441799814184323368;
            break;
        }
        eType = *__pInput_ref.aData.offset(__pInput_ref.iNext as isize);
        match eType as ::core::ffi::c_int {
            84 => {
                let mut nByte: ::core::ffi::c_int = 0;
                let nVar: ::core::ffi::c_int;
                __pInput_ref.iNext += 1;
                rc = sessionChangesetBufferTblhdr(pInput, &raw mut nByte);
                if rc != 0 {
                    current_block = 7699217338199643514;
                    break;
                }
                nVar = sessionVarintGet(
                    __pInput_ref.aData.offset(__pInput_ref.iNext as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0,
                    &raw mut nCol,
                );
                sPK.nBuf = 0 as ::core::ffi::c_int;
                sessionAppendBlob(
                    &raw mut sPK,
                    __pInput_ref
                        .aData
                        .offset((__pInput_ref.iNext + nVar) as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0,
                    nCol,
                    &raw mut rc,
                );
                sessionAppendByte(&raw mut sOut, eType, &raw mut rc);
                sessionAppendBlob(
                    &raw mut sOut,
                    __pInput_ref.aData.offset(__pInput_ref.iNext as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0,
                    nByte,
                    &raw mut rc,
                );
                if rc != 0 {
                    current_block = 7699217338199643514;
                    break;
                }
                __pInput_ref.iNext += nByte;
                crate::src::src::malloc::sqlite3_free(apVal as *mut ::core::ffi::c_void);
                apVal =
                    ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>();
                abPK = sPK.aBuf;
            }
            crate::src::headers::sqlite3_h::SQLITE_INSERT
            | crate::src::headers::sqlite3_h::SQLITE_DELETE => {
                let mut nByte_0: ::core::ffi::c_int = 0;
                let bIndirect: ::core::ffi::c_int = *(*pInput)
                    .aData
                    .offset((__pInput_ref.iNext + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
                let eType2: ::core::ffi::c_int = if eType as ::core::ffi::c_int
                    == crate::src::headers::sqlite3_h::SQLITE_DELETE
                {
                    crate::src::headers::sqlite3_h::SQLITE_INSERT
                } else {
                    crate::src::headers::sqlite3_h::SQLITE_DELETE
                };
                __pInput_ref.iNext += 2 as ::core::ffi::c_int;
                rc = sessionChangesetBufferRecord(pInput, nCol, &raw mut nByte_0);
                sessionAppendByte(
                    &raw mut sOut,
                    eType2 as crate::src::ext::rtree::rtree::U8_0,
                    &raw mut rc,
                );
                sessionAppendByte(
                    &raw mut sOut,
                    bIndirect as crate::src::ext::rtree::rtree::U8_0,
                    &raw mut rc,
                );
                sessionAppendBlob(
                    &raw mut sOut,
                    __pInput_ref.aData.offset(__pInput_ref.iNext as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0,
                    nByte_0,
                    &raw mut rc,
                );
                __pInput_ref.iNext += nByte_0;
                if rc != 0 {
                    current_block = 7699217338199643514;
                    break;
                }
            }
            crate::src::headers::sqlite3_h::SQLITE_UPDATE => {
                let mut iCol: ::core::ffi::c_int;
                if apVal.is_null() {
                    apVal = crate::src::src::malloc::sqlite3_malloc64(
                        (::core::mem::size_of::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(
                        ) as usize)
                            .wrapping_mul(nCol as usize)
                            .wrapping_mul(2 as usize)
                            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                    )
                        as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value;
                    if apVal.is_null() {
                        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                        current_block = 7699217338199643514;
                        break;
                    } else {
                        ::libc::memset(
                            apVal as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (::core::mem::size_of::<
                                *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            >() as crate::__stddef_size_t_h::SizeT)
                                .wrapping_mul(nCol as crate::__stddef_size_t_h::SizeT)
                                .wrapping_mul(2 as crate::__stddef_size_t_h::SizeT),
                        );
                    }
                }
                sessionAppendByte(&raw mut sOut, eType, &raw mut rc);
                sessionAppendByte(
                    &raw mut sOut,
                    *(*pInput)
                        .aData
                        .offset((__pInput_ref.iNext + 1 as ::core::ffi::c_int) as isize),
                    &raw mut rc,
                );
                __pInput_ref.iNext += 2 as ::core::ffi::c_int;
                rc = sessionReadRecord(
                    pInput,
                    nCol,
                    ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
                    apVal.offset(0 as isize)
                        as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    rc = sessionReadRecord(
                        pInput,
                        nCol,
                        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
                        apVal.offset(nCol as isize)
                            as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    );
                }
                iCol = 0 as ::core::ffi::c_int;
                while iCol < nCol {
                    let pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value = *apVal
                        .offset(
                            (iCol
                                + (if *abPK.offset(iCol as isize) as ::core::ffi::c_int != 0 {
                                    0 as ::core::ffi::c_int
                                } else {
                                    nCol
                                })) as isize,
                        );
                    sessionAppendValue(&raw mut sOut, pVal, &raw mut rc);
                    iCol += 1;
                }
                iCol = 0 as ::core::ffi::c_int;
                while iCol < nCol {
                    let pVal_0: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                        if *abPK.offset(iCol as isize) as ::core::ffi::c_int != 0 {
                            ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>()
                        } else {
                            *apVal.offset(iCol as isize)
                        };
                    sessionAppendValue(&raw mut sOut, pVal_0, &raw mut rc);
                    iCol += 1;
                }
                iCol = 0 as ::core::ffi::c_int;
                while iCol < nCol * 2 as ::core::ffi::c_int {
                    crate::src::src::vdbemem::sqlite3ValueFree(*apVal.offset(iCol as isize)
                        as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                    iCol += 1;
                }
                ::libc::memset(
                    apVal as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>()
                        as crate::__stddef_size_t_h::SizeT)
                        .wrapping_mul(nCol as crate::__stddef_size_t_h::SizeT)
                        .wrapping_mul(2 as crate::__stddef_size_t_h::SizeT),
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    current_block = 7699217338199643514;
                    break;
                }
            }
            _ => {
                rc = crate::src::src::main::sqlite3CorruptError(4212 as ::core::ffi::c_int);
                current_block = 7699217338199643514;
                break;
            }
        }
        if !(xOutput.is_some() && sOut.nBuf >= sessions_strm_chunk_size) {
            continue;
        }
        rc = xOutput.expect("non-null function pointer")(
            pOut,
            sOut.aBuf as *const ::core::ffi::c_void,
            sOut.nBuf,
        );
        sOut.nBuf = 0 as ::core::ffi::c_int;
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            current_block = 7699217338199643514;
            break;
        }
    }
    match current_block {
        11441799814184323368 => {
            if !pnInverted.is_null() && !ppInverted.is_null() {
                *pnInverted = sOut.nBuf;
                *ppInverted = sOut.aBuf as *mut ::core::ffi::c_void;
                sOut.aBuf = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
            } else if sOut.nBuf > 0 as ::core::ffi::c_int && xOutput.is_some() {
                rc = xOutput.expect("non-null function pointer")(
                    pOut,
                    sOut.aBuf as *const ::core::ffi::c_void,
                    sOut.nBuf,
                );
            }
        }
        _ => {}
    }
    crate::src::src::malloc::sqlite3_free(sOut.aBuf as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(apVal as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(sPK.aBuf as *mut ::core::ffi::c_void);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_invert(
    nChangeset: ::core::ffi::c_int,
    pChangeset: *const ::core::ffi::c_void,
    pnInverted: *mut ::core::ffi::c_int,
    ppInverted: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sInput: SessionInput = unsafe { ::core::mem::zeroed() };
    sInput.nData = nChangeset;
    sInput.aData = pChangeset as *mut crate::src::ext::rtree::rtree::U8_0;
    sessionChangesetInvert(
        &raw mut sInput,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pnInverted,
        ppInverted,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_invert_strm(
    xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pIn: *mut ::core::ffi::c_void,
    xOutput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pOut: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sInput: SessionInput = unsafe { ::core::mem::zeroed() };
    let rc: ::core::ffi::c_int;
    sInput.xInput = xInput;
    sInput.pIn = pIn;
    rc = sessionChangesetInvert(
        &raw mut sInput,
        xOutput,
        pOut,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    crate::src::src::malloc::sqlite3_free(sInput.buf.aBuf as *mut ::core::ffi::c_void);
    rc
}

pub const SESSION_UPDATE_CACHE_SZ: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

unsafe extern "C" fn sessionUpdateFind(
    pIter: *mut sqlite3_changeset_iter,
    p: *mut SessionApplyCtx,
    bPatchset: ::core::ffi::c_int,
    ppStmt: *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pUp: *mut SessionUpdate = ::core::ptr::null_mut::<SessionUpdate>();
    let nCol: ::core::ffi::c_int = (*pIter).nCol;
    let nU32: ::core::ffi::c_int =
        ((*pIter).nCol + 33 as ::core::ffi::c_int) / 32 as ::core::ffi::c_int;
    let mut ii: ::core::ffi::c_int;
    if (*p).aUpdateMask.is_null() {
        (*p).aUpdateMask =
            crate::src::src::malloc::sqlite3_malloc((nU32 as usize).wrapping_mul(
                ::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>() as usize,
            ) as ::core::ffi::c_int) as *mut crate::src::ext::rtree::rtree::U32_0;
        if (*p).aUpdateMask.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        ::libc::memset(
            (*p).aUpdateMask as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (nU32 as crate::__stddef_size_t_h::SizeT).wrapping_mul(::core::mem::size_of::<
                crate::src::ext::rtree::rtree::U32_0,
            >()
                as crate::__stddef_size_t_h::SizeT),
        );
        rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pIter).nCol {
            if !(*(*pIter).apValue.offset(((*pIter).nCol + ii) as isize)).is_null() {
                *(*p)
                    .aUpdateMask
                    .offset((ii / 32 as ::core::ffi::c_int) as isize) |= ((1 as ::core::ffi::c_int)
                    << ii % 32 as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::U32_0;
                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
            }
            ii += 1;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if bPatchset != 0 {
            *(*p)
                .aUpdateMask
                .offset((nCol / 32 as ::core::ffi::c_int) as isize) |= ((1 as ::core::ffi::c_int)
                << nCol % 32 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U32_0;
        }
        if !(*p).pUp.is_null() {
            let mut nUp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pp: *mut *mut SessionUpdate = &raw mut (*p).pUp;
            loop {
                nUp += 1;
                if 0 as ::core::ffi::c_int
                    == ::libc::memcmp(
                        (*p).aUpdateMask as *const ::core::ffi::c_void,
                        (**pp).aMask as *const ::core::ffi::c_void,
                        (nU32 as crate::__stddef_size_t_h::SizeT).wrapping_mul(
                            ::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                                as crate::__stddef_size_t_h::SizeT,
                        ),
                    )
                {
                    pUp = *pp;
                    *pp = (*pUp).pNext;
                    (*pUp).pNext = (*p).pUp;
                    (*p).pUp = pUp;
                    break;
                } else if !(**pp).pNext.is_null() {
                    pp = &raw mut (**pp).pNext;
                } else {
                    if nUp >= SESSION_UPDATE_CACHE_SZ {
                        crate::src::src::vdbeapi::sqlite3_finalize((**pp).pStmt);
                        crate::src::src::malloc::sqlite3_free(*pp as *mut ::core::ffi::c_void);
                        *pp = ::core::ptr::null_mut::<SessionUpdate>();
                    }
                    break;
                }
            }
        }
        if pUp.is_null() {
            let nByte: ::core::ffi::c_int = (::core::mem::size_of::<SessionUpdate>() as usize)
                .wrapping_mul(nU32 as usize)
                .wrapping_mul(
                    ::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>() as usize,
                ) as ::core::ffi::c_int;
            let bStat1: ::core::ffi::c_int = (crate::src::src::util::sqlite3_stricmp(
                (*pIter).zTab,
                b"sqlite_stat1\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            pUp = crate::src::src::malloc::sqlite3_malloc(nByte) as *mut SessionUpdate;
            if pUp.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                let mut zSep: *const ::core::ffi::c_char =
                    b"\0" as *const u8 as *const ::core::ffi::c_char;
                let mut buf: SessionBuffer = unsafe { ::core::mem::zeroed() };
                (*pUp).aMask = pUp.offset(1 as isize) as *mut SessionUpdate
                    as *mut crate::src::ext::rtree::rtree::U32_0;
                ::core::ptr::copy_nonoverlapping(
                    (*p).aUpdateMask as *const u8,
                    (*pUp).aMask as *mut u8,
                    ((nU32 as crate::__stddef_size_t_h::SizeT).wrapping_mul(
                        ::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                            as crate::__stddef_size_t_h::SizeT,
                    )) as usize,
                );
                sessionAppendStr(
                    &raw mut buf,
                    b"UPDATE main.\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
                let __pIter_ref = unsafe { &mut *pIter };
                sessionAppendIdent(&raw mut buf, __pIter_ref.zTab, &raw mut rc);
                sessionAppendStr(
                    &raw mut buf,
                    b" SET \0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
                ii = 0 as ::core::ffi::c_int;
                while ii < __pIter_ref.nCol {
                    if *(*p).abPK.offset(ii as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && !(*__pIter_ref.apValue.offset((__pIter_ref.nCol + ii) as isize))
                            .is_null()
                    {
                        sessionAppendStr(&raw mut buf, zSep, &raw mut rc);
                        sessionAppendIdent(
                            &raw mut buf,
                            *(*p).azCol.offset(ii as isize),
                            &raw mut rc,
                        );
                        sessionAppendStr(
                            &raw mut buf,
                            b" = ?\0" as *const u8 as *const ::core::ffi::c_char,
                            &raw mut rc,
                        );
                        sessionAppendInteger(
                            &raw mut buf,
                            ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                            &raw mut rc,
                        );
                        zSep = b", \0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    ii += 1;
                }
                zSep = b"\0" as *const u8 as *const ::core::ffi::c_char;
                sessionAppendStr(
                    &raw mut buf,
                    b" WHERE \0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
                ii = 0 as ::core::ffi::c_int;
                while ii < __pIter_ref.nCol {
                    if *(*p).abPK.offset(ii as isize) as ::core::ffi::c_int != 0
                        || bPatchset == 0 as ::core::ffi::c_int
                            && !(*__pIter_ref.apValue.offset(ii as isize)).is_null()
                    {
                        sessionAppendStr(&raw mut buf, zSep, &raw mut rc);
                        if bStat1 != 0 && ii == 1 as ::core::ffi::c_int {
                            sessionAppendStr(
                                &raw mut buf,
                                b"idx IS CASE WHEN length(?4)=0 AND typeof(?4)='blob' THEN NULL ELSE ?4 END \0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                &raw mut rc,
                            );
                        } else {
                            sessionAppendIdent(
                                &raw mut buf,
                                *(*p).azCol.offset(ii as isize),
                                &raw mut rc,
                            );
                            sessionAppendStr(
                                &raw mut buf,
                                b" IS ?\0" as *const u8 as *const ::core::ffi::c_char,
                                &raw mut rc,
                            );
                            sessionAppendInteger(
                                &raw mut buf,
                                ii * 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                                &raw mut rc,
                            );
                        }
                        zSep = b" AND \0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    ii += 1;
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    let zSql: *mut ::core::ffi::c_char = buf.aBuf as *mut ::core::ffi::c_char;
                    rc = crate::src::src::prepare::sqlite3_prepare_v2(
                        (*p).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        zSql,
                        buf.nBuf,
                        &raw mut (*pUp).pStmt,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    );
                }
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    crate::src::src::malloc::sqlite3_free(pUp as *mut ::core::ffi::c_void);
                    pUp = ::core::ptr::null_mut::<SessionUpdate>();
                } else {
                    (*pUp).pNext = (*p).pUp;
                    (*p).pUp = pUp;
                }
                crate::src::src::malloc::sqlite3_free(buf.aBuf as *mut ::core::ffi::c_void);
            }
        }
    }
    if !pUp.is_null() {
        *ppStmt = (*pUp).pStmt;
    } else {
        *ppStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    }
    rc
}

unsafe extern "C" fn sessionUpdateFree(p: *mut SessionApplyCtx) {
    let mut pUp: *mut SessionUpdate;
    let mut pNext: *mut SessionUpdate;
    let __p_ref = unsafe { &mut *p };
    pUp = __p_ref.pUp;
    while !pUp.is_null() {
        pNext = (*pUp).pNext;
        crate::src::src::vdbeapi::sqlite3_finalize((*pUp).pStmt);
        crate::src::src::malloc::sqlite3_free(pUp as *mut ::core::ffi::c_void);
        pUp = pNext;
    }
    __p_ref.pUp = ::core::ptr::null_mut::<SessionUpdate>();
    crate::src::src::malloc::sqlite3_free(__p_ref.aUpdateMask as *mut ::core::ffi::c_void);
    __p_ref.aUpdateMask = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U32_0>();
}

unsafe extern "C" fn sessionDeleteRow(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zTab: *const ::core::ffi::c_char,
    p: *mut SessionApplyCtx,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut zSep: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut buf: SessionBuffer = unsafe { ::core::mem::zeroed() };
    let mut nPk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sessionAppendStr(
        &raw mut buf,
        b"DELETE FROM main.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut rc,
    );
    sessionAppendIdent(&raw mut buf, zTab, &raw mut rc);
    sessionAppendStr(
        &raw mut buf,
        b" WHERE \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut rc,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nCol {
        if *(*p).abPK.offset(i as isize) != 0 {
            nPk += 1;
            sessionAppendStr(&raw mut buf, zSep, &raw mut rc);
            sessionAppendIdent(&raw mut buf, *(*p).azCol.offset(i as isize), &raw mut rc);
            sessionAppendStr(
                &raw mut buf,
                b" = ?\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut rc,
            );
            sessionAppendInteger(&raw mut buf, i + 1 as ::core::ffi::c_int, &raw mut rc);
            zSep = b" AND \0" as *const u8 as *const ::core::ffi::c_char;
        }
        i += 1;
    }
    if nPk < (*p).nCol {
        sessionAppendStr(
            &raw mut buf,
            b" AND (?\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut rc,
        );
        sessionAppendInteger(
            &raw mut buf,
            (*p).nCol + 1 as ::core::ffi::c_int,
            &raw mut rc,
        );
        sessionAppendStr(
            &raw mut buf,
            b" OR \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut rc,
        );
        zSep = b"\0" as *const u8 as *const ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nCol {
            if *(*p).abPK.offset(i as isize) == 0 {
                sessionAppendStr(&raw mut buf, zSep, &raw mut rc);
                sessionAppendIdent(&raw mut buf, *(*p).azCol.offset(i as isize), &raw mut rc);
                sessionAppendStr(
                    &raw mut buf,
                    b" IS ?\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                );
                sessionAppendInteger(&raw mut buf, i + 1 as ::core::ffi::c_int, &raw mut rc);
                zSep = b"AND \0" as *const u8 as *const ::core::ffi::c_char;
            }
            i += 1;
        }
        sessionAppendStr(
            &raw mut buf,
            b")\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut rc,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionPrepare(
            db,
            &raw mut (*p).pDelete,
            &raw mut (*p).zErr,
            buf.aBuf as *mut ::core::ffi::c_char,
        );
    }
    crate::src::src::malloc::sqlite3_free(buf.aBuf as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn sessionSelectRow(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zTab: *const ::core::ffi::c_char,
    p: *mut SessionApplyCtx,
) -> ::core::ffi::c_int {
    let __p_ref = unsafe { &mut *p };
    sessionSelectStmt(
        db,
        __p_ref.bIgnoreNoop as ::core::ffi::c_int,
        b"main\0" as *const u8 as *const ::core::ffi::c_char,
        zTab,
        __p_ref.bRowid,
        __p_ref.nCol,
        __p_ref.azCol,
        __p_ref.abPK,
        &raw mut __p_ref.pSelect,
        &raw mut __p_ref.zErr,
    )
}

unsafe extern "C" fn sessionInsertRow(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zTab: *const ::core::ffi::c_char,
    p: *mut SessionApplyCtx,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i: ::core::ffi::c_int;
    let mut buf: SessionBuffer = unsafe { ::core::mem::zeroed() };
    sessionAppendStr(
        &raw mut buf,
        b"INSERT INTO main.\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut rc,
    );
    sessionAppendIdent(&raw mut buf, zTab, &raw mut rc);
    sessionAppendStr(
        &raw mut buf,
        b"(\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut rc,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nCol {
        if i != 0 as ::core::ffi::c_int {
            sessionAppendStr(
                &raw mut buf,
                b", \0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut rc,
            );
        }
        sessionAppendIdent(&raw mut buf, *(*p).azCol.offset(i as isize), &raw mut rc);
        i += 1;
    }
    sessionAppendStr(
        &raw mut buf,
        b") VALUES(?\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut rc,
    );
    i = 1 as ::core::ffi::c_int;
    while i < (*p).nCol {
        sessionAppendStr(
            &raw mut buf,
            b", ?\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut rc,
        );
        i += 1;
    }
    sessionAppendStr(
        &raw mut buf,
        b")\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut rc,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionPrepare(
            db,
            &raw mut (*p).pInsert,
            &raw mut (*p).zErr,
            buf.aBuf as *mut ::core::ffi::c_char,
        );
    }
    crate::src::src::malloc::sqlite3_free(buf.aBuf as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn sessionStat1Sql(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut SessionApplyCtx,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = sessionSelectRow(
        db,
        b"sqlite_stat1\0" as *const u8 as *const ::core::ffi::c_char,
        p,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionPrepare(
            db,
            &raw mut (*p).pInsert,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            b"INSERT INTO main.sqlite_stat1 VALUES(?1, CASE WHEN length(?2)=0 AND typeof(?2)='blob' THEN NULL ELSE ?2 END, ?3)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionPrepare(
            db,
            &raw mut (*p).pDelete,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            b"DELETE FROM main.sqlite_stat1 WHERE tbl=?1 AND idx IS CASE WHEN length(?2)=0 AND typeof(?2)='blob' THEN NULL ELSE ?2 END AND (?4 OR stat IS ?3)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    rc
}

unsafe extern "C" fn sessionBindValue(
    pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    i: ::core::ffi::c_int,
    pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let eType: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_value_type(
        pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    );
    if (eType == crate::src::headers::sqlite3_h::SQLITE_TEXT
        || eType == crate::src::headers::sqlite3_h::SQLITE_BLOB)
        && (*pVal).z.is_null()
    {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    crate::src::src::vdbeapi::sqlite3_bind_value(
        pStmt,
        i,
        pVal as *const crate::src::headers::vdbeInt_h::sqlite3_value,
    )
}

unsafe extern "C" fn sessionBindRow(
    pIter: *mut sqlite3_changeset_iter,
    xValue: Option<
        unsafe extern "C" fn(
            *mut sqlite3_changeset_iter,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    nCol: ::core::ffi::c_int,
    abPK: *mut crate::src::ext::rtree::rtree::U8_0,
    pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    i = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < nCol {
        if abPK.is_null() || *abPK.offset(i as isize) as ::core::ffi::c_int != 0 {
            let mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
            xValue.expect("non-null function pointer")(pIter, i, &raw mut pVal);
            if pVal.is_null() {
                rc = crate::src::src::main::sqlite3CorruptError(4691 as ::core::ffi::c_int);
            } else {
                rc = sessionBindValue(pStmt, i + 1 as ::core::ffi::c_int, pVal);
            }
        }
        i += 1;
    }
    rc
}

unsafe extern "C" fn sessionSeekToRow(
    pIter: *mut sqlite3_changeset_iter,
    p: *mut SessionApplyCtx,
) -> ::core::ffi::c_int {
    let __p_ref = unsafe { &mut *p };
    let pSelect: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt = __p_ref.pSelect;
    let mut rc: ::core::ffi::c_int;
    let mut nCol: ::core::ffi::c_int = 0;
    let mut op: ::core::ffi::c_int = 0;
    let mut zDummy: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    crate::src::src::vdbeapi::sqlite3_clear_bindings(pSelect);
    sqlite3changeset_op(
        pIter,
        &raw mut zDummy,
        &raw mut nCol,
        &raw mut op,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    rc = sessionBindRow(
        pIter,
        if op == crate::src::headers::sqlite3_h::SQLITE_INSERT {
            Some(
                sqlite3changeset_new
                    as unsafe extern "C" fn(
                        *mut sqlite3_changeset_iter,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> ::core::ffi::c_int,
            )
        } else {
            Some(
                sqlite3changeset_old
                    as unsafe extern "C" fn(
                        *mut sqlite3_changeset_iter,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> ::core::ffi::c_int,
            )
        },
        nCol,
        __p_ref.abPK,
        pSelect,
    );
    if op != crate::src::headers::sqlite3_h::SQLITE_DELETE
        && __p_ref.bIgnoreNoop as ::core::ffi::c_int != 0
    {
        let mut ii: ::core::ffi::c_int;
        ii = 0 as ::core::ffi::c_int;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && ii < nCol {
            if *__p_ref.abPK.offset(ii as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                    ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
                sqlite3changeset_new(pIter, ii, &raw mut pVal);
                crate::src::src::vdbeapi::sqlite3_bind_int(
                    pSelect,
                    ii + 1 as ::core::ffi::c_int + nCol,
                    (pVal
                        == ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>())
                        as ::core::ffi::c_int,
                );
                if !pVal.is_null() {
                    rc = sessionBindValue(pSelect, ii + 1 as ::core::ffi::c_int, pVal);
                }
            }
            ii += 1;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::src::vdbeapi::sqlite3_step(pSelect);
        if rc != crate::src::headers::sqlite3_h::SQLITE_ROW {
            rc = crate::src::src::vdbeapi::sqlite3_reset(pSelect);
        }
    }
    rc
}

unsafe extern "C" fn sessionRebaseAdd(
    p: *mut SessionApplyCtx,
    eType: ::core::ffi::c_int,
    pIter: *mut sqlite3_changeset_iter,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*p).bRebase != 0 {
        let mut i: ::core::ffi::c_int;
        let eOp: ::core::ffi::c_int = (*pIter).op;
        let __p_ref = unsafe { &mut *p };
        if __p_ref.bRebaseStarted as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let zTab: *const ::core::ffi::c_char = (*pIter).zTab;
            sessionAppendByte(
                &raw mut __p_ref.rebase,
                'T' as i32 as crate::src::ext::rtree::rtree::U8_0,
                &raw mut rc,
            );
            sessionAppendVarint(&raw mut __p_ref.rebase, __p_ref.nCol, &raw mut rc);
            sessionAppendBlob(
                &raw mut __p_ref.rebase,
                __p_ref.abPK,
                __p_ref.nCol,
                &raw mut rc,
            );
            sessionAppendBlob(
                &raw mut __p_ref.rebase,
                zTab as *mut crate::src::ext::rtree::rtree::U8_0,
                ::libc::strlen(zTab) as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                &raw mut rc,
            );
            __p_ref.bRebaseStarted = 1 as crate::src::ext::rtree::rtree::U8_0;
        }
        sessionAppendByte(
            &raw mut __p_ref.rebase,
            (if eOp == crate::src::headers::sqlite3_h::SQLITE_DELETE {
                crate::src::headers::sqlite3_h::SQLITE_DELETE
            } else {
                crate::src::headers::sqlite3_h::SQLITE_INSERT
            }) as crate::src::ext::rtree::rtree::U8_0,
            &raw mut rc,
        );
        sessionAppendByte(
            &raw mut __p_ref.rebase,
            (eType == crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_REPLACE)
                as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U8_0,
            &raw mut rc,
        );
        i = 0 as ::core::ffi::c_int;
        while i < __p_ref.nCol {
            let mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
            if eOp == crate::src::headers::sqlite3_h::SQLITE_DELETE
                || eOp == crate::src::headers::sqlite3_h::SQLITE_UPDATE
                    && *__p_ref.abPK.offset(i as isize) as ::core::ffi::c_int != 0
            {
                sqlite3changeset_old(pIter, i, &raw mut pVal);
            } else {
                sqlite3changeset_new(pIter, i, &raw mut pVal);
            }
            sessionAppendValue(&raw mut __p_ref.rebase, pVal, &raw mut rc);
            i += 1;
        }
    }
    rc
}

unsafe extern "C" fn sessionConflictHandler(
    eType: ::core::ffi::c_int,
    p: *mut SessionApplyCtx,
    pIter: *mut sqlite3_changeset_iter,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
    pbReplace: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int =
        crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_OMIT;
    let mut rc: ::core::ffi::c_int;
    let mut nCol: ::core::ffi::c_int = 0;
    let mut op: ::core::ffi::c_int = 0;
    let mut zDummy: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    sqlite3changeset_op(
        pIter,
        &raw mut zDummy,
        &raw mut nCol,
        &raw mut op,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if !pbReplace.is_null() {
        rc = sessionSeekToRow(pIter, p);
    } else {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
        let __p_ref = unsafe { &*p };
        if 0 as ::core::ffi::c_int == __p_ref.bIgnoreNoop as ::core::ffi::c_int
            || 0 as ::core::ffi::c_int
                == crate::src::src::vdbeapi::sqlite3_column_int(
                    __p_ref.pSelect,
                    crate::src::src::vdbeapi::sqlite3_column_count(__p_ref.pSelect)
                        - 1 as ::core::ffi::c_int,
                )
        {
            (*pIter).pConflict = __p_ref.pSelect;
            res = xConflict.expect("non-null function pointer")(pCtx, eType, pIter);
            (*pIter).pConflict =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(__p_ref.pSelect);
    } else if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if (*p).bDeferConstraints != 0
            && eType == crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_CONFLICT
        {
            let __pIter_ref = unsafe { &mut *pIter };
            let aBlob: *mut crate::src::ext::rtree::rtree::U8_0 = __pIter_ref
                .in_0
                .aData
                .offset(__pIter_ref.in_0.iCurrent as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0;
            let nBlob: ::core::ffi::c_int = __pIter_ref.in_0.iNext - __pIter_ref.in_0.iCurrent;
            sessionAppendBlob(&raw mut (*p).constraints, aBlob, nBlob, &raw mut rc);
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        } else if (*p).bIgnoreNoop as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || op != crate::src::headers::sqlite3_h::SQLITE_DELETE
            || eType == crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_CONFLICT
        {
            res = xConflict.expect("non-null function pointer")(
                pCtx,
                eType + 1 as ::core::ffi::c_int,
                pIter,
            );
            if res == crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_REPLACE {
                rc = crate::src::headers::sqlite3_h::SQLITE_MISUSE;
            }
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        match res {
            crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_REPLACE => {
                *pbReplace = 1 as ::core::ffi::c_int;
            }
            crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_OMIT => {}
            crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_ABORT => {
                rc = crate::src::headers::sqlite3_h::SQLITE_ABORT;
            }
            _ => {
                rc = crate::src::headers::sqlite3_h::SQLITE_MISUSE;
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = sessionRebaseAdd(p, res, pIter);
        }
    }
    rc
}

unsafe extern "C" fn sessionApplyOneOp(
    pIter: *mut sqlite3_changeset_iter,
    p: *mut SessionApplyCtx,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
    pbReplace: *mut ::core::ffi::c_int,
    pbRetry: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut zDummy: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut op: ::core::ffi::c_int = 0;
    let mut nCol: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    sqlite3changeset_op(
        pIter,
        &raw mut zDummy,
        &raw mut nCol,
        &raw mut op,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if op == crate::src::headers::sqlite3_h::SQLITE_DELETE {
        let __p_ref = unsafe { &*p };
        let abPK: *mut crate::src::ext::rtree::rtree::U8_0 = if (*pIter).bPatchset != 0 {
            __p_ref.abPK
        } else {
            ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>()
        };
        rc = sessionBindRow(
            pIter,
            Some(
                sqlite3changeset_old
                    as unsafe extern "C" fn(
                        *mut sqlite3_changeset_iter,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> ::core::ffi::c_int,
            ),
            nCol,
            abPK,
            __p_ref.pDelete,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && crate::src::src::vdbeapi::sqlite3_bind_parameter_count(__p_ref.pDelete) > nCol
        {
            rc = crate::src::src::vdbeapi::sqlite3_bind_int(
                __p_ref.pDelete,
                nCol + 1 as ::core::ffi::c_int,
                (pbRetry.is_null() || !abPK.is_null()) as ::core::ffi::c_int,
            );
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        crate::src::src::vdbeapi::sqlite3_step(__p_ref.pDelete);
        rc = crate::src::src::vdbeapi::sqlite3_reset(__p_ref.pDelete);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && crate::src::src::main::sqlite3_changes(
                __p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            ) == 0 as ::core::ffi::c_int
        {
            rc = sessionConflictHandler(
                crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_DATA,
                p,
                pIter,
                xConflict,
                pCtx,
                pbRetry,
            );
        } else if rc & 0xff as ::core::ffi::c_int
            == crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT
        {
            rc = sessionConflictHandler(
                crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_CONFLICT,
                p,
                pIter,
                xConflict,
                pCtx,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        }
    } else if op == crate::src::headers::sqlite3_h::SQLITE_UPDATE {
        let mut i: ::core::ffi::c_int;
        let mut pUp: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        let bPatchset: ::core::ffi::c_int =
            (pbRetry.is_null() || (*pIter).bPatchset != 0) as ::core::ffi::c_int;
        rc = sessionUpdateFind(pIter, p, bPatchset, &raw mut pUp);
        i = 0 as ::core::ffi::c_int;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < nCol {
            let __pIter_ref = unsafe { &mut *pIter };
            let pOld: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                *__pIter_ref.apValue.offset(i as isize);
            let pNew: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                *__pIter_ref.apValue.offset((__pIter_ref.nCol + i) as isize);
            if *(*p).abPK.offset(i as isize) as ::core::ffi::c_int != 0
                || bPatchset == 0 as ::core::ffi::c_int && !pOld.is_null()
            {
                rc = sessionBindValue(
                    pUp,
                    i * 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                    pOld,
                );
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pNew.is_null() {
                rc = sessionBindValue(
                    pUp,
                    i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                    pNew,
                );
            }
            i += 1;
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        crate::src::src::vdbeapi::sqlite3_step(pUp);
        rc = crate::src::src::vdbeapi::sqlite3_reset(pUp);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && crate::src::src::main::sqlite3_changes(
                (*p).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            ) == 0 as ::core::ffi::c_int
        {
            rc = sessionConflictHandler(
                crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_DATA,
                p,
                pIter,
                xConflict,
                pCtx,
                pbRetry,
            );
        } else if rc & 0xff as ::core::ffi::c_int
            == crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT
        {
            rc = sessionConflictHandler(
                crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_CONFLICT,
                p,
                pIter,
                xConflict,
                pCtx,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        }
    } else {
        if (*p).bStat1 != 0 {
            rc = sessionSeekToRow(pIter, p);
            if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
                rc = crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
                crate::src::src::vdbeapi::sqlite3_reset((*p).pSelect);
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let __p_ref = unsafe { &*p };
            rc = sessionBindRow(
                pIter,
                Some(
                    sqlite3changeset_new
                        as unsafe extern "C" fn(
                            *mut sqlite3_changeset_iter,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> ::core::ffi::c_int,
                ),
                nCol,
                ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
                __p_ref.pInsert,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            crate::src::src::vdbeapi::sqlite3_step(__p_ref.pInsert);
            rc = crate::src::src::vdbeapi::sqlite3_reset(__p_ref.pInsert);
        }
        if rc & 0xff as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT {
            rc = sessionConflictHandler(
                crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_CONFLICT,
                p,
                pIter,
                xConflict,
                pCtx,
                pbReplace,
            );
        }
    }
    rc
}

unsafe extern "C" fn sessionApplyOneWithRetry(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pIter: *mut sqlite3_changeset_iter,
    pApply: *mut SessionApplyCtx,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut bReplace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bRetry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    rc = sessionApplyOneOp(
        pIter,
        pApply,
        xConflict,
        pCtx,
        &raw mut bReplace,
        &raw mut bRetry,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if bRetry != 0 {
            rc = sessionApplyOneOp(
                pIter,
                pApply,
                xConflict,
                pCtx,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        } else if bReplace != 0 {
            rc = crate::src::src::legacy::sqlite3_exec(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"SAVEPOINT replace_op\0" as *const u8 as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let __pApply_ref = unsafe { &*pApply };
                rc = sessionBindRow(
                    pIter,
                    Some(
                        sqlite3changeset_new
                            as unsafe extern "C" fn(
                                *mut sqlite3_changeset_iter,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    __pApply_ref.nCol,
                    __pApply_ref.abPK,
                    __pApply_ref.pDelete,
                );
                crate::src::src::vdbeapi::sqlite3_bind_int(
                    __pApply_ref.pDelete,
                    __pApply_ref.nCol + 1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::vdbeapi::sqlite3_step((*pApply).pDelete);
                rc = crate::src::src::vdbeapi::sqlite3_reset((*pApply).pDelete);
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = sessionApplyOneOp(
                    pIter,
                    pApply,
                    xConflict,
                    pCtx,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::src::src::legacy::sqlite3_exec(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    b"RELEASE replace_op\0" as *const u8 as *const ::core::ffi::c_char,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
            }
        }
    }
    rc
}

unsafe extern "C" fn sessionRetryConstraints(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    bPatchset: ::core::ffi::c_int,
    zTab: *const ::core::ffi::c_char,
    pApply: *mut SessionApplyCtx,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    while (*pApply).constraints.nBuf != 0 {
        let mut pIter2: *mut sqlite3_changeset_iter =
            ::core::ptr::null_mut::<sqlite3_changeset_iter>();
        let __pApply_ref = unsafe { &mut *pApply };
        let cons: SessionBuffer = __pApply_ref.constraints;
        ::libc::memset(
            &raw mut __pApply_ref.constraints as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SessionBuffer>() as crate::__stddef_size_t_h::SizeT,
        );
        rc = sessionChangesetStart(
            &raw mut pIter2,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            cons.nBuf,
            cons.aBuf as *mut ::core::ffi::c_void,
            __pApply_ref.bInvertConstraints,
            1 as ::core::ffi::c_int,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let nByte: crate::__stddef_size_t_h::SizeT =
                ((2 as ::core::ffi::c_int * __pApply_ref.nCol) as crate::__stddef_size_t_h::SizeT)
                    .wrapping_mul(::core::mem::size_of::<
                        *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    >() as crate::__stddef_size_t_h::SizeT);
            let rc2: ::core::ffi::c_int;
            let __pIter2_ref = unsafe { &mut *pIter2 };
            __pIter2_ref.bPatchset = bPatchset;
            __pIter2_ref.zTab = zTab as *mut ::core::ffi::c_char;
            __pIter2_ref.nCol = __pApply_ref.nCol;
            __pIter2_ref.abPK = __pApply_ref.abPK;
            sessionBufferGrow(
                &raw mut __pIter2_ref.tblhdr,
                nByte as crate::src::ext::rtree::rtree::I64_0,
                &raw mut rc,
            );
            __pIter2_ref.apValue =
                __pIter2_ref.tblhdr.aBuf as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value;
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                ::libc::memset(
                    __pIter2_ref.apValue as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nByte,
                );
            }
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK
                && crate::src::headers::sqlite3_h::SQLITE_ROW == sqlite3changeset_next(pIter2)
            {
                rc = sessionApplyOneWithRetry(db, pIter2, pApply, xConflict, pCtx);
            }
            rc2 = sqlite3changeset_finalize(pIter2);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = rc2;
            }
        }
        crate::src::src::malloc::sqlite3_free(cons.aBuf as *mut ::core::ffi::c_void);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            break;
        }
        if __pApply_ref.constraints.nBuf >= cons.nBuf {
            __pApply_ref.bDeferConstraints = 0 as ::core::ffi::c_int;
        }
    }
    rc
}

unsafe extern "C" fn sessionChangesetApply(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pIter: *mut sqlite3_changeset_iter,
    xFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    xFilterIter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
    ppRebase: *mut *mut ::core::ffi::c_void,
    pnRebase: *mut ::core::ffi::c_int,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut schemaMismatch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut zTab: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nTab: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sApply: SessionApplyCtx = unsafe { ::core::mem::zeroed() };
    let bPatchset: ::core::ffi::c_int;
    let savedFlag: crate::src::ext::rtree::rtree::U64_0 =
        (*db).flags & crate::src::headers::sqliteInt_h::SQLITE_FkNoAction;
    crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::main::sqlite3_db_mutex(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    if flags & crate::src::ext::session::sqlite3session::SQLITE_CHANGESETAPPLY_FKNOACTION != 0 {
        (*db).flags |= crate::src::headers::sqliteInt_h::SQLITE_FkNoAction;
        (*(*(*db).aDb.offset(0 as isize)).pSchema).schema_cookie -= 32 as ::core::ffi::c_int;
    }
    (*pIter).in_0.bNoDiscard = 1 as ::core::ffi::c_int;
    sApply.bRebase = (!ppRebase.is_null() && !pnRebase.is_null()) as ::core::ffi::c_int
        as crate::src::ext::rtree::rtree::U8_0;
    sApply.bInvertConstraints = (flags
        & crate::src::ext::session::sqlite3session::SQLITE_CHANGESETAPPLY_INVERT
        != 0) as ::core::ffi::c_int;
    sApply.bIgnoreNoop = (flags
        & crate::src::ext::session::sqlite3session::SQLITE_CHANGESETAPPLY_IGNORENOOP
        != 0) as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U8_0;
    if flags & crate::src::ext::session::sqlite3session::SQLITE_CHANGESETAPPLY_NOSAVEPOINT
        == 0 as ::core::ffi::c_int
    {
        rc = crate::src::src::legacy::sqlite3_exec(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            b"SAVEPOINT changeset_apply\0" as *const u8 as *const ::core::ffi::c_char,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::src::legacy::sqlite3_exec(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            b"PRAGMA defer_foreign_keys = 1\0" as *const u8 as *const ::core::ffi::c_char,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && crate::src::headers::sqlite3_h::SQLITE_ROW == sqlite3changeset_next(pIter)
    {
        let mut nCol: ::core::ffi::c_int = 0;
        let mut op: ::core::ffi::c_int = 0;
        let mut zNew: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        sqlite3changeset_op(
            pIter,
            &raw mut zNew,
            &raw mut nCol,
            &raw mut op,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if zTab.is_null()
            || crate::src::src::util::sqlite3_strnicmp(zNew, zTab, nTab + 1 as ::core::ffi::c_int)
                != 0
        {
            let mut abPK: *mut crate::src::ext::rtree::rtree::U8_0 =
                ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
            rc = sessionRetryConstraints(
                db,
                (*pIter).bPatchset,
                zTab,
                &raw mut sApply,
                xConflict,
                pCtx,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                break;
            }
            sessionUpdateFree(&raw mut sApply);
            crate::src::src::malloc::sqlite3_free(
                sApply.azCol as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
            crate::src::src::vdbeapi::sqlite3_finalize(sApply.pDelete);
            crate::src::src::vdbeapi::sqlite3_finalize(sApply.pInsert);
            crate::src::src::vdbeapi::sqlite3_finalize(sApply.pSelect);
            sApply.db = db;
            sApply.pDelete =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
            sApply.pInsert =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
            sApply.pSelect =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
            sApply.nCol = 0 as ::core::ffi::c_int;
            sApply.azCol = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
            sApply.abPK = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
            sApply.bStat1 = 0 as ::core::ffi::c_int;
            sApply.bDeferConstraints = 1 as ::core::ffi::c_int;
            sApply.bRebaseStarted = 0 as crate::src::ext::rtree::rtree::U8_0;
            sApply.bRowid = 0 as ::core::ffi::c_int;
            ::libc::memset(
                &raw mut sApply.constraints as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<SessionBuffer>() as crate::__stddef_size_t_h::SizeT,
            );
            schemaMismatch = (xFilter.is_some()
                && 0 as ::core::ffi::c_int
                    == xFilter.expect("non-null function pointer")(pCtx, zNew))
                as ::core::ffi::c_int;
            if schemaMismatch != 0 {
                zTab = sqlite_printf!("%s", zNew);
                if zTab.is_null() {
                    rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    break;
                } else {
                    nTab = ::libc::strlen(zTab) as ::core::ffi::c_int;
                    sApply.azCol = zTab as *mut *const ::core::ffi::c_char;
                }
            } else {
                let mut nMinCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut i: ::core::ffi::c_int;
                sqlite3changeset_pk(
                    pIter,
                    &raw mut abPK,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                rc = sessionTableInfo(
                    ::core::ptr::null_mut::<sqlite3_session>(),
                    db,
                    b"main\0" as *const u8 as *const ::core::ffi::c_char,
                    zNew,
                    &raw mut sApply.nCol,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    &raw mut zTab,
                    &raw mut sApply.azCol,
                    ::core::ptr::null_mut::<*mut *const ::core::ffi::c_char>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_int>(),
                    &raw mut sApply.abPK,
                    &raw mut sApply.bRowid,
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    break;
                }
                i = 0 as ::core::ffi::c_int;
                while i < sApply.nCol {
                    if *sApply.abPK.offset(i as isize) != 0 {
                        nMinCol = i + 1 as ::core::ffi::c_int;
                    }
                    i += 1;
                }
                if sApply.nCol == 0 as ::core::ffi::c_int {
                    schemaMismatch = 1 as ::core::ffi::c_int;
                    crate::src::printf_c_variadic::sqlite3_log_args(
                        crate::src::headers::sqlite3_h::SQLITE_SCHEMA,
                        b"sqlite3changeset_apply(): no such table: %s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[crate::src::src::printf::PrintfArg::Str(
                            zTab as *mut ::core::ffi::c_char,
                        )],
                    );
                } else if sApply.nCol < nCol {
                    schemaMismatch = 1 as ::core::ffi::c_int;
                    crate::src::printf_c_variadic::sqlite3_log_args(
                        crate::src::headers::sqlite3_h::SQLITE_SCHEMA,
                        b"sqlite3changeset_apply(): table %s has %d columns, expected %d or more\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        &[
                            crate::src::src::printf::PrintfArg::Str(
                                zTab as *mut ::core::ffi::c_char,
                            ),
                            crate::src::src::printf::PrintfArg::Int(
                                sApply.nCol as crate::src::ext::rtree::rtree::I64_0,
                            ),
                            crate::src::src::printf::PrintfArg::Int(
                                nCol as crate::src::ext::rtree::rtree::I64_0,
                            ),
                        ],
                    );
                } else if nCol < nMinCol
                    || ::libc::memcmp(
                        sApply.abPK as *const ::core::ffi::c_void,
                        abPK as *const ::core::ffi::c_void,
                        nCol as crate::__stddef_size_t_h::SizeT,
                    ) != 0 as ::core::ffi::c_int
                {
                    schemaMismatch = 1 as ::core::ffi::c_int;
                    crate::src::printf_c_variadic::sqlite3_log_args(
                        crate::src::headers::sqlite3_h::SQLITE_SCHEMA,
                        b"sqlite3changeset_apply(): primary key mismatch for table %s\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        &[crate::src::src::printf::PrintfArg::Str(
                            zTab as *mut ::core::ffi::c_char,
                        )],
                    );
                } else {
                    sApply.nCol = nCol;
                    if 0 as ::core::ffi::c_int
                        == crate::src::src::util::sqlite3_stricmp(
                            zTab,
                            b"sqlite_stat1\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                    {
                        rc = sessionStat1Sql(db, &raw mut sApply);
                        if rc != 0 {
                            break;
                        }
                        sApply.bStat1 = 1 as ::core::ffi::c_int;
                    } else {
                        rc = sessionSelectRow(db, zTab, &raw mut sApply);
                        if rc != 0
                            || {
                                rc = sessionDeleteRow(db, zTab, &raw mut sApply);
                                rc != 0
                            }
                            || {
                                rc = sessionInsertRow(db, zTab, &raw mut sApply);
                                rc != 0
                            }
                        {
                            break;
                        }
                        sApply.bStat1 = 0 as ::core::ffi::c_int;
                    }
                }
                nTab = crate::src::src::util::sqlite3Strlen30(zTab);
            }
        }
        if schemaMismatch != 0 {
            continue;
        }
        if xFilterIter.is_some()
            && 0 as ::core::ffi::c_int
                == xFilterIter.expect("non-null function pointer")(pCtx, pIter)
        {
            continue;
        }
        rc = sessionApplyOneWithRetry(db, pIter, &raw mut sApply, xConflict, pCtx);
    }
    bPatchset = (*pIter).bPatchset;
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3changeset_finalize(pIter);
    } else {
        sqlite3changeset_finalize(pIter);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionRetryConstraints(db, bPatchset, zTab, &raw mut sApply, xConflict, pCtx);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut nFk: ::core::ffi::c_int = 0;
        let mut notUsed: ::core::ffi::c_int = 0;
        crate::src::src::status::sqlite3_db_status(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            crate::src::headers::sqlite3_h::SQLITE_DBSTATUS_DEFERRED_FKS,
            &raw mut nFk,
            &raw mut notUsed,
            0 as ::core::ffi::c_int,
        );
        if nFk != 0 as ::core::ffi::c_int {
            let res: ::core::ffi::c_int;
            let mut sIter: sqlite3_changeset_iter = unsafe { ::core::mem::zeroed() };
            sIter.nCol = nFk;
            res = xConflict.expect("non-null function pointer")(
                pCtx,
                crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_FOREIGN_KEY,
                &raw mut sIter,
            );
            if res != crate::src::ext::session::sqlite3session::SQLITE_CHANGESET_OMIT {
                rc = crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
            }
        }
    }
    let rc2: ::core::ffi::c_int = crate::src::src::legacy::sqlite3_exec(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        b"PRAGMA defer_foreign_keys = 0\0" as *const u8 as *const ::core::ffi::c_char,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = rc2;
    }
    if flags & crate::src::ext::session::sqlite3session::SQLITE_CHANGESETAPPLY_NOSAVEPOINT
        == 0 as ::core::ffi::c_int
    {
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = crate::src::src::legacy::sqlite3_exec(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"RELEASE changeset_apply\0" as *const u8 as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::legacy::sqlite3_exec(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"ROLLBACK TO changeset_apply\0" as *const u8 as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            crate::src::src::legacy::sqlite3_exec(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"RELEASE changeset_apply\0" as *const u8 as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && bPatchset == 0 as ::core::ffi::c_int
        && sApply.bRebase as ::core::ffi::c_int != 0
    {
        *ppRebase = sApply.rebase.aBuf as *mut ::core::ffi::c_void;
        *pnRebase = sApply.rebase.nBuf;
        sApply.rebase.aBuf = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    }
    sessionUpdateFree(&raw mut sApply);
    crate::src::src::vdbeapi::sqlite3_finalize(sApply.pInsert);
    crate::src::src::vdbeapi::sqlite3_finalize(sApply.pDelete);
    crate::src::src::vdbeapi::sqlite3_finalize(sApply.pSelect);
    crate::src::src::malloc::sqlite3_free(
        sApply.azCol as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
    );
    crate::src::src::malloc::sqlite3_free(
        sApply.constraints.aBuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
    );
    crate::src::src::malloc::sqlite3_free(
        sApply.rebase.aBuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
    );
    if flags & crate::src::ext::session::sqlite3session::SQLITE_CHANGESETAPPLY_FKNOACTION != 0
        && savedFlag == 0 as crate::src::ext::rtree::rtree::U64_0
    {
        (*db).flags &= !crate::src::headers::sqliteInt_h::SQLITE_FkNoAction;
        (*(*(*db).aDb.offset(0 as isize)).pSchema).schema_cookie -= 32 as ::core::ffi::c_int;
    }
    crate::src::src::main::sqlite3_set_errmsg(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        rc,
        sApply.zErr,
    );
    crate::src::src::malloc::sqlite3_free(sApply.zErr as *mut ::core::ffi::c_void);
    crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::main::sqlite3_db_mutex(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    ));
    rc
}

unsafe extern "C" fn sessionChangesetApplyV23(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    nChangeset: ::core::ffi::c_int,
    pChangeset: *mut ::core::ffi::c_void,
    xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pIn: *mut ::core::ffi::c_void,
    xFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    xFilterIter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
    ppRebase: *mut *mut ::core::ffi::c_void,
    pnRebase: *mut ::core::ffi::c_int,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pIter: *mut sqlite3_changeset_iter = ::core::ptr::null_mut::<sqlite3_changeset_iter>();
    let bInverse: ::core::ffi::c_int = (flags
        & crate::src::ext::session::sqlite3session::SQLITE_CHANGESETAPPLY_INVERT
        != 0) as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = sessionChangesetStart(
        &raw mut pIter,
        xInput,
        pIn,
        nChangeset,
        pChangeset,
        bInverse,
        1 as ::core::ffi::c_int,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionChangesetApply(
            db,
            pIter,
            xFilter,
            xFilterIter,
            xConflict,
            pCtx,
            ppRebase,
            pnRebase,
            flags,
        );
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_apply_v2(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    nChangeset: ::core::ffi::c_int,
    pChangeset: *mut ::core::ffi::c_void,
    xFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
    ppRebase: *mut *mut ::core::ffi::c_void,
    pnRebase: *mut ::core::ffi::c_int,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    sessionChangesetApplyV23(
        db,
        nChangeset,
        pChangeset,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        xFilter,
        None,
        xConflict,
        pCtx,
        ppRebase,
        pnRebase,
        flags,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_apply_v3(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    nChangeset: ::core::ffi::c_int,
    pChangeset: *mut ::core::ffi::c_void,
    xFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
    ppRebase: *mut *mut ::core::ffi::c_void,
    pnRebase: *mut ::core::ffi::c_int,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    sessionChangesetApplyV23(
        db,
        nChangeset,
        pChangeset,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
        xFilter,
        xConflict,
        pCtx,
        ppRebase,
        pnRebase,
        flags,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_apply(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    nChangeset: ::core::ffi::c_int,
    pChangeset: *mut ::core::ffi::c_void,
    xFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    sessionChangesetApplyV23(
        db,
        nChangeset,
        pChangeset,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        xFilter,
        None,
        xConflict,
        pCtx,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        0 as ::core::ffi::c_int,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_apply_v3_strm(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pIn: *mut ::core::ffi::c_void,
    xFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
    ppRebase: *mut *mut ::core::ffi::c_void,
    pnRebase: *mut ::core::ffi::c_int,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    sessionChangesetApplyV23(
        db,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        xInput,
        pIn,
        None,
        xFilter,
        xConflict,
        pCtx,
        ppRebase,
        pnRebase,
        flags,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_apply_v2_strm(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pIn: *mut ::core::ffi::c_void,
    xFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
    ppRebase: *mut *mut ::core::ffi::c_void,
    pnRebase: *mut ::core::ffi::c_int,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    sessionChangesetApplyV23(
        db,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        xInput,
        pIn,
        xFilter,
        None,
        xConflict,
        pCtx,
        ppRebase,
        pnRebase,
        flags,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_apply_strm(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pIn: *mut ::core::ffi::c_void,
    xFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    xConflict: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut sqlite3_changeset_iter,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    sessionChangesetApplyV23(
        db,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        xInput,
        pIn,
        xFilter,
        None,
        xConflict,
        pCtx,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        0 as ::core::ffi::c_int,
    )
}

unsafe extern "C" fn sessionChangeMerge(
    pTab: *mut SessionTable,
    bRebase: ::core::ffi::c_int,
    bPatchset: ::core::ffi::c_int,
    pExist: *mut SessionChange,
    op2: ::core::ffi::c_int,
    bIndirect: ::core::ffi::c_int,
    aRec: *mut crate::src::ext::rtree::rtree::U8_0,
    nRec: ::core::ffi::c_int,
    ppNew: *mut *mut SessionChange,
) -> ::core::ffi::c_int {
    let mut pNew: *mut SessionChange = ::core::ptr::null_mut::<SessionChange>();
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if pExist.is_null() {
        pNew = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<SessionChange>() as usize).wrapping_add(nRec as usize)
                as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut SessionChange;
        if pNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        ::libc::memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SessionChange>() as crate::__stddef_size_t_h::SizeT,
        );
        (*pNew).op = op2 as crate::src::ext::rtree::rtree::U8_0;
        (*pNew).bIndirect = bIndirect as crate::src::ext::rtree::rtree::U8_0;
        (*pNew).aRecord = pNew.offset(1 as isize) as *mut SessionChange
            as *mut crate::src::ext::rtree::rtree::U8_0;
        if bIndirect == 0 as ::core::ffi::c_int || bRebase == 0 as ::core::ffi::c_int {
            (*pNew).nRecord = nRec;
            ::core::ptr::copy_nonoverlapping(
                aRec as *const u8,
                (*pNew).aRecord as *mut u8,
                nRec as usize,
            );
        } else {
            let mut i: ::core::ffi::c_int;
            let mut pIn: *mut crate::src::ext::rtree::rtree::U8_0 = aRec;
            let __pNew_ref = unsafe { &mut *pNew };
            let mut pOut: *mut crate::src::ext::rtree::rtree::U8_0 = __pNew_ref.aRecord;
            i = 0 as ::core::ffi::c_int;
            while i < (*pTab).nCol {
                let nIn: ::core::ffi::c_int = sessionSerialLen(pIn);
                if *pIn as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    let fresh27 = pOut;
                    pOut = pOut.offset(1);
                    *fresh27 = 0 as crate::src::ext::rtree::rtree::U8_0;
                } else if *(*pTab).abPK.offset(i as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let fresh28 = pOut;
                    pOut = pOut.offset(1);
                    *fresh28 = 0xff as crate::src::ext::rtree::rtree::U8_0;
                } else {
                    ::core::ptr::copy_nonoverlapping(
                        pIn as *const u8,
                        pOut as *mut u8,
                        nIn as usize,
                    );
                    pOut = pOut.offset(nIn as isize);
                }
                pIn = pIn.offset(nIn as isize);
                i += 1;
            }
            __pNew_ref.nRecord =
                pOut.offset_from(__pNew_ref.aRecord) as ::core::ffi::c_long as ::core::ffi::c_int;
        }
    } else if bRebase != 0 {
        if (*pExist).op as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_DELETE
            && (*pExist).bIndirect as ::core::ffi::c_int != 0
        {
            *ppNew = pExist;
        } else {
            let nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                ((nRec + (*pExist).nRecord) as usize)
                    .wrapping_add(::core::mem::size_of::<SessionChange>() as usize)
                    as crate::src::headers::sqlite3_h::Sqlite3Int64;
            pNew = crate::src::src::malloc::sqlite3_malloc64(
                nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ) as *mut SessionChange;
            if pNew.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                let mut i_0: ::core::ffi::c_int;
                let mut a1: *mut crate::src::ext::rtree::rtree::U8_0 = (*pExist).aRecord;
                let mut a2: *mut crate::src::ext::rtree::rtree::U8_0 = aRec;
                let mut pOut_0: *mut crate::src::ext::rtree::rtree::U8_0;
                ::libc::memset(
                    pNew as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nByte as crate::__stddef_size_t_h::SizeT,
                );
                let __pNew_ref = unsafe { &mut *pNew };
                __pNew_ref.bIndirect = (bIndirect != 0
                    || (*pExist).bIndirect as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int
                    as crate::src::ext::rtree::rtree::U8_0;
                __pNew_ref.op = op2 as crate::src::ext::rtree::rtree::U8_0;
                __pNew_ref.aRecord = pNew.offset(1 as isize) as *mut SessionChange
                    as *mut crate::src::ext::rtree::rtree::U8_0;
                pOut_0 = __pNew_ref.aRecord;
                i_0 = 0 as ::core::ffi::c_int;
                while i_0 < (*pTab).nCol {
                    let n1: ::core::ffi::c_int = sessionSerialLen(a1);
                    let n2: ::core::ffi::c_int = sessionSerialLen(a2);
                    if *a1 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
                        || *(*pTab).abPK.offset(i_0 as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && bIndirect != 0
                    {
                        let fresh29 = pOut_0;
                        pOut_0 = pOut_0.offset(1);
                        *fresh29 = 0xff as crate::src::ext::rtree::rtree::U8_0;
                    } else if *a2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        ::core::ptr::copy_nonoverlapping(
                            a1 as *const u8,
                            pOut_0 as *mut u8,
                            n1 as usize,
                        );
                        pOut_0 = pOut_0.offset(n1 as isize);
                    } else {
                        ::core::ptr::copy_nonoverlapping(
                            a2 as *const u8,
                            pOut_0 as *mut u8,
                            n2 as usize,
                        );
                        pOut_0 = pOut_0.offset(n2 as isize);
                    }
                    a1 = a1.offset(n1 as isize);
                    a2 = a2.offset(n2 as isize);
                    i_0 += 1;
                }
                __pNew_ref.nRecord = pOut_0.offset_from(__pNew_ref.aRecord) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            }
            crate::src::src::malloc::sqlite3_free(pExist as *mut ::core::ffi::c_void);
        }
    } else {
        let op1: ::core::ffi::c_int = (*pExist).op as ::core::ffi::c_int;
        if op1 == crate::src::headers::sqlite3_h::SQLITE_INSERT
            && op2 == crate::src::headers::sqlite3_h::SQLITE_INSERT
            || op1 == crate::src::headers::sqlite3_h::SQLITE_UPDATE
                && op2 == crate::src::headers::sqlite3_h::SQLITE_INSERT
            || op1 == crate::src::headers::sqlite3_h::SQLITE_DELETE
                && op2 == crate::src::headers::sqlite3_h::SQLITE_UPDATE
            || op1 == crate::src::headers::sqlite3_h::SQLITE_DELETE
                && op2 == crate::src::headers::sqlite3_h::SQLITE_DELETE
        {
            pNew = pExist;
        } else if op1 == crate::src::headers::sqlite3_h::SQLITE_INSERT
            && op2 == crate::src::headers::sqlite3_h::SQLITE_DELETE
        {
            crate::src::src::malloc::sqlite3_free(pExist as *mut ::core::ffi::c_void);
        } else {
            let __pExist_ref = unsafe { &mut *pExist };
            let aExist: *mut crate::src::ext::rtree::rtree::U8_0 = __pExist_ref.aRecord;
            let nByte_0: crate::src::headers::sqlite3_h::Sqlite3Int64;
            let mut aCsr: *mut crate::src::ext::rtree::rtree::U8_0;
            nByte_0 = (::core::mem::size_of::<SessionChange>() as usize)
                .wrapping_add(__pExist_ref.nRecord as usize)
                .wrapping_add(nRec as usize)
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
            pNew = crate::src::src::malloc::sqlite3_malloc64(
                nByte_0 as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ) as *mut SessionChange;
            if pNew.is_null() {
                crate::src::src::malloc::sqlite3_free(pExist as *mut ::core::ffi::c_void);
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            ::libc::memset(
                pNew as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<SessionChange>() as crate::__stddef_size_t_h::SizeT,
            );
            (*pNew).bIndirect =
                (bIndirect != 0 && __pExist_ref.bIndirect as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U8_0;
            (*pNew).aRecord = pNew.offset(1 as isize) as *mut SessionChange
                as *mut crate::src::ext::rtree::rtree::U8_0;
            aCsr = (*pNew).aRecord;
            if op1 == crate::src::headers::sqlite3_h::SQLITE_INSERT {
                let mut a1_0: *mut crate::src::ext::rtree::rtree::U8_0 = aRec;
                (*pNew).op = crate::src::headers::sqlite3_h::SQLITE_INSERT
                    as crate::src::ext::rtree::rtree::U8_0;
                if bPatchset == 0 as ::core::ffi::c_int {
                    sessionSkipRecord(&raw mut a1_0, (*pTab).nCol);
                }
                sessionMergeRecord(&raw mut aCsr, (*pTab).nCol, aExist, a1_0);
            } else if op1 == crate::src::headers::sqlite3_h::SQLITE_DELETE {
                (*pNew).op = crate::src::headers::sqlite3_h::SQLITE_UPDATE
                    as crate::src::ext::rtree::rtree::U8_0;
                if bPatchset != 0 {
                    ::core::ptr::copy_nonoverlapping(
                        aRec as *const u8,
                        aCsr as *mut u8,
                        nRec as usize,
                    );
                    aCsr = aCsr.offset(nRec as isize);
                } else if 0 as ::core::ffi::c_int
                    == sessionMergeUpdate(
                        &raw mut aCsr,
                        pTab,
                        bPatchset,
                        aExist,
                        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
                        aRec,
                        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
                    )
                {
                    crate::src::src::malloc::sqlite3_free(pNew as *mut ::core::ffi::c_void);
                    pNew = ::core::ptr::null_mut::<SessionChange>();
                }
            } else if op2 == crate::src::headers::sqlite3_h::SQLITE_UPDATE {
                let mut a1_1: *mut crate::src::ext::rtree::rtree::U8_0 = aExist;
                let mut a2_0: *mut crate::src::ext::rtree::rtree::U8_0 = aRec;
                if bPatchset == 0 as ::core::ffi::c_int {
                    sessionSkipRecord(&raw mut a1_1, (*pTab).nCol);
                    sessionSkipRecord(&raw mut a2_0, (*pTab).nCol);
                }
                (*pNew).op = crate::src::headers::sqlite3_h::SQLITE_UPDATE
                    as crate::src::ext::rtree::rtree::U8_0;
                if 0 as ::core::ffi::c_int
                    == sessionMergeUpdate(&raw mut aCsr, pTab, bPatchset, aRec, aExist, a1_1, a2_0)
                {
                    crate::src::src::malloc::sqlite3_free(pNew as *mut ::core::ffi::c_void);
                    pNew = ::core::ptr::null_mut::<SessionChange>();
                }
            } else {
                (*pNew).op = crate::src::headers::sqlite3_h::SQLITE_DELETE
                    as crate::src::ext::rtree::rtree::U8_0;
                if bPatchset != 0 {
                    ::core::ptr::copy_nonoverlapping(
                        aRec as *const u8,
                        aCsr as *mut u8,
                        nRec as usize,
                    );
                    aCsr = aCsr.offset(nRec as isize);
                } else {
                    sessionMergeRecord(&raw mut aCsr, (*pTab).nCol, aRec, aExist);
                }
            }
            if !pNew.is_null() {
                (*pNew).nRecord =
                    aCsr.offset_from((*pNew).aRecord) as ::core::ffi::c_long as ::core::ffi::c_int;
            }
            crate::src::src::malloc::sqlite3_free(pExist as *mut ::core::ffi::c_void);
        }
    }
    *ppNew = pNew;
    rc
}

unsafe extern "C" fn sessionChangesetCheckCompat(
    pTab: *mut SessionTable,
    nCol: ::core::ffi::c_int,
    abPK: *mut crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let __pTab_ref = unsafe { &mut *pTab };
    if !__pTab_ref.azCol.is_null() && nCol < __pTab_ref.nCol {
        let mut ii: ::core::ffi::c_int;
        ii = 0 as ::core::ffi::c_int;
        while ii < __pTab_ref.nCol {
            let bPK: crate::src::ext::rtree::rtree::U8_0 = (if ii < nCol {
                *abPK.offset(ii as isize) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })
                as crate::src::ext::rtree::rtree::U8_0;
            if *__pTab_ref.abPK.offset(ii as isize) as ::core::ffi::c_int
                != bPK as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            ii += 1;
        }
        return 1 as ::core::ffi::c_int;
    }
    (__pTab_ref.nCol == nCol
        && 0 as ::core::ffi::c_int
            == ::libc::memcmp(
                abPK as *const ::core::ffi::c_void,
                __pTab_ref.abPK as *const ::core::ffi::c_void,
                nCol as crate::__stddef_size_t_h::SizeT,
            )) as ::core::ffi::c_int
}

unsafe extern "C" fn sessionChangesetExtendRecord(
    pGrp: *mut sqlite3_changegroup,
    pTab: *mut SessionTable,
    nCol: ::core::ffi::c_int,
    op: ::core::ffi::c_int,
    aRec: *const crate::src::ext::rtree::rtree::U8_0,
    nRec: ::core::ffi::c_int,
    pOut: *mut SessionBuffer,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut ii: ::core::ffi::c_int;
    (*pOut).nBuf = 0 as ::core::ffi::c_int;
    if op == crate::src::headers::sqlite3_h::SQLITE_INSERT
        || op == crate::src::headers::sqlite3_h::SQLITE_DELETE
            && (*pGrp).bPatch == 0 as ::core::ffi::c_int
    {
        sessionAppendBlob(pOut, aRec, nRec, &raw mut rc);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && (*pTab).pDfltStmt.is_null() {
            rc = sessionPrepareDfltStmt((*pGrp).db, pTab, &raw mut (*pTab).pDfltStmt);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                && crate::src::headers::sqlite3_h::SQLITE_ROW
                    != crate::src::src::vdbeapi::sqlite3_step((*pTab).pDfltStmt)
            {
                rc = crate::src::src::main::sqlite3_errcode(
                    (*pGrp).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                );
            }
        }
        ii = nCol;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && ii < (*pTab).nCol {
            let eType: ::core::ffi::c_int =
                crate::src::src::vdbeapi::sqlite3_column_type((*pTab).pDfltStmt, ii);
            sessionAppendByte(
                pOut,
                eType as crate::src::ext::rtree::rtree::U8_0,
                &raw mut rc,
            );
            match eType {
                crate::src::headers::sqlite3_h::SQLITE_FLOAT_1
                | crate::src::headers::sqlite3_h::SQLITE_INTEGER => {
                    let mut iVal: crate::src::ext::rtree::rtree::I64_0 = 0;
                    if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
                        iVal = crate::src::src::vdbeapi::sqlite3_column_int64((*pTab).pDfltStmt, ii)
                            as crate::src::ext::rtree::rtree::I64_0;
                    } else {
                        let mut rVal: ::core::ffi::c_double =
                            crate::src::src::vdbeapi::sqlite3_column_int64((*pTab).pDfltStmt, ii)
                                as ::core::ffi::c_double;
                        ::core::ptr::copy_nonoverlapping(
                            &raw mut rVal as *const u8,
                            &raw mut iVal as *mut u8,
                            ::core::mem::size_of::<crate::src::ext::rtree::rtree::I64_0>() as usize,
                        );
                    }
                    if crate::src::headers::sqlite3_h::SQLITE_OK
                        == sessionBufferGrow(
                            pOut,
                            8 as crate::src::ext::rtree::rtree::I64_0,
                            &raw mut rc,
                        )
                    {
                        let __pOut_ref = unsafe { &mut *pOut };
                        sessionPutI64(
                            __pOut_ref.aBuf.offset(__pOut_ref.nBuf as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0,
                            iVal as crate::src::headers::sqlite3_h::Sqlite3Int64,
                        );
                        __pOut_ref.nBuf += 8 as ::core::ffi::c_int;
                    }
                }
                crate::src::headers::sqlite3_h::SQLITE_BLOB
                | crate::src::headers::sqlite3_h::SQLITE_TEXT => {
                    let n: ::core::ffi::c_int =
                        crate::src::src::vdbeapi::sqlite3_column_bytes((*pTab).pDfltStmt, ii);
                    sessionAppendVarint(pOut, n, &raw mut rc);
                    if eType == crate::src::headers::sqlite3_h::SQLITE_TEXT {
                        let z: *const crate::src::ext::rtree::rtree::U8_0 =
                            crate::src::src::vdbeapi::sqlite3_column_text((*pTab).pDfltStmt, ii)
                                as *const crate::src::ext::rtree::rtree::U8_0;
                        sessionAppendBlob(pOut, z, n, &raw mut rc);
                    } else {
                        let z_0: *const crate::src::ext::rtree::rtree::U8_0 =
                            crate::src::src::vdbeapi::sqlite3_column_blob((*pTab).pDfltStmt, ii)
                                as *const crate::src::ext::rtree::rtree::U8_0;
                        sessionAppendBlob(pOut, z_0, n, &raw mut rc);
                    }
                }
                _ => {}
            }
            ii += 1;
        }
    } else if op == crate::src::headers::sqlite3_h::SQLITE_UPDATE {
        let mut iOff: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*pGrp).bPatch == 0 as ::core::ffi::c_int {
            ii = 0 as ::core::ffi::c_int;
            while ii < nCol {
                iOff += sessionSerialLen(
                    aRec.offset(iOff as isize) as *const crate::src::ext::rtree::rtree::U8_0
                );
                ii += 1;
            }
            sessionAppendBlob(pOut, aRec, iOff, &raw mut rc);
            ii = 0 as ::core::ffi::c_int;
            while ii < (*pTab).nCol - nCol {
                sessionAppendByte(pOut, 0 as crate::src::ext::rtree::rtree::U8_0, &raw mut rc);
                ii += 1;
            }
        }
        sessionAppendBlob(
            pOut,
            aRec.offset(iOff as isize) as *const crate::src::ext::rtree::rtree::U8_0,
            nRec - iOff,
            &raw mut rc,
        );
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pTab).nCol - nCol {
            sessionAppendByte(pOut, 0 as crate::src::ext::rtree::rtree::U8_0, &raw mut rc);
            ii += 1;
        }
    } else {
        sessionAppendBlob(pOut, aRec, nRec, &raw mut rc);
    }
    rc
}

unsafe extern "C" fn sessionChangesetFindTable(
    pGrp: *mut sqlite3_changegroup,
    zTab: *const ::core::ffi::c_char,
    pIter: *mut sqlite3_changeset_iter,
    ppTab: *mut *mut SessionTable,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pTab: *mut SessionTable;
    let nTab: ::core::ffi::c_int = ::libc::strlen(zTab) as ::core::ffi::c_int;
    let mut abPK: *mut crate::src::ext::rtree::rtree::U8_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *ppTab = ::core::ptr::null_mut::<SessionTable>();
    sqlite3changeset_pk(pIter, &raw mut abPK, &raw mut nCol);
    pTab = (*pGrp).pList;
    while !pTab.is_null() {
        if 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3_strnicmp(
                (*pTab).zName,
                zTab,
                nTab + 1 as ::core::ffi::c_int,
            )
        {
            break;
        }
        pTab = (*pTab).pNext;
    }
    if pTab.is_null() {
        let mut ppNew: *mut *mut SessionTable;
        pTab = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<SessionTable>() as usize)
                .wrapping_add(nCol as usize)
                .wrapping_add(nTab as usize)
                .wrapping_add(1 as usize)
                as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut SessionTable;
        if pTab.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        ::libc::memset(
            pTab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SessionTable>() as crate::__stddef_size_t_h::SizeT,
        );
        (*pTab).nCol = nCol;
        (*pTab).abPK = pTab.offset(1 as isize) as *mut SessionTable
            as *mut crate::src::ext::rtree::rtree::U8_0;
        ::core::ptr::copy_nonoverlapping(abPK as *const u8, (*pTab).abPK as *mut u8, nCol as usize);
        (*pTab).zName = (*pTab).abPK.offset(nCol as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0
            as *mut ::core::ffi::c_char;
        ::core::ptr::copy_nonoverlapping(
            zTab as *const u8,
            (*pTab).zName as *mut u8,
            (nTab + 1 as ::core::ffi::c_int) as usize,
        );
        if !(*pGrp).db.is_null() {
            (*pTab).nCol = 0 as ::core::ffi::c_int;
            rc = sessionInitTable(
                ::core::ptr::null_mut::<sqlite3_session>(),
                pTab,
                (*pGrp).db,
                (*pGrp).zDb,
            );
            if rc != 0 {
                crate::src::src::malloc::sqlite3_free(pTab as *mut ::core::ffi::c_void);
                return rc;
            }
        }
        ppNew = &raw mut (*pGrp).pList;
        while !(*ppNew).is_null() {
            ppNew = &raw mut (**ppNew).pNext;
        }
        *ppNew = pTab;
    }
    if sessionChangesetCheckCompat(pTab, nCol, abPK) == 0 {
        rc = crate::src::headers::sqlite3_h::SQLITE_SCHEMA;
    }
    *ppTab = pTab;
    rc
}

unsafe extern "C" fn sessionOneChangeToHash(
    pGrp: *mut sqlite3_changegroup,
    pIter: *mut sqlite3_changeset_iter,
    bRebase: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut op: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iHash: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bIndirect: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pChange: *mut SessionChange = ::core::ptr::null_mut::<SessionChange>();
    let mut pExist: *mut SessionChange = ::core::ptr::null_mut::<SessionChange>();
    let mut pp: *mut *mut SessionChange;
    let mut pTab: *mut SessionTable = ::core::ptr::null_mut::<SessionTable>();
    let __pIter_ref = unsafe { &mut *pIter };
    let mut aRec: *mut crate::src::ext::rtree::rtree::U8_0 = (*pIter)
        .in_0
        .aData
        .offset((__pIter_ref.in_0.iCurrent + 2 as ::core::ffi::c_int) as isize)
        as *mut crate::src::ext::rtree::rtree::U8_0;
    let mut nRec: ::core::ffi::c_int =
        __pIter_ref.in_0.iNext - __pIter_ref.in_0.iCurrent - 2 as ::core::ffi::c_int;
    if (*pGrp).pList.is_null() {
        (*pGrp).bPatch = __pIter_ref.bPatchset;
    } else if __pIter_ref.bPatchset != (*pGrp).bPatch {
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut zTab: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        sqlite3changeset_op(
            pIter,
            &raw mut zTab,
            &raw mut nCol,
            &raw mut op,
            &raw mut bIndirect,
        );
        rc = sessionChangesetFindTable(pGrp, zTab, pIter, &raw mut pTab);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && nCol < (*pTab).nCol {
        let pBuf: *mut SessionBuffer = &raw mut (*pGrp).rec;
        rc = sessionChangesetExtendRecord(pGrp, pTab, nCol, op, aRec, nRec, pBuf);
        aRec = (*pBuf).aBuf;
        nRec = (*pBuf).nBuf;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && sessionGrowHash(
            ::core::ptr::null_mut::<sqlite3_session>(),
            __pIter_ref.bPatchset,
            pTab,
        ) != 0
    {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        iHash = sessionChangeHash(
            pTab,
            (__pIter_ref.bPatchset != 0 && op == crate::src::headers::sqlite3_h::SQLITE_DELETE)
                as ::core::ffi::c_int,
            aRec,
            (*pTab).nChange,
        ) as ::core::ffi::c_int;
        pp = (*pTab).apChange.offset(iHash as isize) as *mut *mut SessionChange;
        while !(*pp).is_null() {
            let mut bPkOnly1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut bPkOnly2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if __pIter_ref.bPatchset != 0 {
                bPkOnly1 = ((**pp).op as ::core::ffi::c_int
                    == crate::src::headers::sqlite3_h::SQLITE_DELETE)
                    as ::core::ffi::c_int;
                bPkOnly2 =
                    (op == crate::src::headers::sqlite3_h::SQLITE_DELETE) as ::core::ffi::c_int;
            }
            if sessionChangeEqual(pTab, bPkOnly1, (**pp).aRecord, bPkOnly2, aRec) != 0 {
                pExist = *pp;
                *pp = (**pp).pNext;
                (*pTab).nEntry -= 1;
                break;
            } else {
                pp = &raw mut (**pp).pNext;
            }
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionChangeMerge(
            pTab,
            bRebase,
            __pIter_ref.bPatchset,
            pExist,
            op,
            bIndirect,
            aRec,
            nRec,
            &raw mut pChange,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pChange.is_null() {
        let __pTab_ref = unsafe { &mut *pTab };
        (*pChange).pNext = *__pTab_ref.apChange.offset(iHash as isize);
        let ref mut fresh26 = *__pTab_ref.apChange.offset(iHash as isize);
        *fresh26 = pChange;
        __pTab_ref.nEntry += 1;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = __pIter_ref.rc;
    }
    rc
}

unsafe extern "C" fn sessionChangesetToHash(
    pIter: *mut sqlite3_changeset_iter,
    pGrp: *mut sqlite3_changegroup,
    bRebase: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut aRec: *mut crate::src::ext::rtree::rtree::U8_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    let mut nRec: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    (*pIter).in_0.bNoDiscard = 1 as ::core::ffi::c_int;
    while crate::src::headers::sqlite3_h::SQLITE_ROW
        == sessionChangesetNext(
            pIter,
            &raw mut aRec,
            &raw mut nRec,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        )
    {
        rc = sessionOneChangeToHash(pGrp, pIter, bRebase);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            break;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = (*pIter).rc;
    }
    rc
}

unsafe extern "C" fn sessionChangegroupOutput(
    pGrp: *mut sqlite3_changegroup,
    xOutput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pOut: *mut ::core::ffi::c_void,
    pnOut: *mut ::core::ffi::c_int,
    ppOut: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut buf: SessionBuffer = unsafe { ::core::mem::zeroed() };
    let mut pTab: *mut SessionTable;
    pTab = (*pGrp).pList;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pTab.is_null() {
        let mut i: ::core::ffi::c_int;
        if !((*pTab).nEntry == 0 as ::core::ffi::c_int) {
            sessionAppendTableHdr(&raw mut buf, (*pGrp).bPatch, pTab, &raw mut rc);
            i = 0 as ::core::ffi::c_int;
            while i < (*pTab).nChange {
                let mut p: *mut SessionChange;
                p = *(*pTab).apChange.offset(i as isize);
                while !p.is_null() {
                    sessionAppendByte(&raw mut buf, (*p).op, &raw mut rc);
                    sessionAppendByte(&raw mut buf, (*p).bIndirect, &raw mut rc);
                    sessionAppendBlob(&raw mut buf, (*p).aRecord, (*p).nRecord, &raw mut rc);
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                        && xOutput.is_some()
                        && buf.nBuf >= sessions_strm_chunk_size
                    {
                        rc = xOutput.expect("non-null function pointer")(
                            pOut,
                            buf.aBuf as *const ::core::ffi::c_void,
                            buf.nBuf,
                        );
                        buf.nBuf = 0 as ::core::ffi::c_int;
                    }
                    p = (*p).pNext;
                }
                i += 1;
            }
        }
        pTab = (*pTab).pNext;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if xOutput.is_some() {
            if buf.nBuf > 0 as ::core::ffi::c_int {
                rc = xOutput.expect("non-null function pointer")(
                    pOut,
                    buf.aBuf as *const ::core::ffi::c_void,
                    buf.nBuf,
                );
            }
        } else if !ppOut.is_null() {
            *ppOut = buf.aBuf as *mut ::core::ffi::c_void;
            if !pnOut.is_null() {
                *pnOut = buf.nBuf;
            }
            buf.aBuf = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
        }
    }
    crate::src::src::malloc::sqlite3_free(buf.aBuf as *mut ::core::ffi::c_void);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changegroup_new(
    pp: *mut *mut sqlite3_changegroup,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let p: *mut sqlite3_changegroup;
    p = crate::src::src::malloc::sqlite3_malloc(
        ::core::mem::size_of::<sqlite3_changegroup>() as ::core::ffi::c_int
    ) as *mut sqlite3_changegroup;
    if p.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        ::libc::memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sqlite3_changegroup>() as crate::__stddef_size_t_h::SizeT,
        );
    }
    *pp = p;
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changegroup_schema(
    pGrp: *mut sqlite3_changegroup,
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zDb: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !(*pGrp).pList.is_null() || !(*pGrp).db.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_MISUSE;
    } else {
        (*pGrp).zDb = sqlite_printf!("%s", zDb);
        if (*pGrp).zDb.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            (*pGrp).db = db;
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changegroup_add(
    pGrp: *mut sqlite3_changegroup,
    nData: ::core::ffi::c_int,
    pData: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pIter: *mut sqlite3_changeset_iter = ::core::ptr::null_mut::<sqlite3_changeset_iter>();
    let mut rc: ::core::ffi::c_int;
    rc = sqlite3changeset_start(&raw mut pIter, nData, pData);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionChangesetToHash(pIter, pGrp, 0 as ::core::ffi::c_int);
    }
    sqlite3changeset_finalize(pIter);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changegroup_add_change(
    pGrp: *mut sqlite3_changegroup,
    pIter: *mut sqlite3_changeset_iter,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    let __pIter_ref = unsafe { &mut *pIter };
    if __pIter_ref.in_0.iCurrent == __pIter_ref.in_0.iNext
        || __pIter_ref.rc != crate::src::headers::sqlite3_h::SQLITE_OK
        || __pIter_ref.bInvert != 0
    {
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    } else {
        __pIter_ref.in_0.bNoDiscard = 1 as ::core::ffi::c_int;
        rc = sessionOneChangeToHash(pGrp, pIter, 0 as ::core::ffi::c_int);
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changegroup_output(
    pGrp: *mut sqlite3_changegroup,
    pnData: *mut ::core::ffi::c_int,
    ppData: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    sessionChangegroupOutput(
        pGrp,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pnData,
        ppData,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changegroup_add_strm(
    pGrp: *mut sqlite3_changegroup,
    xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pIn: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pIter: *mut sqlite3_changeset_iter = ::core::ptr::null_mut::<sqlite3_changeset_iter>();
    let mut rc: ::core::ffi::c_int;
    rc = sqlite3changeset_start_strm(&raw mut pIter, xInput, pIn);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionChangesetToHash(pIter, pGrp, 0 as ::core::ffi::c_int);
    }
    sqlite3changeset_finalize(pIter);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changegroup_output_strm(
    pGrp: *mut sqlite3_changegroup,
    xOutput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pOut: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    sessionChangegroupOutput(
        pGrp,
        xOutput,
        pOut,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changegroup_delete(pGrp: *mut sqlite3_changegroup) {
    if !pGrp.is_null() {
        let __pGrp_ref = unsafe { &*pGrp };
        crate::src::src::malloc::sqlite3_free(__pGrp_ref.zDb as *mut ::core::ffi::c_void);
        sessionDeleteTable(::core::ptr::null_mut::<sqlite3_session>(), __pGrp_ref.pList);
        crate::src::src::malloc::sqlite3_free(__pGrp_ref.rec.aBuf as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(pGrp as *mut ::core::ffi::c_void);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_concat(
    nLeft: ::core::ffi::c_int,
    pLeft: *mut ::core::ffi::c_void,
    nRight: ::core::ffi::c_int,
    pRight: *mut ::core::ffi::c_void,
    pnOut: *mut ::core::ffi::c_int,
    ppOut: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pGrp: *mut sqlite3_changegroup = ::core::ptr::null_mut::<sqlite3_changegroup>();
    let mut rc: ::core::ffi::c_int;
    rc = sqlite3changegroup_new(&raw mut pGrp);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3changegroup_add(pGrp, nLeft, pLeft);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3changegroup_add(pGrp, nRight, pRight);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3changegroup_output(pGrp, pnOut, ppOut);
    }
    sqlite3changegroup_delete(pGrp);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3changeset_concat_strm(
    xInputA: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pInA: *mut ::core::ffi::c_void,
    xInputB: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pInB: *mut ::core::ffi::c_void,
    xOutput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pOut: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pGrp: *mut sqlite3_changegroup = ::core::ptr::null_mut::<sqlite3_changegroup>();
    let mut rc: ::core::ffi::c_int;
    rc = sqlite3changegroup_new(&raw mut pGrp);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3changegroup_add_strm(pGrp, xInputA, pInA);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3changegroup_add_strm(pGrp, xInputB, pInB);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3changegroup_output_strm(pGrp, xOutput, pOut);
    }
    sqlite3changegroup_delete(pGrp);
    rc
}

unsafe extern "C" fn sessionAppendRecordMerge(
    pBuf: *mut SessionBuffer,
    nCol: ::core::ffi::c_int,
    mut a1: *mut crate::src::ext::rtree::rtree::U8_0,
    n1: ::core::ffi::c_int,
    mut a2: *mut crate::src::ext::rtree::rtree::U8_0,
    n2: ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) {
    sessionBufferGrow(pBuf, (n1 + n2) as crate::src::ext::rtree::rtree::I64_0, pRc);
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut i: ::core::ffi::c_int;
        let __pBuf_ref = unsafe { &mut *pBuf };
        let mut pOut: *mut crate::src::ext::rtree::rtree::U8_0 =
            __pBuf_ref.aBuf.offset(__pBuf_ref.nBuf as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0;
        i = 0 as ::core::ffi::c_int;
        while i < nCol {
            let nn1: ::core::ffi::c_int = sessionSerialLen(a1);
            let nn2: ::core::ffi::c_int = sessionSerialLen(a2);
            if *a1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || *a1 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
            {
                ::core::ptr::copy_nonoverlapping(a2 as *const u8, pOut as *mut u8, nn2 as usize);
                pOut = pOut.offset(nn2 as isize);
            } else {
                ::core::ptr::copy_nonoverlapping(a1 as *const u8, pOut as *mut u8, nn1 as usize);
                pOut = pOut.offset(nn1 as isize);
            }
            a1 = a1.offset(nn1 as isize);
            a2 = a2.offset(nn2 as isize);
            i += 1;
        }
        __pBuf_ref.nBuf =
            pOut.offset_from(__pBuf_ref.aBuf) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn sessionAppendPartialUpdate(
    pBuf: *mut SessionBuffer,
    pIter: *mut sqlite3_changeset_iter,
    aRec: *mut crate::src::ext::rtree::rtree::U8_0,
    nRec: ::core::ffi::c_int,
    aChange: *mut crate::src::ext::rtree::rtree::U8_0,
    nChange: ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) {
    sessionBufferGrow(
        pBuf,
        (2 as ::core::ffi::c_int + nRec + nChange) as crate::src::ext::rtree::rtree::I64_0,
        pRc,
    );
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut bData: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pOut: *mut crate::src::ext::rtree::rtree::U8_0 =
            (*pBuf).aBuf.offset((*pBuf).nBuf as isize) as *mut crate::src::ext::rtree::rtree::U8_0;
        let mut i: ::core::ffi::c_int;
        let mut a1: *mut crate::src::ext::rtree::rtree::U8_0 = aRec;
        let mut a2: *mut crate::src::ext::rtree::rtree::U8_0 = aChange;
        let fresh33 = pOut;
        pOut = pOut.offset(1);
        *fresh33 =
            crate::src::headers::sqlite3_h::SQLITE_UPDATE as crate::src::ext::rtree::rtree::U8_0;
        let fresh34 = pOut;
        pOut = pOut.offset(1);
        *fresh34 = (*pIter).bIndirect as crate::src::ext::rtree::rtree::U8_0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIter).nCol {
            let n1: ::core::ffi::c_int = sessionSerialLen(a1);
            let n2: ::core::ffi::c_int = sessionSerialLen(a2);
            if *(*pIter).abPK.offset(i as isize) as ::core::ffi::c_int != 0
                || *a2.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                if *(*pIter).abPK.offset(i as isize) == 0
                    && *a1.offset(0 as isize) as ::core::ffi::c_int != 0
                {
                    bData = 1 as ::core::ffi::c_int;
                }
                ::core::ptr::copy_nonoverlapping(a1 as *const u8, pOut as *mut u8, n1 as usize);
                pOut = pOut.offset(n1 as isize);
            } else if *a2.offset(0 as isize) as ::core::ffi::c_int != 0xff as ::core::ffi::c_int
                && *a1.offset(0 as isize) as ::core::ffi::c_int != 0
            {
                bData = 1 as ::core::ffi::c_int;
                ::core::ptr::copy_nonoverlapping(a2 as *const u8, pOut as *mut u8, n2 as usize);
                pOut = pOut.offset(n2 as isize);
            } else {
                let fresh35 = pOut;
                pOut = pOut.offset(1);
                *fresh35 = '\0' as i32 as crate::src::ext::rtree::rtree::U8_0;
            }
            a1 = a1.offset(n1 as isize);
            a2 = a2.offset(n2 as isize);
            i += 1;
        }
        if bData != 0 {
            a2 = aChange;
            i = 0 as ::core::ffi::c_int;
            while i < (*pIter).nCol {
                let n1_0: ::core::ffi::c_int = sessionSerialLen(a1);
                let n2_0: ::core::ffi::c_int = sessionSerialLen(a2);
                if *(*pIter).abPK.offset(i as isize) as ::core::ffi::c_int != 0
                    || *a2.offset(0 as isize) as ::core::ffi::c_int != 0xff as ::core::ffi::c_int
                {
                    ::core::ptr::copy_nonoverlapping(
                        a1 as *const u8,
                        pOut as *mut u8,
                        n1_0 as usize,
                    );
                    pOut = pOut.offset(n1_0 as isize);
                } else {
                    let fresh36 = pOut;
                    pOut = pOut.offset(1);
                    *fresh36 = '\0' as i32 as crate::src::ext::rtree::rtree::U8_0;
                }
                a1 = a1.offset(n1_0 as isize);
                a2 = a2.offset(n2_0 as isize);
                i += 1;
            }
            (*pBuf).nBuf =
                pOut.offset_from((*pBuf).aBuf) as ::core::ffi::c_long as ::core::ffi::c_int;
        }
    }
}

unsafe extern "C" fn sessionRebase(
    p: *mut sqlite3_rebaser,
    pIter: *mut sqlite3_changeset_iter,
    xOutput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pOut: *mut ::core::ffi::c_void,
    pnOut: *mut ::core::ffi::c_int,
    ppOut: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut aRec: *mut crate::src::ext::rtree::rtree::U8_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
    let mut nRec: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bNew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pTab: *mut SessionTable = ::core::ptr::null_mut::<SessionTable>();
    let mut sOut: SessionBuffer = unsafe { ::core::mem::zeroed() };
    while crate::src::headers::sqlite3_h::SQLITE_ROW
        == sessionChangesetNext(pIter, &raw mut aRec, &raw mut nRec, &raw mut bNew)
    {
        let mut pChange: *mut SessionChange = ::core::ptr::null_mut::<SessionChange>();
        let mut bDone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if bNew != 0 {
            let __pIter_ref = unsafe { &*pIter };
            let zTab: *const ::core::ffi::c_char = __pIter_ref.zTab;
            pTab = (*p).grp.pList;
            while !pTab.is_null() {
                if 0 as ::core::ffi::c_int
                    == crate::src::src::util::sqlite3_stricmp((*pTab).zName, zTab)
                {
                    break;
                }
                pTab = (*pTab).pNext;
            }
            bNew = 0 as ::core::ffi::c_int;
            if __pIter_ref.bPatchset != 0 {
                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
            sessionAppendByte(
                &raw mut sOut,
                (if __pIter_ref.bPatchset != 0 {
                    'P' as i32
                } else {
                    'T' as i32
                }) as crate::src::ext::rtree::rtree::U8_0,
                &raw mut rc,
            );
            sessionAppendVarint(&raw mut sOut, __pIter_ref.nCol, &raw mut rc);
            sessionAppendBlob(
                &raw mut sOut,
                __pIter_ref.abPK,
                __pIter_ref.nCol,
                &raw mut rc,
            );
            sessionAppendBlob(
                &raw mut sOut,
                __pIter_ref.zTab as *mut crate::src::ext::rtree::rtree::U8_0,
                ::libc::strlen(__pIter_ref.zTab) as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                &raw mut rc,
            );
        }
        if !pTab.is_null() && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let iHash: ::core::ffi::c_int =
                sessionChangeHash(pTab, 0 as ::core::ffi::c_int, aRec, (*pTab).nChange)
                    as ::core::ffi::c_int;
            pChange = *(*pTab).apChange.offset(iHash as isize);
            while !pChange.is_null() {
                if sessionChangeEqual(
                    pTab,
                    0 as ::core::ffi::c_int,
                    aRec,
                    0 as ::core::ffi::c_int,
                    (*pChange).aRecord,
                ) != 0
                {
                    break;
                }
                pChange = (*pChange).pNext;
            }
        }
        if !pChange.is_null() {
            match (*pIter).op {
                crate::src::headers::sqlite3_h::SQLITE_INSERT => {
                    if (*pChange).op as ::core::ffi::c_int
                        == crate::src::headers::sqlite3_h::SQLITE_INSERT
                    {
                        bDone = 1 as ::core::ffi::c_int;
                        if (*pChange).bIndirect as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            sessionAppendByte(
                                &raw mut sOut,
                                crate::src::headers::sqlite3_h::SQLITE_UPDATE
                                    as crate::src::ext::rtree::rtree::U8_0,
                                &raw mut rc,
                            );
                            sessionAppendByte(
                                &raw mut sOut,
                                (*pIter).bIndirect as crate::src::ext::rtree::rtree::U8_0,
                                &raw mut rc,
                            );
                            sessionAppendBlob(
                                &raw mut sOut,
                                (*pChange).aRecord,
                                (*pChange).nRecord,
                                &raw mut rc,
                            );
                            sessionAppendBlob(&raw mut sOut, aRec, nRec, &raw mut rc);
                        }
                    }
                }
                crate::src::headers::sqlite3_h::SQLITE_UPDATE => {
                    bDone = 1 as ::core::ffi::c_int;
                    if (*pChange).op as ::core::ffi::c_int
                        == crate::src::headers::sqlite3_h::SQLITE_DELETE
                    {
                        if (*pChange).bIndirect as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            let mut pCsr: *mut crate::src::ext::rtree::rtree::U8_0 = aRec;
                            let __pIter_ref = unsafe { &*pIter };
                            sessionSkipRecord(&raw mut pCsr, __pIter_ref.nCol);
                            sessionAppendByte(
                                &raw mut sOut,
                                crate::src::headers::sqlite3_h::SQLITE_INSERT
                                    as crate::src::ext::rtree::rtree::U8_0,
                                &raw mut rc,
                            );
                            sessionAppendByte(
                                &raw mut sOut,
                                __pIter_ref.bIndirect as crate::src::ext::rtree::rtree::U8_0,
                                &raw mut rc,
                            );
                            sessionAppendRecordMerge(
                                &raw mut sOut,
                                __pIter_ref.nCol,
                                pCsr,
                                (nRec as ::core::ffi::c_long
                                    - pCsr.offset_from(aRec) as ::core::ffi::c_long)
                                    as ::core::ffi::c_int,
                                (*pChange).aRecord,
                                (*pChange).nRecord,
                                &raw mut rc,
                            );
                        }
                    } else {
                        sessionAppendPartialUpdate(
                            &raw mut sOut,
                            pIter,
                            aRec,
                            nRec,
                            (*pChange).aRecord,
                            (*pChange).nRecord,
                            &raw mut rc,
                        );
                    }
                }
                _ => {
                    bDone = 1 as ::core::ffi::c_int;
                    if (*pChange).op as ::core::ffi::c_int
                        == crate::src::headers::sqlite3_h::SQLITE_INSERT
                    {
                        sessionAppendByte(
                            &raw mut sOut,
                            crate::src::headers::sqlite3_h::SQLITE_DELETE
                                as crate::src::ext::rtree::rtree::U8_0,
                            &raw mut rc,
                        );
                        sessionAppendByte(
                            &raw mut sOut,
                            (*pIter).bIndirect as crate::src::ext::rtree::rtree::U8_0,
                            &raw mut rc,
                        );
                        sessionAppendRecordMerge(
                            &raw mut sOut,
                            (*pIter).nCol,
                            (*pChange).aRecord,
                            (*pChange).nRecord,
                            aRec,
                            nRec,
                            &raw mut rc,
                        );
                    }
                }
            }
        }
        if bDone == 0 as ::core::ffi::c_int {
            sessionAppendByte(
                &raw mut sOut,
                (*pIter).op as crate::src::ext::rtree::rtree::U8_0,
                &raw mut rc,
            );
            sessionAppendByte(
                &raw mut sOut,
                (*pIter).bIndirect as crate::src::ext::rtree::rtree::U8_0,
                &raw mut rc,
            );
            sessionAppendBlob(&raw mut sOut, aRec, nRec, &raw mut rc);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && xOutput.is_some()
            && sOut.nBuf > sessions_strm_chunk_size
        {
            rc = xOutput.expect("non-null function pointer")(
                pOut,
                sOut.aBuf as *const ::core::ffi::c_void,
                sOut.nBuf,
            );
            sOut.nBuf = 0 as ::core::ffi::c_int;
        }
        if rc != 0 {
            break;
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::malloc::sqlite3_free(sOut.aBuf as *mut ::core::ffi::c_void);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if xOutput.is_some() {
            if sOut.nBuf > 0 as ::core::ffi::c_int {
                rc = xOutput.expect("non-null function pointer")(
                    pOut,
                    sOut.aBuf as *const ::core::ffi::c_void,
                    sOut.nBuf,
                );
            }
        } else if !ppOut.is_null() {
            *ppOut = sOut.aBuf as *mut ::core::ffi::c_void;
            *pnOut = sOut.nBuf;
            sOut.aBuf = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>();
        }
    }
    crate::src::src::malloc::sqlite3_free(sOut.aBuf as *mut ::core::ffi::c_void);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3rebaser_create(
    ppNew: *mut *mut sqlite3_rebaser,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let pNew: *mut sqlite3_rebaser;
    pNew = crate::src::src::malloc::sqlite3_malloc(
        ::core::mem::size_of::<sqlite3_rebaser>() as ::core::ffi::c_int
    ) as *mut sqlite3_rebaser;
    if pNew.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        ::libc::memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sqlite3_rebaser>() as crate::__stddef_size_t_h::SizeT,
        );
    }
    *ppNew = pNew;
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3rebaser_configure(
    p: *mut sqlite3_rebaser,
    nRebase: ::core::ffi::c_int,
    pRebase: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pIter: *mut sqlite3_changeset_iter = ::core::ptr::null_mut::<sqlite3_changeset_iter>();
    let mut rc: ::core::ffi::c_int;
    rc = sqlite3changeset_start(&raw mut pIter, nRebase, pRebase as *mut ::core::ffi::c_void);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionChangesetToHash(pIter, &raw mut (*p).grp, 1 as ::core::ffi::c_int);
    }
    sqlite3changeset_finalize(pIter);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3rebaser_rebase(
    p: *mut sqlite3_rebaser,
    nIn: ::core::ffi::c_int,
    pIn: *const ::core::ffi::c_void,
    pnOut: *mut ::core::ffi::c_int,
    ppOut: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pIter: *mut sqlite3_changeset_iter = ::core::ptr::null_mut::<sqlite3_changeset_iter>();
    let mut rc: ::core::ffi::c_int =
        sqlite3changeset_start(&raw mut pIter, nIn, pIn as *mut ::core::ffi::c_void);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionRebase(
            p,
            pIter,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pnOut,
            ppOut,
        );
        sqlite3changeset_finalize(pIter);
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3rebaser_rebase_strm(
    p: *mut sqlite3_rebaser,
    xInput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pIn: *mut ::core::ffi::c_void,
    xOutput: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pOut: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pIter: *mut sqlite3_changeset_iter = ::core::ptr::null_mut::<sqlite3_changeset_iter>();
    let mut rc: ::core::ffi::c_int = sqlite3changeset_start_strm(&raw mut pIter, xInput, pIn);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sessionRebase(
            p,
            pIter,
            xOutput,
            pOut,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        sqlite3changeset_finalize(pIter);
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3rebaser_delete(p: *mut sqlite3_rebaser) {
    if !p.is_null() {
        sessionDeleteTable(::core::ptr::null_mut::<sqlite3_session>(), (*p).grp.pList);
        crate::src::src::malloc::sqlite3_free((*p).grp.rec.aBuf as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3session_config(
    op: ::core::ffi::c_int,
    pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    match op {
        crate::src::ext::session::sqlite3session::SQLITE_SESSION_CONFIG_STRMSIZE => {
            let pInt: *mut ::core::ffi::c_int = pArg as *mut ::core::ffi::c_int;
            if *pInt > 0 as ::core::ffi::c_int {
                sessions_strm_chunk_size = *pInt;
            }
            *pInt = sessions_strm_chunk_size;
        }
        _ => {
            rc = crate::src::headers::sqlite3_h::SQLITE_MISUSE;
        }
    }
    rc
}
