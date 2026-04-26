pub mod stdlib_h {

    #[inline]
    pub unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0;
        let mut ptr = __nptr;
        let mut negative = false;
        while *ptr == b' ' as ::core::ffi::c_char || *ptr == b'\t' as ::core::ffi::c_char {
            ptr = ptr.offset(1);
        }
        if *ptr == b'-' as ::core::ffi::c_char {
            negative = true;
            ptr = ptr.offset(1);
        } else if *ptr == b'+' as ::core::ffi::c_char {
            ptr = ptr.offset(1);
        }
        while *ptr >= b'0' as ::core::ffi::c_char && *ptr <= b'9' as ::core::ffi::c_char {
            result = result * 10 + (*ptr as ::core::ffi::c_int - b'0' as ::core::ffi::c_int);
            ptr = ptr.offset(1);
        }
        if negative { -result } else { result }
    }
}

// Helper function for memcmp operations
#[inline]
unsafe fn fts3_memcmp_safe(
    ptr1: *const ::core::ffi::c_void,
    ptr2: *const ::core::ffi::c_void,
    len: usize,
) -> ::core::ffi::c_int {
    if len == 0 {
        return 0;
    }
    let a = std::slice::from_raw_parts(ptr1 as *const u8, len);
    let b = std::slice::from_raw_parts(ptr2 as *const u8, len);
    match a.cmp(b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Equal => 0,
    }
}
pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_null_h::NULL_0;
pub use crate::__stddef_size_t_h::SizeT;

