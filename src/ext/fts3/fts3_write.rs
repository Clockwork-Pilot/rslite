

pub mod stdlib_h {
    
    #[inline]
    
    pub unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
        ::libc::strtol(
            __nptr,
            crate::__stddef_null_h::NULL as *mut *mut ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
    }
    
    
    
}






pub use crate::__stddef_null_h::NULL;pub use crate::__stddef_null_h::NULL_0;
pub use crate::__stddef_size_t_h::size_t;





pub use crate::src::fts5::i16_0;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::ext::fts3::fts3::sqlite3Fts3CreateStatTable;pub use crate::src::ext::fts3::fts3::sqlite3Fts3DoclistPrev;pub use crate::src::ext::fts3::fts3::sqlite3Fts3FirstFilter;pub use crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint;pub use crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32;pub use crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintU;pub use crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer;pub use crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint;pub use crate::src::ext::fts3::fts3::sqlite3Fts3SegReaderCursor;pub use crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::fts3Int_h::Fts3Cursor;pub use crate::fts3Int_h::Fts3Doclist;pub use crate::fts3Int_h::Fts3Expr;pub use crate::fts3Int_h::Fts3Index;pub use crate::fts3Int_h::Fts3MultiSegReader;pub use crate::fts3Int_h::Fts3Phrase;pub use crate::fts3Int_h::Fts3PhraseToken;pub use crate::fts3Int_h::Fts3SegFilter;pub use crate::fts3Int_h::Fts3Table;pub use crate::fts3Int_h::MatchinfoBuffer;pub use crate::fts3Int_h::FTS3_MAX_PENDING_DATA;pub use crate::fts3Int_h::FTS3_MERGE_COUNT;pub use crate::fts3Int_h::FTS3_SEGCURSOR_ALL;pub use crate::fts3Int_h::FTS3_SEGCURSOR_PENDING;pub use crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL;pub use crate::fts3Int_h::FTS3_SEGMENT_COLUMN_FILTER;pub use crate::fts3Int_h::FTS3_SEGMENT_FIRST;pub use crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY;pub use crate::fts3Int_h::FTS3_SEGMENT_PREFIX;pub use crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS;pub use crate::fts3Int_h::FTS3_SEGMENT_SCAN;pub use crate::fts3Int_h::FTS3_VARINT_MAX;pub use crate::fts3Int_h::FTS_CORRUPT_VTAB;pub use crate::fts3Int_h::LARGEST_INT64;pub use crate::src::ext::fts3::fts3_hash::Fts3Hash;pub use crate::src::ext::fts3::fts3_hash::Fts3HashElem;pub use crate::src::ext::fts3::fts3_hash::_fts3ht;pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashClear;pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFind;pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFindElem;pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInsert;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;pub use crate::src::headers::sqliteInt_h::sqlite3;pub use crate::src::src::vdbeapi::sqlite3_bind_blob;pub use crate::src::src::vdbeapi::sqlite3_bind_int;pub use crate::src::src::vdbeapi::sqlite3_bind_int64;pub use crate::src::src::vdbeapi::sqlite3_bind_null;pub use crate::src::src::vdbeapi::sqlite3_bind_parameter_count;pub use crate::src::src::vdbeapi::sqlite3_bind_text;pub use crate::src::src::vdbeapi::sqlite3_bind_value;pub use crate::src::headers::sqlite3_h::sqlite3_blob;pub use crate::src::src::vdbeblob::sqlite3_blob_bytes;pub use crate::src::src::vdbeblob::sqlite3_blob_close;pub use crate::src::src::vdbeblob::sqlite3_blob_open;pub use crate::src::src::vdbeblob::sqlite3_blob_read;pub use crate::src::src::vdbeblob::sqlite3_blob_reopen;pub use crate::src::src::vdbeapi::sqlite3_column_blob;pub use crate::src::src::vdbeapi::sqlite3_column_bytes;pub use crate::src::src::vdbeapi::sqlite3_column_int;pub use crate::src::src::vdbeapi::sqlite3_column_int64;pub use crate::src::src::vdbeapi::sqlite3_column_text;pub use crate::src::src::vdbeapi::sqlite3_column_type;pub use crate::src::headers::vdbeInt_h::sqlite3_context;pub use crate::src::headers::sqlite3_h::sqlite3_destructor_type;pub use crate::src::src::legacy::sqlite3_exec;pub use crate::src::src::vdbeapi::sqlite3_finalize;pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::src::headers::sqlite3_h::sqlite3_index_info;pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::src::main::sqlite3_last_insert_rowid;pub use crate::src::src::malloc::sqlite3_malloc64;pub use crate::src::headers::sqlite3_h::sqlite3_module;pub use crate::src::src::printf::sqlite3_mprintf;pub use crate::src::src::prepare::sqlite3_prepare_v2;pub use crate::src::src::prepare::sqlite3_prepare_v3;pub use crate::src::src::malloc::sqlite3_realloc64;pub use crate::src::src::vdbeapi::sqlite3_reset;pub use crate::src::src::vdbeapi::sqlite3_step;pub use crate::src::headers::sqlite3_h::sqlite3_stmt;pub use crate::src::src::util::sqlite3_strnicmp;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::headers::vdbeInt_h::sqlite3_value;pub use crate::src::src::vdbeapi::sqlite3_value_bytes;pub use crate::src::src::vdbeapi::sqlite3_value_int;pub use crate::src::src::vdbeapi::sqlite3_value_int64;pub use crate::src::src::vdbeapi::sqlite3_value_text;pub use crate::src::src::vdbeapi::sqlite3_value_type;pub use crate::src::headers::sqlite3_h::sqlite3_vtab;pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;pub use crate::src::src::vtab::sqlite3_vtab_on_conflict;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_BLOB;pub use crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT;pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;pub use crate::src::headers::sqlite3_h::SQLITE_DONE;pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;pub use crate::src::headers::sqlite3_h::SQLITE_INTEGER;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_NULL;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::headers::sqlite3_h::SQLITE_PREPARE_NO_VTAB;pub use crate::src::headers::sqlite3_h::SQLITE_PREPARE_PERSISTENT;pub use crate::src::headers::sqlite3_h::SQLITE_REPLACE;pub use crate::src::headers::sqlite3_h::SQLITE_ROW;pub use crate::src::headers::sqlite3_h::SQLITE_STATIC;pub use crate::stdlib::__compar_fn_t;pub use crate::src::ext::fts3::fts3_write::stdlib_h::atoi;pub use ::libc::qsort;pub use ::libc::strtol;
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
    pub iLastDocid: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub iLastCol: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub iLastPos: crate::src::headers::sqlite3_h::sqlite3_int64,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct Fts3SegReader {
    pub iIdx: ::core::ffi::c_int,
    pub bLookup: crate::src::ext::rtree::rtree::u8_0,
    pub rootOnly: crate::src::ext::rtree::rtree::u8_0,
    pub iStartBlock: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub iLeafEndBlock: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub iEndBlock: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub iCurrentBlock: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub aNode: *mut ::core::ffi::c_char,
    pub nNode: ::core::ffi::c_int,
    pub nPopulate: ::core::ffi::c_int,
    pub pBlob: *mut crate::src::headers::sqlite3_h::sqlite3_blob,
    pub ppNextElem: *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
    pub nTerm: ::core::ffi::c_int,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTermAlloc: ::core::ffi::c_int,
    pub aDoclist: *mut ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
    pub pOffsetList: *mut ::core::ffi::c_char,
    pub nOffsetList: ::core::ffi::c_int,
    pub iDocid: crate::src::headers::sqlite3_h::sqlite3_int64,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct SegmentWriter {
    pub pTree: *mut SegmentNode,
    pub iFirst: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub iFree: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTerm: ::core::ffi::c_int,
    pub nMalloc: ::core::ffi::c_int,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub nSize: ::core::ffi::c_int,
    pub nData: ::core::ffi::c_int,
    pub aData: *mut ::core::ffi::c_char,
    pub nLeafData: crate::src::ext::rtree::rtree::i64_0,
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
    pub nLeafEst: crate::src::ext::rtree::rtree::i64_0,
    pub nWork: crate::src::ext::rtree::rtree::i64_0,
    pub iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub iIdx: ::core::ffi::c_int,
    pub iStart: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub iEnd: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub nLeafData: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub bNoLeafData: crate::src::ext::rtree::rtree::u8_0,
    pub aNodeWriter: [NodeWriter; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct NodeWriter {
    pub iBlock: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub key: Blob,
    pub block: Blob,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct NodeReader {
    pub aNode: *const ::core::ffi::c_char,
    pub nNode: ::core::ffi::c_int,
    pub iOff: ::core::ffi::c_int,
    pub iChild: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub term: Blob,
    pub aDoclist: *const ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
}

pub const FTS_MAX_APPENDABLE_HEIGHT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const FTS3_NODE_PADDING: ::core::ffi::c_int = crate::fts3Int_h::FTS3_VARINT_MAX * 2 as ::core::ffi::c_int;
#[no_mangle]

pub static mut test_fts3_node_chunksize: ::core::ffi::c_int =
    4 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
#[no_mangle]

pub static mut test_fts3_node_chunk_threshold: ::core::ffi::c_int =
    4 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;

pub const FTS_STAT_DOCTOTAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const FTS_STAT_INCRMERGEHINT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const FTS_STAT_AUTOINCRMERGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQL_DELETE_CONTENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQL_IS_EMPTY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQL_DELETE_ALL_CONTENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQL_DELETE_ALL_SEGMENTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQL_DELETE_ALL_SEGDIR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQL_DELETE_ALL_DOCSIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQL_DELETE_ALL_STAT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const SQL_SELECT_CONTENT_BY_ROWID: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const SQL_NEXT_SEGMENT_INDEX: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const SQL_INSERT_SEGMENTS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const SQL_NEXT_SEGMENTS_ID: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const SQL_INSERT_SEGDIR: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const SQL_SELECT_LEVEL: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

pub const SQL_SELECT_LEVEL_RANGE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;

pub const SQL_SELECT_SEGDIR_MAX_LEVEL: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

pub const SQL_DELETE_SEGDIR_LEVEL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const SQL_DELETE_SEGMENTS_RANGE: ::core::ffi::c_int = 17 as ::core::ffi::c_int;

pub const SQL_CONTENT_INSERT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;

pub const SQL_DELETE_DOCSIZE: ::core::ffi::c_int = 19 as ::core::ffi::c_int;

pub const SQL_REPLACE_DOCSIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;

pub const SQL_SELECT_DOCSIZE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;

pub const SQL_SELECT_STAT: ::core::ffi::c_int = 22 as ::core::ffi::c_int;

pub const SQL_REPLACE_STAT: ::core::ffi::c_int = 23 as ::core::ffi::c_int;

pub const SQL_DELETE_SEGDIR_RANGE: ::core::ffi::c_int = 26 as ::core::ffi::c_int;

pub const SQL_SELECT_ALL_LANGID: ::core::ffi::c_int = 27 as ::core::ffi::c_int;

pub const SQL_FIND_MERGE_LEVEL: ::core::ffi::c_int = 28 as ::core::ffi::c_int;

pub const SQL_MAX_LEAF_NODE_ESTIMATE: ::core::ffi::c_int = 29 as ::core::ffi::c_int;

pub const SQL_DELETE_SEGDIR_ENTRY: ::core::ffi::c_int = 30 as ::core::ffi::c_int;

pub const SQL_SHIFT_SEGDIR_ENTRY: ::core::ffi::c_int = 31 as ::core::ffi::c_int;

pub const SQL_SELECT_SEGDIR: ::core::ffi::c_int = 32 as ::core::ffi::c_int;

pub const SQL_CHOMP_SEGDIR: ::core::ffi::c_int = 33 as ::core::ffi::c_int;

pub const SQL_SEGMENT_IS_APPENDABLE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;

pub const SQL_SELECT_INDEXES: ::core::ffi::c_int = 35 as ::core::ffi::c_int;

pub const SQL_SELECT_MXLEVEL: ::core::ffi::c_int = 36 as ::core::ffi::c_int;

pub const SQL_SELECT_LEVEL_RANGE2: ::core::ffi::c_int = 37 as ::core::ffi::c_int;

pub const SQL_UPDATE_LEVEL_IDX: ::core::ffi::c_int = 38 as ::core::ffi::c_int;

pub const SQL_UPDATE_LEVEL: ::core::ffi::c_int = 39 as ::core::ffi::c_int;

unsafe extern "C" fn fts3SqlStmt(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut eStmt: ::core::ffi::c_int,
    mut pp: *mut *mut crate::src::headers::sqlite3_h::sqlite3_stmt,
    mut apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut azSql: [*const ::core::ffi::c_char; 40] = [
        b"DELETE FROM %Q.'%q_content' WHERE rowid = ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT NOT EXISTS(SELECT docid FROM %Q.'%q_content' WHERE rowid!=?)\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_content'\0" as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segments'\0" as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segdir'\0" as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_docsize'\0" as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_stat'\0" as *const u8 as *const ::core::ffi::c_char,
        b"SELECT %s WHERE rowid=?\0" as *const u8 as *const ::core::ffi::c_char,
        b"SELECT (SELECT max(idx) FROM %Q.'%q_segdir' WHERE level = ?) + 1\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"REPLACE INTO %Q.'%q_segments'(blockid, block) VALUES(?, ?)\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT coalesce((SELECT max(blockid) FROM %Q.'%q_segments') + 1, 1)\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"REPLACE INTO %Q.'%q_segdir' VALUES(?,?,?,?,?,?)\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level = ? ORDER BY idx ASC\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?ORDER BY level DESC, idx ASC\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"SELECT count(*) FROM %Q.'%q_segdir' WHERE level = ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT max(level) FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segdir' WHERE level = ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segments' WHERE blockid BETWEEN ? AND ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"INSERT INTO %Q.'%q_content' VALUES(%s)\0" as *const u8
            as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_docsize' WHERE docid = ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"REPLACE INTO %Q.'%q_docsize' VALUES(?,?)\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT size FROM %Q.'%q_docsize' WHERE docid=?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT value FROM %Q.'%q_stat' WHERE id=?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"REPLACE INTO %Q.'%q_stat' VALUES(?,?)\0" as *const u8
            as *const ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT ? UNION SELECT level / (1024 * ?) FROM %Q.'%q_segdir'\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT level, count(*) AS cnt FROM %Q.'%q_segdir'   GROUP BY level HAVING cnt>=?  ORDER BY (level %% 1024) ASC, 2 DESC LIMIT 1\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"SELECT 2 * total(1 + leaves_end_block - start_block)   FROM (SELECT * FROM %Q.'%q_segdir'         WHERE level = ? ORDER BY idx ASC LIMIT ?  )\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"DELETE FROM %Q.'%q_segdir' WHERE level = ? AND idx = ?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"UPDATE %Q.'%q_segdir' SET idx = ? WHERE level=? AND idx=?\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level = ? AND idx = ?\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"UPDATE %Q.'%q_segdir' SET start_block = ?, root = ?WHERE level = ? AND idx = ?\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"SELECT 1 FROM %Q.'%q_segments' WHERE blockid=? AND block IS NULL\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"SELECT idx FROM %Q.'%q_segdir' WHERE level=? ORDER BY 1 ASC\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT max( level %% 1024 ) FROM %Q.'%q_segdir'\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT level, idx, end_block FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ? ORDER BY level DESC, idx ASC\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"UPDATE OR FAIL %Q.'%q_segdir' SET level=-1,idx=? WHERE level=? AND idx=?\0"
            as *const u8 as *const ::core::ffi::c_char,
        b"UPDATE OR FAIL %Q.'%q_segdir' SET level=? WHERE level=-1\0" as *const u8
            as *const ::core::ffi::c_char,
    ];
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    pStmt = (*p).aStmt[eStmt as usize];
    if pStmt.is_null() {
        let mut f: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_PREPARE_PERSISTENT | crate::src::headers::sqlite3_h::SQLITE_PREPARE_NO_VTAB;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if eStmt == SQL_CONTENT_INSERT {
            let __p_ref = unsafe { &*p };
            zSql = crate::src::src::printf::sqlite3_mprintf(
                azSql[eStmt as usize],
                __p_ref.zDb,
                __p_ref.zName,
                __p_ref.zWriteExprlist,
            );
        } else if eStmt == SQL_SELECT_CONTENT_BY_ROWID {
            f &= !crate::src::headers::sqlite3_h::SQLITE_PREPARE_NO_VTAB;
            zSql = crate::src::src::printf::sqlite3_mprintf(azSql[eStmt as usize], (*p).zReadExprlist);
        } else {
            zSql = crate::src::src::printf::sqlite3_mprintf(azSql[eStmt as usize], (*p).zDb, (*p).zName);
        }
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
        let mut i: ::core::ffi::c_int = 0;
        let mut nParam: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_bind_parameter_count(pStmt);
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
    mut pTab: *mut crate::fts3Int_h::Fts3Table,
    mut iDocid: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut ppStmt: *mut *mut crate::src::headers::sqlite3_h::sqlite3_stmt,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SqlStmt(
        pTab,
        SQL_SELECT_DOCSIZE,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iDocid);
        rc = crate::src::src::vdbeapi::sqlite3_step(pStmt);
        if rc != crate::src::headers::sqlite3_h::SQLITE_ROW || crate::src::src::vdbeapi::sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int) != crate::src::headers::sqlite3_h::SQLITE_BLOB {
            rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
            pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    *ppStmt = pStmt;
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3SelectDoctotal(
    mut pTab: *mut crate::fts3Int_h::Fts3Table,
    mut ppStmt: *mut *mut crate::src::headers::sqlite3_h::sqlite3_stmt,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SqlStmt(
        pTab,
        SQL_SELECT_STAT,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, FTS_STAT_DOCTOTAL);
        if crate::src::src::vdbeapi::sqlite3_step(pStmt) != crate::src::headers::sqlite3_h::SQLITE_ROW
            || crate::src::src::vdbeapi::sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int) != crate::src::headers::sqlite3_h::SQLITE_BLOB
        {
            rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
            pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        }
    }
    *ppStmt = pStmt;
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3SelectDocsize(
    mut pTab: *mut crate::fts3Int_h::Fts3Table,
    mut iDocid: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut ppStmt: *mut *mut crate::src::headers::sqlite3_h::sqlite3_stmt,
) -> ::core::ffi::c_int {
    fts3SelectDocsize(pTab, iDocid, ppStmt)
}

unsafe extern "C" fn fts3SqlExec(
    mut pRC: *mut ::core::ffi::c_int,
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut eStmt: ::core::ffi::c_int,
    mut apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
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

unsafe extern "C" fn fts3Writelock(mut p: *mut crate::fts3Int_h::Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*p).nPendingData == 0 as ::core::ffi::c_int {
        let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            SQL_DELETE_SEGDIR_LEVEL,
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
) -> crate::src::headers::sqlite3_h::sqlite3_int64 {
    let mut iBase: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
    iBase = (iLangid as crate::src::headers::sqlite3_h::sqlite3_int64 * (*p).nIndex as crate::src::headers::sqlite3_h::sqlite3_int64 + iIndex as crate::src::headers::sqlite3_h::sqlite3_int64)
        * crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL as crate::src::headers::sqlite3_h::sqlite3_int64;
    iBase + iLevel as crate::src::headers::sqlite3_h::sqlite3_int64
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3AllSegdirs(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
    mut ppStmt: *mut *mut crate::src::headers::sqlite3_h::sqlite3_stmt,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    if iLevel < 0 as ::core::ffi::c_int {
        rc = fts3SqlStmt(
            p,
            SQL_SELECT_LEVEL_RANGE,
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
            SQL_SELECT_LEVEL,
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
    mut pp: *mut *mut PendingList,
    mut i: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut p: *mut PendingList = *pp;
    if p.is_null() {
        p = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<PendingList>() as usize).wrapping_add(100 as usize)
                as crate::src::headers::sqlite3_h::sqlite3_uint64,
        ) as *mut PendingList;
        if p.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        (*p).nSpace = 100 as ::core::ffi::c_int;
        (*p).aData = p.offset(1 as isize) as *mut PendingList
            as *mut ::core::ffi::c_char;
        (*p).nData = 0 as ::core::ffi::c_int;
    } else if (*p).nData + crate::fts3Int_h::FTS3_VARINT_MAX + 1 as ::core::ffi::c_int > (*p).nSpace {
        let mut nNew: crate::src::ext::rtree::rtree::i64_0 = ((*p).nSpace * 2 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
        p = crate::src::src::malloc::sqlite3_realloc64(
            p as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<PendingList>() as crate::src::headers::sqlite3_h::sqlite3_uint64)
                .wrapping_add(nNew as crate::src::headers::sqlite3_h::sqlite3_uint64),
        ) as *mut PendingList;
        if p.is_null() {
            crate::src::src::malloc::sqlite3_free(*pp as *mut ::core::ffi::c_void);
            *pp = ::core::ptr::null_mut::<PendingList>();
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        (*p).nSpace = nNew as ::core::ffi::c_int;
        (*p).aData = p.offset(1 as isize) as *mut PendingList
            as *mut ::core::ffi::c_char;
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
    mut pp: *mut *mut PendingList,
    mut iDocid: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iCol: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iPos: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut p: *mut PendingList = *pp;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if p.is_null() || (*p).iLastDocid != iDocid {
        let mut iDelta: crate::src::ext::rtree::rtree::u64_0 = (iDocid as crate::src::ext::rtree::rtree::u64_0).wrapping_sub(
            (if !p.is_null() {
                (*p).iLastDocid
            } else {
                0 as crate::src::headers::sqlite3_h::sqlite3_int64
            }) as crate::src::ext::rtree::rtree::u64_0,
        );
        if !p.is_null() {
            (*p).nData += 1;
        }
        rc = fts3PendingListAppendVarint(&raw mut p, iDelta as crate::src::headers::sqlite3_h::sqlite3_int64);
        if crate::src::headers::sqlite3_h::SQLITE_OK != rc {
            current_block = 17264066162094073856;
        } else {
            let __p_ref = unsafe { &mut *p };
            __p_ref.iLastCol = -(1 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::sqlite3_int64;
            __p_ref.iLastPos = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
            __p_ref.iLastDocid = iDocid;
            current_block = 3640593987805443782;
        }
    } else {
        current_block = 3640593987805443782;
    }
    match current_block {
        3640593987805443782 => {
            if iCol > 0 as crate::src::headers::sqlite3_h::sqlite3_int64 && (*p).iLastCol != iCol {
                rc = fts3PendingListAppendVarint(&raw mut p, 1 as crate::src::headers::sqlite3_h::sqlite3_int64);
                if crate::src::headers::sqlite3_h::SQLITE_OK != rc || {
                    rc = fts3PendingListAppendVarint(&raw mut p, iCol);
                    crate::src::headers::sqlite3_h::SQLITE_OK != rc
                } {
                    current_block = 17264066162094073856;
                } else {
                    (*p).iLastCol = iCol;
                    (*p).iLastPos = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
                    current_block = 4166486009154926805;
                }
            } else {
                current_block = 4166486009154926805;
            }
            match current_block {
                17264066162094073856 => {}
                _ => {
                    if iCol >= 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
                        rc = fts3PendingListAppendVarint(
                            &raw mut p,
                            2 as crate::src::headers::sqlite3_h::sqlite3_int64 + iPos - (*p).iLastPos,
                        );
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            (*p).iLastPos = iPos;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    *pRc = rc;
    if p != *pp {
        *pp = p;
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn fts3PendingListDelete(mut pList: *mut PendingList) {
    crate::src::src::malloc::sqlite3_free(pList as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn fts3PendingTermsAddOne(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iCol: ::core::ffi::c_int,
    mut iPos: ::core::ffi::c_int,
    mut pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut zToken: *const ::core::ffi::c_char,
    mut nToken: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pList: *mut PendingList = ::core::ptr::null_mut::<PendingList>();
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    pList = crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFind(pHash as *const crate::src::ext::fts3::fts3_hash::Fts3Hash, zToken as *const ::core::ffi::c_void, nToken)
        as *mut PendingList;
    if !pList.is_null() {
        (*p).nPendingData = ((*p).nPendingData as ::core::ffi::c_ulong).wrapping_sub(
            (((*pList).nData + nToken) as usize)
                .wrapping_add(::core::mem::size_of::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>() as usize)
                as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    if fts3PendingListAppend(
        &raw mut pList,
        (*p).iPrevDocid as crate::src::headers::sqlite3_h::sqlite3_int64,
        iCol as crate::src::headers::sqlite3_h::sqlite3_int64,
        iPos as crate::src::headers::sqlite3_h::sqlite3_int64,
        &raw mut rc,
    ) != 0
    {
        if pList
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
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        (*p).nPendingData = ((*p).nPendingData as ::core::ffi::c_ulong).wrapping_add(
            (((*pList).nData + nToken) as usize)
                .wrapping_add(::core::mem::size_of::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>() as usize)
                as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    rc
}

unsafe extern "C" fn fts3PendingTermsAdd(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut zText: *const ::core::ffi::c_char,
    mut iCol: ::core::ffi::c_int,
    mut pnWord: *mut crate::src::ext::rtree::rtree::u32_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut iStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iPos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nWord: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut zToken: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer = (*p).pTokenizer;
    let mut pModule: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = (*pTokenizer).pModule;
    let mut pCsr: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
    let mut xNext: Option<
        unsafe extern "C" fn(
            *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    > = None;
    if zText.is_null() {
        *pnWord = 0 as crate::src::ext::rtree::rtree::u32_0;
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc = crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer(
        
        pTokenizer as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
        iLangid,
        zText,
        -(1 as ::core::ffi::c_int),
        
        &raw mut pCsr as *mut _ as
    *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    xNext = (*pModule).xNext;
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
        let mut i: ::core::ffi::c_int = 0;
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
                &raw mut (*(*p).aIndex.offset(0 as isize)).hPending,
                zToken,
                nToken,
            );
            i = 1 as ::core::ffi::c_int;
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < (*p).nIndex {
                let mut pIndex: *mut crate::fts3Int_h::Fts3Index = (*p).aIndex.offset(i as isize) as *mut crate::fts3Int_h::Fts3Index;
                if !(nToken < (*pIndex).nPrefix) {
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
    *pnWord = (*pnWord).wrapping_add(nWord as crate::src::ext::rtree::rtree::u32_0);
    if rc == crate::src::headers::sqlite3_h::SQLITE_DONE { crate::src::headers::sqlite3_h::SQLITE_OK } else { rc }
}

unsafe extern "C" fn fts3PendingTermsDocid(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut bDelete: ::core::ffi::c_int,
    mut iLangid: ::core::ffi::c_int,
    mut iDocid: crate::src::headers::sqlite3_h::sqlite_int64,
) -> ::core::ffi::c_int {
    let __p_ref = unsafe { &mut *p };
    if iDocid < __p_ref.iPrevDocid
        || iDocid == __p_ref.iPrevDocid && __p_ref.bPrevDelete == 0 as ::core::ffi::c_int
        || __p_ref.iPrevLangid != iLangid
        || __p_ref.nPendingData > __p_ref.nMaxPendingData
    {
        let mut rc: ::core::ffi::c_int = sqlite3Fts3PendingTermsFlush(p);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
    }
    __p_ref.iPrevDocid = iDocid;
    __p_ref.iPrevLangid = iLangid;
    __p_ref.bPrevDelete = bDelete;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3PendingTermsClear(mut p: *mut crate::fts3Int_h::Fts3Table) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nIndex {
        let mut pElem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
        let mut pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash = &raw mut (*(*p).aIndex.offset(i as isize)).hPending;
        pElem = (*pHash).first;
        while !pElem.is_null() {
            let mut pList: *mut PendingList = (*pElem).data as *mut PendingList;
            fts3PendingListDelete(pList);
            pElem = (*pElem).next;
        }
        crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashClear(pHash as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash);
        i += 1;
    }
    (*p).nPendingData = 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn fts3InsertTerms(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    mut aSz: *mut crate::src::ext::rtree::rtree::u32_0,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 2 as ::core::ffi::c_int;
    while i < (*p).nColumn + 2 as ::core::ffi::c_int {
        let mut iCol: ::core::ffi::c_int = i - 2 as ::core::ffi::c_int;
        if *(*p).abNotindexed.offset(iCol as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            let mut zText: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_value_text(*apVal.offset(i as isize)) as *const ::core::ffi::c_char;
            let mut rc: ::core::ffi::c_int = fts3PendingTermsAdd(
                p,
                iLangid,
                zText,
                iCol,
                aSz.offset(iCol as isize) as *mut crate::src::ext::rtree::rtree::u32_0,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            let ref mut fresh2 = *aSz.offset((*p).nColumn as isize);
            *fresh2 =
                (*fresh2).wrapping_add(crate::src::src::vdbeapi::sqlite3_value_bytes(*apVal.offset(i as isize)) as crate::src::ext::rtree::rtree::u32_0);
        }
        i += 1;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3InsertData(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    mut piDocid: *mut crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pContentInsert: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let __p_ref = unsafe { &mut *p };
    if !__p_ref.zContentTbl.is_null() {
        let mut pRowid: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
            *apVal.offset((__p_ref.nColumn + 3 as ::core::ffi::c_int) as isize);
        if crate::src::src::vdbeapi::sqlite3_value_type(pRowid) == crate::src::headers::sqlite3_h::SQLITE_NULL {
            pRowid = *apVal.offset(1 as isize);
        }
        if crate::src::src::vdbeapi::sqlite3_value_type(pRowid) != crate::src::headers::sqlite3_h::SQLITE_INTEGER {
            return crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
        }
        *piDocid = crate::src::src::vdbeapi::sqlite3_value_int64(pRowid);
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc = fts3SqlStmt(
        p,
        SQL_CONTENT_INSERT,
        &raw mut pContentInsert,
        apVal.offset(1 as isize) as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !__p_ref.zLanguageid.is_null() {
        rc = crate::src::src::vdbeapi::sqlite3_bind_int(
            pContentInsert,
            __p_ref.nColumn + 2 as ::core::ffi::c_int,
            crate::src::src::vdbeapi::sqlite3_value_int(*apVal.offset((__p_ref.nColumn + 4 as ::core::ffi::c_int) as isize)),
        );
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    if crate::src::headers::sqlite3_h::SQLITE_NULL
        != crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset((3 as ::core::ffi::c_int + __p_ref.nColumn) as isize))
    {
        if crate::src::headers::sqlite3_h::SQLITE_NULL == crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(0 as isize))
            && crate::src::headers::sqlite3_h::SQLITE_NULL != crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(1 as isize))
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut bContent: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    sqlite3Fts3PendingTermsClear(p);
    if bContent != 0 {
        fts3SqlExec(
            &raw mut rc,
            p,
            SQL_DELETE_ALL_CONTENT,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    fts3SqlExec(
        &raw mut rc,
        p,
        SQL_DELETE_ALL_SEGMENTS,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    fts3SqlExec(
        &raw mut rc,
        p,
        SQL_DELETE_ALL_SEGDIR,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if (*p).bHasDocsize != 0 {
        fts3SqlExec(
            &raw mut rc,
            p,
            SQL_DELETE_ALL_DOCSIZE,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    if (*p).bHasStat != 0 {
        fts3SqlExec(
            &raw mut rc,
            p,
            SQL_DELETE_ALL_STAT,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    rc
}

unsafe extern "C" fn langidFromSelect(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pSelect: *mut crate::src::headers::sqlite3_h::sqlite3_stmt,
) -> ::core::ffi::c_int {
    let mut iLangid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*p).zLanguageid.is_null() {
        iLangid = crate::src::src::vdbeapi::sqlite3_column_int(pSelect, (*p).nColumn + 1 as ::core::ffi::c_int);
    }
    iLangid
}

unsafe extern "C" fn fts3DeleteTerms(
    mut pRC: *mut ::core::ffi::c_int,
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pRowid: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    mut aSz: *mut crate::src::ext::rtree::rtree::u32_0,
    mut pbFound: *mut ::core::ffi::c_int,
) {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pSelect: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    if *pRC != 0 {
        return;
    }
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_CONTENT_BY_ROWID,
        &raw mut pSelect,
        &raw mut pRowid,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pSelect) {
            let mut i: ::core::ffi::c_int = 0;
            let mut iLangid: ::core::ffi::c_int = langidFromSelect(p, pSelect);
            let mut iDocid: crate::src::ext::rtree::rtree::i64_0 = crate::src::src::vdbeapi::sqlite3_column_int64(pSelect, 0 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
            rc = fts3PendingTermsDocid(p, 1 as ::core::ffi::c_int, iLangid, iDocid as crate::src::headers::sqlite3_h::sqlite_int64);
            i = 1 as ::core::ffi::c_int;
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i <= (*p).nColumn {
                let mut iCol: ::core::ffi::c_int = i - 1 as ::core::ffi::c_int;
                if *(*p).abNotindexed.offset(iCol as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let mut zText: *const ::core::ffi::c_char =
                        crate::src::src::vdbeapi::sqlite3_column_text(pSelect, i) as *const ::core::ffi::c_char;
                    rc = fts3PendingTermsAdd(
                        p,
                        iLangid,
                        zText,
                        -(1 as ::core::ffi::c_int),
                        aSz.offset(iCol as isize) as *mut crate::src::ext::rtree::rtree::u32_0,
                    );
                    let ref mut fresh11 = *aSz.offset((*p).nColumn as isize);
                    *fresh11 = (*fresh11).wrapping_add(crate::src::src::vdbeapi::sqlite3_column_bytes(pSelect, i) as crate::src::ext::rtree::rtree::u32_0);
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
    mut piIdx: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pNextIdx: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut iNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = fts3SqlStmt(
        p,
        SQL_NEXT_SEGMENT_INDEX,
        &raw mut pNextIdx,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pNextIdx,
            1 as ::core::ffi::c_int,
            getAbsoluteLevel(p, iLangid, iIndex, iLevel),
        );
        if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pNextIdx) {
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
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3ReadBlock(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iBlockid: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut paBlob: *mut *mut ::core::ffi::c_char,
    mut pnBlob: *mut ::core::ffi::c_int,
    mut pnLoad: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if !(*p).pSegments.is_null() {
        rc = crate::src::src::vdbeblob::sqlite3_blob_reopen((*p).pSegments, iBlockid);
    } else {
        let __p_ref = unsafe { &mut *p };
        if __p_ref.zSegmentsTbl.is_null() {
            __p_ref.zSegmentsTbl = crate::src::src::printf::sqlite3_mprintf(
                b"%s_segments\0" as *const u8 as *const ::core::ffi::c_char,
                __p_ref.zName,
            );
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
        let mut nByte: ::core::ffi::c_int = crate::src::src::vdbeblob::sqlite3_blob_bytes((*p).pSegments);
        *pnBlob = nByte;
        if !paBlob.is_null() {
            let mut aByte: *mut ::core::ffi::c_char =
                crate::src::src::malloc::sqlite3_malloc64((nByte as crate::src::ext::rtree::rtree::i64_0 + FTS3_NODE_PADDING as crate::src::ext::rtree::rtree::i64_0) as crate::src::headers::sqlite3_h::sqlite3_uint64)
                    as *mut ::core::ffi::c_char;
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
                ::libc::memset(
                    aByte.offset(nByte as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    FTS3_NODE_PADDING as crate::__stddef_size_t_h::size_t,
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
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3SegmentsClose(mut p: *mut crate::fts3Int_h::Fts3Table) {
    crate::src::src::vdbeblob::sqlite3_blob_close((*p).pSegments);
    (*p).pSegments = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_blob>();
}

unsafe extern "C" fn fts3SegReaderIncrRead(mut pReader: *mut Fts3SegReader) -> ::core::ffi::c_int {
    let mut nRead: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let __pReader_ref = unsafe { &mut *pReader };
    nRead = if __pReader_ref.nNode - __pReader_ref.nPopulate < test_fts3_node_chunksize {
        __pReader_ref.nNode - __pReader_ref.nPopulate
    } else {
        test_fts3_node_chunksize
    };
    rc = crate::src::src::vdbeblob::sqlite3_blob_read(
        __pReader_ref.pBlob,
        __pReader_ref.aNode.offset(__pReader_ref.nPopulate as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        nRead,
        __pReader_ref.nPopulate,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        __pReader_ref.nPopulate += nRead;
        ::libc::memset(
            __pReader_ref.aNode.offset(__pReader_ref.nPopulate as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            FTS3_NODE_PADDING as crate::__stddef_size_t_h::size_t,
        );
        if __pReader_ref.nPopulate == __pReader_ref.nNode {
            crate::src::src::vdbeblob::sqlite3_blob_close(__pReader_ref.pBlob);
            __pReader_ref.pBlob = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_blob>();
            __pReader_ref.nPopulate = 0 as ::core::ffi::c_int;
        }
    }
    rc
}

unsafe extern "C" fn fts3SegReaderRequire(
    mut pReader: *mut Fts3SegReader,
    mut pFrom: *mut ::core::ffi::c_char,
    mut nByte: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pReader_ref = unsafe { &mut *pReader };
    while !__pReader_ref.pBlob.is_null()
        && rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && pFrom.offset_from(__pReader_ref.aNode) as ::core::ffi::c_long + nByte as ::core::ffi::c_long
            > __pReader_ref.nPopulate as ::core::ffi::c_long
    {
        rc = fts3SegReaderIncrRead(pReader);
    }
    rc
}

unsafe extern "C" fn fts3SegReaderSetEof(mut pSeg: *mut Fts3SegReader) {
    if !((*pSeg).rootOnly as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
        let __pSeg_ref = unsafe { &mut *pSeg };
        crate::src::src::malloc::sqlite3_free(__pSeg_ref.aNode as *mut ::core::ffi::c_void);
        crate::src::src::vdbeblob::sqlite3_blob_close(__pSeg_ref.pBlob);
        __pSeg_ref.pBlob = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_blob>();
    }
    (*pSeg).aNode = ::core::ptr::null_mut::<::core::ffi::c_char>();
}

unsafe extern "C" fn fts3SegReaderNext(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pReader: *mut Fts3SegReader,
    mut bIncr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pNext: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nPrefix: ::core::ffi::c_int = 0;
    let mut nSuffix: ::core::ffi::c_int = 0;
    let __pReader_ref = unsafe { &mut *pReader };
    if __pReader_ref.aDoclist.is_null() {
        pNext = __pReader_ref.aNode;
    } else {
        pNext =
            __pReader_ref.aDoclist.offset(__pReader_ref.nDoclist as isize) as *mut ::core::ffi::c_char;
    }
    if pNext.is_null()
        || pNext >= __pReader_ref.aNode.offset(__pReader_ref.nNode as isize) as *mut ::core::ffi::c_char
    {
        if !__pReader_ref.ppNextElem.is_null() {
            let mut pElem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = *__pReader_ref.ppNextElem;
            crate::src::src::malloc::sqlite3_free(__pReader_ref.aNode as *mut ::core::ffi::c_void);
            __pReader_ref.aNode = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !pElem.is_null() {
                let mut aCopy: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let __pElem_ref = unsafe { &*pElem };
                let mut pList: *mut PendingList = __pElem_ref.data as *mut PendingList;
                let mut nCopy: ::core::ffi::c_int = (*pList).nData + 1 as ::core::ffi::c_int;
                let mut nTerm: ::core::ffi::c_int = __pElem_ref.nKey;
                if nTerm + 1 as ::core::ffi::c_int > __pReader_ref.nTermAlloc {
                    crate::src::src::malloc::sqlite3_free(__pReader_ref.zTerm as *mut ::core::ffi::c_void);
                    __pReader_ref.zTerm = crate::src::src::malloc::sqlite3_malloc64(
                        ((nTerm as crate::src::ext::rtree::rtree::i64_0 + 1 as crate::src::ext::rtree::rtree::i64_0) * 2 as crate::src::ext::rtree::rtree::i64_0) as crate::src::headers::sqlite3_h::sqlite3_uint64,
                    ) as *mut ::core::ffi::c_char;
                    if __pReader_ref.zTerm.is_null() {
                        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    }
                    __pReader_ref.nTermAlloc =
                        (nTerm + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int;
                }
                ::libc::memcpy(
                    __pReader_ref.zTerm as *mut ::core::ffi::c_void,
                    __pElem_ref.pKey,
                    nTerm as crate::__stddef_size_t_h::size_t,
                );
                *__pReader_ref.zTerm.offset(nTerm as isize) = '\0' as i32 as ::core::ffi::c_char;
                __pReader_ref.nTerm = nTerm;
                aCopy = crate::src::src::malloc::sqlite3_malloc64(nCopy as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut ::core::ffi::c_char;
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
            (*p).pSegments = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_blob>();
        }
        pNext = __pReader_ref.aNode;
    }
    rc = fts3SegReaderRequire(pReader, pNext, crate::fts3Int_h::FTS3_VARINT_MAX * 2 as ::core::ffi::c_int);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    pNext = pNext.offset(
        (if *(pNext as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(pNext, &raw mut nPrefix)
        } else {
            nPrefix = *(pNext as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        }) as isize,
    );
    pNext = pNext.offset(
        (if *(pNext as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(pNext, &raw mut nSuffix)
        } else {
            nSuffix = *(pNext as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int;
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
    if nPrefix as crate::src::ext::rtree::rtree::i64_0 + nSuffix as crate::src::ext::rtree::rtree::i64_0 > __pReader_ref.nTermAlloc as crate::src::ext::rtree::rtree::i64_0 {
        let mut nNew: crate::src::ext::rtree::rtree::i64_0 = (nPrefix as crate::src::ext::rtree::rtree::i64_0 + nSuffix as crate::src::ext::rtree::rtree::i64_0) * 2 as crate::src::ext::rtree::rtree::i64_0;
        let mut zNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
            __pReader_ref.zTerm as *mut ::core::ffi::c_void,
            nNew as crate::src::headers::sqlite3_h::sqlite3_uint64,
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
        (if *(pNext as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(pNext, &raw mut __pReader_ref.nDoclist)
        } else {
            __pReader_ref.nDoclist = *(pNext as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int;
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
    mut pTab: *mut crate::fts3Int_h::Fts3Table,
    mut pReader: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pTab).bDescIdx as ::core::ffi::c_int != 0 && !(*pReader).ppNextElem.is_null() {
        let mut bEof: crate::src::ext::rtree::rtree::u8_0 = 0 as crate::src::ext::rtree::rtree::u8_0;
        let __pReader_ref = unsafe { &mut *pReader };
        __pReader_ref.iDocid = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
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
        rc = fts3SegReaderRequire(pReader, (*pReader).aDoclist, crate::fts3Int_h::FTS3_VARINT_MAX);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let __pReader_ref = unsafe { &mut *pReader };
            let mut n: ::core::ffi::c_int =
                crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(__pReader_ref.aDoclist, &raw mut __pReader_ref.iDocid);
            __pReader_ref.pOffsetList =
                __pReader_ref.aDoclist.offset(n as isize) as *mut ::core::ffi::c_char;
        }
    }
    rc
}

unsafe extern "C" fn fts3SegReaderNextDocid(
    mut pTab: *mut crate::fts3Int_h::Fts3Table,
    mut pReader: *mut Fts3SegReader,
    mut ppOffsetList: *mut *mut ::core::ffi::c_char,
    mut pnOffsetList: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut p: *mut ::core::ffi::c_char = (*pReader).pOffsetList;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    if (*pTab).bDescIdx as ::core::ffi::c_int != 0 && !(*pReader).ppNextElem.is_null() {
        let mut bEof: crate::src::ext::rtree::rtree::u8_0 = 0 as crate::src::ext::rtree::rtree::u8_0;
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
        let mut pEnd: *mut ::core::ffi::c_char =
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
                let mut iDelta: crate::src::ext::rtree::rtree::u64_0 = 0;
                (*pReader).pOffsetList =
                    p.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintU(p, &raw mut iDelta) as isize);
                if (*pTab).bDescIdx != 0 {
                    (*pReader).iDocid =
                        ((*pReader).iDocid as crate::src::ext::rtree::rtree::u64_0).wrapping_sub(iDelta) as crate::src::ext::rtree::rtree::i64_0 as crate::src::headers::sqlite3_h::sqlite3_int64;
                } else {
                    (*pReader).iDocid =
                        ((*pReader).iDocid as crate::src::ext::rtree::rtree::u64_0).wrapping_add(iDelta) as crate::src::ext::rtree::rtree::i64_0 as crate::src::headers::sqlite3_h::sqlite3_int64;
                }
            }
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3MsrOvfl(
    mut pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    mut pMsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    mut pnOvfl: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut crate::fts3Int_h::Fts3Table = (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut nOvfl: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ii: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pgsz: ::core::ffi::c_int = (*p).nPgsz;
    ii = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && ii < (*pMsr).nSegment {
        let mut pReader: *mut Fts3SegReader = *(*pMsr).apSegment.offset(ii as isize);
        if (*pReader).ppNextElem.is_null()
            && !((*pReader).rootOnly as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
        {
            let mut jj: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
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
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3SegReaderFree(mut pReader: *mut Fts3SegReader) {
    if !pReader.is_null() {
        let __pReader_ref = unsafe { &*pReader };
        crate::src::src::malloc::sqlite3_free(__pReader_ref.zTerm as *mut ::core::ffi::c_void);
        if !(__pReader_ref.rootOnly as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            crate::src::src::malloc::sqlite3_free(__pReader_ref.aNode as *mut ::core::ffi::c_void);
        }
        crate::src::src::vdbeblob::sqlite3_blob_close(__pReader_ref.pBlob);
    }
    crate::src::src::malloc::sqlite3_free(pReader as *mut ::core::ffi::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3SegReaderNew(
    mut iAge: ::core::ffi::c_int,
    mut bLookup: ::core::ffi::c_int,
    mut iStartLeaf: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iEndLeaf: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iEndBlock: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut zRoot: *const ::core::ffi::c_char,
    mut nRoot: ::core::ffi::c_int,
    mut ppReader: *mut *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut pReader: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
    let mut nExtra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if iStartLeaf == 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
        if iEndLeaf != 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
        }
        nExtra = nRoot + FTS3_NODE_PADDING;
    }
    pReader = crate::src::src::malloc::sqlite3_malloc64(
        (::core::mem::size_of::<Fts3SegReader>() as usize).wrapping_add(nExtra as usize)
            as crate::src::headers::sqlite3_h::sqlite3_uint64,
    ) as *mut Fts3SegReader;
    if pReader.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pReader as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3SegReader>() as crate::__stddef_size_t_h::size_t,
    );
    (*pReader).iIdx = iAge;
    (*pReader).bLookup = (bLookup != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0;
    (*pReader).iStartBlock = iStartLeaf;
    (*pReader).iLeafEndBlock = iEndLeaf;
    (*pReader).iEndBlock = iEndBlock;
    if nExtra != 0 {
        let __pReader_ref = unsafe { &mut *pReader };
        __pReader_ref.aNode = pReader.offset(1 as isize) as *mut Fts3SegReader
            as *mut ::core::ffi::c_char;
        __pReader_ref.rootOnly = 1 as crate::src::ext::rtree::rtree::u8_0;
        __pReader_ref.nNode = nRoot;
        if nRoot != 0 {
            ::core::ptr::copy_nonoverlapping(
                    zRoot as *const u8,
                    __pReader_ref.aNode as *mut u8,
                    nRoot as usize,
                );
        }
        ::libc::memset(
            __pReader_ref.aNode.offset(nRoot as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            FTS3_NODE_PADDING as crate::__stddef_size_t_h::size_t,
        );
    } else {
        (*pReader).iCurrentBlock = iStartLeaf - 1 as crate::src::headers::sqlite3_h::sqlite3_int64;
    }
    *ppReader = pReader;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3CompareElemByTerm(
    mut lhs: *const ::core::ffi::c_void,
    mut rhs: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut z1: *mut ::core::ffi::c_char =
        (**(lhs as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem)).pKey as *mut ::core::ffi::c_char;
    let mut z2: *mut ::core::ffi::c_char =
        (**(rhs as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem)).pKey as *mut ::core::ffi::c_char;
    let mut n1: ::core::ffi::c_int = (**(lhs as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem)).nKey;
    let mut n2: ::core::ffi::c_int = (**(rhs as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem)).nKey;
    let mut n: ::core::ffi::c_int = if n1 < n2 { n1 } else { n2 };
    let mut c: ::core::ffi::c_int = ::libc::memcmp(
        z1 as *const ::core::ffi::c_void,
        z2 as *const ::core::ffi::c_void,
        n as crate::__stddef_size_t_h::size_t,
    );
    if c == 0 as ::core::ffi::c_int {
        c = n1 - n2;
    }
    c
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3SegReaderPending(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iIndex: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut bPrefix: ::core::ffi::c_int,
    mut ppReader: *mut *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut pReader: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
    let mut pE: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    let mut aElem: *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<*mut crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    let mut nElem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3Hash>();
    pHash = &raw mut (*(*p).aIndex.offset(iIndex as isize)).hPending;
    if bPrefix != 0 {
        let mut nAlloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pE = (*pHash).first;
        while !pE.is_null() {
            let mut zKey: *mut ::core::ffi::c_char = (*pE).pKey as *mut ::core::ffi::c_char;
            let mut nKey: ::core::ffi::c_int = (*pE).nKey;
            if nTerm == 0 as ::core::ffi::c_int
                || nKey >= nTerm
                    && 0 as ::core::ffi::c_int
                        == ::libc::memcmp(
                            zKey as *const ::core::ffi::c_void,
                            zTerm as *const ::core::ffi::c_void,
                            nTerm as crate::__stddef_size_t_h::size_t,
                        )
            {
                if nElem == nAlloc {
                    let mut aElem2: *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem =
                        ::core::ptr::null_mut::<*mut crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
                    nAlloc += 16 as ::core::ffi::c_int;
                    aElem2 = crate::src::src::malloc::sqlite3_realloc64(
                        aElem as *mut ::core::ffi::c_void,
                        (nAlloc as usize)
                            .wrapping_mul(::core::mem::size_of::<*mut crate::src::ext::fts3::fts3_hash::Fts3HashElem>() as usize)
                            as crate::src::headers::sqlite3_h::sqlite3_uint64,
                    ) as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem;
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
                let ref mut fresh17 = *aElem.offset(fresh16 as isize);
                *fresh17 = pE;
            }
            pE = (*pE).next;
        }
        if nElem > 1 as ::core::ffi::c_int {
            ::libc::qsort(
                aElem as *mut ::core::ffi::c_void,
                nElem as crate::__stddef_size_t_h::size_t,
                ::core::mem::size_of::<*mut crate::src::ext::fts3::fts3_hash::Fts3HashElem>() as crate::__stddef_size_t_h::size_t,
                Some(
                    fts3CompareElemByTerm
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
        }
    } else {
        pE =  crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFindElem(pHash as *const crate::src::ext::fts3::fts3_hash::Fts3Hash, zTerm as *const ::core::ffi::c_void, nTerm) as
    *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem;
        if !pE.is_null() {
            aElem = &raw mut pE;
            nElem = 1 as ::core::ffi::c_int;
        }
    }
    if nElem > 0 as ::core::ffi::c_int {
        let mut nByte: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
        nByte = (::core::mem::size_of::<Fts3SegReader>() as usize).wrapping_add(
            ((nElem + 1 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<*mut crate::src::ext::fts3::fts3_hash::Fts3HashElem>() as usize),
        ) as crate::src::headers::sqlite3_h::sqlite3_int64;
        pReader = crate::src::src::malloc::sqlite3_malloc64(nByte as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut Fts3SegReader;
        if pReader.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            ::libc::memset(
                pReader as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nByte as crate::__stddef_size_t_h::size_t,
            );
            let __pReader_ref = unsafe { &mut *pReader };
            __pReader_ref.iIdx = 0x7fffffff as ::core::ffi::c_int;
            __pReader_ref.ppNextElem = pReader.offset(1 as isize)
                as *mut Fts3SegReader as *mut *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem;
            ::core::ptr::copy_nonoverlapping(
                    aElem as *const u8,
                    __pReader_ref.ppNextElem as *mut u8,
                    ((nElem as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut crate::src::ext::fts3::fts3_hash::Fts3HashElem>() as crate::__stddef_size_t_h::size_t)) as usize,
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
    mut pLhs: *mut Fts3SegReader,
    mut pRhs: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if !(*pLhs).aNode.is_null() && !(*pRhs).aNode.is_null() {
        let mut rc2: ::core::ffi::c_int = (*pLhs).nTerm - (*pRhs).nTerm;
        if rc2 < 0 as ::core::ffi::c_int {
            rc = ::libc::memcmp(
                (*pLhs).zTerm as *const ::core::ffi::c_void,
                (*pRhs).zTerm as *const ::core::ffi::c_void,
                (*pLhs).nTerm as crate::__stddef_size_t_h::size_t,
            );
        } else {
            rc = ::libc::memcmp(
                (*pLhs).zTerm as *const ::core::ffi::c_void,
                (*pRhs).zTerm as *const ::core::ffi::c_void,
                (*pRhs).nTerm as crate::__stddef_size_t_h::size_t,
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
    mut pLhs: *mut Fts3SegReader,
    mut pRhs: *mut Fts3SegReader,
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
    mut pLhs: *mut Fts3SegReader,
    mut pRhs: *mut Fts3SegReader,
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
    mut pSeg: *mut Fts3SegReader,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*pSeg).aNode.is_null() {
        if (*pSeg).nTerm > nTerm {
            res = ::libc::memcmp(
                (*pSeg).zTerm as *const ::core::ffi::c_void,
                zTerm as *const ::core::ffi::c_void,
                nTerm as crate::__stddef_size_t_h::size_t,
            );
        } else {
            res = ::libc::memcmp(
                (*pSeg).zTerm as *const ::core::ffi::c_void,
                zTerm as *const ::core::ffi::c_void,
                (*pSeg).nTerm as crate::__stddef_size_t_h::size_t,
            );
        }
        if res == 0 as ::core::ffi::c_int {
            res = (*pSeg).nTerm - nTerm;
        }
    }
    res
}

unsafe extern "C" fn fts3SegReaderSort(
    mut apSegment: *mut *mut Fts3SegReader,
    mut nSegment: ::core::ffi::c_int,
    mut nSuspect: ::core::ffi::c_int,
    mut xCmp: Option<
        unsafe extern "C" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> ::core::ffi::c_int,
    >,
) {
    let mut i: ::core::ffi::c_int = 0;
    if nSuspect == nSegment {
        nSuspect -= 1;
    }
    i = nSuspect - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0;
        j = i;
        while j < nSegment - 1 as ::core::ffi::c_int {
            let mut pTmp: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
            if xCmp.expect("non-null function pointer")(
                *apSegment.offset(j as isize),
                *apSegment.offset((j + 1 as ::core::ffi::c_int) as isize),
            ) < 0 as ::core::ffi::c_int
            {
                break;
            }
            pTmp = *apSegment.offset((j + 1 as ::core::ffi::c_int) as isize);
            let ref mut fresh7 = *apSegment.offset((j + 1 as ::core::ffi::c_int) as isize);
            *fresh7 = *apSegment.offset(j as isize);
            let ref mut fresh8 = *apSegment.offset(j as isize);
            *fresh8 = pTmp;
            j += 1;
        }
        i -= 1;
    }
}

unsafe extern "C" fn fts3WriteSegment(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iBlock: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut z: *mut ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SQL_INSERT_SEGMENTS,
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
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3MaxLevel(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pnMax: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut mxLevel: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_MXLEVEL,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pStmt) {
            mxLevel = crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    }
    *pnMax = mxLevel;
    rc
}

unsafe extern "C" fn fts3WriteSegdir(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
    mut iStartBlock: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iLeafEndBlock: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iEndBlock: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut nLeafData: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut zRoot: *mut ::core::ffi::c_char,
    mut nRoot: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SQL_INSERT_SEGDIR,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iLevel);
        crate::src::src::vdbeapi::sqlite3_bind_int(pStmt, 2 as ::core::ffi::c_int, iIdx);
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 3 as ::core::ffi::c_int, iStartBlock);
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 4 as ::core::ffi::c_int, iLeafEndBlock);
        if nLeafData == 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
            crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 5 as ::core::ffi::c_int, iEndBlock);
        } else {
            let mut zEnd: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3_mprintf(
                b"%lld %lld\0" as *const u8 as *const ::core::ffi::c_char,
                iEndBlock,
                nLeafData,
            );
            if zEnd.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            crate::src::src::vdbeapi::sqlite3_bind_text(
                pStmt,
                5 as ::core::ffi::c_int,
                zEnd,
                -(1 as ::core::ffi::c_int),
                Some(crate::src::src::malloc::sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
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
    mut zPrev: *const ::core::ffi::c_char,
    mut nPrev: ::core::ffi::c_int,
    mut zNext: *const ::core::ffi::c_char,
    mut nNext: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut ppTree: *mut *mut SegmentNode,
    mut isCopyTerm: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pTree: *mut SegmentNode = *ppTree;
    let mut rc: ::core::ffi::c_int = 0;
    let mut pNew: *mut SegmentNode = ::core::ptr::null_mut::<SegmentNode>();
    if !pTree.is_null() {
        let __pTree_ref = unsafe { &mut *pTree };
        let mut nData: ::core::ffi::c_int = __pTree_ref.nData;
        let mut nReq: ::core::ffi::c_int = nData;
        let mut nPrefix: ::core::ffi::c_int = 0;
        let mut nSuffix: ::core::ffi::c_int = 0;
        nPrefix = fts3PrefixCompress(__pTree_ref.zTerm, __pTree_ref.nTerm, zTerm, nTerm);
        nSuffix = nTerm - nPrefix;
        if nSuffix <= 0 as ::core::ffi::c_int {
            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
        }
        nReq += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nPrefix as crate::src::headers::sqlite3_h::sqlite3_uint64)
            + crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nSuffix as crate::src::headers::sqlite3_h::sqlite3_uint64)
            + nSuffix;
        if nReq <= (*p).nNodeSize || __pTree_ref.zTerm.is_null() {
            if nReq > (*p).nNodeSize {
                __pTree_ref.aData =
                    crate::src::src::malloc::sqlite3_malloc64(nReq as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut ::core::ffi::c_char;
                if __pTree_ref.aData.is_null() {
                    return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                }
            }
            if !__pTree_ref.zTerm.is_null() {
                nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                    __pTree_ref.aData.offset(nData as isize) as *mut ::core::ffi::c_char,
                    nPrefix as crate::src::headers::sqlite3_h::sqlite3_int64,
                );
            }
            nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                __pTree_ref.aData.offset(nData as isize) as *mut ::core::ffi::c_char,
                nSuffix as crate::src::headers::sqlite3_h::sqlite3_int64,
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
                    let mut zNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
                        __pTree_ref.zMalloc as *mut ::core::ffi::c_void,
                        (nTerm as crate::src::ext::rtree::rtree::i64_0 * 2 as crate::src::ext::rtree::rtree::i64_0) as crate::src::headers::sqlite3_h::sqlite3_uint64,
                    )
                        as *mut ::core::ffi::c_char;
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
            as crate::src::headers::sqlite3_h::sqlite3_uint64,
    ) as *mut SegmentNode;
    if pNew.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pNew as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SegmentNode>() as crate::__stddef_size_t_h::size_t,
    );
    (*pNew).nData = 1 as ::core::ffi::c_int + crate::fts3Int_h::FTS3_VARINT_MAX;
    (*pNew).aData = pNew.offset(1 as isize) as *mut SegmentNode
        as *mut ::core::ffi::c_char;
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
    mut pTree: *mut SegmentNode,
    mut iHeight: ::core::ffi::c_int,
    mut iLeftChild: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut nStart: ::core::ffi::c_int = 0;
    nStart = crate::fts3Int_h::FTS3_VARINT_MAX - crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(iLeftChild as crate::src::headers::sqlite3_h::sqlite3_uint64);
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pTree: *mut SegmentNode,
    mut iHeight: ::core::ffi::c_int,
    mut iLeaf: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iFree: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut piLast: *mut crate::src::headers::sqlite3_h::sqlite3_int64,
    mut paRoot: *mut *mut ::core::ffi::c_char,
    mut pnRoot: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pTree).pParent.is_null() {
        let mut nStart: ::core::ffi::c_int = fts3TreeFinishNode(pTree, iHeight, iLeaf);
        *piLast = iFree - 1 as crate::src::headers::sqlite3_h::sqlite3_int64;
        *pnRoot = (*pTree).nData - nStart;
        *paRoot = (*pTree).aData.offset(nStart as isize) as *mut ::core::ffi::c_char;
    } else {
        let mut pIter: *mut SegmentNode = ::core::ptr::null_mut::<SegmentNode>();
        let mut iNextFree: crate::src::headers::sqlite3_h::sqlite3_int64 = iFree;
        let mut iNextLeaf: crate::src::headers::sqlite3_h::sqlite3_int64 = iLeaf;
        pIter = (*pTree).pLeftmost;
        while !pIter.is_null() && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let mut nStart_0: ::core::ffi::c_int = fts3TreeFinishNode(pIter, iHeight, iNextLeaf);
            let mut nWrite: ::core::ffi::c_int = (*pIter).nData - nStart_0;
            rc = fts3WriteSegment(
                p,
                iNextFree,
                (*pIter).aData.offset(nStart_0 as isize) as *mut ::core::ffi::c_char,
                nWrite,
            );
            iNextFree += 1;
            iNextLeaf += ((*pIter).nEntry + 1 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::sqlite3_int64;
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

unsafe extern "C" fn fts3NodeFree(mut pTree: *mut SegmentNode) {
    if !pTree.is_null() {
        let mut p: *mut SegmentNode = (*pTree).pLeftmost;
        fts3NodeFree((*p).pParent);
        while !p.is_null() {
            let mut pRight: *mut SegmentNode = (*p).pRight;
            if (*p).aData
                != p.offset(1 as isize) as *mut SegmentNode
                    as *mut ::core::ffi::c_char
            {
                crate::src::src::malloc::sqlite3_free((*p).aData as *mut ::core::ffi::c_void);
            }
            crate::src::src::malloc::sqlite3_free((*p).zMalloc as *mut ::core::ffi::c_void);
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
            p = pRight;
        }
    }
}

unsafe extern "C" fn fts3SegWriterAdd(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut ppWriter: *mut *mut SegmentWriter,
    mut isCopyTerm: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut aDoclist: *const ::core::ffi::c_char,
    mut nDoclist: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nPrefix: ::core::ffi::c_int = 0;
    let mut nSuffix: ::core::ffi::c_int = 0;
    let mut nReq: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut nData: ::core::ffi::c_int = 0;
    let mut pWriter: *mut SegmentWriter = *ppWriter;
    if pWriter.is_null() {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        pWriter = crate::src::src::malloc::sqlite3_malloc64(::core::mem::size_of::<SegmentWriter>() as crate::src::headers::sqlite3_h::sqlite3_uint64)
            as *mut SegmentWriter;
        if pWriter.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        ::libc::memset(
            pWriter as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<SegmentWriter>() as crate::__stddef_size_t_h::size_t,
        );
        *ppWriter = pWriter;
        (*pWriter).aData =
            crate::src::src::malloc::sqlite3_malloc64((*p).nNodeSize as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut ::core::ffi::c_char;
        if (*pWriter).aData.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        (*pWriter).nSize = (*p).nNodeSize;
        rc = fts3SqlStmt(
            p,
            SQL_NEXT_SEGMENTS_ID,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pStmt) {
            let __pWriter_ref = unsafe { &mut *pWriter };
            __pWriter_ref.iFree = crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int);
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
    nReq = (crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nPrefix as crate::src::headers::sqlite3_h::sqlite3_uint64)
        + crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nSuffix as crate::src::headers::sqlite3_h::sqlite3_uint64)
        + nSuffix
        + crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nDoclist as crate::src::headers::sqlite3_h::sqlite3_uint64)
        + nDoclist) as crate::src::ext::rtree::rtree::i64_0;
    if nData > 0 as ::core::ffi::c_int && nData as crate::src::ext::rtree::rtree::i64_0 + nReq > (*p).nNodeSize as crate::src::ext::rtree::rtree::i64_0 {
        let mut rc_0: ::core::ffi::c_int = 0;
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
            + crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nTerm as crate::src::headers::sqlite3_h::sqlite3_uint64)
            + nTerm
            + crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nDoclist as crate::src::headers::sqlite3_h::sqlite3_uint64)
            + nDoclist) as crate::src::ext::rtree::rtree::i64_0;
    }
    (*pWriter).nLeafData += nReq;
    if nReq > (*pWriter).nSize as crate::src::ext::rtree::rtree::i64_0 {
        let __pWriter_ref = unsafe { &mut *pWriter };
        let mut aNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
            __pWriter_ref.aData as *mut ::core::ffi::c_void,
            nReq as crate::src::headers::sqlite3_h::sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if aNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pWriter_ref.aData = aNew;
        __pWriter_ref.nSize = nReq as ::core::ffi::c_int;
    }
    nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
        nPrefix as crate::src::headers::sqlite3_h::sqlite3_int64,
    );
    nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
        nSuffix as crate::src::headers::sqlite3_h::sqlite3_int64,
    );
    ::core::ptr::copy_nonoverlapping(
                    zTerm.offset(nPrefix as isize) as *const ::core::ffi::c_char,
                    (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
                    nSuffix as usize,
                );
    nData += nSuffix;
    nData += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
        (*pWriter).aData.offset(nData as isize) as *mut ::core::ffi::c_char,
        nDoclist as crate::src::headers::sqlite3_h::sqlite3_int64,
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
            let mut zNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
                __pWriter_ref.zMalloc as *mut ::core::ffi::c_void,
                (nTerm as crate::src::ext::rtree::rtree::i64_0 * 2 as crate::src::ext::rtree::rtree::i64_0) as crate::src::headers::sqlite3_h::sqlite3_uint64,
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pWriter: *mut SegmentWriter,
    mut iLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if !(*pWriter).pTree.is_null() {
        let mut iLast: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
        let mut iLastLeaf: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
        let mut zRoot: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nRoot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let __pWriter_ref = unsafe { &mut *pWriter };
        iLastLeaf = __pWriter_ref.iFree;
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
                __pWriter_ref.nLeafData as crate::src::headers::sqlite3_h::sqlite3_int64,
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
            0 as crate::src::headers::sqlite3_h::sqlite3_int64,
            0 as crate::src::headers::sqlite3_h::sqlite3_int64,
            0 as crate::src::headers::sqlite3_h::sqlite3_int64,
            __pWriter_ref.nLeafData as crate::src::headers::sqlite3_h::sqlite3_int64,
            __pWriter_ref.aData,
            __pWriter_ref.nData,
        );
    }
    (*p).nLeafAdd = (*p).nLeafAdd.wrapping_add(1);
    rc
}

unsafe extern "C" fn fts3SegWriterFree(mut pWriter: *mut SegmentWriter) {
    if !pWriter.is_null() {
        let __pWriter_ref = unsafe { &*pWriter };
        crate::src::src::malloc::sqlite3_free(__pWriter_ref.aData as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(__pWriter_ref.zMalloc as *mut ::core::ffi::c_void);
        fts3NodeFree(__pWriter_ref.pTree);
        crate::src::src::malloc::sqlite3_free(pWriter as *mut ::core::ffi::c_void);
    }
}

unsafe extern "C" fn fts3IsEmpty(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pRowid: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    mut pisEmpty: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    if !(*p).zContentTbl.is_null() {
        *pisEmpty = 0 as ::core::ffi::c_int;
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    } else {
        rc = fts3SqlStmt(p, SQL_IS_EMPTY, &raw mut pStmt, &raw mut pRowid);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pStmt) {
                *pisEmpty = crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
            }
            rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
        }
    }
    rc
}

unsafe extern "C" fn fts3SegmentMaxLevel(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut pnMax: *mut crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_SEGDIR_MAX_LEVEL,
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iAbsLevel: crate::src::ext::rtree::rtree::i64_0,
    mut pbMax: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = fts3SqlStmt(
        p,
        SQL_SELECT_SEGDIR_MAX_LEVEL,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        pStmt,
        1 as ::core::ffi::c_int,
        iAbsLevel as crate::src::headers::sqlite3_h::sqlite3_int64 + 1 as crate::src::headers::sqlite3_h::sqlite3_int64,
    );
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        pStmt,
        2 as ::core::ffi::c_int,
        (iAbsLevel as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_div(crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_add(1 as crate::src::ext::rtree::rtree::u64_0)
            .wrapping_mul(crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL as crate::src::ext::rtree::rtree::u64_0) as crate::src::headers::sqlite3_h::sqlite3_int64,
    );
    *pbMax = 0 as ::core::ffi::c_int;
    if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pStmt) {
        *pbMax = (crate::src::src::vdbeapi::sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int) == crate::src::headers::sqlite3_h::SQLITE_NULL)
            as ::core::ffi::c_int;
    }
    crate::src::src::vdbeapi::sqlite3_reset(pStmt)
}

unsafe extern "C" fn fts3DeleteSegment(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pSeg: *mut Fts3SegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pSeg).iStartBlock != 0 {
        let mut pDelete: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            SQL_DELETE_SEGMENTS_RANGE,
            &raw mut pDelete,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int64(pDelete, 1 as ::core::ffi::c_int, (*pSeg).iStartBlock);
            crate::src::src::vdbeapi::sqlite3_bind_int64(pDelete, 2 as ::core::ffi::c_int, (*pSeg).iEndBlock);
            crate::src::src::vdbeapi::sqlite3_step(pDelete);
            rc = crate::src::src::vdbeapi::sqlite3_reset(pDelete);
        }
    }
    rc
}

unsafe extern "C" fn fts3DeleteSegdir(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
    mut apSegment: *mut *mut Fts3SegReader,
    mut nReader: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i: ::core::ffi::c_int = 0;
    let mut pDelete: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
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
            SQL_DELETE_SEGDIR_RANGE,
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
            SQL_DELETE_SEGDIR_LEVEL,
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
    mut iCol: ::core::ffi::c_int,
    mut bZero: ::core::ffi::c_int,
    mut ppList: *mut *mut ::core::ffi::c_char,
    mut pnList: *mut ::core::ffi::c_int,
) {
    let mut pList: *mut ::core::ffi::c_char = *ppList;
    let mut nList: ::core::ffi::c_int = *pnList;
    let mut pEnd: *mut ::core::ffi::c_char =
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
            p = pList.offset(1 as isize) as *mut ::core::ffi::c_char;
            p = p.offset(
                (if *(p as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(p, &raw mut iCurrent)
                } else {
                    iCurrent = *(p as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int;
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
        ::libc::memset(
            pList.offset(nList as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            pEnd.offset_from(pList.offset(nList as isize) as *mut ::core::ffi::c_char)
                as ::core::ffi::c_long as crate::__stddef_size_t_h::size_t,
        );
    }
    *ppList = pList;
    *pnList = nList;
}

unsafe extern "C" fn fts3MsrBufferData(
    mut pMsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    mut pList: *mut ::core::ffi::c_char,
    mut nList: crate::src::ext::rtree::rtree::i64_0,
) -> ::core::ffi::c_int {
    let __pMsr_ref = unsafe { &mut *pMsr };
    if nList + FTS3_NODE_PADDING as crate::src::ext::rtree::rtree::i64_0 > __pMsr_ref.nBuffer {
        let mut pNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nNew: ::core::ffi::c_int =
            (nList * 2 as crate::src::ext::rtree::rtree::i64_0 + FTS3_NODE_PADDING as crate::src::ext::rtree::rtree::i64_0) as ::core::ffi::c_int;
        pNew = crate::src::src::malloc::sqlite3_realloc64(
            __pMsr_ref.aBuffer as *mut ::core::ffi::c_void,
            nNew as crate::src::headers::sqlite3_h::sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if pNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pMsr_ref.aBuffer = pNew;
        __pMsr_ref.nBuffer = nNew as crate::src::ext::rtree::rtree::i64_0;
    }
    ::core::ptr::copy_nonoverlapping(
                    pList as *const u8,
                    __pMsr_ref.aBuffer as *mut u8,
                    nList as usize,
                );
    ::libc::memset(
        __pMsr_ref.aBuffer.offset(nList as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        FTS3_NODE_PADDING as crate::__stddef_size_t_h::size_t,
    );
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3MsrIncrNext(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pMsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    mut piDocid: *mut crate::src::headers::sqlite3_h::sqlite3_int64,
    mut paPoslist: *mut *mut ::core::ffi::c_char,
    mut pnPoslist: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nMerge: ::core::ffi::c_int = (*pMsr).nAdvance;
    let mut apSegment: *mut *mut Fts3SegReader = (*pMsr).apSegment;
    let mut xCmp: Option<
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
        let mut pSeg: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
        pSeg = *(*pMsr).apSegment.offset(0 as isize);
        if (*pSeg).pOffsetList.is_null() {
            *paPoslist = ::core::ptr::null_mut::<::core::ffi::c_char>();
            break;
        } else {
            let mut rc: ::core::ffi::c_int = 0;
            let mut pList: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut nList: ::core::ffi::c_int = 0;
            let mut j: ::core::ffi::c_int = 0;
            let mut iDocid: crate::src::headers::sqlite3_h::sqlite3_int64 =
                (**apSegment.offset(0 as isize)).iDocid;
            rc = fts3SegReaderNextDocid(
                p,
                *apSegment.offset(0 as isize),
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
                && !(**apSegment.offset(0 as isize))
                    .ppNextElem
                    .is_null()
            {
                rc = fts3MsrBufferData(pMsr, pList, nList as crate::src::ext::rtree::rtree::i64_0 + 1 as crate::src::ext::rtree::rtree::i64_0);
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
            if !(nList > 0 as ::core::ffi::c_int) {
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let __pCsr_ref = unsafe { &mut *pCsr };
    let mut nSeg: ::core::ffi::c_int = __pCsr_ref.nSegment;
    i = 0 as ::core::ffi::c_int;
    while __pCsr_ref.bRestart == 0 as ::core::ffi::c_int && i < __pCsr_ref.nSegment {
        let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pSeg: *mut Fts3SegReader = *__pCsr_ref.apSegment.offset(i as isize);
        loop {
            let mut rc: ::core::ffi::c_int = fts3SegReaderNext(p, pSeg, 0 as ::core::ffi::c_int);
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
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3SegReaderStart(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    mut pFilter: *mut crate::fts3Int_h::Fts3SegFilter,
) -> ::core::ffi::c_int {
    (*pCsr).pFilter = pFilter;
    fts3SegReaderStart(p, pCsr, (*pFilter).zTerm, (*pFilter).nTerm)
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3MsrIncrStart(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    mut iCol: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let __pCsr_ref = unsafe { &mut *pCsr };
    let mut nSegment: ::core::ffi::c_int = __pCsr_ref.nSegment;
    let mut xCmp: Option<
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
        let mut pSeg: *mut Fts3SegReader = *__pCsr_ref.apSegment.offset(i as isize);
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
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3MsrIncrRestart(
    mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let __pCsr_ref = unsafe { &mut *pCsr };
    __pCsr_ref.nAdvance = 0 as ::core::ffi::c_int;
    __pCsr_ref.bRestart = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < __pCsr_ref.nSegment {
        let ref mut fresh18 = (**__pCsr_ref.apSegment.offset(i as isize)).pOffsetList;
        *fresh18 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (**__pCsr_ref.apSegment.offset(i as isize)).nOffsetList = 0 as ::core::ffi::c_int;
        (**__pCsr_ref.apSegment.offset(i as isize)).iDocid = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
        i += 1;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3GrowSegReaderBuffer(
    mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    mut nReq: crate::src::ext::rtree::rtree::i64_0,
) -> ::core::ffi::c_int {
    if nReq > (*pCsr).nBuffer {
        let mut aNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let __pCsr_ref = unsafe { &mut *pCsr };
        __pCsr_ref.nBuffer = nReq * 2 as crate::src::ext::rtree::rtree::i64_0;
        aNew = crate::src::src::malloc::sqlite3_realloc64(
            __pCsr_ref.aBuffer as *mut ::core::ffi::c_void,
            __pCsr_ref.nBuffer as crate::src::headers::sqlite3_h::sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if aNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pCsr_ref.aBuffer = aNew;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3SegReaderStep(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pCsr_ref = unsafe { &mut *pCsr };
    let mut isIgnoreEmpty: ::core::ffi::c_int =
        (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY;
    let mut isRequirePos: ::core::ffi::c_int = (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS;
    let mut isColFilter: ::core::ffi::c_int = (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_COLUMN_FILTER;
    let mut isPrefix: ::core::ffi::c_int = (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_PREFIX;
    let mut isScan: ::core::ffi::c_int = (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_SCAN;
    let mut isFirst: ::core::ffi::c_int = (*__pCsr_ref.pFilter).flags & crate::fts3Int_h::FTS3_SEGMENT_FIRST;
    let mut apSegment: *mut *mut Fts3SegReader = __pCsr_ref.apSegment;
    let mut nSegment: ::core::ffi::c_int = __pCsr_ref.nSegment;
    let mut pFilter: *mut crate::fts3Int_h::Fts3SegFilter = __pCsr_ref.pFilter;
    let mut xCmp: Option<
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
        let mut nMerge: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < __pCsr_ref.nAdvance {
            let mut pSeg: *mut Fts3SegReader = *apSegment.offset(i as isize);
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
        if (**apSegment.offset(0 as isize))
            .aNode
            .is_null()
        {
            break;
        }
        __pCsr_ref.nTerm = (**apSegment.offset(0 as isize)).nTerm;
        __pCsr_ref.zTerm = (**apSegment.offset(0 as isize)).zTerm;
        if !(*pFilter).zTerm.is_null() && isScan == 0 {
            let __pFilter_ref = unsafe { &*pFilter };
            if __pCsr_ref.nTerm < __pFilter_ref.nTerm
                || isPrefix == 0 && __pCsr_ref.nTerm > __pFilter_ref.nTerm
                || ::libc::memcmp(
                    __pCsr_ref.zTerm as *const ::core::ffi::c_void,
                    __pFilter_ref.zTerm as *const ::core::ffi::c_void,
                    __pFilter_ref.nTerm as crate::__stddef_size_t_h::size_t,
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
                == ::libc::memcmp(
                    __pCsr_ref.zTerm as *const ::core::ffi::c_void,
                    (**apSegment.offset(nMerge as isize)).zTerm as *const ::core::ffi::c_void,
                    __pCsr_ref.nTerm as crate::__stddef_size_t_h::size_t,
                )
        {
            nMerge += 1;
        }
        if nMerge == 1 as ::core::ffi::c_int
            && isIgnoreEmpty == 0
            && isFirst == 0
            && ((*p).bDescIdx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || ((**apSegment.offset(0 as isize)).ppNextElem
                    != ::core::ptr::null_mut::<*mut crate::src::ext::fts3::fts3_hash::Fts3HashElem>())
                    as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int)
        {
            __pCsr_ref.nDoclist = (**apSegment.offset(0 as isize)).nDoclist;
            if !(**apSegment.offset(0 as isize))
                .ppNextElem
                .is_null()
            {
                rc = fts3MsrBufferData(
                    pCsr,
                    (**apSegment.offset(0 as isize)).aDoclist,
                    __pCsr_ref.nDoclist as crate::src::ext::rtree::rtree::i64_0,
                );
                __pCsr_ref.aDoclist = __pCsr_ref.aBuffer;
            } else {
                __pCsr_ref.aDoclist = (**apSegment.offset(0 as isize)).aDoclist;
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::src::headers::sqlite3_h::SQLITE_ROW;
            }
        } else {
            let mut nDoclist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iPrev: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
            i = 0 as ::core::ffi::c_int;
            while i < nMerge {
                fts3SegReaderFirstDocid(p, *apSegment.offset(i as isize));
                i += 1;
            }
            fts3SegReaderSort(apSegment, nMerge, nMerge, xCmp);
            while !(**apSegment.offset(0 as isize))
                .pOffsetList
                .is_null()
            {
                let mut j: ::core::ffi::c_int = 0;
                let mut pList: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut nList: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut nByte: ::core::ffi::c_int = 0;
                let mut iDocid: crate::src::headers::sqlite3_h::sqlite3_int64 =
                    (**apSegment.offset(0 as isize)).iDocid;
                fts3SegReaderNextDocid(
                    p,
                    *apSegment.offset(0 as isize),
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
                    let mut iDelta: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
                    if (*p).bDescIdx as ::core::ffi::c_int != 0
                        && nDoclist > 0 as ::core::ffi::c_int
                    {
                        if iPrev <= iDocid {
                            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
                        }
                        iDelta = (iPrev as crate::src::ext::rtree::rtree::u64_0).wrapping_sub(iDocid as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::i64_0
                            as crate::src::headers::sqlite3_h::sqlite3_int64;
                    } else {
                        if nDoclist > 0 as ::core::ffi::c_int && iPrev >= iDocid {
                            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
                        }
                        iDelta = (iDocid as crate::src::ext::rtree::rtree::u64_0).wrapping_sub(iPrev as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::i64_0
                            as crate::src::headers::sqlite3_h::sqlite3_int64;
                    }
                    nByte = crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(iDelta as crate::src::headers::sqlite3_h::sqlite3_uint64)
                        + (if isRequirePos != 0 {
                            nList + 1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        });
                    rc = fts3GrowSegReaderBuffer(
                        pCsr,
                        nByte as crate::src::ext::rtree::rtree::i64_0 + nDoclist as crate::src::ext::rtree::rtree::i64_0 + FTS3_NODE_PADDING as crate::src::ext::rtree::rtree::i64_0,
                    );
                    if rc != 0 {
                        return rc;
                    }
                    if isFirst != 0 {
                        let mut a: *mut ::core::ffi::c_char =
                            __pCsr_ref.aBuffer.offset(nDoclist as isize) as *mut ::core::ffi::c_char;
                        let mut nWrite: ::core::ffi::c_int = 0;
                        nWrite = crate::src::ext::fts3::fts3::sqlite3Fts3FirstFilter(iDelta, pList, nList, a);
                        if nWrite != 0 {
                            iPrev = iDocid;
                            nDoclist += nWrite;
                        }
                    } else {
                        nDoclist += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                            __pCsr_ref.aBuffer.offset(nDoclist as isize) as *mut ::core::ffi::c_char,
                            iDelta,
                        );
                        iPrev = iDocid;
                        if isRequirePos != 0 {
                            ::core::ptr::copy_nonoverlapping(
                    pList as *const u8,
                    __pCsr_ref.aBuffer.offset(nDoclist as isize)
                                    as *mut ::core::ffi::c_char as *mut u8,
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
                rc = fts3GrowSegReaderBuffer(pCsr, nDoclist as crate::src::ext::rtree::rtree::i64_0 + FTS3_NODE_PADDING as crate::src::ext::rtree::rtree::i64_0);
                if rc != 0 {
                    return rc;
                }
                ::libc::memset(
                    __pCsr_ref.aBuffer.offset(nDoclist as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    FTS3_NODE_PADDING as crate::__stddef_size_t_h::size_t,
                );
                __pCsr_ref.aDoclist = __pCsr_ref.aBuffer;
                __pCsr_ref.nDoclist = nDoclist;
                rc = crate::src::headers::sqlite3_h::SQLITE_ROW;
            }
        }
        __pCsr_ref.nAdvance = nMerge;
        if !(rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
            break;
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3SegReaderFinish(mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader) {
    if !pCsr.is_null() {
        let mut i: ::core::ffi::c_int = 0;
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
    mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt,
    mut iCol: ::core::ffi::c_int,
    mut piEndBlock: *mut crate::src::ext::rtree::rtree::i64_0,
    mut pnByte: *mut crate::src::ext::rtree::rtree::i64_0,
) {
    let mut zText: *const ::core::ffi::c_uchar = crate::src::src::vdbeapi::sqlite3_column_text(pStmt, iCol);
    if !zText.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut iMul: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut iVal: crate::src::ext::rtree::rtree::u64_0 = 0 as crate::src::ext::rtree::rtree::u64_0;
        i = 0 as ::core::ffi::c_int;
        while *zText.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
            && *zText.offset(i as isize) as ::core::ffi::c_int <= '9' as i32
        {
            iVal = iVal.wrapping_mul(10 as crate::src::ext::rtree::rtree::u64_0).wrapping_add(
                (*zText.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as crate::src::ext::rtree::rtree::u64_0,
            );
            i += 1;
        }
        *piEndBlock = iVal as crate::src::ext::rtree::rtree::i64_0;
        while *zText.offset(i as isize) as ::core::ffi::c_int == ' ' as i32 {
            i += 1;
        }
        iVal = 0 as crate::src::ext::rtree::rtree::u64_0;
        if *zText.offset(i as isize) as ::core::ffi::c_int == '-' as i32 {
            i += 1;
            iMul = -(1 as ::core::ffi::c_int);
        }
        while *zText.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
            && *zText.offset(i as isize) as ::core::ffi::c_int <= '9' as i32
        {
            iVal = iVal.wrapping_mul(10 as crate::src::ext::rtree::rtree::u64_0).wrapping_add(
                (*zText.offset(i as isize) as ::core::ffi::c_int - '0' as i32) as crate::src::ext::rtree::rtree::u64_0,
            );
            i += 1;
        }
        *pnByte = iVal as crate::src::ext::rtree::rtree::i64_0 * iMul as crate::src::ext::rtree::rtree::i64_0;
    }
}

unsafe extern "C" fn fts3PromoteSegments(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut nByte: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pRange: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_LEVEL_RANGE2,
        &raw mut pRange,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut bOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iLast: crate::src::ext::rtree::rtree::i64_0 = (iAbsLevel as crate::src::ext::rtree::rtree::i64_0 / crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL as crate::src::ext::rtree::rtree::i64_0 + 1 as crate::src::ext::rtree::rtree::i64_0)
            * crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL as crate::src::ext::rtree::rtree::i64_0
            - 1 as crate::src::ext::rtree::rtree::i64_0;
        let mut nLimit: crate::src::ext::rtree::rtree::i64_0 = nByte as crate::src::ext::rtree::rtree::i64_0 * 3 as crate::src::ext::rtree::rtree::i64_0 / 2 as crate::src::ext::rtree::rtree::i64_0;
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pRange,
            1 as ::core::ffi::c_int,
            iAbsLevel + 1 as crate::src::headers::sqlite3_h::sqlite3_int64,
        );
        crate::src::src::vdbeapi::sqlite3_bind_int64(pRange, 2 as ::core::ffi::c_int, iLast as crate::src::headers::sqlite3_h::sqlite3_int64);
        while crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pRange) {
            let mut nSize: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
            let mut dummy: crate::src::ext::rtree::rtree::i64_0 = 0;
            fts3ReadEndBlockField(
                pRange,
                2 as ::core::ffi::c_int,
                &raw mut dummy,
                &raw mut nSize,
            );
            if nSize <= 0 as crate::src::ext::rtree::rtree::i64_0 || nSize > nLimit {
                bOk = 0 as ::core::ffi::c_int;
                break;
            } else {
                bOk = 1 as ::core::ffi::c_int;
            }
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pRange);
        if bOk != 0 {
            let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pUpdate1: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
            let mut pUpdate2: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3SqlStmt(
                    p,
                    SQL_UPDATE_LEVEL_IDX,
                    &raw mut pUpdate1,
                    ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
                );
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3SqlStmt(
                    p,
                    SQL_UPDATE_LEVEL,
                    &raw mut pUpdate2,
                    ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
                );
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::vdbeapi::sqlite3_bind_int64(pRange, 1 as ::core::ffi::c_int, iAbsLevel);
                while crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pRange) {
                    let fresh3 = iIdx;
                    iIdx += 1;
                    crate::src::src::vdbeapi::sqlite3_bind_int(pUpdate1, 1 as ::core::ffi::c_int, fresh3);
                    crate::src::src::vdbeapi::sqlite3_bind_int(
                        pUpdate1,
                        2 as ::core::ffi::c_int,
                        crate::src::src::vdbeapi::sqlite3_column_int(pRange, 0 as ::core::ffi::c_int),
                    );
                    crate::src::src::vdbeapi::sqlite3_bind_int(
                        pUpdate1,
                        3 as ::core::ffi::c_int,
                        crate::src::src::vdbeapi::sqlite3_column_int(pRange, 1 as ::core::ffi::c_int),
                    );
                    crate::src::src::vdbeapi::sqlite3_step(pUpdate1);
                    rc = crate::src::src::vdbeapi::sqlite3_reset(pUpdate1);
                    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
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
                crate::src::src::vdbeapi::sqlite3_bind_int64(pUpdate2, 1 as ::core::ffi::c_int, iAbsLevel);
                crate::src::src::vdbeapi::sqlite3_step(pUpdate2);
                rc = crate::src::src::vdbeapi::sqlite3_reset(pUpdate2);
            }
        }
    }
    rc
}

unsafe extern "C" fn fts3SegmentMerge(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iLevel: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int = 0;
    let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iNewLevel: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
    let mut pWriter: *mut SegmentWriter = ::core::ptr::null_mut::<SegmentWriter>();
    let mut filter: crate::fts3Int_h::Fts3SegFilter = unsafe { ::core::mem::zeroed() };
    let mut csr: crate::fts3Int_h::Fts3MultiSegReader = crate::fts3Int_h::Fts3MultiSegReader {
    apSegment:  ::core::ptr::null_mut::<*mut Fts3SegReader>(),
    nSegment:  0,
    nAdvance:  0,
    pFilter:  ::core::ptr::null_mut::<crate::fts3Int_h::Fts3SegFilter>(),
    aBuffer:  ::core::ptr::null_mut::<::core::ffi::c_char>(),
    nBuffer:  0,
    iColFilter:  0,
    bRestart:  0,
    nCost:  0,
    bLookup:  0,
    zTerm:  ::core::ptr::null_mut::<::core::ffi::c_char>(),
    nTerm:  0,
    aDoclist:  ::core::ptr::null_mut::<::core::ffi::c_char>(),
    nDoclist:  0,
};
    let mut bIgnoreEmpty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iMaxLevel: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
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
    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK || csr.nSegment == 0 as ::core::ffi::c_int) {
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
                            == ((**csr.apSegment.offset(0 as isize))
                                .ppNextElem
                                != ::core::ptr::null_mut::<*mut crate::src::ext::fts3::fts3_hash::Fts3HashElem>())
                                as ::core::ffi::c_int
                    {
                        rc = crate::src::headers::sqlite3_h::SQLITE_DONE;
                        current_block = 14255763095006029735;
                    } else {
                        iNewLevel = iMaxLevel as crate::src::headers::sqlite3_h::sqlite3_int64;
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
                    bIgnoreEmpty = (iLevel != crate::fts3Int_h::FTS3_SEGCURSOR_PENDING && iNewLevel > iMaxLevel)
                        as ::core::ffi::c_int;
                    current_block = 15652330335145281839;
                }
                match current_block {
                    14255763095006029735 => {}
                    _ => {
                        if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
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
                            if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
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
                                            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                                if iLevel == crate::fts3Int_h::FTS3_SEGCURSOR_PENDING
                                                    || iNewLevel < iMaxLevel
                                                {
                                                    rc = fts3PromoteSegments(
                                                        p,
                                                        iNewLevel,
                                                        (*pWriter).nLeafData as crate::src::headers::sqlite3_h::sqlite3_int64,
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
    }
    fts3SegWriterFree(pWriter);
    sqlite3Fts3SegReaderFinish(&raw mut csr);
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3PendingTermsFlush(mut p: *mut crate::fts3Int_h::Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    let __p_ref = unsafe { &mut *p };
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < __p_ref.nIndex {
        rc = fts3SegmentMerge(p, __p_ref.iPrevLangid, i, crate::fts3Int_h::FTS3_SEGCURSOR_PENDING);
        if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        i += 1;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && __p_ref.bHasStat as ::core::ffi::c_int != 0
        && __p_ref.nAutoincrmerge == 0xff as ::core::ffi::c_int
        && __p_ref.nLeafAdd > 0 as crate::src::ext::rtree::rtree::u32_0
    {
        let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            SQL_SELECT_STAT,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, FTS_STAT_AUTOINCRMERGE);
            rc = crate::src::src::vdbeapi::sqlite3_step(pStmt);
            if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
                __p_ref.nAutoincrmerge = crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
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
    mut N: ::core::ffi::c_int,
    mut a: *mut crate::src::ext::rtree::rtree::u32_0,
    mut zBuf: *mut ::core::ffi::c_char,
    mut pNBuf: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    i = j;
    while i < N {
        j += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
            zBuf.offset(j as isize) as *mut ::core::ffi::c_char,
            *a.offset(i as isize) as crate::src::headers::sqlite3_h::sqlite3_int64,
        );
        i += 1;
    }
    *pNBuf = j;
}

unsafe extern "C" fn fts3DecodeIntArray(
    mut N: ::core::ffi::c_int,
    mut a: *mut crate::src::ext::rtree::rtree::u32_0,
    mut zBuf: *const ::core::ffi::c_char,
    mut nBuf: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nBuf != 0
        && *zBuf.offset((nBuf - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        i = j;
        while i < N && j < nBuf {
            let mut x: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
            j += crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(
                zBuf.offset(j as isize) as *const ::core::ffi::c_char,
                &raw mut x,
            );
            *a.offset(i as isize) = (x & 0xffffffff as crate::src::headers::sqlite3_h::sqlite3_int64) as crate::src::ext::rtree::rtree::u32_0;
            i += 1;
        }
    }
    while i < N {
        let fresh1 = i;
        i += 1;
        *a.offset(fresh1 as isize) = 0 as crate::src::ext::rtree::rtree::u32_0;
    }
}

unsafe extern "C" fn fts3InsertDocsize(
    mut pRC: *mut ::core::ffi::c_int,
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut aSz: *mut crate::src::ext::rtree::rtree::u32_0,
) {
    let mut pBlob: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nBlob: ::core::ffi::c_int = 0;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    if *pRC != 0 {
        return;
    }
    let __p_ref = unsafe { &*p };
    pBlob =
        crate::src::src::malloc::sqlite3_malloc64((10 as crate::src::headers::sqlite3_h::sqlite3_int64 * __p_ref.nColumn as crate::src::headers::sqlite3_h::sqlite3_int64) as crate::src::headers::sqlite3_h::sqlite3_uint64)
            as *mut ::core::ffi::c_char;
    if pBlob.is_null() {
        *pRC = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        return;
    }
    fts3EncodeIntArray(__p_ref.nColumn, aSz, pBlob, &raw mut nBlob);
    rc = fts3SqlStmt(
        p,
        SQL_REPLACE_DOCSIZE,
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
        __p_ref.iPrevDocid as crate::src::headers::sqlite3_h::sqlite3_int64,
    );
    crate::src::src::vdbeapi::sqlite3_bind_blob(
        pStmt,
        2 as ::core::ffi::c_int,
        pBlob as *const ::core::ffi::c_void,
        nBlob,
        Some(crate::src::src::malloc::sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    crate::src::src::vdbeapi::sqlite3_step(pStmt);
    *pRC = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
}

unsafe extern "C" fn fts3UpdateDocTotals(
    mut pRC: *mut ::core::ffi::c_int,
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut aSzIns: *mut crate::src::ext::rtree::rtree::u32_0,
    mut aSzDel: *mut crate::src::ext::rtree::rtree::u32_0,
    mut nChng: ::core::ffi::c_int,
) {
    let mut pBlob: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nBlob: ::core::ffi::c_int = 0;
    let mut a: *mut crate::src::ext::rtree::rtree::u32_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u32_0>();
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut i: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let nStat: ::core::ffi::c_int = (*p).nColumn + 2 as ::core::ffi::c_int;
    if *pRC != 0 {
        return;
    }
    a = crate::src::src::malloc::sqlite3_malloc64(
        ((::core::mem::size_of::<crate::src::ext::rtree::rtree::u32_0>() as usize).wrapping_add(10 as usize) as crate::src::headers::sqlite3_h::sqlite3_uint64)
            .wrapping_mul(nStat as crate::src::headers::sqlite3_h::sqlite3_int64 as crate::src::headers::sqlite3_h::sqlite3_uint64),
    ) as *mut crate::src::ext::rtree::rtree::u32_0;
    if a.is_null() {
        *pRC = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        return;
    }
    pBlob = a.offset(nStat as isize) as *mut crate::src::ext::rtree::rtree::u32_0 as *mut ::core::ffi::c_char;
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_STAT,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc != 0 {
        crate::src::src::malloc::sqlite3_free(a as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, FTS_STAT_DOCTOTAL);
    if crate::src::src::vdbeapi::sqlite3_step(pStmt) == crate::src::headers::sqlite3_h::SQLITE_ROW {
        fts3DecodeIntArray(
            nStat,
            a,
            crate::src::src::vdbeapi::sqlite3_column_blob(pStmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
            crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 0 as ::core::ffi::c_int),
        );
    } else {
        ::libc::memset(
            a as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<crate::src::ext::rtree::rtree::u32_0>() as crate::__stddef_size_t_h::size_t).wrapping_mul(nStat as crate::__stddef_size_t_h::size_t),
        );
    }
    rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::malloc::sqlite3_free(a as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    if nChng < 0 as ::core::ffi::c_int
        && *a.offset(0 as isize) < -nChng as crate::src::ext::rtree::rtree::u32_0
    {
        *a.offset(0 as isize) = 0 as crate::src::ext::rtree::rtree::u32_0;
    } else {
        let ref mut fresh0 = *a.offset(0 as isize);
        *fresh0 = (*fresh0).wrapping_add(nChng as crate::src::ext::rtree::rtree::u32_0);
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nColumn + 1 as ::core::ffi::c_int {
        let mut x: crate::src::ext::rtree::rtree::u32_0 = *a.offset((i + 1 as ::core::ffi::c_int) as isize);
        if x.wrapping_add(*aSzIns.offset(i as isize)) < *aSzDel.offset(i as isize) {
            x = 0 as crate::src::ext::rtree::rtree::u32_0;
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
        SQL_REPLACE_STAT,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc != 0 {
        crate::src::src::malloc::sqlite3_free(a as *mut ::core::ffi::c_void);
        *pRC = rc;
        return;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, FTS_STAT_DOCTOTAL);
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut bReturnDone: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bSeenDone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = 0;
    let mut pAllLangid: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    rc = sqlite3Fts3PendingTermsFlush(p);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = fts3SqlStmt(
            p,
            SQL_SELECT_ALL_LANGID,
            &raw mut pAllLangid,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut rc2: ::core::ffi::c_int = 0;
        crate::src::src::vdbeapi::sqlite3_bind_int(pAllLangid, 1 as ::core::ffi::c_int, (*p).iPrevLangid);
        crate::src::src::vdbeapi::sqlite3_bind_int(pAllLangid, 2 as ::core::ffi::c_int, (*p).nIndex);
        while crate::src::src::vdbeapi::sqlite3_step(pAllLangid) == crate::src::headers::sqlite3_h::SQLITE_ROW {
            let mut i: ::core::ffi::c_int = 0;
            let mut iLangid: ::core::ffi::c_int =
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
        rc2 = crate::src::src::vdbeapi::sqlite3_reset(pAllLangid);
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

unsafe extern "C" fn fts3DoRebuild(mut p: *mut crate::fts3Int_h::Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3DeleteAll(p, 0 as ::core::ffi::c_int);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut aSz: *mut crate::src::ext::rtree::rtree::u32_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u32_0>();
        let mut aSzIns: *mut crate::src::ext::rtree::rtree::u32_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u32_0>();
        let mut aSzDel: *mut crate::src::ext::rtree::rtree::u32_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u32_0>();
        let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        let mut nEntry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zSql: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3_mprintf(
            b"SELECT %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).zReadExprlist,
        );
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
            let mut nByte: crate::src::headers::sqlite3_h::sqlite3_int64 =
                (::core::mem::size_of::<crate::src::ext::rtree::rtree::u32_0>() as ::core::ffi::c_ulonglong)
                    .wrapping_mul(
                        ((*p).nColumn as crate::src::headers::sqlite3_h::sqlite3_int64 + 1 as crate::src::headers::sqlite3_h::sqlite3_int64)
                            as ::core::ffi::c_ulonglong,
                    )
                    .wrapping_mul(3 as ::core::ffi::c_ulonglong) as crate::src::headers::sqlite3_h::sqlite3_int64;
            aSz = crate::src::src::malloc::sqlite3_malloc64(nByte as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut crate::src::ext::rtree::rtree::u32_0;
            if aSz.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                ::libc::memset(
                    aSz as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nByte as crate::__stddef_size_t_h::size_t,
                );
                aSzIns =
                    aSz.offset(((*p).nColumn + 1 as ::core::ffi::c_int) as isize) as *mut crate::src::ext::rtree::rtree::u32_0;
                aSzDel =
                    aSzIns.offset(((*p).nColumn + 1 as ::core::ffi::c_int) as isize) as *mut crate::src::ext::rtree::rtree::u32_0;
            }
        }
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pStmt) {
            let mut iCol: ::core::ffi::c_int = 0;
            let mut iLangid: ::core::ffi::c_int = langidFromSelect(p, pStmt);
            rc = fts3PendingTermsDocid(
                p,
                0 as ::core::ffi::c_int,
                iLangid,
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::sqlite_int64,
            );
            let __p_ref = unsafe { &mut *p };
            ::libc::memset(
                aSz as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<crate::src::ext::rtree::rtree::u32_0>() as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul((__p_ref.nColumn + 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t),
            );
            iCol = 0 as ::core::ffi::c_int;
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && iCol < __p_ref.nColumn {
                if *__p_ref.abNotindexed.offset(iCol as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let mut z: *const ::core::ffi::c_char =
                        crate::src::src::vdbeapi::sqlite3_column_text(pStmt, iCol + 1 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    rc = fts3PendingTermsAdd(
                        p,
                        iLangid,
                        z,
                        iCol,
                        aSz.offset(iCol as isize) as *mut crate::src::ext::rtree::rtree::u32_0,
                    );
                    let ref mut fresh14 = *aSz.offset(__p_ref.nColumn as isize);
                    *fresh14 = (*fresh14)
                        .wrapping_add(
                            crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, iCol + 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::u32_0
                        );
                }
                iCol += 1;
            }
            if __p_ref.bHasDocsize != 0 {
                fts3InsertDocsize(&raw mut rc, p, aSz);
            }
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
                pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
            } else {
                nEntry += 1;
                iCol = 0 as ::core::ffi::c_int;
                while iCol <= __p_ref.nColumn {
                    let ref mut fresh15 = *aSzIns.offset(iCol as isize);
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
            let mut rc2: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = rc2;
            }
        }
    }
    rc
}

unsafe extern "C" fn fts3IncrmergeCsr(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut nSeg: ::core::ffi::c_int,
    mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut nByte: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
    ::libc::memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::fts3Int_h::Fts3MultiSegReader>() as crate::__stddef_size_t_h::size_t,
    );
    nByte = (::core::mem::size_of::<*mut Fts3SegReader>() as usize).wrapping_mul(nSeg as usize)
        as crate::src::headers::sqlite3_h::sqlite3_int64;
    (*pCsr).apSegment = crate::src::src::malloc::sqlite3_malloc64(nByte as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut *mut Fts3SegReader;
    if (*pCsr).apSegment.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        ::libc::memset(
            (*pCsr).apSegment as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nByte as crate::__stddef_size_t_h::size_t,
        );
        rc = fts3SqlStmt(
            p,
            SQL_SELECT_LEVEL,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut i: ::core::ffi::c_int = 0;
        let mut rc2: ::core::ffi::c_int = 0;
        crate::src::src::vdbeapi::sqlite3_bind_int64(pStmt, 1 as ::core::ffi::c_int, iAbsLevel);
        i = 0 as ::core::ffi::c_int;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && crate::src::src::vdbeapi::sqlite3_step(pStmt) == crate::src::headers::sqlite3_h::SQLITE_ROW && i < nSeg {
            rc = sqlite3Fts3SegReaderNew(
                i,
                0 as ::core::ffi::c_int,
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 1 as ::core::ffi::c_int),
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 2 as ::core::ffi::c_int),
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 3 as ::core::ffi::c_int),
                crate::src::src::vdbeapi::sqlite3_column_blob(pStmt, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
                crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 4 as ::core::ffi::c_int),
                (*pCsr).apSegment.offset(i as isize) as *mut *mut Fts3SegReader,
            );
            (*pCsr).nSegment += 1;
            i += 1;
        }
        rc2 = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    rc
}

unsafe extern "C" fn blobGrowBuffer(
    mut pBlob: *mut Blob,
    mut nMin: ::core::ffi::c_int,
    mut pRc: *mut ::core::ffi::c_int,
) {
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK && nMin > (*pBlob).nAlloc {
        let mut nAlloc: ::core::ffi::c_int = nMin;
        let mut a: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
            (*pBlob).a as *mut ::core::ffi::c_void,
            nAlloc as crate::src::headers::sqlite3_h::sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if !a.is_null() {
            (*pBlob).nAlloc = nAlloc;
            (*pBlob).a = a;
        } else {
            *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    }
}

unsafe extern "C" fn nodeReaderNext(mut p: *mut NodeReader) -> ::core::ffi::c_int {
    let __p_ref = unsafe { &mut *p };
    let mut bFirst: ::core::ffi::c_int =
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
            __p_ref.iOff += if *(__p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char
                as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(
                    __p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char,
                    &raw mut nPrefix,
                )
            } else {
                nPrefix = *(__p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char
                    as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            };
        }
        __p_ref.iOff += if *(__p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char
            as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
        {
            crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(
                __p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char,
                &raw mut nSuffix,
            )
        } else {
            nSuffix = *(__p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char
                as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int;
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
            if __p_ref.iChild == 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
                __p_ref.iOff += if *(__p_ref.aNode.offset(__p_ref.iOff as isize)
                    as *const ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0)
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
                        as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                };
                if __p_ref.nNode - __p_ref.iOff < __p_ref.nDoclist {
                    return crate::fts3Int_h::FTS_CORRUPT_VTAB;
                }
                __p_ref.aDoclist = __p_ref.aNode.offset(__p_ref.iOff as isize) as *const ::core::ffi::c_char;
                __p_ref.iOff += __p_ref.nDoclist;
            }
        }
    }
    rc
}

unsafe extern "C" fn nodeReaderRelease(mut p: *mut NodeReader) {
    crate::src::src::malloc::sqlite3_free((*p).term.a as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn nodeReaderInit(
    mut p: *mut NodeReader,
    mut aNode: *const ::core::ffi::c_char,
    mut nNode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<NodeReader>() as crate::__stddef_size_t_h::size_t,
    );
    (*p).aNode = aNode;
    (*p).nNode = nNode;
    if !aNode.is_null()
        && *aNode.offset(0 as isize) as ::core::ffi::c_int != 0
    {
        let __p_ref = unsafe { &mut *p };
        __p_ref.iOff = 1 as ::core::ffi::c_int
            + crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(
                __p_ref.aNode.offset(1 as isize) as *const ::core::ffi::c_char,
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pWriter: *mut IncrmergeWriter,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut iPtr: crate::src::headers::sqlite3_h::sqlite3_int64 = (*pWriter).aNodeWriter[0 as ::core::ffi::c_int as usize].iBlock;
    let mut iLayer: ::core::ffi::c_int = 0;
    iLayer = 1 as ::core::ffi::c_int;
    while iLayer < 16 as ::core::ffi::c_int {
        let mut iNextPtr: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
        let mut pNode: *mut NodeWriter = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
            .offset(iLayer as isize) as *mut NodeWriter;
        let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
        let mut nPrefix: ::core::ffi::c_int = 0;
        let mut nSuffix: ::core::ffi::c_int = 0;
        let mut nSpace: ::core::ffi::c_int = 0;
        let __pNode_ref = unsafe { &mut *pNode };
        nPrefix = fts3PrefixCompress(__pNode_ref.key.a, __pNode_ref.key.n, zTerm, nTerm);
        nSuffix = nTerm - nPrefix;
        if nSuffix <= 0 as ::core::ffi::c_int {
            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
        }
        nSpace = crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nPrefix as crate::src::headers::sqlite3_h::sqlite3_uint64);
        nSpace += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nSuffix as crate::src::headers::sqlite3_h::sqlite3_uint64) + nSuffix;
        if __pNode_ref.key.n == 0 as ::core::ffi::c_int || __pNode_ref.block.n + nSpace <= (*p).nNodeSize
        {
            let mut pBlk: *mut Blob = &raw mut __pNode_ref.block;
            if (*pBlk).n == 0 as ::core::ffi::c_int {
                blobGrowBuffer(pBlk, (*p).nNodeSize, &raw mut rc);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    let __pBlk_ref = unsafe { &mut *pBlk };
                    *__pBlk_ref.a.offset(0 as isize) =
                        iLayer as ::core::ffi::c_char;
                    __pBlk_ref.n = 1 as ::core::ffi::c_int
                        + crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                            __pBlk_ref.a.offset(1 as isize)
                                as *mut ::core::ffi::c_char,
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
                        nPrefix as crate::src::headers::sqlite3_h::sqlite3_int64,
                    );
                }
                __pBlk_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                    __pBlk_ref.a.offset(__pBlk_ref.n as isize) as *mut ::core::ffi::c_char,
                    nSuffix as crate::src::headers::sqlite3_h::sqlite3_int64,
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
            rc = fts3WriteSegment(p, __pNode_ref.iBlock, __pNode_ref.block.a, __pNode_ref.block.n);
            *__pNode_ref.block.a.offset(0 as isize) =
                iLayer as ::core::ffi::c_char;
            __pNode_ref.block.n = 1 as ::core::ffi::c_int
                + crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                    __pNode_ref.block.a.offset(1 as isize)
                        as *mut ::core::ffi::c_char,
                    iPtr + 1 as crate::src::headers::sqlite3_h::sqlite3_int64,
                );
            iNextPtr = __pNode_ref.iBlock;
            __pNode_ref.iBlock += 1;
            __pNode_ref.key.n = 0 as ::core::ffi::c_int;
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK || iNextPtr == 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
            return rc;
        }
        iPtr = iNextPtr;
        iLayer += 1;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn fts3AppendToNode(
    mut pNode: *mut Blob,
    mut pPrev: *mut Blob,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut aDoclist: *const ::core::ffi::c_char,
    mut nDoclist: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pPrev_ref = unsafe { &mut *pPrev };
    let mut bFirst: ::core::ffi::c_int =
        (__pPrev_ref.n == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut nPrefix: ::core::ffi::c_int = 0;
    let mut nSuffix: ::core::ffi::c_int = 0;
    blobGrowBuffer(pPrev, nTerm, &raw mut rc);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    nPrefix = fts3PrefixCompress(__pPrev_ref.a, __pPrev_ref.n, zTerm, nTerm);
    nSuffix = nTerm - nPrefix;
    if nSuffix <= 0 as ::core::ffi::c_int {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    ::core::ptr::copy_nonoverlapping(
                    zTerm as *const u8,
                    __pPrev_ref.a as *mut u8,
                    nTerm as usize,
                );
    __pPrev_ref.n = nTerm;
    let __pNode_ref = unsafe { &mut *pNode };
    if bFirst == 0 as ::core::ffi::c_int {
        __pNode_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
            __pNode_ref.a.offset(__pNode_ref.n as isize) as *mut ::core::ffi::c_char,
            nPrefix as crate::src::headers::sqlite3_h::sqlite3_int64,
        );
    }
    __pNode_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
        __pNode_ref.a.offset(__pNode_ref.n as isize) as *mut ::core::ffi::c_char,
        nSuffix as crate::src::headers::sqlite3_h::sqlite3_int64,
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
            nDoclist as crate::src::headers::sqlite3_h::sqlite3_int64,
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pWriter: *mut IncrmergeWriter,
    mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let __pCsr_ref = unsafe { &*pCsr };
    let mut zTerm: *const ::core::ffi::c_char = __pCsr_ref.zTerm;
    let mut nTerm: ::core::ffi::c_int = __pCsr_ref.nTerm;
    let mut aDoclist: *const ::core::ffi::c_char = __pCsr_ref.aDoclist;
    let mut nDoclist: ::core::ffi::c_int = __pCsr_ref.nDoclist;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut nSpace: ::core::ffi::c_int = 0;
    let mut nPrefix: ::core::ffi::c_int = 0;
    let mut nSuffix: ::core::ffi::c_int = 0;
    let mut pLeaf: *mut NodeWriter = ::core::ptr::null_mut::<NodeWriter>();
    let __pWriter_ref = unsafe { &mut *pWriter };
    pLeaf = (&raw mut __pWriter_ref.aNodeWriter as *mut NodeWriter)
        .offset(0 as isize) as *mut NodeWriter;
    nPrefix = fts3PrefixCompress((*pLeaf).key.a, (*pLeaf).key.n, zTerm, nTerm);
    nSuffix = nTerm - nPrefix;
    if nSuffix <= 0 as ::core::ffi::c_int {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    nSpace = crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nPrefix as crate::src::headers::sqlite3_h::sqlite3_uint64);
    nSpace += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nSuffix as crate::src::headers::sqlite3_h::sqlite3_uint64) + nSuffix;
    nSpace += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nDoclist as crate::src::headers::sqlite3_h::sqlite3_uint64) + nDoclist;
    if (*pLeaf).block.n > 0 as ::core::ffi::c_int
        && (*pLeaf).block.n + nSpace > (*p).nNodeSize
        && (*pLeaf).iBlock < __pWriter_ref.iStart as crate::src::ext::rtree::rtree::i64_0 + __pWriter_ref.nLeafEst
    {
        let __pLeaf_ref = unsafe { &mut *pLeaf };
        rc = fts3WriteSegment(p, __pLeaf_ref.iBlock, __pLeaf_ref.block.a, __pLeaf_ref.block.n);
        __pWriter_ref.nWork += 1;
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = fts3IncrmergePush(p, pWriter, zTerm, nPrefix + 1 as ::core::ffi::c_int);
        }
        __pLeaf_ref.iBlock += 1;
        __pLeaf_ref.key.n = 0 as ::core::ffi::c_int;
        __pLeaf_ref.block.n = 0 as ::core::ffi::c_int;
        nSuffix = nTerm;
        nSpace = 1 as ::core::ffi::c_int;
        nSpace += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nSuffix as crate::src::headers::sqlite3_h::sqlite3_uint64) + nSuffix;
        nSpace += crate::src::ext::fts3::fts3::sqlite3Fts3VarintLen(nDoclist as crate::src::headers::sqlite3_h::sqlite3_uint64) + nDoclist;
    }
    __pWriter_ref.nLeafData += nSpace as crate::src::headers::sqlite3_h::sqlite3_int64;
    blobGrowBuffer(
        &raw mut (*pLeaf).block,
        (*pLeaf).block.n + nSpace,
        &raw mut rc,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pLeaf_ref = unsafe { &mut *pLeaf };
        if __pLeaf_ref.block.n == 0 as ::core::ffi::c_int {
            __pLeaf_ref.block.n = 1 as ::core::ffi::c_int;
            *__pLeaf_ref.block.a.offset(0 as isize) =
                '\0' as i32 as ::core::ffi::c_char;
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pWriter: *mut IncrmergeWriter,
    mut pRc: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut iRoot: ::core::ffi::c_int = 0;
    let mut pRoot: *mut NodeWriter = ::core::ptr::null_mut::<NodeWriter>();
    let mut rc: ::core::ffi::c_int = *pRc;
    iRoot = FTS_MAX_APPENDABLE_HEIGHT - 1 as ::core::ffi::c_int;
    while iRoot >= 0 as ::core::ffi::c_int {
        let mut pNode: *mut NodeWriter = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
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
        let mut pBlock: *mut Blob = &raw mut (*(&raw mut (*pWriter).aNodeWriter
            as *mut NodeWriter)
            .offset(1 as isize))
        .block;
        blobGrowBuffer(
            pBlock,
            1 as ::core::ffi::c_int + crate::fts3Int_h::FTS3_VARINT_MAX,
            &raw mut rc,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let __pBlock_ref = unsafe { &mut *pBlock };
            *__pBlock_ref.a.offset(0 as isize) = 0x1 as ::core::ffi::c_char;
            __pBlock_ref.n = 1 as ::core::ffi::c_int
                + crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                    __pBlock_ref.a.offset(1 as isize)
                        as *mut ::core::ffi::c_char,
                    (*pWriter).aNodeWriter[0 as ::core::ffi::c_int as usize].iBlock,
                );
        }
        iRoot = 1 as ::core::ffi::c_int;
    }
    pRoot = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter).offset(iRoot as isize)
        as *mut NodeWriter;
    i = 0 as ::core::ffi::c_int;
    while i < iRoot {
        let mut pNode_0: *mut NodeWriter = (&raw mut (*pWriter).aNodeWriter as *mut NodeWriter)
            .offset(i as isize) as *mut NodeWriter;
        let __pNode_0_ref = unsafe { &*pNode_0 };
        if __pNode_0_ref.block.n > 0 as ::core::ffi::c_int && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = fts3WriteSegment(p, __pNode_0_ref.iBlock, __pNode_0_ref.block.a, __pNode_0_ref.block.n);
        }
        crate::src::src::malloc::sqlite3_free(__pNode_0_ref.block.a as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(__pNode_0_ref.key.a as *mut ::core::ffi::c_void);
        i += 1;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pWriter_ref = unsafe { &*pWriter };
        rc = fts3WriteSegdir(
            p,
            __pWriter_ref.iAbsLevel + 1 as crate::src::headers::sqlite3_h::sqlite3_int64,
            __pWriter_ref.iIdx,
            __pWriter_ref.iStart,
            __pWriter_ref.aNodeWriter[0 as ::core::ffi::c_int as usize].iBlock,
            __pWriter_ref.iEnd,
            if __pWriter_ref.bNoLeafData as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                __pWriter_ref.nLeafData
            } else {
                0 as crate::src::headers::sqlite3_h::sqlite3_int64
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
    mut zLhs: *const ::core::ffi::c_char,
    mut nLhs: ::core::ffi::c_int,
    mut zRhs: *const ::core::ffi::c_char,
    mut nRhs: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nCmp: ::core::ffi::c_int = if nLhs < nRhs { nLhs } else { nRhs };
    let mut res: ::core::ffi::c_int = 0;
    if nCmp != 0 && !zLhs.is_null() && !zRhs.is_null() {
        res = ::libc::memcmp(
            zLhs as *const ::core::ffi::c_void,
            zRhs as *const ::core::ffi::c_void,
            nCmp as crate::__stddef_size_t_h::size_t,
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iEnd: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut pbRes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bRes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pCheck: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SqlStmt(
        p,
        SQL_SEGMENT_IS_APPENDABLE,
        &raw mut pCheck,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pCheck, 1 as ::core::ffi::c_int, iEnd);
        if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pCheck) {
            bRes = 1 as ::core::ffi::c_int;
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pCheck);
    }
    *pbRes = bRes;
    rc
}

unsafe extern "C" fn fts3IncrmergeLoad(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
    mut zKey: *const ::core::ffi::c_char,
    mut nKey: ::core::ffi::c_int,
    mut pWriter: *mut IncrmergeWriter,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pSelect: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_SEGDIR,
        &raw mut pSelect,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut iStart: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
        let mut iLeafEnd: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
        let mut iEnd: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
        let mut aRoot: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut nRoot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc2: ::core::ffi::c_int = 0;
        let mut bAppendable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pSelect,
            1 as ::core::ffi::c_int,
            iAbsLevel + 1 as crate::src::headers::sqlite3_h::sqlite3_int64,
        );
        crate::src::src::vdbeapi::sqlite3_bind_int(pSelect, 2 as ::core::ffi::c_int, iIdx);
        if crate::src::src::vdbeapi::sqlite3_step(pSelect) == crate::src::headers::sqlite3_h::SQLITE_ROW {
            iStart = crate::src::src::vdbeapi::sqlite3_column_int64(pSelect, 1 as ::core::ffi::c_int);
            iLeafEnd = crate::src::src::vdbeapi::sqlite3_column_int64(pSelect, 2 as ::core::ffi::c_int);
            let __pWriter_ref = unsafe { &mut *pWriter };
            fts3ReadEndBlockField(
                pSelect,
                3 as ::core::ffi::c_int,
                &raw mut iEnd,
                &raw mut __pWriter_ref.nLeafData,
            );
            if __pWriter_ref.nLeafData < 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
                __pWriter_ref.nLeafData *= -(1 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::sqlite3_int64;
            }
            __pWriter_ref.bNoLeafData =
                (__pWriter_ref.nLeafData == 0 as crate::src::headers::sqlite3_h::sqlite3_int64) as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0;
            nRoot = crate::src::src::vdbeapi::sqlite3_column_bytes(pSelect, 4 as ::core::ffi::c_int);
            aRoot =
                crate::src::src::vdbeapi::sqlite3_column_blob(pSelect, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
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
                let mut reader: NodeReader = NodeReader {
                    aNode: ::core::ptr::null::<::core::ffi::c_char>(),
                    nNode: 0,
                    iOff: 0,
                    iChild: 0,
                    term: Blob {
                        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        n: 0,
                        nAlloc: 0,
                    },
                    aDoclist: ::core::ptr::null::<::core::ffi::c_char>(),
                    nDoclist: 0,
                };
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
            let mut i: ::core::ffi::c_int = 0;
            let mut nHeight: ::core::ffi::c_int =
                *aRoot.offset(0 as isize) as ::core::ffi::c_int;
            let mut pNode: *mut NodeWriter = ::core::ptr::null_mut::<NodeWriter>();
            if nHeight < 1 as ::core::ffi::c_int || nHeight >= FTS_MAX_APPENDABLE_HEIGHT {
                crate::src::src::vdbeapi::sqlite3_reset(pSelect);
                return crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
            let __pWriter_ref = unsafe { &mut *pWriter };
            __pWriter_ref.nLeafEst = ((iEnd - iStart + 1 as crate::src::headers::sqlite3_h::sqlite3_int64) as ::core::ffi::c_int
                / FTS_MAX_APPENDABLE_HEIGHT) as crate::src::ext::rtree::rtree::i64_0;
            __pWriter_ref.iStart = iStart;
            __pWriter_ref.iEnd = iEnd;
            __pWriter_ref.iAbsLevel = iAbsLevel;
            __pWriter_ref.iIdx = iIdx;
            i = nHeight + 1 as ::core::ffi::c_int;
            while i < FTS_MAX_APPENDABLE_HEIGHT {
                __pWriter_ref.aNodeWriter[i as usize].iBlock = (__pWriter_ref.iStart as crate::src::ext::rtree::rtree::i64_0
                    + i as crate::src::ext::rtree::rtree::i64_0 * __pWriter_ref.nLeafEst)
                    as crate::src::headers::sqlite3_h::sqlite3_int64;
                i += 1;
            }
            pNode = (&raw mut __pWriter_ref.aNodeWriter as *mut NodeWriter).offset(nHeight as isize)
                as *mut NodeWriter;
            (*pNode).iBlock = (__pWriter_ref.iStart as crate::src::ext::rtree::rtree::i64_0 + __pWriter_ref.nLeafEst * nHeight as crate::src::ext::rtree::rtree::i64_0)
                as crate::src::headers::sqlite3_h::sqlite3_int64;
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
                ::libc::memset(
                    __pNode_ref.block.a.offset(nRoot as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    FTS3_NODE_PADDING as crate::__stddef_size_t_h::size_t,
                );
            }
            i = nHeight;
            while i >= 0 as ::core::ffi::c_int && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let mut reader_0: NodeReader = unsafe { ::core::mem::zeroed() };
                pNode = (&raw mut __pWriter_ref.aNodeWriter as *mut NodeWriter).offset(i as isize)
                    as *mut NodeWriter;
                if !(*pNode).block.a.is_null() {
                    rc = nodeReaderInit(&raw mut reader_0, (*pNode).block.a, (*pNode).block.n);
                    while !reader_0.aNode.is_null() && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
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
                                ::libc::memset(
                                    __pNode_ref.block.a.offset(nBlock as isize)
                                        as *mut ::core::ffi::c_char
                                        as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    FTS3_NODE_PADDING as crate::__stddef_size_t_h::size_t,
                                );
                            }
                            crate::src::src::malloc::sqlite3_free(aBlock as *mut ::core::ffi::c_void);
                        }
                    }
                }
                nodeReaderRelease(&raw mut reader_0);
                i -= 1;
            }
        }
        rc2 = crate::src::src::vdbeapi::sqlite3_reset(pSelect);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    rc
}

unsafe extern "C" fn fts3IncrmergeOutputIdx(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut piIdx: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pOutputIdx: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_NEXT_SEGMENT_INDEX,
        &raw mut pOutputIdx,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pOutputIdx,
            1 as ::core::ffi::c_int,
            iAbsLevel + 1 as crate::src::headers::sqlite3_h::sqlite3_int64,
        );
        crate::src::src::vdbeapi::sqlite3_step(pOutputIdx);
        *piIdx = crate::src::src::vdbeapi::sqlite3_column_int(pOutputIdx, 0 as ::core::ffi::c_int);
        rc = crate::src::src::vdbeapi::sqlite3_reset(pOutputIdx);
    }
    rc
}

unsafe extern "C" fn fts3IncrmergeWriter(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
    mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    mut pWriter: *mut IncrmergeWriter,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut nLeafEst: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
    let mut pLeafEst: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut pFirstBlock: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_MAX_LEAF_NODE_ESTIMATE,
        &raw mut pLeafEst,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pLeafEst, 1 as ::core::ffi::c_int, iAbsLevel);
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            pLeafEst,
            2 as ::core::ffi::c_int,
            (*pCsr).nSegment as crate::src::headers::sqlite3_h::sqlite3_int64,
        );
        if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pLeafEst) {
            nLeafEst = crate::src::src::vdbeapi::sqlite3_column_int64(pLeafEst, 0 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pLeafEst);
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    rc = fts3SqlStmt(
        p,
        SQL_NEXT_SEGMENTS_ID,
        &raw mut pFirstBlock,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    let __pWriter_ref = unsafe { &mut *pWriter };
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pFirstBlock) {
            __pWriter_ref.iStart = crate::src::src::vdbeapi::sqlite3_column_int64(pFirstBlock, 0 as ::core::ffi::c_int);
            __pWriter_ref.iEnd = __pWriter_ref.iStart - 1 as crate::src::headers::sqlite3_h::sqlite3_int64;
            __pWriter_ref.iEnd += (nLeafEst * FTS_MAX_APPENDABLE_HEIGHT as crate::src::ext::rtree::rtree::i64_0) as crate::src::headers::sqlite3_h::sqlite3_int64;
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
        __pWriter_ref.aNodeWriter[i as usize].iBlock =
            (__pWriter_ref.iStart as crate::src::ext::rtree::rtree::i64_0 + i as crate::src::ext::rtree::rtree::i64_0 * __pWriter_ref.nLeafEst) as crate::src::headers::sqlite3_h::sqlite3_int64;
        i += 1;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3RemoveSegdirEntry(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pDelete: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_DELETE_SEGDIR_ENTRY,
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut aIdx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut nIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nAlloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut pSelect: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut pUpdate: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_INDEXES,
        &raw mut pSelect,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut rc2: ::core::ffi::c_int = 0;
        crate::src::src::vdbeapi::sqlite3_bind_int64(pSelect, 1 as ::core::ffi::c_int, iAbsLevel);
        while crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pSelect) {
            if nIdx >= nAlloc {
                let mut aNew: *mut ::core::ffi::c_int =
                    ::core::ptr::null_mut::<::core::ffi::c_int>();
                nAlloc += 16 as ::core::ffi::c_int;
                aNew = crate::src::src::malloc::sqlite3_realloc64(
                    aIdx as *mut ::core::ffi::c_void,
                    (nAlloc as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        as crate::src::headers::sqlite3_h::sqlite3_uint64,
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
            *aIdx.offset(fresh13 as isize) = crate::src::src::vdbeapi::sqlite3_column_int(pSelect, 0 as ::core::ffi::c_int);
        }
        rc2 = crate::src::src::vdbeapi::sqlite3_reset(pSelect);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = fts3SqlStmt(
            p,
            SQL_SHIFT_SEGDIR_ENTRY,
            &raw mut pUpdate,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(pUpdate, 2 as ::core::ffi::c_int, iAbsLevel);
    }
    (*p).bIgnoreSavepoint = 1 as crate::src::ext::rtree::rtree::u8_0;
    i = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < nIdx {
        if *aIdx.offset(i as isize) != i {
            crate::src::src::vdbeapi::sqlite3_bind_int(pUpdate, 3 as ::core::ffi::c_int, *aIdx.offset(i as isize));
            crate::src::src::vdbeapi::sqlite3_bind_int(pUpdate, 1 as ::core::ffi::c_int, i);
            crate::src::src::vdbeapi::sqlite3_step(pUpdate);
            rc = crate::src::src::vdbeapi::sqlite3_reset(pUpdate);
        }
        i += 1;
    }
    (*p).bIgnoreSavepoint = 0 as crate::src::ext::rtree::rtree::u8_0;
    crate::src::src::malloc::sqlite3_free(aIdx as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn fts3StartNode(
    mut pNode: *mut Blob,
    mut iHeight: ::core::ffi::c_int,
    mut iChild: crate::src::headers::sqlite3_h::sqlite3_int64,
) {
    *(*pNode).a.offset(0 as isize) = iHeight as ::core::ffi::c_char;
    if iChild != 0 {
        (*pNode).n = 1 as ::core::ffi::c_int
            + crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
                (*pNode).a.offset(1 as isize) as *mut ::core::ffi::c_char,
                iChild,
            );
    } else {
        (*pNode).n = 1 as ::core::ffi::c_int;
    };
}

unsafe extern "C" fn fts3TruncateNode(
    mut aNode: *const ::core::ffi::c_char,
    mut nNode: ::core::ffi::c_int,
    mut pNew: *mut Blob,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut piBlock: *mut crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut reader: NodeReader = NodeReader {
        aNode: ::core::ptr::null::<::core::ffi::c_char>(),
        nNode: 0,
        iOff: 0,
        iChild: 0,
        term: Blob {
            a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            n: 0,
            nAlloc: 0,
        },
        aDoclist: ::core::ptr::null::<::core::ffi::c_char>(),
        nDoclist: 0,
    };
    let mut prev: Blob = Blob {
        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        n: 0 as ::core::ffi::c_int,
        nAlloc: 0 as ::core::ffi::c_int,
    };
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut bLeaf: ::core::ffi::c_int = 0;
    if nNode < 1 as ::core::ffi::c_int {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    bLeaf = (*aNode.offset(0 as isize) as ::core::ffi::c_int == '\0' as i32)
        as ::core::ffi::c_int;
    blobGrowBuffer(pNew, nNode, &raw mut rc);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    (*pNew).n = 0 as ::core::ffi::c_int;
    let mut current_block_10: u64;
    rc = nodeReaderInit(&raw mut reader, aNode, nNode);
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && !reader.aNode.is_null() {
        if (*pNew).n == 0 as ::core::ffi::c_int {
            let mut res: ::core::ffi::c_int =
                fts3TermCmp(reader.term.a, reader.term.n, zTerm, nTerm);
            if res < 0 as ::core::ffi::c_int
                || bLeaf == 0 as ::core::ffi::c_int && res == 0 as ::core::ffi::c_int
            {
                current_block_10 = 4906268039856690917;
            } else {
                fts3StartNode(
                    pNew,
                    *aNode.offset(0 as isize) as ::core::ffi::c_int,
                    reader.iChild,
                );
                *piBlock = reader.iChild;
                current_block_10 = 12209867499936983673;
            }
        } else {
            current_block_10 = 12209867499936983673;
        }
        match current_block_10 {
            12209867499936983673 => {
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
            _ => {}
        }
        rc = nodeReaderNext(&raw mut reader);
    }
    if (*pNew).n == 0 as ::core::ffi::c_int {
        fts3StartNode(
            pNew,
            *aNode.offset(0 as isize) as ::core::ffi::c_int,
            reader.iChild,
        );
        *piBlock = reader.iChild;
    }
    nodeReaderRelease(&raw mut reader);
    crate::src::src::malloc::sqlite3_free(prev.a as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn fts3TruncateSegment(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut iIdx: ::core::ffi::c_int,
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut root: Blob = Blob {
        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        n: 0 as ::core::ffi::c_int,
        nAlloc: 0 as ::core::ffi::c_int,
    };
    let mut block: Blob = Blob {
        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        n: 0 as ::core::ffi::c_int,
        nAlloc: 0 as ::core::ffi::c_int,
    };
    let mut iBlock: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
    let mut iNewStart: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
    let mut iOldStart: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
    let mut pFetch: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_SEGDIR,
        &raw mut pFetch,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut rc2: ::core::ffi::c_int = 0;
        crate::src::src::vdbeapi::sqlite3_bind_int64(pFetch, 1 as ::core::ffi::c_int, iAbsLevel);
        crate::src::src::vdbeapi::sqlite3_bind_int(pFetch, 2 as ::core::ffi::c_int, iIdx);
        if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pFetch) {
            let mut aRoot: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_column_blob(pFetch, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            let mut nRoot: ::core::ffi::c_int =
                crate::src::src::vdbeapi::sqlite3_column_bytes(pFetch, 4 as ::core::ffi::c_int);
            iOldStart = crate::src::src::vdbeapi::sqlite3_column_int64(pFetch, 1 as ::core::ffi::c_int);
            rc = fts3TruncateNode(aRoot, nRoot, &raw mut root, zTerm, nTerm, &raw mut iBlock);
        }
        rc2 = crate::src::src::vdbeapi::sqlite3_reset(pFetch);
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
        let mut pDel: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            SQL_DELETE_SEGMENTS_RANGE,
            &raw mut pDel,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int64(pDel, 1 as ::core::ffi::c_int, iOldStart);
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                pDel,
                2 as ::core::ffi::c_int,
                iNewStart - 1 as crate::src::headers::sqlite3_h::sqlite3_int64,
            );
            crate::src::src::vdbeapi::sqlite3_step(pDel);
            rc = crate::src::src::vdbeapi::sqlite3_reset(pDel);
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut pChomp: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            SQL_CHOMP_SEGDIR,
            &raw mut pChomp,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_bind_int64(pChomp, 1 as ::core::ffi::c_int, iNewStart);
            crate::src::src::vdbeapi::sqlite3_bind_blob(
                pChomp,
                2 as ::core::ffi::c_int,
                root.a as *const ::core::ffi::c_void,
                root.n,
                crate::src::headers::sqlite3_h::SQLITE_STATIC,
            );
            crate::src::src::vdbeapi::sqlite3_bind_int64(pChomp, 3 as ::core::ffi::c_int, iAbsLevel);
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    mut pnRem: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut nRem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    i = (*pCsr).nSegment - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut pSeg: *mut Fts3SegReader = ::core::ptr::null_mut::<Fts3SegReader>();
        let mut j: ::core::ffi::c_int = 0;
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
            let mut zTerm: *const ::core::ffi::c_char = __pSeg_ref.zTerm;
            let mut nTerm: ::core::ffi::c_int = __pSeg_ref.nTerm;
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pHint: *mut Blob,
) -> ::core::ffi::c_int {
    let mut pReplace: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = fts3SqlStmt(
        p,
        SQL_REPLACE_STAT,
        &raw mut pReplace,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int(pReplace, 1 as ::core::ffi::c_int, FTS_STAT_INCRMERGEHINT);
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pHint: *mut Blob,
) -> ::core::ffi::c_int {
    let mut pSelect: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    (*pHint).n = 0 as ::core::ffi::c_int;
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_STAT,
        &raw mut pSelect,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut rc2: ::core::ffi::c_int = 0;
        crate::src::src::vdbeapi::sqlite3_bind_int(pSelect, 1 as ::core::ffi::c_int, FTS_STAT_INCRMERGEHINT);
        if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pSelect) {
            let mut aHint: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_column_blob(pSelect, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            let mut nHint: ::core::ffi::c_int =
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
        rc2 = crate::src::src::vdbeapi::sqlite3_reset(pSelect);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    rc
}

unsafe extern "C" fn fts3IncrmergeHintPush(
    mut pHint: *mut Blob,
    mut iAbsLevel: crate::src::ext::rtree::rtree::i64_0,
    mut nInput: ::core::ffi::c_int,
    mut pRc: *mut ::core::ffi::c_int,
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
            iAbsLevel as crate::src::headers::sqlite3_h::sqlite3_int64,
        );
        __pHint_ref.n += crate::src::ext::fts3::fts3::sqlite3Fts3PutVarint(
            __pHint_ref.a.offset(__pHint_ref.n as isize) as *mut ::core::ffi::c_char,
            nInput as crate::src::headers::sqlite3_h::sqlite3_int64,
        );
    }
}

unsafe extern "C" fn fts3IncrmergeHintPop(
    mut pHint: *mut Blob,
    mut piAbsLevel: *mut crate::src::ext::rtree::rtree::i64_0,
    mut pnInput: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pHint_ref = unsafe { &mut *pHint };
    let nHint: ::core::ffi::c_int = __pHint_ref.n;
    let mut i: ::core::ffi::c_int = 0;
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
        piAbsLevel as *mut crate::src::headers::sqlite3_h::sqlite_int64,
    );
    i += if *(__pHint_ref.a.offset(i as isize) as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0)
        as ::core::ffi::c_int
        & 0x80 as ::core::ffi::c_int
        != 0
    {
        crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(
            __pHint_ref.a.offset(i as isize) as *mut ::core::ffi::c_char,
            pnInput,
        )
    } else {
        *pnInput = *(__pHint_ref.a.offset(i as isize) as *mut ::core::ffi::c_char as *mut crate::src::ext::rtree::rtree::u8_0)
            as ::core::ffi::c_int;
        1 as ::core::ffi::c_int
    };
    if i != nHint {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3Incrmerge(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut nMerge: ::core::ffi::c_int,
    mut nMin: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut nRem: ::core::ffi::c_int = nMerge;
    let mut pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3MultiSegReader>();
    let mut pFilter: *mut crate::fts3Int_h::Fts3SegFilter = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3SegFilter>();
    let mut pWriter: *mut IncrmergeWriter = ::core::ptr::null_mut::<IncrmergeWriter>();
    let mut nSeg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
    let mut hint: Blob = Blob {
        a: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        n: 0 as ::core::ffi::c_int,
        nAlloc: 0 as ::core::ffi::c_int,
    };
    let mut bDirtyHint: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let nAlloc: ::core::ffi::c_int = (::core::mem::size_of::<crate::fts3Int_h::Fts3MultiSegReader>() as usize)
        .wrapping_add(::core::mem::size_of::<crate::fts3Int_h::Fts3SegFilter>() as usize)
        .wrapping_add(::core::mem::size_of::<IncrmergeWriter>() as usize)
        as ::core::ffi::c_int;
    pWriter = crate::src::src::malloc::sqlite3_malloc64(nAlloc as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut IncrmergeWriter;
    if pWriter.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    pFilter = pWriter.offset(1 as isize) as *mut IncrmergeWriter
        as *mut crate::fts3Int_h::Fts3SegFilter;
    pCsr = pFilter.offset(1 as isize) as *mut crate::fts3Int_h::Fts3SegFilter
        as *mut crate::fts3Int_h::Fts3MultiSegReader;
    rc = fts3IncrmergeHintLoad(p, &raw mut hint);
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && nRem > 0 as ::core::ffi::c_int {
        let nMod: crate::src::ext::rtree::rtree::i64_0 = (crate::fts3Int_h::FTS3_SEGDIR_MAXLEVEL * (*p).nIndex) as crate::src::ext::rtree::rtree::i64_0;
        let mut pFindLevel: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        let mut bUseHint: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = fts3SqlStmt(
            p,
            SQL_FIND_MERGE_LEVEL,
            &raw mut pFindLevel,
            ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
        );
        crate::src::src::vdbeapi::sqlite3_bind_int(
            pFindLevel,
            1 as ::core::ffi::c_int,
            if 2 as ::core::ffi::c_int > nMin {
                2 as ::core::ffi::c_int
            } else {
                nMin
            },
        );
        if crate::src::src::vdbeapi::sqlite3_step(pFindLevel) == crate::src::headers::sqlite3_h::SQLITE_ROW {
            iAbsLevel = crate::src::src::vdbeapi::sqlite3_column_int64(pFindLevel, 0 as ::core::ffi::c_int);
            nSeg = crate::src::src::vdbeapi::sqlite3_column_int(pFindLevel, 1 as ::core::ffi::c_int);
        } else {
            nSeg = -(1 as ::core::ffi::c_int);
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(pFindLevel);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && hint.n != 0 {
            let mut nHint: ::core::ffi::c_int = hint.n;
            let mut iHintAbsLevel: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
            let mut nHintSeg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = fts3IncrmergeHintPop(&raw mut hint, &raw mut iHintAbsLevel, &raw mut nHintSeg);
            if nSeg < 0 as ::core::ffi::c_int
                || iAbsLevel as crate::src::ext::rtree::rtree::i64_0 % nMod >= iHintAbsLevel as crate::src::ext::rtree::rtree::i64_0 % nMod
            {
                iAbsLevel = iHintAbsLevel;
                nSeg = if (if nMin > nSeg { nMin } else { nSeg }) < nHintSeg {
                    if nMin > nSeg {
                        nMin
                    } else {
                        nSeg
                    }
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
        if iAbsLevel < 0 as crate::src::headers::sqlite3_h::sqlite3_int64 || iAbsLevel > nMod << 32 as ::core::ffi::c_int {
            rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
            break;
        } else {
            ::libc::memset(
                pWriter as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nAlloc as crate::__stddef_size_t_h::size_t,
            );
            (*pFilter).flags = crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS;
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3IncrmergeOutputIdx(p, iAbsLevel, &raw mut iIdx);
                if iIdx == 0 as ::core::ffi::c_int
                    || bUseHint != 0 && iIdx == 1 as ::core::ffi::c_int
                {
                    let mut bIgnore: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    rc =
                        fts3SegmentIsMaxLevel(p, iAbsLevel as crate::src::ext::rtree::rtree::i64_0 + 1 as crate::src::ext::rtree::rtree::i64_0, &raw mut bIgnore);
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
                    let mut zKey: *const ::core::ffi::c_char = (*pCsr).zTerm;
                    let mut nKey: ::core::ffi::c_int = (*pCsr).nTerm;
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
                            if (*pWriter).nWork >= nRem as crate::src::ext::rtree::rtree::i64_0 && rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
                                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                            }
                            if !(rc == crate::src::headers::sqlite3_h::SQLITE_ROW) {
                                break;
                            }
                        }
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        nRem =
                            (nRem as crate::src::ext::rtree::rtree::i64_0 - (1 as crate::src::ext::rtree::rtree::i64_0 + (*pWriter).nWork)) as ::core::ffi::c_int;
                        rc = fts3IncrmergeChomp(p, iAbsLevel, pCsr, &raw mut nSeg);
                        if nSeg != 0 as ::core::ffi::c_int {
                            bDirtyHint = 1 as ::core::ffi::c_int;
                            fts3IncrmergeHintPush(
                                &raw mut hint,
                                iAbsLevel as crate::src::ext::rtree::rtree::i64_0,
                                nSeg,
                                &raw mut rc,
                            );
                        }
                    }
                }
                if nSeg != 0 as ::core::ffi::c_int {
                    (*pWriter).nLeafData *= -(1 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::sqlite3_int64;
                }
                fts3IncrmergeRelease(p, pWriter, &raw mut rc);
                if nSeg == 0 as ::core::ffi::c_int
                    && (*pWriter).bNoLeafData as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    fts3PromoteSegments(p, iAbsLevel + 1 as crate::src::headers::sqlite3_h::sqlite3_int64, (*pWriter).nLeafData);
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

unsafe extern "C" fn fts3Getint(mut pz: *mut *const ::core::ffi::c_char) -> ::core::ffi::c_int {
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
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut zParam: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut nMin: ::core::ffi::c_int = (*p).nMergeCount / 2 as ::core::ffi::c_int;
    let mut nMerge: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut z: *const ::core::ffi::c_char = zParam;
    nMerge = fts3Getint(&raw mut z);
    if *z.offset(0 as isize) as ::core::ffi::c_int == ',' as i32
        && *z.offset(1 as isize) as ::core::ffi::c_int != '\0' as i32
    {
        z = z.offset(1);
        nMin = fts3Getint(&raw mut z);
    }
    if *z.offset(0 as isize) as ::core::ffi::c_int != '\0' as i32
        || nMin < 2 as ::core::ffi::c_int
    {
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    } else {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        if (*p).bHasStat == 0 {
            crate::src::ext::fts3::fts3::sqlite3Fts3CreateStatTable(&raw mut rc,  p as *mut crate::fts3Int_h::Fts3Table);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = sqlite3Fts3Incrmerge(p, nMerge, nMin);
        }
        sqlite3Fts3SegmentsClose(p);
    }
    rc
}

unsafe extern "C" fn fts3DoAutoincrmerge(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut zParam: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let __p_ref = unsafe { &mut *p };
    __p_ref.nAutoincrmerge = fts3Getint(&raw mut zParam);
    if __p_ref.nAutoincrmerge == 1 as ::core::ffi::c_int || __p_ref.nAutoincrmerge > __p_ref.nMergeCount {
        __p_ref.nAutoincrmerge = 8 as ::core::ffi::c_int;
    }
    if __p_ref.bHasStat == 0 {
        crate::src::ext::fts3::fts3::sqlite3Fts3CreateStatTable(&raw mut rc,  p as *mut crate::fts3Int_h::Fts3Table);
        if rc != 0 {
            return rc;
        }
    }
    rc = fts3SqlStmt(
        p,
        SQL_REPLACE_STAT,
        &raw mut pStmt,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc != 0 {
        return rc;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, FTS_STAT_AUTOINCRMERGE);
    crate::src::src::vdbeapi::sqlite3_bind_int(pStmt, 2 as ::core::ffi::c_int, __p_ref.nAutoincrmerge);
    crate::src::src::vdbeapi::sqlite3_step(pStmt);
    rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    rc
}

unsafe extern "C" fn fts3ChecksumEntry(
    mut zTerm: *const ::core::ffi::c_char,
    mut nTerm: ::core::ffi::c_int,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut iDocid: crate::src::ext::rtree::rtree::i64_0,
    mut iCol: ::core::ffi::c_int,
    mut iPos: ::core::ffi::c_int,
) -> crate::src::ext::rtree::rtree::u64_0 {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: crate::src::ext::rtree::rtree::u64_0 = iDocid as crate::src::ext::rtree::rtree::u64_0;
    ret = ret.wrapping_add((ret << 3 as ::core::ffi::c_int).wrapping_add(iLangid as crate::src::ext::rtree::rtree::u64_0));
    ret = ret.wrapping_add((ret << 3 as ::core::ffi::c_int).wrapping_add(iIndex as crate::src::ext::rtree::rtree::u64_0));
    ret = ret.wrapping_add((ret << 3 as ::core::ffi::c_int).wrapping_add(iCol as crate::src::ext::rtree::rtree::u64_0));
    ret = ret.wrapping_add((ret << 3 as ::core::ffi::c_int).wrapping_add(iPos as crate::src::ext::rtree::rtree::u64_0));
    i = 0 as ::core::ffi::c_int;
    while i < nTerm {
        ret = ret.wrapping_add(
            (ret << 3 as ::core::ffi::c_int).wrapping_add(*zTerm.offset(i as isize) as crate::src::ext::rtree::rtree::u64_0),
        );
        i += 1;
    }
    ret
}

unsafe extern "C" fn fts3ChecksumIndex(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut iIndex: ::core::ffi::c_int,
    mut pRc: *mut ::core::ffi::c_int,
) -> crate::src::ext::rtree::rtree::u64_0 {
    let mut filter: crate::fts3Int_h::Fts3SegFilter = unsafe { ::core::mem::zeroed() };
    let mut csr: crate::fts3Int_h::Fts3MultiSegReader = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int = 0;
    let mut cksum: crate::src::ext::rtree::rtree::u64_0 = 0 as crate::src::ext::rtree::rtree::u64_0;
    if *pRc != 0 {
        return 0 as crate::src::ext::rtree::rtree::u64_0;
    }
    filter.flags = crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS | crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY;
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
            if !(crate::src::headers::sqlite3_h::SQLITE_ROW == rc) {
                break;
            }
            let mut pCsr: *mut ::core::ffi::c_char = csr.aDoclist;
            let mut pEnd: *mut ::core::ffi::c_char =
                pCsr.offset(csr.nDoclist as isize) as *mut ::core::ffi::c_char;
            let mut iDocid: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
            let mut iCol: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
            let mut iPos: crate::src::ext::rtree::rtree::u64_0 = 0 as crate::src::ext::rtree::rtree::u64_0;
            pCsr = pCsr.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(pCsr, &raw mut iDocid) as isize);
            while pCsr < pEnd {
                let mut iVal: crate::src::ext::rtree::rtree::u64_0 = 0 as crate::src::ext::rtree::rtree::u64_0;
                pCsr = pCsr.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintU(pCsr, &raw mut iVal) as isize);
                if pCsr < pEnd {
                    if iVal == 0 as crate::src::ext::rtree::rtree::u64_0 || iVal == 1 as crate::src::ext::rtree::rtree::u64_0 {
                        iCol = 0 as crate::src::ext::rtree::rtree::i64_0;
                        iPos = 0 as crate::src::ext::rtree::rtree::u64_0;
                        if iVal != 0 {
                            pCsr = pCsr.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(pCsr, &raw mut iCol) as isize);
                        } else {
                            pCsr = pCsr.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintU(pCsr, &raw mut iVal) as isize);
                            if (*p).bDescIdx != 0 {
                                iDocid = (iDocid as crate::src::ext::rtree::rtree::u64_0).wrapping_sub(iVal) as crate::src::ext::rtree::rtree::i64_0;
                            } else {
                                iDocid = (iDocid as crate::src::ext::rtree::rtree::u64_0).wrapping_add(iVal) as crate::src::ext::rtree::rtree::i64_0;
                            }
                        }
                    } else {
                        iPos = iPos.wrapping_add(iVal.wrapping_sub(2 as crate::src::ext::rtree::rtree::u64_0));
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
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3IntegrityCheck(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pbOk: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut cksum1: crate::src::ext::rtree::rtree::u64_0 = 0 as crate::src::ext::rtree::rtree::u64_0;
    let mut cksum2: crate::src::ext::rtree::rtree::u64_0 = 0 as crate::src::ext::rtree::rtree::u64_0;
    let mut pAllLangid: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        SQL_SELECT_ALL_LANGID,
        &raw mut pAllLangid,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut rc2: ::core::ffi::c_int = 0;
        crate::src::src::vdbeapi::sqlite3_bind_int(pAllLangid, 1 as ::core::ffi::c_int, (*p).iPrevLangid);
        crate::src::src::vdbeapi::sqlite3_bind_int(pAllLangid, 2 as ::core::ffi::c_int, (*p).nIndex);
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && crate::src::src::vdbeapi::sqlite3_step(pAllLangid) == crate::src::headers::sqlite3_h::SQLITE_ROW {
            let mut iLangid: ::core::ffi::c_int =
                crate::src::src::vdbeapi::sqlite3_column_int(pAllLangid, 0 as ::core::ffi::c_int);
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nIndex {
                cksum1 ^= fts3ChecksumIndex(p, iLangid, i, &raw mut rc);
                i += 1;
            }
        }
        rc2 = crate::src::src::vdbeapi::sqlite3_reset(pAllLangid);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut pModule: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = (*(*p).pTokenizer).pModule;
        let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        zSql = crate::src::src::printf::sqlite3_mprintf(
            b"SELECT %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).zReadExprlist,
        );
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
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pStmt) {
            let mut iDocid: crate::src::ext::rtree::rtree::i64_0 = crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
            let mut iLang: ::core::ffi::c_int = langidFromSelect(p, pStmt);
            let mut iCol: ::core::ffi::c_int = 0;
            iCol = 0 as ::core::ffi::c_int;
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && iCol < (*p).nColumn {
                if *(*p).abNotindexed.offset(iCol as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let mut zText: *const ::core::ffi::c_char =
                        crate::src::src::vdbeapi::sqlite3_column_text(pStmt, iCol + 1 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
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
                            let mut i_0: ::core::ffi::c_int = 0;
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
        *pbOk = (rc == crate::src::headers::sqlite3_h::SQLITE_OK && cksum1 == cksum2) as ::core::ffi::c_int;
    }
    rc
}

unsafe extern "C" fn fts3DoIntegrityCheck(mut p: *mut crate::fts3Int_h::Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut bOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = sqlite3Fts3IntegrityCheck(p, &raw mut bOk);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && bOk == 0 as ::core::ffi::c_int {
        rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    rc
}

unsafe extern "C" fn fts3SpecialInsert(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    let mut zVal: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(pVal) as *const ::core::ffi::c_char;
    let mut nVal: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_value_bytes(pVal);
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
        rc = fts3DoIncrmerge(
            p,
            zVal.offset(6 as isize) as *const ::core::ffi::c_char,
        );
    } else if nVal > 10 as ::core::ffi::c_int
        && 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3_strnicmp(
                zVal,
                b"automerge=\0" as *const u8 as *const ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
            )
    {
        rc = fts3DoAutoincrmerge(
            p,
            zVal.offset(10 as isize) as *const ::core::ffi::c_char,
        );
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
        let mut v: ::core::ffi::c_int = 0;
        if nVal > 9 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == crate::src::src::util::sqlite3_strnicmp(
                    zVal,
                    b"nodesize=\0" as *const u8 as *const ::core::ffi::c_char,
                    9 as ::core::ffi::c_int,
                )
        {
            v = atoi(zVal.offset(9 as isize) as *const ::core::ffi::c_char);
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
            v = atoi(zVal.offset(11 as isize) as *const ::core::ffi::c_char);
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
            (*p).bNoIncrDoclist =
                atoi(zVal.offset(21 as isize) as *const ::core::ffi::c_char);
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        } else if nVal > 11 as ::core::ffi::c_int
            && 0 as ::core::ffi::c_int
                == crate::src::src::util::sqlite3_strnicmp(
                    zVal,
                    b"mergecount=\0" as *const u8 as *const ::core::ffi::c_char,
                    11 as ::core::ffi::c_int,
                )
        {
            v = atoi(zVal.offset(11 as isize) as *const ::core::ffi::c_char);
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
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3FreeDeferredDoclists(mut pCsr: *mut crate::fts3Int_h::Fts3Cursor) {
    let mut pDef: *mut Fts3DeferredToken = ::core::ptr::null_mut::<Fts3DeferredToken>();
    pDef = (*pCsr).pDeferred;
    while !pDef.is_null() {
        fts3PendingListDelete((*pDef).pList);
        (*pDef).pList = ::core::ptr::null_mut::<PendingList>();
        pDef = (*pDef).pNext;
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3FreeDeferredTokens(mut pCsr: *mut crate::fts3Int_h::Fts3Cursor) {
    let mut pDef: *mut Fts3DeferredToken = ::core::ptr::null_mut::<Fts3DeferredToken>();
    let mut pNext: *mut Fts3DeferredToken = ::core::ptr::null_mut::<Fts3DeferredToken>();
    pDef = (*pCsr).pDeferred;
    while !pDef.is_null() {
        pNext = (*pDef).pNext;
        fts3PendingListDelete((*pDef).pList);
        crate::src::src::malloc::sqlite3_free(pDef as *mut ::core::ffi::c_void);
        pDef = pNext;
    }
    (*pCsr).pDeferred = ::core::ptr::null_mut::<Fts3DeferredToken>();
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3CacheDeferredDoclists(
    mut pCsr: *mut crate::fts3Int_h::Fts3Cursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !(*pCsr).pDeferred.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut iDocid: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
        let mut pDef: *mut Fts3DeferredToken = ::core::ptr::null_mut::<Fts3DeferredToken>();
        let __pCsr_ref = unsafe { &*pCsr };
        let p = &*(__pCsr_ref.base.pVtab as *mut crate::fts3Int_h::Fts3Table);
        let mut pT: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer = p.pTokenizer;
        let mut pModule: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = (*pT).pModule;
        iDocid = crate::src::src::vdbeapi::sqlite3_column_int64(__pCsr_ref.pStmt, 0 as ::core::ffi::c_int);
        i = 0 as ::core::ffi::c_int;
        while i < p.nColumn && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if *p.abNotindexed.offset(i as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                let mut zText: *const ::core::ffi::c_char =
                    crate::src::src::vdbeapi::sqlite3_column_text(__pCsr_ref.pStmt, i + 1 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char;
                let mut pTC: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
                    ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
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
                        let mut pPT: *mut crate::fts3Int_h::Fts3PhraseToken = (*pDef).pToken;
                        let __pPT_ref = unsafe { &*pPT };
                        if ((*pDef).iCol >= p.nColumn || (*pDef).iCol == i)
                            && (__pPT_ref.bFirst == 0 as ::core::ffi::c_int
                                || iPos == 0 as ::core::ffi::c_int)
                            && (__pPT_ref.n == nToken || __pPT_ref.isPrefix != 0 && __pPT_ref.n < nToken)
                            && 0 as ::core::ffi::c_int
                                == ::libc::memcmp(
                                    zToken as *const ::core::ffi::c_void,
                                    __pPT_ref.z as *const ::core::ffi::c_void,
                                    __pPT_ref.n as crate::__stddef_size_t_h::size_t,
                                )
                        {
                            fts3PendingListAppend(
                                &raw mut (*pDef).pList,
                                iDocid,
                                i as crate::src::headers::sqlite3_h::sqlite3_int64,
                                iPos as crate::src::headers::sqlite3_h::sqlite3_int64,
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
                rc = fts3PendingListAppendVarint(&raw mut (*pDef).pList, 0 as crate::src::headers::sqlite3_h::sqlite3_int64);
            }
            pDef = (*pDef).pNext;
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3DeferredTokenList(
    mut p: *mut Fts3DeferredToken,
    mut ppData: *mut *mut ::core::ffi::c_char,
    mut pnData: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nSkip: ::core::ffi::c_int = 0;
    let mut dummy: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
    *ppData = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *pnData = 0 as ::core::ffi::c_int;
    let __p_ref = unsafe { &mut *p };
    if __p_ref.pList.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    pRet = crate::src::src::malloc::sqlite3_malloc64((*__p_ref.pList).nData as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut ::core::ffi::c_char;
    if pRet.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    nSkip = crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint((*__p_ref.pList).aData, &raw mut dummy);
    *pnData = (*__p_ref.pList).nData - nSkip;
    *ppData = pRet;
    ::core::ptr::copy_nonoverlapping(
                    (*__p_ref.pList).aData.offset(nSkip as isize) as *mut ::core::ffi::c_char as *const u8,
                    pRet as *mut u8,
                    *pnData as usize,
                );
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3DeferToken(
    mut pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    mut pToken: *mut crate::fts3Int_h::Fts3PhraseToken,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pDeferred: *mut Fts3DeferredToken = ::core::ptr::null_mut::<Fts3DeferredToken>();
    pDeferred = crate::src::src::malloc::sqlite3_malloc64(::core::mem::size_of::<Fts3DeferredToken>() as crate::src::headers::sqlite3_h::sqlite3_uint64)
        as *mut Fts3DeferredToken;
    if pDeferred.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pDeferred as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3DeferredToken>() as crate::__stddef_size_t_h::size_t,
    );
    (*pDeferred).pToken = pToken;
    (*pDeferred).pNext = (*pCsr).pDeferred;
    (*pDeferred).iCol = iCol;
    (*pCsr).pDeferred = pDeferred;
    (*pToken).pDeferred = pDeferred;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3DeleteByRowid(
    mut p: *mut crate::fts3Int_h::Fts3Table,
    mut pRowid: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    mut pnChng: *mut ::core::ffi::c_int,
    mut aSzDel: *mut crate::src::ext::rtree::rtree::u32_0,
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
                ::libc::memset(
                    aSzDel as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<crate::src::ext::rtree::rtree::u32_0>() as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(((*p).nColumn + 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(2 as crate::__stddef_size_t_h::size_t),
                );
            } else {
                *pnChng -= 1 as ::core::ffi::c_int;
                if (*p).zContentTbl.is_null() {
                    fts3SqlExec(&raw mut rc, p, SQL_DELETE_CONTENT, &raw mut pRowid);
                }
                if (*p).bHasDocsize != 0 {
                    fts3SqlExec(&raw mut rc, p, SQL_DELETE_DOCSIZE, &raw mut pRowid);
                }
            }
        }
    }
    rc
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3UpdateMethod(
    mut pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    mut nArg: ::core::ffi::c_int,
    mut apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    mut pRowid: *mut crate::src::headers::sqlite3_h::sqlite_int64,
) -> ::core::ffi::c_int {
    let mut p: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut aSzIns: *mut crate::src::ext::rtree::rtree::u32_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u32_0>();
    let mut aSzDel: *mut crate::src::ext::rtree::rtree::u32_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u32_0>();
    let mut nChng: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bInsertDone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nArg > 1 as ::core::ffi::c_int
        && crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(0 as isize)) == crate::src::headers::sqlite3_h::SQLITE_NULL
        && crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(((*p).nColumn + 2 as ::core::ffi::c_int) as isize))
            != crate::src::headers::sqlite3_h::SQLITE_NULL
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
            (::core::mem::size_of::<crate::src::ext::rtree::rtree::u32_0>() as crate::src::headers::sqlite3_h::sqlite3_uint64)
                .wrapping_mul(
                    ((*p).nColumn as crate::src::headers::sqlite3_h::sqlite3_int64 + 1 as crate::src::headers::sqlite3_h::sqlite3_int64) as crate::src::headers::sqlite3_h::sqlite3_uint64,
                )
                .wrapping_mul(2 as crate::src::headers::sqlite3_h::sqlite3_uint64),
        ) as *mut crate::src::ext::rtree::rtree::u32_0;
        if aSzDel.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            aSzIns = aSzDel.offset(((*p).nColumn + 1 as ::core::ffi::c_int) as isize) as *mut crate::src::ext::rtree::rtree::u32_0;
            ::libc::memset(
                aSzDel as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<crate::src::ext::rtree::rtree::u32_0>() as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(((*p).nColumn + 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(2 as crate::__stddef_size_t_h::size_t),
            );
            rc = fts3Writelock(p);
            if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                if nArg > 1 as ::core::ffi::c_int && (*p).zContentTbl.is_null() {
                    let mut pNewRowid: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                        *apVal.offset((3 as ::core::ffi::c_int + (*p).nColumn) as isize);
                    if crate::src::src::vdbeapi::sqlite3_value_type(pNewRowid) == crate::src::headers::sqlite3_h::SQLITE_NULL {
                        pNewRowid = *apVal.offset(1 as isize);
                    }
                    if crate::src::src::vdbeapi::sqlite3_value_type(pNewRowid) != crate::src::headers::sqlite3_h::SQLITE_NULL
                        && (crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(0 as isize))
                            == crate::src::headers::sqlite3_h::SQLITE_NULL
                            || crate::src::src::vdbeapi::sqlite3_value_int64(*apVal.offset(0 as isize))
                                != crate::src::src::vdbeapi::sqlite3_value_int64(pNewRowid))
                    {
                        if crate::src::src::vtab::sqlite3_vtab_on_conflict((*p).db) == crate::src::headers::sqlite3_h::SQLITE_REPLACE {
                            rc = fts3DeleteByRowid(p, pNewRowid, &raw mut nChng, aSzDel);
                        } else {
                            rc = fts3InsertData(p, apVal, pRowid as *mut crate::src::headers::sqlite3_h::sqlite3_int64);
                            bInsertDone = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                    if crate::src::src::vdbeapi::sqlite3_value_type(*apVal.offset(0 as isize))
                        != crate::src::headers::sqlite3_h::SQLITE_NULL
                    {
                        rc = fts3DeleteByRowid(
                            p,
                            *apVal.offset(0 as isize),
                            &raw mut nChng,
                            aSzDel,
                        );
                    }
                    if nArg > 1 as ::core::ffi::c_int && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        let mut iLangid: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_value_int(*apVal.offset(
                            (2 as ::core::ffi::c_int + (*p).nColumn + 2 as ::core::ffi::c_int)
                                as isize,
                        ));
                        if bInsertDone == 0 as ::core::ffi::c_int {
                            rc = fts3InsertData(p, apVal, pRowid as *mut crate::src::headers::sqlite3_h::sqlite3_int64);
                            if rc == crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT && (*p).zContentTbl.is_null() {
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
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3Optimize(mut p: *mut crate::fts3Int_h::Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    rc = crate::src::src::legacy::sqlite3_exec(
        (*p).db,
        b"SAVEPOINT fts3\0" as *const u8 as *const ::core::ffi::c_char,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = fts3DoOptimize(p, 1 as ::core::ffi::c_int);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK || rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
            let mut rc2: ::core::ffi::c_int = crate::src::src::legacy::sqlite3_exec(
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