pub use crate::fts3Int_h::FTS_CORRUPT_VTAB;
pub use crate::fts3Int_h::FTS3_MAX_PENDING_DATA;
pub use crate::fts3Int_h::FTS3_MERGE_COUNT;
pub use crate::fts3Int_h::FTS3_SEGCURSOR_ALL;
pub use crate::fts3Int_h::FTS3_SEGCURSOR_PENDING;
pub use crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL;
pub use crate::fts3Int_h::FTS3_SEGMENT_COLUMN_FILTER;
pub use crate::fts3Int_h::FTS3_SEGMENT_FIRST;
pub use crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY;
pub use crate::fts3Int_h::FTS3_SEGMENT_PREFIX;
pub use crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS;
pub use crate::fts3Int_h::FTS3_SEGMENT_SCAN;
pub use crate::fts3Int_h::FTS3_VARINT_MAX;
pub use crate::fts3Int_h::Fts3Cursor;
pub use crate::fts3Int_h::Fts3Doclist;
pub use crate::fts3Int_h::Fts3Expr;
pub use crate::fts3Int_h::Fts3Index;
pub use crate::fts3Int_h::Fts3MultiSegReader;
pub use crate::fts3Int_h::Fts3Phrase;
pub use crate::fts3Int_h::Fts3PhraseToken;
pub use crate::fts3Int_h::Fts3SegFilter;
pub use crate::fts3Int_h::Fts3Table;
pub use crate::fts3Int_h::LARGEST_INT64;
pub use crate::fts3Int_h::MatchinfoBuffer;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3CreateStatTable;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3DoclistPrev;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3FirstFilter;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintU;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3SegReaderCursor;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen;
pub use crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer;
pub use crate::src::ext::fts3::fts3_hash::_fts3ht;
pub use crate::src::ext::fts3::fts3_hash::Fts3Hash;
pub use crate::src::ext::fts3::fts3_hash::Fts3HashElem;
pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashClear;
pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFind;
pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFindElem;
pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInsert;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;
pub use crate::src::ext::fts3::fts3_write::stdlib_h::atoi;
pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::I16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_BLOB;
pub use crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT;
pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
pub use crate::src::headers::sqlite3_h::SQLITE_DONE;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_INTEGER;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_NULL;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_PREPARE_NO_VTAB;
pub use crate::src::headers::sqlite3_h::SQLITE_PREPARE_PERSISTENT;
pub use crate::src::headers::sqlite3_h::SQLITE_REPLACE;
pub use crate::src::headers::sqlite3_h::SQLITE_ROW;
pub use crate::src::headers::sqlite3_h::SQLITE_STATIC;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::Sqlite3Blob;
pub use crate::src::headers::sqlite3_h::Sqlite3DestructorType;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use crate::src::headers::sqlite3_h::sqlite3_index_info;
pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_module;
pub use crate::src::headers::sqlite3_h::Sqlite3Stmt;
pub use crate::src::headers::sqlite3_h::Sqlite3Uint64;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::headers::stdlib::ComparFnT;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::src::legacy::sqlite3_exec;
pub use crate::src::src::main::sqlite3_last_insert_rowid;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_malloc64;
pub use crate::src::src::malloc::sqlite3_realloc64;
pub use crate::src::src::prepare::sqlite3_prepare_v2;
pub use crate::src::src::prepare::sqlite3_prepare_v3;
pub use crate::src::src::util::sqlite3_strnicmp;
pub use crate::src::src::vdbeapi::sqlite3_bind_blob;
pub use crate::src::src::vdbeapi::sqlite3_bind_int;
pub use crate::src::src::vdbeapi::sqlite3_bind_int64;
pub use crate::src::src::vdbeapi::sqlite3_bind_null;
pub use crate::src::src::vdbeapi::sqlite3_bind_parameter_count;
pub use crate::src::src::vdbeapi::sqlite3_bind_text;
pub use crate::src::src::vdbeapi::sqlite3_bind_value;
pub use crate::src::src::vdbeapi::sqlite3_column_blob;
pub use crate::src::src::vdbeapi::sqlite3_column_bytes;
pub use crate::src::src::vdbeapi::sqlite3_column_int;
pub use crate::src::src::vdbeapi::sqlite3_column_int64;
pub use crate::src::src::vdbeapi::sqlite3_column_text;
pub use crate::src::src::vdbeapi::sqlite3_column_type;
pub use crate::src::src::vdbeapi::sqlite3_finalize;
pub use crate::src::src::vdbeapi::sqlite3_reset;
pub use crate::src::src::vdbeapi::sqlite3_step;
pub use crate::src::src::vdbeapi::sqlite3_value_bytes;
pub use crate::src::src::vdbeapi::sqlite3_value_int;
pub use crate::src::src::vdbeapi::sqlite3_value_int64;
pub use crate::src::src::vdbeapi::sqlite3_value_text;
pub use crate::src::src::vdbeapi::sqlite3_value_type;
pub use crate::src::src::vdbeblob::sqlite3_blob_bytes;
pub use crate::src::src::vdbeblob::sqlite3_blob_close;
pub use crate::src::src::vdbeblob::sqlite3_blob_open;
pub use crate::src::src::vdbeblob::sqlite3_blob_read;
pub use crate::src::src::vdbeblob::sqlite3_blob_reopen;
pub use crate::src::src::vtab::sqlite3_vtab_on_conflict;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3DeferredToken {
    pub pToken: *mut crate::fts3Int_h::Fts3PhraseToken,
    pub iCol: ::core::ffi::c_int,
    pub pNext: *mut Fts3DeferredToken,
    pub pList: *mut PendingList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PendingList {
    pub nData: ::core::ffi::c_int,
    pub aData: *mut ::core::ffi::c_char,
    pub nSpace: ::core::ffi::c_int,
    pub iLastDocid: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub iLastCol: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub iLastPos: crate::src::headers::sqlite3_h::Sqlite3Int64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3SegReader {
    pub iIdx: ::core::ffi::c_int,
    pub bLookup: crate::src::ext::rtree::rtree::U8_0,
    pub rootOnly: crate::src::ext::rtree::rtree::U8_0,
    pub iStartBlock: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub iLeafEndBlock: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub iEndBlock: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub iCurrentBlock: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub aNode: *mut ::core::ffi::c_char,
    pub nNode: ::core::ffi::c_int,
    pub nPopulate: ::core::ffi::c_int,
    pub pBlob: *mut crate::src::headers::sqlite3_h::Sqlite3Blob,
    pub ppNextElem: *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
    pub nTerm: ::core::ffi::c_int,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTermAlloc: ::core::ffi::c_int,
    pub aDoclist: *mut ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
    pub pOffsetList: *mut ::core::ffi::c_char,
    pub nOffsetList: ::core::ffi::c_int,
    pub iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SegmentWriter {
    pub pTree: *mut SegmentNode,
    pub iFirst: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub iFree: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTerm: ::core::ffi::c_int,
    pub nMalloc: ::core::ffi::c_int,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub nSize: ::core::ffi::c_int,
    pub nData: ::core::ffi::c_int,
    pub aData: *mut ::core::ffi::c_char,
    pub nLeafData: crate::src::ext::rtree::rtree::I64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SegmentNode {
    pub pParent: *mut SegmentNode,
    pub pRight: *mut SegmentNode,
    pub pLeftmost: *mut SegmentNode,
    pub nEntry: ::core::ffi::c_int,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTerm: ::core::ffi::c_int,
    pub nMalloc: ::core::ffi::c_int,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub nData: ::core::ffi::c_int,
    pub aData: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Blob {
    pub a: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IncrmergeWriter {
    pub nLeafEst: crate::src::ext::rtree::rtree::I64_0,
    pub nWork: crate::src::ext::rtree::rtree::I64_0,
    pub iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub iIdx: ::core::ffi::c_int,
    pub iStart: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub iEnd: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub nLeafData: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub bNoLeafData: crate::src::ext::rtree::rtree::U8_0,
    pub aNodeWriter: [NodeWriter; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeWriter {
    pub iBlock: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub key: Blob,
    pub block: Blob,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeReader {
    pub aNode: *const ::core::ffi::c_char,
    pub nNode: ::core::ffi::c_int,
    pub iOff: ::core::ffi::c_int,
    pub iChild: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub term: Blob,
    pub aDoclist: *const ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
}

pub const FTS_MAX_APPENDABLE_HEIGHT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const FTS3_NODE_PADDING: ::core::ffi::c_int =
    crate::fts3Int_h::FTS3_VARINT_MAX * 2 as ::core::ffi::c_int;

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static mut test_fts3_node_chunksize: ::core::ffi::c_int =
    4 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static mut test_fts3_node_chunk_threshold: ::core::ffi::c_int =
    4 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;

use crate::sqlite_printf;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtsStatConstant {
    FtsStatDoctotal = 0,
    FtsStatIncrmergehint = 1,
    FtsStatAutoincrmerge = 2,
}
impl From<FtsStatConstant> for i32 {
    fn from(v: FtsStatConstant) -> i32 {
        v as i32
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::FromRepr)]
pub enum SqlConstant {
    SqlDeleteContent = 0,
    SqlIsEmpty = 1,
    SqlDeleteAllContent = 2,
    SqlDeleteAllSegments = 3,
    SqlDeleteAllSegdir = 4,
    SqlDeleteAllDocsize = 5,
    SqlDeleteAllStat = 6,
    SqlSelectContentByRowid = 7,
    SqlNextSegmentIndex = 8,
    SqlInsertSegments = 9,
    SqlNextSegmentsId = 10,
    SqlInsertSegdir = 11,
    SqlSelectLevel = 12,
    SqlSelectLevelRange = 13,
    SqlCountSegdirAtLevel = 14,
    SqlSelectSegdirMaxLevel = 15,
    SqlDeleteSegdirLevel = 16,
    SqlDeleteSegmentsRange = 17,
    SqlContentInsert = 18,
    SqlDeleteDocsize = 19,
    SqlReplaceDocsize = 20,
    SqlSelectDocsize = 21,
    SqlSelectStat = 22,
    SqlReplaceStat = 23,
    SqlReserved24 = 24,
    SqlReserved25 = 25,
    SqlDeleteSegdirRange = 26,
    SqlSelectAllLangid = 27,
    SqlFindMergeLevel = 28,
    SqlMaxLeafNodeEstimate = 29,
    SqlDeleteSegdirEntry = 30,
    SqlShiftSegdirEntry = 31,
    SqlSelectSegdir = 32,
    SqlChompSegdir = 33,
    SqlSegmentIsAppendable = 34,
    SqlSelectIndexes = 35,
    SqlSelectMxlevel = 36,
    SqlSelectLevelRange2 = 37,
    SqlUpdateLevelIdx = 38,
    SqlUpdateLevel = 39,
}
unsafe extern "C" fn fts3SqlStmt(
    p: *mut crate::fts3Int_h::Fts3Table,
    eStmt: ::core::ffi::c_int,
    pp: *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt;
    pStmt = (*p).aStmt[eStmt as usize];
    if pStmt.is_null() {
        let mut f: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_PREPARE_PERSISTENT
            | crate::src::headers::sqlite3_h::SQLITE_PREPARE_NO_VTAB;
        
        let zSql: *mut ::core::ffi::c_char = match SqlConstant::from_repr(eStmt).unwrap_or(SqlConstant::SqlReserved24) {
            SqlConstant::SqlDeleteContent => sqlite_printf!(
                "DELETE FROM %Q.'%q_content' WHERE rowid = ?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlIsEmpty => sqlite_printf!(
                "SELECT NOT EXISTS(SELECT docid FROM %Q.'%q_content' WHERE rowid!=?)",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlDeleteAllContent => {
                sqlite_printf!("DELETE FROM %Q.'%q_content'", (*p).zDb, (*p).zName)
            }
            SqlConstant::SqlDeleteAllSegments => {
                sqlite_printf!("DELETE FROM %Q.'%q_segments'", (*p).zDb, (*p).zName)
            }
            SqlConstant::SqlDeleteAllSegdir => {
                sqlite_printf!("DELETE FROM %Q.'%q_segdir'", (*p).zDb, (*p).zName)
            }
            SqlConstant::SqlDeleteAllDocsize => {
                sqlite_printf!("DELETE FROM %Q.'%q_docsize'", (*p).zDb, (*p).zName)
            }
            SqlConstant::SqlDeleteAllStat => {
                sqlite_printf!("DELETE FROM %Q.'%q_stat'", (*p).zDb, (*p).zName)
            }
            SqlConstant::SqlSelectContentByRowid => {
                f &= !crate::src::headers::sqlite3_h::SQLITE_PREPARE_NO_VTAB;
                sqlite_printf!("SELECT %s WHERE rowid=?", (*p).zReadExprlist)
            }
            SqlConstant::SqlNextSegmentIndex => sqlite_printf!(
                "SELECT (SELECT max(idx) FROM %Q.'%q_segdir' WHERE level = ?) + 1",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlInsertSegments => sqlite_printf!(
                "REPLACE INTO %Q.'%q_segments'(blockid, block) VALUES(?, ?)",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlNextSegmentsId => sqlite_printf!(
                "SELECT coalesce((SELECT max(blockid) FROM %Q.'%q_segments') + 1, 1)",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlInsertSegdir => sqlite_printf!(
                "REPLACE INTO %Q.'%q_segdir' VALUES(?,?,?,?,?,?)",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSelectLevel => sqlite_printf!(
                "SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level = ? ORDER BY idx ASC",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSelectLevelRange => sqlite_printf!(
                "SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?ORDER BY level DESC, idx ASC",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlCountSegdirAtLevel => sqlite_printf!(
                "SELECT count(*) FROM %Q.'%q_segdir' WHERE level = ?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSelectSegdirMaxLevel => sqlite_printf!(
                "SELECT max(level) FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlDeleteSegdirLevel => sqlite_printf!(
                "DELETE FROM %Q.'%q_segdir' WHERE level = ?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlDeleteSegmentsRange => sqlite_printf!(
                "DELETE FROM %Q.'%q_segments' WHERE blockid BETWEEN ? AND ?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlContentInsert => sqlite_printf!(
                "INSERT INTO %Q.'%q_content' VALUES(%s)",
                (*p).zDb,
                (*p).zName,
                (*p).zWriteExprlist
            ),
            SqlConstant::SqlDeleteDocsize => sqlite_printf!(
                "DELETE FROM %Q.'%q_docsize' WHERE docid = ?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlReplaceDocsize => sqlite_printf!(
                "REPLACE INTO %Q.'%q_docsize' VALUES(?,?)",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSelectDocsize => sqlite_printf!(
                "SELECT size FROM %Q.'%q_docsize' WHERE docid=?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSelectStat => sqlite_printf!(
                "SELECT value FROM %Q.'%q_stat' WHERE id=?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlReplaceStat => sqlite_printf!(
                "REPLACE INTO %Q.'%q_stat' VALUES(?,?)",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlReserved24 | SqlConstant::SqlReserved25 => ::core::ptr::null_mut(),
            SqlConstant::SqlDeleteSegdirRange => sqlite_printf!(
                "DELETE FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSelectAllLangid => sqlite_printf!(
                "SELECT ? UNION SELECT level / (1024 * ?) FROM %Q.'%q_segdir'",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlFindMergeLevel => sqlite_printf!(
                "SELECT level, count(*) AS cnt FROM %Q.'%q_segdir'   GROUP BY level HAVING cnt>=?  ORDER BY (level %% 1024) ASC, 2 DESC LIMIT 1",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlMaxLeafNodeEstimate => sqlite_printf!(
                "SELECT 2 * total(1 + leaves_end_block - start_block)   FROM (SELECT * FROM %Q.'%q_segdir'         WHERE level = ? ORDER BY idx ASC LIMIT ?  )",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlDeleteSegdirEntry => sqlite_printf!(
                "DELETE FROM %Q.'%q_segdir' WHERE level = ? AND idx = ?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlShiftSegdirEntry => sqlite_printf!(
                "UPDATE %Q.'%q_segdir' SET idx = ? WHERE level=? AND idx=?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSelectSegdir => sqlite_printf!(
                "SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level = ? AND idx = ?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlChompSegdir => sqlite_printf!(
                "UPDATE %Q.'%q_segdir' SET start_block = ?, root = ?WHERE level = ? AND idx = ?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSegmentIsAppendable => sqlite_printf!(
                "SELECT 1 FROM %Q.'%q_segments' WHERE blockid=? AND block IS NULL",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSelectIndexes => sqlite_printf!(
                "SELECT idx FROM %Q.'%q_segdir' WHERE level=? ORDER BY 1 ASC",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSelectMxlevel => sqlite_printf!(
                "SELECT max( level %% 1024 ) FROM %Q.'%q_segdir'",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlSelectLevelRange2 => sqlite_printf!(
                "SELECT level, idx, end_block FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ? ORDER BY level DESC, idx ASC",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlUpdateLevelIdx => sqlite_printf!(
                "UPDATE OR FAIL %Q.'%q_segdir' SET level=-1,idx=? WHERE level=? AND idx=?",
                (*p).zDb,
                (*p).zName
            ),
            SqlConstant::SqlUpdateLevel => sqlite_printf!(
                "UPDATE OR FAIL %Q.'%q_segdir' SET level=? WHERE level=-1",
                (*p).zDb,
                (*p).zName
            ),
        };
        if zSql.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            rc = crate::src::src::prepare::sqlite3_prepare_v3(
                (*p).db,
                zSql,
                -(1 as ::core::ffi::c_int),
                f as ::core::ffi::c_uint,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
            (*p).aStmt[eStmt as usize] = pStmt;
        }
    }
    if !apVal.is_null() {
        let mut i: ::core::ffi::c_int;
        let nParam: ::core::ffi::c_int =
            crate::src::src::vdbeapi::sqlite3_bind_parameter_count(pStmt);
        i = 0 as ::core::ffi::c_int;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < nParam {
            rc = crate::src::src::vdbeapi::sqlite3_bind_value(
                pStmt,
                i + 1 as ::core::ffi::c_int,
                *apVal.offset(i as isize),
            );
            i += 1;
        }
    }
    *pp = pStmt;
    rc
}

unsafe extern "C" fn fts3SelectDocsize(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64,
    ppStmt: *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int;
    rc = fts3SqlStmt(
        pTab,
        SqlConstant::SqlSelectDocsize as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iDocid);
        rc = crate::src::src::vdbeapi::sqlite3_step(pStmt);
        if rc != crate::src::headers::sqlite3_h::SQLITE_ROW
            || crate::src::src::vdbeapi::sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int)
                != crate::src::headers::sqlite3_h::SQLITE_BLOB
        {
            rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
            pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    *ppStmt = pStmt;
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3SelectDoctotal(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    ppStmt: *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int;
    rc = fts3SqlStmt(
        pTab,
        SqlConstant::SqlSelectStat as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int(
            pStmt,
            1 as ::core::ffi::c_int,
            FtsStatConstant::FtsStatDoctotal as ::core::ffi::c_int,
        );
        if crate::src::src::vdbeapi::sqlite3_step(pStmt)
            != crate::src::headers::sqlite3_h::SQLITE_ROW
            || crate::src::src::vdbeapi::sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int)
                != crate::src::headers::sqlite3_h::SQLITE_BLOB
        {
            rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
            pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        }
    }
    *ppStmt = pStmt;
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3SelectDocsize(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64,
    ppStmt: *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
) -> ::core::ffi::c_int {
    fts3SelectDocsize(pTab, iDocid, ppStmt)
}

unsafe extern "C" fn fts3SqlExec(
    pRC: *mut ::core::ffi::c_int,
    p: *mut crate::fts3Int_h::Fts3Table,
    eStmt: ::core::ffi::c_int,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int;
    if *pRC != 0 {
        return;
    }
    rc = fts3SqlStmt(p, eStmt, &raw mut pStmt, apVal);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_step(pStmt);
        rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    }
    *pRC = rc;
}

unsafe extern "C" fn fts3Writelock(p: *mut crate::fts3Int_h::Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*p).nPendingData == 0 as ::core::ffi::c_int {
        let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlDeleteSegdirLevel as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_null(pStmt, 1 as ::core::ffi::c_int);
            crate::src::src::vdbeapi::sqlite3_step(pStmt);
            rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
        }
    }
    rc
}

unsafe extern "C" fn getAbsoluteLevel(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    iIndex: ::core::ffi::c_int,
    iLevel: ::core::ffi::c_int,
) -> crate::src::headers::sqlite3_h::Sqlite3Int64 {
    
    let iBase: crate::src::headers::sqlite3_h::Sqlite3Int64 = (iLangid as crate::src::headers::sqlite3_h::Sqlite3Int64
        * (*p).nIndex as crate::src::headers::sqlite3_h::Sqlite3Int64
        + iIndex as crate::src::headers::sqlite3_h::Sqlite3Int64)
        * crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL as crate::src::headers::sqlite3_h::Sqlite3Int64;
    iBase + iLevel as crate::src::headers::sqlite3_h::Sqlite3Int64
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3AllSegdirs(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    iIndex: ::core::ffi::c_int,
    iLevel: ::core::ffi::c_int,
    ppStmt: *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    if iLevel < 0 as ::core::ffi::c_int {
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlSelectLevelRange as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pStmt,
                1 as ::core::ffi::c_int,
                getAbsoluteLevel(p, iLangid, iIndex, 0 as ::core::ffi::c_int),
            );
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pStmt,
                2 as ::core::ffi::c_int,
                getAbsoluteLevel(
                    p,
                    iLangid,
                    iIndex,
                    crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL - 1 as ::core::ffi::c_int,
                ),
            );
        }
    } else {
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlSelectLevel as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pStmt,
                1 as ::core::ffi::c_int,
                getAbsoluteLevel(p, iLangid, iIndex, iLevel),
            );
        }
    }
    *ppStmt = pStmt;
    rc
}

unsafe extern "C" fn fts3PendingListAppendVarint(
    pp: *mut *mut PendingList,
    i: crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let mut p: *mut PendingList = *pp;
    if p.is_null() {
        p = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<PendingList>() as usize).wrapping_add(100_usize)
                as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut PendingList;
        if p.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        (*p).nSpace = 100 as ::core::ffi::c_int;
        (*p).aData = p.offset(1_isize) as *mut PendingList as *mut ::core::ffi::c_char;
        (*p).nData = 0 as ::core::ffi::c_int;
    } else if (*p).nData + crate::fts3Int_h::FTS3_VARINT_MAX + 1 as ::core::ffi::c_int > (*p).nSpace
    {
        let nNew: crate::src::ext::rtree::rtree::I64_0 =
            ((*p).nSpace * 2 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::I64_0;
        p = crate::src::src::malloc::sqlite3_realloc64(
            p as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<PendingList>()
                as crate::src::headers::sqlite3_h::Sqlite3Uint64)
                .wrapping_add(nNew as crate::src::headers::sqlite3_h::Sqlite3Uint64),
        ) as *mut PendingList;
        if p.is_null() {
            crate::src::src::malloc::sqlite3_free(*pp as *mut ::core::ffi::c_void);
            *pp = ::core::ptr::null_mut::<PendingList>();
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        (*p).nSpace = nNew as ::core::ffi::c_int;
        (*p).aData = p.offset(1_isize) as *mut PendingList as *mut ::core::ffi::c_char;
    }
    (*p).nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
        (*p).aData.offset((*p).nData as isize) as *mut ::core::ffi::c_char,
        i,
    );
    *(*p).aData.offset((*p).nData as isize) = '\0' as i32 as ::core::ffi::c_char;
    *pp = p;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3PendingListAppend(
    pp: *mut *mut PendingList,
    iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iCol: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iPos: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut p: *mut PendingList = *pp;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if p.is_null() || (*p).iLastDocid != iDocid {
        let iDelta: crate::src::ext::rtree::rtree::U64_0 =
            (iDocid as crate::src::ext::rtree::rtree::U64_0).wrapping_sub(
                (if !p.is_null() {
                    (*p).iLastDocid
                } else {
                    0 as crate::src::headers::sqlite3_h::Sqlite3Int64
                }) as crate::src::ext::rtree::rtree::U64_0,
            );
        if !p.is_null() {
            (*p).nData += 1;
        }
        rc = fts3PendingListAppendVarint(
            &raw mut p,
            iDelta as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        if crate::src::headers::sqlite3_h::SQLITE_OK != rc {
            current_block = 17264066162094073856;
        } else {
            let __p_ref = unsafe { &mut *p };
            __p_ref.iLastCol =
                -(1 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::Sqlite3Int64;
            __p_ref.iLastPos = 0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            __p_ref.iLastDocid = iDocid;
            current_block = 3640593987805443782;
        }
    } else {
        current_block = 3640593987805443782;
    }
    if current_block == 3640593987805443782 {
        if iCol > 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 && (*p).iLastCol != iCol {
            rc = fts3PendingListAppendVarint(
                &raw mut p,
                1 as crate::src::headers::sqlite3_h::Sqlite3Int64,
            );
            if crate::src::headers::sqlite3_h::SQLITE_OK != rc || {
                rc = fts3PendingListAppendVarint(&raw mut p, iCol);
                crate::src::headers::sqlite3_h::SQLITE_OK != rc
            } {
                current_block = 17264066162094073856;
            } else {
                (*p).iLastCol = iCol;
                (*p).iLastPos = 0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
                current_block = 4166486009154926805;
            }
        } else {
            current_block = 4166486009154926805;
        }
        match current_block {
            17264066162094073856 => {}
            _ => {
                if iCol >= 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 {
                    rc = fts3PendingListAppendVarint(
                        &raw mut p,
                        2 as crate::src::headers::sqlite3_h::Sqlite3Int64 + iPos
                            - (*p).iLastPos,
                    );
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        (*p).iLastPos = iPos;
                    }
                }
            }
        }
    }
    *pRc = rc;
    if p != *pp {
        *pp = p;
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn fts3PendingListDelete(pList: *mut PendingList) {
    crate::src::src::malloc::sqlite3_free(pList as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn fts3PendingTermsAddOne(
    p: *mut crate::fts3Int_h::Fts3Table,
    iCol: ::core::ffi::c_int,
    iPos: ::core::ffi::c_int,
    pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    zToken: *const ::core::ffi::c_char,
    nToken: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pList: *mut PendingList;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    pList = crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFind(
        pHash as *const crate::src::ext::fts3::fts3_hash::Fts3Hash,
        zToken as *const ::core::ffi::c_void,
        nToken,
    ) as *mut PendingList;
    if !pList.is_null() {
        (*p).nPendingData = ((*p).nPendingData as ::core::ffi::c_ulong).wrapping_sub(
            (((*pList).nData + nToken) as usize).wrapping_add(::core::mem::size_of::<
                crate::src::ext::fts3::fts3_hash::Fts3HashElem,
            >() as usize) as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if fts3PendingListAppend(
        &raw mut pList,
        (*p).iPrevDocid as crate::src::headers::sqlite3_h::Sqlite3Int64,
        iCol as crate::src::headers::sqlite3_h::Sqlite3Int64,
        iPos as crate::src::headers::sqlite3_h::Sqlite3Int64,
        &raw mut rc,
    ) != 0
        && pList
            == crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInsert(
                pHash as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
                zToken as *const ::core::ffi::c_void,
                nToken,
                pList as *mut ::core::ffi::c_void,
            ) as *mut PendingList
        {
            crate::src::src::malloc::sqlite3_free(pList as *mut ::core::ffi::c_void);
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        (*p).nPendingData = ((*p).nPendingData as ::core::ffi::c_ulong).wrapping_add(
            (((*pList).nData + nToken) as usize).wrapping_add(::core::mem::size_of::<
                crate::src::ext::fts3::fts3_hash::Fts3HashElem,
            >() as usize) as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    rc
}

unsafe extern "C" fn fts3PendingTermsAdd(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    zText: *const ::core::ffi::c_char,
    iCol: ::core::ffi::c_int,
    pnWord: *mut crate::src::ext::rtree::rtree::U32_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut iStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iPos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nWord: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut zToken: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer =
        (*p).pTokenizer;
    let pModule: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module =
        (*pTokenizer).pModule;
    let mut pCsr: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
    
    if zText.is_null() {
        *pnWord = 0 as crate::src::ext::rtree::rtree::U32_0;
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc = crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer(
        pTokenizer as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
        iLangid,
        zText,
        -(1 as ::core::ffi::c_int),
        &raw mut pCsr as *mut _
            as *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    let xNext: Option<
        unsafe extern "C" fn(
            *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    > = (*pModule).xNext;
    while crate::src::headers::sqlite3_h::SQLITE_OK == rc && {
        rc = xNext.expect("non-null function pointer")(
            pCsr,
            &raw mut zToken,
            &raw mut nToken,
            &raw mut iStart,
            &raw mut iEnd,
            &raw mut iPos,
        );
        crate::src::headers::sqlite3_h::SQLITE_OK == rc
    } {
        let mut i: ::core::ffi::c_int;
        if iPos >= nWord {
            nWord = iPos + 1 as ::core::ffi::c_int;
        }
        if iPos < 0 as ::core::ffi::c_int || zToken.is_null() || nToken <= 0 as ::core::ffi::c_int {
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            break;
        } else {
            rc = fts3PendingTermsAddOne(
                p,
                iCol,
                iPos,
                &raw mut (*(*p).aIndex.offset(0_isize)).hPending,
                zToken,
                nToken,
            );
            i = 1 as ::core::ffi::c_int;
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < (*p).nIndex {
                let pIndex: *mut crate::fts3Int_h::Fts3Index =
                    (*p).aIndex.offset(i as isize) as *mut crate::fts3Int_h::Fts3Index;
                if (nToken >= (*pIndex).nPrefix) {
                    rc = fts3PendingTermsAddOne(
                        p,
                        iCol,
                        iPos,
                        &raw mut (*pIndex).hPending,
                        zToken,
                        (*pIndex).nPrefix,
                    );
                }
                i += 1;
            }
        }
    }
    (*pModule).xClose.expect("non-null function pointer")(pCsr);
    *pnWord = (*pnWord).wrapping_add(nWord as crate::src::ext::rtree::rtree::U32_0);
    if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
        crate::src::headers::sqlite3_h::SQLITE_OK
    } else {
        rc
    }
}

unsafe extern "C" fn fts3PendingTermsDocid(
    p: *mut crate::fts3Int_h::Fts3Table,
    bDelete: ::core::ffi::c_int,
    iLangid: ::core::ffi::c_int,
    iDocid: crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let __p_ref = unsafe { &mut *p };
    if iDocid < __p_ref.iPrevDocid
        || iDocid == __p_ref.iPrevDocid && __p_ref.bPrevDelete == 0 as ::core::ffi::c_int
        || __p_ref.iPrevLangid != iLangid
        || __p_ref.nPendingData > __p_ref.nMaxPendingData
    {
        let rc: ::core::ffi::c_int = sqlite3Fts3PendingTermsFlush(p);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
    }
    __p_ref.iPrevDocid = iDocid;
    __p_ref.iPrevLangid = iLangid;
    __p_ref.bPrevDelete = bDelete;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3PendingTermsClear(p: *mut crate::fts3Int_h::Fts3Table) {
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nIndex {
        let mut pElem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem;
        let pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash =
            &raw mut (*(*p).aIndex.offset(i as isize)).hPending;
        pElem = (*pHash).first;
        while !pElem.is_null() {
            let pList: *mut PendingList = (*pElem).data as *mut PendingList;
            fts3PendingListDelete(pList);
            pElem = (*pElem).next;
        }
        crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashClear(
            pHash as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
        );
        i += 1;
    }
    (*p).nPendingData = 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn fts3InsertTerms(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    aSz: *mut crate::src::ext::rtree::rtree::U32_0,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    i = 2 as ::core::ffi::c_int;
    while i < (*p).nColumn + 2 as ::core::ffi::c_int {
        let iCol: ::core::ffi::c_int = i - 2 as ::core::ffi::c_int;
        if *(*p).abNotindexed.offset(iCol as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            let zText: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_value_text(*apVal.offset(i as isize))
                    as *const ::core::ffi::c_char;
            let rc: ::core::ffi::c_int = fts3PendingTermsAdd(
                p,
                iLangid,
                zText,
                iCol,
                aSz.offset(iCol as isize) as *mut crate::src::ext::rtree::rtree::U32_0,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            let fresh2 = &mut *aSz.offset((*p).nColumn as isize);
            *fresh2 = (*fresh2).wrapping_add(crate::src::src::vdbeapi::sqlite3_value_bytes(
                *apVal.offset(i as isize),
            ) as crate::src::ext::rtree::rtree::U32_0);
        }
        i += 1;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3InsertData(
    p: *mut crate::fts3Int_h::Fts3Table,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    piDocid: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pContentInsert: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let __p_ref = unsafe { &mut *p };
    if !__p_ref.zContentTbl.is_null() {
        let mut pRowid: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
            *apVal.offset((__p_ref.nColumn + 3 as ::core::ffi::c_int) as isize);
        if crate::src::src::vdbeapi::sqlite3_value_type(pRowid)
            == crate::src::headers::sqlite3_h::SQLITE_NULL
        {
            pRowid = *apVal.offset(1_isize);
        }
        if crate::src::src::vdbeapi::sqlite3_value_type(pRowid)
            != crate::src::headers::sqlite3_h::SQLITE_INTEGER
        {
            return crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
        }
        *piDocid = crate::src::src::vdbeapi::sqlite3_value_int64(pRowid);
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlContentInsert as ::core::ffi::c_int,
        &raw mut pContentInsert,
        apVal.offset(1_isize) as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !__p_ref.zLanguageid.is_null() {
        rc = crate::src::src::vdbeapi::sqlite3_bind_int(
            pContentInsert,
            __p_ref.nColumn + 2 as ::core::ffi::c_int,
            crate::src::src::vdbeapi::sqlite3_value_int(
                *apVal.offset((__p_ref.nColumn + 4 as ::core::ffi::c_int) as isize),
            ),
        );
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    if crate::src::headers::sqlite3_h::SQLITE_NULL
        != crate::src::src::vdbeapi::sqlite3_value_type(
            *apVal.offset((3 as ::core::ffi::c_int + __p_ref.nColumn) as isize),
        )
    {
        if crate::src::headers::sqlite3_h::SQLITE_NULL
            == crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(0_isize))
            && crate::src::headers::sqlite3_h::SQLITE_NULL
                != crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(1_isize))
        {
            return crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
        rc = crate::src::src::vdbeapi::sqlite3_bind_value(
            pContentInsert,
            1 as ::core::ffi::c_int,
            *apVal.offset((3 as ::core::ffi::c_int + __p_ref.nColumn) as isize),
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
    }
    crate::src::src::vdbeapi::sqlite3_step(pContentInsert);
    rc = crate::src::src::vdbeapi::sqlite3_reset(pContentInsert);
    *piDocid = crate::src::src::main::sqlite3_last_insert_rowid(__p_ref.db);
    rc
}

unsafe extern "C" fn fts3DeleteAll(
    p: *mut crate::fts3Int_h::Fts3Table,
    bContent: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    sqlite3Fts3PendingTermsClear(p);
    if bContent != 0 {
        fts3SqlExec(
            &raw mut rc,
            p,
            SqlConstant::SqlDeleteAllContent as ::core::ffi::c_int,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    fts3SqlExec(
        &raw mut rc,
        p,
        SqlConstant::SqlDeleteAllSegments as ::core::ffi::c_int,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    fts3SqlExec(
        &raw mut rc,
        p,
        SqlConstant::SqlDeleteAllSegdir as ::core::ffi::c_int,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if (*p).bHasDocsize != 0 {
        fts3SqlExec(
            &raw mut rc,
            p,
            SqlConstant::SqlDeleteAllDocsize as ::core::ffi::c_int,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    if (*p).bHasStat != 0 {
        fts3SqlExec(
            &raw mut rc,
            p,
            SqlConstant::SqlDeleteAllStat as ::core::ffi::c_int,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    rc
}

unsafe extern "C" fn langidFromSelect(
    p: *mut crate::fts3Int_h::Fts3Table,
    pSelect: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
) -> ::core::ffi::c_int {
    let mut iLangid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*p).zLanguageid.is_null() {
        iLangid = crate::src::src::vdbeapi::sqlite3_column_int(
            pSelect,
            (*p).nColumn + 1 as ::core::ffi::c_int,
        );
    }
    iLangid
}

unsafe extern "C" fn fts3DeleteTerms(
    pRC: *mut ::core::ffi::c_int,
    p: *mut crate::fts3Int_h::Fts3Table,
    mut pRowid: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    aSz: *mut crate::src::ext::rtree::rtree::U32_0,
    pbFound: *mut ::core::ffi::c_int,
) {
    let mut rc: ::core::ffi::c_int;
    let mut pSelect: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    if *pRC != 0 {
        return;
    }
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectContentByRowid as ::core::ffi::c_int,
        &raw mut pSelect,
        &raw mut pRowid,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pSelect)
        {
            let mut i: ::core::ffi::c_int;
            let iLangid: ::core::ffi::c_int = langidFromSelect(p, pSelect);
            let iDocid: crate::src::ext::rtree::rtree::I64_0 =
                crate::src::src::vdbeapi::sqlite3_column_int64(pSelect, 0 as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::I64_0;
            rc = fts3PendingTermsDocid(
                p,
                1 as ::core::ffi::c_int,
                iLangid,
                iDocid as crate::src::headers::sqlite3_h::SqliteInt64,
            );
            i = 1 as ::core::ffi::c_int;
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i <= (*p).nColumn {
                let iCol: ::core::ffi::c_int = i - 1 as ::core::ffi::c_int;
                if *(*p).abNotindexed.offset(iCol as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let zText: *const ::core::ffi::c_char =
                        crate::src::src::vdbeapi::sqlite3_column_text(pSelect, i)
                            as *const ::core::ffi::c_char;
                    rc = fts3PendingTermsAdd(
                        p,
                        iLangid,
                        zText,
                        -(1 as ::core::ffi::c_int),
                        aSz.offset(iCol as isize) as *mut crate::src::ext::rtree::rtree::U32_0,
                    );
                    let fresh11 = &mut *aSz.offset((*p).nColumn as isize);
                    *fresh11 = (*fresh11)
                        .wrapping_add(crate::src::src::vdbeapi::sqlite3_column_bytes(pSelect, i)
                            as crate::src::ext::rtree::rtree::U32_0);
                }
                i += 1;
            }
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::vdbeapi::sqlite3_reset(pSelect);
                *pRC = rc;
                return;
            }
            *pbFound = 1 as ::core::ffi::c_int;
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pSelect);
    } else {
        crate::src::src::vdbeapi::sqlite3_reset(pSelect);
    }
    *pRC = rc;
}

unsafe extern "C" fn fts3AllocateSegdirIdx(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    iIndex: ::core::ffi::c_int,
    iLevel: ::core::ffi::c_int,
    piIdx: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pNextIdx: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut iNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlNextSegmentIndex as ::core::ffi::c_int,
        &raw mut pNextIdx,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pNextIdx,
            1 as ::core::ffi::c_int,
            getAbsoluteLevel(p, iLangid, iIndex, iLevel),
        );
        if crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pNextIdx)
        {
            iNext = crate::src::src::vdbeapi::sqlite3_column_int(pNextIdx, 0 as ::core::ffi::c_int);
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pNextIdx);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if iNext >= (*p).nMergeCount {
            rc = fts3SegmentMerge(p, iLangid, iIndex, iLevel);
            *piIdx = 0 as ::core::ffi::c_int;
        } else {
            *piIdx = iNext;
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3ReadBlock(
    p: *mut crate::fts3Int_h::Fts3Table,
    iBlockid: crate::src::headers::sqlite3_h::Sqlite3Int64,
    paBlob: *mut *mut ::core::ffi::c_char,
    pnBlob: *mut ::core::ffi::c_int,
    pnLoad: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    if !(*p).pSegments.is_null() {
        rc = crate::src::src::vdbeblob::sqlite3_blob_reopen((*p).pSegments, iBlockid);
    } else {
        let __p_ref = unsafe { &mut *p };
        if __p_ref.zSegmentsTbl.is_null() {
            __p_ref.zSegmentsTbl = sqlite_printf!("%s_segments", __p_ref.zName);
            if __p_ref.zSegmentsTbl.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
        }
        rc = crate::src::src::vdbeblob::sqlite3_blob_open(
            __p_ref.db,
            __p_ref.zDb,
            __p_ref.zSegmentsTbl,
            b"block\0" as *const u8 as *const ::core::ffi::c_char,
            iBlockid,
            0 as ::core::ffi::c_int,
            &raw mut __p_ref.pSegments,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut nByte: ::core::ffi::c_int =
            crate::src::src::vdbeblob::sqlite3_blob_bytes((*p).pSegments);
        *pnBlob = nByte;
        if !paBlob.is_null() {
            let mut aByte: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_malloc64(
                (nByte as crate::src::ext::rtree::rtree::I64_0
                    + FTS3_NODE_PADDING as crate::src::ext::rtree::rtree::I64_0)
                    as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ) as *mut ::core::ffi::c_char;
            if aByte.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                if !pnLoad.is_null() && nByte > test_fts3_node_chunk_threshold {
                    nByte = test_fts3_node_chunksize;
                    *pnLoad = nByte;
                }
                rc = crate::src::src::vdbeblob::sqlite3_blob_read(
                    (*p).pSegments,
                    aByte as *mut ::core::ffi::c_void,
                    nByte,
                    0 as ::core::ffi::c_int,
                );
                std::ptr::write_bytes(
                    aByte.offset(nByte as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void as *mut u8,
                    0 as ::core::ffi::c_int as u8,
                    FTS3_NODE_PADDING as crate::__stddef_size_t_h::SizeT,
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    crate::src::src::malloc::sqlite3_free(aByte as *mut ::core::ffi::c_void);
                    aByte = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            }
            *paBlob = aByte;
        }
    } else if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR {
        rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3SegmentsClose(p: *mut crate::fts3Int_h::Fts3Table) {
    crate::src::src::vdbeblob::sqlite3_blob_close((*p).pSegments);
    (*p).pSegments = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Blob>();
}

unsafe extern "C" fn fts3SegReaderIncrRead(pReader: *mut Fts3SegReader) -> ::core::ffi::c_int {
    
    
    let __pReader_ref = unsafe { &mut *pReader };
    let nRead: ::core::ffi::c_int = if __pReader_ref.nNode - __pReader_ref.nPopulate < test_fts3_node_chunksize {
        __pReader_ref.nNode - __pReader_ref.nPopulate
    } else {
        test_fts3_node_chunksize
    };
    let rc: ::core::ffi::c_int = crate::src::src::vdbeblob::sqlite3_blob_read(
        __pReader_ref.pBlob,
        __pReader_ref.aNode.offset(__pReader_ref.nPopulate as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        nRead,
        __pReader_ref.nPopulate,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        __pReader_ref.nPopulate += nRead;
        std::ptr::write_bytes(
            __pReader_ref.aNode.offset(__pReader_ref.nPopulate as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void as *mut u8,
            0 as ::core::ffi::c_int as u8,
            FTS3_NODE_PADDING as crate::__stddef_size_t_h::SizeT,
        );
        if __pReader_ref.nPopulate == __pReader_ref.nNode {
            crate::src::src::vdbeblob::sqlite3_blob_close(__pReader_ref.pBlob);
            __pReader_ref.pBlob =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Blob>();
            __pReader_ref.nPopulate = 0 as ::core::ffi::c_int;
        }
    }
    rc
}

unsafe extern "C" fn fts3SegReaderRequire(
    pReader: *mut Fts3SegReader,
    pFrom: *mut ::core::ffi::c_char,
    nByte: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pReader_ref = unsafe { &mut *pReader };
    while !__pReader_ref.pBlob.is_null()
        && rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && pFrom.offset_from(__pReader_ref.aNode) as ::core::ffi::c_long
            + nByte as ::core::ffi::c_long
            > __pReader_ref.nPopulate as ::core::ffi::c_long
    {
        rc = fts3SegReaderIncrRead(pReader);
    }
    rc
}

unsafe extern "C" fn fts3SegReaderSetEof(pSeg: *mut Fts3SegReader) {
    if ((*pSeg).rootOnly as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
        let __pSeg_ref = unsafe { &mut *pSeg };
        crate::src::src::malloc::sqlite3_free(__pSeg_ref.aNode as *mut ::core::ffi::c_void);
        crate::src::src::vdbeblob::sqlite3_blob_close(__pSeg_ref.pBlob);
        __pSeg_ref.pBlob = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Blob>();
    }
    (*pSeg).aNode = ::core::ptr::null_mut::<::core::ffi::c_char>();
}

unsafe extern "C" fn fts3SegReaderNext(
    p: *mut crate::fts3Int_h::Fts3Table,
    pReader: *mut Fts3SegReader,
    bIncr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pNext: *mut ::core::ffi::c_char;
    let mut nPrefix: ::core::ffi::c_int = 0;
    let mut nSuffix: ::core::ffi::c_int = 0;
    let __pReader_ref = unsafe { &mut *pReader };
    if __pReader_ref.aDoclist.is_null() {
        pNext = __pReader_ref.aNode;
    } else {
        pNext = __pReader_ref
            .aDoclist
            .offset(__pReader_ref.nDoclist as isize) as *mut ::core::ffi::c_char;
    }
    if pNext.is_null()
        || pNext
            >= __pReader_ref.aNode.offset(__pReader_ref.nNode as isize) as *mut ::core::ffi::c_char
    {
        if !__pReader_ref.ppNextElem.is_null() {
            let pElem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem =
                *__pReader_ref.ppNextElem;
            crate::src::src::malloc::sqlite3_free(__pReader_ref.aNode as *mut ::core::ffi::c_void);
            __pReader_ref.aNode = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !pElem.is_null() {
                
                let __pElem_ref = unsafe { &*pElem };
                let pList: *mut PendingList = __pElem_ref.data as *mut PendingList;
                let nCopy: ::core::ffi::c_int = (*pList).nData + 1 as ::core::ffi::c_int;
                let nTerm: ::core::ffi::c_int = __pElem_ref.nKey;
                if nTerm + 1 as ::core::ffi::c_int > __pReader_ref.nTermAlloc {
                    crate::src::src::malloc::sqlite3_free(
                        __pReader_ref.zTerm as *mut ::core::ffi::c_void,
                    );
                    __pReader_ref.zTerm = crate::src::src::malloc::sqlite3_malloc64(
                        ((nTerm as crate::src::ext::rtree::rtree::I64_0
                            + 1 as crate::src::ext::rtree::rtree::I64_0)
                            * 2 as crate::src::ext::rtree::rtree::I64_0)
                            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                    ) as *mut ::core::ffi::c_char;
                    if __pReader_ref.zTerm.is_null() {
                        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    }
                    __pReader_ref.nTermAlloc =
                        (nTerm + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int;
                }
                std::ptr::copy_nonoverlapping(
                    __pElem_ref.pKey as *const u8,
                    __pReader_ref.zTerm as *mut ::core::ffi::c_void as *mut u8,
                    nTerm as crate::__stddef_size_t_h::SizeT,
                );
                *__pReader_ref.zTerm.offset(nTerm as isize) = '\0' as i32 as ::core::ffi::c_char;
                __pReader_ref.nTerm = nTerm;
                let aCopy: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_malloc64(
                    nCopy as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                ) as *mut ::core::ffi::c_char;
                if aCopy.is_null() {
                    return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                }
                ::core::ptr::copy_nonoverlapping(
                    (*pList).aData as *const u8,
                    aCopy as *mut u8,
                    nCopy as usize,
                );
                __pReader_ref.nDoclist = nCopy;
                __pReader_ref.nNode = __pReader_ref.nDoclist;
                __pReader_ref.aDoclist = aCopy;
                __pReader_ref.aNode = __pReader_ref.aDoclist;
                __pReader_ref.ppNextElem = __pReader_ref.ppNextElem.offset(1);
            }
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        fts3SegReaderSetEof(pReader);
        if __pReader_ref.iCurrentBlock >= __pReader_ref.iLeafEndBlock {
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        __pReader_ref.iCurrentBlock += 1;
        rc = sqlite3Fts3ReadBlock(
            p,
            __pReader_ref.iCurrentBlock,
            &raw mut __pReader_ref.aNode,
            &raw mut __pReader_ref.nNode,
            if bIncr != 0 {
                &raw mut __pReader_ref.nPopulate
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_int>()
            },
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        if bIncr != 0 && __pReader_ref.nPopulate < __pReader_ref.nNode {
            __pReader_ref.pBlob = (*p).pSegments;
            (*p).pSegments =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Blob>();
        }
        pNext = __pReader_ref.aNode;
    }
    rc = fts3SegReaderRequire(
        pReader,
        pNext,
        crate::fts3Int_h::FTS3_VARINT_MAX * 2 as ::core::ffi::c_int,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    pNext = pNext.offset(
        (if *(pNext as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
        {
            crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(pNext, &raw mut nPrefix)
        } else {
            nPrefix = *(pNext as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        }) as isize,
    );
    pNext = pNext.offset(
        (if *(pNext as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
        {
            crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(pNext, &raw mut nSuffix)
        } else {
            nSuffix = *(pNext as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        }) as isize,
    );
    if nSuffix <= 0 as ::core::ffi::c_int
        || ((__pReader_ref.aNode.offset(__pReader_ref.nNode as isize) as *mut ::core::ffi::c_char)
            .offset_from(pNext) as ::core::ffi::c_long)
            < nSuffix as ::core::ffi::c_long
        || nPrefix > __pReader_ref.nTerm
    {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    if nPrefix as crate::src::ext::rtree::rtree::I64_0
        + nSuffix as crate::src::ext::rtree::rtree::I64_0
        > __pReader_ref.nTermAlloc as crate::src::ext::rtree::rtree::I64_0
    {
        let nNew: crate::src::ext::rtree::rtree::I64_0 = (nPrefix
            as crate::src::ext::rtree::rtree::I64_0
            + nSuffix as crate::src::ext::rtree::rtree::I64_0)
            * 2 as crate::src::ext::rtree::rtree::I64_0;
        let zNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
            __pReader_ref.zTerm as *mut ::core::ffi::c_void,
            nNew as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut ::core::ffi::c_char;
        if zNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pReader_ref.zTerm = zNew;
        __pReader_ref.nTermAlloc = nNew as ::core::ffi::c_int;
    }
    rc = fts3SegReaderRequire(pReader, pNext, nSuffix + crate::fts3Int_h::FTS3_VARINT_MAX);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    ::core::ptr::copy_nonoverlapping(
        pNext as *const u8,
        __pReader_ref.zTerm.offset(nPrefix as isize) as *mut ::core::ffi::c_char as *mut u8,
        nSuffix as usize,
    );
    __pReader_ref.nTerm = nPrefix + nSuffix;
    pNext = pNext.offset(nSuffix as isize);
    pNext = pNext.offset(
        (if *(pNext as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
        {
            crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(
                pNext,
                &raw mut __pReader_ref.nDoclist,
            )
        } else {
            __pReader_ref.nDoclist =
                *(pNext as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        }) as isize,
    );
    __pReader_ref.aDoclist = pNext;
    __pReader_ref.pOffsetList = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if __pReader_ref.nDoclist as ::core::ffi::c_long
        > __pReader_ref.nNode as ::core::ffi::c_long
            - __pReader_ref.aDoclist.offset_from(__pReader_ref.aNode) as ::core::ffi::c_long
        || __pReader_ref.nPopulate == 0 as ::core::ffi::c_int
            && *(*pReader)
                .aDoclist
                .offset((__pReader_ref.nDoclist - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                != 0
        || __pReader_ref.nDoclist == 0 as ::core::ffi::c_int
    {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3SegReaderFirstDocid(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    pReader: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pTab).bDescIdx as ::core::ffi::c_int != 0 && !(*pReader).ppNextElem.is_null() {
        let mut bEof: crate::src::ext::rtree::rtree::U8_0 =
            0 as crate::src::ext::rtree::rtree::U8_0;
        let __pReader_ref = unsafe { &mut *pReader };
        __pReader_ref.iDocid = 0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        __pReader_ref.nOffsetList = 0 as ::core::ffi::c_int;
        crate::src::ext::fts3::fts3::sqlite3Fts3DoclistPrev(
            0 as ::core::ffi::c_int,
            __pReader_ref.aDoclist,
            __pReader_ref.nDoclist,
            &raw mut __pReader_ref.pOffsetList,
            &raw mut __pReader_ref.iDocid,
            &raw mut __pReader_ref.nOffsetList,
            &raw mut bEof,
        );
    } else {
        rc = fts3SegReaderRequire(
            pReader,
            (*pReader).aDoclist,
            crate::fts3Int_h::FTS3_VARINT_MAX,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let __pReader_ref = unsafe { &mut *pReader };
            let n: ::core::ffi::c_int = crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(
                __pReader_ref.aDoclist,
                &raw mut __pReader_ref.iDocid,
            );
            __pReader_ref.pOffsetList =
                __pReader_ref.aDoclist.offset(n as isize) as *mut ::core::ffi::c_char;
        }
    }
    rc
}

unsafe extern "C" fn fts3SegReaderNextDocid(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    pReader: *mut Fts3SegReader,
    ppOffsetList: *mut *mut ::core::ffi::c_char,
    pnOffsetList: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut p: *mut ::core::ffi::c_char = (*pReader).pOffsetList;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    if (*pTab).bDescIdx as ::core::ffi::c_int != 0 && !(*pReader).ppNextElem.is_null() {
        let mut bEof: crate::src::ext::rtree::rtree::U8_0 =
            0 as crate::src::ext::rtree::rtree::U8_0;
        let __pReader_ref = unsafe { &mut *pReader };
        if !ppOffsetList.is_null() {
            *ppOffsetList = __pReader_ref.pOffsetList;
            *pnOffsetList = __pReader_ref.nOffsetList - 1 as ::core::ffi::c_int;
        }
        crate::src::ext::fts3::fts3::sqlite3Fts3DoclistPrev(
            0 as ::core::ffi::c_int,
            __pReader_ref.aDoclist,
            __pReader_ref.nDoclist,
            &raw mut p,
            &raw mut __pReader_ref.iDocid,
            &raw mut __pReader_ref.nOffsetList,
            &raw mut bEof,
        );
        if bEof != 0 {
            __pReader_ref.pOffsetList = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            __pReader_ref.pOffsetList = p;
        }
    } else {
        let pEnd: *mut ::core::ffi::c_char =
            (*pReader).aDoclist.offset((*pReader).nDoclist as isize) as *mut ::core::ffi::c_char;
        loop {
            while *p as ::core::ffi::c_int | c as ::core::ffi::c_int != 0 {
                let fresh10 = p;
                p = p.offset(1);
                c = (*fresh10 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let __pReader_ref = unsafe { &mut *pReader };
            if __pReader_ref.pBlob.is_null()
                || p < __pReader_ref.aNode.offset(__pReader_ref.nPopulate as isize)
                    as *mut ::core::ffi::c_char
            {
                break;
            }
            rc = fts3SegReaderIncrRead(pReader);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
        }
        p = p.offset(1);
        if !ppOffsetList.is_null() {
            *ppOffsetList = (*pReader).pOffsetList;
            *pnOffsetList = (p.offset_from((*pReader).pOffsetList) as ::core::ffi::c_long
                - 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
        }
        while p < pEnd && *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            p = p.offset(1);
        }
        if p >= pEnd {
            (*pReader).pOffsetList = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            rc = fts3SegReaderRequire(pReader, p, crate::fts3Int_h::FTS3_VARINT_MAX);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let mut iDelta: crate::src::ext::rtree::rtree::U64_0 = 0;
                (*pReader).pOffsetList = p.offset(
                    crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintU(p, &raw mut iDelta) as isize,
                );
                if (*pTab).bDescIdx != 0 {
                    (*pReader).iDocid = ((*pReader).iDocid as crate::src::ext::rtree::rtree::U64_0)
                        .wrapping_sub(iDelta)
                        as crate::src::ext::rtree::rtree::I64_0
                        as crate::src::headers::sqlite3_h::Sqlite3Int64;
                } else {
                    (*pReader).iDocid = ((*pReader).iDocid as crate::src::ext::rtree::rtree::U64_0)
                        .wrapping_add(iDelta)
                        as crate::src::ext::rtree::rtree::I64_0
                        as crate::src::headers::sqlite3_h::Sqlite3Int64;
                }
            }
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3MsrOvfl(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pMsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    pnOvfl: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let p: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut nOvfl: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ii: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let pgsz: ::core::ffi::c_int = (*p).nPgsz;
    ii = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && ii < (*pMsr).nSegment {
        let pReader: *mut Fts3SegReader = *(*pMsr).apSegment.offset(ii as isize);
        if (*pReader).ppNextElem.is_null()
            && ((*pReader).rootOnly as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            let mut jj: crate::src::headers::sqlite3_h::Sqlite3Int64;
            jj = (*pReader).iStartBlock;
            while jj <= (*pReader).iLeafEndBlock {
                let mut nBlob: ::core::ffi::c_int = 0;
                rc = sqlite3Fts3ReadBlock(
                    p,
                    jj,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    &raw mut nBlob,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    break;
                }
                if nBlob + 35 as ::core::ffi::c_int > pgsz {
                    nOvfl += (nBlob + 34 as ::core::ffi::c_int) / pgsz;
                }
                jj += 1;
            }
        }
        ii += 1;
    }
    *pnOvfl = nOvfl;
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3SegReaderFree(pReader: *mut Fts3SegReader) {
    if !pReader.is_null() {
        let __pReader_ref = unsafe { &*pReader };
        crate::src::src::malloc::sqlite3_free(__pReader_ref.zTerm as *mut ::core::ffi::c_void);
        if (__pReader_ref.rootOnly as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
            crate::src::src::malloc::sqlite3_free(__pReader_ref.aNode as *mut ::core::ffi::c_void);
        }
        crate::src::src::vdbeblob::sqlite3_blob_close(__pReader_ref.pBlob);
    }
    crate::src::src::malloc::sqlite3_free(pReader as *mut ::core::ffi::c_void);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3SegReaderNew(
    iAge: ::core::ffi::c_int,
    bLookup: ::core::ffi::c_int,
    iStartLeaf: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iEndLeaf: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iEndBlock: crate::src::headers::sqlite3_h::Sqlite3Int64,
    zRoot: *const ::core::ffi::c_char,
    nRoot: ::core::ffi::c_int,
    ppReader: *mut *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    
    let mut nExtra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if iStartLeaf == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 {
        if iEndLeaf != 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 {
            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
        }
        nExtra = nRoot + FTS3_NODE_PADDING;
    }
    let pReader: *mut Fts3SegReader = crate::src::src::malloc::sqlite3_malloc64(
        (::core::mem::size_of::<Fts3SegReader>() as usize).wrapping_add(nExtra as usize)
            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut Fts3SegReader;
    if pReader.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    std::ptr::write_bytes(
        pReader as *mut ::core::ffi::c_void as *mut u8,
        0 as ::core::ffi::c_int as u8,
        ::core::mem::size_of::<Fts3SegReader>() as crate::__stddef_size_t_h::SizeT,
    );
    (*pReader).iIdx = iAge;
    (*pReader).bLookup = (bLookup != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as crate::src::ext::rtree::rtree::U8_0;
    (*pReader).iStartBlock = iStartLeaf;
    (*pReader).iLeafEndBlock = iEndLeaf;
    (*pReader).iEndBlock = iEndBlock;
    if nExtra != 0 {
        let __pReader_ref = unsafe { &mut *pReader };
        __pReader_ref.aNode =
            pReader.offset(1_isize) as *mut Fts3SegReader as *mut ::core::ffi::c_char;
        __pReader_ref.rootOnly = 1 as crate::src::ext::rtree::rtree::U8_0;
        __pReader_ref.nNode = nRoot;
        if nRoot != 0 {
            ::core::ptr::copy_nonoverlapping(
                zRoot as *const u8,
                __pReader_ref.aNode as *mut u8,
                nRoot as usize,
            );
        }
        std::ptr::write_bytes(
            __pReader_ref.aNode.offset(nRoot as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void as *mut u8,
            0 as ::core::ffi::c_int as u8,
            FTS3_NODE_PADDING as crate::__stddef_size_t_h::SizeT,
        );
    } else {
        (*pReader).iCurrentBlock = iStartLeaf - 1 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    }
    *ppReader = pReader;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3CompareElemByTerm(
    lhs: *const ::core::ffi::c_void,
    rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let z1: *mut ::core::ffi::c_char = (**(lhs
        as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem))
        .pKey as *mut ::core::ffi::c_char;
    let z2: *mut ::core::ffi::c_char = (**(rhs
        as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem))
        .pKey as *mut ::core::ffi::c_char;
    let n1: ::core::ffi::c_int =
        (**(lhs as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem)).nKey;
    let n2: ::core::ffi::c_int =
        (**(rhs as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem)).nKey;
    let n: ::core::ffi::c_int = if n1 < n2 { n1 } else { n2 };
    let mut c: ::core::ffi::c_int = fts3_memcmp_safe(
        z1 as *const ::core::ffi::c_void,
        z2 as *const ::core::ffi::c_void,
        n as usize,
    );
    if c == 0 as ::core::ffi::c_int {
        c = n1 - n2;
    }
    c
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3SegReaderPending(
    p: *mut crate::fts3Int_h::Fts3Table,
    iIndex: ::core::ffi::c_int,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    bPrefix: ::core::ffi::c_int,
    ppReader: *mut *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut pReader: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
    let mut pE: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem;
    let mut aElem: *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem =
        ::core::ptr::null_mut::<*mut crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    let mut nElem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    
    let pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash = &raw mut (*(*p).aIndex.offset(iIndex as isize)).hPending;
    if bPrefix != 0 {
        let mut nAlloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pE = (*pHash).first;
        while !pE.is_null() {
            let zKey: *mut ::core::ffi::c_char = (*pE).pKey as *mut ::core::ffi::c_char;
            let nKey: ::core::ffi::c_int = (*pE).nKey;
            if nTerm == 0 as ::core::ffi::c_int
                || nKey >= nTerm
                    && 0 as ::core::ffi::c_int
                        == fts3_memcmp_safe(
                            zKey as *const ::core::ffi::c_void,
                            zTerm as *const ::core::ffi::c_void,
                            nTerm as usize,
                        )
            {
                if nElem == nAlloc {
                    
                    nAlloc += 16 as ::core::ffi::c_int;
                    let aElem2: *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = crate::src::src::malloc::sqlite3_realloc64(
                        aElem as *mut ::core::ffi::c_void,
                        (nAlloc as usize).wrapping_mul(::core::mem::size_of::<
                            *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
                        >() as usize)
                            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                    )
                        as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem;
                    if aElem2.is_null() {
                        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                        nElem = 0 as ::core::ffi::c_int;
                        break;
                    } else {
                        aElem = aElem2;
                    }
                }
                let fresh16 = nElem;
                nElem += 1;
                let fresh17 = &mut *aElem.offset(fresh16 as isize);
                *fresh17 = pE;
            }
            pE = (*pE).next;
        }
        if nElem > 1 as ::core::ffi::c_int {
            let slice = std::slice::from_raw_parts_mut(
                aElem as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
                nElem as usize,
            );
            slice.sort_by(|a, b| {
                let cmp = fts3CompareElemByTerm(
                    a as *const _ as *const ::core::ffi::c_void,
                    b as *const _ as *const ::core::ffi::c_void,
                );
                if cmp < 0 {
                    std::cmp::Ordering::Less
                } else if cmp > 0 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
        }
    } else {
        pE = crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFindElem(
            pHash as *const crate::src::ext::fts3::fts3_hash::Fts3Hash,
            zTerm as *const ::core::ffi::c_void,
            nTerm,
        ) as *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem;
        if !pE.is_null() {
            aElem = &raw mut pE;
            nElem = 1 as ::core::ffi::c_int;
        }
    }
    if nElem > 0 as ::core::ffi::c_int {
        
        let nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 = (::core::mem::size_of::<Fts3SegReader>() as usize).wrapping_add(
            ((nElem + 1 as ::core::ffi::c_int) as usize).wrapping_mul(::core::mem::size_of::<
                *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
            >() as usize),
        ) as crate::src::headers::sqlite3_h::Sqlite3Int64;
        pReader = crate::src::src::malloc::sqlite3_malloc64(
            nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut Fts3SegReader;
        if pReader.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            std::ptr::write_bytes(
                pReader as *mut ::core::ffi::c_void as *mut u8,
                0 as ::core::ffi::c_int as u8,
                nByte as crate::__stddef_size_t_h::SizeT,
            );
            let __pReader_ref = unsafe { &mut *pReader };
            __pReader_ref.iIdx = 0x7fffffff as ::core::ffi::c_int;
            __pReader_ref.ppNextElem = pReader.offset(1_isize) as *mut Fts3SegReader
                as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem;
            ::core::ptr::copy_nonoverlapping(
                aElem as *const u8,
                __pReader_ref.ppNextElem as *mut u8,
                ((nElem as crate::__stddef_size_t_h::SizeT)
                    .wrapping_mul(::core::mem::size_of::<
                        *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
                    >() as crate::__stddef_size_t_h::SizeT)) as usize,
            );
        }
    }
    if bPrefix != 0 {
        crate::src::src::malloc::sqlite3_free(aElem as *mut ::core::ffi::c_void);
    }
    *ppReader = pReader;
    rc
}

unsafe extern "C" fn fts3SegReaderCmp(
    pLhs: *mut Fts3SegReader,
    pRhs: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    if !(*pLhs).aNode.is_null() && !(*pRhs).aNode.is_null() {
        let rc2: ::core::ffi::c_int = (*pLhs).nTerm - (*pRhs).nTerm;
        if rc2 < 0 as ::core::ffi::c_int {
            rc = fts3_memcmp_safe(
                (*pLhs).zTerm as *const ::core::ffi::c_void,
                (*pRhs).zTerm as *const ::core::ffi::c_void,
                (*pLhs).nTerm as usize,
            );
        } else {
            rc = fts3_memcmp_safe(
                (*pLhs).zTerm as *const ::core::ffi::c_void,
                (*pRhs).zTerm as *const ::core::ffi::c_void,
                (*pRhs).nTerm as usize,
            );
        }
        if rc == 0 as ::core::ffi::c_int {
            rc = rc2;
        }
    } else {
        rc = ((*pLhs).aNode == ::core::ptr::null_mut::<::core::ffi::c_char>())
            as ::core::ffi::c_int
            - ((*pRhs).aNode == ::core::ptr::null_mut::<::core::ffi::c_char>())
                as ::core::ffi::c_int;
    }
    if rc == 0 as ::core::ffi::c_int {
        rc = (*pRhs).iIdx - (*pLhs).iIdx;
    }
    rc
}

unsafe extern "C" fn fts3SegReaderDoclistCmp(
    pLhs: *mut Fts3SegReader,
    pRhs: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = ((*pLhs).pOffsetList
        == ::core::ptr::null_mut::<::core::ffi::c_char>())
        as ::core::ffi::c_int
        - ((*pRhs).pOffsetList == ::core::ptr::null_mut::<::core::ffi::c_char>())
            as ::core::ffi::c_int;
    if rc == 0 as ::core::ffi::c_int {
        if (*pLhs).iDocid == (*pRhs).iDocid {
            rc = (*pRhs).iIdx - (*pLhs).iIdx;
        } else {
            rc = if (*pLhs).iDocid > (*pRhs).iDocid {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            };
        }
    }
    rc
}

unsafe extern "C" fn fts3SegReaderDoclistCmpRev(
    pLhs: *mut Fts3SegReader,
    pRhs: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = ((*pLhs).pOffsetList
        == ::core::ptr::null_mut::<::core::ffi::c_char>())
        as ::core::ffi::c_int
        - ((*pRhs).pOffsetList == ::core::ptr::null_mut::<::core::ffi::c_char>())
            as ::core::ffi::c_int;
    if rc == 0 as ::core::ffi::c_int {
        if (*pLhs).iDocid == (*pRhs).iDocid {
            rc = (*pRhs).iIdx - (*pLhs).iIdx;
        } else {
            rc = if (*pLhs).iDocid < (*pRhs).iDocid {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            };
        }
    }
    rc
}

unsafe extern "C" fn fts3SegReaderTermCmp(
    pSeg: *mut Fts3SegReader,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*pSeg).aNode.is_null() {
        if (*pSeg).nTerm > nTerm {
            res = fts3_memcmp_safe(
                (*pSeg).zTerm as *const ::core::ffi::c_void,
                zTerm as *const ::core::ffi::c_void,
                nTerm as usize,
            );
        } else {
            res = fts3_memcmp_safe(
                (*pSeg).zTerm as *const ::core::ffi::c_void,
                zTerm as *const ::core::ffi::c_void,
                (*pSeg).nTerm as usize,
            );
        }
        if res == 0 as ::core::ffi::c_int {
            res = (*pSeg).nTerm - nTerm;
        }
    }
    res
}

unsafe extern "C" fn fts3SegReaderSort(
    apSegment: *mut *mut Fts3SegReader,
    nSegment: ::core::ffi::c_int,
    mut nSuspect: ::core::ffi::c_int,
    xCmp: Option<
        unsafe extern "C" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> ::core::ffi::c_int,
    >,
) {
    let mut i: ::core::ffi::c_int;
    if nSuspect == nSegment {
        nSuspect -= 1;
    }
    i = nSuspect - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int;
        j = i;
        while j < nSegment - 1 as ::core::ffi::c_int {
            
            if xCmp.expect("non-null function pointer")(
                *apSegment.offset(j as isize),
                *apSegment.offset((j + 1 as ::core::ffi::c_int) as isize),
            ) < 0 as ::core::ffi::c_int
            {
                break;
            }
            let pTmp: *mut Fts3SegReader = *apSegment.offset((j + 1 as ::core::ffi::c_int) as isize);
            let fresh7 = &mut *apSegment.offset((j + 1 as ::core::ffi::c_int) as isize);
            *fresh7 = *apSegment.offset(j as isize);
            let fresh8 = &mut *apSegment.offset(j as isize);
            *fresh8 = pTmp;
            j += 1;
        }
        i -= 1;
    }
}

unsafe extern "C" fn fts3WriteSegment(
    p: *mut crate::fts3Int_h::Fts3Table,
    iBlock: crate::src::headers::sqlite3_h::Sqlite3Int64,
    z: *mut ::core::ffi::c_char,
    n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SqlConstant::SqlInsertSegments as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iBlock);
        crate::src::src::vdbeapi::sqlite3_bind_blob(
            pStmt,
            2 as ::core::ffi::c_int,
            z as *const ::core::ffi::c_void,
            n,
            crate::src::headers::sqlite3_h::SQLITE_STATIC,
        );
        crate::src::src::vdbeapi::sqlite3_step(pStmt);
        rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
        crate::src::src::vdbeapi::sqlite3_bind_null(pStmt, 2 as ::core::ffi::c_int);
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3MaxLevel(
    p: *mut crate::fts3Int_h::Fts3Table,
    pnMax: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut mxLevel: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectMxlevel as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pStmt)
        {
            mxLevel = crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    }
    *pnMax = mxLevel;
    rc
}

unsafe extern "C" fn fts3WriteSegdir(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iIdx: ::core::ffi::c_int,
    iStartBlock: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iLeafEndBlock: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iEndBlock: crate::src::headers::sqlite3_h::Sqlite3Int64,
    nLeafData: crate::src::headers::sqlite3_h::Sqlite3Int64,
    zRoot: *mut ::core::ffi::c_char,
    nRoot: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SqlConstant::SqlInsertSegdir as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iLevel);
        crate::src::src::vdbeapi::sqlite3_bind_int(pStmt, 2 as ::core::ffi::c_int, iIdx);
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 3 as ::core::ffi::c_int, iStartBlock);
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 4 as ::core::ffi::c_int, iLeafEndBlock);
        if nLeafData == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 {
            crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 5 as ::core::ffi::c_int, iEndBlock);
        } else {
            let result = format!("{} {}", iEndBlock, nLeafData);
            let bytes = result.into_bytes();
            let len = bytes.len();
            let ptr =
                unsafe { crate::src::src::malloc::sqlite3_malloc64((len + 1) as u64) } as *mut u8;
            if ptr.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);
                *ptr.add(len) = 0; // null terminate
            }
            let zEnd: *mut ::core::ffi::c_char = ptr as *mut ::core::ffi::c_char;
            crate::src::src::vdbeapi::sqlite3_bind_text(
                pStmt,
                5 as ::core::ffi::c_int,
                zEnd,
                -(1 as ::core::ffi::c_int),
                Some(
                    crate::src::src::malloc::sqlite3_free
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        }
        crate::src::src::vdbeapi::sqlite3_bind_blob(
            pStmt,
            6 as ::core::ffi::c_int,
            zRoot as *const ::core::ffi::c_void,
            nRoot,
            crate::src::headers::sqlite3_h::SQLITE_STATIC,
        );
        crate::src::src::vdbeapi::sqlite3_step(pStmt);
        rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
        crate::src::src::vdbeapi::sqlite3_bind_null(pStmt, 6 as ::core::ffi::c_int);
    }
    rc
}

unsafe extern "C" fn fts3PrefixCompress(
    zPrev: *const ::core::ffi::c_char,
    nPrev: ::core::ffi::c_int,
    zNext: *const ::core::ffi::c_char,
    nNext: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int;
    n = 0 as ::core::ffi::c_int;
    while n < nPrev
        && n < nNext
        && *zPrev.offset(n as isize) as ::core::ffi::c_int
            == *zNext.offset(n as isize) as ::core::ffi::c_int
    {
        n += 1;
    }
    n
}

unsafe extern "C" fn fts3NodeAddTerm(
    p: *mut crate::fts3Int_h::Fts3Table,
    ppTree: *mut *mut SegmentNode,
    isCopyTerm: ::core::ffi::c_int,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pTree: *mut SegmentNode = *ppTree;
    let rc: ::core::ffi::c_int;
    let mut pNew: *mut SegmentNode;
    if !pTree.is_null() {
        let __pTree_ref = unsafe { &mut *pTree };
        let mut nData: ::core::ffi::c_int = __pTree_ref.nData;
        let mut nReq: ::core::ffi::c_int = nData;
        
        
        let nPrefix: ::core::ffi::c_int = fts3PrefixCompress(__pTree_ref.zTerm, __pTree_ref.nTerm, zTerm, nTerm);
        let nSuffix: ::core::ffi::c_int = nTerm - nPrefix;
        if nSuffix <= 0 as ::core::ffi::c_int {
            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
        }
        nReq += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
            nPrefix as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) + crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
            nSuffix as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) + nSuffix;
        if nReq <= (*p).nNodeSize || __pTree_ref.zTerm.is_null() {
            if nReq > (*p).nNodeSize {
                __pTree_ref.aData = crate::src::src::malloc::sqlite3_malloc64(
                    nReq as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                ) as *mut ::core::ffi::c_char;
                if __pTree_ref.aData.is_null() {
                    return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                }
            }
            if !__pTree_ref.zTerm.is_null() {
                nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                    __pTree_ref.aData.offset(nData as isize) as *mut ::core::ffi::c_char,
                    nPrefix as crate::src::headers::sqlite3_h::Sqlite3Int64,
                );
            }
            nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                __pTree_ref.aData.offset(nData as isize) as *mut ::core::ffi::c_char,
                nSuffix as crate::src::headers::sqlite3_h::Sqlite3Int64,
            );
            ::core::ptr::copy_nonoverlapping(
                zTerm.offset(nPrefix as isize) as *const ::core::ffi::c_char,
                __pTree_ref.aData.offset(nData as isize) as *mut ::core::ffi::c_char,
                nSuffix as usize,
            );
            __pTree_ref.nData = nData + nSuffix;
            __pTree_ref.nEntry += 1;
            if isCopyTerm != 0 {
                if __pTree_ref.nMalloc < nTerm {
                    let zNew: *mut ::core::ffi::c_char =
                        crate::src::src::malloc::sqlite3_realloc64(
                            __pTree_ref.zMalloc as *mut ::core::ffi::c_void,
                            (nTerm as crate::src::ext::rtree::rtree::I64_0
                                * 2 as crate::src::ext::rtree::rtree::I64_0)
                                as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                        ) as *mut ::core::ffi::c_char;
                    if zNew.is_null() {
                        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    }
                    __pTree_ref.nMalloc = nTerm * 2 as ::core::ffi::c_int;
                    __pTree_ref.zMalloc = zNew;
                }
                __pTree_ref.zTerm = __pTree_ref.zMalloc;
                ::core::ptr::copy_nonoverlapping(
                    zTerm as *const u8,
                    __pTree_ref.zTerm as *mut u8,
                    nTerm as usize,
                );
                __pTree_ref.nTerm = nTerm;
            } else {
                __pTree_ref.zTerm = zTerm as *mut ::core::ffi::c_char;
                __pTree_ref.nTerm = nTerm;
            }
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    pNew = crate::src::src::malloc::sqlite3_malloc64(
        (::core::mem::size_of::<SegmentNode>() as usize).wrapping_add((*p).nNodeSize as usize)
            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut SegmentNode;
    if pNew.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    std::ptr::write_bytes(
        pNew as *mut ::core::ffi::c_void as *mut u8,
        0 as ::core::ffi::c_int as u8,
        ::core::mem::size_of::<SegmentNode>() as crate::__stddef_size_t_h::SizeT,
    );
    (*pNew).nData = 1 as ::core::ffi::c_int + crate::fts3Int_h::FTS3_VARINT_MAX;
    (*pNew).aData = pNew.offset(1_isize) as *mut SegmentNode as *mut ::core::ffi::c_char;
    if !pTree.is_null() {
        let __pTree_ref = unsafe { &mut *pTree };
        let mut pParent: *mut SegmentNode = __pTree_ref.pParent;
        rc = fts3NodeAddTerm(p, &raw mut pParent, isCopyTerm, zTerm, nTerm);
        if __pTree_ref.pParent.is_null() {
            __pTree_ref.pParent = pParent;
        }
        __pTree_ref.pRight = pNew;
        let __pNew_ref = unsafe { &mut *pNew };
        __pNew_ref.pLeftmost = __pTree_ref.pLeftmost;
        __pNew_ref.pParent = pParent;
        __pNew_ref.zMalloc = __pTree_ref.zMalloc;
        __pNew_ref.nMalloc = __pTree_ref.nMalloc;
        __pTree_ref.zMalloc = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        (*pNew).pLeftmost = pNew;
        rc = fts3NodeAddTerm(p, &raw mut pNew, isCopyTerm, zTerm, nTerm);
    }
    *ppTree = pNew;
    rc
}

unsafe extern "C" fn fts3TreeFinishNode(
    pTree: *mut SegmentNode,
    iHeight: ::core::ffi::c_int,
    iLeftChild: crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    
    let nStart: ::core::ffi::c_int = crate::fts3Int_h::FTS3_VARINT_MAX
        - crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
            iLeftChild as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        );
    *(*pTree).aData.offset(nStart as isize) = iHeight as ::core::ffi::c_char;
    crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
        (*pTree)
            .aData
            .offset((nStart + 1 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_char,
        iLeftChild,
    );
    nStart
}

unsafe extern "C" fn fts3NodeWrite(
    p: *mut crate::fts3Int_h::Fts3Table,
    pTree: *mut SegmentNode,
    iHeight: ::core::ffi::c_int,
    iLeaf: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iFree: crate::src::headers::sqlite3_h::Sqlite3Int64,
    piLast: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    paRoot: *mut *mut ::core::ffi::c_char,
    pnRoot: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pTree).pParent.is_null() {
        let nStart: ::core::ffi::c_int = fts3TreeFinishNode(pTree, iHeight, iLeaf);
        *piLast = iFree - 1 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        *pnRoot = (*pTree).nData - nStart;
        *paRoot = (*pTree).aData.offset(nStart as isize) as *mut ::core::ffi::c_char;
    } else {
        let mut pIter: *mut SegmentNode;
        let mut iNextFree: crate::src::headers::sqlite3_h::Sqlite3Int64 = iFree;
        let mut iNextLeaf: crate::src::headers::sqlite3_h::Sqlite3Int64 = iLeaf;
        pIter = (*pTree).pLeftmost;
        while !pIter.is_null() && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let nStart_0: ::core::ffi::c_int = fts3TreeFinishNode(pIter, iHeight, iNextLeaf);
            let nWrite: ::core::ffi::c_int = (*pIter).nData - nStart_0;
            rc = fts3WriteSegment(
                p,
                iNextFree,
                (*pIter).aData.offset(nStart_0 as isize) as *mut ::core::ffi::c_char,
                nWrite,
            );
            iNextFree += 1;
            iNextLeaf += ((*pIter).nEntry + 1 as ::core::ffi::c_int)
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
            pIter = (*pIter).pRight;
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = fts3NodeWrite(
                p,
                (*pTree).pParent,
                iHeight + 1 as ::core::ffi::c_int,
                iFree,
                iNextFree,
                piLast,
                paRoot,
                pnRoot,
            );
        }
    }
    rc
}

unsafe extern "C" fn fts3NodeFree(pTree: *mut SegmentNode) {
    if !pTree.is_null() {
        let mut p: *mut SegmentNode = (*pTree).pLeftmost;
        fts3NodeFree((*p).pParent);
        while !p.is_null() {
            let pRight: *mut SegmentNode = (*p).pRight;
            if (*p).aData != p.offset(1_isize) as *mut SegmentNode as *mut ::core::ffi::c_char {
                crate::src::src::malloc::sqlite3_free((*p).aData as *mut ::core::ffi::c_void);
            }
            crate::src::src::malloc::sqlite3_free((*p).zMalloc as *mut ::core::ffi::c_void);
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
            p = pRight;
        }
    }
}

unsafe extern "C" fn fts3SegWriterAdd(
    p: *mut crate::fts3Int_h::Fts3Table,
    ppWriter: *mut *mut SegmentWriter,
    isCopyTerm: ::core::ffi::c_int,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    aDoclist: *const ::core::ffi::c_char,
    nDoclist: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nPrefix: ::core::ffi::c_int;
    let mut nSuffix: ::core::ffi::c_int;
    let mut nReq: crate::src::ext::rtree::rtree::I64_0;
    let mut nData: ::core::ffi::c_int;
    let mut pWriter: *mut SegmentWriter = *ppWriter;
    if pWriter.is_null() {
        let mut rc: ::core::ffi::c_int;
        let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        pWriter = crate::src::src::malloc::sqlite3_malloc64(::core::mem::size_of::<SegmentWriter>()
            as crate::src::headers::sqlite3_h::Sqlite3Uint64)
            as *mut SegmentWriter;
        if pWriter.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        std::ptr::write_bytes(
            pWriter as *mut ::core::ffi::c_void as *mut u8,
            0 as ::core::ffi::c_int as u8,
            ::core::mem::size_of::<SegmentWriter>() as crate::__stddef_size_t_h::SizeT,
        );
        *ppWriter = pWriter;
        (*pWriter).aData = crate::src::src::malloc::sqlite3_malloc64(
            (*p).nNodeSize as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut ::core::ffi::c_char;
        if (*pWriter).aData.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        (*pWriter).nSize = (*p).nNodeSize;
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlNextSegmentsId as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        if crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pStmt)
        {
            let __pWriter_ref = unsafe { &mut *pWriter };
            __pWriter_ref.iFree =
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int);
            __pWriter_ref.iFirst = __pWriter_ref.iFree;
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
    }
    nData = (*pWriter).nData;
    nPrefix = fts3PrefixCompress((*pWriter).zTerm, (*pWriter).nTerm, zTerm, nTerm);
    nSuffix = nTerm - nPrefix;
    if nSuffix <= 0 as ::core::ffi::c_int {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    nReq = (crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
        nPrefix as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) + crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
        nSuffix as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) + nSuffix
        + crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
            nDoclist as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        )
        + nDoclist) as crate::src::ext::rtree::rtree::I64_0;
    if nData > 0 as ::core::ffi::c_int
        && nData as crate::src::ext::rtree::rtree::I64_0 + nReq
            > (*p).nNodeSize as crate::src::ext::rtree::rtree::I64_0
    {
        let mut rc_0: ::core::ffi::c_int;
        let __pWriter_ref = unsafe { &mut *pWriter };
        if __pWriter_ref.iFree == crate::fts3Int_h::LARGEST_INT64 {
            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
        }
        let fresh5 = __pWriter_ref.iFree;
        __pWriter_ref.iFree += 1;
        rc_0 = fts3WriteSegment(p, fresh5, __pWriter_ref.aData, nData);
        if rc_0 != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc_0;
        }
        (*p).nLeafAdd = (*p).nLeafAdd.wrapping_add(1);
        rc_0 = fts3NodeAddTerm(
            p,
            &raw mut __pWriter_ref.pTree,
            isCopyTerm,
            zTerm,
            nPrefix + 1 as ::core::ffi::c_int,
        );
        if rc_0 != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc_0;
        }
        nData = 0 as ::core::ffi::c_int;
        __pWriter_ref.nTerm = 0 as ::core::ffi::c_int;
        nPrefix = 0 as ::core::ffi::c_int;
        nSuffix = nTerm;
        nReq = (1 as ::core::ffi::c_int
            + crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
                nTerm as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            )
            + nTerm
            + crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
                nDoclist as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            )
            + nDoclist) as crate::src::ext::rtree::rtree::I64_0;
    }
    (*pWriter).nLeafData += nReq;
    if nReq > (*pWriter).nSize as crate::src::ext::rtree::rtree::I64_0 {
        let __pWriter_ref = unsafe { &mut *pWriter };
        let aNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
            __pWriter_ref.aData as *mut ::core::ffi::c_void,
            nReq as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut ::core::ffi::c_char;
        if aNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pWriter_ref.aData = aNew;
        __pWriter_ref.nSize = nReq as ::core::ffi::c_int;
    }
    nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
        nPrefix as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
        nSuffix as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    ::core::ptr::copy_nonoverlapping(
        zTerm.offset(nPrefix as isize) as *const ::core::ffi::c_char,
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
        nSuffix as usize,
    );
    nData += nSuffix;
    nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
        nDoclist as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    ::core::ptr::copy_nonoverlapping(
        aDoclist as *const u8,
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char as *mut u8,
        nDoclist as usize,
    );
    (*pWriter).nData = nData + nDoclist;
    if isCopyTerm != 0 {
        if nTerm > (*pWriter).nMalloc {
            let __pWriter_ref = unsafe { &mut *pWriter };
            let zNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
                __pWriter_ref.zMalloc as *mut ::core::ffi::c_void,
                (nTerm as crate::src::ext::rtree::rtree::I64_0
                    * 2 as crate::src::ext::rtree::rtree::I64_0)
                    as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ) as *mut ::core::ffi::c_char;
            if zNew.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            __pWriter_ref.nMalloc = nTerm * 2 as ::core::ffi::c_int;
            __pWriter_ref.zMalloc = zNew;
            __pWriter_ref.zTerm = zNew;
        }
        ::core::ptr::copy_nonoverlapping(
            zTerm as *const u8,
            (*pWriter).zTerm as *mut u8,
            nTerm as usize,
        );
    } else {
        (*pWriter).zTerm = zTerm as *mut ::core::ffi::c_char;
    }
    (*pWriter).nTerm = nTerm;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3SegWriterFlush(
    p: *mut crate::fts3Int_h::Fts3Table,
    pWriter: *mut SegmentWriter,
    iLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iIdx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    if !(*pWriter).pTree.is_null() {
        let mut iLast: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        
        let mut zRoot: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nRoot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let __pWriter_ref = unsafe { &mut *pWriter };
        let iLastLeaf: crate::src::headers::sqlite3_h::Sqlite3Int64 = __pWriter_ref.iFree;
        let fresh4 = __pWriter_ref.iFree;
        __pWriter_ref.iFree += 1;
        rc = fts3WriteSegment(p, fresh4, __pWriter_ref.aData, __pWriter_ref.nData);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = fts3NodeWrite(
                p,
                __pWriter_ref.pTree,
                1 as ::core::ffi::c_int,
                __pWriter_ref.iFirst,
                __pWriter_ref.iFree,
                &raw mut iLast,
                &raw mut zRoot,
                &raw mut nRoot,
            );
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = fts3WriteSegdir(
                p,
                iLevel,
                iIdx,
                __pWriter_ref.iFirst,
                iLastLeaf,
                iLast,
                __pWriter_ref.nLeafData as crate::src::headers::sqlite3_h::Sqlite3Int64,
                zRoot,
                nRoot,
            );
        }
    } else {
        let __pWriter_ref = unsafe { &*pWriter };
        rc = fts3WriteSegdir(
            p,
            iLevel,
            iIdx,
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64,
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64,
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64,
            __pWriter_ref.nLeafData as crate::src::headers::sqlite3_h::Sqlite3Int64,
            __pWriter_ref.aData,
            __pWriter_ref.nData,
        );
    }
    (*p).nLeafAdd = (*p).nLeafAdd.wrapping_add(1);
    rc
}

unsafe extern "C" fn fts3SegWriterFree(pWriter: *mut SegmentWriter) {
    if !pWriter.is_null() {
        let __pWriter_ref = unsafe { &*pWriter };
        crate::src::src::malloc::sqlite3_free(__pWriter_ref.aData as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(__pWriter_ref.zMalloc as *mut ::core::ffi::c_void);
        fts3NodeFree(__pWriter_ref.pTree);
        crate::src::src::malloc::sqlite3_free(pWriter as *mut ::core::ffi::c_void);
    }
}

unsafe extern "C" fn fts3IsEmpty(
    p: *mut crate::fts3Int_h::Fts3Table,
    mut pRowid: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pisEmpty: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int;
    if !(*p).zContentTbl.is_null() {
        *pisEmpty = 0 as ::core::ffi::c_int;
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    } else {
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlIsEmpty as ::core::ffi::c_int,
            &raw mut pStmt,
            &raw mut pRowid,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if crate::src::headers::sqlite3_h::SQLITE_ROW
                == crate::src::src::vdbeapi::sqlite3_step(pStmt)
            {
                *pisEmpty =
                    crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
            }
            rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
        }
    }
    rc
}

unsafe extern "C" fn fts3SegmentMaxLevel(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    iIndex: ::core::ffi::c_int,
    pnMax: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    
    let rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectSegdirMaxLevel as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        pStmt,
        1 as ::core::ffi::c_int,
        getAbsoluteLevel(p, iLangid, iIndex, 0 as ::core::ffi::c_int),
    );
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        pStmt,
        2 as ::core::ffi::c_int,
        getAbsoluteLevel(
            p,
            iLangid,
            iIndex,
            crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL - 1 as ::core::ffi::c_int,
        ),
    );
    if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pStmt) {
        *pnMax = crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int);
    }
    crate::src::src::vdbeapi::sqlite3_reset(pStmt)
}

unsafe extern "C" fn fts3SegmentIsMaxLevel(
    p: *mut crate::fts3Int_h::Fts3Table,
    iAbsLevel: crate::src::ext::rtree::rtree::I64_0,
    pbMax: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectSegdirMaxLevel as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        pStmt,
        1 as ::core::ffi::c_int,
        iAbsLevel as crate::src::headers::sqlite3_h::Sqlite3Int64
            + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        pStmt,
        2 as ::core::ffi::c_int,
        (iAbsLevel as crate::src::ext::rtree::rtree::U64_0)
            .wrapping_div(
                crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL as crate::src::ext::rtree::rtree::U64_0,
            )
            .wrapping_add(1 as crate::src::ext::rtree::rtree::U64_0)
            .wrapping_mul(
                crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL as crate::src::ext::rtree::rtree::U64_0,
            ) as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    *pbMax = 0 as ::core::ffi::c_int;
    if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pStmt) {
        *pbMax = (crate::src::src::vdbeapi::sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int)
            == crate::src::headers::sqlite3_h::SQLITE_NULL) as ::core::ffi::c_int;
    }
    crate::src::src::vdbeapi::sqlite3_reset(pStmt)
}

unsafe extern "C" fn fts3DeleteSegment(
    p: *mut crate::fts3Int_h::Fts3Table,
    pSeg: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pSeg).iStartBlock != 0 {
        let mut pDelete: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlDeleteSegmentsRange as ::core::ffi::c_int,
            &raw mut pDelete,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pDelete,
                1 as ::core::ffi::c_int,
                (*pSeg).iStartBlock,
            );
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pDelete,
                2 as ::core::ffi::c_int,
                (*pSeg).iEndBlock,
            );
            crate::src::src::vdbeapi::sqlite3_step(pDelete);
            rc = crate::src::src::vdbeapi::sqlite3_reset(pDelete);
        }
    }
    rc
}

unsafe extern "C" fn fts3DeleteSegdir(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    iIndex: ::core::ffi::c_int,
    iLevel: ::core::ffi::c_int,
    apSegment: *mut *mut Fts3SegReader,
    nReader: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i: ::core::ffi::c_int;
    let mut pDelete: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    i = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < nReader {
        rc = fts3DeleteSegment(p, *apSegment.offset(i as isize));
        i += 1;
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    if iLevel == crate::fts3Int_h::FTS3_SEGCURSOR_ALL {
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlDeleteSegdirRange as ::core::ffi::c_int,
            &raw mut pDelete,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pDelete,
                1 as ::core::ffi::c_int,
                getAbsoluteLevel(p, iLangid, iIndex, 0 as ::core::ffi::c_int),
            );
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pDelete,
                2 as ::core::ffi::c_int,
                getAbsoluteLevel(
                    p,
                    iLangid,
                    iIndex,
                    crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL - 1 as ::core::ffi::c_int,
                ),
            );
        }
    } else {
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlDeleteSegdirLevel as ::core::ffi::c_int,
            &raw mut pDelete,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pDelete,
                1 as ::core::ffi::c_int,
                getAbsoluteLevel(p, iLangid, iIndex, iLevel),
            );
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_step(pDelete);
        rc = crate::src::src::vdbeapi::sqlite3_reset(pDelete);
    }
    rc
}

unsafe extern "C" fn fts3ColumnFilter(
    iCol: ::core::ffi::c_int,
    bZero: ::core::ffi::c_int,
    ppList: *mut *mut ::core::ffi::c_char,
    pnList: *mut ::core::ffi::c_int,
) {
    let mut pList: *mut ::core::ffi::c_char = *ppList;
    let mut nList: ::core::ffi::c_int = *pnList;
    let pEnd: *mut ::core::ffi::c_char =
        pList.offset(nList as isize) as *mut ::core::ffi::c_char;
    let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char = pList;
    loop {
        let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
        while p < pEnd
            && (c as ::core::ffi::c_int | *p as ::core::ffi::c_int) & 0xfe as ::core::ffi::c_int
                != 0
        {
            let fresh9 = p;
            p = p.offset(1);
            c = (*fresh9 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        }
        if iCol == iCurrent {
            nList = p.offset_from(pList) as ::core::ffi::c_long as ::core::ffi::c_int;
            break;
        } else {
            nList -= p.offset_from(pList) as ::core::ffi::c_long as ::core::ffi::c_int;
            pList = p;
            if nList <= 0 as ::core::ffi::c_int {
                break;
            }
            p = pList.offset(1_isize) as *mut ::core::ffi::c_char;
            p = p.offset(
                (if *(p as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    != 0
                {
                    crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(p, &raw mut iCurrent)
                } else {
                    iCurrent =
                        *(p as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
        }
    }
    if bZero != 0
        && pEnd.offset_from(pList.offset(nList as isize) as *mut ::core::ffi::c_char)
            as ::core::ffi::c_long
            > 0 as ::core::ffi::c_long
    {
        std::ptr::write_bytes(
            pList.offset(nList as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void
                as *mut u8,
            0 as ::core::ffi::c_int as u8,
            pEnd.offset_from(pList.offset(nList as isize) as *mut ::core::ffi::c_char)
                as ::core::ffi::c_long as crate::__stddef_size_t_h::SizeT,
        );
    }
    *ppList = pList;
    *pnList = nList;
}

unsafe extern "C" fn fts3MsrBufferData(
    pMsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    pList: *mut ::core::ffi::c_char,
    nList: crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    let __pMsr_ref = unsafe { &mut *pMsr };
    if nList + FTS3_NODE_PADDING as crate::src::ext::rtree::rtree::I64_0 > __pMsr_ref.nBuffer {
        
        let nNew: ::core::ffi::c_int = (nList * 2 as crate::src::ext::rtree::rtree::I64_0
            + FTS3_NODE_PADDING as crate::src::ext::rtree::rtree::I64_0)
            as ::core::ffi::c_int;
        let pNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
            __pMsr_ref.aBuffer as *mut ::core::ffi::c_void,
            nNew as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut ::core::ffi::c_char;
        if pNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pMsr_ref.aBuffer = pNew;
        __pMsr_ref.nBuffer = nNew as crate::src::ext::rtree::rtree::I64_0;
    }
    ::core::ptr::copy_nonoverlapping(
        pList as *const u8,
        __pMsr_ref.aBuffer as *mut u8,
        nList as usize,
    );
    std::ptr::write_bytes(
        __pMsr_ref.aBuffer.offset(nList as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void as *mut u8,
        0 as ::core::ffi::c_int as u8,
        FTS3_NODE_PADDING as crate::__stddef_size_t_h::SizeT,
    );
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3MsrIncrNext(
    p: *mut crate::fts3Int_h::Fts3Table,
    pMsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    piDocid: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    paPoslist: *mut *mut ::core::ffi::c_char,
    pnPoslist: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let nMerge: ::core::ffi::c_int = (*pMsr).nAdvance;
    let apSegment: *mut *mut Fts3SegReader = (*pMsr).apSegment;
    let xCmp: Option<
        unsafe extern "C" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> ::core::ffi::c_int,
    > = if (*p).bDescIdx as ::core::ffi::c_int != 0 {
        Some(
            fts3SegReaderDoclistCmpRev
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            fts3SegReaderDoclistCmp
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    };
    if nMerge == 0 as ::core::ffi::c_int {
        *paPoslist = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    loop {
        
        let pSeg: *mut Fts3SegReader = *(*pMsr).apSegment.offset(0_isize);
        if (*pSeg).pOffsetList.is_null() {
            *paPoslist = ::core::ptr::null_mut::<::core::ffi::c_char>();
            break;
        } else {
            let mut rc: ::core::ffi::c_int;
            let mut pList: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut nList: ::core::ffi::c_int = 0;
            let mut j: ::core::ffi::c_int;
            let iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                (**apSegment.offset(0_isize)).iDocid;
            rc = fts3SegReaderNextDocid(
                p,
                *apSegment.offset(0_isize),
                &raw mut pList,
                &raw mut nList,
            );
            j = 1 as ::core::ffi::c_int;
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK
                && j < nMerge
                && !(**apSegment.offset(j as isize)).pOffsetList.is_null()
                && (**apSegment.offset(j as isize)).iDocid == iDocid
            {
                rc = fts3SegReaderNextDocid(
                    p,
                    *apSegment.offset(j as isize),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                j += 1;
            }
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            fts3SegReaderSort((*pMsr).apSegment, nMerge, j, xCmp);
            if nList > 0 as ::core::ffi::c_int
                && !(**apSegment.offset(0_isize)).ppNextElem.is_null()
            {
                rc = fts3MsrBufferData(
                    pMsr,
                    pList,
                    nList as crate::src::ext::rtree::rtree::I64_0
                        + 1 as crate::src::ext::rtree::rtree::I64_0,
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    return rc;
                }
                pList = (*pMsr).aBuffer;
            }
            if (*pMsr).iColFilter >= 0 as ::core::ffi::c_int {
                fts3ColumnFilter(
                    (*pMsr).iColFilter,
                    1 as ::core::ffi::c_int,
                    &raw mut pList,
                    &raw mut nList,
                );
            }
            if (nList <= 0 as ::core::ffi::c_int) {
                continue;
            }
            *paPoslist = pList;
            *piDocid = iDocid;
            *pnPoslist = nList;
            break;
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3SegReaderStart(
    p: *mut crate::fts3Int_h::Fts3Table,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let __pCsr_ref = unsafe { &mut *pCsr };
    let nSeg: ::core::ffi::c_int = __pCsr_ref.nSegment;
    i = 0 as ::core::ffi::c_int;
    while __pCsr_ref.bRestart == 0 as ::core::ffi::c_int && i < __pCsr_ref.nSegment {
        let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let pSeg: *mut Fts3SegReader = *__pCsr_ref.apSegment.offset(i as isize);
        loop {
            let rc: ::core::ffi::c_int = fts3SegReaderNext(p, pSeg, 0 as ::core::ffi::c_int);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            if !(!zTerm.is_null() && {
                res = fts3SegReaderTermCmp(pSeg, zTerm, nTerm);
                res < 0 as ::core::ffi::c_int
            }) {
                break;
            }
        }
        if (*pSeg).bLookup as ::core::ffi::c_int != 0 && res != 0 as ::core::ffi::c_int {
            fts3SegReaderSetEof(pSeg);
        }
        i += 1;
    }
    fts3SegReaderSort(
        __pCsr_ref.apSegment,
        nSeg,
        nSeg,
        Some(
            fts3SegReaderCmp
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        ),
    );
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3SegReaderStart(
    p: *mut crate::fts3Int_h::Fts3Table,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    pFilter: *mut crate::fts3Int_h::Fts3SegFilter,
) -> ::core::ffi::c_int {
    (*pCsr).pFilter = pFilter;
    fts3SegReaderStart(p, pCsr, (*pFilter).zTerm, (*pFilter).nTerm)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3MsrIncrStart(
    p: *mut crate::fts3Int_h::Fts3Table,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    iCol: ::core::ffi::c_int,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    let __pCsr_ref = unsafe { &mut *pCsr };
    let nSegment: ::core::ffi::c_int = __pCsr_ref.nSegment;
    let xCmp: Option<
        unsafe extern "C" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> ::core::ffi::c_int,
    > = if (*p).bDescIdx as ::core::ffi::c_int != 0 {
        Some(
            fts3SegReaderDoclistCmpRev
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            fts3SegReaderDoclistCmp
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    };
    rc = fts3SegReaderStart(p, pCsr, zTerm, nTerm);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nSegment {
        let pSeg: *mut Fts3SegReader = *__pCsr_ref.apSegment.offset(i as isize);
        if (*pSeg).aNode.is_null() || fts3SegReaderTermCmp(pSeg, zTerm, nTerm) != 0 {
            break;
        }
        i += 1;
    }
    __pCsr_ref.nAdvance = i;
    i = 0 as ::core::ffi::c_int;
    while i < __pCsr_ref.nAdvance {
        rc = fts3SegReaderFirstDocid(p, *__pCsr_ref.apSegment.offset(i as isize));
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        i += 1;
    }
    fts3SegReaderSort(__pCsr_ref.apSegment, i, i, xCmp);
    __pCsr_ref.iColFilter = iCol;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3MsrIncrRestart(
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let __pCsr_ref = unsafe { &mut *pCsr };
    __pCsr_ref.nAdvance = 0 as ::core::ffi::c_int;
    __pCsr_ref.bRestart = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < __pCsr_ref.nSegment {
        let fresh18 = &mut (**__pCsr_ref.apSegment.offset(i as isize)).pOffsetList;
        *fresh18 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (**__pCsr_ref.apSegment.offset(i as isize)).nOffsetList = 0 as ::core::ffi::c_int;
        (**__pCsr_ref.apSegment.offset(i as isize)).iDocid =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        i += 1;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3GrowSegReaderBuffer(
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    nReq: crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    if nReq > (*pCsr).nBuffer {
        
        let __pCsr_ref = unsafe { &mut *pCsr };
        __pCsr_ref.nBuffer = nReq * 2 as crate::src::ext::rtree::rtree::I64_0;
        let aNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
            __pCsr_ref.aBuffer as *mut ::core::ffi::c_void,
            __pCsr_ref.nBuffer as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut ::core::ffi::c_char;
        if aNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pCsr_ref.aBuffer = aNew;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3SegReaderStep(
    p: *mut crate::fts3Int_h::Fts3Table,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pCsr_ref = unsafe { &mut *pCsr };
    let isIgnoreEmpty: ::core::ffi::c_int =
        (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY;
    let isRequirePos: ::core::ffi::c_int =
        (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS;
    let isColFilter: ::core::ffi::c_int =
        (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_COLUMN_FILTER;
    let isPrefix: ::core::ffi::c_int =
        (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_PREFIX;
    let isScan: ::core::ffi::c_int =
        (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_SCAN;
    let isFirst: ::core::ffi::c_int =
        (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_FIRST;
    let apSegment: *mut *mut Fts3SegReader = __pCsr_ref.apSegment;
    let nSegment: ::core::ffi::c_int = __pCsr_ref.nSegment;
    let pFilter: *mut crate::fts3Int_h::Fts3SegFilter = __pCsr_ref.pFilter;
    let xCmp: Option<
        unsafe extern "C" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> ::core::ffi::c_int,
    > = if (*p).bDescIdx as ::core::ffi::c_int != 0 {
        Some(
            fts3SegReaderDoclistCmpRev
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            fts3SegReaderDoclistCmp
                as unsafe extern "C" fn(
                    *mut Fts3SegReader,
                    *mut Fts3SegReader,
                ) -> ::core::ffi::c_int,
        )
    };
    if __pCsr_ref.nSegment == 0 as ::core::ffi::c_int {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    loop {
        let mut nMerge: ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < __pCsr_ref.nAdvance {
            let pSeg: *mut Fts3SegReader = *apSegment.offset(i as isize);
            if (*pSeg).bLookup != 0 {
                fts3SegReaderSetEof(pSeg);
            } else {
                rc = fts3SegReaderNext(p, pSeg, 0 as ::core::ffi::c_int);
            }
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            i += 1;
        }
        fts3SegReaderSort(
            apSegment,
            nSegment,
            __pCsr_ref.nAdvance,
            Some(
                fts3SegReaderCmp
                    as unsafe extern "C" fn(
                        *mut Fts3SegReader,
                        *mut Fts3SegReader,
                    ) -> ::core::ffi::c_int,
            ),
        );
        __pCsr_ref.nAdvance = 0 as ::core::ffi::c_int;
        if (**apSegment.offset(0_isize)).aNode.is_null() {
            break;
        }
        __pCsr_ref.nTerm = (**apSegment.offset(0_isize)).nTerm;
        __pCsr_ref.zTerm = (**apSegment.offset(0_isize)).zTerm;
        if !(*pFilter).zTerm.is_null() && isScan == 0 {
            let __pFilter_ref = unsafe { &*pFilter };
            if __pCsr_ref.nTerm < __pFilter_ref.nTerm
                || isPrefix == 0 && __pCsr_ref.nTerm > __pFilter_ref.nTerm
                || fts3_memcmp_safe(
                    __pCsr_ref.zTerm as *const ::core::ffi::c_void,
                    __pFilter_ref.zTerm as *const ::core::ffi::c_void,
                    __pFilter_ref.nTerm as usize,
                ) != 0
            {
                break;
            }
        }
        nMerge = 1 as ::core::ffi::c_int;
        while nMerge < nSegment
            && !(**apSegment.offset(nMerge as isize)).aNode.is_null()
            && (**apSegment.offset(nMerge as isize)).nTerm == __pCsr_ref.nTerm
            && 0 as ::core::ffi::c_int
                == fts3_memcmp_safe(
                    __pCsr_ref.zTerm as *const ::core::ffi::c_void,
                    (**apSegment.offset(nMerge as isize)).zTerm as *const ::core::ffi::c_void,
                    __pCsr_ref.nTerm as usize,
                )
        {
            nMerge += 1;
        }
        if nMerge == 1 as ::core::ffi::c_int
            && isIgnoreEmpty == 0
            && isFirst == 0
            && ((*p).bDescIdx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || ((**apSegment.offset(0_isize)).ppNextElem
                    != ::core::ptr::null_mut::<*mut crate::src::ext::fts3::fts3_hash::Fts3HashElem>(
                    )) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int)
        {
            __pCsr_ref.nDoclist = (**apSegment.offset(0_isize)).nDoclist;
            if !(**apSegment.offset(0_isize)).ppNextElem.is_null() {
                rc = fts3MsrBufferData(
                    pCsr,
                    (**apSegment.offset(0_isize)).aDoclist,
                    __pCsr_ref.nDoclist as crate::src::ext::rtree::rtree::I64_0,
                );
                __pCsr_ref.aDoclist = __pCsr_ref.aBuffer;
            } else {
                __pCsr_ref.aDoclist = (**apSegment.offset(0_isize)).aDoclist;
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::src::headers::sqlite3_h::SQLITE_ROW;
            }
        } else {
            let mut nDoclist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iPrev: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            i = 0 as ::core::ffi::c_int;
            while i < nMerge {
                fts3SegReaderFirstDocid(p, *apSegment.offset(i as isize));
                i += 1;
            }
            fts3SegReaderSort(apSegment, nMerge, nMerge, xCmp);
            while !(**apSegment.offset(0_isize)).pOffsetList.is_null() {
                let mut j: ::core::ffi::c_int;
                let mut pList: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut nList: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let nByte: ::core::ffi::c_int;
                let iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                    (**apSegment.offset(0_isize)).iDocid;
                fts3SegReaderNextDocid(
                    p,
                    *apSegment.offset(0_isize),
                    &raw mut pList,
                    &raw mut nList,
                );
                j = 1 as ::core::ffi::c_int;
                while j < nMerge
                    && !(**apSegment.offset(j as isize)).pOffsetList.is_null()
                    && (**apSegment.offset(j as isize)).iDocid == iDocid
                {
                    fts3SegReaderNextDocid(
                        p,
                        *apSegment.offset(j as isize),
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    );
                    j += 1;
                }
                if isColFilter != 0 {
                    fts3ColumnFilter(
                        (*pFilter).iCol,
                        0 as ::core::ffi::c_int,
                        &raw mut pList,
                        &raw mut nList,
                    );
                }
                if isIgnoreEmpty == 0 || nList > 0 as ::core::ffi::c_int {
                    let iDelta: crate::src::headers::sqlite3_h::Sqlite3Int64;
                    if (*p).bDescIdx as ::core::ffi::c_int != 0
                        && nDoclist > 0 as ::core::ffi::c_int
                    {
                        if iPrev <= iDocid {
                            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
                        }
                        iDelta = (iPrev as crate::src::ext::rtree::rtree::U64_0)
                            .wrapping_sub(iDocid as crate::src::ext::rtree::rtree::U64_0)
                            as crate::src::ext::rtree::rtree::I64_0
                            as crate::src::headers::sqlite3_h::Sqlite3Int64;
                    } else {
                        if nDoclist > 0 as ::core::ffi::c_int && iPrev >= iDocid {
                            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
                        }
                        iDelta = (iDocid as crate::src::ext::rtree::rtree::U64_0)
                            .wrapping_sub(iPrev as crate::src::ext::rtree::rtree::U64_0)
                            as crate::src::ext::rtree::rtree::I64_0
                            as crate::src::headers::sqlite3_h::Sqlite3Int64;
                    }
                    nByte = crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
                        iDelta as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                    ) + (if isRequirePos != 0 {
                        nList + 1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    });
                    rc = fts3GrowSegReaderBuffer(
                        pCsr,
                        nByte as crate::src::ext::rtree::rtree::I64_0
                            + nDoclist as crate::src::ext::rtree::rtree::I64_0
                            + FTS3_NODE_PADDING as crate::src::ext::rtree::rtree::I64_0,
                    );
                    if rc != 0 {
                        return rc;
                    }
                    if isFirst != 0 {
                        let a: *mut ::core::ffi::c_char =
                            __pCsr_ref.aBuffer.offset(nDoclist as isize)
                                as *mut ::core::ffi::c_char;
                        
                        let nWrite: ::core::ffi::c_int = crate::src::ext::fts3::fts3::sqlite3Fts3FirstFilter(
                            iDelta, pList, nList, a,
                        );
                        if nWrite != 0 {
                            iPrev = iDocid;
                            nDoclist += nWrite;
                        }
                    } else {
                        nDoclist += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                            __pCsr_ref.aBuffer.offset(nDoclist as isize)
                                as *mut ::core::ffi::c_char,
                            iDelta,
                        );
                        iPrev = iDocid;
                        if isRequirePos != 0 {
                            ::core::ptr::copy_nonoverlapping(
                                pList as *const u8,
                                __pCsr_ref.aBuffer.offset(nDoclist as isize)
                                    as *mut ::core::ffi::c_char
                                    as *mut u8,
                                nList as usize,
                            );
                            nDoclist += nList;
                            let fresh6 = nDoclist;
                            nDoclist += 1;
                            *__pCsr_ref.aBuffer.offset(fresh6 as isize) =
                                '\0' as i32 as ::core::ffi::c_char;
                        }
                    }
                }
                fts3SegReaderSort(apSegment, nMerge, j, xCmp);
            }
            if nDoclist > 0 as ::core::ffi::c_int {
                rc = fts3GrowSegReaderBuffer(
                    pCsr,
                    nDoclist as crate::src::ext::rtree::rtree::I64_0
                        + FTS3_NODE_PADDING as crate::src::ext::rtree::rtree::I64_0,
                );
                if rc != 0 {
                    return rc;
                }
                std::ptr::write_bytes(
                    __pCsr_ref.aBuffer.offset(nDoclist as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void as *mut u8,
                    0 as ::core::ffi::c_int as u8,
                    FTS3_NODE_PADDING as crate::__stddef_size_t_h::SizeT,
                );
                __pCsr_ref.aDoclist = __pCsr_ref.aBuffer;
                __pCsr_ref.nDoclist = nDoclist;
                rc = crate::src::headers::sqlite3_h::SQLITE_ROW;
            }
        }
        __pCsr_ref.nAdvance = nMerge;
        if (rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
            break;
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3SegReaderFinish(
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) {
    if !pCsr.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        let __pCsr_ref = unsafe { &mut *pCsr };
        while i < __pCsr_ref.nSegment {
            sqlite3Fts3SegReaderFree(*__pCsr_ref.apSegment.offset(i as isize));
            i += 1;
        }
        crate::src::src::malloc::sqlite3_free(__pCsr_ref.apSegment as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(__pCsr_ref.aBuffer as *mut ::core::ffi::c_void);
        __pCsr_ref.nSegment = 0 as ::core::ffi::c_int;
        __pCsr_ref.apSegment = ::core::ptr::null_mut::<*mut Fts3SegReader>();
        __pCsr_ref.aBuffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}

unsafe extern "C" fn fts3ReadEndBlockField(
    pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    iCol: ::core::ffi::c_int,
    piEndBlock: *mut crate::src::ext::rtree::rtree::I64_0,
    pnByte: *mut crate::src::ext::rtree::rtree::I64_0,
) {
    let zText: *const ::core::ffi::c_uchar =
        crate::src::src::vdbeapi::sqlite3_column_text(pStmt, iCol);
    if !zText.is_null() {
        let mut i: ::core::ffi::c_int;
        let mut iMul: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut iVal: crate::src::ext::rtree::rtree::U64_0 =
            0 as crate::src::ext::rtree::rtree::U64_0;
        i = 0 as ::core::ffi::c_int;
        while *zText.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
            && *zText.offset(i as isize) as ::core::ffi::c_int <= '9' as i32
        {
            iVal = iVal
                .wrapping_mul(10 as crate::src::ext::rtree::rtree::U64_0)
                .wrapping_add(
                    (*zText.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                        as crate::src::ext::rtree::rtree::U64_0,
                );
            i += 1;
        }
        *piEndBlock = iVal as crate::src::ext::rtree::rtree::I64_0;
        while *zText.offset(i as isize) as ::core::ffi::c_int == ' ' as i32 {
            i += 1;
        }
        iVal = 0 as crate::src::ext::rtree::rtree::U64_0;
        if *zText.offset(i as isize) as ::core::ffi::c_int == '-' as i32 {
            i += 1;
            iMul = -(1 as ::core::ffi::c_int);
        }
        while *zText.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
            && *zText.offset(i as isize) as ::core::ffi::c_int <= '9' as i32
        {
            iVal = iVal
                .wrapping_mul(10 as crate::src::ext::rtree::rtree::U64_0)
                .wrapping_add(
                    (*zText.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                        as crate::src::ext::rtree::rtree::U64_0,
                );
            i += 1;
        }
        *pnByte = iVal as crate::src::ext::rtree::rtree::I64_0
            * iMul as crate::src::ext::rtree::rtree::I64_0;
    }
}

unsafe extern "C" fn fts3PromoteSegments(
    p: *mut crate::fts3Int_h::Fts3Table,
    iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    nByte: crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pRange: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectLevelRange2 as ::core::ffi::c_int,
        &raw mut pRange,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut bOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let iLast: crate::src::ext::rtree::rtree::I64_0 = (iAbsLevel
            as crate::src::ext::rtree::rtree::I64_0
            / crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL as crate::src::ext::rtree::rtree::I64_0
            + 1 as crate::src::ext::rtree::rtree::I64_0)
            * crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL as crate::src::ext::rtree::rtree::I64_0
            - 1 as crate::src::ext::rtree::rtree::I64_0;
        let nLimit: crate::src::ext::rtree::rtree::I64_0 = nByte
            as crate::src::ext::rtree::rtree::I64_0
            * 3 as crate::src::ext::rtree::rtree::I64_0
            / 2 as crate::src::ext::rtree::rtree::I64_0;
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pRange,
            1 as ::core::ffi::c_int,
            iAbsLevel + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pRange,
            2 as ::core::ffi::c_int,
            iLast as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        while crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pRange)
        {
            let mut nSize: crate::src::ext::rtree::rtree::I64_0 =
                0 as crate::src::ext::rtree::rtree::I64_0;
            let mut dummy: crate::src::ext::rtree::rtree::I64_0 = 0;
            fts3ReadEndBlockField(
                pRange,
                2 as ::core::ffi::c_int,
                &raw mut dummy,
                &raw mut nSize,
            );
            if nSize <= 0 as crate::src::ext::rtree::rtree::I64_0 || nSize > nLimit {
                bOk = 0 as ::core::ffi::c_int;
                break;
            } else {
                bOk = 1 as ::core::ffi::c_int;
            }
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pRange);
        if bOk != 0 {
            let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pUpdate1: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
            let mut pUpdate2: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3SqlStmt(
                    p,
                    SqlConstant::SqlUpdateLevelIdx as ::core::ffi::c_int,
                    &raw mut pUpdate1,
                    ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
                );
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3SqlStmt(
                    p,
                    SqlConstant::SqlUpdateLevel as ::core::ffi::c_int,
                    &raw mut pUpdate2,
                    ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
                );
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::vdbeapi::sqlite3_bind_int64(
                    pRange,
                    1 as ::core::ffi::c_int,
                    iAbsLevel,
                );
                while crate::src::headers::sqlite3_h::SQLITE_ROW
                    == crate::src::src::vdbeapi::sqlite3_step(pRange)
                {
                    let fresh3 = iIdx;
                    iIdx += 1;
                    crate::src::src::vdbeapi::sqlite3_bind_int(
                        pUpdate1,
                        1 as ::core::ffi::c_int,
                        fresh3,
                    );
                    crate::src::src::vdbeapi::sqlite3_bind_int(
                        pUpdate1,
                        2 as ::core::ffi::c_int,
                        crate::src::src::vdbeapi::sqlite3_column_int(
                            pRange,
                            0 as ::core::ffi::c_int,
                        ),
                    );
                    crate::src::src::vdbeapi::sqlite3_bind_int(
                        pUpdate1,
                        3 as ::core::ffi::c_int,
                        crate::src::src::vdbeapi::sqlite3_column_int(
                            pRange,
                            1 as ::core::ffi::c_int,
                        ),
                    );
                    crate::src::src::vdbeapi::sqlite3_step(pUpdate1);
                    rc = crate::src::src::vdbeapi::sqlite3_reset(pUpdate1);
                    if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                        continue;
                    }
                    crate::src::src::vdbeapi::sqlite3_reset(pRange);
                    break;
                }
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::src::src::vdbeapi::sqlite3_reset(pRange);
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::vdbeapi::sqlite3_bind_int64(
                    pUpdate2,
                    1 as ::core::ffi::c_int,
                    iAbsLevel,
                );
                crate::src::src::vdbeapi::sqlite3_step(pUpdate2);
                rc = crate::src::src::vdbeapi::sqlite3_reset(pUpdate2);
            }
        }
    }
    rc
}

unsafe extern "C" fn fts3SegmentMerge(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    iIndex: ::core::ffi::c_int,
    iLevel: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int;
    let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iNewLevel: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut pWriter: *mut SegmentWriter = ::core::ptr::null_mut::<SegmentWriter>();
    let mut filter: crate::fts3Int_h::Fts3SegFilter = unsafe { ::core::mem::zeroed() };
    let mut csr: crate::fts3Int_h::Fts3MultiSegReader = unsafe { ::core::mem::zeroed() };
    let mut bIgnoreEmpty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iMaxLevel: crate::src::ext::rtree::rtree::I64_0 =
        0 as crate::src::ext::rtree::rtree::I64_0;
    rc = crate::src::ext::fts3::fts3::sqlite3Fts3SegReaderCursor(
        p as *mut crate::fts3Int_h::Fts3Table,
        iLangid,
        iIndex,
        iLevel,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        &raw mut csr as *mut _ as *mut crate::fts3Int_h::Fts3MultiSegReader,
    );
    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK || csr.nSegment == 0 as ::core::ffi::c_int)
    {
        if iLevel != crate::fts3Int_h::FTS3_SEGCURSOR_PENDING {
            rc = fts3SegmentMaxLevel(p, iLangid, iIndex, &raw mut iMaxLevel);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                current_block = 14255763095006029735;
            } else {
                current_block = 13183875560443969876;
            }
        } else {
            current_block = 13183875560443969876;
        }
        match current_block {
            14255763095006029735 => {}
            _ => {
                if iLevel == crate::fts3Int_h::FTS3_SEGCURSOR_ALL {
                    if csr.nSegment == 1 as ::core::ffi::c_int
                        && 0 as ::core::ffi::c_int
                            == ((**csr.apSegment.offset(0_isize)).ppNextElem
                                != ::core::ptr::null_mut::<
                                    *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
                                >()) as ::core::ffi::c_int
                    {
                        rc = crate::src::headers::sqlite3_h::SQLITE_DONE;
                        current_block = 14255763095006029735;
                    } else {
                        iNewLevel = iMaxLevel as crate::src::headers::sqlite3_h::Sqlite3Int64;
                        bIgnoreEmpty = 1 as ::core::ffi::c_int;
                        current_block = 15652330335145281839;
                    }
                } else {
                    iNewLevel =
                        getAbsoluteLevel(p, iLangid, iIndex, iLevel + 1 as ::core::ffi::c_int);
                    rc = fts3AllocateSegdirIdx(
                        p,
                        iLangid,
                        iIndex,
                        iLevel + 1 as ::core::ffi::c_int,
                        &raw mut iIdx,
                    );
                    bIgnoreEmpty = (iLevel != crate::fts3Int_h::FTS3_SEGCURSOR_PENDING
                        && iNewLevel > iMaxLevel)
                        as ::core::ffi::c_int;
                    current_block = 15652330335145281839;
                }
                match current_block {
                    14255763095006029735 => {}
                    _ => {
                        if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                            filter.flags = crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS;
                            filter.flags |= if bIgnoreEmpty != 0 {
                                crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            rc = sqlite3Fts3SegReaderStart(p, &raw mut csr, &raw mut filter);
                            while crate::src::headers::sqlite3_h::SQLITE_OK == rc {
                                rc = sqlite3Fts3SegReaderStep(p, &raw mut csr);
                                if rc != crate::src::headers::sqlite3_h::SQLITE_ROW {
                                    break;
                                }
                                rc = fts3SegWriterAdd(
                                    p,
                                    &raw mut pWriter,
                                    1 as ::core::ffi::c_int,
                                    csr.zTerm,
                                    csr.nTerm,
                                    csr.aDoclist,
                                    csr.nDoclist,
                                );
                            }
                            if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                                if iLevel != crate::fts3Int_h::FTS3_SEGCURSOR_PENDING {
                                    rc = fts3DeleteSegdir(
                                        p,
                                        iLangid,
                                        iIndex,
                                        iLevel,
                                        csr.apSegment,
                                        csr.nSegment,
                                    );
                                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                        current_block = 14255763095006029735;
                                    } else {
                                        current_block = 2569451025026770673;
                                    }
                                } else {
                                    current_block = 2569451025026770673;
                                }
                                match current_block {
                                    14255763095006029735 => {}
                                    _ => {
                                        if !pWriter.is_null() {
                                            rc = fts3SegWriterFlush(p, pWriter, iNewLevel, iIdx);
                                            if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                                                && (iLevel
                                                    == crate::fts3Int_h::FTS3_SEGCURSOR_PENDING
                                                    || iNewLevel < iMaxLevel)
                                                {
                                                    rc = fts3PromoteSegments(
                                                        p,
                                                        iNewLevel,
                                                        (*pWriter).nLeafData as crate::src::headers::sqlite3_h::Sqlite3Int64,
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
        }
    }
    fts3SegWriterFree(pWriter);
    sqlite3Fts3SegReaderFinish(&raw mut csr);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3PendingTermsFlush(
    p: *mut crate::fts3Int_h::Fts3Table,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    let __p_ref = unsafe { &mut *p };
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < __p_ref.nIndex {
        rc = fts3SegmentMerge(
            p,
            __p_ref.iPrevLangid,
            i,
            crate::fts3Int_h::FTS3_SEGCURSOR_PENDING,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        i += 1;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && __p_ref.bHasStat as ::core::ffi::c_int != 0
        && __p_ref.nAutoincrmerge == 0xff as ::core::ffi::c_int
        && __p_ref.nLeafAdd > 0 as crate::src::ext::rtree::rtree::U32_0
    {
        let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlSelectStat as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int(
                pStmt,
                1 as ::core::ffi::c_int,
                FtsStatConstant::FtsStatAutoincrmerge as ::core::ffi::c_int,
            );
            rc = crate::src::src::vdbeapi::sqlite3_step(pStmt);
            if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
                __p_ref.nAutoincrmerge =
                    crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
                if __p_ref.nAutoincrmerge == 1 as ::core::ffi::c_int {
                    __p_ref.nAutoincrmerge = 8 as ::core::ffi::c_int;
                }
            } else if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                __p_ref.nAutoincrmerge = 0 as ::core::ffi::c_int;
            }
            rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        sqlite3Fts3PendingTermsClear(p);
    }
    rc
}

unsafe extern "C" fn fts3EncodeIntArray(
    N: ::core::ffi::c_int,
    a: *mut crate::src::ext::rtree::rtree::U32_0,
    zBuf: *mut ::core::ffi::c_char,
    pNBuf: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    i = j;
    while i < N {
        j += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
            zBuf.offset(j as isize) as *mut ::core::ffi::c_char,
            *a.offset(i as isize) as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        i += 1;
    }
    *pNBuf = j;
}

unsafe extern "C" fn fts3DecodeIntArray(
    N: ::core::ffi::c_int,
    a: *mut crate::src::ext::rtree::rtree::U32_0,
    zBuf: *const ::core::ffi::c_char,
    nBuf: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nBuf != 0
        && *zBuf.offset((nBuf - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        let mut j: ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        i = j;
        while i < N && j < nBuf {
            let mut x: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
            j += crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(
                zBuf.offset(j as isize) as *const ::core::ffi::c_char,
                &raw mut x,
            );
            *a.offset(i as isize) = (x & 0xffffffff
                as crate::src::headers::sqlite3_h::Sqlite3Int64)
                as crate::src::ext::rtree::rtree::U32_0;
            i += 1;
        }
    }
    while i < N {
        let fresh1 = i;
        i += 1;
        *a.offset(fresh1 as isize) = 0 as crate::src::ext::rtree::rtree::U32_0;
    }
}

unsafe extern "C" fn fts3InsertDocsize(
    pRC: *mut ::core::ffi::c_int,
    p: *mut crate::fts3Int_h::Fts3Table,
    aSz: *mut crate::src::ext::rtree::rtree::U32_0,
) {
    
    let mut nBlob: ::core::ffi::c_int = 0;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    
    if *pRC != 0 {
        return;
    }
    let __p_ref = unsafe { &*p };
    let pBlob: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_malloc64(
        (10 as crate::src::headers::sqlite3_h::Sqlite3Int64
            * __p_ref.nColumn as crate::src::headers::sqlite3_h::Sqlite3Int64)
            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut ::core::ffi::c_char;
    if pBlob.is_null() {
        *pRC = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        return;
    }
    fts3EncodeIntArray(__p_ref.nColumn, aSz, pBlob, &raw mut nBlob);
    let rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SqlConstant::SqlReplaceDocsize as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc != 0 {
        crate::src::src::malloc::sqlite3_free(pBlob as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        pStmt,
        1 as ::core::ffi::c_int,
        __p_ref.iPrevDocid as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    crate::src::src::vdbeapi::sqlite3_bind_blob(
        pStmt,
        2 as ::core::ffi::c_int,
        pBlob as *const ::core::ffi::c_void,
        nBlob,
        Some(
            crate::src::src::malloc::sqlite3_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
    );
    crate::src::src::vdbeapi::sqlite3_step(pStmt);
    *pRC = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
}

unsafe extern "C" fn fts3UpdateDocTotals(
    pRC: *mut ::core::ffi::c_int,
    p: *mut crate::fts3Int_h::Fts3Table,
    aSzIns: *mut crate::src::ext::rtree::rtree::U32_0,
    aSzDel: *mut crate::src::ext::rtree::rtree::U32_0,
    nChng: ::core::ffi::c_int,
) {
    
    let mut nBlob: ::core::ffi::c_int = 0;
    
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut i: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    let nStat: ::core::ffi::c_int = (*p).nColumn + 2 as ::core::ffi::c_int;
    if *pRC != 0 {
        return;
    }
    let a: *mut crate::src::ext::rtree::rtree::U32_0 = crate::src::src::malloc::sqlite3_malloc64(
        ((::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>() as usize)
            .wrapping_add(10_usize) as crate::src::headers::sqlite3_h::Sqlite3Uint64)
            .wrapping_mul(
                nStat as crate::src::headers::sqlite3_h::Sqlite3Int64
                    as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ),
    ) as *mut crate::src::ext::rtree::rtree::U32_0;
    if a.is_null() {
        *pRC = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        return;
    }
    let pBlob: *mut ::core::ffi::c_char = a.offset(nStat as isize) as *mut crate::src::ext::rtree::rtree::U32_0
        as *mut ::core::ffi::c_char;
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectStat as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc != 0 {
        crate::src::src::malloc::sqlite3_free(a as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int(
        pStmt,
        1 as ::core::ffi::c_int,
        FtsStatConstant::FtsStatDoctotal.into(),
    );
    if crate::src::src::vdbeapi::sqlite3_step(pStmt) == crate::src::headers::sqlite3_h::SQLITE_ROW {
        fts3DecodeIntArray(
            nStat,
            a,
            crate::src::src::vdbeapi::sqlite3_column_blob(pStmt, 0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char,
            crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 0 as ::core::ffi::c_int),
        );
    } else {
        std::ptr::write_bytes(
            a as *mut ::core::ffi::c_void as *mut u8,
            0 as ::core::ffi::c_int as u8,
            (::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul(nStat as crate::__stddef_size_t_h::SizeT),
        );
    }
    rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::malloc::sqlite3_free(a as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    if nChng < 0 as ::core::ffi::c_int
        && *a.offset(0_isize) < -nChng as crate::src::ext::rtree::rtree::U32_0
    {
        *a.offset(0_isize) = 0 as crate::src::ext::rtree::rtree::U32_0;
    } else {
        let fresh0 = &mut *a.offset(0_isize);
        *fresh0 = (*fresh0).wrapping_add(nChng as crate::src::ext::rtree::rtree::U32_0);
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nColumn + 1 as ::core::ffi::c_int {
        let mut x: crate::src::ext::rtree::rtree::U32_0 =
            *a.offset((i + 1 as ::core::ffi::c_int) as isize);
        if x.wrapping_add(*aSzIns.offset(i as isize)) < *aSzDel.offset(i as isize) {
            x = 0 as crate::src::ext::rtree::rtree::U32_0;
        } else {
            x = x
                .wrapping_add(*aSzIns.offset(i as isize))
                .wrapping_sub(*aSzDel.offset(i as isize));
        }
        *a.offset((i + 1 as ::core::ffi::c_int) as isize) = x;
        i += 1;
    }
    fts3EncodeIntArray(nStat, a, pBlob, &raw mut nBlob);
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlReplaceStat as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc != 0 {
        crate::src::src::malloc::sqlite3_free(a as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int(
        pStmt,
        1 as ::core::ffi::c_int,
        FtsStatConstant::FtsStatDoctotal.into(),
    );
    crate::src::src::vdbeapi::sqlite3_bind_blob(
        pStmt,
        2 as ::core::ffi::c_int,
        pBlob as *const ::core::ffi::c_void,
        nBlob,
        crate::src::headers::sqlite3_h::SQLITE_STATIC,
    );
    crate::src::src::vdbeapi::sqlite3_step(pStmt);
    *pRC = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    crate::src::src::vdbeapi::sqlite3_bind_null(pStmt, 2 as ::core::ffi::c_int);
    crate::src::src::malloc::sqlite3_free(a as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn fts3DoOptimize(
    p: *mut crate::fts3Int_h::Fts3Table,
    bReturnDone: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bSeenDone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    let mut pAllLangid: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    rc = sqlite3Fts3PendingTermsFlush(p);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlSelectAllLangid as ::core::ffi::c_int,
            &raw mut pAllLangid,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        
        crate::src::src::vdbeapi::sqlite3_bind_int(
            pAllLangid,
            1 as ::core::ffi::c_int,
            (*p).iPrevLangid,
        );
        crate::src::src::vdbeapi::sqlite3_bind_int(
            pAllLangid,
            2 as ::core::ffi::c_int,
            (*p).nIndex,
        );
        while crate::src::src::vdbeapi::sqlite3_step(pAllLangid)
            == crate::src::headers::sqlite3_h::SQLITE_ROW
        {
            let mut i: ::core::ffi::c_int;
            let iLangid: ::core::ffi::c_int =
                crate::src::src::vdbeapi::sqlite3_column_int(pAllLangid, 0 as ::core::ffi::c_int);
            i = 0 as ::core::ffi::c_int;
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < (*p).nIndex {
                rc = fts3SegmentMerge(p, iLangid, i, crate::fts3Int_h::FTS3_SEGCURSOR_ALL);
                if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                    bSeenDone = 1 as ::core::ffi::c_int;
                    rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                }
                i += 1;
            }
        }
        let rc2: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_reset(pAllLangid);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    sqlite3Fts3SegmentsClose(p);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && bReturnDone != 0 && bSeenDone != 0 {
        crate::src::headers::sqlite3_h::SQLITE_DONE
    } else {
        rc
    }
}

unsafe extern "C" fn fts3DoRebuild(p: *mut crate::fts3Int_h::Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    rc = fts3DeleteAll(p, 0 as ::core::ffi::c_int);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut aSz: *mut crate::src::ext::rtree::rtree::U32_0 =
            ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U32_0>();
        let mut aSzIns: *mut crate::src::ext::rtree::rtree::U32_0 =
            ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U32_0>();
        let mut aSzDel: *mut crate::src::ext::rtree::rtree::U32_0 =
            ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U32_0>();
        let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        let mut nEntry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let zSql: *mut ::core::ffi::c_char = sqlite_printf!("SELECT %s", (*p).zReadExprlist);
        if zSql.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            rc = crate::src::src::prepare::sqlite3_prepare_v2(
                (*p).db,
                zSql,
                -(1 as ::core::ffi::c_int),
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                (::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                    as ::core::ffi::c_ulonglong)
                    .wrapping_mul(
                        ((*p).nColumn as crate::src::headers::sqlite3_h::Sqlite3Int64
                            + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64)
                            as ::core::ffi::c_ulonglong,
                    )
                    .wrapping_mul(3 as ::core::ffi::c_ulonglong)
                    as crate::src::headers::sqlite3_h::Sqlite3Int64;
            aSz = crate::src::src::malloc::sqlite3_malloc64(
                nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ) as *mut crate::src::ext::rtree::rtree::U32_0;
            if aSz.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                std::ptr::write_bytes(
                    aSz as *mut ::core::ffi::c_void as *mut u8,
                    0 as ::core::ffi::c_int as u8,
                    nByte as crate::__stddef_size_t_h::SizeT,
                );
                aSzIns = aSz.offset(((*p).nColumn + 1 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::ext::rtree::rtree::U32_0;
                aSzDel = aSzIns.offset(((*p).nColumn + 1 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::ext::rtree::rtree::U32_0;
            }
        }
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && crate::src::headers::sqlite3_h::SQLITE_ROW
                == crate::src::src::vdbeapi::sqlite3_step(pStmt)
        {
            let mut iCol: ::core::ffi::c_int;
            let iLangid: ::core::ffi::c_int = langidFromSelect(p, pStmt);
            rc = fts3PendingTermsDocid(
                p,
                0 as ::core::ffi::c_int,
                iLangid,
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int)
                    as crate::src::headers::sqlite3_h::SqliteInt64,
            );
            let __p_ref = unsafe { &mut *p };
            std::ptr::write_bytes(
                aSz as *mut ::core::ffi::c_void as *mut u8,
                0 as ::core::ffi::c_int as u8,
                (::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                    as crate::__stddef_size_t_h::SizeT)
                    .wrapping_mul(
                        (__p_ref.nColumn + 1 as ::core::ffi::c_int)
                            as crate::__stddef_size_t_h::SizeT,
                    ),
            );
            iCol = 0 as ::core::ffi::c_int;
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && iCol < __p_ref.nColumn {
                if *__p_ref.abNotindexed.offset(iCol as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let z: *const ::core::ffi::c_char =
                        crate::src::src::vdbeapi::sqlite3_column_text(
                            pStmt,
                            iCol + 1 as ::core::ffi::c_int,
                        ) as *const ::core::ffi::c_char;
                    rc = fts3PendingTermsAdd(
                        p,
                        iLangid,
                        z,
                        iCol,
                        aSz.offset(iCol as isize) as *mut crate::src::ext::rtree::rtree::U32_0,
                    );
                    let fresh14 = &mut *aSz.offset(__p_ref.nColumn as isize);
                    *fresh14 = (*fresh14).wrapping_add(
                        crate::src::src::vdbeapi::sqlite3_column_bytes(
                            pStmt,
                            iCol + 1 as ::core::ffi::c_int,
                        ) as crate::src::ext::rtree::rtree::U32_0,
                    );
                }
                iCol += 1;
            }
            if __p_ref.bHasDocsize != 0 {
                fts3InsertDocsize(&raw mut rc, p, aSz);
            }
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
                pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
            } else {
                nEntry += 1;
                iCol = 0 as ::core::ffi::c_int;
                while iCol <= __p_ref.nColumn {
                    let fresh15 = &mut *aSzIns.offset(iCol as isize);
                    *fresh15 = (*fresh15).wrapping_add(*aSz.offset(iCol as isize));
                    iCol += 1;
                }
            }
        }
        if (*p).bFts4 != 0 {
            fts3UpdateDocTotals(&raw mut rc, p, aSzIns, aSzDel, nEntry);
        }
        crate::src::src::malloc::sqlite3_free(aSz as *mut ::core::ffi::c_void);
        if !pStmt.is_null() {
            let rc2: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = rc2;
            }
        }
    }
    rc
}

unsafe extern "C" fn fts3IncrmergeCsr(
    p: *mut crate::fts3Int_h::Fts3Table,
    iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    nSeg: ::core::ffi::c_int,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    
    std::ptr::write_bytes(
        pCsr as *mut ::core::ffi::c_void as *mut u8,
        0 as ::core::ffi::c_int as u8,
        ::core::mem::size_of::<crate::fts3Int_h::Fts3MultiSegReader>()
            as crate::__stddef_size_t_h::SizeT,
    );
    let nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 = (::core::mem::size_of::<*mut Fts3SegReader>() as usize).wrapping_mul(nSeg as usize)
        as crate::src::headers::sqlite3_h::Sqlite3Int64;
    (*pCsr).apSegment = crate::src::src::malloc::sqlite3_malloc64(
        nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut *mut Fts3SegReader;
    if (*pCsr).apSegment.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        std::ptr::write_bytes(
            (*pCsr).apSegment as *mut ::core::ffi::c_void as *mut u8,
            0 as ::core::ffi::c_int as u8,
            nByte as crate::__stddef_size_t_h::SizeT,
        );
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlSelectLevel as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut i: ::core::ffi::c_int;
        
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iAbsLevel);
        i = 0 as ::core::ffi::c_int;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && crate::src::src::vdbeapi::sqlite3_step(pStmt)
                == crate::src::headers::sqlite3_h::SQLITE_ROW
            && i < nSeg
        {
            rc = sqlite3Fts3SegReaderNew(
                i,
                0 as ::core::ffi::c_int,
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 1 as ::core::ffi::c_int),
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 2 as ::core::ffi::c_int),
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 3 as ::core::ffi::c_int),
                crate::src::src::vdbeapi::sqlite3_column_blob(pStmt, 4 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 4 as ::core::ffi::c_int),
                (*pCsr).apSegment.offset(i as isize) as *mut *mut Fts3SegReader,
            );
            (*pCsr).nSegment += 1;
            i += 1;
        }
        let rc2: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    rc
}

unsafe extern "C" fn blobGrowBuffer(
    pBlob: *mut Blob,
    nMin: ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) {
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK && nMin > (*pBlob).nAlloc {
        let nAlloc: ::core::ffi::c_int = nMin;
        let a: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
            (*pBlob).a as *mut ::core::ffi::c_void,
            nAlloc as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut ::core::ffi::c_char;
        if !a.is_null() {
            (*pBlob).nAlloc = nAlloc;
            (*pBlob).a = a;
        } else {
            *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    }
}

unsafe extern "C" fn nodeReaderNext(p: *mut NodeReader) -> ::core::ffi::c_int {
    let __p_ref = unsafe { &mut *p };
    let bFirst: ::core::ffi::c_int =
        (__p_ref.term.n == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut nPrefix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nSuffix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if __p_ref.iChild != 0 && bFirst == 0 as ::core::ffi::c_int {
        __p_ref.iChild += 1;
    }
    if __p_ref.iOff >= __p_ref.nNode {
        __p_ref.aNode = ::core::ptr::null::<::core::ffi::c_char>();
    } else {
        if bFirst == 0 as ::core::ffi::c_int {
            __p_ref.iOff += if *(__p_ref.aNode.offset(__p_ref.iOff as isize)
                as *const ::core::ffi::c_char
                as *mut crate::src::ext::rtree::rtree::U8_0)
                as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(
                    __p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char,
                    &raw mut nPrefix,
                )
            } else {
                nPrefix = *(__p_ref.aNode.offset(__p_ref.iOff as isize)
                    as *const ::core::ffi::c_char
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            };
        }
        __p_ref.iOff += if *(__p_ref.aNode.offset(__p_ref.iOff as isize)
            as *const ::core::ffi::c_char
            as *mut crate::src::ext::rtree::rtree::U8_0)
            as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
        {
            crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(
                __p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char,
                &raw mut nSuffix,
            )
        } else {
            nSuffix = *(__p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char
                as *mut crate::src::ext::rtree::rtree::U8_0)
                as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        };
        if nPrefix > __p_ref.term.n
            || nSuffix > __p_ref.nNode - __p_ref.iOff
            || nSuffix == 0 as ::core::ffi::c_int
        {
            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
        }
        blobGrowBuffer(&raw mut __p_ref.term, nPrefix + nSuffix, &raw mut rc);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !__p_ref.term.a.is_null() {
            ::core::ptr::copy_nonoverlapping(
                __p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char,
                __p_ref.term.a.offset(nPrefix as isize) as *mut ::core::ffi::c_char,
                nSuffix as usize,
            );
            __p_ref.term.n = nPrefix + nSuffix;
            __p_ref.iOff += nSuffix;
            if __p_ref.iChild == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 {
                __p_ref.iOff += if *(__p_ref.aNode.offset(__p_ref.iOff as isize)
                    as *const ::core::ffi::c_char
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    != 0
                {
                    crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(
                        __p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char,
                        &raw mut __p_ref.nDoclist,
                    )
                } else {
                    __p_ref.nDoclist = *(__p_ref.aNode.offset(__p_ref.iOff as isize)
                        as *const ::core::ffi::c_char
                        as *mut crate::src::ext::rtree::rtree::U8_0)
                        as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                };
                if __p_ref.nNode - __p_ref.iOff < __p_ref.nDoclist {
                    return crate::fts3Int_h::FTS_CORRUPT_VTAB;
                }
                __p_ref.aDoclist =
                    __p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char;
                __p_ref.iOff += __p_ref.nDoclist;
            }
        }
    }
    rc
}

unsafe extern "C" fn nodeReaderRelease(p: *mut NodeReader) {
    crate::src::src::malloc::sqlite3_free((*p).term.a as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn nodeReaderInit(
    p: *mut NodeReader,
    aNode: *const ::core::ffi::c_char,
    nNode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    std::ptr::write_bytes(
        p as *mut ::core::ffi::c_void as *mut u8,
        0 as ::core::ffi::c_int as u8,
        ::core::mem::size_of::<NodeReader>() as crate::__stddef_size_t_h::SizeT,
    );
    (*p).aNode = aNode;
    (*p).nNode = nNode;
    if !aNode.is_null() && *aNode.offset(0_isize) as ::core::ffi::c_int != 0 {
        let __p_ref = unsafe { &mut *p };
        __p_ref.iOff = 1 as ::core::ffi::c_int
            + crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(
                __p_ref.aNode.offset(1_isize) as *const ::core::ffi::c_char,
                &raw mut __p_ref.iChild,
            );
    } else {
        (*p).iOff = 1 as ::core::ffi::c_int;
    }
    if !aNode.is_null() {
        nodeReaderNext(p)
    } else {
        crate::src::headers::sqlite3_h::SQLITE_OK
    }
}

unsafe extern "C" fn fts3IncrmergePush(
    p: *mut crate::fts3Int_h::Fts3Table,
    pWriter: *mut IncrmergeWriter,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut iPtr: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        (*pWriter).aNodeWriter[0 as ::core::ffi::c_int as usize].iBlock;
    let mut iLayer: ::core::ffi::c_int;
    iLayer = 1 as ::core::ffi::c_int;
    while iLayer < 16 as ::core::ffi::c_int {
        let mut iNextPtr: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let pNode: *mut NodeWriter = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
            .offset(iLayer as isize) as *mut NodeWriter;
        let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
        
        
        let mut nSpace: ::core::ffi::c_int;
        let __pNode_ref = unsafe { &mut *pNode };
        let nPrefix: ::core::ffi::c_int = fts3PrefixCompress(__pNode_ref.key.a, __pNode_ref.key.n, zTerm, nTerm);
        let nSuffix: ::core::ffi::c_int = nTerm - nPrefix;
        if nSuffix <= 0 as ::core::ffi::c_int {
            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
        }
        nSpace = crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
            nPrefix as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        );
        nSpace += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
            nSuffix as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) + nSuffix;
        if __pNode_ref.key.n == 0 as ::core::ffi::c_int
            || __pNode_ref.block.n + nSpace <= (*p).nNodeSize
        {
            let pBlk: *mut Blob = &raw mut __pNode_ref.block;
            if (*pBlk).n == 0 as ::core::ffi::c_int {
                blobGrowBuffer(pBlk, (*p).nNodeSize, &raw mut rc);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    let __pBlk_ref = unsafe { &mut *pBlk };
                    *__pBlk_ref.a.offset(0_isize) = iLayer as ::core::ffi::c_char;
                    __pBlk_ref.n = 1 as ::core::ffi::c_int
                        + crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                            __pBlk_ref.a.offset(1_isize) as *mut ::core::ffi::c_char,
                            iPtr,
                        );
                }
            }
            blobGrowBuffer(pBlk, (*pBlk).n + nSpace, &raw mut rc);
            blobGrowBuffer(&raw mut __pNode_ref.key, nTerm, &raw mut rc);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let __pBlk_ref = unsafe { &mut *pBlk };
                if __pNode_ref.key.n != 0 {
                    __pBlk_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                        __pBlk_ref.a.offset(__pBlk_ref.n as isize) as *mut ::core::ffi::c_char,
                        nPrefix as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    );
                }
                __pBlk_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                    __pBlk_ref.a.offset(__pBlk_ref.n as isize) as *mut ::core::ffi::c_char,
                    nSuffix as crate::src::headers::sqlite3_h::Sqlite3Int64,
                );
                ::core::ptr::copy_nonoverlapping(
                    zTerm.offset(nPrefix as isize) as *const ::core::ffi::c_char,
                    __pBlk_ref.a.offset(__pBlk_ref.n as isize) as *mut ::core::ffi::c_char,
                    nSuffix as usize,
                );
                __pBlk_ref.n += nSuffix;
                ::core::ptr::copy_nonoverlapping(
                    zTerm as *const u8,
                    __pNode_ref.key.a as *mut u8,
                    nTerm as usize,
                );
                __pNode_ref.key.n = nTerm;
            }
        } else {
            rc = fts3WriteSegment(
                p,
                __pNode_ref.iBlock,
                __pNode_ref.block.a,
                __pNode_ref.block.n,
            );
            *__pNode_ref.block.a.offset(0_isize) = iLayer as ::core::ffi::c_char;
            __pNode_ref.block.n = 1 as ::core::ffi::c_int
                + crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                    __pNode_ref.block.a.offset(1_isize) as *mut ::core::ffi::c_char,
                    iPtr + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64,
                );
            iNextPtr = __pNode_ref.iBlock;
            __pNode_ref.iBlock += 1;
            __pNode_ref.key.n = 0 as ::core::ffi::c_int;
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK
            || iNextPtr == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
        {
            return rc;
        }
        iPtr = iNextPtr;
        iLayer += 1;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn fts3AppendToNode(
    pNode: *mut Blob,
    pPrev: *mut Blob,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    aDoclist: *const ::core::ffi::c_char,
    nDoclist: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPrev_ref = unsafe { &mut *pPrev };
    let bFirst: ::core::ffi::c_int =
        (__pPrev_ref.n == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    
    
    blobGrowBuffer(pPrev, nTerm, &raw mut rc);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    let nPrefix: ::core::ffi::c_int = fts3PrefixCompress(__pPrev_ref.a, __pPrev_ref.n, zTerm, nTerm);
    let nSuffix: ::core::ffi::c_int = nTerm - nPrefix;
    if nSuffix <= 0 as ::core::ffi::c_int {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    ::core::ptr::copy_nonoverlapping(zTerm as *const u8, __pPrev_ref.a as *mut u8, nTerm as usize);
    __pPrev_ref.n = nTerm;
    let __pNode_ref = unsafe { &mut *pNode };
    if bFirst == 0 as ::core::ffi::c_int {
        __pNode_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
            __pNode_ref.a.offset(__pNode_ref.n as isize) as *mut ::core::ffi::c_char,
            nPrefix as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
    }
    __pNode_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
        __pNode_ref.a.offset(__pNode_ref.n as isize) as *mut ::core::ffi::c_char,
        nSuffix as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    ::core::ptr::copy_nonoverlapping(
        zTerm.offset(nPrefix as isize) as *const ::core::ffi::c_char,
        __pNode_ref.a.offset(__pNode_ref.n as isize) as *mut ::core::ffi::c_char,
        nSuffix as usize,
    );
    __pNode_ref.n += nSuffix;
    if !aDoclist.is_null() {
        __pNode_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
            __pNode_ref.a.offset(__pNode_ref.n as isize) as *mut ::core::ffi::c_char,
            nDoclist as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        ::core::ptr::copy_nonoverlapping(
            aDoclist as *const u8,
            __pNode_ref.a.offset(__pNode_ref.n as isize) as *mut ::core::ffi::c_char as *mut u8,
            nDoclist as usize,
        );
        __pNode_ref.n += nDoclist;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3IncrmergeAppend(
    p: *mut crate::fts3Int_h::Fts3Table,
    pWriter: *mut IncrmergeWriter,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let __pCsr_ref = unsafe { &*pCsr };
    let zTerm: *const ::core::ffi::c_char = __pCsr_ref.zTerm;
    let nTerm: ::core::ffi::c_int = __pCsr_ref.nTerm;
    let aDoclist: *const ::core::ffi::c_char = __pCsr_ref.aDoclist;
    let nDoclist: ::core::ffi::c_int = __pCsr_ref.nDoclist;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut nSpace: ::core::ffi::c_int;
    
    let mut nSuffix: ::core::ffi::c_int;
    
    let __pWriter_ref = unsafe { &mut *pWriter };
    let pLeaf: *mut NodeWriter = (&raw mut __pWriter_ref.aNodeWriter as *mut NodeWriter).offset(0_isize)
        as *mut NodeWriter;
    let nPrefix: ::core::ffi::c_int = fts3PrefixCompress((*pLeaf).key.a, (*pLeaf).key.n, zTerm, nTerm);
    nSuffix = nTerm - nPrefix;
    if nSuffix <= 0 as ::core::ffi::c_int {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    nSpace = crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
        nPrefix as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    );
    nSpace += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
        nSuffix as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) + nSuffix;
    nSpace += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
        nDoclist as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) + nDoclist;
    if (*pLeaf).block.n > 0 as ::core::ffi::c_int
        && (*pLeaf).block.n + nSpace > (*p).nNodeSize
        && (*pLeaf).iBlock
            < __pWriter_ref.iStart as crate::src::ext::rtree::rtree::I64_0 + __pWriter_ref.nLeafEst
    {
        let __pLeaf_ref = unsafe { &mut *pLeaf };
        rc = fts3WriteSegment(
            p,
            __pLeaf_ref.iBlock,
            __pLeaf_ref.block.a,
            __pLeaf_ref.block.n,
        );
        __pWriter_ref.nWork += 1;
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = fts3IncrmergePush(p, pWriter, zTerm, nPrefix + 1 as ::core::ffi::c_int);
        }
        __pLeaf_ref.iBlock += 1;
        __pLeaf_ref.key.n = 0 as ::core::ffi::c_int;
        __pLeaf_ref.block.n = 0 as ::core::ffi::c_int;
        nSuffix = nTerm;
        nSpace = 1 as ::core::ffi::c_int;
        nSpace += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
            nSuffix as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) + nSuffix;
        nSpace += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(
            nDoclist as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) + nDoclist;
    }
    __pWriter_ref.nLeafData += nSpace as crate::src::headers::sqlite3_h::Sqlite3Int64;
    blobGrowBuffer(
        &raw mut (*pLeaf).block,
        (*pLeaf).block.n + nSpace,
        &raw mut rc,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pLeaf_ref = unsafe { &mut *pLeaf };
        if __pLeaf_ref.block.n == 0 as ::core::ffi::c_int {
            __pLeaf_ref.block.n = 1 as ::core::ffi::c_int;
            *__pLeaf_ref.block.a.offset(0_isize) = '\0' as i32 as ::core::ffi::c_char;
        }
        rc = fts3AppendToNode(
            &raw mut __pLeaf_ref.block,
            &raw mut __pLeaf_ref.key,
            zTerm,
            nTerm,
            aDoclist,
            nDoclist,
        );
    }
    rc
}

unsafe extern "C" fn fts3IncrmergeRelease(
    p: *mut crate::fts3Int_h::Fts3Table,
    pWriter: *mut IncrmergeWriter,
    pRc: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int;
    let mut iRoot: ::core::ffi::c_int;
    
    let mut rc: ::core::ffi::c_int = *pRc;
    iRoot = FTS_MAX_APPENDABLE_HEIGHT - 1 as ::core::ffi::c_int;
    while iRoot >= 0 as ::core::ffi::c_int {
        let pNode: *mut NodeWriter = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
            .offset(iRoot as isize) as *mut NodeWriter;
        let __pNode_ref = unsafe { &*pNode };
        if __pNode_ref.block.n > 0 as ::core::ffi::c_int {
            break;
        }
        crate::src::src::malloc::sqlite3_free(__pNode_ref.block.a as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(__pNode_ref.key.a as *mut ::core::ffi::c_void);
        iRoot -= 1;
    }
    if iRoot < 0 as ::core::ffi::c_int {
        return;
    }
    if iRoot == 0 as ::core::ffi::c_int {
        let pBlock: *mut Blob =
            &raw mut (*(&raw mut (*pWriter).aNodeWriter as *mut NodeWriter).offset(1_isize))
                .block;
        blobGrowBuffer(
            pBlock,
            1 as ::core::ffi::c_int + crate::fts3Int_h::FTS3_VARINT_MAX,
            &raw mut rc,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let __pBlock_ref = unsafe { &mut *pBlock };
            *__pBlock_ref.a.offset(0_isize) = 0x1 as ::core::ffi::c_char;
            __pBlock_ref.n = 1 as ::core::ffi::c_int
                + crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                    __pBlock_ref.a.offset(1_isize) as *mut ::core::ffi::c_char,
                    (*pWriter).aNodeWriter[0 as ::core::ffi::c_int as usize].iBlock,
                );
        }
        iRoot = 1 as ::core::ffi::c_int;
    }
    let pRoot: *mut NodeWriter = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter).offset(iRoot as isize)
        as *mut NodeWriter;
    i = 0 as ::core::ffi::c_int;
    while i < iRoot {
        let pNode_0: *mut NodeWriter = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
            .offset(i as isize) as *mut NodeWriter;
        let __pNode_0_ref = unsafe { &*pNode_0 };
        if __pNode_0_ref.block.n > 0 as ::core::ffi::c_int
            && rc == crate::src::headers::sqlite3_h::SQLITE_OK
        {
            rc = fts3WriteSegment(
                p,
                __pNode_0_ref.iBlock,
                __pNode_0_ref.block.a,
                __pNode_0_ref.block.n,
            );
        }
        crate::src::src::malloc::sqlite3_free(__pNode_0_ref.block.a as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(__pNode_0_ref.key.a as *mut ::core::ffi::c_void);
        i += 1;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pWriter_ref = unsafe { &*pWriter };
        rc = fts3WriteSegdir(
            p,
            __pWriter_ref.iAbsLevel + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64,
            __pWriter_ref.iIdx,
            __pWriter_ref.iStart,
            __pWriter_ref.aNodeWriter[0 as ::core::ffi::c_int as usize].iBlock,
            __pWriter_ref.iEnd,
            if __pWriter_ref.bNoLeafData as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                __pWriter_ref.nLeafData
            } else {
                0 as crate::src::headers::sqlite3_h::Sqlite3Int64
            },
            (*pRoot).block.a,
            (*pRoot).block.n,
        );
    }
    crate::src::src::malloc::sqlite3_free((*pRoot).block.a as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free((*pRoot).key.a as *mut ::core::ffi::c_void);
    *pRc = rc;
}

unsafe extern "C" fn fts3TermCmp(
    zLhs: *const ::core::ffi::c_char,
    nLhs: ::core::ffi::c_int,
    zRhs: *const ::core::ffi::c_char,
    nRhs: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let nCmp: ::core::ffi::c_int = if nLhs < nRhs { nLhs } else { nRhs };
    let mut res: ::core::ffi::c_int;
    if nCmp != 0 && !zLhs.is_null() && !zRhs.is_null() {
        res = fts3_memcmp_safe(
            zLhs as *const ::core::ffi::c_void,
            zRhs as *const ::core::ffi::c_void,
            nCmp as usize,
        );
    } else {
        res = 0 as ::core::ffi::c_int;
    }
    if res == 0 as ::core::ffi::c_int {
        res = nLhs - nRhs;
    }
    res
}

unsafe extern "C" fn fts3IsAppendable(
    p: *mut crate::fts3Int_h::Fts3Table,
    iEnd: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pbRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bRes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pCheck: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int;
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlSegmentIsAppendable as ::core::ffi::c_int,
        &raw mut pCheck,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pCheck, 1 as ::core::ffi::c_int, iEnd);
        if crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pCheck)
        {
            bRes = 1 as ::core::ffi::c_int;
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pCheck);
    }
    *pbRes = bRes;
    rc
}

unsafe extern "C" fn fts3IncrmergeLoad(
    p: *mut crate::fts3Int_h::Fts3Table,
    iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iIdx: ::core::ffi::c_int,
    zKey: *const ::core::ffi::c_char,
    nKey: ::core::ffi::c_int,
    pWriter: *mut IncrmergeWriter,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pSelect: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectSegdir as ::core::ffi::c_int,
        &raw mut pSelect,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let iStart: crate::src::headers::sqlite3_h::Sqlite3Int64;
        let iLeafEnd: crate::src::headers::sqlite3_h::Sqlite3Int64;
        let mut iEnd: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let aRoot: *const ::core::ffi::c_char;
        let nRoot: ::core::ffi::c_int;
        
        let mut bAppendable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pSelect,
            1 as ::core::ffi::c_int,
            iAbsLevel + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        crate::src::src::vdbeapi::sqlite3_bind_int(pSelect, 2 as ::core::ffi::c_int, iIdx);
        if crate::src::src::vdbeapi::sqlite3_step(pSelect)
            == crate::src::headers::sqlite3_h::SQLITE_ROW
        {
            iStart =
                crate::src::src::vdbeapi::sqlite3_column_int64(pSelect, 1 as ::core::ffi::c_int);
            iLeafEnd =
                crate::src::src::vdbeapi::sqlite3_column_int64(pSelect, 2 as ::core::ffi::c_int);
            let __pWriter_ref = unsafe { &mut *pWriter };
            fts3ReadEndBlockField(
                pSelect,
                3 as ::core::ffi::c_int,
                &raw mut iEnd,
                &raw mut __pWriter_ref.nLeafData,
            );
            if __pWriter_ref.nLeafData < 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 {
                __pWriter_ref.nLeafData *=
                    -(1 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::Sqlite3Int64;
            }
            __pWriter_ref.bNoLeafData =
                (__pWriter_ref.nLeafData == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64)
                    as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U8_0;
            nRoot =
                crate::src::src::vdbeapi::sqlite3_column_bytes(pSelect, 4 as ::core::ffi::c_int);
            aRoot = crate::src::src::vdbeapi::sqlite3_column_blob(pSelect, 4 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
            if aRoot.is_null() {
                crate::src::src::vdbeapi::sqlite3_reset(pSelect);
                return if nRoot != 0 {
                    crate::src::headers::sqlite3_h::SQLITE_NOMEM
                } else {
                    crate::fts3Int_h::FTS_CORRUPT_VTAB
                };
            }
        } else {
            return crate::src::src::vdbeapi::sqlite3_reset(pSelect);
        }
        rc = fts3IsAppendable(p, iEnd, &raw mut bAppendable);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && bAppendable != 0 {
            let mut aLeaf: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut nLeaf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = sqlite3Fts3ReadBlock(
                p,
                iLeafEnd,
                &raw mut aLeaf,
                &raw mut nLeaf,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let mut reader: NodeReader = unsafe { ::core::mem::zeroed() };
                rc = nodeReaderInit(&raw mut reader, aLeaf, nLeaf);
                while rc == crate::src::headers::sqlite3_h::SQLITE_OK && !reader.aNode.is_null() {
                    rc = nodeReaderNext(&raw mut reader);
                }
                if fts3TermCmp(zKey, nKey, reader.term.a, reader.term.n) <= 0 as ::core::ffi::c_int
                {
                    bAppendable = 0 as ::core::ffi::c_int;
                }
                nodeReaderRelease(&raw mut reader);
            }
            crate::src::src::malloc::sqlite3_free(aLeaf as *mut ::core::ffi::c_void);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && bAppendable != 0 {
            let mut i: ::core::ffi::c_int;
            let nHeight: ::core::ffi::c_int = *aRoot.offset(0_isize) as ::core::ffi::c_int;
            let mut pNode: *mut NodeWriter;
            if nHeight < 1 as ::core::ffi::c_int || nHeight >= FTS_MAX_APPENDABLE_HEIGHT {
                crate::src::src::vdbeapi::sqlite3_reset(pSelect);
                return crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
            let __pWriter_ref = unsafe { &mut *pWriter };
            __pWriter_ref.nLeafEst = ((iEnd - iStart
                + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64)
                as ::core::ffi::c_int
                / FTS_MAX_APPENDABLE_HEIGHT)
                as crate::src::ext::rtree::rtree::I64_0;
            __pWriter_ref.iStart = iStart;
            __pWriter_ref.iEnd = iEnd;
            __pWriter_ref.iAbsLevel = iAbsLevel;
            __pWriter_ref.iIdx = iIdx;
            i = nHeight + 1 as ::core::ffi::c_int;
            while i < FTS_MAX_APPENDABLE_HEIGHT {
                __pWriter_ref.aNodeWriter[i as usize].iBlock = (__pWriter_ref.iStart
                    as crate::src::ext::rtree::rtree::I64_0
                    + i as crate::src::ext::rtree::rtree::I64_0 * __pWriter_ref.nLeafEst)
                    as crate::src::headers::sqlite3_h::Sqlite3Int64;
                i += 1;
            }
            pNode = (&raw mut __pWriter_ref.aNodeWriter as *mut NodeWriter).offset(nHeight as isize)
                as *mut NodeWriter;
            (*pNode).iBlock = (__pWriter_ref.iStart as crate::src::ext::rtree::rtree::I64_0
                + __pWriter_ref.nLeafEst * nHeight as crate::src::ext::rtree::rtree::I64_0)
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
            blobGrowBuffer(
                &raw mut (*pNode).block,
                (if nRoot > (*p).nNodeSize {
                    nRoot
                } else {
                    (*p).nNodeSize
                }) + FTS3_NODE_PADDING,
                &raw mut rc,
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let __pNode_ref = unsafe { &mut *pNode };
                ::core::ptr::copy_nonoverlapping(
                    aRoot as *const u8,
                    __pNode_ref.block.a as *mut u8,
                    nRoot as usize,
                );
                __pNode_ref.block.n = nRoot;
                std::ptr::write_bytes(
                    __pNode_ref.block.a.offset(nRoot as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void as *mut u8,
                    0 as ::core::ffi::c_int as u8,
                    FTS3_NODE_PADDING as crate::__stddef_size_t_h::SizeT,
                );
            }
            i = nHeight;
            while i >= 0 as ::core::ffi::c_int && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let mut reader_0: NodeReader = unsafe { ::core::mem::zeroed() };
                pNode = (&raw mut __pWriter_ref.aNodeWriter as *mut NodeWriter).offset(i as isize)
                    as *mut NodeWriter;
                if !(*pNode).block.a.is_null() {
                    rc = nodeReaderInit(&raw mut reader_0, (*pNode).block.a, (*pNode).block.n);
                    while !reader_0.aNode.is_null()
                        && rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    {
                        rc = nodeReaderNext(&raw mut reader_0);
                    }
                    blobGrowBuffer(&raw mut (*pNode).key, reader_0.term.n, &raw mut rc);
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        if reader_0.term.n > 0 as ::core::ffi::c_int {
                            ::core::ptr::copy_nonoverlapping(
                                reader_0.term.a as *const u8,
                                (*pNode).key.a as *mut u8,
                                reader_0.term.n as usize,
                            );
                        }
                        (*pNode).key.n = reader_0.term.n;
                        if i > 0 as ::core::ffi::c_int {
                            let mut aBlock: *mut ::core::ffi::c_char =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            let mut nBlock: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            pNode = (&raw mut __pWriter_ref.aNodeWriter as *mut NodeWriter)
                                .offset((i - 1 as ::core::ffi::c_int) as isize)
                                as *mut NodeWriter;
                            (*pNode).iBlock = reader_0.iChild;
                            rc = sqlite3Fts3ReadBlock(
                                p,
                                reader_0.iChild,
                                &raw mut aBlock,
                                &raw mut nBlock,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            );
                            blobGrowBuffer(
                                &raw mut (*pNode).block,
                                (if nBlock > (*p).nNodeSize {
                                    nBlock
                                } else {
                                    (*p).nNodeSize
                                }) + FTS3_NODE_PADDING,
                                &raw mut rc,
                            );
                            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                let __pNode_ref = unsafe { &mut *pNode };
                                ::core::ptr::copy_nonoverlapping(
                                    aBlock as *const u8,
                                    __pNode_ref.block.a as *mut u8,
                                    nBlock as usize,
                                );
                                __pNode_ref.block.n = nBlock;
                                std::ptr::write_bytes(
                                    __pNode_ref.block.a.offset(nBlock as isize)
                                        as *mut ::core::ffi::c_char
                                        as *mut ::core::ffi::c_void
                                        as *mut u8,
                                    0 as ::core::ffi::c_int as u8,
                                    FTS3_NODE_PADDING as crate::__stddef_size_t_h::SizeT,
                                );
                            }
                            crate::src::src::malloc::sqlite3_free(
                                aBlock as *mut ::core::ffi::c_void,
                            );
                        }
                    }
                }
                nodeReaderRelease(&raw mut reader_0);
                i -= 1;
            }
        }
        let rc2: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_reset(pSelect);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    rc
}

unsafe extern "C" fn fts3IncrmergeOutputIdx(
    p: *mut crate::fts3Int_h::Fts3Table,
    iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    piIdx: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pOutputIdx: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlNextSegmentIndex as ::core::ffi::c_int,
        &raw mut pOutputIdx,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pOutputIdx,
            1 as ::core::ffi::c_int,
            iAbsLevel + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        crate::src::src::vdbeapi::sqlite3_step(pOutputIdx);
        *piIdx = crate::src::src::vdbeapi::sqlite3_column_int(pOutputIdx, 0 as ::core::ffi::c_int);
        rc = crate::src::src::vdbeapi::sqlite3_reset(pOutputIdx);
    }
    rc
}

unsafe extern "C" fn fts3IncrmergeWriter(
    p: *mut crate::fts3Int_h::Fts3Table,
    iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iIdx: ::core::ffi::c_int,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    pWriter: *mut IncrmergeWriter,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut nLeafEst: crate::src::ext::rtree::rtree::I64_0 =
        0 as crate::src::ext::rtree::rtree::I64_0;
    let mut pLeafEst: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut pFirstBlock: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlMaxLeafNodeEstimate as ::core::ffi::c_int,
        &raw mut pLeafEst,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pLeafEst, 1 as ::core::ffi::c_int, iAbsLevel);
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pLeafEst,
            2 as ::core::ffi::c_int,
            (*pCsr).nSegment as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        if crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pLeafEst)
        {
            nLeafEst =
                crate::src::src::vdbeapi::sqlite3_column_int64(pLeafEst, 0 as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::I64_0;
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pLeafEst);
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlNextSegmentsId as ::core::ffi::c_int,
        &raw mut pFirstBlock,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    let __pWriter_ref = unsafe { &mut *pWriter };
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pFirstBlock)
        {
            __pWriter_ref.iStart = crate::src::src::vdbeapi::sqlite3_column_int64(
                pFirstBlock,
                0 as ::core::ffi::c_int,
            );
            __pWriter_ref.iEnd =
                __pWriter_ref.iStart - 1 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            __pWriter_ref.iEnd += (nLeafEst
                * FTS_MAX_APPENDABLE_HEIGHT as crate::src::ext::rtree::rtree::I64_0)
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pFirstBlock);
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    rc = fts3WriteSegment(
        p,
        __pWriter_ref.iEnd,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    __pWriter_ref.iAbsLevel = iAbsLevel;
    __pWriter_ref.nLeafEst = nLeafEst;
    __pWriter_ref.iIdx = iIdx;
    i = 0 as ::core::ffi::c_int;
    while i < FTS_MAX_APPENDABLE_HEIGHT {
        __pWriter_ref.aNodeWriter[i as usize].iBlock = (__pWriter_ref.iStart
            as crate::src::ext::rtree::rtree::I64_0
            + i as crate::src::ext::rtree::rtree::I64_0 * __pWriter_ref.nLeafEst)
            as crate::src::headers::sqlite3_h::Sqlite3Int64;
        i += 1;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3RemoveSegdirEntry(
    p: *mut crate::fts3Int_h::Fts3Table,
    iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iIdx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pDelete: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlDeleteSegdirEntry as ::core::ffi::c_int,
        &raw mut pDelete,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pDelete, 1 as ::core::ffi::c_int, iAbsLevel);
        crate::src::src::vdbeapi::sqlite3_bind_int(pDelete, 2 as ::core::ffi::c_int, iIdx);
        crate::src::src::vdbeapi::sqlite3_step(pDelete);
        rc = crate::src::src::vdbeapi::sqlite3_reset(pDelete);
    }
    rc
}

unsafe extern "C" fn fts3RepackSegdirLevel(
    p: *mut crate::fts3Int_h::Fts3Table,
    iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut aIdx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut nIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nAlloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut pSelect: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut pUpdate: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectIndexes as ::core::ffi::c_int,
        &raw mut pSelect,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        
        crate::src::src::vdbeapi::sqlite3_bind_int64(pSelect, 1 as ::core::ffi::c_int, iAbsLevel);
        while crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pSelect)
        {
            if nIdx >= nAlloc {
                
                nAlloc += 16 as ::core::ffi::c_int;
                let aNew: *mut ::core::ffi::c_int = crate::src::src::malloc::sqlite3_realloc64(
                    aIdx as *mut ::core::ffi::c_void,
                    (nAlloc as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                ) as *mut ::core::ffi::c_int;
                if aNew.is_null() {
                    rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    break;
                } else {
                    aIdx = aNew;
                }
            }
            let fresh13 = nIdx;
            nIdx += 1;
            *aIdx.offset(fresh13 as isize) =
                crate::src::src::vdbeapi::sqlite3_column_int(pSelect, 0 as ::core::ffi::c_int);
        }
        let rc2: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_reset(pSelect);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlShiftSegdirEntry as ::core::ffi::c_int,
            &raw mut pUpdate,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pUpdate, 2 as ::core::ffi::c_int, iAbsLevel);
    }
    (*p).bIgnoreSavepoint = 1 as crate::src::ext::rtree::rtree::U8_0;
    i = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < nIdx {
        if *aIdx.offset(i as isize) != i {
            crate::src::src::vdbeapi::sqlite3_bind_int(
                pUpdate,
                3 as ::core::ffi::c_int,
                *aIdx.offset(i as isize),
            );
            crate::src::src::vdbeapi::sqlite3_bind_int(pUpdate, 1 as ::core::ffi::c_int, i);
            crate::src::src::vdbeapi::sqlite3_step(pUpdate);
            rc = crate::src::src::vdbeapi::sqlite3_reset(pUpdate);
        }
        i += 1;
    }
    (*p).bIgnoreSavepoint = 0 as crate::src::ext::rtree::rtree::U8_0;
    crate::src::src::malloc::sqlite3_free(aIdx as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn fts3StartNode(
    pNode: *mut Blob,
    iHeight: ::core::ffi::c_int,
    iChild: crate::src::headers::sqlite3_h::Sqlite3Int64,
) {
    *(*pNode).a.offset(0_isize) = iHeight as ::core::ffi::c_char;
    if iChild != 0 {
        (*pNode).n = 1 as ::core::ffi::c_int
            + crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                (*pNode).a.offset(1_isize) as *mut ::core::ffi::c_char,
                iChild,
            );
    } else {
        (*pNode).n = 1 as ::core::ffi::c_int;
    };
}

unsafe extern "C" fn fts3TruncateNode(
    aNode: *const ::core::ffi::c_char,
    nNode: ::core::ffi::c_int,
    pNew: *mut Blob,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    piBlock: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let mut reader: NodeReader = unsafe { ::core::mem::zeroed() };
    let mut prev: Blob = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    
    if nNode < 1 as ::core::ffi::c_int {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    let bLeaf: ::core::ffi::c_int = (*aNode.offset(0_isize) as ::core::ffi::c_int == '\0' as i32) as ::core::ffi::c_int;
    blobGrowBuffer(pNew, nNode, &raw mut rc);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    (*pNew).n = 0 as ::core::ffi::c_int;
    let mut current_block_10: u64;
    rc = nodeReaderInit(&raw mut reader, aNode, nNode);
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && !reader.aNode.is_null() {
        if (*pNew).n == 0 as ::core::ffi::c_int {
            let res: ::core::ffi::c_int =
                fts3TermCmp(reader.term.a, reader.term.n, zTerm, nTerm);
            if res < 0 as ::core::ffi::c_int
                || bLeaf == 0 as ::core::ffi::c_int && res == 0 as ::core::ffi::c_int
            {
                current_block_10 = 4906268039856690917;
            } else {
                fts3StartNode(
                    pNew,
                    *aNode.offset(0_isize) as ::core::ffi::c_int,
                    reader.iChild,
                );
                *piBlock = reader.iChild;
                current_block_10 = 12209867499936983673;
            }
        } else {
            current_block_10 = 12209867499936983673;
        }
        if current_block_10 == 12209867499936983673 {
            rc = fts3AppendToNode(
                pNew,
                &raw mut prev,
                reader.term.a,
                reader.term.n,
                reader.aDoclist,
                reader.nDoclist,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                break;
            }
        }
        rc = nodeReaderNext(&raw mut reader);
    }
    if (*pNew).n == 0 as ::core::ffi::c_int {
        fts3StartNode(
            pNew,
            *aNode.offset(0_isize) as ::core::ffi::c_int,
            reader.iChild,
        );
        *piBlock = reader.iChild;
    }
    nodeReaderRelease(&raw mut reader);
    crate::src::src::malloc::sqlite3_free(prev.a as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn fts3TruncateSegment(
    p: *mut crate::fts3Int_h::Fts3Table,
    iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iIdx: ::core::ffi::c_int,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut root: Blob = unsafe { ::core::mem::zeroed() };
    let mut block: Blob = unsafe { ::core::mem::zeroed() };
    let mut iBlock: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut iNewStart: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut iOldStart: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut pFetch: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectSegdir as ::core::ffi::c_int,
        &raw mut pFetch,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        
        crate::src::src::vdbeapi::sqlite3_bind_int64(pFetch, 1 as ::core::ffi::c_int, iAbsLevel);
        crate::src::src::vdbeapi::sqlite3_bind_int(pFetch, 2 as ::core::ffi::c_int, iIdx);
        if crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pFetch)
        {
            let aRoot: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_column_blob(pFetch, 4 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
            let nRoot: ::core::ffi::c_int =
                crate::src::src::vdbeapi::sqlite3_column_bytes(pFetch, 4 as ::core::ffi::c_int);
            iOldStart =
                crate::src::src::vdbeapi::sqlite3_column_int64(pFetch, 1 as ::core::ffi::c_int);
            rc = fts3TruncateNode(aRoot, nRoot, &raw mut root, zTerm, nTerm, &raw mut iBlock);
        }
        let rc2: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_reset(pFetch);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && iBlock != 0 {
        let mut aBlock: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nBlock: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        iNewStart = iBlock;
        rc = sqlite3Fts3ReadBlock(
            p,
            iBlock,
            &raw mut aBlock,
            &raw mut nBlock,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = fts3TruncateNode(
                aBlock,
                nBlock,
                &raw mut block,
                zTerm,
                nTerm,
                &raw mut iBlock,
            );
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = fts3WriteSegment(p, iNewStart, block.a, block.n);
        }
        crate::src::src::malloc::sqlite3_free(aBlock as *mut ::core::ffi::c_void);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && iNewStart != 0 {
        let mut pDel: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlDeleteSegmentsRange as ::core::ffi::c_int,
            &raw mut pDel,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int64(pDel, 1 as ::core::ffi::c_int, iOldStart);
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pDel,
                2 as ::core::ffi::c_int,
                iNewStart - 1 as crate::src::headers::sqlite3_h::Sqlite3Int64,
            );
            crate::src::src::vdbeapi::sqlite3_step(pDel);
            rc = crate::src::src::vdbeapi::sqlite3_reset(pDel);
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut pChomp: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        rc = fts3SqlStmt(
            p,
            SqlConstant::SqlChompSegdir as ::core::ffi::c_int,
            &raw mut pChomp,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pChomp,
                1 as ::core::ffi::c_int,
                iNewStart,
            );
            crate::src::src::vdbeapi::sqlite3_bind_blob(
                pChomp,
                2 as ::core::ffi::c_int,
                root.a as *const ::core::ffi::c_void,
                root.n,
                crate::src::headers::sqlite3_h::SQLITE_STATIC,
            );
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pChomp,
                3 as ::core::ffi::c_int,
                iAbsLevel,
            );
            crate::src::src::vdbeapi::sqlite3_bind_int(pChomp, 4 as ::core::ffi::c_int, iIdx);
            crate::src::src::vdbeapi::sqlite3_step(pChomp);
            rc = crate::src::src::vdbeapi::sqlite3_reset(pChomp);
            crate::src::src::vdbeapi::sqlite3_bind_null(pChomp, 2 as ::core::ffi::c_int);
        }
    }
    crate::src::src::malloc::sqlite3_free(root.a as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(block.a as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn fts3IncrmergeChomp(
    p: *mut crate::fts3Int_h::Fts3Table,
    iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    pnRem: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut nRem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    i = (*pCsr).nSegment - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut pSeg: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
        let mut j: ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        while j < (*pCsr).nSegment {
            pSeg = *(*pCsr).apSegment.offset(j as isize);
            if (*pSeg).iIdx == i {
                break;
            }
            j += 1;
        }
        if (*pSeg).aNode.is_null() {
            rc = fts3DeleteSegment(p, pSeg);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3RemoveSegdirEntry(p, iAbsLevel, (*pSeg).iIdx);
            }
            *pnRem = 0 as ::core::ffi::c_int;
        } else {
            let __pSeg_ref = unsafe { &*pSeg };
            let zTerm: *const ::core::ffi::c_char = __pSeg_ref.zTerm;
            let nTerm: ::core::ffi::c_int = __pSeg_ref.nTerm;
            rc = fts3TruncateSegment(p, iAbsLevel, __pSeg_ref.iIdx, zTerm, nTerm);
            nRem += 1;
        }
        i -= 1;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && nRem != (*pCsr).nSegment {
        rc = fts3RepackSegdirLevel(p, iAbsLevel);
    }
    *pnRem = nRem;
    rc
}

unsafe extern "C" fn fts3IncrmergeHintStore(
    p: *mut crate::fts3Int_h::Fts3Table,
    pHint: *mut Blob,
) -> ::core::ffi::c_int {
    let mut pReplace: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int;
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlReplaceStat as ::core::ffi::c_int,
        &raw mut pReplace,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int(
            pReplace,
            1 as ::core::ffi::c_int,
            FtsStatConstant::FtsStatIncrmergehint.into(),
        );
        crate::src::src::vdbeapi::sqlite3_bind_blob(
            pReplace,
            2 as ::core::ffi::c_int,
            (*pHint).a as *const ::core::ffi::c_void,
            (*pHint).n,
            crate::src::headers::sqlite3_h::SQLITE_STATIC,
        );
        crate::src::src::vdbeapi::sqlite3_step(pReplace);
        rc = crate::src::src::vdbeapi::sqlite3_reset(pReplace);
        crate::src::src::vdbeapi::sqlite3_bind_null(pReplace, 2 as ::core::ffi::c_int);
    }
    rc
}

unsafe extern "C" fn fts3IncrmergeHintLoad(
    p: *mut crate::fts3Int_h::Fts3Table,
    pHint: *mut Blob,
) -> ::core::ffi::c_int {
    let mut pSelect: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int;
    (*pHint).n = 0 as ::core::ffi::c_int;
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectStat as ::core::ffi::c_int,
        &raw mut pSelect,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        
        crate::src::src::vdbeapi::sqlite3_bind_int(
            pSelect,
            1 as ::core::ffi::c_int,
            FtsStatConstant::FtsStatIncrmergehint.into(),
        );
        if crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pSelect)
        {
            let aHint: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_column_blob(pSelect, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
            let nHint: ::core::ffi::c_int =
                crate::src::src::vdbeapi::sqlite3_column_bytes(pSelect, 0 as ::core::ffi::c_int);
            if !aHint.is_null() {
                blobGrowBuffer(pHint, nHint, &raw mut rc);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    if !(*pHint).a.is_null() {
                        ::core::ptr::copy_nonoverlapping(
                            aHint as *const u8,
                            (*pHint).a as *mut u8,
                            nHint as usize,
                        );
                    }
                    (*pHint).n = nHint;
                }
            }
        }
        let rc2: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_reset(pSelect);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    rc
}

unsafe extern "C" fn fts3IncrmergeHintPush(
    pHint: *mut Blob,
    iAbsLevel: crate::src::ext::rtree::rtree::I64_0,
    nInput: ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) {
    blobGrowBuffer(
        pHint,
        (*pHint).n + 2 as ::core::ffi::c_int * crate::fts3Int_h::FTS3_VARINT_MAX,
        pRc,
    );
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pHint_ref = unsafe { &mut *pHint };
        __pHint_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
            __pHint_ref.a.offset(__pHint_ref.n as isize) as *mut ::core::ffi::c_char,
            iAbsLevel as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        __pHint_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
            __pHint_ref.a.offset(__pHint_ref.n as isize) as *mut ::core::ffi::c_char,
            nInput as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
    }
}

unsafe extern "C" fn fts3IncrmergeHintPop(
    pHint: *mut Blob,
    piAbsLevel: *mut crate::src::ext::rtree::rtree::I64_0,
    pnInput: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pHint_ref = unsafe { &mut *pHint };
    let nHint: ::core::ffi::c_int = __pHint_ref.n;
    let mut i: ::core::ffi::c_int;
    i = __pHint_ref.n - 1 as ::core::ffi::c_int;
    if *__pHint_ref.a.offset(i as isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    while i > 0 as ::core::ffi::c_int
        && *__pHint_ref.a.offset((i - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
    {
        i -= 1;
    }
    if i == 0 as ::core::ffi::c_int {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    i -= 1;
    while i > 0 as ::core::ffi::c_int
        && *__pHint_ref.a.offset((i - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
    {
        i -= 1;
    }
    __pHint_ref.n = i;
    i += crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(
        __pHint_ref.a.offset(i as isize) as *mut ::core::ffi::c_char,
        piAbsLevel as *mut crate::src::headers::sqlite3_h::SqliteInt64,
    );
    i += if *(__pHint_ref.a.offset(i as isize) as *mut ::core::ffi::c_char
        as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
        & 0x80 as ::core::ffi::c_int
        != 0
    {
        crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(
            __pHint_ref.a.offset(i as isize) as *mut ::core::ffi::c_char,
            pnInput,
        )
    } else {
        *pnInput = *(__pHint_ref.a.offset(i as isize) as *mut ::core::ffi::c_char
            as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
        1 as ::core::ffi::c_int
    };
    if i != nHint {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3Incrmerge(
    p: *mut crate::fts3Int_h::Fts3Table,
    nMerge: ::core::ffi::c_int,
    nMin: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut nRem: ::core::ffi::c_int = nMerge;
    
    
    
    let mut nSeg: ::core::ffi::c_int;
    let mut iAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut hint: Blob = unsafe { ::core::mem::zeroed() };
    let mut bDirtyHint: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let nAlloc: ::core::ffi::c_int = (::core::mem::size_of::<crate::fts3Int_h::Fts3MultiSegReader>()
        as usize)
        .wrapping_add(::core::mem::size_of::<crate::fts3Int_h::Fts3SegFilter>() as usize)
        .wrapping_add(::core::mem::size_of::<IncrmergeWriter>() as usize)
        as ::core::ffi::c_int;
    let pWriter: *mut IncrmergeWriter = crate::src::src::malloc::sqlite3_malloc64(
        nAlloc as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut IncrmergeWriter;
    if pWriter.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    let pFilter: *mut crate::fts3Int_h::Fts3SegFilter = pWriter.offset(1_isize) as *mut IncrmergeWriter as *mut crate::fts3Int_h::Fts3SegFilter;
    let pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader = pFilter.offset(1_isize) as *mut crate::fts3Int_h::Fts3SegFilter
        as *mut crate::fts3Int_h::Fts3MultiSegReader;
    rc = fts3IncrmergeHintLoad(p, &raw mut hint);
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && nRem > 0 as ::core::ffi::c_int {
        let nMod: crate::src::ext::rtree::rtree::I64_0 = (crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL
            * (*p).nIndex)
            as crate::src::ext::rtree::rtree::I64_0;
        let pFindLevel: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        let mut bUseHint: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        crate::src::src::vdbeapi::sqlite3_bind_int(
            pFindLevel,
            1 as ::core::ffi::c_int,
            if 2 as ::core::ffi::c_int > nMin {
                2 as ::core::ffi::c_int
            } else {
                nMin
            },
        );
        if crate::src::src::vdbeapi::sqlite3_step(pFindLevel)
            == crate::src::headers::sqlite3_h::SQLITE_ROW
        {
            iAbsLevel =
                crate::src::src::vdbeapi::sqlite3_column_int64(pFindLevel, 0 as ::core::ffi::c_int);
            nSeg =
                crate::src::src::vdbeapi::sqlite3_column_int(pFindLevel, 1 as ::core::ffi::c_int);
        } else {
            nSeg = -(1 as ::core::ffi::c_int);
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pFindLevel);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && hint.n != 0 {
            let nHint: ::core::ffi::c_int = hint.n;
            let mut iHintAbsLevel: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            let mut nHintSeg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = fts3IncrmergeHintPop(&raw mut hint, &raw mut iHintAbsLevel, &raw mut nHintSeg);
            if nSeg < 0 as ::core::ffi::c_int
                || iAbsLevel as crate::src::ext::rtree::rtree::I64_0 % nMod
                    >= iHintAbsLevel as crate::src::ext::rtree::rtree::I64_0 % nMod
            {
                iAbsLevel = iHintAbsLevel;
                nSeg = if (if nMin > nSeg { nMin } else { nSeg }) < nHintSeg {
                    if nMin > nSeg { nMin } else { nSeg }
                } else {
                    nHintSeg
                };
                bUseHint = 1 as ::core::ffi::c_int;
                bDirtyHint = 1 as ::core::ffi::c_int;
            } else {
                hint.n = nHint;
            }
        }
        if nSeg <= 0 as ::core::ffi::c_int {
            break;
        }
        if iAbsLevel < 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
            || iAbsLevel > nMod << 32 as ::core::ffi::c_int
        {
            rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
            break;
        } else {
            std::ptr::write_bytes(
                pWriter as *mut ::core::ffi::c_void as *mut u8,
                0 as ::core::ffi::c_int as u8,
                nAlloc as crate::__stddef_size_t_h::SizeT,
            );
            (*pFilter).flags = crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS;
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3IncrmergeOutputIdx(p, iAbsLevel, &raw mut iIdx);
                if iIdx == 0 as ::core::ffi::c_int
                    || bUseHint != 0 && iIdx == 1 as ::core::ffi::c_int
                {
                    let mut bIgnore: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    rc = fts3SegmentIsMaxLevel(
                        p,
                        iAbsLevel as crate::src::ext::rtree::rtree::I64_0
                            + 1 as crate::src::ext::rtree::rtree::I64_0,
                        &raw mut bIgnore,
                    );
                    if bIgnore != 0 {
                        (*pFilter).flags |= crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY;
                    }
                }
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3IncrmergeCsr(p, iAbsLevel, nSeg, pCsr);
            }
            if crate::src::headers::sqlite3_h::SQLITE_OK == rc && (*pCsr).nSegment == nSeg && {
                rc = sqlite3Fts3SegReaderStart(p, pCsr, pFilter);
                crate::src::headers::sqlite3_h::SQLITE_OK == rc
            } {
                let mut bEmpty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                rc = sqlite3Fts3SegReaderStep(p, pCsr);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    bEmpty = 1 as ::core::ffi::c_int;
                } else if rc != crate::src::headers::sqlite3_h::SQLITE_ROW {
                    sqlite3Fts3SegReaderFinish(pCsr);
                    break;
                }
                if bUseHint != 0 && iIdx > 0 as ::core::ffi::c_int {
                    let zKey: *const ::core::ffi::c_char = (*pCsr).zTerm;
                    let nKey: ::core::ffi::c_int = (*pCsr).nTerm;
                    rc = fts3IncrmergeLoad(
                        p,
                        iAbsLevel,
                        iIdx - 1 as ::core::ffi::c_int,
                        zKey,
                        nKey,
                        pWriter,
                    );
                } else {
                    rc = fts3IncrmergeWriter(p, iAbsLevel, iIdx, pCsr, pWriter);
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK && (*pWriter).nLeafEst != 0 {
                    if bEmpty == 0 as ::core::ffi::c_int {
                        loop {
                            rc = fts3IncrmergeAppend(p, pWriter, pCsr);
                            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                rc = sqlite3Fts3SegReaderStep(p, pCsr);
                            }
                            if (*pWriter).nWork >= nRem as crate::src::ext::rtree::rtree::I64_0
                                && rc == crate::src::headers::sqlite3_h::SQLITE_ROW
                            {
                                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                            }
                            if (rc != crate::src::headers::sqlite3_h::SQLITE_ROW) {
                                break;
                            }
                        }
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        nRem = (nRem as crate::src::ext::rtree::rtree::I64_0
                            - (1 as crate::src::ext::rtree::rtree::I64_0 + (*pWriter).nWork))
                            as ::core::ffi::c_int;
                        rc = fts3IncrmergeChomp(p, iAbsLevel, pCsr, &raw mut nSeg);
                        if nSeg != 0 as ::core::ffi::c_int {
                            bDirtyHint = 1 as ::core::ffi::c_int;
                            fts3IncrmergeHintPush(
                                &raw mut hint,
                                iAbsLevel as crate::src::ext::rtree::rtree::I64_0,
                                nSeg,
                                &raw mut rc,
                            );
                        }
                    }
                }
                if nSeg != 0 as ::core::ffi::c_int {
                    (*pWriter).nLeafData *=
                        -(1 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::Sqlite3Int64;
                }
                fts3IncrmergeRelease(p, pWriter, &raw mut rc);
                if nSeg == 0 as ::core::ffi::c_int
                    && (*pWriter).bNoLeafData as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    fts3PromoteSegments(
                        p,
                        iAbsLevel + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64,
                        (*pWriter).nLeafData,
                    );
                }
            }
            sqlite3Fts3SegReaderFinish(pCsr);
        }
    }
    if bDirtyHint != 0 && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = fts3IncrmergeHintStore(p, &raw mut hint);
    }
    crate::src::src::malloc::sqlite3_free(pWriter as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(hint.a as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn fts3Getint(pz: *mut *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut z: *const ::core::ffi::c_char = *pz;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *z as ::core::ffi::c_int >= '0' as i32
        && *z as ::core::ffi::c_int <= '9' as i32
        && i < 214748363 as ::core::ffi::c_int
    {
        let fresh12 = z;
        z = z.offset(1);
        i = 10 as ::core::ffi::c_int * i + *fresh12 as ::core::ffi::c_int - '0' as i32;
    }
    *pz = z;
    i
}

unsafe extern "C" fn fts3DoIncrmerge(
    p: *mut crate::fts3Int_h::Fts3Table,
    zParam: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut nMin: ::core::ffi::c_int = (*p).nMergeCount / 2 as ::core::ffi::c_int;
    
    let mut z: *const ::core::ffi::c_char = zParam;
    let nMerge: ::core::ffi::c_int = fts3Getint(&raw mut z);
    if *z.offset(0_isize) as ::core::ffi::c_int == ',' as i32
        && *z.offset(1_isize) as ::core::ffi::c_int != '\0' as i32
    {
        z = z.offset(1);
        nMin = fts3Getint(&raw mut z);
    }
    if *z.offset(0_isize) as ::core::ffi::c_int != '\0' as i32 || nMin < 2 as ::core::ffi::c_int
    {
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    } else {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        if (*p).bHasStat == 0 {
            crate::src::ext::fts3::fts3::sqlite3Fts3CreateStatTable(
                &raw mut rc,
                p as *mut crate::fts3Int_h::Fts3Table,
            );
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = sqlite3Fts3Incrmerge(p, nMerge, nMin);
        }
        sqlite3Fts3SegmentsClose(p);
    }
    rc
}

unsafe extern "C" fn fts3DoAutoincrmerge(
    p: *mut crate::fts3Int_h::Fts3Table,
    mut zParam: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let __p_ref = unsafe { &mut *p };
    __p_ref.nAutoincrmerge = fts3Getint(&raw mut zParam);
    if __p_ref.nAutoincrmerge == 1 as ::core::ffi::c_int
        || __p_ref.nAutoincrmerge > __p_ref.nMergeCount
    {
        __p_ref.nAutoincrmerge = 8 as ::core::ffi::c_int;
    }
    if __p_ref.bHasStat == 0 {
        crate::src::ext::fts3::fts3::sqlite3Fts3CreateStatTable(
            &raw mut rc,
            p as *mut crate::fts3Int_h::Fts3Table,
        );
        if rc != 0 {
            return rc;
        }
    }
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlReplaceStat as ::core::ffi::c_int,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc != 0 {
        return rc;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int(
        pStmt,
        1 as ::core::ffi::c_int,
        FtsStatConstant::FtsStatAutoincrmerge.into(),
    );
    crate::src::src::vdbeapi::sqlite3_bind_int(
        pStmt,
        2 as ::core::ffi::c_int,
        __p_ref.nAutoincrmerge,
    );
    crate::src::src::vdbeapi::sqlite3_step(pStmt);
    rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    rc
}

unsafe extern "C" fn fts3ChecksumEntry(
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    iLangid: ::core::ffi::c_int,
    iIndex: ::core::ffi::c_int,
    iDocid: crate::src::ext::rtree::rtree::I64_0,
    iCol: ::core::ffi::c_int,
    iPos: ::core::ffi::c_int,
) -> crate::src::ext::rtree::rtree::U64_0 {
    let mut i: ::core::ffi::c_int;
    let mut ret: crate::src::ext::rtree::rtree::U64_0 =
        iDocid as crate::src::ext::rtree::rtree::U64_0;
    ret = ret.wrapping_add(
        (ret << 3 as ::core::ffi::c_int)
            .wrapping_add(iLangid as crate::src::ext::rtree::rtree::U64_0),
    );
    ret = ret.wrapping_add(
        (ret << 3 as ::core::ffi::c_int)
            .wrapping_add(iIndex as crate::src::ext::rtree::rtree::U64_0),
    );
    ret = ret.wrapping_add(
        (ret << 3 as ::core::ffi::c_int).wrapping_add(iCol as crate::src::ext::rtree::rtree::U64_0),
    );
    ret = ret.wrapping_add(
        (ret << 3 as ::core::ffi::c_int).wrapping_add(iPos as crate::src::ext::rtree::rtree::U64_0),
    );
    i = 0 as ::core::ffi::c_int;
    while i < nTerm {
        ret = ret.wrapping_add(
            (ret << 3 as ::core::ffi::c_int)
                .wrapping_add(*zTerm.offset(i as isize) as crate::src::ext::rtree::rtree::U64_0),
        );
        i += 1;
    }
    ret
}

unsafe extern "C" fn fts3ChecksumIndex(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    iIndex: ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) -> crate::src::ext::rtree::rtree::U64_0 {
    let mut filter: crate::fts3Int_h::Fts3SegFilter = unsafe { ::core::mem::zeroed() };
    let mut csr: crate::fts3Int_h::Fts3MultiSegReader = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int;
    let mut cksum: crate::src::ext::rtree::rtree::U64_0 = 0 as crate::src::ext::rtree::rtree::U64_0;
    if *pRc != 0 {
        return 0 as crate::src::ext::rtree::rtree::U64_0;
    }
    filter.flags =
        crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS | crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY;
    filter.flags |= crate::fts3Int_h::FTS3_SEGMENT_SCAN;
    rc = crate::src::ext::fts3::fts3::sqlite3Fts3SegReaderCursor(
        p as *mut crate::fts3Int_h::Fts3Table,
        iLangid,
        iIndex,
        crate::fts3Int_h::FTS3_SEGCURSOR_ALL,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        &raw mut csr as *mut _ as *mut crate::fts3Int_h::Fts3MultiSegReader,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3Fts3SegReaderStart(p, &raw mut csr, &raw mut filter);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        loop {
            rc = sqlite3Fts3SegReaderStep(p, &raw mut csr);
            if (crate::src::headers::sqlite3_h::SQLITE_ROW != rc) {
                break;
            }
            let mut pCsr: *mut ::core::ffi::c_char = csr.aDoclist;
            let pEnd: *mut ::core::ffi::c_char =
                pCsr.offset(csr.nDoclist as isize) as *mut ::core::ffi::c_char;
            let mut iDocid: crate::src::ext::rtree::rtree::I64_0 =
                0 as crate::src::ext::rtree::rtree::I64_0;
            let mut iCol: crate::src::ext::rtree::rtree::I64_0 =
                0 as crate::src::ext::rtree::rtree::I64_0;
            let mut iPos: crate::src::ext::rtree::rtree::U64_0 =
                0 as crate::src::ext::rtree::rtree::U64_0;
            pCsr = pCsr.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(
                pCsr,
                &raw mut iDocid,
            ) as isize);
            while pCsr < pEnd {
                let mut iVal: crate::src::ext::rtree::rtree::U64_0 =
                    0 as crate::src::ext::rtree::rtree::U64_0;
                pCsr = pCsr.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintU(
                    pCsr,
                    &raw mut iVal,
                ) as isize);
                if pCsr < pEnd {
                    if iVal == 0 as crate::src::ext::rtree::rtree::U64_0
                        || iVal == 1 as crate::src::ext::rtree::rtree::U64_0
                    {
                        iCol = 0 as crate::src::ext::rtree::rtree::I64_0;
                        iPos = 0 as crate::src::ext::rtree::rtree::U64_0;
                        if iVal != 0 {
                            pCsr = pCsr.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(
                                pCsr,
                                &raw mut iCol,
                            ) as isize);
                        } else {
                            pCsr = pCsr.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintU(
                                pCsr,
                                &raw mut iVal,
                            ) as isize);
                            if (*p).bDescIdx != 0 {
                                iDocid = (iDocid as crate::src::ext::rtree::rtree::U64_0)
                                    .wrapping_sub(iVal)
                                    as crate::src::ext::rtree::rtree::I64_0;
                            } else {
                                iDocid = (iDocid as crate::src::ext::rtree::rtree::U64_0)
                                    .wrapping_add(iVal)
                                    as crate::src::ext::rtree::rtree::I64_0;
                            }
                        }
                    } else {
                        iPos = iPos.wrapping_add(
                            iVal.wrapping_sub(2 as crate::src::ext::rtree::rtree::U64_0),
                        );
                        cksum ^= fts3ChecksumEntry(
                            csr.zTerm,
                            csr.nTerm,
                            iLangid,
                            iIndex,
                            iDocid,
                            iCol as ::core::ffi::c_int,
                            iPos as ::core::ffi::c_int,
                        );
                    }
                }
            }
        }
    }
    sqlite3Fts3SegReaderFinish(&raw mut csr);
    *pRc = rc;
    cksum
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3IntegrityCheck(
    p: *mut crate::fts3Int_h::Fts3Table,
    pbOk: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut cksum1: crate::src::ext::rtree::rtree::U64_0 =
        0 as crate::src::ext::rtree::rtree::U64_0;
    let mut cksum2: crate::src::ext::rtree::rtree::U64_0 =
        0 as crate::src::ext::rtree::rtree::U64_0;
    let mut pAllLangid: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    rc = fts3SqlStmt(
        p,
        SqlConstant::SqlSelectAllLangid as ::core::ffi::c_int,
        &raw mut pAllLangid,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        
        crate::src::src::vdbeapi::sqlite3_bind_int(
            pAllLangid,
            1 as ::core::ffi::c_int,
            (*p).iPrevLangid,
        );
        crate::src::src::vdbeapi::sqlite3_bind_int(
            pAllLangid,
            2 as ::core::ffi::c_int,
            (*p).nIndex,
        );
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && crate::src::src::vdbeapi::sqlite3_step(pAllLangid)
                == crate::src::headers::sqlite3_h::SQLITE_ROW
        {
            let iLangid: ::core::ffi::c_int =
                crate::src::src::vdbeapi::sqlite3_column_int(pAllLangid, 0 as ::core::ffi::c_int);
            let mut i: ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nIndex {
                cksum1 ^= fts3ChecksumIndex(p, iLangid, i, &raw mut rc);
                i += 1;
            }
        }
        let rc2: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_reset(pAllLangid);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let pModule: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module =
            (*(*p).pTokenizer).pModule;
        let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        let zSql: *mut ::core::ffi::c_char = sqlite_printf!("SELECT %s", (*p).zReadExprlist);
        if zSql.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            rc = crate::src::src::prepare::sqlite3_prepare_v2(
                (*p).db,
                zSql,
                -(1 as ::core::ffi::c_int),
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
        }
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && crate::src::headers::sqlite3_h::SQLITE_ROW
                == crate::src::src::vdbeapi::sqlite3_step(pStmt)
        {
            let iDocid: crate::src::ext::rtree::rtree::I64_0 =
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::I64_0;
            let iLang: ::core::ffi::c_int = langidFromSelect(p, pStmt);
            let mut iCol: ::core::ffi::c_int;
            iCol = 0 as ::core::ffi::c_int;
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && iCol < (*p).nColumn {
                if *(*p).abNotindexed.offset(iCol as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let zText: *const ::core::ffi::c_char =
                        crate::src::src::vdbeapi::sqlite3_column_text(
                            pStmt,
                            iCol + 1 as ::core::ffi::c_int,
                        ) as *const ::core::ffi::c_char;
                    let mut pT: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
                        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
                    rc = crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer(
                        
                        (*p).pTokenizer as
    *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                        iLang,
                        zText,
                        -(1 as ::core::ffi::c_int),
                        
                        &raw mut pT as *mut _ as
    *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                    );
                    while rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        let mut zToken: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut iDum1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut iDum2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut iPos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        rc = (*pModule).xNext.expect("non-null function pointer")(
                            pT,
                            &raw mut zToken,
                            &raw mut nToken,
                            &raw mut iDum1,
                            &raw mut iDum2,
                            &raw mut iPos,
                        );
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            let mut i_0: ::core::ffi::c_int;
                            cksum2 ^= fts3ChecksumEntry(
                                zToken,
                                nToken,
                                iLang,
                                0 as ::core::ffi::c_int,
                                iDocid,
                                iCol,
                                iPos,
                            );
                            i_0 = 1 as ::core::ffi::c_int;
                            while i_0 < (*p).nIndex {
                                if (*(*p).aIndex.offset(i_0 as isize)).nPrefix <= nToken {
                                    cksum2 ^= fts3ChecksumEntry(
                                        zToken,
                                        (*(*p).aIndex.offset(i_0 as isize)).nPrefix,
                                        iLang,
                                        i_0,
                                        iDocid,
                                        iCol,
                                        iPos,
                                    );
                                }
                                i_0 += 1;
                            }
                        }
                    }
                    if !pT.is_null() {
                        (*pModule).xClose.expect("non-null function pointer")(pT);
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                    }
                }
                iCol += 1;
            }
        }
        crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        *pbOk = 0 as ::core::ffi::c_int;
    } else {
        *pbOk = (rc == crate::src::headers::sqlite3_h::SQLITE_OK && cksum1 == cksum2)
            as ::core::ffi::c_int;
    }
    rc
}

unsafe extern "C" fn fts3DoIntegrityCheck(
    p: *mut crate::fts3Int_h::Fts3Table,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut bOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = sqlite3Fts3IntegrityCheck(p, &raw mut bOk);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && bOk == 0 as ::core::ffi::c_int {
        rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    rc
}

unsafe extern "C" fn fts3SpecialInsert(
    p: *mut crate::fts3Int_h::Fts3Table,
    pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    let zVal: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(pVal) as *const ::core::ffi::c_char;
    let nVal: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_value_bytes(pVal);
    if zVal.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else if nVal == 8 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3_strnicmp(
                zVal,
                b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
                8 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoOptimize(p, 0 as ::core::ffi::c_int);
    } else if nVal == 7 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3_strnicmp(
                zVal,
                b"rebuild\0" as *const u8 as *const ::core::ffi::c_char,
                7 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoRebuild(p);
    } else if nVal == 15 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3_strnicmp(
                zVal,
                b"integrity-check\0" as *const u8 as *const ::core::ffi::c_char,
                15 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoIntegrityCheck(p);
    } else if nVal > 6 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3_strnicmp(
                zVal,
                b"merge=\0" as *const u8 as *const ::core::ffi::c_char,
                6 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoIncrmerge(p, zVal.offset(6_isize) as *const ::core::ffi::c_char);
    } else if nVal > 10 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3_strnicmp(
                zVal,
                b"automerge=\0" as *const u8 as *const ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoAutoincrmerge(p, zVal.offset(10_isize) as *const ::core::ffi::c_char);
    } else if nVal == 5 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3_strnicmp(
                zVal,
                b"flush\0" as *const u8 as *const ::core::ffi::c_char,
                5 as ::core::ffi::c_int,
            )
    {
        rc = sqlite3Fts3PendingTermsFlush(p);
    } else {
        let v: ::core::ffi::c_int;
        if nVal > 9 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == crate::src::src::util::sqlite3_strnicmp(
                    zVal,
                    b"nodesize=\0" as *const u8 as *const ::core::ffi::c_char,
                    9 as ::core::ffi::c_int,
                )
        {
            v = atoi(zVal.offset(9_isize) as *const ::core::ffi::c_char);
            if v >= 24 as ::core::ffi::c_int && v <= (*p).nPgsz - 35 as ::core::ffi::c_int {
                (*p).nNodeSize = v;
            }
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        } else if nVal > 11 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == crate::src::src::util::sqlite3_strnicmp(
                    zVal,
                    b"maxpending=\0" as *const u8 as *const ::core::ffi::c_char,
                    9 as ::core::ffi::c_int,
                )
        {
            v = atoi(zVal.offset(11_isize) as *const ::core::ffi::c_char);
            if v >= 64 as ::core::ffi::c_int && v <= crate::fts3Int_h::FTS3_MAX_PENDING_DATA {
                (*p).nMaxPendingData = v;
            }
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        } else if nVal > 21 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == crate::src::src::util::sqlite3_strnicmp(
                    zVal,
                    b"test-no-incr-doclist=\0" as *const u8 as *const ::core::ffi::c_char,
                    21 as ::core::ffi::c_int,
                )
        {
            (*p).bNoIncrDoclist = atoi(zVal.offset(21_isize) as *const ::core::ffi::c_char);
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        } else if nVal > 11 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == crate::src::src::util::sqlite3_strnicmp(
                    zVal,
                    b"mergecount=\0" as *const u8 as *const ::core::ffi::c_char,
                    11 as ::core::ffi::c_int,
                )
        {
            v = atoi(zVal.offset(11_isize) as *const ::core::ffi::c_char);
            if v >= 4 as ::core::ffi::c_int
                && v <= crate::fts3Int_h::FTS3_MERGE_COUNT
                && v & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                (*p).nMergeCount = v;
            }
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3FreeDeferredDoclists(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
) {
    let mut pDef: *mut Fts3DeferredToken;
    pDef = (*pCsr).pDeferred;
    while !pDef.is_null() {
        fts3PendingListDelete((*pDef).pList);
        (*pDef).pList = ::core::ptr::null_mut::<PendingList>();
        pDef = (*pDef).pNext;
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3FreeDeferredTokens(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
) {
    let mut pDef: *mut Fts3DeferredToken;
    let mut pNext: *mut Fts3DeferredToken;
    pDef = (*pCsr).pDeferred;
    while !pDef.is_null() {
        pNext = (*pDef).pNext;
        fts3PendingListDelete((*pDef).pList);
        crate::src::src::malloc::sqlite3_free(pDef as *mut ::core::ffi::c_void);
        pDef = pNext;
    }
    (*pCsr).pDeferred = ::core::ptr::null_mut::<Fts3DeferredToken>();
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3CacheDeferredDoclists(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !(*pCsr).pDeferred.is_null() {
        let mut i: ::core::ffi::c_int;
        
        let mut pDef: *mut Fts3DeferredToken;
        let __pCsr_ref = unsafe { &*pCsr };
        let p = &*(__pCsr_ref.base.pVtab as *mut crate::fts3Int_h::Fts3Table);
        let pT: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer = p.pTokenizer;
        let pModule: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module =
            (*pT).pModule;
        let iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64 = crate::src::src::vdbeapi::sqlite3_column_int64(
            __pCsr_ref.pStmt,
            0 as ::core::ffi::c_int,
        );
        i = 0 as ::core::ffi::c_int;
        while i < p.nColumn && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if *p.abNotindexed.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let zText: *const ::core::ffi::c_char =
                    crate::src::src::vdbeapi::sqlite3_column_text(
                        __pCsr_ref.pStmt,
                        i + 1 as ::core::ffi::c_int,
                    ) as *const ::core::ffi::c_char;
                let mut pTC: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
                    ::core::ptr::null_mut::<
                        crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                    >();
                rc = crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer(
                    
                    pT as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                    __pCsr_ref.iLangid,
                    zText,
                    -(1 as ::core::ffi::c_int),
                    
                    &raw mut pTC as *mut _ as
    *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                );
                while rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    let mut zToken: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iDum1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iDum2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iPos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    rc = (*pModule).xNext.expect("non-null function pointer")(
                        pTC,
                        &raw mut zToken,
                        &raw mut nToken,
                        &raw mut iDum1,
                        &raw mut iDum2,
                        &raw mut iPos,
                    );
                    pDef = __pCsr_ref.pDeferred;
                    while !pDef.is_null() && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        let pPT: *mut crate::fts3Int_h::Fts3PhraseToken = (*pDef).pToken;
                        let __pPT_ref = unsafe { &*pPT };
                        if ((*pDef).iCol >= p.nColumn || (*pDef).iCol == i)
                            && (__pPT_ref.bFirst == 0 as ::core::ffi::c_int
                                || iPos == 0 as ::core::ffi::c_int)
                            && (__pPT_ref.n == nToken
                                || __pPT_ref.isPrefix != 0 && __pPT_ref.n < nToken)
                            && 0 as ::core::ffi::c_int
                                == fts3_memcmp_safe(
                                    zToken as *const ::core::ffi::c_void,
                                    __pPT_ref.z as *const ::core::ffi::c_void,
                                    __pPT_ref.n as usize,
                                )
                        {
                            fts3PendingListAppend(
                                &raw mut (*pDef).pList,
                                iDocid,
                                i as crate::src::headers::sqlite3_h::Sqlite3Int64,
                                iPos as crate::src::headers::sqlite3_h::Sqlite3Int64,
                                &raw mut rc,
                            );
                        }
                        pDef = (*pDef).pNext;
                    }
                }
                if !pTC.is_null() {
                    (*pModule).xClose.expect("non-null function pointer")(pTC);
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                    rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                }
            }
            i += 1;
        }
        pDef = __pCsr_ref.pDeferred;
        while !pDef.is_null() && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if !(*pDef).pList.is_null() {
                rc = fts3PendingListAppendVarint(
                    &raw mut (*pDef).pList,
                    0 as crate::src::headers::sqlite3_h::Sqlite3Int64,
                );
            }
            pDef = (*pDef).pNext;
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3DeferredTokenList(
    p: *mut Fts3DeferredToken,
    ppData: *mut *mut ::core::ffi::c_char,
    pnData: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    
    let mut dummy: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
    *ppData = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *pnData = 0 as ::core::ffi::c_int;
    let __p_ref = unsafe { &mut *p };
    if __p_ref.pList.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    let pRet: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_malloc64(
        (*__p_ref.pList).nData as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut ::core::ffi::c_char;
    if pRet.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    let nSkip: ::core::ffi::c_int = crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint((*__p_ref.pList).aData, &raw mut dummy);
    *pnData = (*__p_ref.pList).nData - nSkip;
    *ppData = pRet;
    ::core::ptr::copy_nonoverlapping(
        (*__p_ref.pList).aData.offset(nSkip as isize) as *mut ::core::ffi::c_char as *const u8,
        pRet as *mut u8,
        *pnData as usize,
    );
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3DeferToken(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pToken: *mut crate::fts3Int_h::Fts3PhraseToken,
    iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let pDeferred: *mut Fts3DeferredToken = crate::src::src::malloc::sqlite3_malloc64(::core::mem::size_of::<Fts3DeferredToken>()
            as crate::src::headers::sqlite3_h::Sqlite3Uint64) as *mut Fts3DeferredToken;
    if pDeferred.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    std::ptr::write_bytes(
        pDeferred as *mut ::core::ffi::c_void as *mut u8,
        0 as ::core::ffi::c_int as u8,
        ::core::mem::size_of::<Fts3DeferredToken>() as crate::__stddef_size_t_h::SizeT,
    );
    (*pDeferred).pToken = pToken;
    (*pDeferred).pNext = (*pCsr).pDeferred;
    (*pDeferred).iCol = iCol;
    (*pCsr).pDeferred = pDeferred;
    (*pToken).pDeferred = pDeferred;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3DeleteByRowid(
    p: *mut crate::fts3Int_h::Fts3Table,
    mut pRowid: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pnChng: *mut ::core::ffi::c_int,
    aSzDel: *mut crate::src::ext::rtree::rtree::U32_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut bFound: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    fts3DeleteTerms(&raw mut rc, p, pRowid, aSzDel, &raw mut bFound);
    if bFound != 0 && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut isEmpty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = fts3IsEmpty(p, pRowid, &raw mut isEmpty);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if isEmpty != 0 {
                rc = fts3DeleteAll(p, 1 as ::core::ffi::c_int);
                *pnChng = 0 as ::core::ffi::c_int;
                std::ptr::write_bytes(
                    aSzDel as *mut ::core::ffi::c_void as *mut u8,
                    0 as ::core::ffi::c_int as u8,
                    (::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                        as crate::__stddef_size_t_h::SizeT)
                        .wrapping_mul(
                            ((*p).nColumn + 1 as ::core::ffi::c_int)
                                as crate::__stddef_size_t_h::SizeT,
                        )
                        .wrapping_mul(2 as crate::__stddef_size_t_h::SizeT),
                );
            } else {
                *pnChng -= 1 as ::core::ffi::c_int;
                if (*p).zContentTbl.is_null() {
                    fts3SqlExec(
                        &raw mut rc,
                        p,
                        SqlConstant::SqlDeleteContent as ::core::ffi::c_int,
                        &raw mut pRowid,
                    );
                }
                if (*p).bHasDocsize != 0 {
                    fts3SqlExec(
                        &raw mut rc,
                        p,
                        SqlConstant::SqlDeleteDocsize as ::core::ffi::c_int,
                        &raw mut pRowid,
                    );
                }
            }
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3UpdateMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    nArg: ::core::ffi::c_int,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pRowid: *mut crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let p: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut rc: ::core::ffi::c_int;
    let aSzIns: *mut crate::src::ext::rtree::rtree::U32_0;
    let mut aSzDel: *mut crate::src::ext::rtree::rtree::U32_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U32_0>();
    let mut nChng: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bInsertDone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nArg > 1 as ::core::ffi::c_int
        && crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(0_isize))
            == crate::src::headers::sqlite3_h::SQLITE_NULL
        && crate::src::src::vdbeapi::sqlite3_value_type(
            *apVal.offset(((*p).nColumn + 2 as ::core::ffi::c_int) as isize),
        ) != crate::src::headers::sqlite3_h::SQLITE_NULL
    {
        rc = fts3SpecialInsert(
            p,
            *apVal.offset(((*p).nColumn + 2 as ::core::ffi::c_int) as isize),
        );
    } else if nArg > 1 as ::core::ffi::c_int
        && crate::src::src::vdbeapi::sqlite3_value_int(
            *apVal.offset(
                (2 as ::core::ffi::c_int + (*p).nColumn + 2 as ::core::ffi::c_int) as isize,
            ),
        ) < 0 as ::core::ffi::c_int
    {
        rc = crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
    } else {
        aSzDel = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                as crate::src::headers::sqlite3_h::Sqlite3Uint64)
                .wrapping_mul(
                    ((*p).nColumn as crate::src::headers::sqlite3_h::Sqlite3Int64
                        + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64)
                        as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                )
                .wrapping_mul(2 as crate::src::headers::sqlite3_h::Sqlite3Uint64),
        ) as *mut crate::src::ext::rtree::rtree::U32_0;
        if aSzDel.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            aSzIns = aSzDel.offset(((*p).nColumn + 1 as ::core::ffi::c_int) as isize)
                as *mut crate::src::ext::rtree::rtree::U32_0;
            std::ptr::write_bytes(
                aSzDel as *mut ::core::ffi::c_void as *mut u8,
                0 as ::core::ffi::c_int as u8,
                (::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                    as crate::__stddef_size_t_h::SizeT)
                    .wrapping_mul(
                        ((*p).nColumn + 1 as ::core::ffi::c_int)
                            as crate::__stddef_size_t_h::SizeT,
                    )
                    .wrapping_mul(2 as crate::__stddef_size_t_h::SizeT),
            );
            rc = fts3Writelock(p);
            if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                if nArg > 1 as ::core::ffi::c_int && (*p).zContentTbl.is_null() {
                    let mut pNewRowid: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                        *apVal.offset((3 as ::core::ffi::c_int + (*p).nColumn) as isize);
                    if crate::src::src::vdbeapi::sqlite3_value_type(pNewRowid)
                        == crate::src::headers::sqlite3_h::SQLITE_NULL
                    {
                        pNewRowid = *apVal.offset(1_isize);
                    }
                    if crate::src::src::vdbeapi::sqlite3_value_type(pNewRowid)
                        != crate::src::headers::sqlite3_h::SQLITE_NULL
                        && (crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(0_isize))
                            == crate::src::headers::sqlite3_h::SQLITE_NULL
                            || crate::src::src::vdbeapi::sqlite3_value_int64(
                                *apVal.offset(0_isize),
                            ) != crate::src::src::vdbeapi::sqlite3_value_int64(pNewRowid))
                    {
                        if crate::src::src::vtab::sqlite3_vtab_on_conflict((*p).db)
                            == crate::src::headers::sqlite3_h::SQLITE_REPLACE
                        {
                            rc = fts3DeleteByRowid(p, pNewRowid, &raw mut nChng, aSzDel);
                        } else {
                            rc = fts3InsertData(
                                p,
                                apVal,
                                pRowid as *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
                            );
                            bInsertDone = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                    if crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(0_isize))
                        != crate::src::headers::sqlite3_h::SQLITE_NULL
                    {
                        rc =
                            fts3DeleteByRowid(p, *apVal.offset(0_isize), &raw mut nChng, aSzDel);
                    }
                    if nArg > 1 as ::core::ffi::c_int
                        && rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    {
                        let iLangid: ::core::ffi::c_int =
                            crate::src::src::vdbeapi::sqlite3_value_int(*apVal.offset(
                                (2 as ::core::ffi::c_int + (*p).nColumn + 2 as ::core::ffi::c_int)
                                    as isize,
                            ));
                        if bInsertDone == 0 as ::core::ffi::c_int {
                            rc = fts3InsertData(
                                p,
                                apVal,
                                pRowid as *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
                            );
                            if rc == crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT
                                && (*p).zContentTbl.is_null()
                            {
                                rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
                            }
                        }
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            rc =
                                fts3PendingTermsDocid(p, 0 as ::core::ffi::c_int, iLangid, *pRowid);
                        }
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            rc = fts3InsertTerms(p, iLangid, apVal, aSzIns);
                        }
                        if (*p).bHasDocsize != 0 {
                            fts3InsertDocsize(&raw mut rc, p, aSzIns);
                        }
                        nChng += 1;
                    }
                    if (*p).bFts4 != 0 {
                        fts3UpdateDocTotals(&raw mut rc, p, aSzIns, aSzDel, nChng);
                    }
                }
            }
        }
    }
    crate::src::src::malloc::sqlite3_free(aSzDel as *mut ::core::ffi::c_void);
    sqlite3Fts3SegmentsClose(p);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3Optimize(
    p: *mut crate::fts3Int_h::Fts3Table,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    rc = crate::src::src::legacy::sqlite3_exec(
        (*p).db,
        b"SAVEPOINT fts3\0" as *const u8 as *const ::core::ffi::c_char,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = fts3DoOptimize(p, 1 as ::core::ffi::c_int);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK
            || rc == crate::src::headers::sqlite3_h::SQLITE_DONE
        {
            let rc2: ::core::ffi::c_int = crate::src::src::legacy::sqlite3_exec(
                (*p).db,
                b"RELEASE fts3\0" as *const u8 as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            if rc2 != crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = rc2;
            }
        } else {
            crate::src::src::legacy::sqlite3_exec(
                (*p).db,
                b"ROLLBACK TO fts3\0" as *const u8 as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            crate::src::src::legacy::sqlite3_exec(
                (*p).db,
                b"RELEASE fts3\0" as *const u8 as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
    }
    sqlite3Fts3SegmentsClose(p);
    rc
}
